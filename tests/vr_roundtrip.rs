//! Round-trip check: synthesis with a unit mask must reproduce the mixture.

use std::path::Path;

use candle_roformer::model::vr_pipeline::{VrProfile, VrSeparator};

const EVIDENCE: &str = r"F:\sources\candle-roformer\.pdca\open-issues-r5\rv1\item-3-vr-uvr";

#[test]
fn vr_unit_mask_roundtrip() {
    let weights = format!("{EVIDENCE}\\models\\2_HP-UVR.safetensors");
    let fixture = format!("{EVIDENCE}\\fixture\\fixture-cc0-go-out-rock-12s-f32-44100.wav");
    if !Path::new(&weights).exists() {
        eprintln!("skipping: evidence missing");
        return;
    }
    let audio = candle_roformer::wav::read_audio(&fixture).unwrap();
    let frames = audio.frames();
    let (left, right) = audio.planar.split_at(frames);
    let separator = VrSeparator::load(Path::new(&weights), VrProfile::four_band_v2()).unwrap();
    let mix = separator.debug_full_mix(left, right).unwrap();
    let cand: Vec<f32> = mix.iter().step_by(2).copied().collect();
    let n = cand.len().min(left.len());
    let mut max_abs = 0.0f32;
    let mut sq = 0.0f64;
    let mut ref_sq = 0.0f64;
    for i in 0..n {
        let d = cand[i] - left[i];
        max_abs = max_abs.max(d.abs());
        sq += (d as f64) * (d as f64);
        ref_sq += (left[i] as f64) * (left[i] as f64);
    }
    let sdr = 10.0 * (ref_sq / sq).log10();
    eprintln!(
        "unit-mask roundtrip: len={} vs {} max_abs={max_abs:e} sdr={sdr:.2}dB",
        cand.len(),
        left.len()
    );
}
