use std::fs;

use candle_roformer::config::SeparatorConfig;
use candle_roformer::wav::{read_audio, write_wav_f32, AudioBuffer};
use candle_roformer::{DeviceChoice, LocalSeparationAssets, Separator};

fn tmpdir(name: &str) -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("cr_stream_{}", name));
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

fn write_tiny_model(dir: &std::path::Path, cfg: &SeparatorConfig) -> (std::path::PathBuf, std::path::PathBuf) {
    use candle_core::{DType, Device};
    use candle_nn::{VarBuilder, VarMap};
    let dev = Device::Cpu;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &dev);
    let _ = candle_roformer::model::MelBandRoformer::new(cfg, vb).unwrap();
    let model = dir.join("tiny.safetensors");
    varmap.save(&model).unwrap();

    let yaml = format!(
        "model:\n  dim: {}\n  depth: {}\n  stereo: true\n  num_stems: 1\n  time_transformer_depth: 1\n  freq_transformer_depth: 1\n  num_bands: {}\n  dim_head: {}\n  heads: {}\n  flash_attn: false\n  dim_freqs_in: {}\n  sample_rate: 44100\n  stft_n_fft: {}\n  stft_hop_length: {}\n  stft_win_length: {}\n  stft_normalized: false\n  mask_estimator_depth: 2\n  use_torch_checkpoint: false\n  mlp_expansion_factor: 4\n  linear_transformer_depth: 0\n  norm_output: true\n  skip_connection: false\n  zero_dc: true\n",
        cfg.dim, cfg.depth, cfg.num_bands, cfg.dim_head, cfg.heads, cfg.dim_freqs_in,
        cfg.stft_n_fft, cfg.stft_hop_length, cfg.stft_win_length
    );
    let config = dir.join("tiny.yaml");
    fs::write(&config, yaml).unwrap();
    (model, config)
}

#[test]
fn chunked_length_and_residual() {
    let cfg = tiny_config();
    let dir = tmpdir("len");
    let (model, config) = write_tiny_model(&dir, &cfg);
    let assets = LocalSeparationAssets::new(&model, &config);
    let separator = Separator::load(&assets, DeviceChoice::Cpu).unwrap();

    let frames = 5000usize;
    let samples: Vec<f32> = (0..frames).map(|i| (i as f32 * 0.03).sin() * 0.4).collect();
    let mut planar = vec![0f32; 2 * frames];
    for i in 0..frames {
        planar[i] = samples[i];
        planar[frames + i] = samples[i] * 0.5;
    }
    let audio = AudioBuffer {
        sample_rate: 44100,
        channels: 2,
        planar: planar.clone(),
    };
    let input_path = dir.join("in.wav");
    write_wav_f32(&input_path, &audio).unwrap();

    let vpath = dir.join("v.wav");
    let ipath = dir.join("i.wav");
    let cd = 3200.0 / 44100.0;
    separator.separate_file(&input_path, &vpath, &ipath, cd).unwrap();

    let v = read_audio(&vpath).unwrap();
    let iv = read_audio(&ipath).unwrap();
    assert_eq!(v.frames(), frames, "vocals length mismatch: got {} expected {}", v.frames(), frames);
    assert_eq!(iv.frames(), frames, "instrumental length mismatch: got {} expected {}", iv.frames(), frames);
    for i in 0..(2 * frames) {
        let sum = v.planar[i] + iv.planar[i];
        assert!(
            (planar[i] - sum).abs() < 1e-5,
            "residual too large at {}: {} vs {}",
            i, planar[i], sum
        );
    }
}

#[test]
fn chunked_boundary_parity() {
    let cfg = tiny_config();
    let dir = tmpdir("parity");
    let (model, config) = write_tiny_model(&dir, &cfg);
    let assets = LocalSeparationAssets::new(&model, &config);
    let separator = Separator::load(&assets, DeviceChoice::Cpu).unwrap();

    let frames = 6000usize;
    let samples: Vec<f32> = (0..frames).map(|i| (i as f32 * 0.037).sin() * 0.3).collect();
    let mut planar = vec![0f32; 2 * frames];
    for i in 0..frames {
        planar[i] = samples[i];
        planar[frames + i] = samples[i] * 0.5;
    }
    let audio = AudioBuffer {
        sample_rate: 44100,
        channels: 2,
        planar: planar.clone(),
    };
    let input_path = dir.join("in.wav");
    write_wav_f32(&input_path, &audio).unwrap();

    // monolithic
    let stems = separator.separate(&audio).unwrap();

    // chunked
    let vpath = dir.join("v.wav");
    let ipath = dir.join("i.wav");
    let cd = 3200.0 / 44100.0;
    separator.separate_file(&input_path, &vpath, &ipath, cd).unwrap();
    let v = read_audio(&vpath).unwrap();

    let mut max_abs = 0f32;
    let mut sq = 0f64;
    let n = stems.vocals.planar.len();
    for i in 0..n {
        let d = (stems.vocals.planar[i] - v.planar[i]).abs();
        if d > max_abs { max_abs = d; }
        sq += (d as f64) * (d as f64);
    }
    let rmse = (sq / n as f64).sqrt();
    // document in test output
    eprintln!("E_boundary max_abs={} rmse={}", max_abs, rmse);
    assert!(max_abs <= 1e-4, "boundary max_abs too large: {}", max_abs);
}

#[test]
fn chunked_mono() {
    let cfg = tiny_config();
    let dir = tmpdir("mono");
    let (model, config) = write_tiny_model(&dir, &cfg);
    let assets = LocalSeparationAssets::new(&model, &config);
    let separator = Separator::load(&assets, DeviceChoice::Cpu).unwrap();

    let frames = 3000usize;
    let samples: Vec<f32> = (0..frames).map(|i| (i as f32 * 0.05).sin() * 0.4).collect();
    let audio = AudioBuffer {
        sample_rate: 44100,
        channels: 1,
        planar: samples.clone(),
    };
    let input_path = dir.join("in.wav");
    write_wav_f32(&input_path, &audio).unwrap();

    let vpath = dir.join("v.wav");
    let ipath = dir.join("i.wav");
    let cd = 3200.0 / 44100.0;
    separator.separate_file(&input_path, &vpath, &ipath, cd).unwrap();

    let v = read_audio(&vpath).unwrap();
    let iv = read_audio(&ipath).unwrap();
    assert_eq!(v.frames(), frames);
    assert_eq!(iv.frames(), frames);
    for i in 0..frames {
        let sum = v.planar[i] + iv.planar[i];
        assert!((samples[i] - sum).abs() < 1e-5);
    }
}
