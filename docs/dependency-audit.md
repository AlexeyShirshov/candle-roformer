# Dependency license audit (Step 1)

Crate: `candle-roformer` (lib target `candle_roformer`, bin `candle-roformer`).

Pinned Candle source: `https://github.com/huggingface/candle`, rev `cfcbd7186d13271843ce7eb41d25659bd0835850` (manifest version 0.11.1).

This repository's own license: `LICENSE` (MIT, copyright 2026 Alexey) — see spec §14.

## Direct dependencies

| Crate | Version | License (from fetched manifest) | Evidence file |
|---|---:|---|---|
| candle-core | 0.11.1 | MIT OR Apache-2.0 | `docs/dependency-audit-evidence/candle-core-0.11.1.txt` |
| candle-nn | 0.11.1 | MIT OR Apache-2.0 | `metadata_only` (no licence file in source tree) |
| rustfft | 6.4.1 | MIT OR Apache-2.0 | `docs/dependency-audit-evidence/rustfft-6.4.1.txt` |
| thiserror | 2.0.21 | MIT OR Apache-2.0 | `docs/dependency-audit-evidence/thiserror-2.0.21.txt` |
| serde | 1.x | MIT OR Apache-2.0 | config deserialization |
| serde_yaml | 0.9 | MIT OR Apache-2.0 | v7-style YAML profile loading |
| symphonia | 0.5.5 | MPL-2.0 | audio decoding (m4a/AAC, mp3, flac, ogg, wav) |

Note on `symphonia`: MPL-2.0 is file-level copyleft; it is used as an unmodified
dependency (no Symphonia source is copied into this repository), which keeps this
project's own MIT-licensed code unaffected. `symphonia` is the only decoder; no
`ffmpeg` or other external binary is invoked at build or run time.

All direct dependencies are MIT/Apache-2.0 compatible. `candle-transformers` is intentionally not added in Step 1; it will be reconsidered when Whisper integration (Step 4) requires it. No weights or model assets are bundled or downloaded by the crate.

## Transitive dependencies

`cargo metadata` resolved **186** crates in the dependency graph (dependency metadata inventory, not compiled packages) (including the Candle git checkout at rev `cfcbd7186d13271843ce7eb41d25659bd0835850`). For the Step-1 graph (**159** crates) **155** carry a byte-genuine evidence file under `docs/dependency-audit-evidence/` (153 copies plus 2 byte-identical copies: `windows-link-0.2.1.txt` and `windows-sys-0.61.2.txt` are byte-identical copies of their source `license-mit`, sha256 `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383`); **4** are `metadata_only` (licence expression from `cargo metadata`, no quoted licence text): `candle-nn 0.11.1`, `pulp-wasm-simd-flag 0.1.1`, `r-efi 5.3.0`, `r-efi 6.0.0`.

The Step-2 additions (**27** crates) are inventoried here by licence expression from `cargo metadata`; byte-genuine evidence files for them have **not** been regenerated yet:
- `symphonia` 0.5.5 and its 15 sub-crates — **MPL-2.0** (unmodified dependency; no Symphonia source is copied into this repository, so the project's MIT code is unaffected);
- `serde` 1.0.229 / `serde_derive` 1.0.229 / `serde_yaml` 0.9.34 / `unsafe-libyaml` 0.2.11 / `indexmap` / `hashbrown` / `equivalent` / `itoa` / `ryu` / `memchr` / `lazy_static` — MIT OR Apache-2.0 (or MIT / Apache-2.0 / Unlicense equivalents).

No crate in the graph is recorded as `UNKNOWN — evidence unavailable`; all 186 carry an explicit licence string from metadata.

Full list with licence strings and evidence class: `docs/dependency-audit-evidence/s1-licence-audit.txt` (generated from `cargo metadata`); metadata-only rows are marked `metadata_only` in the evidence-file column.
