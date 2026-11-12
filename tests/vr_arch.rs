//! Numerical parity for the VR Arch (UVR) network port.
//!
//! The reference mask is produced by `audio-separator` 0.48.0 / PyTorch 2.15.0
//! from the published `2_HP-UVR.pth` checkpoint on a deterministic input patch
//! (`torch.rand * 0.2`, seed 1234). The test is skipped when the (gitignored)
//! evidence artifacts are not present on this machine.

use std::path::Path;

use candle_core::{DType, Device};
use candle_nn::VarBuilder;

use candle_roformer::model::vr_arch::CascadedAsppNet;

const EVIDENCE: &str = r"F:\sources\candle-roformer\.pdca\open-issues-r5\rv1\item-3-vr-uvr";
/// Internal diagnostic bound for the network alone. The acceptance gate for
/// item #3 is the *waveform* parity against audio-separator (max_abs <= 1e-5,
/// RMSE <= 1e-6, SDR >= 100 dB, corr >= 0.999999); this mask check only guards
/// the port from structural mistakes.
const TOLERANCE: f32 = 2e-5;

#[test]
fn vr_arch_mask_matches_pytorch_reference() {
    let weights = format!("{EVIDENCE}\\models\\2_HP-UVR.safetensors");
    let reference = format!("{EVIDENCE}\\reference\\mask-ref-1x2x672x768.safetensors");
    if !Path::new(&weights).exists() || !Path::new(&reference).exists() {
        eprintln!("skipping: evidence artifacts are not available at {EVIDENCE}");
        return;
    }

    let device = Device::Cpu;
    // SAFETY: both files are read-only fixtures produced by the offline converter.
    let vb =
        unsafe { VarBuilder::from_mmaped_safetensors(&[weights], DType::F32, &device).unwrap() };
    let model = CascadedAsppNet::load(vb, 1344, 123812).unwrap();

    // SAFETY: read-only reference fixture.
    let refs =
        unsafe { VarBuilder::from_mmaped_safetensors(&[reference], DType::F32, &device).unwrap() };
    let input = refs.get((1, 2, 672, 768), "input").unwrap();
    let expected = refs.get((1, 2, 673, 512), "mask").unwrap();

    let got = model.predict_mask(&input).unwrap();
    assert_eq!(got.dims(), expected.dims(), "mask shape mismatch");

    let diff = (got - expected).unwrap().abs().unwrap();
    let max_abs = diff
        .flatten_all()
        .unwrap()
        .max(0)
        .unwrap()
        .to_scalar::<f32>()
        .unwrap();
    let mean_abs = diff
        .flatten_all()
        .unwrap()
        .mean(0)
        .unwrap()
        .to_scalar::<f32>()
        .unwrap();

    // Locate the worst error: padded bin row vs. the network's own rows.
    let body = diff.narrow(2, 0, 672).unwrap();
    let body_max = body
        .flatten_all()
        .unwrap()
        .max(0)
        .unwrap()
        .to_scalar::<f32>()
        .unwrap();
    let pad_row = diff.narrow(2, 672, 1).unwrap();
    let pad_max = pad_row
        .flatten_all()
        .unwrap()
        .max(0)
        .unwrap()
        .to_scalar::<f32>()
        .unwrap();
    let interior = diff.narrow(3, 1, 510).unwrap();
    let interior_max = interior
        .flatten_all()
        .unwrap()
        .max(0)
        .unwrap()
        .to_scalar::<f32>()
        .unwrap();
    eprintln!(
        "VR arch mask parity: max_abs={max_abs:e} mean_abs={mean_abs:e} body_max={body_max:e} pad_row_max={pad_max:e} interior_max={interior_max:e}"
    );
    assert!(
        max_abs <= TOLERANCE,
        "mask max abs difference {max_abs:e} exceeds {TOLERANCE:e}"
    );
}
