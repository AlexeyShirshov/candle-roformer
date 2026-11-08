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
- Normal CLI path mandatorily validates against the versioned v7 schema (no `None`/optional disable):
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

Blocked:

- `BLOCKED — no NVIDIA GPU / no Metal host`: hardware smoke tests for `DeviceChoice::Cuda` and `DeviceChoice::Metal` on real CUDA/Metal backends were not run. On this Linux host `nvidia-smi` is unavailable and there is no macOS/Metal. `DeviceChoice::Cuda` and `DeviceChoice::Metal` deterministically return a typed `DeviceError::Unavailable` (checked by CPU unit tests) and never silently fall back to CPU. No `PASS` is claimed for CUDA or Metal execution.

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

Status of the transcription layer, reported separately from the separation
gates because it has its own blocker (no local Whisper weights).

Verified offline on synthetic fixtures:

- `cargo build --offline` — exit code 0 (with the pinned Candle
  `candle-transformers` at rev `cfcbd7186d13271843ce7eb41d25659bd0835850`).
- `cargo test --offline` — 43 tests, 0 failed:
  - 10 crate unit tests, including Whisper timestamp-token math (20 ms steps,
    window offsets, clamping, special-token filtering, decode-error
    propagation), mel filter shapes (80x201 and 128x201, finite/non-negative),
    zlib compression-ratio repetition detection, deterministic sampler, channel
    downmix;
  - 5 `tests/whisper_tests.rs` integration tests, including an end-to-end run
    of `Transcriber::load` + `transcribe` against a tiny randomly-initialised
    Whisper checkpoint with a hand-written tokenizer (proves loading, log-mel,
    encoder/decoder plumbing, timestamp rules, segment assembly and the
    `[0, duration]`/monotonic invariants execute), and a full
    `separate_and_transcribe_file` run that writes two WAVs plus `.txt`/`.json`
    and refuses to overwrite without the flag;
  - the existing 3 library-API, 13 pipeline, 11 DSP and 1 doc tests.
- Embedded mel filter assets: byte-for-byte copies from the pinned Candle
  checkout, sha256 recorded in `assets/whisper/README.md`.

Blocked:

- `BLOCKED — no local Whisper weights`: no `config.json` + `tokenizer.json` +
  `model.safetensors` checkpoint is present in this environment, and downloading
  or bundling weights is forbidden by the project rules. Therefore real
  transcription, language detection on real audio, and any accuracy/WER claim
  were **not** run. The synthetic test uses random weights and is a plumbing
  check only, not a quality result.
- `BLOCKED — no GPU/Metal host` (shared with G2): Whisper was exercised on CPU
  only. CUDA/Metal execution is untested here.
- Known risk (spec §9): Whisper on sung vocals may hallucinate or mis-transcribe
  lyrics; timestamps are segment-level, not word alignment.
