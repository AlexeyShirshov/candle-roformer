# Dependency license audit (Step 1)

Crate: `candle-roformer` (lib target `candle_roformer`, bin `candle-roformer`).

Pinned Candle source: `https://github.com/huggingface/candle`, rev `cfcbd7186d13271843ce7eb41d25659bd0835850` (manifest version 0.11.1).

This repository's own license: `LICENSE` (MIT, copyright 2026 Alexey) — see spec §14.

## Direct dependencies

| Crate | Version | License (from fetched manifest) | Evidence file |
|---|---:|---|---|
| candle-core | 0.11.1 | MIT OR Apache-2.0 | `docs/dependency-audit-evidence/candle-core-0.11.1.txt` |
| candle-nn | 0.11.1 | MIT OR Apache-2.0 | `metadata_only` (no licence file in source tree) |
| samplerate | 0.2.4 | BSD-2-Clause | `metadata_only` (crate manifest; wraps libsamplerate) |
| libsamplerate-sys | 0.1.12 | BSD-2-Clause | `metadata_only` (crate manifest; builds vendored libsamplerate C sources with CMake) |
| candle-transformers | 0.11.1 | MIT OR Apache-2.0 | pinned Candle checkout; Whisper encoder/decoder/log-mel |
| tokenizers | 0.23.2 | Apache-2.0 | Whisper `tokenizer.json` loading and text decoding |
| serde_json | 1.0.154 | MIT OR Apache-2.0 | Whisper `config.json` and the transcript JSON sidecar |
| flate2 | 1.1.11 | MIT OR Apache-2.0 | zlib compression ratio for Whisper temperature fallback |
| rustfft | 6.4.1 | MIT OR Apache-2.0 | `docs/dependency-audit-evidence/rustfft-6.4.1.txt` |
| thiserror | 2.0.21 | MIT OR Apache-2.0 | `docs/dependency-audit-evidence/thiserror-2.0.21.txt` |
| serde | 1.x | MIT OR Apache-2.0 | config deserialization |
| serde_yaml | 0.9 | MIT OR Apache-2.0 | v7-style YAML profile loading |
| symphonia | 0.6.1 | MPL-2.0 | demuxing + AAC/MP3/ALAC/Vorbis/PCM decoding |
| claxon | 0.4.3 | Apache-2.0 | FLAC frame decoding (also FLAC-in-MP4, which symphonia mis-decodes) |
| tract-onnx | 0.23.10 | MIT OR Apache-2.0 | MDX-Net ONNX graph execution, CPU only (`src/mdx.rs`); evidence `docs/dependency-audit-evidence/tract-onnx-0.23.10.txt` |
| sha2 | 0.10.9 | MIT OR Apache-2.0 | SHA-256 verification of the user-supplied MDX-Net `.onnx` before inference; evidence `docs/dependency-audit-evidence/sha2-0.10.9.txt` |

### MDX-Net (issue #2) additions

`tract-onnx` 0.23.10 (`license = "MIT OR Apache-2.0"` in its published
`Cargo.toml`, with `LICENSE-APACHE` and `LICENSE-MIT` in the crate) is the
pure-Rust ONNX runtime used for MDX-Net inference. It is CPU-only here: the
graph is optimised and executed in-process and is never dispatched to CUDA,
Metal or an external ONNX Runtime/Python process. `sha2` 0.10.9 is likewise
`MIT OR Apache-2.0` (RustCrypto) and is used only to hash the checkpoint before
inference. Both licence texts are copied from the published crates under
`docs/dependency-audit-evidence/`.

The MDX-Net checkpoint itself is **not** vendored, bundled, downloaded or
cached by this repository. `crate::mdx::MdxProfile::resolve_verified_asset`
takes a caller-supplied model directory, resolves the published
`UVR-MDX-NET-Inst_HQ_3.onnx` (66,759,214 bytes) and refuses to load it unless
its SHA-256 is exactly
`317554b07fe1ea5279a77f2b1520a41ea4b93432560c4ffd08792c30fddf9adc`
(the same digest as `audio-separator` 0.48.0's `model-checksums.json`). The
checkpoint keeps its own upstream licence; the repository only supplies the
loader and the pinned profile. Because the model file is user-supplied, the
`tract` dependency graph is the only new code in the audited set; its crates
carry MIT/Apache-2.0-compatible licences from their published manifests, and
the Step-1/Step-2 graph counts in this document predate this addition.

#### Resolved dependency closure of `tract-onnx` and `sha2`

Every dependency added or used for MDX-Net ONNX support and for the checkpoint
checksum is listed below by name, resolved version and licence expression. The
two direct dependencies are `tract-onnx` 0.23.10 (ONNX graph execution, CPU
only) and `sha2` 0.10.9 (SHA-256 of the caller-supplied `.onnx`); the table is
their complete transitive closure exactly as the current `Cargo.lock` resolves
it — all nine `tract-*` crates included.

Generated from authoritative tooling, not from memory: `cargo metadata
--format-version 1` supplied each package's `license` field (SPDX expression as
the crate publishes it) and the `resolve` graph, which was walked from the two
roots; `Cargo.lock` was the version cross-check for every row; `cargo tree`
supplied the host build graph so that a row the lock file resolves for another
target or for an optional feature cargo does not enable here is marked
`resolved only` instead of being presented as compiled. Result: **149 crates** —
118 in the host build graph, 31 resolved only, 0 with an empty licence field
(nothing is guessed; a crate reporting no licence would be recorded as such).

The lock file resolves feature-independently, so the `resolved only` rows are
part of the audited dependency set but are not compiled into this build. Across
all 149 rows there are 15 distinct licence expressions and every one of them
offers a permissive option (MIT, Apache-2.0, BSD-2-Clause, Zlib, Unlicense,
BlueOak-1.0.0 or Unicode-3.0); the only copyleft mention is
`LGPL-2.1-or-later` as one alternative of `r-efi`'s `MIT OR Apache-2.0 OR
LGPL-2.1-or-later` expression, and `r-efi` is resolved only here. No
GPL-only, AGPL or SSPL crate appears.

<!-- dependency-inventory:begin (generated; do not hand-edit the rows) -->
| Crate | Version | Licence (SPDX, from `cargo metadata`) | Role / why |
|---|---:|---|---|
| `allocator-api2` | 0.2.21 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `anyhow` | 1.0.104 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `anymap3` | 1.1.0 | BlueOak-1.0.0 OR MIT OR Apache-2.0 | MDX inference (transitive) |
| `autocfg` | 1.5.1 | Apache-2.0 OR MIT | MDX inference (transitive) |
| `bit-set` | 0.11.1 | Apache-2.0 OR MIT | MDX inference (transitive) |
| `bit-vec` | 0.10.1 | Apache-2.0 OR MIT | MDX inference (transitive) |
| `bitflags` | 2.13.2 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `block-buffer` | 0.10.4 | MIT OR Apache-2.0 | checksum (transitive) |
| `borsh` | 1.8.1 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `borsh-derive` | 1.8.1 | Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `bumpalo` | 3.20.3 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `bytemuck` | 1.25.2 | Zlib OR Apache-2.0 OR MIT | MDX inference (transitive) |
| `bytemuck_derive` | 1.12.1 | Zlib OR Apache-2.0 OR MIT | MDX inference (transitive) |
| `byteorder` | 1.5.0 | Unlicense OR MIT | MDX inference (transitive) |
| `bytes` | 1.12.1 | MIT | MDX inference (transitive) |
| `cc` | 1.11.1 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `cfg-if` | 1.0.5 | MIT OR Apache-2.0 | shared MDX inference + checksum (transitive) |
| `cfg_aliases` | 0.2.2 | MIT | MDX inference (transitive) - resolved only, not in the host build graph |
| `chacha20` | 0.10.2 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `cpufeatures` | 0.2.17 | MIT OR Apache-2.0 | checksum (transitive) |
| `cpufeatures` | 0.3.1 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `crossbeam-deque` | 0.8.8 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `crossbeam-epoch` | 0.9.21 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `crossbeam-utils` | 0.8.23 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `crunchy` | 0.2.4 | MIT | MDX inference (transitive) - resolved only, not in the host build graph |
| `crypto-common` | 0.1.7 | MIT OR Apache-2.0 | checksum (transitive) |
| `derive-new` | 0.7.0 | MIT | MDX inference (transitive) |
| `digest` | 0.10.7 | MIT OR Apache-2.0 | checksum (transitive) |
| `downcast-rs` | 2.0.2 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `dyn-clone` | 1.0.20 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `dyn-hash` | 1.0.0 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `either` | 1.19.0 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `equivalent` | 1.0.3 | Apache-2.0 OR MIT | MDX inference (transitive) |
| `erased-serde` | 0.4.11 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `errno` | 0.3.14 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `filetime` | 0.2.29 | MIT/Apache-2.0 | MDX inference (transitive) |
| `find-msvc-tools` | 0.1.15 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `float-ord` | 0.3.2 | MIT / Apache-2.0 | MDX inference (transitive) |
| `foldhash` | 0.2.0 | Zlib | MDX inference (transitive) |
| `generic-array` | 0.14.7 | MIT | checksum (transitive) |
| `getrandom` | 0.3.4 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `getrandom` | 0.4.3 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `half` | 2.7.1 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `hashbrown` | 0.16.1 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `hashbrown` | 0.17.1 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `indexmap` | 2.14.2 | Apache-2.0 OR MIT | MDX inference (transitive) - resolved only, not in the host build graph |
| `inventory` | 0.3.25 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `itertools` | 0.14.0 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `itertools` | 0.15.0 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `itoa` | 1.0.18 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `js-sys` | 0.3.107 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `lazy_static` | 1.5.1 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `libc` | 0.2.191 | MIT OR Apache-2.0 | shared MDX inference + checksum (transitive) - resolved only, not in the host build graph |
| `libm` | 0.2.16 | MIT | MDX inference (transitive) |
| `linux-raw-sys` | 0.12.1 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | MDX inference (transitive) - resolved only, not in the host build graph |
| `lock_api` | 0.4.14 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `log` | 0.4.35 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `maplit` | 1.0.2 | MIT/Apache-2.0 | MDX inference (transitive) |
| `matrixmultiply` | 0.3.11 | MIT/Apache-2.0 | MDX inference (transitive) |
| `memchr` | 2.8.3 | Unlicense OR MIT | MDX inference (transitive) |
| `memmap2` | 0.9.12 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `metrics` | 0.24.6 | MIT | MDX inference (transitive) |
| `minijinja` | 3.1.0 | Apache-2.0 | MDX inference (transitive) |
| `ndarray` | 0.17.2 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `nom` | 8.0.0 | MIT | MDX inference (transitive) |
| `nom-language` | 0.1.0 | MIT | MDX inference (transitive) |
| `num-bigint` | 0.4.8 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `num-complex` | 0.4.6 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `num-integer` | 0.1.47 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `num-rational` | 0.4.2 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `num-traits` | 0.2.19 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `once_cell` | 1.21.4 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `parking_lot` | 0.12.5 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `parking_lot_core` | 0.9.12 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `pastey` | 0.2.3 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `portable-atomic` | 1.15.0 | Apache-2.0 OR MIT | MDX inference (transitive) - resolved only, not in the host build graph |
| `portable-atomic-util` | 0.2.8 | Apache-2.0 OR MIT | MDX inference (transitive) - resolved only, not in the host build graph |
| `ppv-lite86` | 0.2.21 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `primal-check` | 0.3.4 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `proc-macro-crate` | 3.5.0 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `proc-macro2` | 1.0.107 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `prost` | 0.14.4 | Apache-2.0 | MDX inference (transitive) |
| `prost-derive` | 0.14.4 | Apache-2.0 | MDX inference (transitive) |
| `quote` | 1.0.47 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `r-efi` | 5.3.0 | MIT OR Apache-2.0 OR LGPL-2.1-or-later | MDX inference (transitive) - resolved only, not in the host build graph |
| `r-efi` | 6.0.0 | MIT OR Apache-2.0 OR LGPL-2.1-or-later | MDX inference (transitive) - resolved only, not in the host build graph |
| `rand` | 0.10.3 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `rand` | 0.9.5 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `rand_chacha` | 0.9.0 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `rand_core` | 0.10.1 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `rand_core` | 0.9.5 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `rand_distr` | 0.5.1 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `rand_distr` | 0.6.0 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `rapidhash` | 4.5.1 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `rawpointer` | 0.2.1 | MIT/Apache-2.0 | MDX inference (transitive) |
| `rayon` | 1.12.0 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `rayon-core` | 1.13.0 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `redox_syscall` | 0.5.18 | MIT | MDX inference (transitive) - resolved only, not in the host build graph |
| `rustfft` | 6.4.1 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `rustix` | 1.1.6 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | MDX inference (transitive) - resolved only, not in the host build graph |
| `rustversion` | 1.0.23 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `same-file` | 1.0.6 | Unlicense/MIT | MDX inference (transitive) |
| `scopeguard` | 1.2.0 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `serde` | 1.0.229 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `serde_core` | 1.0.229 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `serde_derive` | 1.0.229 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `serde_json` | 1.0.154 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `sha2` | 0.10.9 | MIT OR Apache-2.0 | checksum (direct dependency) |
| `shlex` | 2.0.1 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `simd-adler32` | 0.3.10 | MIT | MDX inference (transitive) |
| `smallvec` | 1.16.5 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `stable_deref_trait` | 1.2.1 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `strength_reduce` | 0.2.4 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `string-interner` | 0.20.0 | MIT/Apache-2.0 | MDX inference (transitive) |
| `syn` | 2.0.119 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `syn` | 3.0.9 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `tar` | 0.4.46 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `toml_datetime` | 1.1.3+spec-1.1.0 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `toml_edit` | 0.25.19+spec-1.1.0 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `toml_parser` | 1.1.5+spec-1.1.0 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `tract-core` | 0.23.10 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `tract-data` | 0.23.10 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `tract-extra` | 0.23.10 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `tract-hir` | 0.23.10 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `tract-linalg` | 0.23.10 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `tract-nnef` | 0.23.10 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `tract-onnx` | 0.23.10 | MIT OR Apache-2.0 | MDX inference (direct dependency) |
| `tract-onnx-opl` | 0.23.10 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `tract-transformers` | 0.23.10 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `transpose` | 0.2.3 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `typeid` | 1.0.3 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `typenum` | 1.20.1 | MIT OR Apache-2.0 | checksum (transitive) |
| `unicode-ident` | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 | MDX inference (transitive) |
| `version_check` | 0.9.5 | MIT/Apache-2.0 | checksum (transitive) |
| `walkdir` | 2.5.0 | Unlicense/MIT | MDX inference (transitive) |
| `wasip2` | 1.0.4+wasi-0.2.12 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | MDX inference (transitive) - resolved only, not in the host build graph |
| `wasm-bindgen` | 0.2.130 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `wasm-bindgen-macro` | 0.2.130 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `wasm-bindgen-macro-support` | 0.2.130 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `wasm-bindgen-shared` | 0.2.130 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `winapi-util` | 0.1.11 | Unlicense OR MIT | MDX inference (transitive) |
| `windows-link` | 0.2.1 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `windows-sys` | 0.61.2 | MIT OR Apache-2.0 | MDX inference (transitive) |
| `winnow` | 1.0.5 | MIT | MDX inference (transitive) - resolved only, not in the host build graph |
| `wit-bindgen` | 0.57.1 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | MDX inference (transitive) - resolved only, not in the host build graph |
| `xattr` | 1.7.0 | MIT OR Apache-2.0 | MDX inference (transitive) - resolved only, not in the host build graph |
| `zerocopy` | 0.8.63 | BSD-2-Clause OR Apache-2.0 OR MIT | MDX inference (transitive) |
| `zerocopy-derive` | 0.8.63 | BSD-2-Clause OR Apache-2.0 OR MIT | MDX inference (transitive) |
| `zmij` | 1.0.23 | MIT | MDX inference (transitive) |
<!-- dependency-inventory:end -->

Raw tool output for this table (`cargo metadata --format-version 1` JSON,
`cargo tree` logs, and the generated rows) is persisted in the cycle evidence
root under `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/`; it is evidence, not
part of the crate.

Note on `symphonia`: MPL-2.0 is file-level copyleft; it is used as an unmodified
dependency (no Symphonia source is copied into this repository), which keeps this
project's own MIT-licensed code unaffected. `symphonia` is the only decoder; no
`ffmpeg` or other external binary is invoked at build or run time.

All direct dependencies are MIT/Apache-2.0 compatible. `candle-transformers` is
used only for the Whisper model (Step 4); it pulls `fancy-regex`, `serde_plain`,
`rayon`, `tracing` and related crates from the same pinned Candle graph. The
Whisper mel filter banks in `assets/whisper/` are numeric constants (MIT /
Apache-2.0 provenance documented there), not model weights. No weights or
trained model assets are bundled or downloaded by the crate; `tokenizers` is a
pure-Rust tokenizer library, and no Python is invoked at build or run time.

## Transitive dependencies

`cargo metadata` resolved **194** crates in the dependency graph (dependency metadata inventory, not compiled packages) (including the Candle git checkout at rev `cfcbd7186d13271843ce7eb41d25659bd0835850`). For the Step-1 graph (**159** crates) **155** carry a byte-genuine evidence file under `docs/dependency-audit-evidence/` (153 copies plus 2 byte-identical copies: `windows-link-0.2.1.txt` and `windows-sys-0.61.2.txt` are byte-identical copies of their source `license-mit`, sha256 `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383`); **4** are `metadata_only` (licence expression from `cargo metadata`, no quoted licence text): `candle-nn 0.11.1`, `pulp-wasm-simd-flag 0.1.1`, `r-efi 5.3.0`, `r-efi 6.0.0`.

The Step-2 additions (**27** crates) are inventoried here by licence expression from `cargo metadata`; byte-genuine evidence files for them have **not** been regenerated yet:
- `symphonia` 0.5.5 and its 15 sub-crates — **MPL-2.0** (unmodified dependency; no Symphonia source is copied into this repository, so the project's MIT code is unaffected);
- `serde` 1.0.229 / `serde_derive` 1.0.229 / `serde_yaml` 0.9.34 / `unsafe-libyaml` 0.2.11 / `indexmap` / `hashbrown` / `equivalent` / `itoa` / `ryu` / `memchr` / `lazy_static` — MIT OR Apache-2.0 (or MIT / Apache-2.0 / Unlicense equivalents).

No crate in the graph is recorded as `UNKNOWN — evidence unavailable`; all 194 carry an explicit licence string from metadata. The Step-4 additions (`candle-transformers` 0.11.1, `tokenizers` 0.23.2, `serde_json` 1.0.154, `flate2` 1.1.11, `fancy-regex`, `serde_plain`, `miniz_oxide`, `rayon`, `tracing`, and the new `tokenizers` graph) are inventoried by licence expression from `cargo metadata`; byte-genuine evidence files for them have **not** been regenerated yet, consistent with the Step-2/3 policy above.

Full list with licence strings and evidence class: `docs/dependency-audit-evidence/s1-licence-audit.txt` (generated from `cargo metadata`); metadata-only rows are marked `metadata_only` in the evidence-file column.
