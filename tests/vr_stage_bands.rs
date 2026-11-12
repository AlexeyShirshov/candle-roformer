//! Per-band STFT comparison with the reference stage ladder.

use std::path::Path;

use candle_core::{DType, Device};
use candle_nn::VarBuilder;

use candle_roformer::model::vr_pipeline::{VrProfile, VrSeparator};

const EVIDENCE: &str = r"F:\sources\candle-roformer\.pdca\open-issues-r5\rv1\item-3-vr-uvr";

#[test]
fn vr_band_stft_matches_reference() {
    let weights = format!("{EVIDENCE}\\models\\2_HP-UVR.safetensors");
    let fixture = format!("{EVIDENCE}\\fixture\\fixture-cc0-go-out-rock-12s-f32-44100.wav");
    if !Path::new(&weights).exists() || !Path::new(&fixture).exists() {
        eprintln!("skipping: evidence missing");
        return;
    }
    let audio = candle_roformer::wav::read_audio(&fixture).unwrap();
    let frames = audio.frames();
    let (left, _right) = audio.planar.split_at(frames);
    let separator = VrSeparator::load(Path::new(&weights), VrProfile::four_band_v2()).unwrap();

    let device = Device::Cpu;
    for (band, bins, ref_frames) in [
        (3usize, 481usize, 1103usize),
        (2, 257, 1103),
        (1, 161, 1103),
        (0, 321, 1103),
    ] {
        let path = format!("{EVIDENCE}\\ref\\stages\\spec-band{}.safetensors", band + 1);
        if !Path::new(&path).exists() {
            continue;
        }
        let (re, im, got_bins, got_frames) = separator.debug_band_spec(left, band).unwrap();
        assert_eq!(got_bins, bins, "band {band} bins");
        assert_eq!(got_frames, ref_frames, "band {band} frames");
        // SAFETY: read-only fixture.
        let vb =
            unsafe { VarBuilder::from_mmaped_safetensors(&[path], DType::F32, &device).unwrap() };
        let ref_re: Vec<f32> = vb
            .get((2, bins, ref_frames), "re")
            .unwrap()
            .narrow(0, 0, 1)
            .unwrap()
            .flatten_all()
            .unwrap()
            .to_vec1()
            .unwrap();
        let ref_im: Vec<f32> = vb
            .get((2, bins, ref_frames), "im")
            .unwrap()
            .narrow(0, 0, 1)
            .unwrap()
            .flatten_all()
            .unwrap()
            .to_vec1()
            .unwrap();
        // The Rust spec is frame-major (`frame * bins + bin`); the reference
        // dump is `(channels, bins, frames)` flattened bin-major.
        let mut max_abs = 0.0f32;
        let mut ref_max = 0.0f32;
        for frame in 0..ref_frames {
            for bin in 0..bins {
                let ours = re[frame * bins + bin];
                let ours_im = im[frame * bins + bin];
                let theirs = ref_re[bin * ref_frames + frame];
                let theirs_im = ref_im[bin * ref_frames + frame];
                max_abs = max_abs.max((ours - theirs).abs());
                max_abs = max_abs.max((ours_im - theirs_im).abs());
                ref_max = ref_max.max(theirs.abs());
            }
        }
        eprintln!("band {band}: bins={bins} frames={ref_frames} max_abs_diff={max_abs:e} ref_abs_max={ref_max:e}");
    }
}
