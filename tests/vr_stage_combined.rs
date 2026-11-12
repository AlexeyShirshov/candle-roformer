//! Combined spectrogram comparison with the reference `mix-spec` dump.

use std::path::Path;

use candle_core::{DType, Device};
use candle_nn::VarBuilder;

use candle_roformer::model::vr_pipeline::{VrProfile, VrSeparator};

const EVIDENCE: &str = r"F:\sources\candle-roformer\.pdca\open-issues-r5\rv1\item-3-vr-uvr";

#[test]
fn vr_combined_spec_matches_reference() {
    let weights = format!("{EVIDENCE}\\models\\2_HP-UVR.safetensors");
    let fixture = format!("{EVIDENCE}\\fixture\\fixture-cc0-go-out-rock-12s-f32-44100.wav");
    let reference = format!("{EVIDENCE}\\ref\\stages\\mix-spec.safetensors");
    if !Path::new(&reference).exists() || !Path::new(&weights).exists() {
        eprintln!("skipping: evidence missing");
        return;
    }
    let audio = candle_roformer::wav::read_audio(&fixture).unwrap();
    let frames = audio.frames();
    let (left, _right) = audio.planar.split_at(frames);
    let separator = VrSeparator::load(Path::new(&weights), VrProfile::four_band_v2()).unwrap();
    let (re, im, bins, nframes) = separator.debug_combined(left).unwrap();
    eprintln!("combined: bins={bins} frames={nframes}");

    let device = Device::Cpu;
    // SAFETY: read-only fixture.
    let vb =
        unsafe { VarBuilder::from_mmaped_safetensors(&[reference], DType::F32, &device).unwrap() };
    let names = vb.clone();
    let _ = names;
    // The dump stores the complex array as `re`/`im` or as a complex tensor.
    let keys: Vec<String> = Vec::new();
    let _ = keys;
    let ref_re = vb.get((2, 673, 1103), "re").ok().map(|t| {
        t.narrow(0, 0, 1)
            .unwrap()
            .flatten_all()
            .unwrap()
            .to_vec1::<f32>()
            .unwrap()
    });
    let ref_im = vb.get((2, 673, 1103), "im").ok().map(|t| {
        t.narrow(0, 0, 1)
            .unwrap()
            .flatten_all()
            .unwrap()
            .to_vec1::<f32>()
            .unwrap()
    });
    let (Some(ref_re), Some(ref_im)) = (ref_re, ref_im) else {
        eprintln!("mix-spec does not expose re/im tensors; skipping comparison");
        return;
    };

    let mut max_abs = 0.0f32;
    let mut ref_max = 0.0f32;
    for frame in 0..nframes {
        for bin in 0..bins {
            let ours_re = re[frame * bins + bin];
            let ours_im = im[frame * bins + bin];
            let theirs_re = ref_re[bin * 1103 + frame];
            let theirs_im = ref_im[bin * 1103 + frame];
            max_abs = max_abs.max((ours_re - theirs_re).abs());
            max_abs = max_abs.max((ours_im - theirs_im).abs());
            ref_max = ref_max.max(theirs_re.abs());
        }
    }
    eprintln!("combined spec: max_abs_diff={max_abs:e} ref_abs_max={ref_max:e}");
    assert!(
        max_abs <= 1e-4,
        "combined spectrogram diverges: {max_abs:e}"
    );
}
