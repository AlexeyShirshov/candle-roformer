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

All direct dependencies are MIT/Apache-2.0 compatible. `candle-transformers` is intentionally not added in Step 1; it will be reconsidered when Whisper integration (Step 4) requires it. No weights or model assets are bundled or downloaded by the crate.

## Transitive dependencies

`cargo metadata` resolved **159** crates in the dependency graph (dependency metadata inventory, not compiled packages) (including the Candle git checkout at rev `cfcbd7186d13271843ce7eb41d25659bd0835850`). **155** of those crates carry a byte-genuine evidence file under `docs/dependency-audit-evidence/` (153 copies plus 2 byte-identical copies: `windows-link-0.2.1.txt` and `windows-sys-0.61.2.txt` are byte-identical copies of their source `license-mit`, sha256 `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383`). The remaining **4** crates have no licence file in their source tree and are recorded as `metadata_only` (licence expression taken from their own `Cargo.toml`/`cargo metadata` output, no quoted licence text): `candle-nn 0.11.1`, `pulp-wasm-simd-flag 0.1.1`, `r-efi 5.3.0`, `r-efi 6.0.0`. No crate in the graph is recorded as `UNKNOWN — evidence unavailable`; all 159 carry an explicit licence string from metadata.

Full list with licence strings and evidence class: `docs/dependency-audit-evidence/s1-licence-audit.txt` (generated from `cargo metadata`); metadata-only rows are marked `metadata_only` in the evidence-file column.
