//! End-to-end parity of the Demucs v4 (HTDemucs) path against
//! `audio-separator` 0.48.0.
//!
//! Requires the gitignored evidence artifacts of item #4:
//! `models/955717e8-8726e21a.safetensors` (produced by
//! `tools/convert_demucs.py` from the pinned `.th`), the frozen fixture and the
//! four reference stems under `ref/stems/`.
//!
//! Paths default to `.pdca/open-issues-r5/rv1/item-4-demucs` and can be
//! overridden with `DEMUCS_EVIDENCE`, `DEMUCS_WEIGHTS`, `DEMUCS_FIXTURE` and
//! `DEMUCS_REF_STEMS`.
//!
//! Measurement is the approved one: per stem, per channel, over every sample of
//! the full output, no trimming and no normalisation.
//!   max_abs = max |candidate - reference|
//!   RMSE    = sqrt(mean((candidate - reference)^2))
//!   SDR     = 10 log10(sum(reference^2) / sum((candidate - reference)^2))
//! Thresholds: max_abs <= 3.5e-4, RMSE <= 1e-4, SDR >= 80 dB on all four stems.

use std::path::{Path, PathBuf};

use candle_roformer::model::DemucsSeparator;
use candle_roformer::{DeviceChoice, Stems};

fn default_evidence() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".pdca/open-issues-r5/rv1/item-4-demucs")
}

const MAX_ABS: f64 = 3.5e-4;
const RMSE: f64 = 1e-4;
const SDR_DB: f64 = 80.0;

fn env_path(key: &str, default: PathBuf) -> PathBuf {
    match std::env::var(key) {
        Ok(v) if !v.is_empty() => PathBuf::from(v),
        _ => default,
    }
}

fn evidence_root() -> PathBuf {
    env_path("DEMUCS_EVIDENCE", default_evidence())
}

fn metrics(candidate: &[f32], reference: &[f32]) -> (f64, f64, f64) {
    assert_eq!(candidate.len(), reference.len(), "length mismatch");
    let mut max_abs = 0.0f64;
    let mut err_sq = 0.0f64;
    let mut ref_sq = 0.0f64;
    for (a, b) in candidate.iter().zip(reference.iter()) {
        let d = *a as f64 - *b as f64;
        max_abs = max_abs.max(d.abs());
        err_sq += d * d;
        ref_sq += (*b as f64) * (*b as f64);
    }
    let n = candidate.len() as f64;
    let rmse = (err_sq / n).sqrt();
    let sdr = if err_sq == 0.0 {
        f64::INFINITY
    } else {
        10.0 * (ref_sq / err_sq).log10()
    };
    (max_abs, rmse, sdr)
}

fn reference_stem_path(stems_dir: &Path, stem: &str) -> PathBuf {
    let capitalised = match stem {
        "drums" => "Drums",
        "bass" => "Bass",
        "other" => "Other",
        "vocals" => "Vocals",
        other => panic!("unexpected stem name {other}"),
    };
    stems_dir.join(format!(
        "fixture-go-out-rock-30s-10s-f32-44100_({capitalised})_htdemucs.wav"
    ))
}

#[test]
fn demucs_four_stems_match_audio_separator() {
    let root = evidence_root();
    let weights = env_path(
        "DEMUCS_WEIGHTS",
        root.join("models").join("955717e8-8726e21a.safetensors"),
    );
    let fixture = env_path(
        "DEMUCS_FIXTURE",
        root.join("fixture")
            .join("fixture-go-out-rock-30s-10s-f32-44100.wav"),
    );
    let ref_stems = env_path("DEMUCS_REF_STEMS", root.join("ref").join("stems"));
    for path in [&weights, &fixture, &ref_stems] {
        if !path.exists() {
            eprintln!("skipping: {} is not available", path.display());
            return;
        }
    }

    let audio = candle_roformer::wav::read_audio(&fixture).expect("read fixture");
    assert_eq!(audio.channels, 2, "fixture must be stereo");
    assert_eq!(audio.sample_rate, 44100);

    let separator = DemucsSeparator::load(&weights, DeviceChoice::Cpu).expect("load HTDemucs");
    assert_eq!(separator.segment_samples(), 343_980);
    let stems: Stems = separator.separate(&audio).expect("separate");

    let names: Vec<&str> = stems.names();
    assert_eq!(
        names,
        vec!["drums", "bass", "other", "vocals"],
        "per-stem output naming"
    );

    let frames = audio.frames();
    let mut failures: Vec<String> = Vec::new();
    for stem in ["drums", "bass", "other", "vocals"] {
        let candidate = stems.get(stem).unwrap_or_else(|| panic!("missing {stem}"));
        assert_eq!(candidate.channels, 2, "{stem}: stereo output");
        assert_eq!(candidate.sample_rate, 44100, "{stem}: sample rate");
        assert_eq!(candidate.frames(), frames, "{stem}: full length, no trimming");

        let reference_path = reference_stem_path(&ref_stems, stem);
        let reference = candle_roformer::wav::read_audio(&reference_path)
            .unwrap_or_else(|e| panic!("read {}: {e}", reference_path.display()));
        assert_eq!(reference.channels, 2);

        for ch in 0..2usize {
            let cand = &candidate.planar[ch * frames..(ch + 1) * frames];
            let refc = &reference.planar[ch * frames..(ch + 1) * frames];
            let (max_abs, rmse, sdr) = metrics(cand, refc);
            eprintln!(
                "{stem} ch{ch}: max_abs={max_abs:.3e} rmse={rmse:.3e} sdr={sdr:.2} dB"
            );
            if max_abs > MAX_ABS {
                failures.push(format!("{stem} ch{ch}: max_abs {max_abs:.3e} > {MAX_ABS:.1e}"));
            }
            if rmse > RMSE {
                failures.push(format!("{stem} ch{ch}: rmse {rmse:.3e} > {RMSE:.1e}"));
            }
            if sdr < SDR_DB {
                failures.push(format!("{stem} ch{ch}: sdr {sdr:.2} dB < {SDR_DB:.0} dB"));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "Demucs parity failed:\n{}",
        failures.join("\n")
    );
}
