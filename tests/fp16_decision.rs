//! fp16 decision evidence for issue #11.
//!
//! The issue asks for a decision *with numerical evidence*: enable reduced
//! precision (fp16) on verified model families, or reject it explicitly, and in
//! either case keep numerics-sensitive ops in f32.
//!
//! This test gathers the evidence on a **real** checkpoint. It is `#[ignore]`d
//! because it needs the converted BS-RoFormer safetensors:
//!
//! ```sh
//! CANDLE_ROFORMER_BS_SAFETENSORS=/models/model_bs_roformer_ep_317_sdr_12.9755.safetensors \
//!   cargo test --release --test fp16_decision -- --ignored --nocapture
//! ```
//!
//! Two independent measurements:
//!
//! 1. **Structural**: can the current model run at all with f16 weights? The
//!    network is built from an f16 `VarBuilder` and driven once; the exact
//!    Candle error is printed and asserted to be a dtype rejection. There is no
//!    dtype-generic forward path in `src/model`, so this is the only way the
//!    question can be answered today without a model rewrite.
//! 2. **Numerical**: what would f16 costing buy/lose on real data? The fixture's
//!    STFT coefficients and (when a real mask dump is supplied) the model's own
//!    mask are round-tripped through f16 and resynthesised; the waveform SDR
//!    against the f32 result is the accuracy price of the precision change.
//!
//! A real mask dump is produced by any run with `CANDLE_ROFORMER_DUMP=<dir>`;
//! point `CANDLE_ROFORMER_F16_MASK` at the `bs_03_mask_*.f32` file (its
//! `*.shape` sibling is read too). Without it the mask measurement is reported
//! as unmeasured — never as zero error.

use std::path::PathBuf;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_roformer::config::SeparatorConfig;
use candle_roformer::dsp::{istft, stft, StftResult};
use candle_roformer::BSRoformer;

const N_FFT: usize = 2048;
const HOP: usize = 441;
const WIN: usize = 2048;
const SAMPLE_RATE: u32 = 44_100;
const SECONDS: usize = 2;

fn checkpoint() -> PathBuf {
    let path = std::env::var("CANDLE_ROFORMER_GPU_SMOKE_SAFETENSORS")
        .ok()
        .or_else(|| std::env::var("CANDLE_ROFORMER_BS_SAFETENSORS").ok())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| {
            panic!(
                "set CANDLE_ROFORMER_BS_SAFETENSORS (or CANDLE_ROFORMER_GPU_SMOKE_SAFETENSORS) \
                 to the converted BS-RoFormer safetensors file"
            )
        });
    PathBuf::from(path)
}

fn config() -> PathBuf {
    std::env::var("CANDLE_ROFORMER_BS_CONFIG")
        .ok()
        .or_else(|| std::env::var("CANDLE_ROFORMER_GPU_SMOKE_CONFIG").ok())
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("configs/bs_roformer_viperx_1297.yaml")
        })
}

/// Deterministic stereo fixture, identical in shape to `tests/gpu_smoke.rs`.
fn fixture() -> Vec<f32> {
    let frames = SAMPLE_RATE as usize * SECONDS;
    let mut planar = Vec::with_capacity(frames * 2);
    let mut state: u64 = 0x2545_F491_4F6C_DD1D;
    for ch in 0..2usize {
        for i in 0..frames {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let noise = ((state >> 11) as f64 / (1u64 << 52) as f64 - 1.0) as f32 * 0.02;
            let t = i as f32 / SAMPLE_RATE as f32;
            let tone = (2.0 * std::f32::consts::PI * 220.0 * t).sin() * 0.35;
            let harm = (2.0 * std::f32::consts::PI * 1_300.0 * t).sin() * 0.12;
            let phase = if ch == 0 { 0.0 } else { 0.3 };
            planar.push((tone + harm) * (1.0 - phase * 0.25) + noise);
        }
    }
    planar
}

/// Round a slice through f16 storage using Candle's own conversion, so the
/// measurement uses exactly the rounding an f16 backend would apply.
fn round_f16(values: &[f32]) -> Vec<f32> {
    let dev = Device::Cpu;
    let t = Tensor::from_vec(values.to_vec(), values.len(), &dev).expect("tensor");
    t.to_dtype(DType::F16)
        .and_then(|t| t.to_dtype(DType::F32))
        .and_then(|t| t.to_vec1::<f32>())
        .expect("f16 round-trip")
}

fn sdr_db(reference: &[f32], candidate: &[f32]) -> f64 {
    assert_eq!(reference.len(), candidate.len());
    let mut sq_err = 0f64;
    let mut sq_ref = 0f64;
    let mut max_abs = 0f64;
    for (r, c) in reference.iter().zip(candidate.iter()) {
        let (r, c) = (*r as f64, *c as f64);
        let d = c - r;
        max_abs = max_abs.max(d.abs());
        sq_err += d * d;
        sq_ref += r * r;
    }
    let sdr = if sq_err == 0.0 {
        f64::INFINITY
    } else {
        10.0 * (sq_ref / sq_err).log10()
    };
    println!("    max_abs={max_abs:.3e} sdr_db={sdr:.2}");
    sdr
}

/// (1) Structural check: an f16 weight buffer cannot drive this forward path.
#[test]
#[ignore = "needs the real converted BS-RoFormer safetensors"]
fn f16_weights_are_rejected_by_the_current_forward_path() {
    let weights = checkpoint();
    let cfg = SeparatorConfig::from_yaml_file(config()).expect("parse config");
    let dev = Device::Cpu;
    let vb = unsafe {
        VarBuilder::from_mmaped_safetensors(&[weights.to_str().expect("utf-8 path")], DType::F16, &dev)
    }
    .expect("mmap checkpoint as f16");
    let model = BSRoformer::new(&cfg, vb).expect("build the network with f16 weights");
    // 0.2 s of silence: enough for the STFT front end, far too little to be slow.
    let audio = Tensor::zeros((1, cfg.stereo as usize + 1, 8_820), DType::F32, &dev).expect("input");
    let err = model
        .forward(&audio)
        .err()
        .expect("the f32 forward path must reject f16 weights; if this ever succeeds the decision needs revisiting");
    let message = err.to_string();
    println!("[fp16] f16_weights_forward_error={message}");
    assert!(
        message.to_lowercase().contains("dtype")
            || message.to_lowercase().contains("f16")
            || message.to_lowercase().contains("f32"),
        "expected a dtype rejection, got: {message}"
    );
    println!("[fp16] structural=REJECTED (the f32 forward path has no dtype-generic branch)");
}

/// (2) Numerical cost of f16 rounding on the real STFT, and on the model's own
/// mask when a real dump is supplied.
#[test]
#[ignore = "needs the real converted BS-RoFormer safetensors"]
fn f16_rounding_cost_on_stft_and_mask() {
    let planar = fixture();
    let frames = planar.len() / 2;
    let channel0 = &planar[..frames];

    // --- STFT coefficients ------------------------------------------------
    let f32_spec = stft(channel0, N_FFT, HOP, WIN).expect("stft");
    let f32_wave = istft(&f32_spec, N_FFT, HOP, WIN, frames).expect("istft");

    let mut rounded = f32_spec.clone();
    rounded.re = round_f16(&f32_spec.re);
    rounded.im = round_f16(&f32_spec.im);
    let f16_spec = istft(&rounded, N_FFT, HOP, WIN, frames).expect("istft");
    println!(
        "[fp16] stft_coefficients frames={} bins={}",
        f32_spec.frames, f32_spec.bins
    );
    let stft_sdr = sdr_db(&f32_wave, &f16_spec);

    // --- the model's own mask (real dump, optional) ------------------------
    let mask_path = std::env::var("CANDLE_ROFORMER_F16_MASK")
        .ok()
        .filter(|v| !v.is_empty())
        .map(PathBuf::from);
    match mask_path {
        Some(path) => {
            let bytes = std::fs::read(&path).expect("read mask dump");
            let values: Vec<f32> = bytes
                .chunks_exact(4)
                .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect();
            let shape_path = path.with_extension("shape");
            let shape_text = std::fs::read_to_string(&shape_path).expect("read mask shape");
            let shape: Vec<usize> = shape_text
                .trim()
                .split(',')
                .map(|v| v.trim().parse::<usize>().expect("shape dim"))
                .collect();
            println!(
                "[fp16] mask={} shape={:?} values={}",
                path.display(),
                shape,
                values.len()
            );
            // Dumped mask layout is (batch, stems, freq*channels, time, 2): the
            // frequency axis interleaves the audio channels (`f * ch + c`) and
            // the trailing pair is the mask's (re, im). The waveform probe uses
            // channel 0.
            assert_eq!(shape.len(), 5, "unexpected mask rank in {shape:?}");
            let (b, n, freq_c, time, pairs) = (shape[0], shape[1], shape[2], shape[3], shape[4]);
            let bins = N_FFT / 2 + 1;
            assert_eq!(pairs, 2, "mask dump must carry an (re, im) pair");
            assert_eq!(
                freq_c,
                bins * 2,
                "mask dump must interleave 2 channels over {bins} bins"
            );
            assert_eq!(b * n, 1, "the decision evidence assumes one stem");
            assert!(
                time.abs_diff(f32_spec.frames) <= 1,
                "mask dump has {time} frames but the fixture STFT has {}",
                f32_spec.frames
            );
            let frames_used = time.min(f32_spec.frames);
            let mut masked_f32 = f32_spec.clone();
            let mut masked_f16 = f32_spec.clone();
            let rounded_mask = round_f16(&values);
            for t in 0..frames_used {
                for f in 0..bins {
                    let src = ((f * 2) * time + t) * 2; // channel 0 of the pair
                    let dst = t * bins + f;
                    let (re, im) = (f32_spec.re[dst], f32_spec.im[dst]);
                    let (mre, mim) = (values[src], values[src + 1]);
                    let (qre, qim) = (rounded_mask[src], rounded_mask[src + 1]);
                    masked_f32.re[dst] = re * mre - im * mim;
                    masked_f32.im[dst] = re * mim + im * mre;
                    masked_f16.re[dst] = re * qre - im * qim;
                    masked_f16.im[dst] = re * qim + im * qre;
                }
            }
            let f32_wave_masked = istft(&masked_f32, N_FFT, HOP, WIN, frames).expect("istft");
            let f16_wave_masked = istft(&masked_f16, N_FFT, HOP, WIN, frames).expect("istft");
            println!("[fp16] mask_multiplication (real mask, f16-rounded):");
            let mask_sdr = sdr_db(&f32_wave_masked, &f16_wave_masked);
            println!(
                "[fp16] summary stft_coefficients_sdr_db={stft_sdr:.2} mask_multiply_sdr_db={mask_sdr:.2}"
            );
            assert!(mask_sdr > 20.0, "f16 mask error is implausibly large");
        }
        None => {
            println!(
                "[fp16] mask_multiplication=UNMEASURED (set CANDLE_ROFORMER_F16_MASK to a \
                 `bs_03_mask_*.f32` dump produced with CANDLE_ROFORMER_DUMP=<dir>)"
            );
            println!("[fp16] summary stft_coefficients_sdr_db={stft_sdr:.2} mask_multiply_sdr_db=unmeasured");
        }
    }

    // Sanity bounds only: f16 keeps ~11 significand bits, so the measured
    // error must be small but clearly worse than the f32 path. The numbers
    // themselves (not these floors) are the decision evidence.
    assert!(
        stft_sdr > 60.0,
        "f16 STFT rounding measured {stft_sdr:.2} dB, which is worse than a \
         11-bit significand can explain"
    );
}

/// Keep the StftResult import meaningful for readers of the probe above.
#[allow(dead_code)]
fn _assert_spec_type(spec: &StftResult) -> usize {
    spec.re.len()
}
