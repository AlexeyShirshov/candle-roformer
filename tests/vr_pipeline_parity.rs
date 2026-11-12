//! End-to-end parity of the VR Arch pipeline against `audio-separator` 0.48.0.
//!
//! Requires the gitignored evidence artifacts:
//! `models/2_HP-UVR.safetensors`, `fixture/*.wav` and
//! `ref/reference-{primary,secondary}-f32.wav`.

use std::path::Path;

use candle_roformer::model::vr_pipeline::{VrProfile, VrSeparator};

const EVIDENCE: &str = r"F:\sources\candle-roformer\.pdca\open-issues-r5\rv1\item-3-vr-uvr";

fn metrics(candidate: &[f32], reference: &[f32]) -> (f32, f32, f32, f64) {
    let n = candidate.len().min(reference.len());
    let mut max_abs = 0.0f32;
    let mut sq = 0.0f64;
    let mut ref_sq = 0.0f64;
    let mut err_sq = 0.0f64;
    let (mut sum_a, mut sum_b, mut sum_aa, mut sum_bb, mut sum_ab) =
        (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for i in 0..n {
        let a = candidate[i] as f64;
        let b = reference[i] as f64;
        let d = a - b;
        max_abs = max_abs.max(d.abs() as f32);
        sq += d * d;
        err_sq += d * d;
        ref_sq += b * b;
        sum_a += a;
        sum_b += b;
        sum_aa += a * a;
        sum_bb += b * b;
        sum_ab += a * b;
    }
    let rmse = (sq / n as f64).sqrt() as f32;
    let sdr = if err_sq == 0.0 {
        f32::INFINITY
    } else {
        (10.0 * (ref_sq / err_sq).log10()) as f32
    };
    let nf = n as f64;
    let cov = sum_ab / nf - (sum_a / nf) * (sum_b / nf);
    let var_a = sum_aa / nf - (sum_a / nf).powi(2);
    let var_b = sum_bb / nf - (sum_b / nf).powi(2);
    let corr = cov / (var_a.sqrt() * var_b.sqrt());
    (max_abs, rmse, sdr, corr)
}

#[test]
fn vr_pipeline_matches_audio_separator() {
    let weights = format!("{EVIDENCE}\\models\\2_HP-UVR.safetensors");
    let fixture = format!("{EVIDENCE}\\fixture\\fixture-cc0-go-out-rock-12s-f32-44100.wav");
    let ref_primary = format!("{EVIDENCE}\\ref\\reference-primary-f32.wav");
    let ref_secondary = format!("{EVIDENCE}\\ref\\reference-secondary-f32.wav");
    for path in [&weights, &fixture, &ref_primary, &ref_secondary] {
        if !Path::new(path).exists() {
            eprintln!("skipping: {path} is not available");
            return;
        }
    }

    let audio = candle_roformer::wav::read_audio(&fixture).unwrap();
    assert_eq!(audio.channels, 2, "fixture must be stereo");
    assert_eq!(audio.sample_rate, 44100);
    let frames = audio.frames();
    let (left, right) = audio.planar.split_at(frames);

    let separator = VrSeparator::load(Path::new(&weights), VrProfile::four_band_v2()).unwrap();
    let (primary, secondary) = separator.separate(left, right).unwrap();

    let reference_primary = candle_roformer::wav::read_audio(&ref_primary).unwrap();
    let reference_secondary = candle_roformer::wav::read_audio(&ref_secondary).unwrap();

    // Persist the Rust output so the reference comparison can be inspected
    // outside the test (cross-correlation, spectrograms).
    for (name, data) in [("rust-primary", &primary), ("rust-secondary", &secondary)] {
        let path = format!("{EVIDENCE}\\out\\{name}.wav");
        let ch_frames = data.len() / 2;
        let mut planar = Vec::with_capacity(data.len());
        for ch in 0..2 {
            planar.extend(data.iter().skip(ch).step_by(2));
        }
        debug_assert_eq!(planar.len(), 2 * ch_frames);
        let buffer = candle_roformer::wav::AudioBuffer {
            sample_rate: 44100,
            channels: 2,
            planar,
        };
        candle_roformer::wav::write_wav_f32(&path, &buffer).unwrap();
    }

    // Both sides are interleaved stereo.
    let mut worst_ok = true;
    for (name, candidate, reference) in [
        ("primary", &primary, &reference_primary.planar),
        ("secondary", &secondary, &reference_secondary.planar),
    ] {
        let cand_rms = (candidate
            .iter()
            .map(|v| (*v as f64) * (*v as f64))
            .sum::<f64>()
            / candidate.len() as f64)
            .sqrt();
        let ref_rms = (reference
            .iter()
            .map(|v| (*v as f64) * (*v as f64))
            .sum::<f64>()
            / reference.len() as f64)
            .sqrt();
        eprintln!(
            "{name}: rms candidate={cand_rms:.6} reference={ref_rms:.6} first={:?} vs {:?}",
            &candidate[..4.min(candidate.len())],
            &reference[..4.min(reference.len())]
        );
        assert_eq!(
            candidate.len(),
            reference.len(),
            "{name}: length {} vs reference {}",
            candidate.len(),
            reference.len()
        );
        for ch in 0..2 {
            // `candidate` is interleaved stereo, `reference.planar` is channel-major.
            let cand: Vec<f32> = candidate.iter().skip(ch).step_by(2).copied().collect();
            let ch_frames = reference.len() / 2;
            let refc: Vec<f32> = reference[ch * ch_frames..(ch + 1) * ch_frames].to_vec();
            let (max_abs, rmse, sdr, corr) = metrics(&cand, &refc);
            eprintln!(
                "{name} ch{ch}: max_abs={max_abs:e} rmse={rmse:e} sdr={sdr:.2}dB corr={corr:.12}"
            );
            worst_ok &= max_abs <= 1e-5 && rmse <= 1e-6 && sdr >= 100.0 && corr >= 0.999999;
        }
    }
    assert!(worst_ok, "VR pipeline parity exceeds the agreed tolerance");
}
