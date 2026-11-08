//! Whisper transcription of an isolated vocal stem.
//!
//! The heavy lifting (tensors, encoder/decoder, log-mel) comes from the pinned
//! Candle `whisper` model. This module adds the parts Candle's tensor API does
//! not provide: long-form decoding over 30 s windows, timestamp-token rules,
//! temperature fallback, language detection, and assembly into
//! [`Transcript`] / [`TranscriptSegment`] values.
//!
//! The decoding loop and timestamp rules are adapted from
//! `candle-examples/examples/whisper/main.rs` at Candle rev
//! `cfcbd7186d13271843ce7eb41d25659bd0835850` (MIT OR Apache-2.0), with the
//! Whisper reference semantics from `openai/whisper` (MIT). Two deliberate
//! differences from the example: timestamp seconds are
//! `(token - timestamp_begin) / 50` where `timestamp_begin` is `<|0.00|>` (the
//! reference formula; the example adds a 0.04 s offset), and the no-speech
//! probability is read from the logits after the full prompt instead of from
//! the first prompt position.

use std::io::Write;

use candle_core::{IndexOp, Tensor, D};
use candle_nn::ops::{log_softmax, softmax};
use candle_nn::VarBuilder;
use candle_transformers::models::whisper::{self as m, audio, Config};
use tokenizers::Tokenizer;

use crate::assets::LocalWhisperAssets;
use crate::device::DeviceChoice;
use crate::error::{AsrError, Error};
use crate::resample;
use crate::wav::AudioBuffer;

pub mod lang;
pub mod tokens;

pub use tokens::{segments_from_tokens, timestamp_seconds, Transcript, TranscriptSegment};

/// Whisper always consumes 16 kHz mono audio (`models/mod.rs` in Candle).
pub const SAMPLE_RATE: u32 = m::SAMPLE_RATE as u32;

const MEL_FILTERS_80: &[u8] = include_bytes!("../../assets/whisper/melfilters.bytes");
const MEL_FILTERS_128: &[u8] = include_bytes!("../../assets/whisper/melfilters128.bytes");
const MEL_BINS_PER_ROW: usize = 1 + m::N_FFT / 2;

/// Decoding knobs. Defaults match OpenAI Whisper.
#[derive(Debug, Clone)]
pub struct TranscribeOptions {
    /// `None` or `"auto"` detects the language on multilingual models.
    /// An explicit code (`"ru"`, `"en"`, ...) forces it.
    pub language: Option<String>,
    /// Deterministic seed for temperature sampling (greedy pass is unaffected).
    pub seed: u64,
    pub no_speech_threshold: f64,
    pub logprob_threshold: f64,
    pub compression_ratio_threshold: f64,
}

impl Default for TranscribeOptions {
    fn default() -> Self {
        Self {
            language: None,
            seed: 299_792_458,
            no_speech_threshold: m::NO_SPEECH_THRESHOLD,
            logprob_threshold: m::LOGPROB_THRESHOLD,
            compression_ratio_threshold: m::COMPRESSION_RATIO_THRESHOLD,
        }
    }
}

/// A loaded Whisper model bound to a device. Load once, transcribe many stems.
pub struct Transcriber {
    model: m::model::Whisper,
    config: Config,
    tokenizer: Tokenizer,
    device: DeviceChoice,
    mel_filters: Vec<f32>,
    multilingual: bool,
    language_token: Option<u32>,
    sot_token: u32,
    transcribe_token: u32,
    eot_token: u32,
    no_speech_token: u32,
    no_timestamps_token: u32,
    timestamp_begin: u32,
    suppress_tokens: Tensor,
    rng: Rng,
    /// Prompt/special ids that must not be decoded as text.
    special: Vec<u32>,
}

impl Transcriber {
    /// Load `config.json`, `tokenizer.json` and `model.safetensors` from local
    /// paths. Nothing is downloaded and no quantized/fp16 path is used: the
    /// checkpoint is read as f32.
    pub fn load(assets: &LocalWhisperAssets, device: DeviceChoice) -> Result<Self, Error> {
        assets.validate()?;
        for (label, path) in [
            ("config", &assets.config),
            ("tokenizer", &assets.tokenizer),
            ("model", &assets.model),
        ] {
            if !path.is_file() {
                return Err(Error::Asr(AsrError::InvalidAssets(format!(
                    "whisper {} file not found: {}",
                    label,
                    path.display()
                ))));
            }
        }

        let config_text = std::fs::read_to_string(&assets.config).map_err(|e| {
            Error::Asr(AsrError::InvalidAssets(format!(
                "cannot read config {}: {}",
                assets.config.display(),
                e
            )))
        })?;
        let config: Config = serde_json::from_str(&config_text).map_err(|e| {
            Error::Asr(AsrError::InvalidAssets(format!(
                "invalid whisper config {}: {}",
                assets.config.display(),
                e
            )))
        })?;
        if config.d_model == 0
            || config.vocab_size == 0
            || config.max_source_positions == 0
            || config.max_target_positions < 2
        {
            return Err(Error::Asr(AsrError::InvalidAssets(
                "whisper config has zero-sized dimensions".into(),
            )));
        }
        let mel_filters = mel_filter_bank(config.num_mel_bins)?;

        let tokenizer = Tokenizer::from_file(&assets.tokenizer).map_err(|e| {
            Error::Asr(AsrError::InvalidAssets(format!(
                "cannot load tokenizer {}: {}",
                assets.tokenizer.display(),
                e
            )))
        })?;

        let no_timestamps_token = required_token(&tokenizer, m::NO_TIMESTAMPS_TOKEN)?;
        let sot_token = required_token(&tokenizer, m::SOT_TOKEN)?;
        let transcribe_token = required_token(&tokenizer, m::TRANSCRIBE_TOKEN)?;
        let eot_token = required_token(&tokenizer, m::EOT_TOKEN)?;
        let no_speech_token = m::NO_SPEECH_TOKENS
            .iter()
            .find_map(|token| tokenizer.token_to_id(token))
            .ok_or_else(|| {
                Error::Asr(AsrError::InvalidAssets(
                    "tokenizer has no no-speech token (<|nocaptions|>/<|nospeech|>)".into(),
                ))
            })?;
        let timestamp_begin = no_timestamps_token
            .checked_add(1)
            .ok_or_else(|| Error::Asr(AsrError::InvalidAssets("token id overflow".into())))?;
        if config.vocab_size as u32 <= timestamp_begin {
            return Err(Error::Asr(AsrError::InvalidAssets(format!(
                "vocab_size {} leaves no room for timestamp tokens starting at {}",
                config.vocab_size, timestamp_begin
            ))));
        }
        if tokenizer.token_to_id("<|0.00|>") != Some(timestamp_begin) {
            return Err(Error::Asr(AsrError::InvalidAssets(
                "tokenizer is not a Whisper tokenizer: <|0.00|> does not follow <|notimestamps|>"
                    .into(),
            )));
        }

        let multilingual = lang::is_multilingual(&tokenizer);
        let mut suppress = vec![0f32; config.vocab_size];
        for &id in &config.suppress_tokens {
            if (id as usize) < suppress.len() {
                suppress[id as usize] = f32::NEG_INFINITY;
            }
        }
        // Timestamps mode is always on, so the notimestamps token is forbidden.
        suppress[no_timestamps_token as usize] = f32::NEG_INFINITY;

        let dev = device.resolve()?;
        let suppress_tokens = Tensor::new(suppress.as_slice(), &dev)?;

        let model_path = assets.model.to_str().ok_or_else(|| {
            Error::Asr(AsrError::InvalidAssets(
                "model path is not valid UTF-8".into(),
            ))
        })?;
        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_path], m::DTYPE, &dev) }
            .map_err(Error::Candle)?;
        let model = m::model::Whisper::load(&vb, config.clone()).map_err(Error::Candle)?;

        Ok(Self {
            model,
            config,
            tokenizer,
            device,
            mel_filters,
            multilingual,
            language_token: None,
            sot_token,
            transcribe_token,
            eot_token,
            no_speech_token,
            no_timestamps_token,
            timestamp_begin,
            suppress_tokens,
            rng: Rng::new(TranscribeOptions::default().seed),
            special: Vec::new(),
        }
        .with_special())
    }

    fn with_special(mut self) -> Self {
        let translate_token = self.tokenizer.token_to_id(m::TRANSLATE_TOKEN);
        self.special = [
            Some(self.sot_token),
            Some(self.eot_token),
            Some(self.transcribe_token),
            Some(self.no_timestamps_token),
            translate_token,
        ]
        .into_iter()
        .flatten()
        .collect();
        self
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn device_choice(&self) -> DeviceChoice {
        self.device
    }

    /// True when the loaded tokenizer/model carries language tokens.
    pub fn is_multilingual(&self) -> bool {
        self.multilingual
    }

    /// Transcribe an isolated vocal stem with default options.
    pub fn transcribe(&mut self, vocals: &AudioBuffer) -> Result<Transcript, Error> {
        self.transcribe_with(vocals, &TranscribeOptions::default())
    }

    /// Transcribe an isolated vocal stem.
    ///
    /// The buffer is downmixed to mono and resampled to 16 kHz on a copy; the
    /// caller's stem is never modified. Timestamps are relative to the start of
    /// this buffer. Audio is decoded in non-overlapping 30 s windows; segments
    /// are monotonic, finite, and clamped to the buffer duration.
    pub fn transcribe_with(
        &mut self,
        vocals: &AudioBuffer,
        options: &TranscribeOptions,
    ) -> Result<Transcript, Error> {
        vocals.validate()?;
        let mono = downmix_to_mono(vocals);
        let pcm = resample::resample_planar(&mono, 1, vocals.sample_rate, SAMPLE_RATE);
        if pcm.is_empty() {
            return Err(Error::Asr(AsrError::Failed(
                "vocal stem is empty after resampling".into(),
            )));
        }
        // Clamp segment timestamps to the caller's timeline, not to the
        // resampler's rounded sample count.
        let total_duration = vocals.duration_secs();

        let mel = audio::pcm_to_mel(&self.config, &pcm, &self.mel_filters);
        let n_mel = self.config.num_mel_bins;
        let frames = mel.len() / n_mel;
        if frames == 0 {
            return Err(Error::Asr(AsrError::Failed(
                "log-mel spectrogram is empty".into(),
            )));
        }
        let mel = Tensor::from_vec(mel, (1, n_mel, frames), self.suppress_tokens.device())?;

        let (language_token, language) = self.resolve_language(options, &mel)?;
        self.language_token = language_token;
        self.rng = Rng::new(options.seed);
        self.model.reset_kv_cache();

        let mut segments: Vec<TranscriptSegment> = Vec::new();
        let mut seek = 0usize;
        while seek < frames {
            let window_frames = (frames - seek).min(m::N_FRAMES);
            let offset = seek as f64 * m::HOP_LENGTH as f64 / SAMPLE_RATE as f64;
            let window_end =
                offset + window_frames as f64 * m::HOP_LENGTH as f64 / SAMPLE_RATE as f64;
            let mel_window = mel.narrow(2, seek, window_frames)?;
            seek += window_frames;

            let dr = self.decode_with_fallback(&mel_window, options)?;
            if dr.no_speech_prob > options.no_speech_threshold
                && dr.avg_logprob < options.logprob_threshold
            {
                continue;
            }

            let mut window_segments = segments_from_tokens(
                &dr.tokens,
                self.timestamp_begin,
                offset,
                window_end,
                total_duration,
                &self.special,
                |ids| self.tokenizer.decode(ids, true).map_err(|e| e.to_string()),
            )
            .map_err(|e| Error::Asr(AsrError::Failed(format!("tokenizer decode: {}", e))))?;
            if let (Some(last), Some(first)) = (segments.last(), window_segments.first()) {
                if tokens::is_boundary_duplicate(last, first) {
                    window_segments.remove(0);
                }
            }
            segments.extend(window_segments);
        }

        Ok(Transcript { language, segments })
    }

    fn resolve_language(
        &mut self,
        options: &TranscribeOptions,
        mel: &Tensor,
    ) -> Result<(Option<u32>, Option<String>), Error> {
        match options.language.as_deref() {
            Some(code)
                if code.eq_ignore_ascii_case("auto") || code.eq_ignore_ascii_case("detect") =>
            {
                self.auto_language(mel)
            }
            Some(code) => {
                let code = code.to_ascii_lowercase();
                if !self.multilingual {
                    return Err(Error::Asr(AsrError::UnsupportedLanguage(format!(
                        "{}: the loaded Whisper model has no language tokens (English-only model)",
                        code
                    ))));
                }
                let token = self
                    .tokenizer
                    .token_to_id(&format!("<|{code}|>"))
                    .ok_or_else(|| {
                        Error::Asr(AsrError::UnsupportedLanguage(format!(
                            "{} is not a Whisper language code",
                            code
                        )))
                    })?;
                Ok((Some(token), Some(code)))
            }
            None => self.auto_language(mel),
        }
    }

    fn auto_language(&mut self, mel: &Tensor) -> Result<(Option<u32>, Option<String>), Error> {
        if !self.multilingual {
            return Ok((None, Some("en".to_string())));
        }
        let code = self.detect_language(mel)?;
        let token = self
            .tokenizer
            .token_to_id(&format!("<|{code}|>"))
            .ok_or_else(|| {
                Error::Asr(AsrError::InvalidAssets(format!(
                    "tokenizer lacks <|{}|> after detection",
                    code
                )))
            })?;
        Ok((Some(token), Some(code)))
    }

    /// Language detection on the first window, as in the Whisper reference.
    fn detect_language(&mut self, mel: &Tensor) -> Result<String, Error> {
        let (_, _, seq_len) = mel.dims3()?;
        let mel = mel.narrow(2, 0, seq_len.min(self.config.max_source_positions))?;
        let ids = lang::language_token_ids(&self.tokenizer).map_err(AsrError::InvalidAssets)?;
        let audio_features = self.model.encoder.forward(&mel, true)?;
        let tokens = Tensor::new(&[[self.sot_token]], mel.device())?;
        let language_ids = Tensor::new(ids.as_slice(), mel.device())?;
        let ys = self.model.decoder.forward(&tokens, &audio_features, true)?;
        let logits = self.model.decoder.final_linear(&ys.i(..1)?)?.i(0)?.i(0)?;
        let logits = logits.index_select(&language_ids, 0)?;
        let probs = softmax(&logits, D::Minus1)?.to_vec1::<f32>()?;
        let (index, _) = probs
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .ok_or_else(|| Error::Asr(AsrError::Failed("language scores are empty".into())))?;
        Ok(lang::LANGUAGES[index].0.to_string())
    }

    fn decode_with_fallback(
        &mut self,
        mel: &Tensor,
        options: &TranscribeOptions,
    ) -> Result<DecodingResult, Error> {
        let mut last_error = None;
        for (i, &temperature) in m::TEMPERATURES.iter().enumerate() {
            let is_last = i + 1 == m::TEMPERATURES.len();
            match self.decode(mel, temperature) {
                Ok(dr) => {
                    let needs_fallback = dr.compression_ratio > options.compression_ratio_threshold
                        || dr.avg_logprob < options.logprob_threshold;
                    if is_last || !needs_fallback || dr.no_speech_prob > options.no_speech_threshold
                    {
                        return Ok(dr);
                    }
                }
                Err(e) => {
                    last_error = Some(e);
                    if is_last {
                        break;
                    }
                }
            }
        }
        Err(last_error.unwrap_or_else(|| {
            Error::Asr(AsrError::Failed(
                "decoding produced no result at any temperature".into(),
            ))
        }))
    }

    fn decode(&mut self, mel: &Tensor, temperature: f64) -> Result<DecodingResult, Error> {
        let audio_features = self.model.encoder.forward(mel, true)?;
        let sample_len = self.config.max_target_positions / 2;
        let mut tokens = vec![self.sot_token];
        if let Some(language) = self.language_token {
            tokens.push(language);
        }
        tokens.push(self.transcribe_token);
        let prefix_len = tokens.len();

        let mut sum_logprob = 0f64;
        let mut sampled = 0usize;
        let mut no_speech_prob = f64::NAN;

        for i in 0..sample_len {
            let tokens_t = Tensor::new(tokens.as_slice(), mel.device())?.unsqueeze(0)?;
            let ys = self
                .model
                .decoder
                .forward(&tokens_t, &audio_features, i == 0)?;

            let (_, seq_len, _) = ys.dims3()?;
            // Logits after the full prompt: position `seq_len - 1`, which is
            // the first position that predicts a generated token. The no-speech
            // probability must be read here (not at position zero, as the
            // Candle example does), before timestamp masks are applied.
            let logits = self
                .model
                .decoder
                .final_linear(&ys.i((..1, seq_len - 1..))?)?
                .i(0)?
                .i(0)?;
            if i == 0 {
                no_speech_prob = softmax(&logits, 0)?
                    .i(self.no_speech_token as usize)?
                    .to_scalar::<f32>()? as f64;
            }

            let logits = self.apply_timestamp_rules(&logits, &tokens)?;
            let logits = logits.broadcast_add(&self.suppress_tokens)?;

            let next_token = if temperature > 0.0 {
                let probs = softmax(&(&logits / temperature)?, 0)?;
                let probs: Vec<f32> = probs.to_vec1()?;
                self.rng.sample(&probs).unwrap_or(0) as u32
            } else {
                let logits: Vec<f32> = logits.to_vec1()?;
                argmax(&logits)
            };

            let prob = softmax(&logits, D::Minus1)?
                .i(next_token as usize)?
                .to_scalar::<f32>()? as f64;
            tokens.push(next_token);
            sampled += 1;
            if prob.is_finite() && prob > 0.0 {
                sum_logprob += prob.ln();
            }

            if next_token == self.eot_token || tokens.len() > self.config.max_target_positions {
                break;
            }
        }

        let content = &tokens[prefix_len..];
        let text = self
            .tokenizer
            .decode(content, true)
            .map_err(|e| Error::Asr(AsrError::Failed(format!("tokenizer decode: {}", e))))?;

        Ok(DecodingResult {
            // Content tokens only; the prompt is not part of the transcript.
            tokens: tokens[prefix_len..].to_vec(),
            avg_logprob: if sampled == 0 {
                f64::NEG_INFINITY
            } else {
                sum_logprob / sampled as f64
            },
            no_speech_prob,
            compression_ratio: compression_ratio(&text),
        })
    }

    /// Whisper timestamp rules (openai/whisper `ApplyTimestampRules`).
    fn apply_timestamp_rules(
        &self,
        input_logits: &Tensor,
        tokens: &[u32],
    ) -> Result<Tensor, Error> {
        let device = input_logits.device().clone();
        let timestamp_begin = self.timestamp_begin;
        let vocab_size = self.config.vocab_size as u32;
        let sample_begin = if self.language_token.is_some() { 3 } else { 2 };

        let sampled_tokens = if tokens.len() > sample_begin {
            &tokens[sample_begin..]
        } else {
            &[]
        };

        let mut masks: Vec<Tensor> = Vec::new();
        let mut buffer = vec![0f32; vocab_size as usize];

        // 1. Timestamps come in pairs, except right before EOT.
        if !sampled_tokens.is_empty() {
            let last_was_timestamp = sampled_tokens
                .last()
                .map(|&t| t >= timestamp_begin)
                .unwrap_or(false);
            let penultimate_was_timestamp = sampled_tokens
                .len()
                .checked_sub(2)
                .map(|i| sampled_tokens[i] >= timestamp_begin)
                .unwrap_or(false);

            if last_was_timestamp {
                if penultimate_was_timestamp {
                    // Two timestamps in a row: next must be text.
                    for (i, slot) in buffer.iter_mut().enumerate() {
                        *slot = if i as u32 >= timestamp_begin {
                            f32::NEG_INFINITY
                        } else {
                            0.0
                        };
                    }
                } else {
                    // Timestamp followed by text: next cannot be plain text.
                    for (i, slot) in buffer.iter_mut().enumerate() {
                        *slot = if (i as u32) < self.eot_token {
                            f32::NEG_INFINITY
                        } else {
                            0.0
                        };
                    }
                }
                masks.push(Tensor::new(buffer.as_slice(), &device)?);
            }

            // 2. Timestamps must not decrease.
            let timestamps: Vec<u32> = sampled_tokens
                .iter()
                .copied()
                .filter(|&t| t >= timestamp_begin)
                .collect();
            if let Some(&last) = timestamps.last() {
                let last_allowed = if last_was_timestamp && !penultimate_was_timestamp {
                    last
                } else {
                    last.saturating_add(1)
                };
                for (i, slot) in buffer.iter_mut().enumerate() {
                    *slot = if i as u32 >= timestamp_begin && (i as u32) < last_allowed {
                        f32::NEG_INFINITY
                    } else {
                        0.0
                    };
                }
                masks.push(Tensor::new(buffer.as_slice(), &device)?);
            }
        }

        // 3. The first generated token must be a timestamp.
        if tokens.len() == sample_begin {
            for (i, slot) in buffer.iter_mut().enumerate() {
                *slot = if (i as u32) < timestamp_begin {
                    f32::NEG_INFINITY
                } else {
                    0.0
                };
            }
            masks.push(Tensor::new(buffer.as_slice(), &device)?);
        }

        let mut logits = input_logits.clone();
        for mask in &masks {
            logits = logits.broadcast_add(mask)?;
        }

        // 4. If the summed probability of timestamps exceeds any text token,
        //    only timestamps are allowed.
        let log_probs = log_softmax(&logits, 0)?;
        let timestamp_log_probs = log_probs.narrow(
            0,
            timestamp_begin as usize,
            vocab_size as usize - timestamp_begin as usize,
        )?;
        let text_log_probs = log_probs.narrow(0, 0, timestamp_begin as usize)?;
        let timestamp_logprob = {
            let max_val = timestamp_log_probs.max(0)?;
            let shifted = timestamp_log_probs.broadcast_sub(&max_val)?;
            let sum_exp = shifted.exp()?.sum(0)?;
            max_val.broadcast_add(&sum_exp.log()?)?.to_scalar::<f32>()? as f64
        };
        let max_text = text_log_probs.max(0)?.to_scalar::<f32>()? as f64;
        if timestamp_logprob > max_text {
            for (i, slot) in buffer.iter_mut().enumerate() {
                *slot = if (i as u32) < timestamp_begin {
                    f32::NEG_INFINITY
                } else {
                    0.0
                };
            }
            logits = logits.broadcast_add(&Tensor::new(buffer.as_slice(), &device)?)?;
        }
        Ok(logits)
    }
}

struct DecodingResult {
    /// Generated content tokens (prompt stripped; special ids still present and
    /// filtered during segment assembly).
    tokens: Vec<u32>,
    avg_logprob: f64,
    no_speech_prob: f64,
    compression_ratio: f64,
}

fn required_token(tokenizer: &Tokenizer, name: &str) -> Result<u32, Error> {
    tokenizer.token_to_id(name).ok_or_else(|| {
        Error::Asr(AsrError::InvalidAssets(format!(
            "tokenizer has no {} token",
            name
        )))
    })
}

fn argmax(values: &[f32]) -> u32 {
    values
        .iter()
        .enumerate()
        .filter(|(_, v)| v.is_finite())
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(i, _)| i as u32)
        .unwrap_or(0)
}

/// Log-mel filter bank for the given bin count (80 or 128), parsed from the
/// embedded Whisper assets as little-endian f32.
pub fn mel_filter_bank(num_mel_bins: usize) -> Result<Vec<f32>, Error> {
    let bytes = match num_mel_bins {
        80 => MEL_FILTERS_80,
        128 => MEL_FILTERS_128,
        other => {
            return Err(Error::Asr(AsrError::InvalidAssets(format!(
                "unsupported num_mel_bins {} (expected 80 or 128)",
                other
            ))))
        }
    };
    let expected = num_mel_bins * MEL_BINS_PER_ROW;
    if bytes.len() != expected * 4 {
        return Err(Error::Asr(AsrError::InvalidAssets(format!(
            "mel filter asset has {} bytes, expected {}",
            bytes.len(),
            expected * 4
        ))));
    }
    let mut filters = Vec::with_capacity(expected);
    for chunk in bytes.chunks_exact(4) {
        let value = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        if !value.is_finite() || value < 0.0 {
            return Err(Error::Asr(AsrError::InvalidAssets(
                "mel filter asset contains a non-finite or negative value".into(),
            )));
        }
        filters.push(value);
    }
    Ok(filters)
}

/// Average all channels of an audio buffer into one mono channel. The input is
/// not modified.
pub fn downmix_to_mono(audio: &AudioBuffer) -> Vec<f32> {
    let channels = audio.channels.max(1) as usize;
    let frames = audio.frames();
    if channels == 1 {
        return audio.planar.clone();
    }
    let mut mono = vec![0f32; frames];
    for c in 0..channels {
        for (i, sample) in mono.iter_mut().enumerate() {
            *sample += audio.planar[c * frames + i];
        }
    }
    let scale = 1.0 / channels as f32;
    for sample in &mut mono {
        *sample *= scale;
    }
    mono
}

/// zlib compression ratio of the decoded text, as used by Whisper's
/// temperature fallback to detect repetition loops.
fn compression_ratio(text: &str) -> f64 {
    if text.is_empty() {
        return 1.0;
    }
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
    if encoder.write_all(text.as_bytes()).is_err() {
        return f64::INFINITY;
    }
    match encoder.finish() {
        Ok(compressed) if !compressed.is_empty() => text.len() as f64 / compressed.len() as f64,
        _ => 1.0,
    }
}

/// Small deterministic xorshift64* PRNG for temperature sampling. Using an
/// in-crate generator keeps sampling reproducible across platforms and avoids
/// depending on `rand`'s global state.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Sample an index from unnormalised probabilities. Falls back to argmax
    /// when the distribution is degenerate (all zeros/NaNs).
    fn sample(&mut self, probs: &[f32]) -> Option<usize> {
        let total: f64 = probs
            .iter()
            .filter(|p| p.is_finite() && **p > 0.0)
            .map(|p| *p as f64)
            .sum();
        if !(total > 0.0) {
            return probs
                .iter()
                .enumerate()
                .filter(|(_, p)| p.is_finite())
                .max_by(|(_, a), (_, b)| a.total_cmp(b))
                .map(|(i, _)| i);
        }
        let mut target = self.next_f64() * total;
        for (i, &p) in probs.iter().enumerate() {
            if p.is_finite() && p > 0.0 {
                target -= p as f64;
                if target <= 0.0 {
                    return Some(i);
                }
            }
        }
        probs
            .iter()
            .enumerate()
            .filter(|(_, p)| p.is_finite())
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(i, _)| i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mel_filters_have_whisper_shape() {
        for bins in [80usize, 128] {
            let f = mel_filter_bank(bins).unwrap();
            assert_eq!(f.len(), bins * 201);
            assert!(f.iter().all(|v| v.is_finite() && *v >= 0.0));
            assert!(f.iter().any(|v| *v > 0.0));
        }
        assert!(mel_filter_bank(64).is_err());
    }

    #[test]
    fn compression_ratio_flags_repetition() {
        let plain = "a normal sentence with several distinct words";
        let repeated = "la la la la la la la la la la la la la la la la la la la la la la la la";
        assert!(compression_ratio(repeated) > compression_ratio(plain));
        assert_eq!(compression_ratio(""), 1.0);
    }

    #[test]
    fn downmix_averages_channels() {
        let audio = AudioBuffer {
            sample_rate: 44100,
            channels: 2,
            planar: vec![1.0, 0.0, 0.0, 1.0],
        };
        assert_eq!(downmix_to_mono(&audio), vec![0.5, 0.5]);
    }

    #[test]
    fn sampler_respects_the_distribution() {
        let mut rng = Rng::new(7);
        let probs = [0.0f32, 1.0, 0.0];
        for _ in 0..100 {
            assert_eq!(rng.sample(&probs), Some(1));
        }
        assert_eq!(rng.sample(&[f32::NAN, f32::NAN]), None);
    }
}
