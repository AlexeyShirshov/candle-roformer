# candle-roformer

Offline vocal/instrumental separation in Rust on top of [Candle](https://github.com/huggingface/candle).
No Python at runtime, no hidden model downloads.

**The crate is the product.** It is the Rust/Candle port of the
[`audio-separator`](https://github.com/nomadkaraoke/python-audio-separator)
library API: load a local checkpoint once, separate in-memory audio buffers.
Command-line usage (output naming, timestamped sidecars, batches) lives in the
separate [`karaoke-cli`](https://github.com/AlexeyShirshov/karaoke-cli) project.

## Library API

```rust
use candle_roformer::{AudioBuffer, DeviceChoice, LocalSeparationAssets, Separator};

let assets = LocalSeparationAssets::new("vocals.safetensors", "vocals.yaml");
let separator = Separator::load(&assets, DeviceChoice::Cpu)?;

let audio = candle_roformer::wav::read_audio("song.m4a")?;   // or build AudioBuffer yourself
let stems = separator.separate(&audio)?;                     // in-memory, no file I/O

candle_roformer::wav::write_wav_f32("out/vocals.wav", &stems.vocals)?;
candle_roformer::wav::write_wav_f32("out/instrumental.wav", &stems.instrumental)?;
```

- `Separator::load` validates the checkpoint against the profile (full key set,
  shapes) before allocating the model, so a mismatched pair fails loudly.
- `Separator::separate` takes an `AudioBuffer` (planar f32 + sample rate +
  channels) and returns a [`Stems`] set at the input's sample rate, channel
  layout and exact length. The set always carries the named stems under
  `Stems::stems` (a `Vec<(String, AudioBuffer)>`) and keeps the two-stem
  convenience fields `Stems::vocals` / `Stems::instrumental`.
- **Residual vs. independent stems.** A single-stem profile (`num_stems == 1`,
  e.g. MelBandRoformer vocals) runs in *residual* mode: `vocals` is the model
  estimate and `instrumental` is the input residual. The pointwise identity
  `vocals + instrumental == input` holds exactly only in exact residual mode
  (level processing disabled: `normalization = 0.0`, `amplification = 0.0`,
  `invert_spect = false`); see "Level processing" below. A multi-stem profile
  (`num_stems > 1`, e.g. Demucs / MDX / VR) runs in *independent* mode:
  every stem is an independent model estimate and no stem is replaced by
  `input - stem`, so the sum need not equal the input. The mode follows from
  `num_stems`; it is not a runtime flag.
- A 4-stem profile returns four named `AudioBuffer`s through
  `Stems::stems` / `Stems::get`.

- **Level processing** (reference `audio-separator` defaults). Controlled by
  `InferenceConfig` (`normalization`, `amplification`, `invert_spect`):
  - `normalization` (default `0.9`): peak-normalize the input mix and every
    output stem to the threshold (`x *= threshold / peak(x)` for `peak > 0`;
    silent buffers are untouched, no `NaN`/`Inf`). Set to `0.0` to disable;
    negative or non-finite values are rejected by `InferenceConfig::validate`.
  - `amplification` (default `0.0` = off): scale a quiet input up to the
    threshold when `0 < peak < threshold`.
  - `invert_spect` (default `false`): in residual mode compute the secondary
    stem by spectral subtraction (STFT/iSTFT) instead of the waveform
    residual.
  - **Exact residual mode** (`normalization = 0.0`, `amplification = 0.0`,
    `invert_spect = false`): `instrumental = input - vocals`, so
    `vocals + instrumental == input` pointwise (within `f32` round-off).
  - **Normalized mode** (default `normalization = 0.9`): the input and each
    stem are peak-normalized independently, so the pointwise identity above no
    longer holds in the original input scale; with `invert_spect = true` the
    secondary stem is a spectral residual.
- `AudioBuffer::validate` rejects absurd rates, >2 channels, misaligned or
  non-finite buffers.
- Reading and writing files is limited to [`wav`]: `read_audio` (m4a/mp3/flac/
  ogg/wav) and `write_wav_f32`. Buffers are planar f32; the crate never invents
  file names and never writes sidecars.

### Vocal transcription (Whisper)

```rust
use candle_roformer::{DeviceChoice, LocalWhisperAssets, Transcriber};

let assets = LocalWhisperAssets::new("config.json", "tokenizer.json", "model.safetensors");
let mut transcriber = Transcriber::load(&assets, DeviceChoice::Cpu)?;
let transcript = transcriber.transcribe(&stems.vocals)?;   // vocals only, never the mix

println!("language: {:?}", transcript.language);
for segment in &transcript.segments {
    println!("{:.2}-{:.2}s: {}", segment.start_seconds, segment.end_seconds, segment.text);
}
```

`Transcriber::transcribe` downmixes a copy of the vocal stem to mono, resamples
it to 16 kHz, decodes in non-overlapping 30 s windows and returns
`Transcript { language, segments }` with `TranscriptSegment { start_seconds,
end_seconds, text }`. Timestamps are relative to the buffer passed in (the
separator returns stems on the input timeline). Language is auto-detected on
multilingual checkpoints; `TranscribeOptions::language` forces a code and
`transcribe_with` exposes the no-speech / log-prob / compression-ratio
thresholds. A `*.en` checkpoint rejects an explicit foreign language.

### Combining the two

The crate deliberately stops at in-memory buffers: it does not pick output
names, write `.txt`/`.json` sidecars or manage batches. Wire the layers together
in the order you need, e.g.

```rust
let stems = separator.separate(&audio)?;
let transcript = transcriber.transcribe(&stems.vocals)?;
candle_roformer::wav::write_wav_f32("out/vocals.wav", &stems.vocals)?;
```

The separator can be dropped before the Whisper model is loaded, so both model
sets are never resident at once.

## CLI

The command-line tool is a separate project:
[`karaoke-cli`](https://github.com/AlexeyShirshov/karaoke-cli) (batch input,
`<input>_(Vocals)_<model>.wav` naming, timestamped `.txt`/`.json` sidecars).
It depends on this crate by git revision.

## Input formats

m4a (AAC, ALAC, FLAC-in-MP4), mp3, flac, ogg/vorbis, wav. FLAC is decoded with
claxon because symphonia mis-decodes some FLAC-in-MP4 bitstreams.

Mono input is duplicated to stereo for the model and the separated vocal is
downmixed back to mono; stereo stays stereo.

## Model files

| Family | Status | Config file(s) | Checkpoint expectation |
|---|---|---|---|
| MelBandRoformer | implemented | `configs/voc_fv7.yaml` (dim 256 / depth 12), `configs/kijai_melband_384.yaml` (384 / 6) | `voc_fv7.ckpt` converted with `tools/convert_voc_fv7.py`; `MelBandRoformer_fp32.safetensors` as published |
| BS-RoFormer | implemented | `configs/bs_roformer_viperx_1297.yaml` (dim 512 / depth 12, 62 bands) | `model_bs_roformer_ep_317_sdr_12.9755.ckpt` converted with `tools/convert_bs_roformer.py` (pinned size/SHA-256) |
| Whisper | implemented (transcription) | `config.json` from the checkpoint | `model.safetensors` + `tokenizer.json` |
| MDX-Net (ONNX) | implemented | built-in profile `mdx::uvr_mdx_net_inst_hq_3()` | `UVR-MDX-NET-Inst_HQ_3.onnx` as published (SHA-256 checked before inference) |
| VR Arch / UVR (.pth) | implemented | built-in `4band_v2` profile (`VrProfile::four_band_v2()`, nn_arch 123812) | `2_HP-UVR.pth` converted offline with `tools/convert_vr_arch.py` (pinned SHA-256 and size) |
| Demucs, MDX23C | not implemented | - | - |

The family is selected from the profile: an explicit `family:` key (`mel_band_roformer` /
`bs_roformer`), or, when absent, the config shape — `freqs_per_bands` selects BS-RoFormer,
`num_bands` selects MelBandRoformer. `Separator::load` validates every tensor name and shape
against that family before allocating the model, so a wrong architecture, a partial download or a
stray tensor fails loudly instead of loading garbage.

BS-RoFormer profiles may carry their sample rate in the `audio:` block (as the published UVR
profiles do) and use the Python-only `!!python/tuple` tag for `freqs_per_bands`; both are accepted.
The rotary tables are part of the checkpoint and are loaded, not recomputed. A published BS-RoFormer
profile has no licence file alongside its weights; this repository asserts none.

Whisper assets must be in the Candle-compatible layout (`config.json`,
`tokenizer.json`, `model.safetensors`). f32 only; no GGUF/quantized path in v1.
The mel filter banks are numeric constants embedded at build time with
provenance in `assets/whisper/README.md`.

The original `.ckpt` is a PyTorch pickle and is **not** read by the runtime.
`tools/convert_voc_fv7.py` and `tools/convert_bs_roformer.py` perform the one-time offline
conversion (`torch.load(weights_only=True)`, pinned SHA256) and emit safetensors plus a manifest.
The BS-RoFormer converter checks the key set and every tensor shape against the profile schema and
then casts every tensor to float32; `tools/convert_voc_fv7.py` additionally checks dtypes.

The strict check is reproducible against a real checkpoint: `tests/bs_roformer.rs` carries an
`#[ignore]`d `real_checkpoint_passes_strict_validation` test that reads the converted file and its
profile from `CANDLE_ROFORMER_BS_SAFETENSORS` / `CANDLE_ROFORMER_BS_CONFIG`, reports the
missing / unexpected / shape-mismatch counts for the crate's own `expected_key_shapes` schema, and
then loads the model through `Separator::load`:

```sh
CANDLE_ROFORMER_BS_SAFETENSORS=/models/model_bs_roformer_ep_317_sdr_12.9755.safetensors \
CANDLE_ROFORMER_BS_CONFIG=/models/model_bs_roformer_ep_317_sdr_12.9755.yaml \
  cargo test --release --test bs_roformer -- --ignored --nocapture \
    real_checkpoint_passes_strict_validation
```

`bs_missing_key_is_rejected_by_the_loader` and `bs_extra_key_is_rejected_by_the_loader` cover the
negative path on a tiny checkpoint.

Weights are never downloaded, bundled, cached or redistributed by this project.
You supply locally obtained, lawfully licensed weights.

## Build

```sh
cargo build --release
```

CUDA: `cargo build --release --features cuda` (NVIDIA + CUDA toolkit).
Metal: `cargo build --release --features metal` (macOS).

Both feature builds compile. Metal execution is verified on the host listed under
"Status and limits" (one Apple Silicon host, BS-RoFormer, one fixture); CUDA and
every other GPU family remain untested — see `docs/gpu-smoke-evidence/`.

## Status and limits

- CPU inference works but is slow: on a machine without a GPU a
  MelBandRoformer 384/6 pass runs at roughly a quarter of real time.
  `CANDLE_NUM_THREADS=8` uses more cores.
- GPU (CUDA/Metal) execution:
  - **Metal: tested and passing on one Apple Silicon host, for BS-RoFormer only.**
    GitHub Actions `macos-15` (Apple M1 (Virtual), 3 vCPU, 7.5 GB, macOS 15),
    commit `2a22721`, run
    [43227797972](https://github.com/AlexeyShirshov/candle-roformer/actions/runs/43227797972):
    `--features metal`, real ViperX 1297 BS-RoFormer checkpoint, 2 s stereo
    fixture, 88,200-sample chunk. Metal vs CPU: vocals `max_abs 4.1e-6`, `RMSE
    7.8e-8`, `SDR 114.5 dB`, corr `1.0000000`; instrumental `max_abs 1.9e-7`,
    `RMSE 5.1e-9`, `SDR 159.0 dB`, corr `1.0000000`. Peak memory 2,577 MiB RSS
    (baseline 987, delta 1,590 MiB; unified-memory proxy, not a driver-reported
    device high-water mark); timing CPU 53.7 s vs Metal 41.6 s for the 2 s fixture.
    This is the *entire* Metal claim: one host, one family, this fixture, f32.
    MelBandRoformer, VR/UVR, MDX-Net and Whisper are **not tested** on Metal, and
    neither are larger chunks. Evidence: `docs/gpu-smoke-evidence/`.
  - **CUDA: not tested.** Windows, RTX 5070 Ti 16 GB (compute capability 12.0),
    driver 610.88 (CUDA UMD 13.3), CUDA Toolkit 13.5: the crate builds and the
    checkpoint loads onto the device, then the first kernel launch fails with
    `CUDA_ERROR_UNSUPPORTED_PTX_VERSION`. `candle-kernels 0.11.1` ships PTX only;
    nvcc 13.5 emits PTX ISA 9.1 while driver 610.88 accepts PTX ISA ≤ 9.0, so the
    kernels cannot be JIT-compiled. No CUDA parity and no CUDA peak-memory number
    exists; CUDA Toolkit ≤ 13.3 or a newer display driver is required.
  - `DeviceChoice::Cuda`/`DeviceChoice::Metal` still fail with a typed
    `DeviceError::Unavailable` instead of silently falling back to CPU.
- Reduced precision (fp16) is **rejected for every supported family** in this
  release. Evidence (`tests/fp16_decision.rs`, `docs/gpu-smoke-evidence/`):
  the forward path has no dtype-generic branch — f16 weights fail immediately with
  `dtype mismatch in mul, lhs: F32, rhs: F16` — and rounding the real intermediate
  tensors through f16 costs 81.0 dB SDR on the STFT coefficients and 70.3 dB SDR
  on the model's own mask, 20–30 dB worse than the f32 parity reported elsewhere in
  this file. STFT/ISTFT, the mask multiplication, overlap-add accumulation and
  RMSNorm therefore stay f32; a future fp16 path must keep exactly those in f32.

- Numerical parity with the PyTorch reference on a frozen 8 s chunk of a real
  song (Kijai 384/6): the model pass matches at correlation 0.9999999 and
  max abs error 3.5e-4, inside the provisional gate (max abs ≤ 1e-3 when
  reference RMS > 1e-3). The file-level adapter overlaps 8 s chunks with a
  4 s step, so a written stem is not bit-identical to a single-pass reference
  run. Measured with the original Candle git revision and re-checked against
  crates.io `candle-core 0.11.1`: both builds produce byte-identical stems.
  Hardware smoke tests (G2) remain open — see the CUDA/Metal entry above and
  `docs/gpu-smoke-evidence/`.
- Supported architecture families: MelBandRoformer (`voc_fv7` 256/12 and Kimberley 384/6),
  BS-RoFormer (`configs/bs_roformer_viperx_1297.yaml`, "Viperx 1297" 512/12, 62 bands), and
  MDX-Net (ONNX) via a caller-supplied checkpoint (`UVR-MDX-NET-Inst_HQ_3.onnx`, pinned SHA-256
  `317554b07fe1ea5279a77f2b1520a41ea4b93432560c4ffd08792c30fddf9adc`) on the CPU through the
  pure-Rust `tract-onnx` runtime. On a frozen 20 s stereo fixture the MDX-Net stems match
  `audio-separator` 0.48.0 at correlation 0.999999999999, max abs difference 1.4e-6 and RMSE 1.5e-7
  (SDR 118.9 dB). `audio-separator` also covers VR/UVR, Demucs and MDX23C - those are not ported
  yet. The MDX-Net checkpoint is user-supplied and is not downloaded, bundled or vendored; see
  `docs/dependency-audit.md` for the licence audit.
- VR Arch (UVR) support: the published `2_HP-UVR.pth` (126,782,699 B, SHA-256
  `26d6400d2d2e90811ba2381828c8c3519196097375af524c3d0edd969acf5356`) is converted offline to
  safetensors by `tools/convert_vr_arch.py` (pinned size and checksum, refusal paths covered by
  `tools/tests/test_convert_vr_arch.py`; the `.pth` is never read at runtime). Inference reproduces
  the `4band_v2` profile (band STFTs 960/512/320/640 with hops 480/160/80/80, `window_size=512`,
  `aggression=5`, no TTA / high-end / post-process) on the CPU in f32. On a frozen 12 s stereo
  fixture the stems match `audio-separator` 0.48.0 at max abs difference 1.4e-6, RMSE 3.6e-8 and
  SDR 112.8 dB (correlation 0.999999999998) for the primary stem. The synthesis resampler is
  `libsamplerate` (`sinc_fastest`), which is what the reference uses on non-ARM hosts; building the
  crate therefore needs CMake on the build machine. The checkpoint is user-supplied.
- BS-RoFormer parity: the published `model_bs_roformer_ep_317_sdr_12.9755.ckpt` (639,331,213 B,
  SHA-256 `5b84f37e…115aa`) passes strict validation (699 tensors, zero missing / unexpected /
  shape mismatches against the crate's own `expected_key_shapes` schema; see the opt-in test under
  `## Model files`) and separates a frozen 8 s fixture at correlation 0.99999999999996 and max abs
  error 4.8e-7 against audio-separator 0.48.0 + torch 2.15.0 (CPU, f32), with RMSE 3.8e-8 and SDR
  130.3 dB — inside the same gate as the MelBand claim above. That comparison is a single 8 s
  segment on both sides: the fixture equals one BS-RoFormer chunk (441 × 800 = 352800 samples), so
  each side runs one window and the overlap-add envelope cancels. Multi-window blending is not part
  of that measurement.
- Chunk windows are tail-aligned: the last window ends at the end of the input instead of being a
  short, zero-padded window. A partial tail window feeds the model a zero tail and contaminates the
  tail output with the wrong context; the in-memory and streaming paths share this schedule.
- Whisper transcription is implemented (80/128 mel bins, multilingual and
  English-only checkpoints, auto/explicit language, segment timestamps, 30 s
  windows) and was exercised with real `openai/whisper-tiny` and
  `openai/whisper-small` checkpoints (downloaded only for local verification;
  not part of this repository): the JFK sample transcribes exactly, and a 30 s
  window of the separated vocal of «В сигаретном дыму» yields the real lyric
  line «Я болею тобой». Singing remains hard for Whisper: short windows and
  some regions produce gibberish or hallucinated text. Segments with no audio
  support (timestamps past the end of the window) are dropped instead of being
  reported at the boundary. There is no WER/accuracy guarantee (spec §9).

  Try it with the example (no separator needed):

  ```sh
  cargo run --release --example transcribe -- song.wav /path/to/whisper-dir ru
  ```
- Weights licensing is not resolved by this repository; see the design spec
  [docs/superpowers/specs/2026-11-07-candle-roformer-design.md](https://github.com/AlexeyShirshov/candle-roformer/blob/main/docs/superpowers/specs/2026-11-07-candle-roformer-design.md).


## File-based separation (`separate_file` / `separate_streaming`)

`Separator::separate_file` / `separate_streaming` separate an audio file on disk
and write the resulting stems to WAV files. Two code paths are used:

- **Bounded streaming path** (input sample rate == model sample rate, residual
  profile, `invert_spect == false`): the input is decoded in `chunk_duration`-
  sized windows and processed with overlap-add (OLA), so peak RSS stays
  `O(chunk)` and does not grow with the input length. `chunk_duration` drives
  the OLA chunk size and therefore the separation granularity. Amplification
  and input normalization are computed from a streaming global peak pre-pass;
  output peaks are normalized in a second pass over the (disk-backed) raw stem
  buffers, so `normalization` still applies. The secondary stem is the waveform
  residual `input - vocals` (exact in exact residual mode).
- **In-memory fallback** (input sample rate != model sample rate, multi-stem
  profiles, or `invert_spect == true`): the whole input is decoded into memory
  and processed by `Separator::separate`, then written with `write_wav_f32`.
  Correct but not bounded-memory. `invert_spect` requires full-signal STFT and
  therefore uses this path.

Both paths share the same level-processing semantics (`normalization`,
`amplification`, `invert_spect`) and residual semantics (exact residual vs.
normalized mode); only memory behavior and `chunk_duration` relevance differ.

### Bounded memory (streaming path)

The bounded streaming path keeps peak RSS `O(chunk_duration)` and does not grow
with input length. Measured on the synthetic tiny model (CPU, `chunk_duration =
0.07256 s`, stereo, 44100 Hz, input read from a pre-generated file, each
duration in a separate process, `VmHWM` from `/proc/self/status`):

| input length | VmHWM (KiB) | VmHWM (MiB) |
|---|---|---|
| 10 s  | 17476 | 17.07 |
| 30 s  | 17420 | 17.01 |
| 60 s  | 18964 | 18.52 |
| 120 s | 18820 | 18.38 |

Relative growth 10 s → 120 s: **18820 / 17476 = 1.077** (≤ 1.15 threshold
from plan M1). The small non-monotonic spread is allocator/page noise, not an
input-length dependence.

The in-memory fallback (non-matching sample rate, multi-stem profiles, or
`invert_spect == true`) decodes the whole input and is **not** bounded-memory;
`invert_spect` requires a full-signal STFT and therefore uses that path.

**Scale caveat.** Only short-scale measurements (10–120 s, tiny model,
`chunk_duration = 0.07256 s`) are performed and claimed. Hour-scale runs
(e.g. 7200 s) are **not** performed and are **not** claimed; the
`O(chunk)` bound is an algorithmic statement about the streaming path, not a
measured result at hour scale. A real-weights series on long inputs is likewise
not measured here.

### Invariants

- Output length equals input length exactly (0-sample deviation) for every stem.
- In exact residual mode (level processing disabled) `vocals + instrumental ==
  input` with `max |...| <= 1e-5` (f32 WAV precision); in normalized mode the
  pointwise identity does not hold (see "Level processing").
- No NaN/Inf in outputs.

Tests: `cargo test --test streaming_chunked`.


## License and attribution

MIT, see
[`LICENSE`](https://github.com/AlexeyShirshov/candle-roformer/blob/main/LICENSE) and
[`NOTICE`](https://github.com/AlexeyShirshov/candle-roformer/blob/main/NOTICE).

The separation model is a Rust/Candle port of MelBandRoformer from
[Music-Source-Separation-Training](https://github.com/ZFTurbo/Music-Source-Separation-Training)
(MIT, Copyright 2024 Roman Solovyev), which builds on code by
[lucidrains](https://github.com/lucidrains) (rotary embeddings) and the
[Ultimate Vocal Remover](https://github.com/Anjok07/ultimatevocalremovergui)
project. The Whisper layer follows the
[Candle Whisper example](https://github.com/huggingface/candle) and OpenAI's
[Whisper](https://github.com/openai/whisper) reference (tokenizer order,
language table, timestamp rules); the mel filter banks are OpenAI's published
constants. Pinned revisions and checksums are recorded in the repository under
`docs/provenance/`.

Model weights are not part of this crate and keep their own licences.
