# candle-roformer

Offline vocal/instrumental separation in Rust on top of [Candle](https://github.com/huggingface/candle).
No Python at runtime, no hidden model downloads.

**The crate is the product.** The `candle-roformer` binary is a thin CLI wrapper
over the same public API.

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
  channels) and returns `Stems { vocals, instrumental }` at the input's sample
  rate, channel layout and exact length. The instrumental is the
  input-waveform residual: `input - vocals`.
- `AudioBuffer::validate` rejects absurd rates, >2 channels, misaligned or
  non-finite buffers.
- File orchestration (decode, trim window, staging + rename) is the separate
  `pipeline` module; programs that already hold audio in memory never touch it.

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

### File flow: stems + text + JSON

```rust
use candle_roformer::pipeline::{separate_and_transcribe_file, TranscriptionRequest};
use candle_roformer::{DeviceChoice, LocalSeparationAssets, LocalWhisperAssets};

let mut request = TranscriptionRequest::new(
    LocalSeparationAssets::new("vocals.safetensors", "vocals.yaml"),
    LocalWhisperAssets::new("config.json", "tokenizer.json", "model.safetensors"),
    "song.m4a",
    "out/",
);
request.device = DeviceChoice::Cpu;
let out = separate_and_transcribe_file(&request)?;
```

This writes `<input>_(Vocals)_<model>.wav`, `<input>_(Instrumental)_<model>.wav`,
`<input>_(Vocals)_<model>.txt` (timestamped lines) and `.json` (language,
duration, segments). All paths are preflighted before any work starts; without
`overwrite` existing outputs are refused. Each file goes through a staging name
and a rename, but this is not a multi-file atomic transaction. The separator is
dropped before Whisper loads, so both model sets are never resident at once.

## CLI (thin wrapper)

```sh
./target/release/candle-roformer "/path/to/song.m4a" \
  --model_filename /path/to/MelBandRoformer_fp32.safetensors \
  --config configs/kijai_melband_384.yaml \
  --output_dir out/
```

Outputs follow the `audio-separator` naming convention:
`<input>_(Vocals)_<model>.wav` and `<input>_(Instrumental)_<model>.wav`.

Options: `--model_filename`/`--model`, `--model_file_dir`, `--config`
(defaults to `<model-stem>.yaml` next to the model), `--output_dir`,
`--output_format wav` (other formats are not implemented yet), `--single_stem`,
`--device cpu|cuda|metal` (an unavailable device is an error, never a silent
CPU fallback), `--start`, `--duration`, `--overwrite`.

Transcription is opt-in and never downloads anything:

```sh
./target/release/candle-roformer "song.m4a" \
  --model_filename /models/MelBandRoformer_fp32.safetensors \
  --config configs/kijai_melband_384.yaml --output_dir out/ \
  --transcribe --whisper_model_dir /models/whisper-small-ft \
  --language auto
```

`--whisper_model_dir` must contain `config.json`, `tokenizer.json` and
`model.safetensors` (override individually with `--whisper_config`,
`--whisper_tokenizer`, `--whisper_model`).

## Input formats

m4a (AAC, ALAC, FLAC-in-MP4), mp3, flac, ogg/vorbis, wav. FLAC is decoded with
claxon because symphonia mis-decodes some FLAC-in-MP4 bitstreams.

Mono input is duplicated to stereo for the model and the separated vocal is
downmixed back to mono; stereo stays stereo.

## Model files

| Profile | Config | Weights |
|---|---|---|
| GaboxR67 `voc_fv7` | `configs/voc_fv7.yaml` | convert `voc_fv7.ckpt` with `tools/convert_voc_fv7.py` |
| KimberleyJSN / Kijai mirror | `configs/kijai_melband_384.yaml` | `MelBandRoformer_fp32.safetensors` as published |
| Whisper (tiny..large, `*.en` or multilingual) | `config.json` from the checkpoint | `model.safetensors` + `tokenizer.json` |

Whisper assets must be in the Candle-compatible layout (`config.json`,
`tokenizer.json`, `model.safetensors`). f32 only; no GGUF/quantized path in v1.
The mel filter banks are numeric constants embedded at build time with
provenance in `assets/whisper/README.md`.

The original `.ckpt` is a PyTorch pickle and is **not** read by the runtime.
`tools/convert_voc_fv7.py` performs the one-time offline conversion
(`torch.load(weights_only=True)`, pinned SHA256, strict key/shape/dtype
validation) and emits safetensors plus a manifest.

Weights are never downloaded, bundled, cached or redistributed by this project.
You supply locally obtained, lawfully licensed weights.

## Build

```sh
cargo build --release
```

CUDA: `cargo build --release --features cuda` (NVIDIA + CUDA toolkit).
Metal: `cargo build --release --features metal` (macOS).

## Status and limits

- CPU inference works but is slow: on a machine without a GPU a
  MelBandRoformer 384/6 pass runs at roughly a quarter of real time.
  `CANDLE_NUM_THREADS=8` uses more cores. CUDA/Metal execution has **not**
  been tested here (no NVIDIA GPU, no macOS host).
- Numerical parity with the PyTorch reference is verified end-to-end
  (corr 1.000000 on a frozen 8 s input); hardware smoke tests (G2) remain open.
- Supported architecture family: MelBandRoformer (`voc_fv7` 256/12 and
  Kimberley 384/6). `audio-separator` also covers MDX-Net, VR/UVR, Demucs and
  MDX23C — those are not ported yet.
- Whisper transcription is implemented (80/128 mel bins, multilingual and
  English-only checkpoints, auto/explicit language, segment timestamps, 30 s
  windows) but has **not** been run with real Whisper weights on this machine:
  none are available locally. Loading, mel computation, decoding, timestamp
  rules and sidecar writing are exercised by an end-to-end test with a tiny
  randomly-initialised model; that is a plumbing test, not an accuracy claim.
  Hallucinations and wrong lyrics on sung vocals remain an explicit risk
  (spec §9).
- Weights licensing is not resolved by this repository; see the design spec
  `docs/superpowers/specs/2026-11-07-candle-roformer-design.md`.
