//! Segment-wise Demucs inference: the `demucs.apply.apply_model` split path,
//! audio-separator's per-track normalisation and the named four-stem output.
//!
//! The reference call is
//!
//! ```text
//! apply_model(model, mix[None], shifts=0, split=True, overlap=0.25,
//!             transition_power=1.0, static_shifts=1, device="cpu")
//! ```
//!
//! so this module reproduces, in order:
//!
//! 1. audio-separator's per-track normalisation
//!    (`ref = mix.mean(0)`, `(mix - ref.mean()) / max(ref.std(), eps)`);
//! 2. `segment = int(samplerate * model.segment)` = 343980 samples and
//!    `stride = int((1 - overlap) * segment)` = 257985;
//! 3. the triangular cross-fade weight `cat([1..half], [half..1]) / half`;
//! 4. `TensorChunk.padded(segment)` centring (zero padding both sides) and
//!    `center_trim` back to the chunk length;
//! 5. de-normalisation with the **unclamped** `ref.std()`;
//! 6. the four stems in the contract's order `drums, bass, other, vocals`.
//!
//! `shifts = 0` means the random-shift branch of `apply_model` is skipped
//! entirely, so this path is deterministic.

use std::path::Path;

use candle_core::{Device, Tensor};

use crate::device::DeviceChoice;
use crate::error::{AudioError, Error};
use crate::separator::Stems;
use crate::wav::AudioBuffer;

use super::demucs::{DemucsConfig, HTDemucs, SOURCES};

/// The approved reference overlap (`demucs_params.overlap`).
pub const DEFAULT_OVERLAP: f64 = 0.25;

/// A loaded Demucs v4 separator.
///
/// Construct with [`DemucsSeparator::load`] on the `safetensors` artifact
/// produced by `tools/convert_demucs.py`; the original `.th` package is never
/// read at runtime.
pub struct DemucsSeparator {
    model: HTDemucs,
    overlap: f64,
    device: DeviceChoice,
    device_t: Device,
}

impl DemucsSeparator {
    /// Load a converted HTDemucs artifact with full tensor-name/shape validation.
    pub fn load(checkpoint: impl AsRef<Path>, device: DeviceChoice) -> Result<Self, Error> {
        let path = checkpoint.as_ref();
        if !path.is_file() {
            return Err(Error::Audio(AudioError::Io(format!(
                "Demucs checkpoint not found: {}",
                path.display()
            ))));
        }
        let dev = device.resolve()?;
        let tensors = candle_core::safetensors::load(path, &dev).map_err(|e| {
            Error::Audio(AudioError::Io(format!(
                "cannot read Demucs safetensors artifact {}: {e}",
                path.display()
            )))
        })?;
        let cfg = DemucsConfig::default();
        let model = HTDemucs::load(tensors, cfg, &dev)?;
        Ok(Self {
            model,
            overlap: DEFAULT_OVERLAP,
            device,
            device_t: dev,
        })
    }

    pub fn config(&self) -> &DemucsConfig {
        &self.model.cfg
    }

    pub fn segment_samples(&self) -> usize {
        self.model.segment_samples()
    }

    /// Stem names in output order (`HTDemucs.sources`).
    pub fn stem_names(&self) -> [&'static str; 4] {
        SOURCES
    }

    pub fn device_choice(&self) -> DeviceChoice {
        self.device
    }

    /// Separate an in-memory buffer.
    ///
    /// The input is resampled to 44.1 kHz, stereo, normalised exactly like
    /// audio-separator, run through `apply_model(split=True)`, de-normalised and
    /// resampled back to the input rate. Stems keep the input's length and are
    /// returned under the contract names `drums`, `bass`, `other`, `vocals`.
    pub fn separate(&self, audio: &AudioBuffer) -> Result<Stems, Error> {
        audio.validate()?;
        let channels = audio.channels as usize;
        let input_sr = audio.sample_rate;
        let n = audio.planar.len() / channels;
        let model_channels = self.model.cfg.audio_channels;

        let model_input = if channels == 1 {
            let mut v = Vec::with_capacity(audio.planar.len() * model_channels);
            for _ in 0..model_channels {
                v.extend_from_slice(&audio.planar);
            }
            v
        } else if channels == model_channels {
            audio.planar.clone()
        } else {
            return Err(Error::Audio(AudioError::Io(format!(
                "Demucs expects 1 or {model_channels} input channels, got {channels}"
            ))));
        };
        let model_sr = self.model.cfg.samplerate as u32;
        let resampled = crate::resample::resample_planar(&model_input, model_channels, input_sr, model_sr);
        let m = resampled.len() / model_channels;

        // ---- audio-separator normalisation ---------------------------------
        let mut ref_series = vec![0f32; m];
        for i in 0..m {
            let mut acc = 0f32;
            for c in 0..model_channels {
                acc += resampled[c * m + i];
            }
            ref_series[i] = acc / model_channels as f32;
        }
        let ref_mean = mean(&ref_series);
        let ref_std = std_unbiased(&ref_series);
        let norm_std = ref_std.max(f32::EPSILON);
        let mut norm = vec![0f32; resampled.len()];
        for (k, v) in resampled.iter().enumerate() {
            norm[k] = (*v - ref_mean) / norm_std;
        }

        // ---- apply_model(split=True, shifts=0) -----------------------------
        let stems = self.apply_split(&norm, model_channels, m)?;

        // ---- de-normalisation ----------------------------------------------
        let mut planar: Vec<Vec<f32>> = Vec::with_capacity(SOURCES.len());
        for s in 0..SOURCES.len() {
            let mut buf = stems[s].clone();
            for v in buf.iter_mut() {
                *v = *v * ref_std + ref_mean;
            }
            planar.push(buf);
        }

        // ---- back to the input layout --------------------------------------
        let mut out: Vec<(String, AudioBuffer)> = Vec::with_capacity(SOURCES.len());
        for (s, name) in SOURCES.iter().enumerate() {
            let mut buf = planar[s].clone();
            if input_sr != model_sr {
                buf = crate::resample::resample_planar(&buf, model_channels, model_sr, input_sr);
            }
            let out_channels = audio.channels;
            let planar_out = if channels == 1 {
                // Average the duplicated channels back to mono.
                let len = buf.len() / model_channels;
                let mut mono = vec![0f32; len];
                for i in 0..len {
                    let mut acc = 0f32;
                    for c in 0..model_channels {
                        acc += buf[c * len + i];
                    }
                    mono[i] = acc / model_channels as f32;
                }
                mono
            } else {
                buf
            };
            let len = planar_out.len() / out_channels as usize;
            let _ = n;
            out.push((
                (*name).to_string(),
                AudioBuffer {
                    sample_rate: input_sr,
                    channels: out_channels,
                    planar: planar_out[..len * out_channels as usize].to_vec(),
                },
            ));
        }

        let vocals = out
            .iter()
            .find(|(n, _)| n == "vocals")
            .map(|(_, b)| b.clone())
            .or_else(|| out.first().map(|(_, b)| b.clone()))
            .ok_or_else(|| Error::Audio(AudioError::Io("no Demucs stems produced".into())))?;
        let instrumental = out
            .iter()
            .find(|(n, _)| n == "drums")
            .map(|(_, b)| b.clone())
            .or_else(|| out.get(1).map(|(_, b)| b.clone()))
            .unwrap_or_else(|| vocals.clone());
        Ok(Stems {
            vocals,
            instrumental,
            stems: out,
        })
    }

    /// The split/overlap-add path of `apply_model`, returning four stems of
    /// `n` samples each (model sample rate, normalised scale).
    fn apply_split(&self, mix: &[f32], channels: usize, n: usize) -> Result<Vec<Vec<f32>>, Error> {
        let seg = self.model.segment_samples();
        let stride = ((1.0 - self.overlap) * seg as f64) as usize;
        let ns = SOURCES.len();
        let mut acc = vec![0f32; ns * channels * n];
        let mut sum_weight = vec![0f32; n];
        let weight = triangle_weight(seg);

        let mut offset = 0usize;
        while offset < n {
            let chunk_len = seg.min(n - offset);
            // TensorChunk.padded(seg): centre the chunk, zero-pad both sides.
            let delta = seg - chunk_len;
            let start = offset as isize - (delta / 2) as isize;
            let end = start + seg as isize;
            let correct_start = start.max(0) as usize;
            let correct_end = (end.min(n as isize)).max(0) as usize;
            let pad_left = correct_start as isize - start;
            let pad_right = end - correct_end as isize;
            let mut padded = vec![0f32; channels * seg];
            for c in 0..channels {
                let dst = c * seg + pad_left as usize;
                let src = &mix[c * n + correct_start..c * n + correct_end];
                padded[dst..dst + src.len()].copy_from_slice(src);
            }
            let _ = pad_right;
            let x = Tensor::from_vec(padded, (1, channels, seg), &self.device_t)
                .map_err(Error::Candle)?;
            let out = self.model.forward_segment(&x)?;
            // center_trim(out, chunk_len)
            let trim = delta / 2;
            let out = out.narrow(3, trim, chunk_len).map_err(Error::Candle)?;
            let data = out
                .to_device(&Device::Cpu)
                .map_err(Error::Candle)?
                .flatten_all()
                .map_err(Error::Candle)?
                .to_vec1::<f32>()
                .map_err(Error::Candle)?;
            for s in 0..ns {
                for c in 0..channels {
                    let src = (s * channels + c) * chunk_len;
                    let dst = (s * channels + c) * n + offset;
                    for i in 0..chunk_len {
                        acc[dst + i] += weight[i] * data[src + i];
                    }
                }
            }
            for i in 0..chunk_len {
                sum_weight[offset + i] += weight[i];
            }
            offset += stride;
        }

        let mut stems: Vec<Vec<f32>> = Vec::with_capacity(ns);
        for s in 0..ns {
            let mut buf = vec![0f32; channels * n];
            for c in 0..channels {
                for i in 0..n {
                    let w = sum_weight[i];
                    buf[c * n + i] = if w > 0.0 {
                        acc[(s * channels + c) * n + i] / w
                    } else {
                        0.0
                    };
                }
            }
            stems.push(buf);
        }
        Ok(stems)
    }
}

/// `apply_model`'s triangular cross-fade weight, peak 1 at the segment centre.
fn triangle_weight(segment: usize) -> Vec<f32> {
    let half = segment / 2;
    let max = half as f32;
    let mut w = vec![0f32; segment];
    for i in 0..half {
        w[i] = (i + 1) as f32 / max;
    }
    for i in 0..segment - half {
        w[half + i] = (segment - half - i) as f32 / max;
    }
    w
}

fn mean(x: &[f32]) -> f32 {
    if x.is_empty() {
        return 0.0;
    }
    let sum: f64 = x.iter().map(|v| *v as f64).sum();
    (sum / x.len() as f64) as f32
}

/// `torch.std()` default (unbiased, `N - 1`).
fn std_unbiased(x: &[f32]) -> f32 {
    if x.len() < 2 {
        return 0.0;
    }
    let mu = mean(x) as f64;
    let sq: f64 = x.iter().map(|v| (*v as f64 - mu).powi(2)).sum();
    (sq / (x.len() - 1) as f64).sqrt() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_weight_matches_the_reference() {
        let w = triangle_weight(8);
        assert_eq!(w.len(), 8);
        // half = 4 -> 1,2,3,4 then 4,3,2,1 divided by 4
        let expect = [0.25f32, 0.5, 0.75, 1.0, 1.0, 0.75, 0.5, 0.25];
        for (a, b) in w.iter().zip(expect.iter()) {
            assert!((a - b).abs() < 1e-6, "{w:?}");
        }
    }

    #[test]
    fn std_is_unbiased() {
        let x = [1.0f32, 2.0, 3.0, 4.0];
        // sample std of 1..4 is sqrt(5/3) ~ 1.290994
        assert!((std_unbiased(&x) - 1.290_994_4).abs() < 1e-6);
    }
}

