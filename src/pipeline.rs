use std::fs;
use std::path::Path;

use candle_core::{DType, Device};
use candle_core::safetensors::Load;
use candle_nn::{VarBuilder, VarMap};
use serde::Deserialize;

use crate::config::SeparatorConfig;
use crate::error::{AudioError, Error};
use crate::model::MelBandRoformer;
use crate::resample;
use crate::separate;
use crate::wav::{read_audio, write_wav_f32, AudioBuffer};

#[derive(Deserialize)]
struct YamlRoot {
    model: SeparatorConfig,
}

pub fn run_pipeline(
    model_path: &str,
    config_path: &str,
    input_path: &str,
    output_dir: &str,
    device: crate::device::DeviceChoice,
    overwrite: bool,
    limit_seconds: Option<f64>,
) -> Result<(), Error> {
    // parse config
    let yaml_str = fs::read_to_string(config_path)
        .map_err(|e| Error::Audio(AudioError::Io(e.to_string())))?;
    let root: YamlRoot = serde_yaml::from_str(&yaml_str)
        .map_err(|e| Error::Config(crate::error::ConfigError::Invalid(e.to_string())))?;
    let cfg = root.model;
    cfg.validate()?;

    let dev = device.resolve()?;

    // strict key check + load
    strict_check(model_path, &cfg, &dev)?;
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_path], DType::F32, &dev) }
        .map_err(Error::Candle)?;
    let model = MelBandRoformer::new(&cfg, vb).map_err(Error::Candle)?;

    // decode
    let mut audio = read_audio(input_path)?;
    if let Some(secs) = limit_seconds {
        if secs <= 0.0 || !secs.is_finite() {
            return Err(Error::Audio(AudioError::InvalidSampleRate(
                "invalid --duration".into(),
            )));
        }
        let frames = audio.planar.len() / audio.channels as usize;
        let want = ((secs * audio.sample_rate as f64).round() as usize).min(frames);
        if want == 0 {
            return Err(Error::Audio(AudioError::EmptyInput(
                "duration trims the whole input".into(),
            )));
        }
        audio.planar.truncate(want * audio.channels as usize);
    }
    let channels = audio.channels as usize;
    if channels == 0 || channels > 2 {
        return Err(Error::Audio(AudioError::TooManyChannels(format!("{}", channels))));
    }
    let input_sr = audio.sample_rate;

    // mono duplicate
    let model_channels = if channels == 1 { 2 } else { channels };
    let mut model_input = if channels == 1 {
        let mut v = Vec::with_capacity(audio.planar.len() * 2);
        v.extend_from_slice(&audio.planar);
        v.extend_from_slice(&audio.planar);
        v
    } else {
        audio.planar.clone()
    };

    // resample to 44100
    let resampled = resample::resample_planar(&model_input, model_channels, input_sr, 44100);

    // separate
    let vocals_44k = separate::separate(&model, &resampled, model_channels)?;

    // downmix vocals to original channels
    let vocals_orig_44k = if channels == 1 {
        let n = vocals_44k.len() / 2;
        let mut mono = vec![0f32; n];
        for i in 0..n {
            mono[i] = (vocals_44k[i] + vocals_44k[n + i]) * 0.5;
        }
        mono
    } else {
        vocals_44k
    };

    // resample back
    let vocals_out = resample::resample_planar(&vocals_orig_44k, channels, 44100, input_sr);

    // exact N
    let n = audio.planar.len() / channels;
    let vframes = vocals_out.len() / channels;
    let mut vocals_final = vec![0f32; channels * n];
    for ch in 0..channels {
        for i in 0..n {
            let src = ch * vframes + i;
            if src < vocals_out.len() {
                vocals_final[ch * n + i] = vocals_out[src];
            }
        }
    }
    let mut instr_final = vec![0f32; channels * n];
    for i in 0..channels * n {
        instr_final[i] = audio.planar[i] - vocals_final[i];
    }

    // output dir
    let out_dir = Path::new(output_dir);
    fs::create_dir_all(out_dir)
        .map_err(|e| Error::Audio(AudioError::Io(e.to_string())))?;

    let vocals_path = out_dir.join("vocals.wav");
    let instr_path = out_dir.join("instrumental.wav");

    if !overwrite && (vocals_path.exists() || instr_path.exists()) {
        return Err(Error::Audio(AudioError::Io(
            "output file exists; use --overwrite".into(),
        )));
    }

    let staging_v = out_dir.join(".vocals.wav.tmp");
    let staging_i = out_dir.join(".instrumental.wav.tmp");

    let buf_v = AudioBuffer {
        sample_rate: input_sr,
        channels: channels as u16,
        planar: vocals_final,
    };
    let buf_i = AudioBuffer {
        sample_rate: input_sr,
        channels: channels as u16,
        planar: instr_final,
    };

    if let Err(e) = write_wav_f32(&staging_v, &buf_v) {
        let _ = fs::remove_file(&staging_v);
        return Err(e);
    }
    if let Err(e) = write_wav_f32(&staging_i, &buf_i) {
        let _ = fs::remove_file(&staging_v);
        let _ = fs::remove_file(&staging_i);
        return Err(e);
    }
    if let Err(e) = fs::rename(&staging_v, &vocals_path) {
        let _ = fs::remove_file(&staging_v);
        let _ = fs::remove_file(&staging_i);
        return Err(Error::Audio(AudioError::Io(e.to_string())));
    }
    if let Err(e) = fs::rename(&staging_i, &instr_path) {
        let _ = fs::remove_file(&staging_i);
        return Err(Error::Audio(AudioError::Io(e.to_string())));
    }
    Ok(())
}

fn strict_check(model_path: &str, cfg: &SeparatorConfig, dev: &Device) -> Result<(), Error> {
    use std::collections::HashSet;

    let expected = crate::model::expected_key_shapes(cfg);
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

    // Per-tensor shape check; tensors are loaded one at a time (a few MB each)
    // instead of materialising a full dummy model.
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
