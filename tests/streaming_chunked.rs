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

fn write_tiny_model(
    dir: &std::path::Path,
    cfg: &SeparatorConfig,
) -> (std::path::PathBuf, std::path::PathBuf) {
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
    let mut _y = fs::read_to_string(&config).unwrap();
    _y.push_str(
        "\ninference:\n  normalization: 0.0\n  amplification: 0.0\n  invert_spect: false\n",
    );
    fs::write(&config, _y).unwrap();
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
    separator
        .separate_file(&input_path, &vpath, &ipath, cd)
        .unwrap();

    let v = read_audio(&vpath).unwrap();
    let iv = read_audio(&ipath).unwrap();
    assert_eq!(
        v.frames(),
        frames,
        "vocals length mismatch: got {} expected {}",
        v.frames(),
        frames
    );
    assert_eq!(
        iv.frames(),
        frames,
        "instrumental length mismatch: got {} expected {}",
        iv.frames(),
        frames
    );
    for i in 0..(2 * frames) {
        let sum = v.planar[i] + iv.planar[i];
        assert!(
            (planar[i] - sum).abs() < 1e-5,
            "residual too large at {}: {} vs {}",
            i,
            planar[i],
            sum
        );
    }
}

#[test]
fn chunked_boundary_parity() {
    let cfg = tiny_config();
    let dir = tmpdir("parity");
    let (model, config) = write_tiny_model(&dir, &cfg);
    let mut _y = fs::read_to_string(&config).unwrap();
    _y.push_str(
        "\ninference:\n  normalization: 0.0\n  amplification: 0.0\n  invert_spect: false\n",
    );
    fs::write(&config, _y).unwrap();
    let assets = LocalSeparationAssets::new(&model, &config);
    let separator = Separator::load(&assets, DeviceChoice::Cpu).unwrap();

    let frames = 6000usize;
    let samples: Vec<f32> = (0..frames)
        .map(|i| (i as f32 * 0.037).sin() * 0.3)
        .collect();
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
    separator
        .separate_file(&input_path, &vpath, &ipath, cd)
        .unwrap();
    let v = read_audio(&vpath).unwrap();

    let mut max_abs = 0f32;
    let mut sq = 0f64;
    let n = stems.vocals.planar.len();
    for i in 0..n {
        let d = (stems.vocals.planar[i] - v.planar[i]).abs();
        if d > max_abs {
            max_abs = d;
        }
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
    let mut _y = fs::read_to_string(&config).unwrap();
    _y.push_str(
        "\ninference:\n  normalization: 0.0\n  amplification: 0.0\n  invert_spect: false\n",
    );
    fs::write(&config, _y).unwrap();
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
    separator
        .separate_file(&input_path, &vpath, &ipath, cd)
        .unwrap();

    let v = read_audio(&vpath).unwrap();
    let iv = read_audio(&ipath).unwrap();
    assert_eq!(v.frames(), frames);
    assert_eq!(iv.frames(), frames);
    for i in 0..frames {
        let sum = v.planar[i] + iv.planar[i];
        assert!((samples[i] - sum).abs() < 1e-5);
    }
}

#[test]
fn stream_default_normalization_peaks() {
    let cfg = tiny_config();
    let dir = tmpdir("stream_norm_peaks");
    let (model, config) = write_tiny_model(&dir, &cfg);
    // Default normalized mode (normalization = 0.9).

    let assets = LocalSeparationAssets::new(&model, &config);
    let separator = Separator::load(&assets, DeviceChoice::Cpu).unwrap();

    let frames = 2000usize;
    let samples: Vec<f32> = (0..frames).map(|i| (i as f32 * 0.05).sin() * 0.5).collect();
    let audio = AudioBuffer {
        sample_rate: 44100,
        channels: 1,
        planar: samples.clone(),
    };
    let input_path = dir.join("in.wav");
    write_wav_f32(&input_path, &audio).unwrap();

    let vpath = dir.join("v.wav");
    let ipath = dir.join("i.wav");
    separator
        .separate_file(&input_path, &vpath, &ipath, 1.0)
        .unwrap();

    let v = read_audio(&vpath).unwrap();
    let iv = read_audio(&ipath).unwrap();
    let pv = v.planar.iter().map(|x| x.abs()).fold(0.0f32, f32::max);
    let pi = iv.planar.iter().map(|x| x.abs()).fold(0.0f32, f32::max);
    eprintln!("STREAM normalized peaks vocals={} instrumental={}", pv, pi);
    assert!((pv - 0.9).abs() < 1e-3, "vocals peak {} not ~0.9", pv);
    assert!((pi - 0.9).abs() < 1e-3, "instrumental peak {} not ~0.9", pi);
}

#[test]
fn stream_amplification_positive() {
    let cfg = tiny_config();
    let dir = tmpdir("stream_amp");
    let (model, config) = write_tiny_model(&dir, &cfg);
    let mut yaml = fs::read_to_string(&config).unwrap();
    yaml.push_str(
        "\ninference:\n  normalization: 0.0\n  amplification: 0.9\n  invert_spect: false\n",
    );
    fs::write(&config, yaml).unwrap();

    let assets = LocalSeparationAssets::new(&model, &config);
    let separator = Separator::load(&assets, DeviceChoice::Cpu).unwrap();

    let frames = 2000usize;
    let samples: Vec<f32> = (0..frames).map(|i| (i as f32 * 0.05).sin() * 0.1).collect();
    let before_peak = samples.iter().map(|x| x.abs()).fold(0.0f32, f32::max);
    let audio = AudioBuffer {
        sample_rate: 44100,
        channels: 1,
        planar: samples.clone(),
    };
    let input_path = dir.join("in.wav");
    write_wav_f32(&input_path, &audio).unwrap();

    let vpath = dir.join("v.wav");
    let ipath = dir.join("i.wav");
    separator
        .separate_file(&input_path, &vpath, &ipath, 1.0)
        .unwrap();

    let v = read_audio(&vpath).unwrap();
    let iv = read_audio(&ipath).unwrap();

    let mut amp_input = samples.clone();
    candle_roformer::dsp::amplify_quiet(&mut amp_input, 0.9);
    let after_peak = amp_input.iter().map(|x| x.abs()).fold(0.0f32, f32::max);

    let mut maxerr = 0.0f32;
    for i in 0..frames {
        let sum = v.planar[i] + iv.planar[i];
        maxerr = maxerr.max((amp_input[i] - sum).abs());
    }
    eprintln!(
        "STREAM amplification before_peak={} after_peak={} maxerr={}",
        before_peak, after_peak, maxerr
    );
    assert!(
        after_peak > before_peak * 2.0,
        "amplification did not raise level"
    );
    assert!(
        maxerr < 1e-5,
        "amplified residual reconstruction error too large: {}",
        maxerr
    );
}

#[test]
fn stream_spectral_inversion_matches_independent_residual() {
    let cfg = tiny_config();
    let dir = tmpdir("stream_specinv");
    let (model, config) = write_tiny_model(&dir, &cfg);
    let mut yaml = fs::read_to_string(&config).unwrap();
    yaml.push_str(
        "\ninference:\n  normalization: 0.0\n  amplification: 0.0\n  invert_spect: true\n",
    );
    fs::write(&config, yaml).unwrap();

    let assets = LocalSeparationAssets::new(&model, &config);
    let separator = Separator::load(&assets, DeviceChoice::Cpu).unwrap();

    let frames = 2000usize;
    let samples: Vec<f32> = (0..frames).map(|i| (i as f32 * 0.05).sin() * 0.5).collect();
    let audio = AudioBuffer {
        sample_rate: 44100,
        channels: 1,
        planar: samples.clone(),
    };
    let input_path = dir.join("in.wav");
    write_wav_f32(&input_path, &audio).unwrap();

    let vpath = dir.join("v.wav");
    let ipath = dir.join("i.wav");
    separator
        .separate_file(&input_path, &vpath, &ipath, 1.0)
        .unwrap();

    let iv = read_audio(&ipath).unwrap();

    // Independently compute expected spectral residual at model rate.
    let dev = candle_core::Device::Cpu;
    let vb = unsafe {
        candle_nn::VarBuilder::from_mmaped_safetensors(
            &[model.to_str().unwrap()],
            candle_core::DType::F32,
            &dev,
        )
    }
    .unwrap();
    let mdl = candle_roformer::model::MelBandRoformer::new(&cfg, vb).unwrap();
    let model_channels = 2usize;
    let mut model_input = vec![0.0f32; samples.len() * model_channels];
    for i in 0..samples.len() {
        model_input[i] = samples[i];
        model_input[samples.len() + i] = samples[i];
    }
    let mut icfg = candle_roformer::config::InferenceConfig::default();
    icfg.normalization = 0.0;
    icfg.amplification = 0.0;
    icfg.invert_spect = false;
    let outs = candle_roformer::separate::separate_all_with_config(
        &mdl,
        &model_input,
        model_channels,
        &icfg,
    )
    .unwrap();
    let vocals_model = &outs[0];
    let mv_frames = vocals_model.len() / model_channels;
    let in_frames = model_input.len() / model_channels;
    let frames_m = mv_frames.min(in_frames);

    let mut instr_model = vec![0.0f32; model_channels * frames_m];
    for ch in 0..model_channels {
        let in_ch: Vec<f32> = (0..frames_m)
            .map(|i| model_input[ch * in_frames + i])
            .collect();
        let voc_ch: Vec<f32> = (0..frames_m)
            .map(|i| vocals_model[ch * mv_frames + i])
            .collect();
        let spec_in = candle_roformer::dsp::stft(
            &in_ch,
            cfg.stft_n_fft,
            cfg.stft_hop_length,
            cfg.stft_win_length,
        )
        .unwrap();
        let spec_v = candle_roformer::dsp::stft(
            &voc_ch,
            cfg.stft_n_fft,
            cfg.stft_hop_length,
            cfg.stft_win_length,
        )
        .unwrap();
        let mut re = spec_in.re.clone();
        let mut im = spec_in.im.clone();
        for i in 0..re.len() {
            re[i] -= spec_v.re[i];
            im[i] -= spec_v.im[i];
        }
        let spec = candle_roformer::dsp::StftResult {
            re,
            im,
            frames: spec_in.frames,
            bins: spec_in.bins,
        };
        let res = candle_roformer::dsp::istft(
            &spec,
            cfg.stft_n_fft,
            cfg.stft_hop_length,
            cfg.stft_win_length,
            in_ch.len(),
        )
        .unwrap();
        for i in 0..frames_m {
            instr_model[ch * frames_m + i] = res[i];
        }
    }
    let mut mono_model = vec![0.0f32; frames_m];
    for i in 0..frames_m {
        mono_model[i] = (instr_model[i] + instr_model[frames_m + i]) * 0.5;
    }

    let mut maxdiff = 0.0f32;
    for i in 0..frames.min(frames_m) {
        let d = (iv.planar[i] - mono_model[i]).abs();
        if d > maxdiff {
            maxdiff = d;
        }
    }
    eprintln!(
        "STREAM spectral inversion maxdiff_vs_independent={}",
        maxdiff
    );
    assert!(
        maxdiff < 1e-2,
        "spectral inversion output should match independent residual (maxdiff {})",
        maxdiff
    );
}

#[test]
fn chunked_multiblock_stereo_residual() {
    let cfg = tiny_config();
    let dir = tmpdir("multiblock");
    let (model, config) = write_tiny_model(&dir, &cfg);
    let mut _y = fs::read_to_string(&config).unwrap();
    _y.push_str(
        "\ninference:\n  normalization: 0.0\n  amplification: 0.0\n  invert_spect: false\n",
    );
    fs::write(&config, _y).unwrap();
    let assets = LocalSeparationAssets::new(&model, &config);
    let separator = Separator::load(&assets, DeviceChoice::Cpu).unwrap();

    // >= 65537 frames, distinct L/R content, input sr == model sr.
    let frames = 65537usize;
    let mut planar = vec![0f32; 2 * frames];
    for i in 0..frames {
        planar[i] = (i as f32 * 0.013).sin() * 0.4;
        planar[frames + i] = (i as f32 * 0.021).cos() * 0.25;
    }
    let audio = AudioBuffer {
        sample_rate: 44100,
        channels: 2,
        planar: planar.clone(),
    };
    let input_path = dir.join("in.wav");
    write_wav_f32(&input_path, &audio).unwrap();

    // in-memory reference
    let stems = separator.separate(&audio).unwrap();

    // chunked file path
    let vpath = dir.join("v.wav");
    let ipath = dir.join("i.wav");
    let cd = 3200.0 / 44100.0;
    separator
        .separate_file(&input_path, &vpath, &ipath, cd)
        .unwrap();
    let v = read_audio(&vpath).unwrap();
    let iv = read_audio(&ipath).unwrap();

    assert_eq!(v.frames(), frames);
    assert_eq!(iv.frames(), frames);

    // exact residual
    let mut maxres = 0f32;
    for i in 0..(2 * frames) {
        let sum = v.planar[i] + iv.planar[i];
        let d = (planar[i] - sum).abs();
        if d > maxres {
            maxres = d;
        }
    }
    assert!(maxres < 1e-5, "multi-block residual too large: {}", maxres);

    // file vs in-memory
    let mut maxdiff = 0f32;
    for i in 0..stems.vocals.planar.len() {
        let d = (stems.vocals.planar[i] - v.planar[i]).abs();
        if d > maxdiff {
            maxdiff = d;
        }
        let d2 = (stems.instrumental.planar[i] - iv.planar[i]).abs();
        if d2 > maxdiff {
            maxdiff = d2;
        }
    }
    assert!(
        maxdiff < 1e-5,
        "multi-block file vs in-memory diff: {}",
        maxdiff
    );
}

#[test]
fn chunked_normalized_parity() {
    let cfg = tiny_config();
    let dir = tmpdir("norm_parity");
    let (model, config) = write_tiny_model(&dir, &cfg);
    // Default inference: normalization = 0.9 (no yaml override).
    let assets = LocalSeparationAssets::new(&model, &config);
    let separator = Separator::load(&assets, DeviceChoice::Cpu).unwrap();

    let frames = 6000usize;
    let samples: Vec<f32> = (0..frames)
        .map(|i| (i as f32 * 0.037).sin() * 0.3)
        .collect();
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

    let stems = separator.separate(&audio).unwrap();

    let vpath = dir.join("v.wav");
    let ipath = dir.join("i.wav");
    let cd = 3200.0 / 44100.0;
    separator
        .separate_file(&input_path, &vpath, &ipath, cd)
        .unwrap();
    let v = read_audio(&vpath).unwrap();
    let iv = read_audio(&ipath).unwrap();

    let mut maxv = 0f32;
    let mut maxi = 0f32;
    for i in 0..stems.vocals.planar.len() {
        maxv = maxv.max((stems.vocals.planar[i] - v.planar[i]).abs());
        maxi = maxi.max((stems.instrumental.planar[i] - iv.planar[i]).abs());
    }
    eprintln!(
        "NORMALIZED PARITY max_abs vocals={} instrumental={}",
        maxv, maxi
    );
    assert!(maxv <= 1e-4, "vocals parity too large: {}", maxv);
    assert!(maxi <= 1e-4, "instrumental parity too large: {}", maxi);
}
