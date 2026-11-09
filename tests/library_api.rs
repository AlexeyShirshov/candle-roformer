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

fn tiny_config_n(num_stems: usize) -> SeparatorConfig {
    let mut cfg = tiny_config();
    cfg.num_stems = num_stems;
    cfg
}

fn write_model(dir: &std::path::Path, cfg: &SeparatorConfig) -> (std::path::PathBuf, std::path::PathBuf) {
    let dev = Device::Cpu;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &dev);
    let _ = MelBandRoformer::new(cfg, vb).unwrap();
    let model = dir.join("tiny.safetensors");
    varmap.save(&model).unwrap();

    let yaml = format!(
        "model:\n  dim: {}\n  depth: {}\n  stereo: true\n  num_stems: {}\n  time_transformer_depth: 1\n  freq_transformer_depth: 1\n  num_bands: {}\n  dim_head: {}\n  heads: {}\n  flash_attn: false\n  dim_freqs_in: {}\n  sample_rate: 44100\n  stft_n_fft: {}\n  stft_hop_length: {}\n  stft_win_length: {}\n  stft_normalized: false\n  mask_estimator_depth: 2\n  use_torch_checkpoint: false\n  mlp_expansion_factor: 4\n  linear_transformer_depth: 0\n  norm_output: true\n  skip_connection: false\n  zero_dc: true\n",
        cfg.dim,
        cfg.depth,
        cfg.num_stems,
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
fn four_stem_path_returns_four_named_buffers() {
    let cfg = tiny_config_n(4);
    let dir = tmpdir("four");
    let (model, config) = write_model(&dir, &cfg);

    let assets = LocalSeparationAssets::new(&model, &config);
    let separator = Separator::load(&assets, DeviceChoice::Cpu).unwrap();
    assert_eq!(separator.config().num_stems, 4);

    let frames = 128usize;
    let samples: Vec<f32> = (0..frames).map(|i| (i as f32 * 0.05).sin() * 0.5).collect();
    let audio = AudioBuffer {
        sample_rate: 44100,
        channels: 1,
        planar: samples.clone(),
    };

    let stems = separator.separate(&audio).unwrap();
    assert_eq!(stems.len(), 4);
    assert_eq!(stems.names().len(), 4);
    for name in ["drums", "bass", "other", "vocals"] {
        let buf = stems.get(name).expect(name);
        assert_eq!(buf.sample_rate, 44100);
        assert_eq!(buf.channels, 1);
        assert_eq!(buf.frames(), frames);
    }
}

#[test]
fn residual_and_independent_modes_are_distinct() {
    let frames = 128usize;
    let samples: Vec<f32> = (0..frames).map(|i| (i as f32 * 0.05).sin() * 0.5).collect();
    let audio = AudioBuffer {
        sample_rate: 44100,
        channels: 1,
        planar: samples.clone(),
    };

    // Residual mode (num_stems == 1): instrumental is input - vocals.
    let cfg1 = tiny_config();
    let dir1 = tmpdir("resid");
    let (m1, c1) = write_model(&dir1, &cfg1);
    let sep1 = Separator::load(&LocalSeparationAssets::new(&m1, &c1), DeviceChoice::Cpu).unwrap();
    let s1 = sep1.separate(&audio).unwrap();
    assert_eq!(s1.len(), 2);
    assert!(s1.get("vocals").is_some());
    assert!(s1.get("instrumental").is_some());
    for i in 0..frames {
        let sum = s1.vocals.planar[i] + s1.instrumental.planar[i];
        assert!((samples[i] - sum).abs() < 1e-5);
    }

    // Independent mode (num_stems > 1): no residual substitution; sum need not
    // equal the input.
    let cfg4 = tiny_config_n(4);
    let dir4 = tmpdir("indep");
    let (m4, c4) = write_model(&dir4, &cfg4);
    let sep4 = Separator::load(&LocalSeparationAssets::new(&m4, &c4), DeviceChoice::Cpu).unwrap();
    let s4 = sep4.separate(&audio).unwrap();
    assert_eq!(s4.len(), 4);
    let mut any_mismatch = false;
    for i in 0..frames {
        let sum = s4.vocals.planar[i] + s4.instrumental.planar[i];
        if (samples[i] - sum).abs() > 1e-6 {
            any_mismatch = true;
            break;
        }
    }
    assert!(any_mismatch, "independent mode must not be a pure residual");
}

#[test]
fn four_stem_buffers_are_independent_not_residual_substitutions() {
    // Distinct, non-degenerate input: a sum of sinusoids at several
    // frequencies plus a linear ramp, so that no stem collapses to zero and a
    // residual `input - other` is distinguishable in every position.
    let frames = 128usize;
    let samples: Vec<f32> = (0..frames)
        .map(|i| {
            let t = i as f32;
            0.25 * (t * 0.11).sin()
                + 0.2 * (t * 0.37).cos()
                + 0.15 * (t * 0.71).sin()
                + 0.1 * (t * 1.13).cos()
                + 0.05 * t
        })
        .collect();
    assert!(samples.iter().any(|x| x.abs() > 1e-6));
    assert!(samples.iter().all(|x| x.is_finite()));

    let cfg = tiny_config_n(4);
    let dir = tmpdir("indep_proof");
    let (model_path, config) = write_model(&dir, &cfg);

    // Load the model independently (same path as Separator::load) to verify the
    // model-output layout that the overlap-add relies on: num_stems * channels *
    // chunk samples, batch-major then stem-major.
    let dev = candle_core::Device::Cpu;
    let vb = unsafe { candle_nn::VarBuilder::from_mmaped_safetensors(&[model_path.to_str().unwrap()], candle_core::DType::F32, &dev) }.unwrap();
    let mdl = candle_roformer::model::MelBandRoformer::new(&cfg, vb).unwrap();
    let chunk = candle_roformer::separate::chunk_size_for(&mdl);
    // Non-degenerate padded input for the independent-output proof: distinct
    // non-zero content (not a zero buffer), so a cached/zero result cannot pass.
    let padded: Vec<f32> = (0..2 * chunk)
        .map(|i| (i as f32 * 0.013).sin() * 0.5 + 0.1)
        .collect();
    assert!(padded.iter().any(|x| x.abs() > 1e-6));
    // Independently compute the model output by calling model.forward directly
    // (the reference, independent of infer_chunk).
    let t = candle_core::Tensor::from_vec(padded.clone(), (1usize, 2usize, chunk), &dev).unwrap();
    let out = mdl.forward(&t).unwrap();
    let independent: Vec<f32> = out.flatten_all().unwrap().to_vec1().unwrap();
    assert_eq!(independent.len(), 4 * 2 * chunk, "independent model output must be 4 stems flattened");
    // infer_chunk (non-TTA) must reproduce the independently computed model
    // output exactly. Kills: "infer_chunk returns zeros/cached/wrong layout".
    let raw = candle_roformer::separate::infer_chunk(&mdl, &candle_roformer::config::InferenceConfig::default(), &padded, 2, chunk)
        .expect("infer_chunk");
    assert_eq!(raw.len(), independent.len());
    for (a, b) in raw.iter().zip(independent.iter()) {
        assert!((a - b).abs() < 1e-6, "infer_chunk must equal independent model output");
    }

    let sep = Separator::load(&LocalSeparationAssets::new(&model_path, &config), DeviceChoice::Cpu).unwrap();
    let audio = AudioBuffer {
        sample_rate: 44100,
        channels: 1,
        planar: samples.clone(),
    };
    let stems = sep.separate(&audio).unwrap();
    assert_eq!(stems.len(), 4);
    let names = ["drums", "bass", "other", "vocals"];
    for name in names {
        let b = stems.get(name).expect(name);
        assert_eq!(b.frames(), frames);
        assert!(b.planar.iter().all(|x| x.is_finite()));
        assert!(
            b.planar.iter().any(|x| x.abs() > 1e-8),
            "{} must be non-degenerate",
            name
        );
    }

    // Independent model-output proof: each named buffer must equal the
    // corresponding stem produced by the model on the same input (padded to a
    // chunk, mono duplicated to stereo, downmixed back to mono). This proves the
    // named buffers are real model outputs, not fabricated residuals.
    // Kills: "separator substitutes residuals for model outputs".
    let chunk = candle_roformer::separate::chunk_size_for(&mdl);
    let mut padded_stereo = vec![0.0f32; 2 * chunk];
    for i in 0..frames {
        padded_stereo[i] = samples[i];
        padded_stereo[chunk + i] = samples[i];
    }
    let t2 = candle_core::Tensor::from_vec(padded_stereo, (1usize, 2usize, chunk), &dev).unwrap();
    let out2 = mdl.forward(&t2).unwrap();
    let out2v: Vec<f32> = out2.flatten_all().unwrap().to_vec1().unwrap();
    for (si, name) in names.iter().enumerate() {
        let buf = stems.get(name).unwrap();
        for i in 0..frames {
            let ch0 = out2v[(si * 2 + 0) * chunk + i];
            let ch1 = out2v[(si * 2 + 1) * chunk + i];
            let expected = (ch0 + ch1) * 0.5;
            assert!(
                (buf.planar[i] - expected).abs() < 1e-3,
                "{} frame {} does not match independent model output ({} vs {})",
                name, i, buf.planar[i], expected
            );
        }
    }

    // Collect the four buffers in named order.
    let bufs: Vec<&AudioBuffer> = names.iter().map(|n| stems.get(n).unwrap()).collect();

    // Pairwise distinctness: no two stems may be (approximately) equal.
    for i in 0..4 {
        for j in (i + 1)..4 {
            let maxdiff = bufs[i]
                .planar
                .iter()
                .zip(bufs[j].planar.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f32, f32::max);
            assert!(
                maxdiff > 1e-3,
                "{} and {} are indistinguishable (maxdiff {})",
                names[i],
                names[j],
                maxdiff
            );
        }
    }

    // Residual-substitution detection: if any position were silently replaced by
    // `input - other` (single other stem) or `input - sum(others)`, the
    // resulting buffer would match that residual. Genuine independent model
    // outputs must not.
    for i in 0..4 {
        // single-other residual candidates
        for j in 0..4 {
            if i == j {
                continue;
            }
            let residual: Vec<f32> = samples
                .iter()
                .zip(bufs[j].planar.iter())
                .map(|(x, y)| x - y)
                .collect();
            let maxdiff = bufs[i]
                .planar
                .iter()
                .zip(residual.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f32, f32::max);
            assert!(
                maxdiff > 1e-3,
                "stem {} matches residual input - {} (maxdiff {}) — would indicate \
                 silent residual substitution",
                names[i],
                names[j],
                maxdiff
            );
        }
        // sum-of-others residual candidate
        let residual: Vec<f32> = (0..frames)
            .map(|k| {
                let others: f32 = (0..4)
                    .filter(|&s| s != i)
                    .map(|s| bufs[s].planar[k])
                    .sum();
                samples[k] - others
            })
            .collect();
        let maxdiff = bufs[i]
            .planar
            .iter()
            .zip(residual.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f32, f32::max);
        assert!(
            maxdiff > 1e-3,
            "stem {} matches residual input - sum(others) (maxdiff {}) — would \
             indicate silent residual substitution",
            names[i],
            maxdiff
        );
    }
}

#[test]
fn four_stem_tta_path_returns_four_named_buffers() {
    let cfg = tiny_config_n(4);
    let dir = tmpdir("four_tta");
    let (model, config) = write_model(&dir, &cfg);
    // Enable TTA through the public inference configuration (YAML path).
    let mut yaml = fs::read_to_string(&config).unwrap();
    yaml.push_str("\ninference:\n  tta: true\n");
    fs::write(&config, yaml).unwrap();

    let assets = LocalSeparationAssets::new(&model, &config);
    let separator = Separator::load(&assets, DeviceChoice::Cpu).unwrap();
    assert!(separator.inference_config().tta, "TTA must be enabled via config");

    let frames = 128usize;
    let samples: Vec<f32> = (0..frames).map(|i| (i as f32 * 0.05).sin() * 0.5).collect();
    let audio = AudioBuffer {
        sample_rate: 44100,
        channels: 1,
        planar: samples,
    };

    // Public path must succeed (overlap-add accepts the stem-expanded TTA output).
    let stems = separator.separate(&audio).unwrap();
    assert_eq!(stems.len(), 4);
    assert_eq!(stems.names(), vec!["drums", "bass", "other", "vocals"]);
    for name in ["drums", "bass", "other", "vocals"] {
        let buf = stems.get(name).expect(name);
        assert_eq!(buf.sample_rate, 44100);
        assert_eq!(buf.channels, 1);
        assert_eq!(buf.frames(), frames);
    }

    // Stem-expanded output length through infer_chunk with TTA enabled.
    let dev = candle_core::Device::Cpu;
    let vb = unsafe {
        candle_nn::VarBuilder::from_mmaped_safetensors(&[model.to_str().unwrap()], candle_core::DType::F32, &dev)
    }
    .unwrap();
    let mdl = candle_roformer::model::MelBandRoformer::new(&cfg, vb).unwrap();
    let chunk = candle_roformer::separate::chunk_size_for(&mdl);
    let mut icfg = candle_roformer::config::InferenceConfig::default();
    icfg.tta = true;
    let padded: Vec<f32> = (0..2 * chunk)
        .map(|i| (i as f32 * 0.013).sin() * 0.5 + 0.1)
        .collect();
    let raw = candle_roformer::separate::infer_chunk(&mdl, &icfg, &padded, 2, chunk)
        .expect("infer_chunk tta");
    assert_eq!(raw.len(), 4 * 2 * chunk, "TTA must return stem-expanded output");
}
