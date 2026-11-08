//! Public API tests: the crate is the product, the CLI lives in `karaoke-cli`.

use std::fs;

use candle_core::{DType, Device};
use candle_nn::{VarBuilder, VarMap};
use candle_roformer::config::SeparatorConfig;
use candle_roformer::model::MelBandRoformer;
use candle_roformer::wav::AudioBuffer;
use candle_roformer::{DeviceChoice, LocalSeparationAssets, Separator};

fn tmpdir(name: &str) -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("cr_libapi_{}", name));
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
    let dev = Device::Cpu;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &dev);
    let _ = MelBandRoformer::new(cfg, vb).unwrap();
    let model = dir.join("tiny.safetensors");
    varmap.save(&model).unwrap();

    let yaml = format!(
        "model:\n  dim: {}\n  depth: {}\n  stereo: true\n  num_stems: 1\n  time_transformer_depth: 1\n  freq_transformer_depth: 1\n  num_bands: {}\n  dim_head: {}\n  heads: {}\n  flash_attn: false\n  dim_freqs_in: {}\n  sample_rate: 44100\n  stft_n_fft: {}\n  stft_hop_length: {}\n  stft_win_length: {}\n  stft_normalized: false\n  mask_estimator_depth: 2\n  use_torch_checkpoint: false\n  mlp_expansion_factor: 4\n  linear_transformer_depth: 0\n  norm_output: true\n  skip_connection: false\n  zero_dc: true\n",
        cfg.dim,
        cfg.depth,
        cfg.num_bands,
        cfg.dim_head,
        cfg.heads,
        cfg.dim_freqs_in,
        cfg.stft_n_fft,
        cfg.stft_hop_length,
        cfg.stft_win_length
    );
    let config = dir.join("tiny.yaml");
    fs::write(&config, yaml).unwrap();
    (model, config)
}

#[test]
fn separator_loads_and_separates_in_memory() {
    let cfg = tiny_config();
    let dir = tmpdir("stems");
    let (model, config) = write_tiny_model(&dir, &cfg);

    let assets = LocalSeparationAssets::new(&model, &config);
    let separator = Separator::load(&assets, DeviceChoice::Cpu).unwrap();
    assert_eq!(separator.config().dim, cfg.dim);

    let frames = 2000usize;
    let samples: Vec<f32> = (0..frames).map(|i| (i as f32 * 0.05).sin() * 0.5).collect();
    let audio = AudioBuffer {
        sample_rate: 44100,
        channels: 1,
        planar: samples.clone(),
    };

    let stems = separator.separate(&audio).unwrap();
    assert_eq!(stems.vocals.sample_rate, 44100);
    assert_eq!(stems.vocals.channels, 1);
    assert_eq!(stems.vocals.frames(), frames);
    assert_eq!(stems.instrumental.frames(), frames);
    for i in 0..frames {
        let sum = stems.vocals.planar[i] + stems.instrumental.planar[i];
        assert!((samples[i] - sum).abs() < 1e-5);
    }
}

#[test]
fn separator_rejects_checkpoint_mismatch() {
    let cfg = tiny_config();
    let dir = tmpdir("mismatch");
    let (model, config) = write_tiny_model(&dir, &cfg);

    // A different profile cannot match the checkpoint: loading must fail, not
    // run with garbage weights.
    let other = SeparatorConfig {
        num_bands: 5,
        ..cfg.clone()
    };
    let other_config = dir.join("other.yaml");
    let yaml = fs::read_to_string(&config).unwrap().replace("num_bands: 4", "num_bands: 5");
    fs::write(&other_config, yaml).unwrap();
    let _ = other;

    let assets = LocalSeparationAssets::new(&model, &other_config);
    let err = Separator::load(&assets, DeviceChoice::Cpu);
    assert!(err.is_err(), "mismatched profile must be rejected");
}

#[test]
fn audio_buffer_validation() {
    let ok = AudioBuffer { sample_rate: 44100, channels: 2, planar: vec![0.0; 100] };
    assert!(ok.validate().is_ok());

    let bad_rate = AudioBuffer { sample_rate: 0, channels: 1, planar: vec![0.0; 10] };
    assert!(bad_rate.validate().is_err());

    let bad_channels = AudioBuffer { sample_rate: 44100, channels: 3, planar: vec![0.0; 30] };
    assert!(bad_channels.validate().is_err());

    let bad_len = AudioBuffer { sample_rate: 44100, channels: 2, planar: vec![0.0; 31] };
    assert!(bad_len.validate().is_err());

    let non_finite = AudioBuffer { sample_rate: 44100, channels: 1, planar: vec![f32::NAN] };
    assert!(non_finite.validate().is_err());
}
