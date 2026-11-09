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

/// The two stems produced by the separator. Sample rate, channel count and
/// length match the input buffer exactly.
#[derive(Debug, Clone)]
pub struct Stems {
    pub vocals: AudioBuffer,
    pub instrumental: AudioBuffer,
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
    /// The instrumental is the input-waveform residual: `input - vocals`.
    pub fn separate(&self, audio: &AudioBuffer) -> Result<Stems, Error> {
        audio.validate()?;
        let channels = audio.channels as usize;
        let input_sr = audio.sample_rate;

        // mono is duplicated to stereo for the model; the vocal is downmixed back
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
        let vocals_model = separate::separate_with_config(&self.model, &resampled, model_channels, &self.inference)?;

        let vocals_orig_sr = if channels == 1 {
            let n = vocals_model.len() / 2;
            let mut mono = vec![0f32; n];
            for i in 0..n {
                mono[i] = (vocals_model[i] + vocals_model[n + i]) * 0.5;
            }
            mono
        } else {
            vocals_model
        };

        let vocals_resampled =
            resample::resample_planar(&vocals_orig_sr, channels, model_sr, input_sr);

        let n = audio.planar.len() / channels;
        let vframes = vocals_resampled.len() / channels;
        let mut vocals_final = vec![0f32; channels * n];
        for ch in 0..channels {
            for i in 0..n {
                let src = ch * vframes + i;
                if src < vocals_resampled.len() {
                    vocals_final[ch * n + i] = vocals_resampled[src];
                }
            }
        }
        let mut instr_final = vec![0f32; channels * n];
        for i in 0..channels * n {
            instr_final[i] = audio.planar[i] - vocals_final[i];
        }

        Ok(Stems {
            vocals: AudioBuffer {
                sample_rate: input_sr,
                channels: audio.channels,
                planar: vocals_final,
            },
            instrumental: AudioBuffer {
                sample_rate: input_sr,
                channels: audio.channels,
                planar: instr_final,
            },
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
