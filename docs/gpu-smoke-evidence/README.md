# G2 hardware smoke evidence (issue #11)

Real-hardware runs of `tests/gpu_smoke.rs`. Everything below is a measurement, not
an expectation: each log is the actual output of the command quoted next to it,
including the failures. Nothing here is a claim about a backend that did not run.

| File | What it is |
|---|---|
| `cuda-2026-11-12-host.txt` | Host facts for the original CUDA attempt (driver 610.88). |
| `cuda-2026-11-12-smoke.log` | Original CUDA smoke on driver 610.88. **Failed** with PTX-version mismatch; kept as historical evidence. |
| `cuda-2026-11-13-host.txt` | Host facts for the passing CUDA run (GPU, compute capability, updated driver, toolkit, OS/toolchain). |
| `cuda-2026-11-13-run.txt` | UTC start/exit record for the passing run. |
| `cuda-2026-11-13-smoke.log` | `cargo-cuda.bat test --release --offline --features cuda --test gpu_smoke -- --ignored --nocapture` on the same Windows host. **Passed**. |
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

## CUDA — verified on one Windows host, for BS-RoFormer

Host: Windows 11 x86_64, NVIDIA GeForce RTX 5070 Ti (compute capability 12.0,
16,303 MiB), driver 621.24 (CUDA UMD 13.5), CUDA Toolkit 13.5 (nvcc 13.5.37,
`CUDARC_CUDA_VERSION=13040` workaround), MSVC 2022, CMake 3.31.8. Host details
and the captured command/exit are in the files listed above.

Command (through `cargo-cuda.bat`, which supplies the CUDA/MSVC build environment):

```
cargo-cuda.bat test --release --offline --features cuda --test gpu_smoke -- --ignored --nocapture
```

Exit `0`, `1 passed; 0 failed`. On the real ViperX 1297 BS-RoFormer checkpoint
(639,109,056 B f32) and deterministic 2 s stereo fixture (88,200 frames, 88,200
sample chunk), CUDA matched the CPU reference:

```
[gpu-smoke] peak_device_mib=1475 baseline_mib=611 delta_mib=864 source=nvidia-smi memory.used (whole device, MiB)
[gpu-smoke] stem=vocals samples=176400 max_abs=1.371e-6 rmse=4.823e-8 sdr_db=118.66 corr=1.0000000 ref_peak=0.9000 cand_peak=0.9000
[gpu-smoke] stem=instrumental samples=176400 max_abs=8.941e-8 rmse=3.500e-9 sdr_db=162.21 corr=1.0000000 ref_peak=0.9000 cand_peak=0.9000
[gpu-smoke] timing cpu_seconds=18.56 backend_load_seconds=0.50 backend_seconds=10.96 stems_compared=2 result=PASS
```

The earlier attempt on driver 610.88 / CUDA UMD 13.3 failed with
`CUDA_ERROR_UNSUPPORTED_PTX_VERSION`: nvcc 13.5 emits PTX ISA 9.1, which that
driver could not JIT. The original failure remains in
`cuda-2026-11-12-smoke.log`; after installing driver 621.24 / CUDA UMD 13.5, the
same smoke passed. This confirms one real CUDA execution path, not every family
or workload: the claim is limited to this Windows host, BS-RoFormer, this fixture
and chunk size, f32. No throughput claim is made.

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
