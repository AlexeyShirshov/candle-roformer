//! Public library API: load a local separator checkpoint once, then run
//! separation on in-memory audio buffers.
//!
//! ```no_run
//! use candle_roformer::{AudioBuffer, DeviceChoice, LocalSeparationAssets, Separator};
//!
//! # fn main() -> Result<(), candle_roformer::Error> {
//! let assets = LocalSeparationAssets::new("/models/voc.safetensors", "/models/voc.yaml");
//! let separator = Separator::load(&assets, DeviceChoice::Cpu)?;
//! let audio = candle_roformer::wav::read_audio("/music/song.wav")?;
//! let stems = separator.separate(&audio)?;
//! candle_roformer::wav::write_wav_f32("/out/vocals.wav", &stems.vocals)?;
//! candle_roformer::wav::write_wav_f32("/out/instrumental.wav", &stems.instrumental)?;
//! # Ok(())
//! # }
//! ```

use std::path::Path;

use candle_core::safetensors::Load;
use candle_core::{DType, Device};
use candle_nn::VarBuilder;

use crate::assets::LocalSeparationAssets;
use crate::config::{InferenceConfig, SeparatorConfig};
use crate::device::DeviceChoice;
use crate::error::{AudioError, Error};
use crate::model::{expected_key_shapes, MelBandRoformer};
use crate::resample;
use crate::separate;
use crate::wav::AudioBuffer;

/// The stem set produced by the separator.
///
/// # Named stems
///
/// A separator may produce any number of named stems (`num_stems` in the
/// profile). They are all returned here in model order under [`Stems::stems`]
/// as `(name, AudioBuffer)` pairs, and are reachable by name through
/// [`Stems::get`]. The buffer's sample rate, channel count and length match
/// the input buffer exactly.
///
/// # Two-stem convenience
///
/// For the common vocal/instrumental ("rest of the mix") case the separator
/// still exposes [`Stems::vocals`] and [`Stems::instrumental`] as direct
/// fields, so existing two-stem callers keep working unchanged. In that mode
/// `stems` contains exactly the two entries `("vocals", ..)` and
/// `("instrumental", ..)`.
///
/// # Residual semantics
///
/// Two distinct modes exist and are *not* interchangeable:
///
/// * **Residual mode ("rest of the mix").** Used when the model estimates a
///   single target stem (`num_stems == 1`, e.g. a MelBandRoformer vocals
///   model). The model produces the target (`vocals`); the secondary stem
///   (`instrumental`) is computed as the input-waveform residual
///   `input - vocals`. Therefore `vocals + instrumental == input` exactly
///   (within f32 round-off) and the two buffers are complementary.
/// * **Independent multi-stem mode.** Used when the model estimates several
///   stems (`num_stems > 1`, e.g. Demucs / MDX / VR karaoke 4-stem models).
///   Every stem is an independent estimate taken straight from the model's own
///   mask estimator; **no** stem is replaced by `input - stem` and the sum of
///   the stems **need not** (and generally does not) equal the input. Treating
///   the last stem as "the residual" in this mode would be wrong.
///
/// Which mode applies is determined by `num_stems` of the loaded profile, not
/// by a runtime flag: a single-stem profile is residual, a multi-stem profile
/// is independent.
#[derive(Debug, Clone)]
pub struct Stems {
    /// Two-stem convenience: the vocal (target) stem. In residual mode this
    /// is the model estimate; in independent mode it is the stem named
    /// `"vocals"` if present, otherwise the first stem.
    pub vocals: AudioBuffer,
    /// Two-stem convenience: the instrumental stem. In residual mode this is
    /// the exact input residual `input - vocals`; in independent mode it is
    /// the stem named `"instrumental"` if present, otherwise the second stem
    /// (or the first when only one exists).
    pub instrumental: AudioBuffer,
    /// All stems returned by the model, in model order, as `(name, buffer)`
    /// pairs. In residual mode this is `[("vocals", ..), ("instrumental", ..)]`;
    /// in independent mode it has one entry per `num_stems`.
    pub stems: Vec<(String, AudioBuffer)>,
}

impl Stems {
    /// Look up a stem by name.
    pub fn get(&self, name: &str) -> Option<&AudioBuffer> {
        self.stems
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, b)| b)
    }

    /// Look up a stem by name (mutable).
    pub fn get_mut(&mut self, name: &str) -> Option<&mut AudioBuffer> {
        self.stems
            .iter_mut()
            .find(|(n, _)| n == name)
            .map(|(_, b)| b)
    }

    /// Number of named stems.
    pub fn len(&self) -> usize {
        self.stems.len()
    }

    /// Whether the stem set is empty.
    pub fn is_empty(&self) -> bool {
        self.stems.is_empty()
    }

    /// Names of the stems, in model order.
    pub fn names(&self) -> Vec<&str> {
        self.stems.iter().map(|(n, _)| n.as_str()).collect()
    }

    /// Iterate over `(name, buffer)` pairs.
    pub fn iter(&self) -> std::slice::Iter<'_, (String, AudioBuffer)> {
        self.stems.iter()
    }
}

/// A loaded MelBandRoformer separator bound to a device.
///
/// Loading validates the checkpoint against the config (full key set, shapes,
/// dtypes) before allocating the model, so a mismatched pair fails with a
/// typed error instead of producing garbage.
pub struct Separator {
    model: MelBandRoformer,
    config: SeparatorConfig,
    inference: InferenceConfig,
    device: DeviceChoice,
}



fn default_stem_names(n: usize) -> Vec<String> {
    match n {
        1 => vec!["vocals".to_string()],
        4 => vec!["drums", "bass", "other", "vocals"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        _ => (0..n).map(|i| format!("stem_{}", i)).collect(),
    }
}

fn named_or_index(stems: &[(String, AudioBuffer)], name: &str, idx: usize) -> AudioBuffer {
    if let Some((_, b)) = stems.iter().find(|(n, _)| n == name) {
        return b.clone();
    }
    if let Some((_, b)) = stems.get(idx) {
        return b.clone();
    }
    if let Some((_, b)) = stems.first() {
        return b.clone();
    }
    AudioBuffer {
        sample_rate: 44100,
        channels: 1,
        planar: Vec::new(),
    }
}

impl Separator {
    /// Load a local checkpoint (`safetensors`) and its YAML profile.
    pub fn load(assets: &LocalSeparationAssets, device: DeviceChoice) -> Result<Self, Error> {
        assets.validate()?;
        let cfg = SeparatorConfig::from_yaml_file(&assets.config)?;
        cfg.validate()?;
        let inference = InferenceConfig::from_yaml_file(&assets.config)?;
        inference.validate()?;
        let dev = device.resolve()?;
        strict_check(
            assets.checkpoint.to_str().ok_or_else(|| {
                Error::Audio(AudioError::Io("checkpoint path is not UTF-8".into()))
            })?,
            &cfg,
            &dev,
        )?;
        let path = assets.checkpoint.to_str().ok_or_else(|| {
            Error::Audio(AudioError::Io("checkpoint path is not UTF-8".into()))
        })?;
        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[path], DType::F32, &dev) }
            .map_err(Error::Candle)?;
        let model = MelBandRoformer::new(&cfg, vb).map_err(Error::Candle)?;
        Ok(Self {
            model,
            config: cfg,
            inference,
            device,
        })
    }

    /// Load a checkpoint with the config resolved as `<checkpoint>.yaml`.
    pub fn load_with_sidecar_config(
        checkpoint: impl AsRef<Path>,
        device: DeviceChoice,
    ) -> Result<Self, Error> {
        let checkpoint = checkpoint.as_ref();
        let assets = LocalSeparationAssets::new(checkpoint, checkpoint.with_extension("yaml"));
        Self::load(&assets, device)
    }

    pub fn config(&self) -> &SeparatorConfig {
        &self.config
    }

    pub fn inference_config(&self) -> &InferenceConfig {
        &self.inference
    }

    pub fn device_choice(&self) -> DeviceChoice {
        self.device
    }

    /// Separate an in-memory buffer. The input is resampled to the model's
    /// sample rate, processed in chunked overlap-add, then returned at the
    /// input's sample rate with the input's channel layout and exact length.
    ///
    /// The number of stems returned follows the loaded profile:
    /// - `num_stems == 1`: residual mode — `vocals` is the model estimate and
    ///   `instrumental` is the input residual `input - vocals`; `stems` holds
    ///   `[("vocals", ..), ("instrumental", ..)]`.
    /// - `num_stems > 1`: independent mode — every stem comes from the model
    ///   and `stems` holds one named entry per `num_stems` (no residual).
    pub fn separate(&self, audio: &AudioBuffer) -> Result<Stems, Error> {
        audio.validate()?;
        let channels = audio.channels as usize;
        let input_sr = audio.sample_rate;
        let n = audio.planar.len() / channels;

        // mono is duplicated to stereo for the model; stems are downmixed back
        let model_channels = if channels == 1 { 2 } else { channels };
        let model_input = if channels == 1 {
            let mut v = Vec::with_capacity(audio.planar.len() * 2);
            v.extend_from_slice(&audio.planar);
            v.extend_from_slice(&audio.planar);
            v
        } else {
            audio.planar.clone()
        };

        let model_sr = self.config.sample_rate as u32;
        let resampled = resample::resample_planar(&model_input, model_channels, input_sr, model_sr);
        let stem_outputs = separate::separate_all_with_config(
            &self.model,
            &resampled,
            model_channels,
            &self.inference,
        )?;

        // Convert each model stem back to the input sample rate / channel layout.
        let mut stem_audios: Vec<AudioBuffer> = Vec::with_capacity(stem_outputs.len());
        for out in &stem_outputs {
            let orig_sr = if channels == 1 {
                let m = out.len() / 2;
                let mut mono = vec![0f32; m];
                for i in 0..m {
                    mono[i] = (out[i] + out[m + i]) * 0.5;
                }
                mono
            } else {
                out.clone()
            };
            let resampled_back = resample::resample_planar(&orig_sr, channels, model_sr, input_sr);
            let mut final_buf = vec![0f32; channels * n];
            let vframes = resampled_back.len() / channels;
            for ch in 0..channels {
                for i in 0..n {
                    let src = ch * vframes + i;
                    if src < resampled_back.len() {
                        final_buf[ch * n + i] = resampled_back[src];
                    }
                }
            }
            stem_audios.push(AudioBuffer {
                sample_rate: input_sr,
                channels: audio.channels,
                planar: final_buf,
            });
        }

        let residual_mode = self.model.num_stems <= 1;

        let mut stems_vec: Vec<(String, AudioBuffer)> = Vec::new();
        let (vocals, instrumental) = if residual_mode {
            let vocals_audio = stem_audios
                .into_iter()
                .next()
                .unwrap_or_else(|| AudioBuffer {
                    sample_rate: input_sr,
                    channels: audio.channels,
                    planar: vec![0f32; channels * n],
                });
            let mut instr_final = vec![0f32; channels * n];
            for i in 0..channels * n {
                instr_final[i] = audio.planar[i] - vocals_audio.planar[i];
            }
            let instrumental_audio = AudioBuffer {
                sample_rate: input_sr,
                channels: audio.channels,
                planar: instr_final,
            };
            stems_vec.push(("vocals".to_string(), vocals_audio.clone()));
            stems_vec.push(("instrumental".to_string(), instrumental_audio.clone()));
            (vocals_audio, instrumental_audio)
        } else {
            let names = default_stem_names(stem_audios.len());
            for (name, audio_buf) in names.iter().zip(stem_audios.into_iter()) {
                stems_vec.push((name.clone(), audio_buf));
            }
            // Two-stem convenience: prefer named stems, else fall back to
            // the first / second stem.
            let vocals_audio = named_or_index(&stems_vec, "vocals", 0);
            let instrumental_audio = named_or_index(&stems_vec, "instrumental", 1);
            (vocals_audio, instrumental_audio)
        };

        Ok(Stems {
            vocals,
            instrumental,
            stems: stems_vec,
        })
    }
}

/// Verify the checkpoint against the config without materialising a model.
fn strict_check(model_path: &str, cfg: &SeparatorConfig, dev: &Device) -> Result<(), Error> {
    use std::collections::HashSet;

    let expected = expected_key_shapes(cfg);
    let expected_names: HashSet<String> = expected.iter().map(|(n, _)| n.clone()).collect();

    let ms = unsafe { candle_core::safetensors::MmapedSafetensors::multi(&[model_path]) }
        .map_err(Error::Candle)?;
    let actual_names: HashSet<String> = ms.tensors().iter().map(|(n, _)| n.clone()).collect();

    let mut missing: Vec<String> = expected_names.difference(&actual_names).cloned().collect();
    let mut extra: Vec<String> = actual_names.difference(&expected_names).cloned().collect();
    missing.sort();
    extra.sort();
    if !missing.is_empty() || !extra.is_empty() {
        return Err(Error::Audio(AudioError::UnsupportedFormat(format!(
            "checkpoint key mismatch: missing {:?}, extra {:?}",
            missing, extra
        ))));
    }

    let mut shape_errors: Vec<String> = Vec::new();
    for (name, shape) in &expected {
        let tv = ms.get(name).map_err(Error::Candle)?;
        let t = tv.load(dev).map_err(Error::Candle)?;
        if t.dims() != shape.as_slice() {
            shape_errors.push(format!("{}: file {:?} vs expected {:?}", name, t.dims(), shape));
        }
    }
    if !shape_errors.is_empty() {
        return Err(Error::Audio(AudioError::UnsupportedFormat(format!(
            "checkpoint shape mismatch: {:?}",
            shape_errors
        ))));
    }
    Ok(())
}

struct FrameBuf {
    channels: usize,
    data: Vec<f32>,
    base: usize,
}

impl FrameBuf {
    fn frames(&self) -> usize {
        self.data.len() / self.channels.max(1)
    }
    fn available(&self) -> usize {
        self.base + self.frames()
    }
    fn append(&mut self, planar: &[f32]) {
        let ch = self.channels.max(1);
        let new_frames = planar.len() / ch;
        if new_frames == 0 {
            return;
        }
        let old_frames = self.frames();
        let mut merged = vec![0f32; ch * (old_frames + new_frames)];
        for c in 0..ch {
            for i in 0..old_frames {
                merged[c * (old_frames + new_frames) + i] =
                    self.data[c * old_frames + i];
            }
            for i in 0..new_frames {
                merged[c * (old_frames + new_frames) + (old_frames + i)] =
                    planar[c * new_frames + i];
            }
        }
        self.data = merged;
    }
    fn slice(&self, global_start: usize, n: usize) -> Vec<f32> {
        let ch = self.channels.max(1);
        let frames = self.frames();
        let off = global_start.saturating_sub(self.base);
        let n = n.min(frames.saturating_sub(off));
        let mut out = vec![0f32; ch * n];
        for c in 0..ch {
            for i in 0..n {
                out[c * n + i] = self.data[c * frames + (off + i)];
            }
        }
        out
    }
    fn drop_before(&mut self, idx: usize) {
        let ch = self.channels.max(1);
        if idx <= self.base {
            return;
        }
        let frames = self.frames();
        let drop = (idx - self.base).min(frames);
        let keep = frames - drop;
        let mut new = vec![0f32; ch * keep];
        for c in 0..ch {
            for i in 0..keep {
                new[c * keep + i] = self.data[c * frames + (drop + i)];
            }
        }
        self.data = new;
        self.base = self.base + drop;
    }
}

impl Separator {
    pub fn separate_file(
        &self,
        input_path: impl AsRef<std::path::Path>,
        vocals_path: impl AsRef<std::path::Path>,
        instrumental_path: impl AsRef<std::path::Path>,
        chunk_duration: f64,
    ) -> Result<(), crate::error::Error> {
        use crate::wav::{AudioStreamReader, StreamingWavWriter};

        let mut reader = AudioStreamReader::open(input_path.as_ref())?;
        let input_sr = reader.sample_rate();
        let channels = reader.channels() as usize;
        let model_sr = self.config.sample_rate as u32;

        // Fallback for non-matching sample rates: decode whole input, use the
        // in-memory path, write outputs. Correct but not bounded.
        if input_sr != model_sr {
            let mut all = Vec::new();
            loop {
                let c = reader.read_frames(65536)?;
                if c.is_empty() {
                    break;
                }
                all.extend(c);
            }
            let audio = crate::wav::AudioBuffer {
                sample_rate: input_sr,
                channels: channels as u16,
                planar: all,
            };
            let stems = self.separate(&audio)?;
            crate::wav::write_wav_f32(vocals_path, &stems.vocals)?;
            crate::wav::write_wav_f32(instrumental_path, &stems.instrumental)?;
            return Ok(());
        }

        let model_channels = if channels == 1 { 2 } else { channels };
        let chunk = ((chunk_duration * model_sr as f64).round() as usize).max(1);
        let step = self.inference.step_for(chunk).max(1);
        let fade = self.inference.fade_for(chunk).max(1).min(chunk);

        let mut fade_w = vec![1f32; chunk];
        for (t, w) in fade_w.iter_mut().enumerate() {
            let mut fi = 1.0f32;
            if t < fade {
                fi = (t as f32 + 1.0) / fade as f32;
            }
            let mut fo = 1.0f32;
            if t >= chunk - fade {
                fo = (chunk - t) as f32 / fade as f32;
            }
            *w = fi.min(fo).max(1.0 / fade as f32);
        }

        let mut vw = StreamingWavWriter::create(vocals_path.as_ref(), input_sr, channels as u16)?;
        let mut iw = StreamingWavWriter::create(instrumental_path.as_ref(), input_sr, channels as u16)?;

        let capacity = chunk + step + 4;
        let mut acc_v = vec![0f32; model_channels * capacity];
        let mut acc_w = vec![0f32; capacity];
        let mut frontier: usize = 0;
        let mut current_max: usize = 0;
        let mut idx: usize = 0;
        let mut eof = false;
        let mut total_frames: usize = 0;

        let mut orig_buf = FrameBuf {
            channels,
            data: Vec::new(),
            base: 0,
        };
        let mut model_buf = FrameBuf {
            channels: model_channels,
            data: Vec::new(),
            base: 0,
        };

        loop {
            let start = idx * step;

            // Ensure enough decoded frames for this chunk (or hit EOF).
            while !eof && model_buf.available() < start + chunk {
                let need = (start + chunk).saturating_sub(model_buf.available()).max(1);
                let block = reader.read_frames(need.min(65536))?;
                if block.is_empty() {
                    eof = true;
                    break;
                }
                let frames = block.len() / channels;
                if frames == 0 {
                    eof = true;
                    break;
                }
                orig_buf.append(&block);
                let mut model_block = vec![0f32; model_channels * frames];
                for mc in 0..model_channels {
                    let oc = if channels == 1 { 0 } else { mc };
                    for f in 0..frames {
                        model_block[mc * frames + f] = block[oc * frames + f];
                    }
                }
                model_buf.append(&model_block);
            }

            if eof && total_frames == 0 {
                total_frames = model_buf.available();
            }

            if model_buf.available() <= start {
                break;
            }

            let seg_len = (model_buf.available() - start).min(chunk);
            if seg_len == 0 {
                break;
            }

            // Build padded chunk for model inference.
            let mut pcm = vec![0f32; model_channels * chunk];
            let seg = model_buf.slice(start, seg_len);

            for mc in 0..model_channels {
                for i in 0..seg_len {
                    pcm[mc * chunk + i] = seg[mc * seg_len + i];
                }
            }
            let model_out = crate::separate::infer_chunk(&self.model, &self.inference, &pcm, model_channels, chunk)?;

            for i in 0..seg_len {
                let t = start + i;
                let local = t % capacity;
                if local >= capacity {
                    // Should not happen; guard.
                    continue;
                }
                for mc in 0..model_channels {
                    acc_v[mc * capacity + local] += model_out[mc * chunk + i] * fade_w[i];
                }
                acc_w[local] += fade_w[i];
            }
            current_max = current_max.max(start + seg_len);

            // Finalize up to start+step.
            let mut up_to = start + step;
            if eof {
                up_to = up_to.min(total_frames);
            }
            if up_to > current_max {
                up_to = current_max;
            }
            if up_to > frontier {
                for t in frontier..up_to {
                    let local = t % capacity;
                    let w = acc_w[local];
                    let mut vocals_model = vec![0f32; model_channels];
                    for mc in 0..model_channels {
                        vocals_model[mc] = if w > 1e-8 {
                            acc_v[mc * capacity + local] / w
                        } else {
                            0.0
                        };
                    }
                    let mut vocals_orig = vec![0f32; channels];
                    if channels == 1 {
                        vocals_orig[0] = (vocals_model[0] + vocals_model[1]) * 0.5;
                    } else {
                        for c in 0..channels {
                            vocals_orig[c] = vocals_model[c];
                        }
                    }
                    let orig_frame = orig_buf.slice(t, 1);
                    let mut instr_orig = vec![0f32; channels];
                    for c in 0..channels {
                        instr_orig[c] = orig_frame[c] - vocals_orig[c];
                    }
                    vw.write_planar(&vocals_orig)?;
                    iw.write_planar(&instr_orig)?;
                }
                for t in frontier..up_to {
                    let local = t % capacity;
                    for mc in 0..model_channels {
                        acc_v[mc * capacity + local] = 0.0;
                    }
                    acc_w[local] = 0.0;
                }
                frontier = up_to;
            }

            model_buf.drop_before(frontier);
            orig_buf.drop_before(frontier);

            idx += 1;

            if eof && start + step >= total_frames {
                // finalize remaining up to total_frames
                let mut up_to = total_frames;
                if up_to > current_max {
                    up_to = current_max;
                }
                if up_to > frontier {
                    for t in frontier..up_to {
                        let local = t % capacity;
                        let w = acc_w[local];
                        let mut vocals_model = vec![0f32; model_channels];
                        for mc in 0..model_channels {
                            vocals_model[mc] = if w > 1e-8 {
                                acc_v[mc * capacity + local] / w
                            } else {
                                0.0
                            };
                        }
                        let mut vocals_orig = vec![0f32; channels];
                        if channels == 1 {
                            vocals_orig[0] = (vocals_model[0] + vocals_model[1]) * 0.5;
                        } else {
                            for c in 0..channels {
                                vocals_orig[c] = vocals_model[c];
                            }
                        }
                        let orig_frame = orig_buf.slice(t, 1);
                        let mut instr_orig = vec![0f32; channels];
                        for c in 0..channels {
                            instr_orig[c] = orig_frame[c] - vocals_orig[c];
                        }
                        vw.write_planar(&vocals_orig)?;
                        iw.write_planar(&instr_orig)?;
                    }
                    // zero finalized
                    for t in frontier..up_to {
                        let local = t % capacity;
                        for mc in 0..model_channels {
                            acc_v[mc * capacity + local] = 0.0;
                        }
                        acc_w[local] = 0.0;
                    }
                    frontier = up_to;
                }
                break;
            }
        }

        vw.finalize()?;
        iw.finalize()?;
        Ok(())
    }

    pub fn separate_streaming(
        &self,
        input_path: impl AsRef<std::path::Path>,
        vocals_path: impl AsRef<std::path::Path>,
        instrumental_path: impl AsRef<std::path::Path>,
        chunk_duration: f64,
    ) -> Result<(), crate::error::Error> {
        self.separate_file(input_path, vocals_path, instrumental_path, chunk_duration)
    }
}
