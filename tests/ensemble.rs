//! Behaviour of the ensemble operators and preset resolution.
//!
//! These tests assert the documented semantics of issue #8's `avg_wave` /
//! `median_wave` operators (per-sample mean; per-sample median across models,
//! with zero-padding of shorter waveforms and no trimming or normalization) and
//! the documented composition of the `vocal_balanced` preset. They use only
//! synthetic in-memory buffers, so they run without any checkpoint.
//!
//! `ensemble_matches_audio_separator_reference` is the `#[ignore]`d parity run
//! against the frozen `audio-separator` 0.48.0 fixture; see
//! `docs/ensemble-presets.md`.

use candle_roformer::{
    ensemble_waveforms, EnsembleAlgorithm, EnsembleError, EnsemblePreset, Error, WavError,
};

fn buffer(sample_rate: u32, channels: u16, planar: Vec<f32>) -> candle_roformer::AudioBuffer {
    candle_roformer::AudioBuffer {
        sample_rate,
        channels,
        planar,
    }
}

fn ensemble_error(err: Error) -> EnsembleError {
    match err {
        Error::Ensemble(e) => e,
        other => panic!("expected a typed ensemble error, got {other:?}"),
    }
}

#[test]
fn avg_wave_equal_weights_is_the_per_sample_mean() {
    let a = buffer(44_100, 1, vec![0.5, -0.5, 0.25, 0.0]);
    let b = buffer(44_100, 1, vec![0.25, -0.25, -0.75, 1.0]);
    let out = ensemble_waveforms(EnsembleAlgorithm::AvgWave, None, &[a.clone(), b.clone()])
        .expect("avg_wave");
    assert_eq!(out.sample_rate, 44_100);
    assert_eq!(out.channels, 1);
    assert_eq!(out.planar.len(), 4);
    for i in 0..4 {
        let expected = ((0.0f32 + a.planar[i]) + b.planar[i]) / 2.0;
        assert_eq!(out.planar[i], expected, "sample {i}");
    }
}

#[test]
fn avg_wave_weights_divide_by_the_weight_sum() {
    let a = buffer(44_100, 1, vec![0.5, -0.5, 0.25]);
    let b = buffer(44_100, 1, vec![0.25, -0.25, -0.75]);
    let out = ensemble_waveforms(
        EnsembleAlgorithm::AvgWave,
        Some(&[3.0, 1.0]),
        &[a.clone(), b.clone()],
    )
    .expect("weighted avg_wave");
    for i in 0..3 {
        let acc = (0.0f32 + a.planar[i] * 3.0) + b.planar[i] * 1.0;
        assert_eq!(out.planar[i], acc / 4.0, "sample {i}");
    }
    // Sanity: a weight vector that shifts the balance must move the result away
    // from the equal-weight mean.
    let equal = ensemble_waveforms(EnsembleAlgorithm::AvgWave, None, &[a, b]).expect("avg_wave");
    assert!(out.planar[0] > equal.planar[0]);
}

#[test]
fn avg_wave_pads_the_shorter_waveform_with_zeros() {
    let a = buffer(44_100, 1, vec![0.5, 0.5, 0.5, 0.5]);
    let b = buffer(44_100, 1, vec![0.25, 0.25]);
    let out = ensemble_waveforms(EnsembleAlgorithm::AvgWave, None, &[a, b]).expect("avg_wave");
    assert_eq!(out.frames(), 4, "output length is the longest input");
    assert_eq!(out.planar, vec![0.375, 0.375, 0.25, 0.25]);
}

#[test]
fn avg_wave_pads_every_channel_of_the_shorter_waveform() {
    // Equal lengths: both buffers are 2 channels x 2 frames.
    // Stereo, planar layout: [L0, L1, R0, R1].
    let a = buffer(44_100, 2, vec![1.0, 1.0, 1.0, 1.0]);
    let b = buffer(44_100, 2, vec![0.0, 0.0, 0.0, 0.0]);
    let out = ensemble_waveforms(EnsembleAlgorithm::AvgWave, None, &[a, b]).expect("avg_wave");
    assert_eq!(out.planar, vec![0.5, 0.5, 0.5, 0.5]);
}

#[test]
fn avg_wave_pads_each_channel_separately_for_unequal_length_stereo() {
    // The long member is 2 channels x 3 frames, the short one 2 channels x 2
    // frames (`planar` is channel-major: [L0, L1, R0, R1]). Padding must happen
    // per channel, in frames, like `np.pad` on the reference's
    // `[channels, frames]` array — the short channel 0 becomes [1, 2, 0] and
    // the short channel 1 becomes [3, 4, 0], so the padded short waveform is
    // [1, 2, 0, 3, 4, 0]. Padding the flattened buffer instead would read
    // [1, 2, 3, 4, 0, 0] and shift channel 1 into channel 0's tail.
    let long = buffer(44_100, 2, vec![9.0, 9.0, 9.0, 9.0, 9.0, 9.0]);
    let short = buffer(44_100, 2, vec![1.0, 2.0, 3.0, 4.0]);

    // Weights [0, 1] make the short member the only contributor, so the output
    // is exactly the padded short waveform.
    let padded = ensemble_waveforms(
        EnsembleAlgorithm::AvgWave,
        Some(&[0.0, 1.0]),
        &[long.clone(), short.clone()],
    )
    .expect("weighted avg_wave");
    assert_eq!(
        padded.frames(),
        3,
        "output length is the longest input's frames"
    );
    assert_eq!(padded.channels, 2);
    assert_eq!(padded.planar, vec![1.0, 2.0, 0.0, 3.0, 4.0, 0.0]);

    // Equal weights: the per-position mean of the same padded layout.
    let mean =
        ensemble_waveforms(EnsembleAlgorithm::AvgWave, None, &[long, short]).expect("avg_wave");
    assert_eq!(
        mean.planar,
        vec![5.0, 5.5, 4.5, 6.0, 6.5, 4.5],
        "channel 0 = (9 + [1,2,0])/2, channel 1 = (9 + [3,4,0])/2"
    );
}

#[test]
fn median_wave_odd_model_count_picks_the_middle_sample_per_position() {
    let a = buffer(44_100, 1, vec![0.9, -0.9, 0.1]);
    let b = buffer(44_100, 1, vec![0.5, -0.5, 0.1]);
    let c = buffer(44_100, 1, vec![-0.1, 0.0, 0.1]);
    let out = ensemble_waveforms(EnsembleAlgorithm::MedianWave, None, &[a, b, c]).expect("median");
    assert_eq!(out.planar, vec![0.5, -0.5, 0.1]);
}

#[test]
fn median_wave_even_model_count_is_the_mean_of_the_two_middles() {
    let a = buffer(44_100, 1, vec![0.9, -0.9, 0.4]);
    let b = buffer(44_100, 1, vec![0.1, 0.3, -0.2]);
    let out = ensemble_waveforms(EnsembleAlgorithm::MedianWave, None, &[a, b]).expect("median");
    // With two models all samples are "central", so the median is their mean —
    // the reference's np.median behaviour for an even count, evaluated in f32.
    assert_eq!(
        out.planar,
        vec![
            (0.9f32 + 0.1) / 2.0,
            (-0.9f32 + 0.3) / 2.0,
            (0.4f32 + -0.2) / 2.0
        ]
    );
}

#[test]
fn median_wave_ignores_weights_like_the_reference() {
    let a = buffer(44_100, 1, vec![0.9, -0.9, 0.4]);
    let b = buffer(44_100, 1, vec![0.1, 0.3, -0.2]);
    let equal = ensemble_waveforms(EnsembleAlgorithm::MedianWave, None, &[a.clone(), b.clone()])
        .expect("median");
    let weighted = ensemble_waveforms(EnsembleAlgorithm::MedianWave, Some(&[10.0, 1.0]), &[a, b])
        .expect("median");
    assert_eq!(equal.planar, weighted.planar);
}

#[test]
fn median_wave_pads_the_shorter_waveform_with_zeros() {
    let a = buffer(44_100, 1, vec![0.5, 0.5, 0.5, 0.5]);
    let b = buffer(44_100, 1, vec![0.25, 0.25]);
    let out = ensemble_waveforms(EnsembleAlgorithm::MedianWave, None, &[a, b]).expect("median");
    assert_eq!(out.planar, vec![0.375, 0.375, 0.25, 0.25]);
}

#[test]
fn median_wave_pads_each_channel_separately_for_unequal_length_stereo() {
    // 2 channels x 3 frames vs 2 channels x 2 frames vs 2 channels x 3 frames.
    // Per-channel padding puts the short member's channel 0 at [1, 2, 0] and its
    // channel 1 at [3, 4, 0]; the flat-buffer padding would give [1, 2, 3, 4, 0, 0]
    // and mix channel 1 into channel 0.
    let a = buffer(44_100, 2, vec![10.0, 10.0, 10.0, 20.0, 20.0, 20.0]);
    let short = buffer(44_100, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let c = buffer(44_100, 2, vec![0.5, 0.5, 0.5, 0.5, 0.5, 0.5]);
    let out = ensemble_waveforms(
        EnsembleAlgorithm::MedianWave,
        None,
        &[a.clone(), short.clone(), c.clone()],
    )
    .expect("median");
    assert_eq!(out.frames(), 3);
    assert_eq!(out.planar, vec![1.0, 2.0, 0.5, 3.0, 4.0, 0.5]);

    // The reference ignores weights for `median_wave`; that stays true when the
    // shorter member is padded per channel.
    let weighted = ensemble_waveforms(
        EnsembleAlgorithm::MedianWave,
        Some(&[1.0, 8.0, 2.0]),
        &[a, short, c],
    )
    .expect("weighted median");
    assert_eq!(weighted.planar, out.planar);
}

#[test]
fn single_model_is_returned_unchanged() {
    let only = buffer(44_100, 1, vec![0.1, -0.2, 0.3]);
    let out =
        ensemble_waveforms(EnsembleAlgorithm::AvgWave, None, &[only.clone()]).expect("avg_wave");
    assert_eq!(out.planar, only.planar);
    assert_eq!(out.sample_rate, only.sample_rate);
    assert_eq!(out.channels, only.channels);
    // The reference returns waveforms[0] as-is, weights or not.
    let weighted = ensemble_waveforms(EnsembleAlgorithm::MedianWave, Some(&[7.0]), &[only.clone()])
        .expect("median");
    assert_eq!(weighted.planar, only.planar);
}

#[test]
fn empty_waveform_list_is_a_typed_error() {
    let err = ensemble_waveforms(EnsembleAlgorithm::AvgWave, None, &[]).expect_err("empty");
    assert_eq!(
        ensemble_error(err),
        EnsembleError::EmptyWaveforms,
        "an empty model list must fail, not produce a silent zero waveform"
    );
}

#[test]
fn channel_mismatch_is_a_typed_error() {
    let stereo = buffer(44_100, 2, vec![0.0, 0.0, 0.0, 0.0]);
    let mono = buffer(44_100, 1, vec![0.0, 0.0]);
    let err = ensemble_waveforms(EnsembleAlgorithm::AvgWave, None, &[stereo, mono])
        .expect_err("channel mismatch");
    assert!(matches!(
        ensemble_error(err),
        EnsembleError::ChannelMismatch(_)
    ));
}

#[test]
fn sample_rate_mismatch_is_a_typed_error() {
    let a = buffer(44_100, 1, vec![0.0, 0.0]);
    let b = buffer(48_000, 1, vec![0.0, 0.0]);
    let err =
        ensemble_waveforms(EnsembleAlgorithm::AvgWave, None, &[a, b]).expect_err("rate mismatch");
    assert!(matches!(
        ensemble_error(err),
        EnsembleError::SampleRateMismatch(_)
    ));
}

#[test]
fn weights_of_the_wrong_length_are_rejected() {
    let a = buffer(44_100, 1, vec![0.1]);
    let b = buffer(44_100, 1, vec![0.2]);
    let err = ensemble_waveforms(EnsembleAlgorithm::AvgWave, Some(&[1.0]), &[a, b])
        .expect_err("weight count");
    assert!(matches!(
        ensemble_error(err),
        EnsembleError::InvalidWeights(_)
    ));
}

#[test]
fn zero_or_non_finite_weights_are_rejected() {
    let a = buffer(44_100, 1, vec![0.1]);
    let b = buffer(44_100, 1, vec![0.2]);
    let zero = ensemble_waveforms(
        EnsembleAlgorithm::AvgWave,
        Some(&[1.0, -1.0]),
        &[a.clone(), b.clone()],
    )
    .expect_err("zero sum");
    assert!(matches!(
        ensemble_error(zero),
        EnsembleError::InvalidWeights(_)
    ));
    let nan = ensemble_waveforms(EnsembleAlgorithm::AvgWave, Some(&[f32::NAN, 1.0]), &[a, b])
        .expect_err("non-finite");
    assert!(matches!(
        ensemble_error(nan),
        EnsembleError::InvalidWeights(_)
    ));
}

#[test]
fn operator_ids_round_trip_and_unknown_operators_are_typed() {
    for id in EnsembleAlgorithm::SUPPORTED {
        let algo = EnsembleAlgorithm::from_id(id).expect("supported id");
        assert_eq!(algo.as_str(), id);
    }
    // Reference operator ids this crate does not implement are errors, never a
    // silent substitution of different semantics.
    for id in ["avg_fft", "median_fft", "uvr_max_spec", "ensemble_wav", ""] {
        match EnsembleAlgorithm::from_id(id) {
            Err(EnsembleError::UnknownAlgorithm {
                id: echoed,
                supported,
            }) => {
                assert_eq!(echoed, id);
                assert!(supported.contains("avg_wave") && supported.contains("median_wave"));
            }
            other => panic!("`{id}` must be a typed UnknownAlgorithm, got {other:?}"),
        }
    }
}

#[test]
fn vocal_balanced_has_the_documented_composition() {
    let preset = EnsemblePreset::builtin("vocal_balanced").expect("built-in preset");
    assert_eq!(preset.id, "vocal_balanced");
    assert_eq!(preset.algorithm, EnsembleAlgorithm::AvgWave);
    assert_eq!(
        preset.weights, None,
        "equal weights are documented as `null`"
    );
    assert_eq!(
        preset.stems,
        vec!["vocals".to_string(), "instrumental".to_string()]
    );
    let checkpoints: Vec<&str> = preset
        .models
        .iter()
        .map(|m| m.checkpoint.as_str())
        .collect();
    assert_eq!(
        checkpoints,
        vec!["model_bs_roformer_ep_317_sdr_12.9755.ckpt", "voc_fv7.ckpt"],
        "the preset is BS-RoFormer + MelBand, in application order"
    );
    for member in &preset.models {
        assert_eq!(member.role, "vocals");
        assert!(member.profile.starts_with("configs/"));
    }
    preset.validate().expect("the built-in preset validates");
}

#[test]
fn unknown_preset_is_a_typed_error() {
    match EnsemblePreset::builtin("vocal_clean") {
        Err(EnsembleError::UnknownPreset { id, known }) => {
            assert_eq!(id, "vocal_clean");
            assert!(known.contains("vocal_balanced"));
        }
        other => panic!("unknown preset must be typed, got {other:?}"),
    }
}

#[test]
fn preset_parses_from_yaml_and_validates_its_weights() {
    let good = r#"
id: demo
name: Demo
algorithm: median_wave
weights: [1.0, 2.0]
models:
  - checkpoint: a.ckpt
    profile: configs/a.yaml
    role: vocals
  - checkpoint: b.ckpt
    profile: configs/b.yaml
    role: vocals
stems: [vocals, instrumental]
"#;
    let preset = EnsemblePreset::from_yaml_str(good).expect("valid preset");
    assert_eq!(preset.algorithm, EnsembleAlgorithm::MedianWave);
    assert_eq!(preset.weights, Some(vec![1.0, 2.0]));

    let bad_weights = good.replace("weights: [1.0, 2.0]", "weights: [1.0]");
    match EnsemblePreset::from_yaml_str(&bad_weights) {
        Err(EnsembleError::InvalidWeights(_)) => {}
        other => panic!("weight count must be rejected, got {other:?}"),
    }

    let bad_role = good.replace(
        "role: vocals\n  - checkpoint: b.ckpt",
        "role: other\n  - checkpoint: b.ckpt",
    );
    match EnsemblePreset::from_yaml_str(&bad_role) {
        Err(EnsembleError::InvalidPreset(_)) => {}
        other => panic!("a role outside the stem set must be rejected, got {other:?}"),
    }

    let unknown_operator = good.replace("median_wave", "avg_fft");
    match EnsemblePreset::from_yaml_str(&unknown_operator) {
        Err(err) => {
            // serde reports the unknown enum variant; the point is that it is a
            // typed error and not a silent substitution.
            assert!(format!("{err}").contains("avg_fft"), "got {err}");
        }
        Ok(_) => panic!("an unknown operator must not resolve"),
    }

    let duplicate_stems = good.replace("stems: [vocals, instrumental]", "stems: [vocals, vocals]");
    match EnsemblePreset::from_yaml_str(&duplicate_stems) {
        Err(EnsembleError::InvalidPreset(_)) => {}
        other => panic!("duplicate stems must be rejected, got {other:?}"),
    }

    let no_models = r#"
id: demo
name: Demo
algorithm: avg_wave
models: []
stems: [vocals, instrumental]
"#;
    match EnsemblePreset::from_yaml_str(no_models) {
        Err(EnsembleError::InvalidPreset(_)) => {}
        other => panic!("an empty model list must be rejected, got {other:?}"),
    }
}

#[test]
fn output_keeps_the_input_layout_and_is_finite() {
    let a = buffer(48_000, 2, vec![0.25; 8]);
    let b = buffer(48_000, 2, vec![-0.25; 8]);
    for algorithm in [EnsembleAlgorithm::AvgWave, EnsembleAlgorithm::MedianWave] {
        let out = ensemble_waveforms(algorithm, None, &[a.clone(), b.clone()]).expect("ensemble");
        assert_eq!(out.sample_rate, 48_000);
        assert_eq!(out.channels, 2);
        assert_eq!(out.frames(), a.frames());
        assert!(out.planar.iter().all(|v| v.is_finite()));
        // Zero-mean inputs stay zero-mean: nothing is normalized or rescaled.
        assert!(out.planar.iter().all(|v| *v == 0.0));
    }
}

#[test]
fn empty_buffer_is_rejected_by_the_audio_validator() {
    let empty = buffer(44_100, 1, Vec::new());
    let ok = buffer(44_100, 1, vec![0.0]);
    let err = ensemble_waveforms(EnsembleAlgorithm::AvgWave, None, &[ok, empty])
        .expect_err("empty buffer");
    assert!(matches!(err, Error::Audio(WavError::UnsupportedFormat(_))));
}

// ---------------------------------------------------------------------------
// Parity against the frozen audio-separator 0.48.0 fixture (`#[ignore]`d).
//
// The fixture directory holds, for every stem of the preset, the two real
// checkpoints' waveforms as planar f32 dumps (`inputs/member<i>_<stem>.f32`) and
// `inputs/layout.json` with
// `{"channels":2,"frames":N,"sample_rate":44100,"stems":["vocals","instrumental"]}`,
// plus the reference outputs produced by `Ensembler.ensemble` on exactly those
// arrays (`reference/<algorithm>_<stem>.f64`, the dtype `Ensembler` returns).
// The test writes the candidate dumps as f32 under `$CANDLE_ROFORMER_ENSEMBLE_OUT`
// and asserts the issue #8 tolerance over every sample of every channel:
// `max_abs <= 1e-6`, `RMSE <= 1e-7`.
// ---------------------------------------------------------------------------

#[cfg(test)]
fn read_planar_f32(path: &std::path::Path) -> Vec<f32> {
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    assert_eq!(
        bytes.len() % 4,
        0,
        "{} is not a whole number of f32",
        path.display()
    );
    bytes
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect()
}

#[cfg(test)]
fn read_planar_f64(path: &std::path::Path) -> Vec<f64> {
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    assert_eq!(
        bytes.len() % 8,
        0,
        "{} is not a whole number of f64",
        path.display()
    );
    bytes
        .chunks_exact(8)
        .map(|c| f64::from_le_bytes([c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]))
        .collect()
}

#[test]
#[ignore = "needs the frozen audio-separator 0.48.0 fixture: set CANDLE_ROFORMER_ENSEMBLE_FIXTURE"]
fn ensemble_matches_audio_separator_reference() {
    let fixture = std::env::var("CANDLE_ROFORMER_ENSEMBLE_FIXTURE")
        .expect("set CANDLE_ROFORMER_ENSEMBLE_FIXTURE to the frozen fixture directory");
    let out_dir = std::env::var("CANDLE_ROFORMER_ENSEMBLE_OUT")
        .expect("set CANDLE_ROFORMER_ENSEMBLE_OUT to the candidate output directory");
    let fixture = std::path::PathBuf::from(fixture);
    let out_dir = std::path::PathBuf::from(out_dir);
    std::fs::create_dir_all(&out_dir).expect("create candidate dir");

    let layout_text = std::fs::read_to_string(fixture.join("layout.json")).expect("layout");
    let layout: serde_json::Value = serde_json::from_str(&layout_text).expect("layout json");
    let channels = layout["channels"].as_u64().expect("channels") as u16;
    let frames = layout["frames"].as_u64().expect("frames") as usize;
    let sample_rate = layout["sample_rate"].as_u64().expect("sample_rate") as u32;
    let stems: Vec<String> = layout["stems"]
        .as_array()
        .expect("stems")
        .iter()
        .map(|v| v.as_str().expect("stem name").to_string())
        .collect();
    assert!(!stems.is_empty(), "the fixture declares no stem");

    let mut compared = 0usize;
    for stem in &stems {
        let mut members = Vec::new();
        for index in 0..2 {
            let planar = read_planar_f32(&fixture.join(format!("inputs/member{index}_{stem}.f32")));
            assert_eq!(
                planar.len(),
                frames * channels as usize,
                "{stem}: member {index} length"
            );
            members.push(candle_roformer::AudioBuffer {
                sample_rate,
                channels,
                planar,
            });
        }

        for (algorithm, name) in [
            (EnsembleAlgorithm::AvgWave, "avg_wave"),
            (EnsembleAlgorithm::MedianWave, "median_wave"),
        ] {
            let file = format!("{name}_{stem}");
            let expected = read_planar_f64(&fixture.join(format!("reference/{file}.f64")));
            let candidate = ensemble_waveforms(algorithm, None, &members).expect("ensemble");
            assert_eq!(candidate.planar.len(), expected.len(), "{file} length");

            std::fs::write(
                out_dir.join(format!("{file}.f32")),
                candidate
                    .planar
                    .iter()
                    .flat_map(|v| v.to_le_bytes())
                    .collect::<Vec<u8>>(),
            )
            .expect("write candidate dump");

            let mut max_abs = 0f64;
            let mut sq = 0f64;
            for (a, b) in candidate.planar.iter().zip(expected.iter()) {
                assert!(a.is_finite() && b.is_finite(), "{file}: non-finite sample");
                let d = (*a as f64) - *b;
                max_abs = max_abs.max(d.abs());
                sq += d * d;
            }
            let rmse = (sq / expected.len() as f64).sqrt();
            println!(
                "{file}: max_abs={max_abs:.3e} rmse={rmse:.3e} samples={}",
                expected.len()
            );

            // Per channel too, so a channel-localized failure cannot hide in the mix.
            for ch in 0..channels as usize {
                let mut ch_max = 0f64;
                let mut ch_sq = 0f64;
                for i in 0..frames {
                    let idx = ch * frames + i;
                    let d = (candidate.planar[idx] as f64) - expected[idx];
                    ch_max = ch_max.max(d.abs());
                    ch_sq += d * d;
                }
                let ch_rmse = (ch_sq / frames as f64).sqrt();
                println!("{file}: channel {ch} max_abs={ch_max:.3e} rmse={ch_rmse:.3e}");
                assert!(
                    ch_max <= 1e-6,
                    "{file} channel {ch}: max_abs {ch_max:e} > 1e-6"
                );
                assert!(
                    ch_rmse <= 1e-7,
                    "{file} channel {ch}: rmse {ch_rmse:e} > 1e-7"
                );
            }
            assert!(max_abs <= 1e-6, "{file}: max_abs {max_abs:e} > 1e-6");
            assert!(rmse <= 1e-7, "{file}: rmse {rmse:e} > 1e-7");
            compared += 1;
        }
    }
    assert!(compared > 0, "no reference comparison ran");
    println!("compared {compared} algorithm x stem pairs over {stems:?}");
}
