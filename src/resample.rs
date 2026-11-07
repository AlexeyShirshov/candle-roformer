pub fn resample(input: &[f32], from_sr: u32, to_sr: u32) -> Vec<f32> {
    resample_planar(input, 1, from_sr, to_sr)
}

pub fn resample_planar(input: &[f32], channels: usize, from_sr: u32, to_sr: u32) -> Vec<f32> {
    if channels == 0 {
        return Vec::new();
    }
    if from_sr == to_sr {
        return input.to_vec();
    }
    let frames = input.len() / channels;
    let ratio = to_sr as f64 / from_sr as f64;
    let out_frames = ((frames as f64) * ratio).round() as usize;
    let mut out = vec![0f32; out_frames * channels];
    let half = 16i64;
    for ch in 0..channels {
        for n in 0..out_frames {
            let t = n as f64 / to_sr as f64;
            let src_pos = t * from_sr as f64;
            let i0 = src_pos.floor() as i64;
            let mut acc = 0.0f64;
            for k in -half..=half {
                let i = i0 + k;
                if i < 0 || i >= frames as i64 {
                    continue;
                }
                let x = src_pos - i as f64;
                let w = sinc(x) * hann_window(x, half as f64);
                acc += input[ch + i as usize * channels] as f64 * w;
            }
            out[ch + n * channels] = acc as f32;
        }
    }
    out
}

fn sinc(x: f64) -> f64 {
    if x.abs() < 1e-12 {
        1.0
    } else {
        let px = std::f64::consts::PI * x;
        px.sin() / px
    }
}

fn hann_window(x: f64, half: f64) -> f64 {
    let ax = x.abs();
    if ax >= half {
        0.0
    } else {
        0.5 * (1.0 + (std::f64::consts::PI * x / half).cos())
    }
}
