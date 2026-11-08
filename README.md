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
  channels) and returns `Stems { vocals, instrumental }` at the input's sample
  rate, channel layout and exact length. The instrumental is the
  input-waveform residual: `input - vocals`.
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
- Numerical parity with the PyTorch reference on a frozen 8 s chunk of a real
  song (Kijai 384/6): the model pass matches at correlation 0.9999999 and
  max abs error 3.5e-4, inside the provisional gate (max abs ≤ 1e-3 when
  reference RMS > 1e-3). The file-level adapter overlaps 8 s chunks with a
  4 s step, so a written stem is not bit-identical to a single-pass reference
  run. Measured with the original Candle git revision and re-checked against
  crates.io `candle-core 0.11.1`: both builds produce byte-identical stems.
  Hardware smoke tests (G2) remain open.
- Supported architecture family: MelBandRoformer (`voc_fv7` 256/12 and
  Kimberley 384/6). `audio-separator` also covers MDX-Net, VR/UVR, Demucs and
  MDX23C — those are not ported yet.
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
