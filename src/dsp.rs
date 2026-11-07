use std::f32::consts::PI;

use crate::error::DspError;

pub const STFT_N_FFT_V7: usize = 2048;
pub const STFT_HOP_V7: usize = 441;
pub const STFT_WIN_V7: usize = 2048;

#[derive(Debug, Clone)]
pub struct StftResult {
    pub re: Vec<f32>,
    pub im: Vec<f32>,
    pub frames: usize,
    pub bins: usize,
}

pub fn hann_window_periodic(n: usize) -> Vec<f32> {
    if n == 0 {
        return Vec::new();
    }
    (0..n)
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / n as f32).cos()))
        .collect()
}

fn reflect_pad(input: &[f32], pad: usize) -> Vec<f32> {
    let n = input.len();
    let mut out = Vec::with_capacity(n + 2 * pad);
    for i in 0..pad {
        let r = (pad - i) as isize;
        let reflected = if n <= 1 {
            0usize
        } else {
            let n_i = n as isize;
            let two_n_minus_2 = 2 * n_i - 2;
            let idx = if r < n_i { r } else { (two_n_minus_2 - r).max(0).min(n_i - 1) };
            idx as usize
        };
        let reflected = reflected.min(n.saturating_sub(1));
        out.push(input[reflected]);
    }
    out.extend_from_slice(input);
    for i in 0..pad {
        let reflected = if n <= 1 {
            0usize
        } else {
            let n_i = n as isize;
            let r = (n + i) as isize;
            let two_n_minus_2 = 2 * n_i - 2;
            let m = two_n_minus_2 - r;
            let idx = m.max(0).min(n_i - 1);
            idx as usize
        };
        let reflected = reflected.min(n.saturating_sub(1));
        out.push(input[reflected]);
    }
    out
}

pub fn stft(
    input: &[f32],
    n_fft: usize,
    hop: usize,
    win_length: usize,
) -> Result<StftResult, DspError> {
    if n_fft == 0 || n_fft % 2 != 0 {
        return Err(DspError::InvalidParams(
            "n_fft must be positive even".into(),
        ));
    }
    if hop == 0 {
        return Err(DspError::InvalidParams("hop must be > 0".into()));
    }
    if win_length == 0 || win_length > n_fft {
        return Err(DspError::InvalidParams(
            "win_length must be in 1..=n_fft".into(),
        ));
    }
    if input.is_empty() {
        return Err(DspError::InvalidParams("input must not be empty".into()));
    }
    if win_length != n_fft {
        // Keep the spike focused on the v7 profile (win == n_fft).
        return Err(DspError::InvalidParams(
            "this spike requires win_length == n_fft".into(),
        ));
    }

    let pad = n_fft / 2;
    // Reflect padding (torch/NumPy 'reflect') requires input longer than pad;
    // otherwise it cannot be performed faithfully -> typed error, no silent substitution.
    if input.len() <= pad {
        return Err(DspError::InvalidParams(format!(
            "input too short for reflect padding: need length > {} (pad), got {}",
            pad, input.len()
        )));
    }
    let padded = reflect_pad(input, pad);
    if padded.len() < n_fft {
        return Err(DspError::InvalidParams("input too short".into()));
    }
    let frames = (padded.len() - n_fft) / hop + 1;
    let bins = n_fft / 2 + 1;

    let window = hann_window_periodic(win_length);
    let mut re = vec![0.0f32; frames * bins];
    let mut im = vec![0.0f32; frames * bins];

    let mut planner = rustfft::FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(n_fft);

    for frame in 0..frames {
        let start = frame * hop;
        let mut buf = vec![rustfft::num_complex::Complex::new(0.0f32, 0.0f32); n_fft];
        for i in 0..n_fft {
            buf[i] = rustfft::num_complex::Complex::new(padded[start + i] * window[i], 0.0);
        }
        fft.process(&mut buf);
        for b in 0..bins {
            re[frame * bins + b] = buf[b].re;
            im[frame * bins + b] = buf[b].im;
        }
    }

    Ok(StftResult {
        re,
        im,
        frames,
        bins,
    })
}

pub fn istft(
    spec: &StftResult,
    n_fft: usize,
    hop: usize,
    win_length: usize,
    length: usize,
) -> Result<Vec<f32>, DspError> {
    if n_fft == 0 || n_fft % 2 != 0 {
        return Err(DspError::InvalidParams(
            "n_fft must be positive even".into(),
        ));
    }
    if hop == 0 {
        return Err(DspError::InvalidParams("hop must be > 0".into()));
    }
    if win_length == 0 || win_length > n_fft {
        return Err(DspError::InvalidParams(
            "win_length must be in 1..=n_fft".into(),
        ));
    }
    if spec.bins != n_fft / 2 + 1 {
        return Err(DspError::ShapeMismatch(format!(
            "expected {} bins, got {}",
            n_fft / 2 + 1,
            spec.bins
        )));
    }
    if win_length != n_fft {
        return Err(DspError::InvalidParams(
            "this spike requires win_length == n_fft".into(),
        ));
    }
    if spec.frames == 0 {
        return Err(DspError::InvalidParams("spec has no frames".into()));
    }

    let window = hann_window_periodic(win_length);
    let mut window_full = vec![0.0f32; n_fft];
    window_full.copy_from_slice(&window);

    let padded_len = (spec.frames - 1) * hop + n_fft;
    let mut out = vec![0.0f32; padded_len];
    let mut wsum = vec![0.0f32; padded_len];

    let mut planner = rustfft::FftPlanner::<f32>::new();
    let ifft = planner.plan_fft_inverse(n_fft);

    for frame in 0..spec.frames {
        let mut buf = vec![rustfft::num_complex::Complex::new(0.0f32, 0.0f32); n_fft];
        for b in 0..spec.bins {
            buf[b] = rustfft::num_complex::Complex::new(
                spec.re[frame * spec.bins + b],
                spec.im[frame * spec.bins + b],
            );
        }
        // Hermitian symmetry for the upper half.
        for k in spec.bins..n_fft {
            let src = n_fft - k;
            buf[k] = rustfft::num_complex::Complex::new(
                spec.re[frame * spec.bins + src],
                -spec.im[frame * spec.bins + src],
            );
        }

        ifft.process(&mut buf);
        let scale = 1.0 / n_fft as f32;
        for i in 0..n_fft {
            let t = buf[i].re * scale;
            let w = window_full[i];
            let pos = frame * hop + i;
            out[pos] += t * w;
            wsum[pos] += w * w;
        }
    }

    for i in 0..padded_len {
        if wsum[i].abs() > 1e-8 {
            out[i] /= wsum[i];
        }
    }

    let pad = n_fft / 2;
    let start = pad.min(padded_len);
    let end = (start + length).min(padded_len);
    let mut result = out[start..end].to_vec();
    if result.len() < length {
        result.resize(length, 0.0);
    }
    Ok(result)
}
