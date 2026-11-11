# Ensemble presets and operators

This crate combines several separation models into one stem set. Two waveform
operators and one preset are supported; both mirror `audio-separator` 0.48.0
(`audio_separator/separator/ensembler.py`, `Ensembler.ensemble`), which is the
reference named by issue #8.

## Operators

| Operator id | Semantics |
|---|---|
| `avg_wave` | Per-sample mean of the model waveforms. Weights are applied as `accumulator += waveform * weight` in the waveforms' `f32` dtype (the product and the addition are evaluated in `f64`, one round-to-`f32` per model, in application order, exactly like the reference's `float32` accumulator and `float64` weights), then divided by the weight sum. |
| `median_wave` | Per-sample median across the models, independently per channel. For an even model count this is the `f32` mean of the two middle values. Weights are ignored, as in the reference. |

Both operators zero-pad a shorter waveform **per channel**, in frames, to the
longest waveform's frame count: channel `c` contributes its own frames and then
zeros, exactly like the reference's `np.pad` over a channel-major
`[channels, frames]` array. No channel is shifted by another channel's padding.
Neither operator trims, normalizes or rescales the result. An empty waveform
list, a channel or sample-rate mismatch, and a weight vector of the wrong length
are typed errors (`EnsembleError`) instead of the reference's `None` return or
silent equal-weight fallback — a mis-specified ensemble fails loudly rather than
changing semantics quietly.

Operator ids this crate does not implement — including the reference's
spectral operators `avg_fft`, `median_fft`, `min_fft`, `max_fft`,
`uvr_max_spec`, `uvr_min_spec` and `ensemble_wav` — are rejected with
`EnsembleError::UnknownAlgorithm`. They are never mapped onto a different
computation.

## Preset `vocal_balanced`

| Field | Value |
|---|---|
| `id` | `vocal_balanced` |
| `name` | `Vocal Balanced` |
| `algorithm` | `avg_wave` |
| `weights` | `null` (equal weights: each member contributes `1/2`) |
| `stems` | `vocals`, `instrumental` |

Members, in application order (the order the weights and the padding apply in):

| # | Role | Checkpoint | Bytes | SHA-256 | Profile |
|---|---|---|---|---|---|
| 0 | `vocals` | `model_bs_roformer_ep_317_sdr_12.9755.ckpt` | 639,331,213 | `5b84f37e8d444c8cb30c79d77f613a41c05868ff9c9ac6c7049c00aefae115aa` | `configs/bs_roformer_viperx_1297.yaml` (BS-RoFormer, `dim=512`, `depth=12`, 64 frequency bands) |
| 1 | `vocals` | `voc_fv7.ckpt` | 489,571,079 | `77b9e4b54802d670c02daceb30c61ec825dc54ca6c29c34b03cdd5e9f78382b6` | `configs/voc_fv7.yaml` (MelBandRoformer, `dim=256`, `depth=12`, `num_bands=60`) |

The weights are equal, so the preset is the arithmetic mean of the two models'
vocals stems. `instrumental` is the input residual (`input - vocals`, pointwise
on the same planar buffer), so `vocals + instrumental` reconstructs the mixture
sample for sample. No other stem is produced.

### It is this repository's own preset

`audio-separator` 0.48.0 ships an ensemble preset that is also named
`vocal_balanced`, and it is **not** this one: upstream's preset uses `avg_fft`
(a spectral operator this crate does not implement) with a different model pair.
The shared id is a naming coincidence of intent ("a balanced vocals ensemble"),
not a claim of interchangeability. Treating one as the other would silently
change both the operator and the models.

## Running it from the command line

The crate's command-line surface is its examples. `ensemble_separate` runs a
documented preset end to end and writes exactly the preset's stem set:

```sh
cargo run --release --example ensemble_separate -- \
  --preset vocal_balanced \
  --input song.wav \
  --out out_dir \
  --member model_bs_roformer_ep_317_sdr_12.9755.safetensors:configs/bs_roformer_viperx_1297.yaml \
  --member voc_fv7.safetensors:configs/voc_fv7.yaml
```

* `--member` is repeatable and must be given once per preset model, in the
  preset's documented order, as `<checkpoint.safetensors>:<profile.yaml>`.
  Checkpoints are loaded from safetensors; convert the published `.ckpt` files
  with `tools/convert_bs_roformer.py` and `tools/convert_voc_fv7.py` first.
* Use paths without a drive-letter colon (relative paths) for `--member`: the
  argument is split on the first `:`, so `C:\...` would be read as the checkpoint
  name `C`.
* `--out` receives `vocals.wav` and `instrumental.wav` (float32 WAV, the input's
  sample rate and channel count) and nothing else; the program exits non-zero if
  the produced stem set differs from the preset's documented set.
* `--list-presets` prints the built-in preset ids and their composition.

Presets can also be supplied from YAML (`EnsemblePreset::from_yaml_str`) with
`id`, `name`, `algorithm`, optional `weights`, `models[]` (`checkpoint`,
`profile`, `role`) and `stems[]`; `EnsemblePreset::validate` rejects an empty
model list, duplicate or empty stems, a role outside the stem set, and weights
whose count or sum cannot be honoured.

## Measured behaviour

Numerical parity against `audio-separator` 0.48.0 and the end-to-end preset run
on a real CC0 song are measured, not asserted here: the frozen fixture, the two
captured member waveforms, the reference operator outputs and the per-stem ×
channel `max_abs` / `RMSE` / `SDR` / `corr` metrics live under
`.pdca/open-issues-r5/rv1/item-8-ensembles/` (`manifest.json`, `index.md`,
`metrics/parity.json`), which is workspace evidence and is not part of the
published crate. The parity test that consumes the frozen fixture is
`tests/ensemble.rs::ensemble_matches_audio_separator_reference`; it is
`#[ignore]`d because the checkpoints are not shipped, and it runs when
`CANDLE_ROFORMER_ENSEMBLE_FIXTURE` and `CANDLE_ROFORMER_ENSEMBLE_OUT` point at
the fixture and the candidate output directory.

CUDA and Metal are not tested for the ensemble path; see the note in the
project README.
