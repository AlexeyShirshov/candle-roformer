# Whisper mel filter bank assets

These are the log-mel filter banks used by OpenAI Whisper, stored as raw
little-endian `f32` in row-major `[num_mel_bins, 1 + N_FFT / 2]` order
(`N_FFT = 400`, so `201` bins per row).

| File | Shape | Bytes | SHA256 |
|---|---|---:|---|
| `melfilters.bytes` | 80 x 201 | 64320 | `85818f156f7e189453901a515e4726d270d307f976e161cf9403e8caab405498` |
| `melfilters128.bytes` | 128 x 201 | 102912 | `2a5f9822897750e047c85dea37cc268d3be0ecfd23c28a5f10da129d99d05afe` |

Provenance: copied byte-for-byte from the pinned Candle checkout
`huggingface/candle` @ `cfcbd7186d13271843ce7eb41d25659bd0835850`
(`candle-examples/examples/whisper/melfilters.bytes` and `melfilters128.bytes`).
Those files are in turn the filter banks from OpenAI Whisper's
`whisper/assets/mel_filters.npz`.

Candle is dual-licensed MIT OR Apache-2.0; OpenAI Whisper is MIT. The files are
numeric constants (no code) and are shipped unmodified. They are not model
weights: no trained parameters are bundled anywhere in this repository.

The runtime does not generate filters in Python and does not download anything;
the bytes are embedded at compile time and parsed as little-endian `f32`.
