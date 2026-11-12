//! Isolated per-backend GPU smoke tests (issue #11, gate G2).
//!
//! Every test here is **opt-in** (`#[ignore]`), so an ordinary `cargo test`
//! stays CPU-only and never claims a GPU was exercised. Each test is compiled
//! only when its backend feature is enabled (`cuda` / `metal`, see
//! `Cargo.toml`), and it runs one real checkpoint through the public
//! [`Separator`] API on the GPU backend and on the CPU, then compares the two
//! waveforms.
//!
//! What one run reports (one line per metric, greppable):
//!
//! * `backend=…`, the resolved device and its capability where available;
//! * `checkpoint=… bytes=… family=…`, the real weights actually used;
//! * `fixture=… frames=… chunk=…`, the deterministic fixture and chunking;
//! * per stem `max_abs=… rmse=… sdr_db=… corr=…` against the CPU reference;
//! * `peak_device_mib=… source=…`, the peak device memory observed while the
//!   backend run is in flight;
//! * `timing cpu_seconds=… gpu_seconds=…`.
//!
//! The run is a failure — never a pass — when the backend is unavailable, the
//! checkpoint is missing, nothing was compared, or a stem is silent: a vacuous
//! run cannot be green.
//!
//! ```sh
//! # CUDA host (weights converted once with tools/convert_bs_roformer.py):
//! CANDLE_ROFORMER_GPU_SMOKE_SAFETENSORS=/models/model_bs_roformer_ep_317_sdr_12.9755.safetensors \
//! CANDLE_ROFORMER_GPU_SMOKE_CONFIG=configs/bs_roformer_viperx_1297.yaml \
//!   cargo test --release --features cuda --test gpu_smoke -- --ignored --nocapture
//! ```
//!
//! `CANDLE_ROFORMER_GPU_SMOKE_SAFETENSORS`/`CANDLE_ROFORMER_GPU_SMOKE_CONFIG`
//! fall back to the `CANDLE_ROFORMER_BS_SAFETENSORS`/`CANDLE_ROFORMER_BS_CONFIG`
//! pair used by `tests/bs_roformer.rs`; the config falls back to the published
//! BS-RoFormer profile in `configs/`.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use candle_roformer::{AudioBuffer, DeviceChoice, LocalSeparationAssets, Separator};

/// Tolerance for a GPU backend against the CPU reference on the same weights.
///
/// These are deliberately loose enough for f32 reduction-order differences
/// (cuBLAS/Metal vs the CPU kernels) and far too tight for a wrong kernel: a
/// broken or mis-wired backend lands near 0 dB SDR, not at 60 dB.
const MAX_ABS_LIMIT: f64 = 1e-3;
const RMSE_LIMIT: f64 = 1e-4;
const SDR_MIN_DB: f64 = 60.0;
const CORR_MIN: f64 = 0.999;

/// Fixture length. Short on purpose: the smoke test proves the backend path,
/// it is not a throughput benchmark. Both backends see exactly this buffer.
const FIXTURE_SECONDS: usize = 2;

// ---------------------------------------------------------------------------
// Deterministic fixture: two channels of tones plus seeded noise. No external
// audio file, no RNG dependency, byte-identical on every host and platform.
// ---------------------------------------------------------------------------

fn fixture(sample_rate: u32) -> AudioBuffer {
    let frames = sample_rate as usize * FIXTURE_SECONDS;
    let mut planar = Vec::with_capacity(frames * 2);
    let mut state: u64 = 0x2545_F491_4F6C_DD1D; // fixed seed
    let mut next_noise = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        // map to [-1, 1)
        ((state >> 11) as f64 / (1u64 << 52) as f64 - 1.0) as f32
    };
    for ch in 0..2usize {
        for i in 0..frames {
            let t = i as f32 / sample_rate as f32;
            // A 220 Hz "vocal-ish" tone, a 1.3 kHz harmonic and low-level noise.
            let tone = (2.0 * std::f32::consts::PI * 220.0 * t).sin() * 0.35;
            let harm = (2.0 * std::f32::consts::PI * 1_300.0 * t).sin() * 0.12;
            let noise = next_noise() * 0.02;
            let phase = if ch == 0 { 0.0 } else { 0.3 };
            planar.push((tone + harm) * (1.0 - phase * 0.25) + noise);
        }
    }
    AudioBuffer {
        sample_rate,
        channels: 2,
        planar,
    }
}

// ---------------------------------------------------------------------------
// Waveform comparison
// ---------------------------------------------------------------------------

struct Metrics {
    max_abs: f64,
    rmse: f64,
    sdr_db: f64,
    corr: f64,
    samples: usize,
}

fn compare(reference: &[f32], candidate: &[f32]) -> Metrics {
    assert_eq!(
        reference.len(),
        candidate.len(),
        "stem lengths differ: reference {} vs candidate {}",
        reference.len(),
        candidate.len()
    );
    assert!(!reference.is_empty(), "empty stem: nothing to compare");
    let n = reference.len();
    let mut max_abs = 0f64;
    let mut sq_err = 0f64;
    let mut sq_ref = 0f64;
    let mut sum_ref = 0f64;
    let mut sum_cand = 0f64;
    for (r, c) in reference.iter().zip(candidate.iter()) {
        assert!(
            r.is_finite() && c.is_finite(),
            "non-finite sample: reference {r} candidate {c}"
        );
        let (r, c) = (*r as f64, *c as f64);
        let d = c - r;
        max_abs = max_abs.max(d.abs());
        sq_err += d * d;
        sq_ref += r * r;
        sum_ref += r;
        sum_cand += c;
    }
    let rmse = (sq_err / n as f64).sqrt();
    let sdr_db = if sq_err == 0.0 {
        f64::INFINITY
    } else {
        10.0 * (sq_ref / sq_err).log10()
    };
    // Pearson correlation, computed from the same pass plus a second one for
    // the cross term.
    let mean_r = sum_ref / n as f64;
    let mean_c = sum_cand / n as f64;
    let mut cov = 0f64;
    let mut var_r = 0f64;
    let mut var_c = 0f64;
    for (r, c) in reference.iter().zip(candidate.iter()) {
        let (dr, dc) = (*r as f64 - mean_r, *c as f64 - mean_c);
        cov += dr * dc;
        var_r += dr * dr;
        var_c += dc * dc;
    }
    let corr = if var_r == 0.0 || var_c == 0.0 {
        0.0
    } else {
        cov / (var_r.sqrt() * var_c.sqrt())
    };
    Metrics {
        max_abs,
        rmse,
        sdr_db,
        corr,
        samples: n,
    }
}

/// Peak of the absolute value: a silent stem makes every metric meaningless.
fn peak(x: &[f32]) -> f64 {
    x.iter().fold(0f64, |acc, v| acc.max((*v as f64).abs()))
}

// ---------------------------------------------------------------------------
// Peak memory sampling
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
enum MemProbe {
    /// NVIDIA device memory in use, whole device (`nvidia-smi`).
    NvidiaSmi,
    /// Resident set size of this process (`ps -o rss=`), used on Apple Silicon
    /// where GPU buffers live in unified memory.
    ProcessRss,
}

struct MemSampler {
    stop: Arc<AtomicBool>,
    peak: Arc<AtomicU64>,
    baseline: u64,
    source: &'static str,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl MemSampler {
    fn start(probe: MemProbe) -> Option<Self> {
        let source = match probe {
            MemProbe::NvidiaSmi => "nvidia-smi memory.used (whole device, MiB)",
            MemProbe::ProcessRss => "ps -o rss (process RSS, MiB; unified memory proxy)",
        };
        let baseline = read_probe(probe)?;
        let stop = Arc::new(AtomicBool::new(false));
        let peak = Arc::new(AtomicU64::new(baseline));
        let stop_t = stop.clone();
        let peak_t = peak.clone();
        let handle = std::thread::spawn(move || {
            while !stop_t.load(Ordering::Relaxed) {
                if let Some(v) = read_probe(probe) {
                    peak_t.fetch_max(v, Ordering::Relaxed);
                }
                std::thread::sleep(Duration::from_millis(50));
            }
        });
        Some(Self {
            stop,
            peak,
            baseline,
            source,
            handle: Some(handle),
        })
    }

    /// `(peak_mib, baseline_mib, source)`.
    fn finish(mut self) -> (u64, u64, &'static str) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
        (self.peak.load(Ordering::Relaxed), self.baseline, self.source)
    }
}

fn read_probe(probe: MemProbe) -> Option<u64> {
    let mut cmd = match probe {
        MemProbe::NvidiaSmi => {
            let mut c = std::process::Command::new("nvidia-smi");
            c.args([
                "--query-gpu=memory.used",
                "--format=csv,noheader,nounits",
            ]);
            c
        }
        MemProbe::ProcessRss => {
            let mut c = std::process::Command::new("ps");
            c.args(["-o", "rss=", "-p", &std::process::id().to_string()]);
            c
        }
    };
    let out = cmd.output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let first = text.lines().next()?.trim().to_string();
    let value: u64 = first.parse().ok()?;
    Some(match probe {
        MemProbe::NvidiaSmi => value,
        // `ps` reports KiB on macOS.
        MemProbe::ProcessRss => value / 1024,
    })
}

// ---------------------------------------------------------------------------
// Checkpoint + config resolution
// ---------------------------------------------------------------------------

fn env_path(key: &str) -> Option<PathBuf> {
    std::env::var(key).ok().filter(|v| !v.is_empty()).map(PathBuf::from)
}

fn resolve_checkpoint() -> (PathBuf, PathBuf) {
    let checkpoint = env_path("CANDLE_ROFORMER_GPU_SMOKE_SAFETENSORS")
        .or_else(|| env_path("CANDLE_ROFORMER_BS_SAFETENSORS"))
        .unwrap_or_else(|| {
            panic!(
                "no real checkpoint for the GPU smoke test: set \
                 CANDLE_ROFORMER_GPU_SMOKE_SAFETENSORS (or CANDLE_ROFORMER_BS_SAFETENSORS) \
                 to a converted BS-RoFormer safetensors file. There is no fallback: without \
                 real weights the run would be vacuous, not a smoke test."
            )
        });
    let config = env_path("CANDLE_ROFORMER_GPU_SMOKE_CONFIG")
        .or_else(|| env_path("CANDLE_ROFORMER_BS_CONFIG"))
        .unwrap_or_else(|| {
            Path::new(env!("CARGO_MANIFEST_DIR")).join("configs/bs_roformer_viperx_1297.yaml")
        });
    for path in [&checkpoint, &config] {
        assert!(
            path.exists(),
            "{} does not exist: the GPU smoke test needs a real local checkpoint",
            path.display()
        );
    }
    (checkpoint, config)
}

/// Keep the fixture cheap: with the published profile the default chunk is
/// 800 hops (8 s at 44.1 kHz) and the CPU reference would dominate the run.
/// The override only changes overlap-add granularity, identically on both
/// backends, so the comparison stays valid.
fn set_short_chunk() {
    if std::env::var("CANDLE_ROFORMER_CHUNK_HOPS").is_err() {
        // SAFETY: single-threaded within this test process for the duration of
        // the run (`--test-threads=1` is implied by the per-backend filter);
        // nothing else in this binary reads the variable concurrently.
        std::env::set_var("CANDLE_ROFORMER_CHUNK_HOPS", "200");
    }
}

// ---------------------------------------------------------------------------
// The one run, shared by every backend
// ---------------------------------------------------------------------------

fn run_backend(choice: DeviceChoice, probe: MemProbe) {
    let (checkpoint, config) = resolve_checkpoint();
    set_short_chunk();

    // Fail loudly if the backend cannot be created: an unavailable backend is a
    // failed smoke run, never a skip.
    let _device = choice
        .resolve()
        .unwrap_or_else(|e| panic!("{choice:?} backend unavailable on this host: {e}"));

    let assets = LocalSeparationAssets::new(&checkpoint, &config);
    let bytes = std::fs::metadata(&checkpoint)
        .map(|m| m.len())
        .unwrap_or(0);
    println!(
        "[gpu-smoke] backend={choice:?} checkpoint={} bytes={} config={} profile_family_source={}",
        checkpoint.display(),
        bytes,
        config.display(),
        assets.config.display()
    );

    // CPU reference first, on the same weights and the same fixture.
    let cpu = Separator::load(&assets, DeviceChoice::Cpu).expect("CPU load");
    let sr = cpu.config().sample_rate as u32;
    let audio = fixture(sr);
    let frames = audio.planar.len() / audio.channels as usize;
    let chunk = {
        let hops = std::env::var("CANDLE_ROFORMER_CHUNK_HOPS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(800);
        cpu.config().stft_hop_length.max(1) * hops
    };
    println!(
        "[gpu-smoke] fixture=synthetic-stereo sr={sr} channels={} frames={frames} peak={:.4} chunk={chunk} family={:?} stems={}",
        audio.channels,
        peak(&audio.planar),
        cpu.config().family(),
        cpu.config().num_stems
    );

    let t_cpu = Instant::now();
    let cpu_stems = cpu.separate(&audio).expect("CPU separation");
    let cpu_seconds = t_cpu.elapsed().as_secs_f64();
    let reference: Vec<(String, Vec<f32>)> = cpu_stems
        .stems
        .iter()
        .map(|(name, buf)| (name.clone(), buf.planar.clone()))
        .collect();
    assert!(
        !reference.is_empty(),
        "the CPU reference produced no stems: nothing to compare, so the run is vacuous"
    );
    drop(cpu);

    // Backend run, with peak memory sampled for exactly this window.
    let sampler = MemSampler::start(probe);
    let t_load = Instant::now();
    let sep = Separator::load(&assets, choice).unwrap_or_else(|e| panic!("{choice:?} load: {e}"));
    let load_seconds = t_load.elapsed().as_secs_f64();
    let t_gpu = Instant::now();
    let gpu_stems = sep
        .separate(&audio)
        .unwrap_or_else(|e| panic!("{choice:?} separation: {e}"));
    let gpu_seconds = t_gpu.elapsed().as_secs_f64();
    let (peak_mib, baseline_mib, source) = match sampler {
        Some(s) => s.finish(),
        None => (0, 0, "unavailable on this host"),
    };
    println!(
        "[gpu-smoke] peak_device_mib={peak_mib} baseline_mib={baseline_mib} delta_mib={} source={source}",
        peak_mib.saturating_sub(baseline_mib)
    );

    // Compare every stem the model produces, not just the first one.
    let mut compared = 0usize;
    for (name, expected) in &reference {
        let actual = gpu_stems.get(name).unwrap_or_else(|| {
            panic!("backend run is missing stem {name:?}; got {:?}", gpu_stems.names())
        });
        assert_eq!(actual.planar.len(), expected.len(), "{name}: length");
        let candidate_peak = peak(&actual.planar);
        assert!(
            candidate_peak > 1e-6,
            "{name}: backend stem is silent (peak {candidate_peak:.3e}); a vacuous result is a failure"
        );
        let m = compare(expected, &actual.planar);
        println!(
            "[gpu-smoke] stem={name} samples={} max_abs={:.3e} rmse={:.3e} sdr_db={:.2} corr={:.7} ref_peak={:.4} cand_peak={:.4}",
            m.samples,
            m.max_abs,
            m.rmse,
            m.sdr_db,
            m.corr,
            peak(expected),
            candidate_peak
        );
        assert!(
            m.max_abs <= MAX_ABS_LIMIT,
            "{name}: max_abs {:.3e} exceeds {MAX_ABS_LIMIT:.0e}",
            m.max_abs
        );
        assert!(
            m.rmse <= RMSE_LIMIT,
            "{name}: rmse {:.3e} exceeds {RMSE_LIMIT:.0e}",
            m.rmse
        );
        assert!(
            m.sdr_db >= SDR_MIN_DB,
            "{name}: sdr {:.2} dB below the {SDR_MIN_DB:.0} dB floor",
            m.sdr_db
        );
        assert!(
            m.corr >= CORR_MIN,
            "{name}: corr {:.6} below {CORR_MIN}",
            m.corr
        );
        compared += 1;
    }
    assert!(compared > 0, "no stem was compared: a vacuous run is a failure");
    println!(
        "[gpu-smoke] timing cpu_seconds={cpu_seconds:.2} backend_load_seconds={load_seconds:.2} backend_seconds={gpu_seconds:.2} stems_compared={compared} result=PASS"
    );
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(feature = "cuda")]
#[test]
#[ignore = "needs a real CUDA host and a local checkpoint: set CANDLE_ROFORMER_GPU_SMOKE_SAFETENSORS"]
fn cuda_bs_roformer_matches_cpu() {
    run_backend(DeviceChoice::Cuda, MemProbe::NvidiaSmi);
}

#[cfg(feature = "metal")]
#[test]
#[ignore = "needs a real Apple Silicon host and a local checkpoint: set CANDLE_ROFORMER_GPU_SMOKE_SAFETENSORS"]
fn metal_bs_roformer_matches_cpu() {
    run_backend(DeviceChoice::Metal, MemProbe::ProcessRss);
}

/// Without a GPU feature this binary has nothing to assert: say so loudly
/// instead of silently passing as if a backend had been exercised.
#[cfg(not(any(feature = "cuda", feature = "metal")))]
#[test]
fn gpu_smoke_requires_a_backend_feature() {
    println!(
        "[gpu-smoke] no GPU backend compiled in (build with --features cuda or --features metal); \
         nothing was executed and no backend is claimed"
    );
}
