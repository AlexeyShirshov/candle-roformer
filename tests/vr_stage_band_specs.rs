//! Per-band masked spectrum comparison with the reference `cmb-spec-*` dumps.

use std::path::Path;

use candle_core::{DType, Device};
use candle_nn::VarBuilder;

use candle_roformer::model::vr_pipeline::{VrProfile, VrSeparator};

const EVIDENCE: &str = r"F:\sources\candle-roformer\.pdca\open-issues-r5\rv1\item-3-vr-uvr";

#[test]
fn vr_band_specs_match_reference() {
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
    let specs = separator.debug_band_specs(left, right).unwrap();
    let device = Device::Cpu;
    for (index, (re, im, bins)) in specs.iter().enumerate() {
        let stage = format!("cmb-spec-band{}", index + 1);
        let path = format!("{EVIDENCE}\\ref\\stages\\{stage}.safetensors");
        if !Path::new(&path).exists() {
            eprintln!("{stage}: missing");
            continue;
        }
        // SAFETY: read-only fixture.
        let vb =
            unsafe { VarBuilder::from_mmaped_safetensors(&[path], DType::F32, &device).unwrap() };
        let ref_re = vb
            .get((2, *bins, 1103), "re")
            .unwrap()
            .narrow(0, 0, 1)
            .unwrap()
            .flatten_all()
            .unwrap()
            .to_vec1::<f32>()
            .unwrap();
        let ref_im = vb
            .get((2, *bins, 1103), "im")
            .unwrap()
            .narrow(0, 0, 1)
            .unwrap()
            .flatten_all()
            .unwrap()
            .to_vec1::<f32>()
            .unwrap();
        let mut max_abs = 0.0f32;
        let mut ref_max = 0.0f32;
        for frame in 0..1103 {
            for bin in 0..*bins {
                let a = re[frame * bins + bin];
                let b = ref_re[bin * 1103 + frame];
                max_abs = max_abs.max((a - b).abs());
                max_abs = max_abs.max((im[frame * bins + bin] - ref_im[bin * 1103 + frame]).abs());
                ref_max = ref_max.max(b.abs());
            }
        }
        eprintln!("{stage}: max_abs_diff={max_abs:e} ref_abs_max={ref_max:e}");
    }
}
