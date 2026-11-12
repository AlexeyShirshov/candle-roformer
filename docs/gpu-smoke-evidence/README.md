# G2 hardware smoke evidence (issue #11)

Real-hardware runs of `tests/gpu_smoke.rs`. Everything below is a measurement, not
an expectation: each log is the actual output of the command quoted next to it,
including the failures. Nothing here is a claim about a backend that did not run.

| File | What it is |
|---|---|
| `cuda-2026-11-12-host.txt` | Host facts for the CUDA attempt (GPU, compute capability, driver, toolkit, OS/toolchain). |
| `cuda-2026-11-12-smoke.log` | `cargo test --release --features cuda --test gpu_smoke -- --ignored --nocapture` on that host. **Failed** (see below). |
| `metal-2026-11-12-runner.txt` | Host facts for the Metal run (GitHub Actions `macos-15`, Apple M1 virtual). |
| `metal-2026-11-12-macos-15.log` | CI artifact `metal-smoke-macos-15` of run [43227797972](https://github.com/AlexeyShirshov/candle-roformer/actions/runs/43227797972): same command with `--features metal` on `macos-15`. **Passed** (see below). |
| `metal-2026-11-12-macos-15-before-fix.log` | The same job on the two earlier commits: **failed** with `MetalKernelError::MatMulNonContiguous` before the operand-materialisation fixes. Kept so the fix is auditable. |
| `fp16-2026-11-12.log` | `cargo test --release --test fp16_decision -- --ignored --nocapture`: the fp16 decision evidence (structural + numerical). **Passed** (it is a measurement, not a performance claim). |

The smoke test always runs the same script: load a real BS-RoFormer
`safetensors` checkpoint (published UVR "Viperx 1297", 639,331,213 B `.ckpt`,
converted with `tools/convert_bs_roformer.py` to 639,109,056 B f32), separate a
deterministic 2 s stereo fixture on the CPU and on the backend, then compare
`max_abs` / `RMSE` / `SDR` / correlation per stem and report peak device memory.
Zero compared stems or a silent stem fails the run.

## CUDA — not verified (toolkit/driver version skew)

Host: Windows 11 x86_64, NVIDIA GeForce RTX 5070 Ti (compute capability 12.0,
16,303 MiB), driver 610.88 (CUDA UMD 13.3), CUDA Toolkit 13.5 (nvcc 13.5.37,
`CUDARC_CUDA_VERSION=13040` workaround), MSVC 2022, CMake 3.31.8.

Build with `--features cuda` succeeds (exit 0). The run itself reaches the device:
the checkpoint is loaded and the model is materialised on the GPU, then the first
kernel launch fails with

```
DriverError(CUDA_ERROR_UNSUPPORTED_PTX_VERSION, "the provided PTX was compiled with an unsupported toolchain.")
```

Cause: `candle-kernels 0.11.1` ships **PTX only** (`cudaforge` `build_ptx`,
i.e. `nvcc --ptx -gencode arch=compute_120,code=compute_120`). nvcc 13.5 emits
PTX ISA 9.1, while driver 610.88 (CUDA 13.3 UMD) accepts PTX ISA ≤ 9.0, so
`cuModuleLoadData` cannot JIT the modules. Consequence: **no CUDA parity, no CUDA
peak-memory number, and no CUDA support claim.** This is an environment
requirement, not a crate defect; either an installation of CUDA Toolkit ≤ 13.3 or
a newer display driver resolves it, and both need administrator rights that the
executing agent did not have.

Additional facts worth keeping: `nvcc` needs `vcvars64.bat`; it fails when
`TMP`/`TEMP` contain non-ASCII characters (Cyrillic user profile); it cannot open
sources under the non-ASCII default `%USERPROFILE%\.cargo`, so the build used an
ASCII `CARGO_HOME`; CUDA 13.5's CCCL headers reject MSVC's traditional
preprocessor, so `NVCC_PREPEND_FLAGS=-Xcompiler /Zc:preprocessor` is required;
`cudarc 0.19.10` panics on an unknown toolkit version, hence
`CUDARC_CUDA_VERSION=13040`.

## Metal — verified on one Apple Silicon host, for BS-RoFormer

Runner: GitHub Actions `macos-15`, `Apple M1 (Virtual)`, 3 vCPU, 7,516,192,768 B
RAM, macOS 15, rustc 1.99.1 `aarch64-apple-darwin`. Metal is present in the VM
even though `system_profiler SPDisplaysDataType` reports no display device: candle
creates the Metal device and the whole smoke run completes.

```
[gpu-smoke] backend=Metal … bytes=639109056 … family=BSRoformer stems=1
[gpu-smoke] fixture=synthetic-stereo sr=44100 channels=2 frames=88200 chunk=88200
[gpu-smoke] peak_device_mib=2577 baseline_mib=987 delta_mib=1590 source=ps -o rss (process RSS, MiB; unified memory proxy)
[gpu-smoke] stem=vocals       samples=176400 max_abs=4.053e-6 rmse=7.773e-8 sdr_db=114.51 corr=1.0000000
[gpu-smoke] stem=instrumental samples=176400 max_abs=1.937e-7 rmse=5.069e-9 sdr_db=158.99 corr=1.0000000
[gpu-smoke] timing cpu_seconds=53.72 backend_load_seconds=2.15 backend_seconds=41.57 stems_compared=2 result=PASS
```

Peak memory is process RSS, not a driver allocation counter: on Apple Silicon the
GPU buffers are unified memory, so RSS is the proxy the crate can observe without
reaching into the Metal device API. Both stems share the same input peak (0.9)
after the separator's peak normalization.

Scope: one host, BS-RoFormer (ViperX 1297), this 2 s fixture and 88,200-sample
chunk, f32. Nothing here says anything about MelBandRoformer, VR/UVR, MDX-Net or
Whisper on Metal, about other chunk sizes, or about fp16.

The two earlier attempts failed with

```
Metal error Invalid matmul arguments [6380544, 31744, 1] [0, 1, 512] (201, 2048, 512)
```

which is `candle-metal-kernels`'s `MetalKernelError::MatMulNonContiguous`: candle's
Metal GEMM only accepts contiguous operands, and the model fed it views —
`x.narrow(2, i, 1)?.squeeze(2)` of `(b, t, bands, dim)` for every mask-estimator
band, plus `k.transpose`/`q.narrow` in attention. Making those operands contiguous
(commits `183e3d8`, `dec5f97`, `2a22721`) is a copy, not a numerical change: the
CPU suite stayed green and the CPU reference is unchanged, and the passing run
above is 114–159 dB SDR against that reference.
