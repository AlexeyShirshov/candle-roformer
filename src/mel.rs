// Mel filterbank membership following librosa.filters.mel (slaney normalization).
// Computed in f64; endpoints forced positive; membership is `> 0`; every one of
// (n_fft/2 + 1) bins must be covered (upstream asserts this).

fn hz_to_mel(hz: f64) -> f64 {
    let f_min = 0.0f64;
    let f_sp = 200.0f64 / 3.0;
    let min_log_hz = 1000.0f64;
    let min_log_mel = (min_log_hz - f_min) / f_sp;
    let log_step = (6.4f64).ln() / 27.0f64;
    if hz < min_log_hz {
        (hz - f_min) / f_sp
    } else {
        min_log_mel + (hz / min_log_hz).ln() / log_step
    }
}

fn mel_to_hz(mel: f64) -> f64 {
    let f_min = 0.0f64;
    let f_sp = 200.0f64 / 3.0;
    let min_log_hz = 1000.0f64;
    let min_log_mel = (min_log_hz - f_min) / f_sp;
    let log_step = (6.4f64).ln() / 27.0f64;
    if mel < min_log_mel {
        f_min + f_sp * mel
    } else {
        min_log_hz * (log_step * (mel - min_log_mel)).exp()
    }
}

pub fn mel_frequencies(n_mels: usize, fmin: f64, fmax: f64) -> Vec<f64> {
    let mel_min = hz_to_mel(fmin);
    let mel_max = hz_to_mel(fmax);
    (0..=n_mels + 1)
        .map(|i| {
            let mel = mel_min + (mel_max - mel_min) * i as f64 / (n_mels + 1) as f64;
            mel_to_hz(mel)
        })
        .collect()
}

pub fn mel_filterbank(n_fft: usize, sample_rate: usize, n_mels: usize) -> Vec<Vec<f64>> {
    let fmax = sample_rate as f64 / 2.0;
    let hz = mel_frequencies(n_mels, 0.0, fmax);
    let n_bins = n_fft / 2 + 1;
    let fft_freqs: Vec<f64> = (0..n_bins)
        .map(|i| i as f64 * sample_rate as f64 / n_fft as f64)
        .collect();

    let mut weights = vec![vec![0.0f64; n_mels]; n_bins];
    for i in 0..n_mels {
        let f_left = hz[i];
        let f_center = hz[i + 1];
        let f_right = hz[i + 2];
        let denom_left = (f_center - f_left).max(f64::EPSILON);
        let denom_right = (f_right - f_center).max(f64::EPSILON);
        for (bin, &freq) in fft_freqs.iter().enumerate() {
            let lower = (freq - f_left) / denom_left;
            let upper = (f_right - freq) / denom_right;
            let w = lower.min(upper).max(0.0);
            weights[bin][i] = w;
        }
    }

    // Slaney normalization.
    for i in 0..n_mels {
        let enorm = 2.0 / (hz[i + 2] - hz[i]).max(f64::EPSILON);
        for bin in 0..n_bins {
            weights[bin][i] *= enorm;
        }
    }

    // Force endpoints (upstream). Only when there is at least one band.
    if n_mels > 0 {
        weights[0][0] = 1.0;
        weights[n_bins - 1][n_mels - 1] = 1.0;
    }

    weights
}

pub fn mel_bin_membership(n_fft: usize, sample_rate: usize, n_mels: usize) -> Vec<Vec<usize>> {
    let weights = mel_filterbank(n_fft, sample_rate, n_mels);
    let n_bins = n_fft / 2 + 1;

    let mut membership = vec![Vec::new(); n_mels];
    for bin in 0..n_bins {
        let mut assigned = false;
        for band in 0..n_mels {
            if weights[bin][band] > 0.0 {
                membership[band].push(bin);
                assigned = true;
            }
        }
        // No nearest-band fallback: an orphan bin is a hard error (upstream assert).
        if !assigned {
            panic!("mel filterbank left an orphan frequency bin {}", bin);
        }
    }
    // require every band non-empty
    for band in 0..n_mels {
        if membership[band].is_empty() {
            panic!("mel band {} is empty", band);
        }
    }
    // coverage assert
    let mut covered = vec![false; n_bins];
    for band in 0..n_mels {
        for &bin in &membership[band] {
            covered[bin] = true;
        }
    }
    if !covered.iter().all(|x| *x) {
        panic!("not all {} bins covered", n_bins);
    }
    membership
}
