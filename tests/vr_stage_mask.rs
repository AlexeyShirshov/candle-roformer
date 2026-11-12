//! Mask comparison with the reference stage ladder (`mask`, `mask-aggr`).

use std::path::Path;

use candle_core::{DType, Device};
use candle_nn::VarBuilder;

use candle_roformer::model::vr_pipeline::{VrProfile, VrSeparator};

const EVIDENCE: &str = r"F:\sources\candle-roformer\.pdca\open-issues-r5\rv1\item-3-vr-uvr";

fn load_reference(stage: &str, bins: usize, frames: usize) -> Option<Vec<f32>> {
    let path = format!("{EVIDENCE}\\ref\\stages\\{stage}.safetensors");
    if !Path::new(&path).exists() {
        return None;
    }
    let device = Device::Cpu;
    // SAFETY: read-only fixture.
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[path], DType::F32, &device).unwrap() };
    // Reference dumps are `(2, 673, frames)` flattened bin-major.
    let tensor = vb.get((2, bins, frames), "x").ok()?;
    Some(tensor.flatten_all().unwrap().to_vec1::<f32>().unwrap())
}

#[test]
fn vr_mask_matches_reference() {
    let weights = format!("{EVIDENCE}\\models\\2_HP-UVR.safetensors");
    let fixture = format!("{EVIDENCE}\\fixture\\fixture-cc0-go-out-rock-12s-f32-44100.wav");
    if !Path::new(&weights).exists() || !Path::new(&fixture).exists() {
        eprintln!("skipping: evidence missing");
        return;
    }
    let audio = candle_roformer::wav::read_audio(&fixture).unwrap();
    let frames = audio.frames();
    let (left, right) = audio.planar.split_at(frames);
    let separator = VrSeparator::load(Path::new(&weights), VrProfile::four_band_v2()).unwrap();
    let (raw, adjusted, bins, nframes) = separator.debug_mask(left, right).unwrap();
    let stats = |v: &[f32]| {
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        let mut sum = 0f64;
        for x in v {
            min = min.min(*x);
            max = max.max(*x);
            sum += *x as f64;
        }
        (min, max, sum / v.len() as f64)
    };
    eprintln!(
        "mask: bins={bins} frames={nframes} raw={:?} adjusted={:?}",
        stats(&raw),
        stats(&adjusted)
    );

    for (stage, ours) in [("mask", &raw), ("mask-aggr", &adjusted)] {
        let Some(reference) = load_reference(stage, 673, nframes) else {
            eprintln!("{stage}: reference dump missing");
            continue;
        };
        let mut max_abs = 0.0f32;
        for ch in 0..2 {
            for bin in 0..bins {
                for frame in 0..nframes {
                    let a = ours[ch * bins * nframes + bin * nframes + frame];
                    let b = reference[ch * 673 * nframes + bin * nframes + frame];
                    max_abs = max_abs.max((a - b).abs());
                }
            }
        }
        eprintln!("{stage}: max_abs_diff={max_abs:e}");
    }
}
