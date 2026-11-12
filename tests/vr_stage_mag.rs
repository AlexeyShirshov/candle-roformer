//! Stage-by-stage comparison of the VR analysis spectrogram with the reference.

use std::path::Path;

use candle_core::{DType, Device};
use candle_nn::VarBuilder;

use candle_roformer::model::vr_pipeline::{VrProfile, VrSeparator};

const EVIDENCE: &str = r"F:\sources\candle-roformer\.pdca\open-issues-r5\rv1\item-3-vr-uvr";

#[test]
fn vr_analysis_mag_matches_reference() {
    let weights = format!("{EVIDENCE}\\models\\2_HP-UVR.safetensors");
    let fixture = format!("{EVIDENCE}\\fixture\\fixture-cc0-go-out-rock-12s-f32-44100.wav");
    let reference = format!("{EVIDENCE}\\ref\\stages\\mag.safetensors");
    for path in [&weights, &fixture, &reference] {
        if !Path::new(path).exists() {
            eprintln!("skipping: {path} is not available");
            return;
        }
    }
    let audio = candle_roformer::wav::read_audio(&fixture).unwrap();
    let frames = audio.frames();
    let (left, right) = audio.planar.split_at(frames);

    let separator = VrSeparator::load(Path::new(&weights), VrProfile::four_band_v2()).unwrap();
    let (mag, bins, nframes) = separator.debug_mag(left, right).unwrap();
    eprintln!("rust mag: bins={bins} frames={nframes}");

    let device = Device::Cpu;
    // SAFETY: read-only fixture.
    let vb =
        unsafe { VarBuilder::from_mmaped_safetensors(&[reference], DType::F32, &device).unwrap() };
    let reference_mag = vb.get((2, 673, 1103), "x").unwrap();
    let reference_mag: Vec<f32> = reference_mag.flatten_all().unwrap().to_vec1().unwrap();

    assert_eq!(nframes, 1103, "frame count mismatch");
    let mut max_abs = 0.0f32;
    let mut sum_abs = 0f64;
    for ch in 0..2 {
        for bin in 0..bins {
            for frame in 0..nframes {
                let ours = mag[ch * bins * nframes + bin * nframes + frame];
                let theirs = reference_mag[ch * 673 * 1103 + bin * 1103 + frame];
                let d = (ours - theirs).abs();
                max_abs = max_abs.max(d);
                sum_abs += d as f64;
            }
        }
    }
    let count = (2 * bins * nframes) as f64;
    eprintln!(
        "analysis mag: max_abs={max_abs:e} mean_abs={:e}",
        sum_abs / count
    );
    assert!(max_abs <= 1e-4, "analysis magnitude diverges: {max_abs:e}");
}
