//! Rational-ratio polyphase resampling compatible with `scipy.signal.resample_poly`.
//!
//! The UVR/VR pipeline resamples between the model's frequency bands with
//! integer ratios (1/3, 1/2, 2, 3). `librosa` (with `res_type="polyphase"`)
//! delegates to `scipy.signal.resample_poly`, whose exact behaviour is
//! reproduced here: a Kaiser-windowed sinc FIR (beta = 5, `half_len =
//! 10 * max(up, down)`), unity DC gain, upsampling by zero insertion, "same"
//! filtering and decimation with the pre/post padding scipy applies.

/// First kind modified Bessel function of order 0 (series expansion).
fn bessel_i0(x: f64) -> f64 {
    let mut sum = 1.0f64;
    let mut term = 1.0f64;
    let quarter = x * x / 4.0;
    for k in 1..64 {
        term *= quarter / (k as f64 * k as f64);
        sum += term;
        if term.abs() < 1e-18 * sum.abs() {
            break;
        }
    }
    sum
}

/// Symmetric Kaiser window, `N` points, periodic (`fftbins=True`).
fn kaiser_window(n: usize, beta: f64) -> Vec<f64> {
    let denom = bessel_i0(beta);
    (0..n)
        .map(|i| {
            let t = 2.0 * i as f64 / (n as f64 - 1.0) - 1.0;
            bessel_i0(beta * (1.0 - t * t).max(0.0).sqrt()) / denom
        })
        .collect()
}

fn sinc(x: f64) -> f64 {
    if x.abs() < 1e-12 {
        1.0
    } else {
        let p = std::f64::consts::PI * x;
        p.sin() / p
    }
}

/// `scipy.signal.firwin(2 * half_len + 1, cutoff, window=('kaiser', 5.0))`
/// with the default `scale=True` (unity gain at DC).
fn firwin_lowpass(half_len: usize, cutoff: f64) -> Vec<f64> {
    let n = 2 * half_len + 1;
    let window = kaiser_window(n, 5.0);
    let mid = (n - 1) as f64 / 2.0;
    let mut h: Vec<f64> = (0..n)
        .map(|i| cutoff * sinc(cutoff * (i as f64 - mid)) * window[i])
        .collect();
    let sum: f64 = h.iter().sum();
    if sum != 0.0 {
        for value in h.iter_mut() {
            *value /= sum;
        }
    }
    h
}

/// Output length used by `scipy.signal.upfirdn`.
fn upfirdn_len(h_len: usize, n_in: usize, up: usize, down: usize) -> usize {
    let numerator = (n_in.saturating_sub(1)) * up + h_len;
    (numerator + down - 1) / down
}

/// `scipy.signal.resample_poly(x, up, down)` for a mono f32 signal.
pub fn resample_poly(x: &[f32], up: usize, down: usize) -> Vec<f32> {
    if x.is_empty() || up == 0 || down == 0 {
        return Vec::new();
    }
    let g = gcd(up, down);
    let up = up / g;
    let down = down / g;
    if up == down {
        return x.to_vec();
    }

    let max_rate = up.max(down);
    let half_len = 10 * max_rate;
    let mut h = firwin_lowpass(half_len, 1.0 / max_rate as f64);
    for value in h.iter_mut() {
        *value *= up as f64;
    }

    let n_in = x.len();
    let n_out = (n_in * up + down - 1) / down;

    let n_pre_pad = down - (half_len % down);
    let n_pre_remove = (half_len + n_pre_pad) / down;
    let mut n_post_pad = 0usize;
    while upfirdn_len(h.len() + n_pre_pad + n_post_pad, n_in, up, down) < n_out + n_pre_remove {
        n_post_pad += 1;
    }

    let mut filter = vec![0.0f64; n_pre_pad];
    filter.extend_from_slice(&h);
    filter.extend(std::iter::repeat_n(0.0f64, n_post_pad));

    let out_len = upfirdn_len(filter.len(), n_in, up, down);
    let mut y = vec![0.0f64; out_len];
    // upfirdn: y[n] = sum_k filter[k] * x_up[n * down - k], where x_up places
    // x[i] at position i * up and is zero elsewhere.
    for (n, slot) in y.iter_mut().enumerate() {
        let base = (n * down) as i64;
        let mut acc = 0.0f64;
        for (k, coeff) in filter.iter().enumerate() {
            let pos = base - k as i64;
            if pos < 0 {
                break;
            }
            if pos % up as i64 != 0 {
                continue;
            }
            let idx = pos / up as i64;
            if idx as usize >= n_in {
                continue;
            }
            acc += coeff * x[idx as usize] as f64;
        }
        *slot = acc;
    }

    let start = n_pre_remove.min(y.len());
    let end = (start + n_out).min(y.len());
    let mut out: Vec<f32> = y[start..end].iter().map(|v| *v as f32).collect();
    if out.len() < n_out {
        out.resize(n_out, 0.0);
    }
    out
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Resampling used by the *synthesis* path of the VR reference.
///
/// `spec_utils.wav_resolution` is `"sinc_fastest"` on non-ARM hosts, which
/// `librosa` routes to `libsamplerate`. Using the same library and converter is
/// the only way to stay inside the waveform tolerance.
pub fn resample_sinc_fastest(input: &[f32], from: u32, to: u32) -> Result<Vec<f32>, String> {
    if input.is_empty() || from == to {
        return Ok(input.to_vec());
    }
    samplerate::convert(from, to, 1, samplerate::ConverterType::SincFastest, input)
        .map_err(|err| format!("libsamplerate error: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_ratio_is_a_copy() {
        let x: Vec<f32> = (0..32).map(|i| i as f32).collect();
        assert_eq!(resample_poly(&x, 3, 3), x);
    }

    #[test]
    fn constant_signal_stays_constant() {
        let x = vec![0.25f32; 480];
        for (up, down) in [(1usize, 3usize), (1, 2), (2, 1), (3, 1)] {
            let y = resample_poly(&x, up, down);
            let expected = (x.len() * up + down - 1) / down;
            assert_eq!(y.len(), expected, "length for {up}/{down}");
            // Exact scipy parity is covered by `tests/vr_resample.rs`; here we
            // only assert the output stays close to the input DC level in the
            // interior (the Kaiser lowpass has a small passband ripple, exactly
            // like scipy's).
            let half = 4 * 10 * up.max(down) + 4;
            if y.len() <= 2 * half + 1 {
                continue;
            }
            for v in &y[half..y.len() - half] {
                assert!((v - 0.25).abs() < 1e-2, "constant {v} for {up}/{down}");
            }
        }
    }
}
