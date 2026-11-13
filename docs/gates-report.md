# Gates report — candle-roformer (Step 1)

Evidence root: `.pdca/candle-roformer-impl/rv4/` (full logs under `attempts/`).

## Gate definitions (verbatim from spec §12)

- **G1** (spec §12:114): verify the local checksum + safe-format / compatibility of keys, forms and defaults with v7, freeze the reference call and the manifest; on incompatibility — stop/report, do not invent layers. (No numeric thresholds.)
- **G2** (spec §12:116): isolated mandatory smoke tests of operations on CUDA/Metal for each declared supported backend; portable CPU unit tests of DSP parity; do not promise tested platforms without real hardware. (No numeric thresholds; no SDR.)
- **G3** (spec §12:118): Python vs Rust tensors and waveform fixtures (trusted reference, offline dev only), pinned package versions/seed/eval/defaults, f32 attention baseline, comparison. Target (unmeasured) thresholds on a limited deterministic fixture: RMS error ≤ 1e-4 and maxabs ≤ 1e-3 when ref RMS > 1e-3, otherwise relL2 <= 1e-3; silence — absolute only (max_abs <= 1e-3). These are architect acceptance targets, not observed results; on failure investigate/report/re-approve thresholds, do not silently expand. Reference float32 path without AMP by default. Production target is the requested GPU without fallback.

## G1 — converter / checkpoint verification

Offline-checked (synthetic fixtures, no real checkpoint):

- Converter refuses a missing checkpoint path:
  - Command: `python3 tools/convert_voc_fv7.py /tmp/opencode/nope.ckpt`
  - Exit code: `1`
  - Log: `.pdca/candle-roformer-impl/rv1/attempts/s1-converter-refusal.log`
  - Message: `ERROR: checkpoint not found: ...`
- Converter refuses an explicit-path omission:
  - Command: `python3 tools/convert_voc_fv7.py`
  - Exit code: `1`
  - Message: `ERROR: explicit local checkpoint path is required`
- The normal model-loading path mandatorily validates against the versioned v7 schema (no `None`/optional disable):
  - Negative control: `.pdca/candle-roformer-impl/rv1/attempts/s1-neg-control-schema.log`
  - Exit code: `1`
  - Message: `ERROR: state_dict key mismatch: missing=[], unexpected=['totally_unexpected_key']`
- Synthetic state-dict refusal suite (keys/shapes/dtypes/buffers/element-count/writer dtype):
  - Command: `python3 -m unittest discover -s tools/tests -v`
  - Exit code: `0`
  - Log: `.pdca/candle-roformer-impl/rv4/attempts/s1-pytest-discover-r4.log`
  - Result: `65 tests, 65 passed, 0 failed, 0 skipped, 0 errors` (direct runs: `tools/tests/test_convert_voc_fv7.py` = 50 tests; `tools/tests/test_docs_evidence.py` = 15 tests)
- Restricted deserialization uses `torch.load(..., weights_only=True, map_location='cpu')` only; unsupported restricted mode fails and is never retried with `weights_only=False` (source contains no `weights_only=False`).
- Safetensors writer always compares `product(shape)` with the value count (no single-dimension shortcut) and emits F32 only; non-F32 source dtypes are refused with a typed error. Manifest records `written_dtype: F32` per parameter and `dtype_conversion`.
- Manifest never derives "expected" from what it found: it states `validated_schema` (v7 schema version) and computes matched/missing/unexpected against that schema's expected set.

Blocked:

- `BLOCKED — missing local checkpoint voc_fv7.ckpt`: the real checkpoint `voc_fv7.ckpt` (expected SHA256 `77b9e4b54802d670c02daceb30c61ec825dc54ca6c29c34b03cdd5e9f78382b6`, size `489571079`) is not present in this environment, so real conversion, key/shape validation against the trained checkpoint, and manifest emission for the real file were not run. No `PASS` is claimed for G1 on the real checkpoint.

## G2 — CPU DSP / primitive smoke and hardware

CPU results:

- Crate build (offline):
  - Command: `export PATH="$HOME/.cargo/bin:$PATH"; cargo build --offline`
  - Exit code: `0`
  - Log: `.pdca/candle-roformer-impl/rv4/attempts/s1-cargo-build.log`
- CPU unit tests (11 tests):
  - Command: `export PATH="$HOME/.cargo/bin:$PATH"; cargo test --offline`
  - Exit code: `0`
  - Log: `.pdca/candle-roformer-impl/rv4/attempts/s1-cargo-test.log`
  - Result: `11 passed; 0 failed; 0 ignored` (STFT/ISTFT round-trip within f32 tolerance, 60 mel bands covering all 1025 bins, all 60 bands non-empty, gather/scatter_add parity, `DeviceChoice` CPU resolution and CUDA/Metal refusal, short-input defined behaviour at boundaries 1/2 and reconstruction at 2047/2048/2049).
- DSP short-input fix: `src/dsp.rs` uses signed arithmetic in `reflect_pad` (no unsigned underflow) and `stft` refuses inputs with length <= pad (1024) with a typed `DspError::InvalidParams` instead of silently substituting padding.
- Dependency license audit: `docs/dependency-audit.md` (191 third-party crates per `Cargo.lock`, per-crate evidence under `docs/dependency-audit-evidence/`).

Hardware (real-backend runs of `tests/gpu_smoke.rs`, 2026-11-12 and 2026-11-13):

- Harness: `tests/gpu_smoke.rs` holds one opt-in (`#[ignore]`d) test per backend,
  compiled only under `--features cuda` / `--features metal`. Each run loads a real
  BS-RoFormer safetensors checkpoint (published UVR "Viperx 1297": `.ckpt`
  639,331,213 B, converted to 639,109,056 B f32), separates a deterministic 2 s
  stereo fixture on the CPU and on the backend, compares every stem
  (`max_abs`, `RMSE`, `SDR`, correlation; floors 1e-3 / 1e-4 / 60 dB / 0.999) and
  reports peak device memory. A missing checkpoint, an unavailable backend, a
  silent stem or zero compared stems fails the run — it never skips green.
- `CUDA — VERIFIED (one host, one family, this fixture)` (Windows 11, RTX 5070 Ti,
  compute capability 12.0, driver 621.24 / CUDA UMD 13.5, CUDA Toolkit 13.5.37):
  - Command: `cargo-cuda.bat test --release --offline --features cuda --test gpu_smoke -- --ignored --nocapture`
    → exit `0`, `1 passed; 0 failed`. The wrapper configures the MSVC/CUDA build
    environment; exact invocation and exit are in
    `docs/gpu-smoke-evidence/cuda-2026-11-13-run.txt`.
  - Evidence: `docs/gpu-smoke-evidence/cuda-2026-11-13-smoke.log` and
    `docs/gpu-smoke-evidence/cuda-2026-11-13-host.txt`.
  - Result, BS-RoFormer ViperX 1297 (`639,109,056 B` f32), 2 s stereo fixture,
    88,200-sample chunk, CPU reference vs CUDA:
    | stem | samples | max_abs | RMSE | SDR | corr | peak |
    |---|---:|---:|---:|---:|---:|---:|
    | vocals | 176,400 | 1.371e-6 | 4.823e-8 | 118.66 dB | 1.0000000 | 0.9000 / 0.9000 |
    | instrumental | 176,400 | 8.941e-8 | 3.500e-9 | 162.21 dB | 1.0000000 | 0.9000 / 0.9000 |
  - Peak memory: `peak_device_mib=1475`, baseline `611`, delta `864 MiB`, measured
    with `nvidia-smi` for the whole device. Timing: CPU reference 18.56 s, CUDA
    model load 0.50 s, CUDA separation 10.96 s for the 2 s fixture.
  - The earlier 2026-11-12 run on driver 610.88 / CUDA UMD 13.3 failed at the
    first kernel launch with `CUDA_ERROR_UNSUPPORTED_PTX_VERSION`; its original
    log remains preserved as `cuda-2026-11-12-smoke.log`. The 621.24 driver update
    resolved that PTX mismatch. Scope: one Windows host, BS-RoFormer, this fixture
    and chunk size, f32; other model families and larger chunks are not verified.
- `Metal — VERIFIED (one host, one family, this fixture)` (GitHub Actions
  `macos-15`: Apple M1 (Virtual), 3 vCPU, 7.5 GB RAM, rustc 1.99.1
  aarch64-apple-darwin):
  - Command: `cargo build --release --locked --features metal` → exit `0`, then
    `cargo test --release --locked --features metal --test gpu_smoke -- --ignored --nocapture`
    → exit `0`, `1 passed; 0 failed`, run
    [43227797972](https://github.com/AlexeyShirshov/candle-roformer/actions/runs/43227797972)
    (commit `2a22721`).
  - Evidence: `docs/gpu-smoke-evidence/metal-2026-11-12-macos-15.log` (artifact
    `metal-smoke-macos-15`), host facts in
    `docs/gpu-smoke-evidence/metal-2026-11-12-runner.txt`.
  - Result, BS-RoFormer ViperX 1297 (`639,109,056 B` f32), 2 s stereo fixture,
    88,200-sample chunk, CPU reference vs Metal:
    | stem | samples | max_abs | RMSE | SDR | corr | peak |
    |---|---:|---|---|---|---|---|
    | vocals | 176,400 | 4.053e-6 | 7.773e-8 | 114.51 dB | 1.0000000 | 0.9000 / 0.9000 |
    | instrumental | 176,400 | 1.937e-7 | 5.069e-9 | 158.99 dB | 1.0000000 | 0.9000 / 0.9000 |
  - Peak memory (Metal): `peak_device_mib=2577`, baseline `987`, delta
    `1590 MiB`, measured as process RSS — on Apple Silicon the GPU buffers live in
    unified memory, so RSS is the available proxy; it is not a driver-reported
    device-allocation high-water mark.
  - Timing on that runner: CPU reference 53.72 s, Metal model load 2.15 s, Metal
    separation 41.57 s for the 2 s fixture (3 vCPU virtual M1; a throughput figure,
    not a speed claim).
  - Scope of the claim: one Apple Silicon host, the BS-RoFormer family, this
    fixture and chunk size, f32. It says nothing about MelBandRoformer, VR/UVR,
    MDX-Net or Whisper on Metal, about larger chunks, or about fp16.
  - Two prior runs of the same gate failed and are kept as evidence:
    `metal-2026-11-12-macos-15-before-fix.log` (runs
    [43219881846](https://github.com/AlexeyShirshov/candle-roformer/actions/runs/43219881846),
    [43222506483](https://github.com/AlexeyShirshov/candle-roformer/actions/runs/43222506483))
    failed with `MetalKernelError::MatMulNonContiguous`, because candle's Metal GEMM
    only accepts contiguous operands and the model fed it narrow/squeeze views. The
    fixes (commits `183e3d8`, `dec5f97`, `2a22721`) materialise those views with
    `.contiguous()`; numerically identical, CPU suite unchanged.
- CI coverage (`gpu-smoke` workflow, issue #11): `no-GPU sanity` is green on
  `windows-latest` and `ubuntu-latest` (build + full CPU suite, no GPU assertions).
  `macos-14` was dropped from the matrix: GitHub answered
  `The configuration 'macos-14-arm64-us-default' is not supported`.

The issue #11 hardware-smoke `Done when` is now met: one real CUDA host and one
Apple Silicon host passed, with both logs committed as evidence. This is a narrow
smoke-test result, not a general CUDA/Metal support claim. fp16 remains rejected;
the decision and numerical evidence are recorded below.

Reduced precision (fp16) — **decision: rejected for every supported family**:

- Evidence: `tests/fp16_decision.rs` (opt-in), log
  `docs/gpu-smoke-evidence/fp16-2026-11-12.log`, run on the same real ViperX 1297
  BS-RoFormer checkpoint and the same 2 s fixture.
- Structural: the forward path has no dtype-generic branch. Building the network
  from an f16 `VarBuilder` succeeds and the first forward fails with
  `dtype mismatch in mul, lhs: F32, rhs: F16`; the STFT front end and the
  overlap-add stage are host-side f32 by construction, so f16 would only touch the
  matmul weights.
- Numerical: rounding the real intermediate tensors through f16 (Candle's own
  conversion) costs `SDR 81.03 dB` on the STFT coefficients and `SDR 70.27 dB` on
  the model's own mask (real `bs_03_mask` dump, `1x1x2050x201x2`) after the mask
  multiplication and resynthesis. Both are 20–30 dB worse than the f32 parity this
  crate reports for its CPU paths (MDX-Net 118.9 dB, VR/UVR 112.8 dB).
- Sensitive ops that must stay f32 in any future fp16 path: STFT/ISTFT, the complex
  mask multiplication, overlap-add accumulation and RMSNorm. (All are f32 today.)

## Factory verification (G1-derived, N=3, r=4)

- Upstream factory: `get_model_from_config` from `utils/settings.py` (rev `f0b527736e7da24eae5b561a2bb1efd0569ec757`, repo `Music-Source-Separation-Training`), invoked with model type `mel_band_roformer` and `docs/provenance/v7.yaml` (rev `7860d31d8cc1c73b46c1cbdc2b497210c97478ae`).
- Instantiation performed **without loading weights** `voc_fv7.ckpt` (no weights downloaded/bundled/cached). CPU-only dev environment under `/tmp/opencode` (isolated venv; not in repo).
- Result: model instantiated successfully; `state_dict()` has **828 keys**. Full key/shape table recorded in `docs/provenance/state_dict_keys.json` (sha256 `098415873e10f0420f4e3065c9c4e8149691b416b463bc19abc9bd6c01836510`).
- Comparison to `V7_SCHEMA` (`tools/convert_voc_fv7.py`): 828/828 keys, 0 key mismatches, 0 shape mismatches, 0 dtype mismatches; exact match.
- Buffer truth: 8 named buffers are registered, all with `persistent=False` (upstream `register_buffer(..., persistent=False)` at `mel_band_roformer.py` lines 490-491, 496-497 and `rotary_embedding_torch.py` lines 128, 137). PyTorch `state_dict()` excludes non-persistent buffers, so `state_dict()` contains **0 buffers**; all 828 entries are parameters. `V7_SCHEMA["buffers"]` is `[]`, consistent. The earlier "NAMED BUFFERS (persistent): 8" wording was incorrect; the correct statement is "8 registered named buffers, all non-persistent; state_dict contains 0 buffers".
- Provenance for the factory inputs is recorded in `docs/provenance/PROVENANCE.json` (includes `attend.py`, imported by `mel_band_roformer.py`): `mel_band_roformer.py`, `settings.py`, `v7.yaml`, `attend.py` classified `verified_factory`; `rotary_embedding_torch.py` classified `authored`/`unverified_offline`; `model_utils_load.py` remains `unverified_offline`.
- The tensor-name/shape correspondence (`PyTorch key` ↔ `Candle key` ↔ `shape`) in this report's derived schema matches the recorded `state_dict()` (828 keys, rotary path `layers.{i}.{0|1}.layers.{d}.0.rotary_embed.freqs`, 24 entries shape `(32,)`).
- Full reproducible run: `.pdca/candle-roformer-impl/rv4/attempts/s1-factory-verify.log` and `.md` (command, exit status 0, results).

## Notes

- No model weights were downloaded, bundled, cached, or committed.
- No `git push` was performed. Step 1 is not committed pending independent CHECK.
- Red state for the F05 underflow was not captured (the fix was applied before a failing run could be recorded); the boundary tests now pass with the fix and would fail without it because the pre-fix code panics on unsigned underflow in debug builds.

## G1 addendum — derived v7 schema (attempt 3, expert review)

Schema status: **derived, corroborated, unproven against the checkpoint**. The real proof is the fail-closed G1 run on the actual `voc_fv7.ckpt`.

- PROFILE parity: `time_transformer_depth=1`, `freq_transformer_depth=1`, `mlp_expansion_factor=4`, `linear_transformer_depth=0`, `norm_output=true`, `skip_connection=false`, `zero_dc=true` (plus existing fields). Provenance: HF `v7.yaml` rev `7860d31d8cc1c73b46c1cbdc2b497210c97478ae` (sha256 `f51b06e470a8f4e73e87fe97ff85fd2471b1034683c5a9088ab33d41a751ce2f`) and MSST `mel_band_roformer.py` rev `f0b527736e7da24eae5b561a2bb1efd0569ec757` (sha256 `a28fce62020faad6461176660e012cf5799ae8a26c76fe4b2bc636f5b44b9fc5`).
- Schema generator: 828 parameters, 0 buffers (all eight `register_buffer` calls are `persistent=False`). Per-parameter shapes derived from mel counts (60 bands).
- Rotary duplication rationale: one `rotary_embed.freqs` per Attention, path `layers.{i}.{0|1}.layers.{d}.0.rotary_embed.freqs` (Attention is element 0 of each `ModuleList`); 24 entries, shape (32,); all identical in shape (bitwise on a real file). The abbreviated `layers.0.rotary_embed` form is not used. `dim_freqs_in` is unused upstream and is not used for per-band dims.
- Upstream construct citations: `RMSNorm.gamma` (64), `FeedForward` linears (83/86), `to_qkv` no-bias (118), `to_gates` (120), `to_out` no-bias (123), band-split `nn.Linear(dim_in, dim)` (279), `MaskEstimator` class (320) with `MLP` linears (310), final norm when `norm_output`, one rotary `freqs` per attention.
- f32-size corroboration: 489,234,960 derived vs 489,571,079 actual, ratio 1.0007.
- Depth discrepancy recorded as a **profile-parity omission in the spec field list**, not a criterion change: the spec enumerated `depth=12` but omitted `time_transformer_depth`/`freq_transformer_depth` (v7.yaml sets both to 1). Resolved by adding them to PROFILE/`SeparatorConfig`.
- Residual uncertainties: key set unproven against the checkpoint; training-time upstream code version unconfirmed; yaml↔ckpt co-location is metadata-only; rotary count is a PyTorch `state_dict`-semantics prediction; per-band shapes rest on mel counts computed in our own code (not librosa); the ~336 KB size remainder is attributed to zip/pickle overhead.
- No partial/range read of the weights was performed. yaml↔ckpt co-location confirmed only via HF metadata/HEAD/tree API (size/oid), never by reading weight bytes.
- Flag for Step 2: `candle-nn` has no `strict` loading, so the Rust loader must compare the full key set against the manifest explicitly.

## G3 — numerical parity (real checkpoint)

- `BLOCKED — missing local checkpoint voc_fv7.ckpt`. Not run; no partial/range read performed. G3 target thresholds per spec §12:118 (provisional / unmeasured until real weights): when ref RMS > 1e-3: RMS error ≤ 1e-4 and max_abs ≤ 1e-3; when 0 < ref RMS ≤ 1e-3: relL2 ≤ 1e-3; silence (ref RMS == 0): absolute only, max_abs ≤ 1e-3. Provisional / unmeasured before real weights; no PASS claimed. (G3 silence threshold: max_abs <= 1e-3.)

## Whisper (ASR) — Step 4

Status: **functional validation on real weights done; no accuracy/WER guarantee**
(especially for singing). Reported separately from the separation gates.

### Models and fixtures (local validation only, not committed)

Downloaded on 2026-11-08 into `/tmp/opencode/` for this validation; they are not
bundled, cached in the repository, or redistributed, and the crate never
downloads weights itself.

| Asset | Size | SHA256 |
|---|---:|---|
| `openai/whisper-tiny` `model.safetensors` | 151,061,672 | `7ebd0e69e78190ffe1438491fa05cc1f5c1aa3a4c4db3bc1723adbb551ea2395` |
| `openai/whisper-tiny` `config.json` | 1,983 | `ffdccec4f3211f4c63310f2b7098f309fe70f3952cedc5e4d11e43f5b2379b98` |
| `openai/whisper-tiny` `tokenizer.json` | 2,480,466 | `27fc476bfe7f17299480be2273fc0608e4d5a99aba2ab5dec5374b4482d1a566` |
| `openai/whisper-small` `model.safetensors` | 966,995,080 | `1d7734884874f1a1513ed9aa760a4f8e97aaa02fd6d93a3a85d27b2ae9ca596b` |
| `openai/whisper-small` `config.json` | 1,967 | `e6a2b489da1b5aed65a8eb8d1e7466fa867ad5643a8bc138ba708bd56b2875c4` |
| `samples_jfk.wav` (Candle demo dataset) | 352,078 | `59dfb9a4acb36fe2a2affc14bacbee2920ff435cb13cc314a08c13f66ba7860e` |

### Speech check — JFK sample (known reference text)

Command (example built from this crate):
`./target/release/examples/transcribe /tmp/opencode/jfk/samples_jfk.wav <model-dir>`

- tiny, auto language: `en`, `[0.00 -> 10.50] And so my fellow Americans ask not
  what your country can do for you ask what you can do for your country.`
  — wall 2.3 s, peak RSS 323 MB.
- small, auto language: `en`, `[0.00 -> 11.00] And so my fellow Americans, ask
  not what your country can do for you, ask what you can do for your country.`
  — wall 16.1 s, peak RSS 1.90 GB.

Both match the reference sentence; this is a correctness check of mel
computation, encoder/decoder plumbing, timestamp rules and segment assembly.

### Real vocal check — «В сигаретном дыму» (cover)

Input: the user's m4a, separated by this crate with the Kijai MelBandRoformer
checkpoint. Whisper consumed the **vocal stem only**, never the mix.

- 8 s window at 75 s, small, forced `ru`: `0.0–7.0 Умирал оттаскирив.`
  — a garbled mid-phrase result; wall 2:27 including separation, peak RSS
  1.97 GB. This is the expected failure mode for a short, mid-phrase window.
- 30 s window at 75 s, small, forced `ru`:
  `[20.00–25.00] Я болею тобой` — verified against the published lyrics of the
  song; wall 8:39 including separation, peak RSS 2.05 GB.
  The same run initially also produced
  `[30.00–30.00] Редактор субтитров Н.Новикова` / `Корректор А.Кулакова`, the
  well-known Whisper hallucination for padded silence (the window is exactly
  30 s). Those segments had timestamps past the audio and are now dropped by the
  no-audio-support rule; re-running the saved stem leaves only the real line.
- Language auto-detection on this sung vocal is unreliable (tiny chose `ja`,
  small chose `lt`); forcing `ru` is required for a usable result.

### Bug found in the Candle reference (fixed here)

The pinned Candle whisper example computes
`penultimate_was_timestamp = len >= 2 && tokens[len-2] >= timestamp_begin`,
i.e. `false` for the first generated token. OpenAI's reference uses
`len(seq) < 2 or ...`, i.e. `true` there. With the example's version the first
timestamp suppresses all text tokens, so greedy decoding degenerates to
`[<|0.00|>, <|21.00|>, <|eot|>]` and the text only appeared through
temperature fallback, with hallucinated timestamps. Fixed to match OpenAI;
JFK now decodes the full sentence at temperature 0. Two other deliberate
differences from the example: timestamp seconds use
`(token - timestamp_begin) / 50` (the example adds 0.04 s), and no-speech
probability is read from the logits after the full prompt.

### Automated tests

`cargo test --offline` (crate) — 41 tests, 0 failed: 11 crate unit tests
(timestamp math, assembly, clamping and no-audio-support dropping, mel filter
shapes, zlib ratio, sampler, downmix), 3 Whisper integration tests (including an
end-to-end run on a tiny random checkpoint), 3 library-API, 12 core
(model/resample/OLA/WAV) and 11 DSP plus 1 doc test.

The file-level flow (output naming, `.txt`/`.json` sidecars, `--overwrite`
preflight, trim window, full separate-then-transcribe run) was part of this
crate's suite before the split; it now lives in the `karaoke-cli` repository,
which depends on the crate by git revision and re-runs the same synthetic
end-to-end test there.

### Still blocked / not claimed

- `BLOCKED — CUDA/Metal`: Whisper ran on CPU only (no NVIDIA GPU, no Metal
  host). No PASS is claimed for GPU execution.
- No WER, no lyric-accuracy or language-coverage claim. Whisper on singing can
  hallucinate or mis-transcribe; timestamps are segment-level, not word
  alignment. Segment times in a trimmed run are relative to the written stems,
  with the window offset recorded as `start_offset_seconds` in the JSON sidecar.
