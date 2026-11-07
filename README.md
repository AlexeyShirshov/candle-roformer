# candle-roformer

Offline vocal/instrumental separation in Rust on top of [Candle](https://github.com/huggingface/candle).
No Python at runtime, no hidden model downloads.

## What it does

1. loads a local MelBandRoformer checkpoint (safetensors) and its YAML profile;
2. takes an input audio file from the command line;
3. separates it with the model (chunked inference with weighted overlap-add);
4. writes the results: `vocals.wav` and `instrumental.wav` (float32 WAV, input
   sample rate and channel layout, exact input length).

The instrumental is the input-waveform residual: `instrumental = input - vocals`.

## Input formats

Anything Symphonia decodes: **m4a/AAC**, mp3, flac, ogg/vorbis, wav.
Mono input is duplicated to stereo for the model and the separated vocal is
downmixed back to mono; stereo stays stereo.

## Model files

Two profiles are supported out of the box:

| Profile | Config | Weights |
|---|---|---|
| GaboxR67 `voc_fv7` | `configs/voc_fv7.yaml` | convert `voc_fv7.ckpt` with `tools/convert_voc_fv7.py` |
| KimberleyJSN / Kijai mirror | `configs/kijai_melband_384.yaml` | `MelBandRoformer_fp32.safetensors` as published |

The original `.ckpt` is a PyTorch pickle and is **not** read by the runtime.
`tools/convert_voc_fv7.py` performs the one-time offline conversion
(`torch.load(weights_only=True)`, pinned SHA256, strict v7 key/shape/dtype
validation) and emits safetensors plus a manifest.

Weights are never downloaded, bundled, cached or redistributed by this project.
You supply locally obtained, lawfully licensed weights.

## Build

```sh
cargo build --release
```

CUDA: `cargo build --release --features cuda` (NVIDIA + CUDA toolkit).
Metal: `cargo build --release --features metal` (macOS).

## Run

```sh
./target/release/candle-roformer separate \
  --model /path/to/MelBandRoformer_fp32.safetensors \
  --config configs/kijai_melband_384.yaml \
  --input "/path/to/song.m4a" \
  --output out/ \
  --device cpu
```

Options:

- `--device cpu|cuda|metal` — an unavailable device is an error, never a silent
  CPU fallback;
- `--overwrite` — allow replacing existing outputs (default: refuse);
- `--duration <seconds>` — process only the first N seconds (quick preview);
- outputs are written to staging files and renamed one by one after success;
  a failure removes this program's staging files and leaves your existing files
  untouched.

## Status and limits

- CPU inference works but is slow: this machine has no NVIDIA GPU, so a
  MelBandRoformer 384/6 pass runs at roughly half real time and far slower than
  an optimized BLAS/PyTorch build. `CANDLE_NUM_THREADS=8` uses more cores.
  CUDA/Metal execution has **not** been tested on this machine.
- Supported architecture family: MelBandRoformer (`voc_fv7` 256/12 and
  Kimberley 384/6 profiles). `audio-separator` also covers MDX-Net, VR/UVR,
  Demucs and MDX23C — those are not ported yet.
- Real-checkpoint numerical parity (G3) and hardware smoke tests (G2) are open
  gates; see `docs/gates-report.md`.
- Vocal transcription (Whisper) is not implemented yet.
- Weights licensing is not resolved by this repository; see the design spec
  `docs/superpowers/specs/2026-11-07-candle-roformer-design.md`.
