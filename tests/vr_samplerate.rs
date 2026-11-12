//! Parity of the synthesis resampler (`libsamplerate`, `sinc_fastest`) against
//! the Python binding used by `audio-separator` through `librosa`.

use std::path::Path;

use candle_core::{DType, Device};
use candle_nn::VarBuilder;

use candle_roformer::model::vr_resample::resample_sinc_fastest;

const EVIDENCE: &str = r"F:\sources\candle-roformer\.pdca\open-issues-r5\rv1\item-3-vr-uvr";

#[test]
fn sinc_fastest_matches_libsamplerate() {
    let reference = format!("{EVIDENCE}\\reference\\libsamplerate-ref.safetensors");
    if !Path::new(&reference).exists() {
        eprintln!("skipping: {reference} is not available");
        return;
    }
    let device = Device::Cpu;
    // SAFETY: read-only fixture.
    let vb =
        unsafe { VarBuilder::from_mmaped_safetensors(&[reference], DType::F32, &device).unwrap() };
    let input: Vec<f32> = vb.get(3000, "input").unwrap().to_vec1().unwrap();

    for (tag, from, to) in [
        ("7350_14700", 7350u32, 14700u32),
        ("14700_44100", 14700, 44100),
    ] {
        let expected: Vec<f32> = vb
            .get(3000 * (to / from) as usize, format!("y_{tag}").as_str())
            .unwrap()
            .to_vec1()
            .unwrap();
        let got = resample_sinc_fastest(&input, from, to).unwrap();
        assert_eq!(got.len(), expected.len(), "length {tag}");
        let mut max_abs = 0.0f32;
        for (a, b) in got.iter().zip(expected.iter()) {
            max_abs = max_abs.max((a - b).abs());
        }
        eprintln!("sinc_fastest {tag}: max_abs_diff={max_abs:e}");
        assert!(
            max_abs <= 1e-6,
            "sinc_fastest {tag} max abs diff {max_abs:e} exceeds 1e-6"
        );
    }
}
