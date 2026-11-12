//! Parity of the polyphase resampler against `scipy.signal.resample_poly`.

use std::path::Path;

use candle_core::{DType, Device};
use candle_nn::VarBuilder;

use candle_roformer::model::vr_resample::resample_poly;

const EVIDENCE: &str = r"F:\sources\candle-roformer\.pdca\open-issues-r5\rv1\item-3-vr-uvr";

#[test]
fn resample_poly_matches_scipy() {
    let reference = format!("{EVIDENCE}\\reference\\resample-poly-ref.safetensors");
    if !Path::new(&reference).exists() {
        eprintln!("skipping: {reference} is not available");
        return;
    }
    let device = Device::Cpu;
    // SAFETY: read-only fixture.
    let vb =
        unsafe { VarBuilder::from_mmaped_safetensors(&[reference], DType::F32, &device).unwrap() };
    let input = vb.get(2400, "input").unwrap();
    let input: Vec<f32> = input.to_vec1().unwrap();

    for (up, down) in [(1usize, 3usize), (1, 2), (2, 1), (3, 1)] {
        let name = format!("y_{up}_{down}");
        let expected: Vec<f32> = vb
            .get((2400 * up + down - 1) / down, name.as_str())
            .unwrap()
            .to_vec1()
            .unwrap();
        let got = resample_poly(&input, up, down);
        assert_eq!(got.len(), expected.len(), "length {up}/{down}");
        let mut max_abs = 0.0f32;
        for (a, b) in got.iter().zip(expected.iter()) {
            max_abs = max_abs.max((a - b).abs());
        }
        eprintln!("resample_poly {up}/{down}: max_abs_diff={max_abs:e}");
        assert!(
            max_abs <= 1e-6,
            "resample_poly {up}/{down} max abs diff {max_abs:e} exceeds 1e-6"
        );
    }
}
