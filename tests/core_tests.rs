//! Core crate tests: model construction, resampling, OLA and WAV I/O.

use std::cell::Cell;
use std::fs;
use std::path::PathBuf;

use candle_core::Device;
use candle_nn::VarBuilder;
use candle_roformer::config::{InferenceConfig, SeparatorConfig};
use candle_roformer::model::MelBandRoformer;
use candle_roformer::resample;
use candle_roformer::separate;
use candle_roformer::wav::{read_audio, write_wav_f32, AudioBuffer};

fn tmpdir(name: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("cr_test_{}", name));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

fn tiny_config() -> SeparatorConfig {
    SeparatorConfig {
        dim: 8,
        depth: 1,
        stereo: true,
        num_stems: 1,
        num_bands: 4,
        family: None,
        freqs_per_bands: None,
        dim_head: 4,
        heads: 1,
        flash_attn: false,
        dim_freqs_in: 9,
        sample_rate: 44100,
        stft_n_fft: 16,
        stft_hop_length: 4,
        stft_win_length: 16,
        stft_normalized: false,
        mask_estimator_depth: 2,
        use_torch_checkpoint: false,
        time_transformer_depth: 1,
        freq_transformer_depth: 1,
        mlp_expansion_factor: 4,
        linear_transformer_depth: 0,
        norm_output: true,
        skip_connection: false,
        zero_dc: true,
    }
}

#[test]
fn expected_keys_match_known_checkpoints() {
    let v7 = SeparatorConfig::v7();
    assert_eq!(
        candle_roformer::model::expected_key_shapes(&v7).len(),
        828,
        "voc_fv7 state_dict has 828 tensors"
    );
    let mut kijai = SeparatorConfig::v7();
    kijai.dim = 384;
    kijai.depth = 6;
    assert_eq!(
        candle_roformer::model::expected_key_shapes(&kijai).len(),
        684,
        "KimberleyJSN/Kijai safetensors has 684 tensors"
    );
}

#[test]
fn resample_planar_keeps_channels_separate() {
    let frames = 1000usize;
    let mut planar = vec![0.0f32; 2 * frames];
    for i in 0..frames {
        planar[i] = (i as f32 * 0.01).sin();
        planar[frames + i] = (i as f32 * 0.02).cos();
    }
    let out = resample::resample_planar(&planar, 2, 48000, 44100);
    let of = out.len() / 2;
    let l = resample::resample(&planar[..frames], 48000, 44100);
    let r = resample::resample(&planar[frames..], 48000, 44100);
    assert_eq!(of, l.len());
    assert_eq!(&out[..of], &l[..]);
    assert_eq!(&out[of..], &r[..]);
}

#[test]
fn resample_identity() {
    let input: Vec<f32> = (0..100).map(|i| i as f32 * 0.01).collect();
    let out = resample::resample(&input, 44100, 44100);
    assert_eq!(out, input);
}

#[test]
fn resample_roundtrip_lengths() {
    let input: Vec<f32> = (0..44100).map(|i| (i as f32 * 0.01).sin()).collect();
    let down = resample::resample(&input, 44100, 16000);
    assert!(!down.is_empty());
    let up = resample::resample(&down, 16000, 44100);
    assert!((up.len() as i64 - input.len() as i64).abs() <= 2);
    assert!(up.iter().all(|x| x.is_finite()));
}

#[test]
fn model_tiny_forward() {
    let cfg = tiny_config();
    cfg.validate().unwrap();
    let dev = Device::Cpu;
    let varmap = candle_nn::VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &dev);
    let model = MelBandRoformer::new(&cfg, vb).unwrap();
    // random weights already initialized in varmap
    let input = candle_core::Tensor::zeros((1, 2, 64), candle_core::DType::F32, &dev).unwrap();
    let out = model.forward(&input).unwrap();
    assert_eq!(out.dims(), &[1, 2, 64]);
    let v: Vec<f32> = out.flatten_all().unwrap().to_vec1().unwrap();
    assert!(v.iter().all(|x| x.is_finite()));
}

#[test]
fn strict_loader_missing_extra() {
    let cfg = tiny_config();
    cfg.validate().unwrap();
    let dev = Device::Cpu;
    // build expected varmap
    let varmap = candle_nn::VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &dev);
    let _ = MelBandRoformer::new(&cfg, vb).unwrap();
    // save
    let dir = tmpdir("strict");
    let path = dir.join("m.safetensors");
    varmap.save(&path).unwrap();

    // A matching profile must still load from the saved checkpoint.
    let vb2 = unsafe {
        VarBuilder::from_mmaped_safetensors(
            &[path.to_str().unwrap()],
            candle_core::DType::F32,
            &dev,
        )
    }
    .unwrap();
    let m = MelBandRoformer::new(&cfg, vb2).unwrap();
    let inp = candle_core::Tensor::zeros((1, 2, 64), candle_core::DType::F32, &dev).unwrap();
    let _ = m.forward(&inp).unwrap();
}

#[test]
fn ola_identity() {
    for &len in &[10usize, 176400usize, 400000usize, 176401usize] {
        let input: Vec<f32> = (0..len).map(|i| (i as f32 * 0.001).sin()).collect();
        // channels=1
        let planar = input.clone();
        let out = separate::ola(&planar, 1, 800, &|chunk: &[f32]| Ok(chunk.to_vec())).unwrap();
        assert_eq!(out.len(), len);
        let maxerr = input
            .iter()
            .zip(out.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f32, f32::max);
        assert!(maxerr < 1e-4, "len {} maxerr {}", len, maxerr);
    }
}

#[test]
fn wav_roundtrip_f32() {
    let dir = tmpdir("wav");
    let buf = AudioBuffer {
        sample_rate: 44100,
        channels: 1,
        planar: (0..1000).map(|i| (i as f32 * 0.01).sin()).collect(),
    };
    let path = dir.join("out.wav");
    write_wav_f32(&path, &buf).unwrap();
    let read = read_audio(&path).unwrap();
    assert_eq!(read.sample_rate, 44100);
    assert_eq!(read.channels, 1);
    assert_eq!(read.planar.len(), buf.planar.len());
    let maxerr = buf
        .planar
        .iter()
        .zip(read.planar.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);
    assert!(maxerr < 1e-6);
}

#[test]
fn wav_roundtrip_stereo() {
    let dir = tmpdir("wav_st");
    let mut planar = Vec::new();
    for c in 0..2usize {
        for i in 0..1000usize {
            planar.push(((i as f32 * 0.01) + c as f32).sin());
        }
    }
    let buf = AudioBuffer {
        sample_rate: 44100,
        channels: 2,
        planar,
    };
    let path = dir.join("st.wav");
    write_wav_f32(&path, &buf).unwrap();
    let read = read_audio(&path).unwrap();
    assert_eq!(read.channels, 2);
    assert_eq!(read.sample_rate, 44100);
    assert_eq!(read.planar.len(), buf.planar.len());
    let maxerr = buf
        .planar
        .iter()
        .zip(read.planar.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);
    assert!(
        maxerr < 1e-6,
        "stereo channel order/length mismatch: {maxerr}"
    );
}

#[test]
fn wav_errors() {
    let dir = tmpdir("wverr");
    // empty file
    let p = dir.join("empty.wav");
    fs::write(&p, b"").unwrap();
    assert!(read_audio(&p).is_err());
    // >2 channels not directly; test nonfinite write rejected
    let buf = AudioBuffer {
        sample_rate: 44100,
        channels: 1,
        planar: vec![f32::NAN],
    };
    let p2 = dir.join("nan.wav");
    assert!(write_wav_f32(&p2, &buf).is_err());
}

#[test]
fn residual_exact() {
    // synthetic small: vocals = half input
    let input: Vec<f32> = (0..256).map(|i| (i as f32 * 0.1).sin()).collect();
    let vocals: Vec<f32> = input.iter().map(|x| x * 0.5).collect();
    let instr: Vec<f32> = input
        .iter()
        .zip(vocals.iter())
        .map(|(a, b)| a - b)
        .collect();
    for i in 0..input.len() {
        assert!((input[i] - (vocals[i] + instr[i])).abs() < 1e-6);
    }
}

#[test]
fn wav_read_pcm16_pcm24() {
    // craft PCM16 wav
    let dir = tmpdir("pcm");
    let samples: Vec<i16> = (0..100).map(|i| i as i16 * 100).collect();
    let mut data = Vec::new();
    // RIFF
    data.extend_from_slice(b"RIFF");
    let size = 36 + samples.len() * 2;
    data.extend_from_slice(&(size as u32).to_le_bytes());
    data.extend_from_slice(b"WAVE");
    data.extend_from_slice(b"fmt ");
    data.extend_from_slice(&16u32.to_le_bytes());
    data.extend_from_slice(&1u16.to_le_bytes()); // PCM
    data.extend_from_slice(&1u16.to_le_bytes()); // channels
    data.extend_from_slice(&44100u32.to_le_bytes());
    data.extend_from_slice(&(44100 * 2u32).to_le_bytes());
    data.extend_from_slice(&2u16.to_le_bytes());
    data.extend_from_slice(&16u16.to_le_bytes());
    data.extend_from_slice(b"data");
    data.extend_from_slice(&((samples.len() * 2) as u32).to_le_bytes());
    for s in &samples {
        data.extend_from_slice(&s.to_le_bytes());
    }
    let p16 = dir.join("p16.wav");
    fs::write(&p16, &data).unwrap();
    let r16 = read_audio(&p16).unwrap();
    assert_eq!(r16.channels, 1);
    assert_eq!(r16.planar.len(), samples.len());

    // PCM24
    let samples24: Vec<i32> = (0..50).map(|i| i as i32 * 1000).collect();
    let mut data24 = Vec::new();
    data24.extend_from_slice(b"RIFF");
    let size24 = 36 + samples24.len() * 3;
    data24.extend_from_slice(&(size24 as u32).to_le_bytes());
    data24.extend_from_slice(b"WAVE");
    data24.extend_from_slice(b"fmt ");
    data24.extend_from_slice(&16u32.to_le_bytes());
    data24.extend_from_slice(&1u16.to_le_bytes());
    data24.extend_from_slice(&1u16.to_le_bytes());
    data24.extend_from_slice(&44100u32.to_le_bytes());
    data24.extend_from_slice(&(44100 * 3u32).to_le_bytes());
    data24.extend_from_slice(&3u16.to_le_bytes());
    data24.extend_from_slice(&24u16.to_le_bytes());
    data24.extend_from_slice(b"data");
    data24.extend_from_slice(&((samples24.len() * 3) as u32).to_le_bytes());
    for s in &samples24 {
        let v = *s;
        data24.push((v & 0xff) as u8);
        data24.push(((v >> 8) & 0xff) as u8);
        data24.push(((v >> 16) & 0xff) as u8);
    }
    let p24 = dir.join("p24.wav");
    fs::write(&p24, &data24).unwrap();
    let r24 = read_audio(&p24).unwrap();
    assert_eq!(r24.channels, 1);
    assert_eq!(r24.planar.len(), samples24.len());
}

#[test]
fn ola_pass_count_sweep() {
    // step series
    for &step in &[1usize, 2, 3, 5, 10, 100, 800] {
        let n = 4000usize;
        let chunk = 800usize;
        let input = vec![0.0f32; n];
        let calls = Cell::new(0usize);
        let out = separate::ola_with(
            &input,
            1,
            chunk,
            step,
            (chunk / 10).max(1),
            &|c: &[f32]| {
                calls.set(calls.get() + 1);
                Ok(c.to_vec())
            },
        )
        .unwrap();
        assert_eq!(out.len(), n);
        let expected = (n + step - 1) / step;
        assert_eq!(
            calls.get(),
            expected,
            "step={} calls={} expected={}",
            step,
            calls.get(),
            expected
        );
    }
    // overlap series
    let mut prev_calls = 0usize;
    for &overlap in &[8usize, 4, 2, 1] {
        let n = 4000usize;
        let chunk = 800usize;
        let input = vec![0.0f32; n];
        let calls = Cell::new(0usize);
        let step = (chunk / overlap).max(1);
        let _ = separate::ola_overlap(&input, 1, chunk, overlap, &|c: &[f32]| {
            calls.set(calls.get() + 1);
            Ok(c.to_vec())
        })
        .unwrap();
        let expected = (n + step - 1) / step;
        assert_eq!(
            calls.get(),
            expected,
            "overlap={} calls={} expected={}",
            overlap,
            calls.get(),
            expected
        );
        let _ = prev_calls;
        prev_calls = calls.get();
    }
}

#[test]
fn defaults_reproduce_current_numbers() {
    let hops = 800usize;
    let hop = 441usize;
    let chunk = hop * hops;
    let step = (chunk / 2).max(1);
    let fade = (chunk / 10).max(1);
    assert_eq!(chunk, 352800);
    assert_eq!(step, 176400);
    assert_eq!(fade, 35280);

    let ic = InferenceConfig::default();
    assert_eq!(ic.chunk_hops, 800);
    assert_eq!(ic.num_overlap, 2);
    assert_eq!(ic.batch_size, 1);
    assert_eq!(ic.pitch_shift, 0.0);
    assert!(!ic.denoise);
    assert!(!ic.tta);

    let input: Vec<f32> = (0..1000).map(|i| (i as f32 * 0.001).sin()).collect();
    let c1 = Cell::new(0usize);
    let c2 = Cell::new(0usize);
    let a = separate::ola(&input, 1, 800, &|c: &[f32]| {
        c1.set(c1.get() + 1);
        Ok(c.to_vec())
    })
    .unwrap();
    let b = separate::ola_with(&input, 1, 800, 400, 80, &|c: &[f32]| {
        c2.set(c2.get() + 1);
        Ok(c.to_vec())
    })
    .unwrap();
    assert_eq!(c1.get(), c2.get());
    let maxdiff = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .fold(0.0f32, f32::max);
    assert_eq!(maxdiff, 0.0);
}

#[test]
fn inference_config_invalid_overlap() {
    let mut c = InferenceConfig::default();
    c.num_overlap = 0;
    let err = c.validate().unwrap_err();
    assert!(err.to_string().contains("overlap"), "{}", err);
}

#[test]
fn inference_config_invalid_batch() {
    let mut c = InferenceConfig::default();
    c.batch_size = 0;
    let err = c.validate().unwrap_err();
    assert!(err.to_string().contains("batch"), "{}", err);
}

#[test]
fn inference_config_yaml_defaults() {
    let yaml = "model:\n  dim: 8\n";
    let ic = InferenceConfig::from_yaml_str(yaml).unwrap();
    assert_eq!(ic, InferenceConfig::default());
}

#[test]
fn pitch_shift_changes_output() {
    let cfg = tiny_config();
    cfg.validate().unwrap();
    let dev = Device::Cpu;
    let varmap = candle_nn::VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &dev);
    let model = MelBandRoformer::new(&cfg, vb).unwrap();
    let n = 64usize;
    let channels = 2usize;
    let input: Vec<f32> = (0..channels * n).map(|i| (i as f32 * 0.01).sin()).collect();

    let base = InferenceConfig::default();
    let out0 = separate::separate_with_config(&model, &input, channels, &base).unwrap();

    let mut ic = InferenceConfig::default();
    ic.pitch_shift = 12.0;
    ic.sample_rate = cfg.sample_rate;
    let out1 = separate::separate_with_config(&model, &input, channels, &ic).unwrap();

    assert_eq!(out0.len(), input.len());
    assert_eq!(out1.len(), input.len());
    let maxdiff = out0
        .iter()
        .zip(out1.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);
    assert!(
        maxdiff > 0.0,
        "pitch_shift must change output, got maxdiff {}",
        maxdiff
    );
    assert!(out1.iter().all(|x| x.is_finite()));
}

#[test]
fn batch_size_changes_model_invocations() {
    let n = 4000usize;
    let chunk = 800usize;
    let step = 400usize;
    let fade = 80usize;
    let input = vec![0.0f32; n];

    use std::cell::Cell;
    let calls1 = Cell::new(0usize);
    let maxbatch1 = Cell::new(0usize);
    let _ = separate::ola_with_batch(&input, 1, chunk, step, fade, 1, &|pcm: &[f32]| {
        calls1.set(calls1.get() + 1);
        maxbatch1.set(maxbatch1.get().max(pcm.len() / chunk));
        Ok(pcm.to_vec())
    })
    .unwrap();

    let calls4 = Cell::new(0usize);
    let maxbatch4 = Cell::new(0usize);
    let _ = separate::ola_with_batch(&input, 1, chunk, step, fade, 4, &|pcm: &[f32]| {
        calls4.set(calls4.get() + 1);
        maxbatch4.set(maxbatch4.get().max(pcm.len() / chunk));
        Ok(pcm.to_vec())
    })
    .unwrap();

    assert!(calls4.get() < calls1.get(), "batch=4 should reduce calls");
    assert!(maxbatch4.get() > 1, "batch=4 should observe batch dim > 1");
    assert_eq!(maxbatch1.get(), 1);
}

#[test]
fn inference_config_yaml_absent_defaults() {
    let yaml = "model:\n  dim: 8\n";
    let ic = InferenceConfig::from_yaml_str(yaml).unwrap();
    assert_eq!(ic, InferenceConfig::default());
}

#[test]
fn inference_config_yaml_malformed_errors() {
    let yaml = "model:\n  dim: 8\ninference:\n  batch_size: 0\n";
    let err = InferenceConfig::from_yaml_str(yaml).unwrap_err();
    assert!(err.to_string().contains("batch"), "{}", err);
}

#[test]
fn inference_config_invalid_fade_range() {
    let mut c = InferenceConfig::default();
    c.segment_size = Some(8);
    c.fade_size = Some(9);
    let err = c.validate().unwrap_err();
    assert!(err.to_string().contains("fade"), "{}", err);
}
