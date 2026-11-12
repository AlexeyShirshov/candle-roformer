//! STFT/iSTFT round-trip fidelity for the VR band geometry.

use candle_roformer::dsp::{istft, stft_zeros};

#[test]
fn vr_band_stft_istft_roundtrip() {
    let n = 88_160usize;
    let x: Vec<f32> = (0..n)
        .map(|i| {
            let t = i as f32 / 7350.0;
            0.5 * (2.0 * std::f32::consts::PI * 220.0 * t).sin()
                + 0.25 * (2.0 * std::f32::consts::PI * 3000.0 * t).sin()
                + 0.1 * (2.0 * std::f32::consts::PI * 97.0 * t).sin()
        })
        .collect();
    for (n_fft, hop) in [(640usize, 80usize), (320, 80), (512, 160), (960, 480)] {
        let spec = stft_zeros(&x, n_fft, hop, n_fft).unwrap();
        let y = istft(&spec, n_fft, hop, n_fft, (spec.frames - 1) * hop).unwrap();
        let m = y.len().min(n);
        let mut sq = 0.0f64;
        let mut ref_sq = 0.0f64;
        let mut max_abs = 0.0f32;
        for i in 0..m {
            let d = y[i] - x[i];
            sq += (d as f64) * (d as f64);
            ref_sq += (x[i] as f64) * (x[i] as f64);
            max_abs = max_abs.max(d.abs());
        }
        let sdr = 10.0 * (ref_sq / sq).log10();
        eprintln!("n_fft={n_fft} hop={hop}: sdr={sdr:.2}dB max_abs={max_abs:e}");
    }
}
