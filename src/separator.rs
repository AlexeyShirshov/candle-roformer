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
