//! BS-RoFormer family: profile parsing, key schema, loader dispatch and the
//! reference-faithful chunk schedule.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use candle_core::{Device, Tensor};
use candle_roformer::config::{InferenceConfig, ModelFamily, SeparatorConfig};
use candle_roformer::model::bs_roformer::expected_key_shapes;
use candle_roformer::{AudioBuffer, DeviceChoice, LocalSeparationAssets, Separator};

fn published_profile() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("configs/bs_roformer_viperx_1297.yaml")
}

#[test]
fn bs_profile_parses_from_published_yaml() {
    let cfg = SeparatorConfig::from_yaml_file(published_profile()).expect("parse");
    cfg.validate().expect("validate");
    assert_eq!(cfg.family(), ModelFamily::BSRoformer);
    assert_eq!(cfg.dim, 512);
    assert_eq!(cfg.depth, 12);
    assert!(cfg.stereo);
    assert_eq!(cfg.num_stems, 1);
    assert_eq!(cfg.dim_head, 64);
    assert_eq!(cfg.heads, 8);
    let freqs = cfg.freqs_per_bands.as_ref().expect("freqs_per_bands");
    assert_eq!(freqs.len(), 62);
    assert_eq!(freqs.iter().sum::<usize>(), 1025);
    assert_eq!(freqs[freqs.len() - 1], 129);
    // The sample rate lives in the `audio:` block of the published profile.
    assert_eq!(cfg.sample_rate, 44100);
    assert_eq!(cfg.stft_n_fft, 2048);
    assert_eq!(cfg.stft_hop_length, 441);
    assert_eq!(cfg.stft_win_length, 2048);
    // No mel band count in a BS profile; the family is structural.
    assert_eq!(cfg.num_bands, 0);

    let inference = InferenceConfig::from_yaml_file(published_profile()).expect("inference");
    assert_eq!(inference.num_overlap, 4);
    assert_eq!(inference.sample_rate, 44100);
}

#[test]
fn python_tuple_tag_and_audio_sample_rate_are_tolerated() {
    let yaml = "\
audio:
  sample_rate: 48000
model:
  family: bs_roformer
  dim: 8
  depth: 1
  stereo: true
  num_stems: 1
  time_transformer_depth: 1
  freq_transformer_depth: 1
  freqs_per_bands: !!python/tuple
    - 2
    - 3
  dim_head: 4
  heads: 2
  dim_freqs_in: 5
  stft_n_fft: 8
  stft_hop_length: 2
  stft_win_length: 8
  mask_estimator_depth: 2
";
    let cfg = SeparatorConfig::from_yaml_str(yaml).expect("parse");
    cfg.validate().expect("validate");
    assert_eq!(cfg.family(), ModelFamily::BSRoformer);
    assert_eq!(cfg.freqs_per_bands.as_deref(), Some(&[2usize, 3][..]));
    assert_eq!(cfg.sample_rate, 48000);
}

#[test]
fn mel_band_and_bs_families_are_distinguished() {
    let mel = SeparatorConfig::v7();
    assert_eq!(mel.family(), ModelFamily::MelBandRoformer);
    let bs = SeparatorConfig::from_yaml_file(published_profile()).expect("parse");
    assert_eq!(bs.family(), ModelFamily::BSRoformer);
    // An explicit family key wins over the structural inference.
    let mut forced = mel.clone();
    forced.family = Some(ModelFamily::BSRoformer);
    assert_eq!(forced.family(), ModelFamily::BSRoformer);
}

#[test]
fn bs_expected_keys_match_the_published_checkpoint_schema() {
    let cfg = SeparatorConfig::from_yaml_file(published_profile()).expect("parse");
    let keys = expected_key_shapes(&cfg);
    // The published `model_bs_roformer_ep_317_sdr_12.9755` state dict has 699
    // tensors; the derived schema must have exactly as many, with no duplicates.
    let unique: std::collections::HashSet<&String> = keys.iter().map(|(n, _)| n).collect();
    assert_eq!(unique.len(), keys.len(), "schema keys must be unique");

    let count = |prefix: &str| keys.iter().filter(|(n, _)| n.starts_with(prefix)).count();
    assert_eq!(count("band_split."), 62 * 3);
    assert_eq!(count("mask_estimators."), 62 * 2 * 2);
    assert_eq!(count("final_norm."), 1);
    let rotary = keys
        .iter()
        .filter(|(n, _)| n.ends_with("rotary_embed.freqs"))
        .count();
    assert_eq!(rotary, 12 * 2, "one rotary table per attention block");
    assert_eq!(keys.len(), 699);

    let map: HashMap<&str, &Vec<usize>> = keys.iter().map(|(n, s)| (n.as_str(), s)).collect();
    assert_eq!(
        map["layers.11.1.layers.0.0.to_qkv.weight"].as_slice(),
        [3 * 8 * 64, 512]
    );
    assert_eq!(
        map["layers.0.0.layers.0.0.rotary_embed.freqs"].as_slice(),
        [32]
    );
    assert_eq!(
        map["mask_estimators.0.to_freqs.61.0.2.weight"].as_slice(),
        [2 * (2 * 129 * 2), 2048]
    );
    assert_eq!(
        map["mask_estimators.0.to_freqs.0.0.2.weight"].as_slice(),
        [2 * (2 * 2 * 2), 2048]
    );
    assert_eq!(
        map["band_split.to_features.61.1.weight"].as_slice(),
        [512, 2 * 129 * 2]
    );
}

#[test]
fn ola_schedule_matches_the_reference_windows() {
    use std::cell::{Cell, RefCell};

    let chunk = 3200usize;
    let step = 1600usize;
    let fade = 320usize;
    let signal =
        |n: usize| -> Vec<f32> { (0..n).map(|i| (i as f32 * 0.001).sin() * 0.5).collect() };

    // Exactly one chunk of audio: a single window, so the envelope cancels and
    // the output equals the model output (the reference emits no extra window).
    let n = chunk;
    let input = signal(n);
    let calls = Cell::new(0usize);
    let outs = candle_roformer::separate::ola_all_with_batch(
        &input,
        1,
        chunk,
        step,
        fade,
        1,
        1,
        &|pcm: &[f32]| {
            calls.set(calls.get() + 1);
            Ok(pcm.to_vec())
        },
    )
    .unwrap();
    assert_eq!(calls.get(), 1, "a chunk-length input must be one window");
    let maxdiff = outs[0]
        .iter()
        .zip(input.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);
    assert!(
        maxdiff < 1e-5,
        "single-window envelope must cancel: {maxdiff}"
    );

    // Exact multiple of the step: the windows march, with no extra tail window.
    let n = 2 * chunk;
    let input = signal(n);
    let starts: RefCell<Vec<usize>> = RefCell::new(Vec::new());
    let cursor = Cell::new(0usize);
    let _ = candle_roformer::separate::ola_all_with_batch(
        &input,
        1,
        chunk,
        step,
        fade,
        1,
        1,
        &|pcm: &[f32]| {
            starts.borrow_mut().push(cursor.get());
            cursor.set(cursor.get() + step);
            Ok(vec![0f32; pcm.len()])
        },
    )
    .unwrap();
    let n_calls = starts.borrow().len();
    assert_eq!(n_calls, 3, "starts must be 0, step, 2*step");

    // Remainder: the final window is shifted so that it ends at the input end
    // (sample 5000 - 3200 = 1800) instead of being a partial window at 3200.
    let n = 5000usize;
    let input = signal(n);
    let first_samples: RefCell<Vec<f32>> = RefCell::new(Vec::new());
    let _ = candle_roformer::separate::ola_all_with_batch(
        &input,
        1,
        chunk,
        step,
        fade,
        1,
        1,
        &|pcm: &[f32]| {
            first_samples.borrow_mut().push(pcm[0]);
            Ok(vec![0f32; pcm.len()])
        },
    )
    .unwrap();
    let got = first_samples.borrow();
    assert_eq!(got.len(), 3, "windows at 0, step and n - chunk");
    assert!((got[0] - input[0]).abs() < 1e-6);
    assert!((got[1] - input[1600]).abs() < 1e-6);
    assert!(
        (got[2] - input[1800]).abs() < 1e-6,
        "tail window must start at n - chunk"
    );
}

// ---------- tiny end-to-end dispatch test ----------

const TINY_BS_YAML: &str = "\
model:
  family: bs_roformer
  dim: 8
  depth: 1
  stereo: true
  num_stems: 1
  time_transformer_depth: 1
  freq_transformer_depth: 1
  freqs_per_bands: !!python/tuple
    - 2
    - 2
    - 2
    - 3
  dim_head: 4
  heads: 2
  flash_attn: true
  dim_freqs_in: 9
  stft_n_fft: 16
  stft_hop_length: 4
  stft_win_length: 16
  mask_estimator_depth: 2
  mlp_expansion_factor: 2
  linear_transformer_depth: 0
audio:
  sample_rate: 44100
inference:
  num_overlap: 2
";

fn tmpdir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "candle-roformer-bs-{}-{}-{}",
        tag,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Deterministic small fill so the forward is not degenerate (all zeros would
/// pass a finiteness check trivially, but a non-zero input is a better probe).
fn seeded(shape: &[usize], seed: u64, scale: f32) -> Tensor {
    let n: usize = shape.iter().product();
    let mut state = seed | 1;
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let unit = ((state >> 33) as f32) / (u32::MAX >> 1) as f32;
        v.push((unit - 0.5) * 2.0 * scale);
    }
    Tensor::from_vec(v, shape, &Device::Cpu).expect("tensor")
}

fn write_tiny_checkpoint(dir: &Path, drop_one: bool, add_extra: bool) -> (PathBuf, PathBuf, usize) {
    let config = dir.join("bs-tiny.yaml");
    std::fs::write(&config, TINY_BS_YAML).expect("write config");
    let cfg = SeparatorConfig::from_yaml_str(TINY_BS_YAML).expect("parse tiny");
    let schema = expected_key_shapes(&cfg);
    let mut tensors: HashMap<String, Tensor> = HashMap::new();
    for (i, (name, shape)) in schema.iter().enumerate() {
        if drop_one && i == 0 {
            continue;
        }
        let is_norm = name.ends_with("gamma");
        let t = if is_norm {
            Tensor::ones(shape.clone(), candle_core::DType::F32, &Device::Cpu).expect("ones")
        } else {
            seeded(shape, i as u64 + 1, 0.05)
        };
        tensors.insert(name.clone(), t);
    }
    if add_extra {
        tensors.insert(
            "not_a_module.weight".to_string(),
            seeded(&[2, 2], 999, 0.05),
        );
    }
    let weights = dir.join("bs-tiny.safetensors");
    candle_core::safetensors::save(&tensors, &weights).expect("save safetensors");
    (weights, config, schema.len())
}

#[test]
fn bs_tiny_model_loads_strictly_and_separates() {
    let dir = tmpdir("tiny");
    let (weights, config, key_count) = write_tiny_checkpoint(&dir, false, false);
    assert_eq!(key_count, 51, "tiny schema size");

    let assets = LocalSeparationAssets::new(&weights, &config);
    let separator = Separator::load(&assets, DeviceChoice::Cpu).expect("load");
    assert_eq!(separator.config().family(), ModelFamily::BSRoformer);
    assert_eq!(separator.config().num_stems, 1);

    let frames = 64usize;
    let planar: Vec<f32> = (0..2 * frames)
        .map(|i| ((i as f32) * 0.07).sin() * 0.4)
        .collect();
    let audio = AudioBuffer {
        sample_rate: 44100,
        channels: 2,
        planar,
    };
    let stems = separator.separate(&audio).expect("separate");
    assert_eq!(stems.vocals.frames(), frames);
    assert_eq!(stems.vocals.channels, 2);
    assert!(stems.vocals.planar.iter().all(|x| x.is_finite()));
    // The BS forward must actually compute a mask, not return silence.
    assert!(stems.vocals.planar.iter().any(|x| x.abs() > 1e-9));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn bs_missing_key_is_rejected_by_the_loader() {
    let dir = tmpdir("missing");
    let (weights, config, _) = write_tiny_checkpoint(&dir, true, false);
    let assets = LocalSeparationAssets::new(&weights, &config);
    let err = match Separator::load(&assets, DeviceChoice::Cpu) {
        Ok(_) => panic!("loader must reject a checkpoint with a missing key"),
        Err(e) => e.to_string(),
    };
    assert!(err.contains("missing"), "unexpected error: {}", err);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn bs_extra_key_is_rejected_by_the_loader() {
    let dir = tmpdir("extra");
    let (weights, config, _) = write_tiny_checkpoint(&dir, false, true);
    let assets = LocalSeparationAssets::new(&weights, &config);
    let err = match Separator::load(&assets, DeviceChoice::Cpu) {
        Ok(_) => panic!("loader must reject a checkpoint with an unexpected key"),
        Err(e) => e.to_string(),
    };
    assert!(err.contains("extra"), "unexpected error: {}", err);
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------- real-checkpoint strict validation (opt-in) ----------

/// Strict validation of a real converted BS-RoFormer checkpoint against the
/// crate's own [`expected_key_shapes`] schema, followed by the loader path.
///
/// Ignored by default: it needs the ~640 MB published weight file. The
/// published `.ckpt` is converted offline by `tools/convert_bs_roformer.py`;
/// this test pins the converted safetensors and its YAML profile:
///
/// ```sh
/// CANDLE_ROFORMER_BS_SAFETENSORS=/models/model_bs_roformer_ep_317_sdr_12.9755.safetensors \
/// CANDLE_ROFORMER_BS_CONFIG=/models/model_bs_roformer_ep_317_sdr_12.9755.yaml \
///   cargo test --release --test bs_roformer -- --ignored --nocapture \
///     real_checkpoint_passes_strict_validation
/// ```
#[test]
#[ignore = "needs the published 640 MB checkpoint: set CANDLE_ROFORMER_BS_SAFETENSORS"]
fn real_checkpoint_passes_strict_validation() {
    use candle_core::safetensors::{Load, MmapedSafetensors};

    let weights = std::env::var("CANDLE_ROFORMER_BS_SAFETENSORS")
        .expect("set CANDLE_ROFORMER_BS_SAFETENSORS to the converted safetensors file");
    let config = std::env::var("CANDLE_ROFORMER_BS_CONFIG")
        .map(PathBuf::from)
        .unwrap_or_else(|_| published_profile());
    let cfg = SeparatorConfig::from_yaml_file(&config).expect("parse config");
    let schema = expected_key_shapes(&cfg);
    let expected: HashMap<&str, &Vec<usize>> =
        schema.iter().map(|(n, s)| (n.as_str(), s)).collect();

    let ms = unsafe { MmapedSafetensors::multi(&[weights.as_str()]) }.expect("mmap checkpoint");
    let names: Vec<String> = ms.tensors().iter().map(|(n, _)| n.clone()).collect();
    let actual: std::collections::HashSet<&str> = names.iter().map(|s| s.as_str()).collect();

    let mut missing: Vec<&str> = expected
        .keys()
        .copied()
        .filter(|n| !actual.contains(n))
        .collect();
    let mut unexpected: Vec<&str> = actual
        .iter()
        .copied()
        .filter(|n| !expected.contains_key(n))
        .collect();
    missing.sort_unstable();
    unexpected.sort_unstable();

    let dev = Device::Cpu;
    let mut shape_mismatches: Vec<String> = Vec::new();
    for (name, shape) in &schema {
        let t = ms
            .get(name)
            .expect("schema key present in file")
            .load(&dev)
            .expect("load tensor");
        if t.dims() != shape.as_slice() {
            shape_mismatches.push(format!(
                "{}: file {:?} vs expected {:?}",
                name,
                t.dims(),
                shape
            ));
        }
    }

    println!(
        "strict-validation checkpoint={} bytes={} config={} expected={} actual={} missing={} unexpected={} shape_mismatches={}",
        weights,
        std::fs::metadata(&weights).map(|m| m.len()).unwrap_or(0),
        config.display(),
        schema.len(),
        names.len(),
        missing.len(),
        unexpected.len(),
        shape_mismatches.len()
    );
    assert!(missing.is_empty(), "missing keys: {:?}", missing);
    assert!(unexpected.is_empty(), "unexpected keys: {:?}", unexpected);
    assert!(
        shape_mismatches.is_empty(),
        "shape mismatches: {:?}",
        shape_mismatches
    );
    assert_eq!(schema.len(), 699, "published BS-RoFormer schema size");

    // `Separator::load` runs the same check before allocating the model.
    let assets = LocalSeparationAssets::new(&weights, &config);
    let separator = Separator::load(&assets, DeviceChoice::Cpu).expect("strict load");
    assert_eq!(separator.config().family(), ModelFamily::BSRoformer);
    assert_eq!(separator.config().num_stems, 1);
    println!("strict-load: ok");
}
