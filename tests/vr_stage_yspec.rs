//! Comparison of the masked spectrum (`yspec`) with the reference dump.

use std::path::Path;

use candle_core::{DType, Device};
use candle_nn::VarBuilder;

use candle_roformer::model::vr_pipeline::{VrProfile, VrSeparator};

const EVIDENCE: &str = r"F:\sources\candle-roformer\.pdca\open-issues-r5\rv1\item-3-vr-uvr";

#[test]
fn vr_yspec_matches_reference() {
    let weights = format!("{EVIDENCE}\\models\\2_HP-UVR.safetensors");
    let fixture = format!("{EVIDENCE}\\fixture\\fixture-cc0-go-out-rock-12s-f32-44100.wav");
    let reference = format!("{EVIDENCE}\\ref\\stages\\yspec.safetensors");
    if !Path::new(&reference).exists() {
        eprintln!("skipping: evidence missing");
        return;
    }
    let audio = candle_roformer::wav::read_audio(&fixture).unwrap();
    let frames = audio.frames();
    let (left, right) = audio.planar.split_at(frames);
    let separator = VrSeparator::load(Path::new(&weights), VrProfile::four_band_v2()).unwrap();
    let (re, im, bins, nframes) = separator.debug_yspec(left, right).unwrap();

    let device = Device::Cpu;
    // SAFETY: read-only fixture.
    let vb =
        unsafe { VarBuilder::from_mmaped_safetensors(&[reference], DType::F32, &device).unwrap() };
    let Some(ref_re) = vb.get((2, 673, nframes), "re").ok().map(|t| {
        t.narrow(0, 0, 1)
            .unwrap()
            .flatten_all()
            .unwrap()
            .to_vec1::<f32>()
            .unwrap()
    }) else {
        eprintln!("yspec dump has no `re` tensor; skipping");
        return;
    };
    let ref_im = vb
        .get((2, 673, nframes), "im")
        .unwrap()
        .narrow(0, 0, 1)
        .unwrap()
        .flatten_all()
        .unwrap()
        .to_vec1::<f32>()
        .unwrap();

    let mut max_abs = 0.0f32;
    let mut ref_max = 0.0f32;
    for frame in 0..nframes {
        for bin in 0..bins {
            let a = re[frame * bins + bin];
            let b = ref_re[bin * nframes + frame];
            max_abs = max_abs.max((a - b).abs());
            max_abs = max_abs.max((im[frame * bins + bin] - ref_im[bin * nframes + frame]).abs());
            ref_max = ref_max.max(b.abs());
        }
    }
    eprintln!("yspec: max_abs_diff={max_abs:e} ref_abs_max={ref_max:e}");
    assert!(max_abs <= 1e-4, "yspec diverges: {max_abs:e}");
}
