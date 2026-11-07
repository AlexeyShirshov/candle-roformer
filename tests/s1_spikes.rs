use candle_core::{Device, Tensor};
use candle_roformer::dsp::{istft, stft, STFT_HOP_V7, STFT_N_FFT_V7, STFT_WIN_V7};
use candle_roformer::{
    average_by_counts, gather_1d, mel_bin_membership, scatter_add_1d, DeviceChoice, SeparatorConfig,
};
#[test]
fn separator_config_v7_validates() {
    let cfg = SeparatorConfig::v7();
    cfg.validate().unwrap();
    assert_eq!(cfg.dim, 256);
    assert_eq!(cfg.depth, 12);
    assert!(cfg.stereo);
    assert_eq!(cfg.num_stems, 1);
    assert_eq!(cfg.num_bands, 60);
    assert_eq!(cfg.dim_head, 64);
    assert_eq!(cfg.heads, 8);
    assert!(cfg.flash_attn);
    assert_eq!(cfg.dim_freqs_in, 1025);
    assert_eq!(cfg.sample_rate, 44100);
    assert_eq!(cfg.stft_n_fft, 2048);
    assert_eq!(cfg.stft_hop_length, 441);
    assert_eq!(cfg.stft_win_length, 2048);
    assert!(!cfg.stft_normalized);
    assert_eq!(cfg.mask_estimator_depth, 2);
    assert!(cfg.use_torch_checkpoint);
    assert_eq!(cfg.time_transformer_depth, 1);
    assert_eq!(cfg.freq_transformer_depth, 1);
    assert_eq!(cfg.mlp_expansion_factor, 4);
    assert_eq!(cfg.linear_transformer_depth, 0);
    assert!(cfg.norm_output);
    assert!(!cfg.skip_connection);
    assert!(cfg.zero_dc);
}

#[test]
fn stft_istft_roundtrip() {
    // 0.25 s of a sine at 440 Hz, enough frames.
    let sr = 44100usize;
    let n = sr / 4;
    let input: Vec<f32> = (0..n)
        .map(|i| (2.0 * std::f32::consts::PI * 440.0 * i as f32 / sr as f32).sin() * 0.5)
        .collect();
    let spec = stft(&input, STFT_N_FFT_V7, STFT_HOP_V7, STFT_WIN_V7).unwrap();
    assert_eq!(spec.bins, STFT_N_FFT_V7 / 2 + 1);
    assert!(spec.frames > 0);
    let recon = istft(&spec, STFT_N_FFT_V7, STFT_HOP_V7, STFT_WIN_V7, input.len()).unwrap();
    assert_eq!(recon.len(), input.len());
    let max_err = input
        .iter()
        .zip(recon.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);
    // f32 tolerance for this overlap configuration.
    assert!(max_err < 1e-3, "max reconstruction error = {max_err}");
}

#[test]
fn mel_membership_covers_all_bins_and_bands() {
    let m = mel_bin_membership(2048, 44100, 60);
    assert_eq!(m.len(), 60);
    let mut covered = vec![false; 1025];
    for band in &m {
        assert!(!band.is_empty(), "empty mel band");
        for &bin in band {
            assert!(bin < 1025);
            covered[bin] = true;
        }
    }
    assert!(covered.iter().all(|x| *x), "not all 1025 bins covered");
}

#[test]
fn device_choice_cpu_resolves() {
    assert!(DeviceChoice::Cpu.resolve().is_ok());
}

#[test]
fn device_choice_cuda_no_fallback() {
    let err = DeviceChoice::Cuda.resolve().unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Cuda") || msg.to_lowercase().contains("cuda"));
}

#[test]
fn device_choice_metal_no_fallback() {
    let err = DeviceChoice::Metal.resolve().unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Metal") || msg.to_lowercase().contains("metal"));
}

#[test]
fn gather_scatter_add_matches_reference() {
    let input = Tensor::from_vec(
        (0..6).map(|x| x as f32).collect::<Vec<_>>(),
        6,
        &Device::Cpu,
    )
    .unwrap();
    let indices = vec![1usize, 3, 3, 5];
    let gathered = gather_1d(&input, &indices).unwrap();
    let got: Vec<f32> = gathered.to_vec1().unwrap();
    let expected: Vec<f32> = indices.iter().map(|&i| i as f32).collect();
    assert_eq!(got, expected);

    let source = Tensor::from_vec(vec![1.0f32, 2.0, 3.0, 4.0], 4, &Device::Cpu).unwrap();
    let scatter_idx = vec![0usize, 1, 1, 2];
    let out = scatter_add_1d(3, &source, &scatter_idx, &Device::Cpu).unwrap();
    let got_out: Vec<f32> = out.to_vec1().unwrap();
    assert_eq!(got_out, vec![1.0, 5.0, 4.0]);

    let counts = Tensor::from_vec(vec![1.0f32, 2.0, 1.0], 3, &Device::Cpu).unwrap();
    let avg = average_by_counts(&out, &counts).unwrap();
    let got_avg: Vec<f32> = avg.to_vec1().unwrap();
    assert_eq!(got_avg, vec![1.0, 2.5, 4.0]);
}

#[test]
fn stft_short_inputs_defined() {
    // Boundaries: lengths that cannot be reflected faithfully must be typed errors.
    for &len in &[1usize, 2usize] {
        let input = vec![0.1f32; len];
        let res = stft(&input, STFT_N_FFT_V7, STFT_HOP_V7, STFT_WIN_V7);
        assert!(res.is_err(), "len={} should be rejected", len);
    }
}

#[test]
fn stft_boundary_lengths_reconstruct() {
    // Boundaries around 2*border (border = n_fft/2 = 1024).
    for &len in &[2047usize, 2048usize, 2049usize] {
        let input: Vec<f32> = (0..len)
            .map(|i| (i as f32 * 0.01).sin() * 0.5)
            .collect();
        let spec = stft(&input, STFT_N_FFT_V7, STFT_HOP_V7, STFT_WIN_V7).unwrap();
        assert!(spec.frames > 0);
        let recon = istft(&spec, STFT_N_FFT_V7, STFT_HOP_V7, STFT_WIN_V7, input.len()).unwrap();
        assert_eq!(recon.len(), input.len());
        let max_err = input.iter().zip(recon.iter()).map(|(a,b)|(a-b).abs()).fold(0.0f32, f32::max);
        assert!(max_err < 1e-3, "len={} max_err={}", len, max_err);
    }
}


#[test]
#[should_panic(expected = "orphan frequency bin")]
fn mel_orphan_bin_is_hard_error() {
    // All bands (vacuously) non-empty and exactly one uncovered frequency bin:
    // with zero mel bands, bin 0 is uncovered, so the orphan guard must fire
    // and name the bin index. Deleting the guard makes this test fail because
    // control falls through to the coverage assertion instead.
    let _ = mel_bin_membership(2, 44100, 0);
}

#[test]
fn mel_orphan_bin_positive_twin_full_coverage() {
    // Positive twin: full coverage (60 bands, 1025 bins) passes without panic.
    let m = mel_bin_membership(2048, 44100, 60);
    assert_eq!(m.len(), 60);
    assert!(m.iter().all(|b| !b.is_empty()));
}
