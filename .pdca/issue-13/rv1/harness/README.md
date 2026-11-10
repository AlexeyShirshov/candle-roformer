# RSS harness — bounded-memory measurement (issue #13, defect 13-D4)

Measures peak RSS (`VmHWM` from `/proc/self/status`) of `Separator::separate_file` as a
function of input duration at a fixed `chunk_duration`, with level processing enabled
(default `normalization = 0.9`). The criterion: peak RSS must stay roughly constant while
the input grows (the recorded baseline used a 10→120 s series, ratio ≤ 1.15).

## Files

| File | Purpose |
|---|---|
| `gen_tiny_model.rs` | Writes `tiny.yaml` + `tiny.safetensors` (a tiny MelBand config, no real weights). |
| `rss_measure.rs` | The harness. `MODE=gen` generates one fixture; `MODE=measure` runs one separation and prints `MEASURE_RESULT ... HWM_KIB=...`. Each mode must run in its own process. |
| `rss_measure_example.rs` | Superseded single-process variant. It generates the fixture inside the measured process, so RSS grows with the fixture and the result is an artefact (see `../rss/ATTRIBUTION.md`). Kept as the counter-example. |

## Reproducing

`gen_tiny_model.rs` is a `main`; `rss_measure.rs` is a `#[test]`, so they are copied into the
crate only for the run and removed afterwards:

```sh
mkdir -p /tmp/rss/tiny
cp .pdca/issue-13/rv1/harness/gen_tiny_model.rs examples/gen_tiny_model.rs
cargo run --release --example gen_tiny_model -- /tmp/rss/tiny

cp .pdca/issue-13/rv1/harness/rss_measure.rs tests/rss_measure.rs
# 1) generate each fixture in its own process
for d in 10 30 60 120; do
  MODE=gen DUR=$d FIXTURE=/tmp/rss/in_${d}s.wav \
    cargo test --release --test rss_measure -- --nocapture
done
# 2) measure each fixture in its own process
for d in 10 30 60 120; do
  MODE=measure DUR=$d CD=0.07256 FIXTURE=/tmp/rss/in_${d}s.wav \
    MODEL=/tmp/rss/tiny/tiny.safetensors \
    cargo test --release --test rss_measure -- --nocapture
done
```

Take `VmHWM` per run; never measure two durations in one process.

## Baseline (2026-11-10, CPU, release)

| input | VmHWM |
|---|---|
| 10 s | 17.07 MiB |
| 30 s | 17.01 MiB |
| 60 s | 18.52 MiB |
| 120 s | 18.38 MiB |

`ratio(10→120 s) = 1.077`. Raw logs and attribution: `../rss/`.

## Caveats

- Tiny synthetic model, not the real MelBand weights.
- Measured up to 120 s only; the 2 h scale is **not** measured.
- `separate_file` falls back to reading the whole input when the sample rate differs, for
  multi-stem models and for `invert_spect`; that path is O(T) memory and documented as
  non-bounded.

To re-check the long scale with real weights, rerun the series above with a longer fixture
and `MODEL` pointing at the real checkpoint.
