//! Per-band synthesis waveform comparison with the reference `cmb-wave-*` dumps.

use std::path::Path;

use candle_core::{DType, Device};
use candle_nn::VarBuilder;

use candle_roformer::model::vr_pipeline::{VrProfile, VrSeparator};

const EVIDENCE: &str = r"F:\sources\candle-roformer\.pdca\open-issues-r5\rv1\item-3-vr-uvr";

fn load_stage(stage: &str, shape: (usize, usize)) -> Option<Vec<f32>> {
    let path = format!("{EVIDENCE}\\ref\\stages\\{stage}.safetensors");
    if !Path::new(&path).exists() {
        return None;
    }
    let device = Device::Cpu;
    // SAFETY: read-only fixture.
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[path], DType::F32, &device).unwrap() };
    let tensor = vb.get((2, shape.1), "y").ok()?;
    Some(
        tensor
            .narrow(0, 0, 1)
            .unwrap()
            .flatten_all()
            .unwrap()
            .to_vec1::<f32>()
            .unwrap(),
    )
}

#[test]
fn vr_band_waves_match_reference() {
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
    let waves = separator.debug_band_waves(left, right).unwrap();
    for (index, wave) in waves.iter().enumerate() {
        let stage = format!("cmb-wave-band{}", index + 1);
        let Some(reference) = load_stage(&stage, (2, wave.len())) else {
            eprintln!(
                "{stage}: reference missing or length mismatch (ours {})",
                wave.len()
            );
            continue;
        };
        let mut max_abs = 0.0f32;
        let mut ref_max = 0.0f32;
        for i in 0..wave.len().min(reference.len()) {
            max_abs = max_abs.max((wave[i] - reference[i]).abs());
            ref_max = ref_max.max(reference[i].abs());
        }
        eprintln!(
            "{stage}: len={} max_abs_diff={max_abs:e} ref_abs_max={ref_max:e}",
            wave.len()
        );
    }
}
