# Cycle status: issue-1-r4 / cycle 1

## Current state

- task: `issue-1-r4` · form: `cycle` · specialized skill: `pdca-coder`
- mode: `autonomous`
- cycle: `N=1` · revision: `r=1` · attempt: `n=2`
- scope: collection item `#1` of `open-issues-r4` — BS-RoFormer and the general MDXC/RoFormer family in `candle-roformer`, whole item goal at `r1` · status path: `.pdca/status/issue-1-r4.md`
- goal: a real BS-RoFormer checkpoint loads, passes strict validation and separates audio, with numerical parity against a pinned PyTorch reference (MelBand parity thresholds) and README listing the supported families and config files — the §PLAN record and §Original task are authoritative
- phase: `ACT` · record: `completed` · outcome: `accepted` — `AC-1` **`met`** per CHECK #3 (final), which routed `→ ACT` with no new proving run. Attempt `n=1` produced the candidate, CHECK returned **not accepted** (`BS-EVID-001` blocking, `BS-DOC-003` and `BS-PROV-002` minor), attempt `n=2` fixed the two documentation/provenance inaccuracies and re-froze/re-measured the candidate, and CHECK #3 accepted it (`BS-EVID-001`, `BS-PROV-002`, `BS-DOC-003` refuted for `n2`; `BS-PROV-004` minor/confirmed). ACT corrected the remaining build-root wording (`BS-PROV-004`) and committed the item files as `dc94520`; the complete accepted report is in §CHECK record, the outcome in §ACT record
- provenance correction (ACT, `BS-PROV-004`): the build and the strict test ran from the live root `F:\sources\candle-roformer` with `CARGO_TARGET_DIR` redirected, not inside `snapshot-build/tree`; the tie to the frozen candidate is build-input equality (269/269 shared files, 21/21 compiler inputs, no extra build-affecting input, `logs/provenance-facts.txt`). The earlier wording in `manifest.json`, `index.md` and §DO result is refuted and corrected; raw logs are kept unchanged
- blockers: `CONC-1 resolved` — the duplicate writer exited and 5+ minutes were quiet before any write; the implementation and measurements below were re-run (or, where the earlier run was on the same content, re-verified bit-identical) by the single writer. Evidence of the original conflict stays at `logs/concurrency-conflict.log`. A quiet-tree check before and after attempt `n=2` found no running python/cargo/rustc process and no watched-path write. At ACT/commit time the same check found no cargo/rustc/python process, an unchanged product delta, and 14/14 live files still hash-equal to `snapshot-sha256.txt`, so no `CONC-6` condition arose.
- trajectory note (not a defect): git has no configured author identity in this workspace (`user.name`/`user.email` unset locally and globally), so the single commit was made with the identity already recorded on every existing commit (`Alexey Shirshov <alex.shirshov@vk.com>`) via per-command `-c`; no git config file was written.
- source: GitHub issue `#1` of `AlexeyShirshov/candle-roformer`, item of collection `open-issues-r4` · context refs: `.pdca/status/collection-open-issues-r4.md` §`PLAN record` `r1` + `### Schedule s2`, `.pdca/open-issues-r4/rv1/plan.md`
- criteria: `AC-1` — exact text in §PLAN record `r1` (source: `.pdca/status/collection-open-issues-r4.md` §Input item 1), not retyped here

## Original task

Item brief, verbatim:

> #1 feat: support BS-RoFormer and the general MDXC/RoFormer family

Item `Done when` criterion (`AC-1`, verbatim, immutable) — exact reference:
`.pdca/status/collection-open-issues-r4.md` §`Input` item `1`; the recorded text there is
`"a real BS-RoFormer checkpoint loads, passes strict validation and separates audio; numerical parity
against a pinned PyTorch reference (same thresholds as MelBand parity gate); README lists supported
families and config files."`

The collection task, verbatim (original language, Russian), by exact reference to
`.pdca/status/collection-open-issues-r4.md` §`Original task`:

> pdca по открытым задачам из github

The follow-up instruction, verbatim (original language, Russian), by exact reference to the same
section:

> тогда делай автономно с автокомитом

No criterion is retyped as a substitute: the source above is authoritative and the excerpt is quoted
only to make this record readable on its own.

## PLAN record

### r1

`recorded by reference, not re-planned` — the authoritative item plan of this cycle is the collection
PLAN return at `r1`: `.pdca/status/collection-open-issues-r4.md` §`PLAN record` → `### r1` (item
status line `#1 ready`; acceptance criterion `AC-1` verbatim; measurement method for `#1`: "waveform
parity in f32 PCM, max abs error / RMSE / SDR over all samples/channels/stems on a frozen fixture;
same thresholds as the MelBand parity gate; strict validation with zero missing/unexpected
tensors"), plus `### Schedule s2` (group `G1` reduced to `[#1]`, one lane on the current worktree
`main`, `plan_cap 1`, `lane_cap 1`). A byte-identical copy of the collection PLAN return is
persisted at `.pdca/open-issues-r4/rv1/plan.md`. No new PLAN was started in this cycle and the form,
contract and overlay are as decided at entry (`cycle` / `pdca` / `pdca-coder`).

Operational scope carried by this item's DO brief (the contract's execution detail, not a new
criterion): full BS-RoFormer architecture with `freqs_per_bands` band split, rotary time/frequency
transformers, erf-GELU feed-forward and per-stem GLU mask estimators; `.ckpt` → safetensors
conversion with a documented key mapping; strict validation with zero missing/unexpected/shape
mismatches; pinned PyTorch reference via audio-separator 0.48.0 + torch 2.15.0+cpu on a frozen
fixture; waveform parity at correlation ≥ 0.9999999 and max abs ≤ 3.5e-4; README family/config
listing.

## Approval record

`not applicable: autonomous mode` — the user instruction "тогда делай автономно с автокомитом"
authorizes execution without an approval gate; no approval ref was opened, requested or fabricated.

## DO result

`candidate-ready — attempt n=2 (CHECK→DO re-gather after not accepted)` (a DO result, not a verdict).
The candidate of attempt `n=1` was rejected by CHECK; this attempt applied the two requested
documentation/provenance corrections, re-froze the candidate from the live tree and re-ran the
strict-validation test, the separation and the parity comparison **only** from that frozen input.
No CHECK has re-run and no verdict or acceptance is recorded here.

(State at DO. Superseded by CHECK #3 `accepted` — §CHECK record — and by the build-root correction at
the end of the `n=2` block: "only from that frozen input" holds by build-input equality, not by cwd.)

### Attempt n=2 — what this execution did (current candidate identity)

CHECK findings this execution addressed (ids and severities as delivered by the driving role's brief;
the complete report text was not persisted in this evidence root, so it is not retyped here):
`BS-EVID-001` **blocking / unverified** — the candidate could not be tied to a reproducible
snapshot-built measurement; `BS-DOC-003` minor — README overclaimed strict **dtype** validation by
the BS converter; `BS-PROV-002` minor — `manifest.json` attributed `logs/convert-report.json` to the
wrong converter.

- **product/doc change**: `README.md` only. The sentence that grouped both converters under "strict
  key/shape/dtype validation" now says what the code does: `tools/convert_bs_roformer.py` checks the
  key set and every tensor shape against the profile schema and then casts every tensor to float32
  (`convert_bs_roformer.py:194-211`); `tools/convert_voc_fv7.py` additionally checks dtypes. The
  family/config table, the CUDA/Metal "not tested" statements and the BS parity wording bound to the
  crate schema are unchanged. No code, test, config or tool edit.
- **provenance fix**: `manifest.json` → `safetensors.converter` now records `scripts/convert_ckpt.py`
  as the program that produced `logs/convert-report.json` (the report's `audio_separator`,
  `reference_state_dict_keys`, `strict_load_into_reference` and `roundtrip_spot_check` fields are
  emitted only by that script, and its `out_sha256` equals the shipped safetensors), and describes
  `tools/convert_bs_roformer.py` as the in-tree converter (pinned size/SHA-256, key/shape schema
  check, float32 cast, no dtype validation) that did not produce that file. The two converters were
  not cross-compared byte-for-byte — recorded as a scope note, not claimed.
- **re-freeze** from the live tree after the doc fix: `snapshot/` 14 files (9 modified + 5 added),
  `snapshot-sha256.txt`, `snapshot.diff` vs `bf2cab6` (161,968 B, generated with `git diff --no-index`
  against a baseline-version tree — no index, no staging), `manifest.json`, `index.md`. Identity:
  only `README.md` moved versus the attempt-1 freeze (`ea9b9e35…74f94` → `90c7a890…62514`); the other
  13 files are byte-identical.
- **machine report** `logs/refreeze-report.json`: sha256 + size + mtime of every snapshot file and its
  live counterpart (14/14 equal), manifest `changed_files` ↔ snapshot list equal (14 = 14), `HEAD` =
  `bf2cab62e1350eec59a4e5283d18e818d8ed3c54` (unmoved), and `git status --porcelain` showing exactly
  the 9 modified + 5 untracked product paths as the delta — no other product path is part of it.
- **materialization**: `git archive bf2cab6 | tar -x -C snapshot-build/tree` (rc 0, 264 files; no
  branch, no worktree) with the `snapshot/` overlay at relative paths; all 14 overlaid files re-hashed
  against `snapshot-sha256.txt` before any build → 0 mismatches
  (`logs/snapshot-build-materialize.log`). `CARGO_TARGET_DIR` =
  `snapshot-build/target`, fresh.
- **build / test / run in that tree**: `cargo build --release --example bs_roformer_separate` → rc 0
  (`logs/snapshot-build-build.log`, rustc 1.99.1); the opt-in strict-validation test with
  `CANDLE_ROFORMER_BS_SAFETENSORS` / `CANDLE_ROFORMER_BS_CONFIG` pointing at the frozen evidence
  models → rc 0, 1 passed / 0 failed, `expected=699 actual=699 missing=0 unexpected=0
  shape_mismatches=0` and `strict-load: ok` (`logs/snapshot-build-strict-validation.log`);
  the resulting executable (sha256 `4fe007ef25e51953f73db23c1827c50d4c3a3691bc6979be54a164062e89dbe8`,
  6,445,568 B) on `fixture/bs-roformer-fixture-8s-f32-44100.wav` into a fresh
  `snapshot-build/run/out` → rc 0, `load_secs=0.452`, `separation_secs=91.678`, both stems
  352,800 frames × 2 ch × 44.1 kHz, sha256 vocals
  `4d9e3ab94ecc1b8cf973739bec54e5eb07f5c450d3f0cd7a3ca888b5acf6bfcd`, instrumental
  `b37a10d4313033f161bad26cd0fe3424f36e3875a7f59978842a8c0fbd5cebc1` — byte-identical to
  `candidate/final/*.wav`, so only the README wording moved
  (`logs/snapshot-build-candidate-run.log`).
- **parity from the frozen candidate**: `scripts/compare_stems.py` run through
  `scripts/assert_stem_gate.py` (the compare script records the gate flags but does not exit non-zero
  on a miss, so the wrapper asserts corr ≥ 0.9999999 and max abs ≤ 3.5e-4 for overall and per channel,
  equal sample rate/shape, all samples finite, and reference determinism → non-zero exit on any miss).
  Both exit codes 0, 12/12 checks pass per stem. **vocals** overall corr `0.9999999999999561`,
  max abs `4.76837e-07`, RMSE `3.75262e-08`, SDR `130.302177 dB`; ch0 corr `0.9999999999999559` /
  `4.17233e-07` / `4.07631e-08` / `130.238295`; ch1 corr `0.9999999999999564` / `4.76837e-07` /
  `3.39823e-08` / `130.39248`. **instrumental** overall corr `0.9999999999999664` / `3.65078e-07` /
  `2.8601e-08` / `131.728898 dB`; ch0 `0.9999999999999648` / `3.65078e-07` / `3.07945e-08` /
  `131.525218`; ch1 `0.9999999999999685` / `2.84985e-07` / `2.62247e-08` / `131.994945`.
  Files: `compare/comparison-snapshot.json`, `compare/comparison-instrumental-snapshot.json`,
  `compare/assertions-snapshot.json`, `compare/assertions-instrumental-snapshot.json`;
  logs `logs/compare-stems-snapshot.log`, `logs/compare-stems-instrumental-snapshot.log`;
  machine report `logs/snapshot-build-report.json`.
- **environment observation, recorded because it affects reproduction**: the comparison needed the
  pinned reference venv `C:\Users\Пользователь\AppData\Local\Temp\opencode\mdx-ref-venv\Scripts\python.exe`
  (Python 3.12.10, numpy 2.5.4, soundfile 0.14.0, torch 2.15.0+cpu — the recorded reference stack),
  because the bare `...\Python312\python.exe` no longer has numpy or soundfile installed. The venv
  lives outside `.pdca/`, so it is a reproduction prerequisite, not frozen evidence.
- **hygiene re-check**: `rustfmt --check --config skip_children=true` on the same 11 task-owned Rust
  files → rc 0 after the README-only change (`logs/rustfmt-item-files-recheck.log`). No code changed,
  so the attempt-1 clippy/test totals (recorded below) still describe the same source content.

### Correction to the build root of this attempt (recorded by ACT, defect `BS-PROV-004`; the wording above is refuted)

Two bullets above say the build/test/run happened "in that tree" (`snapshot-build/tree`). That is not
what happened. CHECK #3 (final) confirmed this as a minor but real residual metadata inaccuracy and
routed it to ACT for correction. The corrected statement, as required by CHECK:

> Cargo build и strict-test выполнялись из live-корня `F:\sources\candle-roformer`, с перенаправленным
> `CARGO_TARGET_DIR`. Их входы соответствуют frozen candidate `bf2cab6 + snapshot/`: 269/269 общих
> файлов и 21/21 compiler inputs равны, дополнительных build-affecting inputs не обнаружено.
> Предыдущее утверждение «собрано внутри snapshot-build/tree» опровергнуто.

What actually happened: cargo was invoked from the live root with `CARGO_TARGET_DIR` redirected into
`snapshot-build/target`; cargo recorded `Compiling candle-roformer v0.1.0
(F:\sources\candle-roformer)` and the dep-info carries
`env-dep:CARGO_MANIFEST_DIR=F:\\sources\\candle-roformer`, every `.d` source path is relative and
resolves against that manifest dir, and no metadata under the target dir references
`snapshot-build\tree` (`logs/provenance-facts.txt` §4). The materialized tree was still built and
hash-verified (14/14 overlay hashes) but served as the comparison reference for build-input equality:
269/269 shared files byte-identical, 21/21 compiler inputs equal, and no build-affecting file in the
live root that the tree lacks (no `build.rs`, `.cargo/config*`, `rust-toolchain*`, `[patch]`/path
dependency, parent `[workspace]` manifest, `Cargo.lock` drift, generated source or extra target path).
The attribution of the measured executable to the frozen candidate therefore rests on that byte
equality, not on the working directory; no measured number of this attempt changes.

Raw logs are kept byte-identical; the hand-written header annotations in
`logs/snapshot-build-build.log`, `logs/snapshot-build-strict-validation.log` and
`logs/snapshot-build-materialize.log` are designated **refuted** here (and in `manifest.json` →
`provenance_correction`, and `index.md`) rather than rewritten.

### Attempt n=1 — retained history of the candidate that CHECK rejected

What was completed in this resumed execution (all on the final tree, `main @ bf2cab6` + this item's
uncommitted delta):

- **hygiene**: `rustfmt --edition 2021` on the 11 task-owned Rust files, then
  `rustfmt --check --config skip_children=true` on the same files → exit 0
  (`logs/rustfmt-item-files.log`). `cargo fmt --check` over the whole crate → **exit 1**, with the
  remaining diffs exclusively in `src/dsp.rs`, `src/wav.rs`, `tests/s1_spikes.rs` — files this item
  does not touch and that are byte-identical to `HEAD`; the crate was never rustfmt-clean at baseline
  and no whole-crate reformat was done (`logs/cargo-fmt-check.log`). `cargo clippy --all-targets` →
  **exit 0**, 0 errors, 51 warnings in the lib (52 in the lib test, 7/11/6 in the three legacy test
  binaries, 41 unique lint kinds), none pointing at `src/model/bs_roformer.rs`,
  `tests/bs_roformer.rs` or `examples/bs_roformer_separate.rs`; a clean-`HEAD` clippy baseline was not
  measured, so new-vs-pre-existing warnings are not fully attributed (`logs/cargo-clippy.log`).
  `cargo test` → **exit 0, 87 passed / 0 failed / 1 ignored** (the ignored test is the opt-in
  real-checkpoint validation) (`logs/cargo-test-full-final.log`); `cargo test --release --test
  bs_roformer` → exit 0, 8 passed / 0 failed / 1 ignored (`logs/cargo-test-bs.log`).
- **strict validation against the real checkpoint, with the crate's own schema**: the new opt-in test
  `tests/bs_roformer.rs::real_checkpoint_passes_strict_validation` (env-gated on
  `CANDLE_ROFORMER_BS_SAFETENSORS` / `CANDLE_ROFORMER_BS_CONFIG`, documented in README) printed
  `expected=699 actual=699 missing=0 unexpected=0 shape_mismatches=0` and then loaded the model via
  `Separator::load` (`strict-load: ok`), exit 0 (`logs/strict-validation-real-ckpt.log`). The negative
  path passes too: `bs_missing_key_is_rejected_by_the_loader` and the new
  `bs_extra_key_is_rejected_by_the_loader` both reject the tiny checkpoint with the expected error.
- **final-tree parity run** (rebuild after formatting, then one candidate run): `cargo build --release
  --example bs_roformer_separate` exit 0 (`logs/cargo-build-release-example-bs.log`); candidate exit
  0, `load_secs=0.457`, `separation_secs=95.753`, both stems 352,800 frames × 2 ch × 44.1 kHz
  (`logs/candidate-run-final.log`). Against `reference/run1-*` (audio-separator 0.48.0 + torch
  2.15.0+cpu, CPU/f32): **vocals** overall corr `0.9999999999999561`, max abs `4.768e-7`, RMSE
  `3.753e-8`, SDR `130.302 dB`; ch0 corr `0.9999999999999559` / max abs `4.172e-7` / RMSE `4.076e-8` /
  SDR `130.238 dB`; ch1 corr `0.9999999999999564` / max abs `4.768e-7` / RMSE `3.398e-8` / SDR
  `130.392 dB`. **instrumental** overall corr `0.9999999999999664`, max abs `3.651e-7`, RMSE
  `2.860e-8`, SDR `131.729 dB`; ch0 `3.651e-7`, ch1 `2.850e-7`. All channels pass the gate
  (corr ≥ 0.9999999, max abs ≤ 3.5e-4). Candidate sha256: vocals
  `4d9e3ab94ecc1b8cf973739bec54e5eb07f5c450d3f0cd7a3ca888b5acf6bfcd`, instrumental
  `b37a10d4313033f161bad26cd0fe3424f36e3875a7f59978842a8c0fbd5cebc1`; both are byte-identical to the
  earlier pre-rustfmt run of the same content, so the formatting-only rebuild did not move inference.
  Comparison artefacts: `compare/comparison.json` (sha256 `fa222316…b46ed`) and
  `compare/comparison-instrumental.json` (sha256 `0b370b80…835eb`), generated from this final run.
- **README**: `## Model files` carries the family/config table (MelBandRoformer with
  `configs/voc_fv7.yaml` and `configs/kijai_melband_384.yaml`; BS-RoFormer with
  `configs/bs_roformer_viperx_1297.yaml` and its `.ckpt` + `tools/convert_bs_roformer.py`; Whisper;
  an explicit not-implemented row for MDX-Net/VR/UVR/Demucs/MDX23C), documents the reproducible
  strict-validation test, and `## Status and limits` keeps CUDA/Metal as **not tested** with the
  BS-RoFormer parity wording bound to the crate schema. No GPU result is claimed.
- **evidence packaged** under `.pdca/open-issues-r4/rv1/item-1-bs-roformer/`: `snapshot/` (14
  byte copies, relative paths), `snapshot-sha256.txt`, `snapshot.diff` (vs `bf2cab6`, 165,921 B,
  tracked changes plus new-file diffs), `manifest.json` (checkpoint URL/size/sha256, config and
  safetensors hashes, fixture identity, reference stack/command/determinism, candidate command and
  final numbers, gates, changed files, log paths) and `index.md` (one-page AC-1 evidence map).

Known limits carried to CHECK (unchanged, not hidden): the parity fixture is one 8 s window on both
sides (352,800 samples = one chunk at hop 441 / chunk_hops 800), so multi-window overlap-add blending
is not measured; CPU/f32 only; clippy attribution is whole-crate totals; the reference shell line was
not persisted and is reconstructed from the script CLI plus `reference/summary-run1.json`.

Recovery context: the first execution of this same attempt `n=1` stopped on `CONC-1` with no product
edit and no measurement by this session. That report is retained verbatim below; it is superseded by
the candidate above and stays part of the attempt's history.

### Blocked report of the first execution of this attempt (retained, superseded)

`blocked — attempt n=1 stopped, no product edit and no measurement by this session` (a DO result, not
a verdict). Defect/blocker key `CONC-1`: a **second execution process is running this same item
attempt in the same working tree**. It edits `src/**`, the item scripts and the item logs, and it
will write this same status file, so measurements from here cannot be attributed to a fixed tree and
the single-writer rule cannot be guaranteed.

Evidence (full log `.pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/concurrency-conflict.log`):

- this session's own runs: `cargo build --lib` exit 0 at 20:30:39 local (its warning names
  `src/model/mod.rs:373`); `convert-ckpt.log` exit 1 at 20:31:46 (the published config's
  `!!python/tuple` tag broke `yaml.safe_load`) — the conversion did not run here
- runs this session did not make: `logs/hash-verify.log` (20:31:27), `logs/install-safetensors.log`
  (20:31:43), `scripts/convert_ckpt.py` rewritten twice (6424 B → 6602 B → 6864 B across
  20:31:55/20:32:13), `models/…safetensors` 639,109,056 B (20:32:19), `logs/convert-report.json` +
  `logs/convert.log` rc=0 (20:32:19/20:32:20), two python PIDs 19032 and 21824 under pwsh 24000
  (20:32:15–20:32:20), `logs/load-smoke.log` (20:33:12, whose build output names
  `src/model/mod.rs:355` for the same warning this session's build reported at line 373)
- product tree changed under this session: `src/model/bs_roformer.rs` (18,825 → 18,435 B),
  `src/model/mod.rs` (1010 → 992 lines), `src/config.rs` (14,283 B), with `git diff --stat` moving
  from `281 insertions / 94 deletions` to `284 / 95`

What this session did complete before stopping (verified on disk, `cargo build --lib` exit 0 at
20:30:39 was against a tree that no longer exists):

- item cycle record created and the collection rows updated; `status_lint.py` exit 0
- checkpoint verified: 639,331,213 B, sha256
  `5b84f37e8d444c8cb30c79d77f613a41c05868ff9c9ac6c7049c00aefae115aa` (matches the pinned value;
  `logs/checkpoint-verify.log`)
- fixture verified: 2,822,488 B, sha256 `99b1d531…3541`; config yaml verified: 2,273 B, sha256
  `2bfdd16c…af7f`

Residual work of attempt `n=1` (unchanged, not attempted here): reconcile the converted key set
against `BSRoformer::expected_key_shapes` with zero missing/unexpected/shape mismatches; two pinned
reference runs with measured run-to-run determinism; the release candidate run; the compare gate
(correlation ≥ 0.9999999, max abs ≤ 3.5e-4). The other process reported `key_count=699`,
`param_count=159,758,796`, zero missing/unexpected/shape mismatches against the *Python reference*
module and `strict_load_into_reference=true`; those are its numbers, recorded here as observed, not
as this session's evidence and not as acceptance.

Required recovery (driving role): interrupt the older executor session per `AGENTS.md`
(`POST /api/session/<sessionID>/interrupt`), let one writer finish attempt `n=1`, then run CHECK once
against that single candidate.

## CHECK record

`not run: DO in progress` — no candidate exists yet at this record's creation, so no independent
CHECK could run. This section is not a pass.

Appended after CHECK: the line above describes the record at its creation. A CHECK did run on the
attempt-`n1` candidate and returned **not accepted**, findings `BS-EVID-001` (blocking, unverified),
`BS-DOC-003` and `BS-PROV-002` (minor); the full report was delivered to DO by the driving role but
is not persisted in this evidence root, so only the finding identities are recorded. The original
report is not rewritten. **A recheck is pending** on the attempt-`n2` frozen candidate; this section
is still not a pass.

### CHECK #3 (final) — `accepted`, recorded as returned

The three earlier paragraphs stay as history; the earlier finding identities are kept as history too.
The complete final report, verbatim in its original language (Russian), as returned by the independent
`pdca-check` (source: the driving role's dispatch of 2026-11-10 for cycle `issue-1-r4`, `N=1`, `r=1`,
`n=2`; same report that routed the item `→ ACT`):

````text
## Вердикт: `accepted` → ACT

Новые наблюдения в `logs/provenance-facts.txt` закрывают атрибуцию **по эквивалентности полных build inputs**, а не по заявлению о рабочем каталоге. AC-1 не требует сборки физически внутри `snapshot-build/tree`; это был способ доказательства, а не пользовательский критерий.

### Матрица

| ID | Статус | Доказательства |
|---|---|---|
| **AC-1** | **met** | Реальный checkpoint идентифицирован SHA-256; strict-test rc 0: **699/699**, missing/unexpected/shape mismatches **0**. Разделение rc 0. Происхождение executable связано с frozen candidate через 269/269 одинаковых файлов, 21/21 compiler inputs и отсутствие дополнительных build-входов (`provenance-facts.txt:15–286`). Хеш executable соответствует run report; хеши stems — comparison JSON. README перечисляет семьи/config-файлы и сохраняет GPU «not tested». |

Паритет пересчитанных stems, по всем **705 600 отсчётам каждого stem**, включая каждый канал:

| Stem | Correlation | Max abs | RMSE | SDR |
|---|---:|---:|---:|---:|
| vocals | 0.9999999999999561 | 4.76837e-7 | 3.75262e-8 | 130.302177 dB |
| instrumental | 0.9999999999999664 | 3.65078e-7 | 2.86010e-8 | 131.728898 dB |

Оба канала проходят corr ≥ 0.9999999 и max abs ≤ 3.5e-4; `compare/assertions-*-snapshot.json`: 12/12 проверок на stem.

### Findings

- **BS-EVID-001 / AC-1 / blocking:** текущий разрыв **refuted**, закрыт новыми наблюдениями. Историческая проблема была реальной.
- **BS-PROV-002 / AC-1 / minor:** **refuted для n2**, converter provenance исправлена.
- **BS-DOC-003 / AC-1 / minor:** **refuted для n2**, dtype-overclaim исправлен.
- **BS-PROV-004 / AC-1 / minor / confirmed:** в manifest/index/status остаётся неверное утверждение о сборке внутри materialized tree. Это остаточная неточность метаданных, не разрыв атрибуции после новых доказательств.

### Обязательная корректировка записи в ACT

В `manifest.json`, `index.md` и status записать:

> Cargo build и strict-test выполнялись из live-корня `F:\sources\candle-roformer`, с перенаправленным `CARGO_TARGET_DIR`. Их входы соответствуют frozen candidate `bf2cab62e1350eec59a4e5283d18e818d8ed3c54 + snapshot/`: 269/269 общих файлов и 21/21 compiler inputs равны, дополнительных build-affecting inputs не обнаружено. Предыдущее утверждение «собрано внутри snapshot-build/tree» опровергнуто.

Сырые логи сохранить; ложные аннотации обозначить как опровергнутые. BS-PROV-004 закрыть после исправления записей.

### Пределы и переход

Критерий покрывает цель, измерим, противоречий нет. Проверены CPU/f32 и одно восьмисекундное окно; multi-window parity и GPU не установлены. Хеширование выполнено наблюдателями; я проверил предоставленный артефакт, самостоятельно команды не запускал. Временная непрерывность опирается на записанные hashes/mtime, не на forensic-аудит.

**→ ACT:** результат принят; нового proving run не требуется. `N=1`, `r=1`, `n=2/3` неизменны. Уверенность высокая в пределах указанной проверки.
````

Item-level result of this report: `AC-1` **`met`**; no criterion is `unmet` or `unverified`. Findings:
`BS-EVID-001`, `BS-PROV-002` and `BS-DOC-003` refuted for `n2`; `BS-PROV-004` (minor, confirmed)
routed to ACT and corrected there (§ACT record, §Defect history). Counters unchanged: `N=1`, `r=1`,
`n=2` of 3.

## ACT record

`completed — outcome: accepted` (recorded by ACT from CHECK #3's `accepted`; ACT issues no verdict of
its own and ran no new proving action).

- **outcome**: `accepted` for item `#1`; `AC-1` **`met`**; no `unmet` or `unverified` requirement
  remains. `BS-EVID-001` (blocking), `BS-PROV-002` and `BS-DOC-003` are refuted for `n2`;
  `BS-PROV-004` (minor, confirmed) is corrected as recorded below.
- **completed work**
  - **`BS-PROV-004` record correction** — `manifest.json` gained a `provenance_correction` block and
    corrected values for `candidate.build_command`, `candidate.built_from`, `candidate.build_root`,
    `parity.snapshot_build.what`, `gates.snapshot_build`, `repo`; `index.md` gained a "Provenance
    correction (BS-PROV-004)" block in `## Frozen candidate and the build that was measured` and
    corrected AC-1 rows. The statement CHECK mandated is quoted verbatim in `manifest.json`
    (`provenance_correction.statement_ru`), `index.md` and this record:

    > Cargo build и strict-test выполнялись из live-корня `F:\sources\candle-roformer`, с
    > перенаправленным `CARGO_TARGET_DIR`. Их входы соответствуют frozen candidate
    > `bf2cab62e1350eec59a4e5283d18e818d8ed3c54 + snapshot/`: 269/269 общих файлов и 21/21 compiler
    > inputs равны, дополнительных build-affecting inputs не обнаружено. Предыдущее утверждение
    > «собрано внутри snapshot-build/tree» опровергнуто.

    Raw logs are untouched; the false hand-written annotations in `logs/snapshot-build-build.log`,
    `logs/snapshot-build-strict-validation.log` and `logs/snapshot-build-materialize.log` are
    designated refuted in all three records instead of being rewritten. `manifest.json` re-validated
    with `json.load` → OK.
  - **single atomic item commit** `dc94520550418df3f5d583523d04251bf86859bc`
    (`#1 feat: support BS-RoFormer and the general MDXC/RoFormer family`) on `main` (no branch
    switch): exactly the 14 authorized product paths, 2500 insertions / 343 deletions; no weights,
    no `.pdca/**`, no `.openidea/**`, no `target/**`, no `.gitignore` or unrelated file; no push, no
    amend, no history rewrite. Log: `logs/commit-message.txt`, `logs/commit-verify.log`.
  - pre-commit verification: all 14 product files still hash-equal to `snapshot-sha256.txt` (14/14,
    0 mismatches — the committed bytes are the measured bytes); a quiet-tree check found no
    cargo/rustc/python process and no product write, so no `CONC-6` condition.
  - post-commit verification: `git log -1` = the hash above, `git show --stat` = exactly the 14
    named paths, `git status --porcelain` shows only untracked `.pdca/status/*.md` records and
    `.openidea/` (no product path), `git rev-parse --abbrev-ref HEAD` = `main`.
  - this record and `.pdca/status/collection-open-issues-r4.md` updated; `status_lint.py` exit 0 on
    both (`logs/lint-05-item-act.log`, `logs/lint-06-collection-act.log`).
- **evidence pointers**: `logs/provenance-facts.txt` (build-root attribution: 269/269 shared files,
  21/21 compiler inputs, `CARGO_MANIFEST_DIR` = live root, no extra build-affecting input);
  `manifest.json` → `provenance_correction`; `index.md` → "Provenance correction"; the accepted CHECK
  report in §CHECK record; `compare/assertions-*-snapshot.json`; `logs/snapshot-build-report.json`;
  commit `dc94520` and `logs/commit-verify.log`.
- **remaining work for `AC-1`**: `none`.
- **limitations carried into the accepted result** (unchanged, not hidden): CPU/f32 only; the parity
  fixture is one 8 s window on both sides (352,800 samples = one chunk at hop 441 / chunk_hops 800),
  so multi-window overlap-add blending is not measured; no GPU claim — CUDA/Metal stay "not tested"
  in README and the accepted result says nothing about them; the build-input hashes and mtimes were
  gathered by read-only observers and CHECK verified the provided artefact without re-running the
  commands itself; temporal continuity rests on the recorded hashes/mtimes, not on a forensic audit;
  clippy new-vs-pre-existing attribution remains whole-crate totals; `cargo fmt --check` is still
  red only on three files outside this item's footprint; the reference shell line is reconstructed
  from the script CLI plus `reference/summary-run1.json`.
- **handoff**: item `#1` is finalized as `done` in the collection record (`disposition=finalized`,
  attempts 2, group `G1` → `done`). The next authorized action is the collection-level
  **integrated-tree CHECK** of `open-issues-r4` — item `#1` is accepted, `#2`, `#3`, `#4`, `#8` and
  `#11` are PLAN-terminal, so that CHECK is the remaining gate before the collection's own ACT and
  its mandatory retrospective. This item record authorizes no further item work.

### External GitHub closure (recorded 2026-11-10T17:38Z; appended, the report above is not rewritten)

**GitHub issue `#1` closed as completed; comment `6774583625`; commit `dc94520`.**

- Closed on the GitHub side at `2026-11-10T17:38:31Z` through the GitHub API, closure comment id
  `6774583625`, referencing the product commit
  `dc94520550418df3f5d583523d04251bf86859bc`
  (`#1 feat: support BS-RoFormer and the general MDXC/RoFormer family`).
- This is the **external closure of the already accepted item**, not a new acceptance or verdict: the
  outcome stays `accepted` / `completed`, `AC-1` stays `met`, and the counters stay `N=1`, `r=1`, `n=2` of 3.
- Nothing about the accepted result changed: no criterion, measurement, evidence artefact or product file
  was touched, and the commit `dc94520` remains the item's one atomic product commit.
- Source: the driving role's dispatch of 2026-11-10T17:38Z. The closure happened outside this workspace, so
  no local run log exists for it and none is fabricated.

## Lessons learned

The mandatory `pdca-check` retrospective ran in `mode: retrospective` and returned the report below;
it is recorded verbatim in its original language (Russian), as returned (source: the driving role's
dispatch of 2026-11-10 for cycle `issue-1-r4`, after CHECK #3 `accepted`). It adds no verdict,
criterion, counter, attempt or verification; the accepted outcome of §ACT record is unchanged.

````text
## Ретроспектива `issue-1-r4`

**Сработало:** остановка при конкурирующей записи сохранила честную атрибуцию; строгая проверка схемы и численный gate дали конкретные результаты; сопоставление входов сборки позволило использовать существующие измерения без повторного запуска.

### Уроки

1. **Оркестрация — один исполнитель на общий каталог.**  
   `CONC-1` сопровождался изменениями исходников и логов другим процессом. Короткая тишина не гарантировала завершения: записи возобновлялись. В следующий раз перед повторным назначением подтверждать завершение либо точечную отмену прежнего исполнителя; затем восстанавливать привязку измерений к фиксированному состоянию.  
   **Основание:** `logs/concurrency-conflict.log`, §2–6; `.pdca/status/issue-1-r4.md`, история восстановления.

2. **Происхождение доказательств — записывать фактический корень сборки.**  
   Три прохода CHECK понадобились из-за атрибуции, а не трёх исправлений вычислений: сначала отсутствовала связь snapshot→executable; после повторного сбора заявление «собрано внутри дерева» противоречило Cargo; затем связь установили через равенство входов. `CARGO_TARGET_DIR` задаёт каталог результатов, не исходников. В следующий раз фиксировать фактические cwd/manifest и производителя каждого отчёта непосредственно при запуске.  
   **Основание:** переданная история CHECK; `logs/provenance-facts.txt`, §4; `index.md:66–69`. Цена неточной формулировки — дополнительный CHECK и эскалация; длительность потерь не измерена.

3. **Экономия проверки — сначала выяснять, чего именно недостаёт.**  
   Наблюдение установило равенство 269 общих файлов, 21 входа компиляции, Cargo.toml/lock и 14 файлов overlay, без дополнительных влияющих файлов. Это закрыло конкретный пробел привязки без новой сборки. В следующий раз различать отсутствие вычислительного результата и отсутствие доказательства его происхождения; повторять вычисления только при необходимости. Равенство входов само по себе не доказывает корректность алгоритма.  
   **Основание:** `logs/provenance-facts.txt`, §1–6; переданная история CHECK #3.

### Ограничения и неопределённость

- Полные CHECK #1–#3 и ESCALATE в прочитанных записях отсутствуют; последовательность поздних решений дана в brief. Статусные файлы пока отражают более раннее состояние.
- Завершение конкурирующего исполнителя отмечено в DO; подтверждение API отмены здесь отсутствует.
- Доказательства ограничены CPU/f32 и одной восьмисекундной фикстурой; многоконное смешивание и GPU не исследованы.
- Исправление формулировки `BS-PROV-004` обозначено как предстоящее; его выполнение не наблюдалось. Новых проверок и изменений не выполнялось.

**Сигнал завершения:** ретроспективный анализ завершён; сохранение отчёта исполнителем не подтверждено.
````

Record closure note on the retrospective's own stated limits: the report was produced before ACT's
corrections, so its last limitation item ("`BS-PROV-004` ... обозначено как предстоящее") is the
state it observed. ACT has since performed that correction in `manifest.json`, `index.md` and these
records, and this section is the record of persistence its completion signal asked for; the report's
text is not edited. The three lessons are accepted as recorded:
`CONC-1` (orchestration, evidence `logs/concurrency-conflict.log`), build-root provenance
(evidence `logs/provenance-facts.txt` §4), and proving-economy (evidence
`logs/provenance-facts.txt` §1–6). Their scope limits stay those the report states: CPU/f32, one 8 s
fixture, no multi-window or GPU evidence, and observer-gathered hashes rather than a forensic audit.

## Revision history

`none`

## Defect history

- `CONC-1` | observed at r1 / attempt `n1` (DO stopped before any product edit or measurement) | fixes applied 0 | evidence: `.pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/concurrency-conflict.log` | last recurrence: first occurrence — concurrent duplicate execution of the same item attempt in the same tree; returned to the driving role for session-level recovery (interrupt the duplicate executor), not an implementation defect and not a criteria defect. **Resolved** in the same attempt `n1` by the single remaining writer: the duplicate exited, 5+ minutes were quiet before any write, the implementation was re-verified on disk and every measurement in §DO result was produced or re-verified by this one writer; no fixation was needed and the key stays open only as history
- `BS-EVID-001` | observed at r1 / attempt `n1` (CHECK `not accepted`; blocking, unverified) | fixes applied 1 (attempt `n2`) | cause: the attempt-`n1` numbers were measured in the live workspace tree and nothing proved that the measured binary was built from the frozen 14-file delta | fix: re-froze `snapshot/` from the live tree (14/14 files hash-equal to their live counterparts, `logs/refreeze-report.json`), materialized `bf2cab6` with `git archive` + snapshot overlay, verified 14/14 overlaid hashes before building, then ran build → strict test → separation → compare in that tree, all rc 0 (`logs/snapshot-build-*.log`, `compare/*-snapshot.json`, `logs/snapshot-build-report.json`) | last recurrence: first occurrence; **closed** — CHECK #3 (final) refuted the remaining gap for `n2`, tied the executable to the frozen candidate by build-input equality and accepted `AC-1` (§CHECK record; `logs/provenance-facts.txt`) (fixes applied 1, no third-fix claim)
- `BS-DOC-003` | observed at r1 / attempt `n1` (CHECK minor) | fixes applied 1 (attempt `n2`) | cause: README grouped both converters under "strict key/shape/dtype validation" although the BS converter only checks keys/shapes and casts to float32 (`tools/convert_bs_roformer.py:194-211`) | fix: `README.md` now states that behaviour exactly and says the v7 converter additionally checks dtypes | evidence: `snapshot/README.md` sha256 `ea9b9e35…74f94` → `90c7a890…62514`, `snapshot.diff`, `logs/refreeze-report.json` | last recurrence: first occurrence, doc-only, no code change; **refuted for `n2`** by CHECK #3 (the README now states the converter behaviour exactly) — closed
- `BS-PROV-002` | observed at r1 / attempt `n1` (CHECK minor) | fixes applied 1 (attempt `n2`) | cause: `manifest.json` attributed `logs/convert-report.json` to `tools/convert_bs_roformer.py` | fix: `safetensors.converter` now records `scripts/convert_ckpt.py` as the producer (matching the report's script-only fields and its `out_sha256`) and describes the in-tree tool separately as a converter that did not produce the file, with an explicit note that the two were not cross-compared byte-for-byte | evidence: `manifest.json` → `safetensors.converter` | last recurrence: first occurrence, provenance-only; **refuted for `n2`** by CHECK #3 (converter provenance corrected) — closed
- `BS-PROV-004` | observed at r1 / attempt `n2` (CHECK #3, minor, **confirmed**) | fixes applied 1 (ACT) | cause: `manifest.json`, `index.md` and the status records stated that the cargo build and the strict-validation test ran inside the materialized `snapshot-build/tree`, while cargo actually ran from the live root with `CARGO_TARGET_DIR` redirected (`env-dep:CARGO_MANIFEST_DIR=F:\sources\candle-roformer`, `Compiling candle-roformer v0.1.0 (F:\sources\candle-roformer)`, and no artifact metadata references the tree path) — residual metadata inaccuracy, not a break of the attribution after the build-input evidence | fix: corrected the wording in `manifest.json` (`provenance_correction` block plus `candidate.build_command`/`built_from`/`build_root`, `parity.snapshot_build.what`, `gates.snapshot_build`, `repo`), `index.md` ("Provenance correction (BS-PROV-004)" block, AC-1 rows, build section) and both status records, restating the attribution as build-input equality (269/269 shared files, 21/21 compiler inputs, no extra build-affecting input); raw logs kept byte-identical and the false hand-written annotations designated refuted; `manifest.json` re-validated with `json.load` OK | evidence: `logs/provenance-facts.txt` §1–2, §4–6; `manifest.json` → `provenance_correction`; `index.md` → "Provenance correction" | last recurrence: first occurrence, metadata-only, no measured number moved — **closed after the record correction** as CHECK required; no escalation and no `P:` task

## Changed files

Task-owned product paths (item `#1`) — **committed** as the single atomic commit
`dc94520550418df3f5d583523d04251bf86859bc` (`#1 feat: support BS-RoFormer and the general
MDXC/RoFormer family`) on `main`: exactly these 14 paths, 2500 insertions / 343 deletions, no
weights, no `.pdca/**`, no `.openidea/**`, no `target/**`:

- `src/config.rs` → modified: model-family enum (`ModelFamily`), `freqs_per_bands`, inference profile fields
- `src/model/mod.rs` → modified: `RoformerModel` trait, derived rotary tables (`RotaryEmbedding::standard`), erf-GELU feed-forward, family dispatch for `expected_key_shapes`
- `src/model/bs_roformer.rs` → created: full BS-RoFormer model + `expected_key_shapes`
- `src/separator.rs` → modified: family dispatch, `Box<dyn RoformerModel>`
- `src/separate.rs` → modified: `&dyn RoformerModel`, `chunk_starts`
- `src/lib.rs` → modified: exports (`BSRoformer`, `RoformerModel`)
- `configs/bs_roformer_viperx_1297.yaml` → created: published Viperx 1297 BS-RoFormer profile
- `examples/bs_roformer_separate.rs` → created: example driver for the parity run
- `tests/bs_roformer.rs` → created: profile parsing, key schema, OLA schedule, tiny end-to-end load,
  missing/extra-key negative path, and the opt-in `real_checkpoint_passes_strict_validation`
- `tests/core_tests.rs`, `tests/library_api.rs`, `tests/streaming_chunked.rs` → modified: the two new
  `SeparatorConfig` fields in each tiny config (plus rustfmt-only changes)
- `tools/convert_bs_roformer.py` → created: offline `.ckpt` → safetensors converter with the pinned
  size/SHA-256, a key/shape schema check, a float32 cast and a report (no dtype validation — the
  corrected §BS-DOC-003 wording)
- `README.md` → modified: family/config table, reproducible strict-validation instructions,
  BS-RoFormer parity wording bound to the crate schema; CUDA/Metal stay "not tested". Attempt `n2`
  corrected the converter paragraph: the BS converter checks the key set and shapes and then casts to
  float32, `tools/convert_voc_fv7.py` additionally checks dtypes (`BS-DOC-003`)

Evidence artefacts (`.pdca` only, ignored by `/.pdca/*`): this record; under
`.pdca/open-issues-r4/rv1/item-1-bs-roformer/` — `fixture/`, `models/`, `scripts/`, `logs/`,
`reference/`, `candidate/`, `compare/`, plus the packaged `snapshot/`, `snapshot.diff`,
`snapshot-sha256.txt`, `manifest.json` and `index.md`. Logs of this resumed execution:
`logs/cargo-fmt-check.log`, `logs/rustfmt-item-files.log`, `logs/cargo-clippy.log`,
`logs/cargo-test-full-final.log`, `logs/cargo-test-bs.log`, `logs/strict-validation-real-ckpt.log`,
`logs/cargo-build-release-example-bs.log`, `logs/candidate-run-final.log`,
`logs/compare-stems.log`, `logs/compare-stems-instrumental.log`, `logs/checkpoint-url-head.log`.
The original blocker evidence stays at `logs/concurrency-conflict.log`.

Attempt `n2` evidence (same root): `logs/rustfmt-item-files-recheck.log`,
`logs/refreeze-report.json`, `logs/snapshot-build-materialize.log`, `logs/snapshot-build-build.log`,
`logs/snapshot-build-strict-validation.log`, `logs/snapshot-build-candidate-run.log`,
`logs/compare-stems-snapshot.log`, `logs/compare-stems-instrumental-snapshot.log`,
`logs/snapshot-build-report.json`; artefacts `snapshot/`, `snapshot-sha256.txt`, `snapshot.diff`,
`manifest.json`, `index.md`, `scripts/assert_stem_gate.py`, `compare/comparison-snapshot.json`,
`compare/comparison-instrumental-snapshot.json`, `compare/assertions-snapshot.json`,
`compare/assertions-instrumental-snapshot.json`, and the materialized tree + build output under
`snapshot-build/` (`tree/`, `target/`, `run/out/`, `diffbase/`).
`.pdca/scripts/status_lint.py` output for this update: `logs/lint-04-item-regather.log`.

ACT evidence (same root): `logs/provenance-facts.txt` (build-root attribution, source of the
`BS-PROV-004` correction), `logs/commit-message.txt` (the committed message, verbatim),
`logs/commit-verify.log` (`git log -1`, `git show --stat`, `git status --porcelain`,
`git rev-parse --abbrev-ref HEAD` after the commit), `logs/lint-05-item-act.log`; the corrected
`manifest.json` (→ `provenance_correction`, `repo`) and `index.md` (→ "Provenance correction").

## Git state

`status path: .pdca/status/issue-1-r4.md — created and modified this cycle (attempt n1, the CHECK→DO
re-gather of attempt n2, then the ACT finalization) · untracked (durable on disk, not committed)`

- baseline supplied by the cycle brief: `main @ bf2cab6` (subject `chore: track the remaining #13
  cycle evidence referenced by the status record`)
- **item commit**: `dc94520550418df3f5d583523d04251bf86859bc` — `#1 feat: support BS-RoFormer and the
  general MDXC/RoFormer family` — on `main` (no branch switch, no worktree, no merge): exactly the 14
  task-owned paths, 2500 insertions / 343 deletions; no weights, no `.pdca/**`, no `.openidea/**`, no
  `target/**`; no push, no amend, no history rewrite. The author identity was reused from the existing
  history (`Alexey Shirshov <alex.shirshov@vk.com>`) through per-command `-c` flags because no git
  identity is configured in this workspace; no git config file was written
- pre-commit verification: 14/14 product files hash-equal to `snapshot-sha256.txt`, so the committed
  bytes are the measured bytes; quiet tree with no cargo/rustc/python process — no `CONC-6`
- post-commit observation: `HEAD` = `dc94520`; `git show --stat` lists exactly those 14 paths;
  `git status --porcelain` = `?? .openidea/`, `?? .pdca/status/collection-open-issues-r3.md`,
  `?? .pdca/status/collection-open-issues-r4.md`, `?? .pdca/status/issue-1-r4.md`,
  `?? .pdca/status/issue-2-r4.md` — no product path dirty; `git rev-parse --abbrev-ref HEAD` = `main`
- line endings: `core.autocrlf=true` and no `.gitattributes` in this repo, so git stores LF blobs;
  `README.md` and `src/lib.rs` are CRLF in the working tree and LF in the index. The measured bytes
  were the working-tree bytes and a fresh checkout converts back to CRLF, so a checkout reproduces
  the measured file. Pre-existing repo behaviour (the baseline blobs are already `i/lf`), not a change
  introduced by this item
- `src/dsp.rs`, `src/wav.rs` and `tests/s1_spikes.rs` are byte-identical to `HEAD`; a `rustfmt` run
  that followed `mod` declarations into `src/dsp.rs`/`src/wav.rs` was reverted with
  `git checkout -- src/dsp.rs src/wav.rs` before any measurement, so they carry no item delta
- pending commit: none for this item — its accepted delta is committed. The status records and the
  `.pdca` evidence stay uncommitted by design (`/.pdca/*` is ignored); nothing of them was staged
- observed, not this item's delta and unclaimed: `.openidea/`, `.pdca/status/issue-2-r4.md`,
  `.pdca/status/collection-open-issues-r3.md`, `.pdca/status/collection-open-issues-r4.md`
- work outside this item's delta stays untouched and unclaimed

Appended on the external closure of `#1` (2026-11-10T17:38Z; the ledger above is not rewritten):

- this record, `.pdca/status/collection-open-issues-r4.md` and `.pdca/status/issue-2-r4.md` are included in
  one bookkeeping commit on `main` (`chore: record PDCA r4 status and close #1 evidence`), authored with the
  existing repository identity through per-command `-c` flags; no `git config` file was written, no push,
  no amend, no history rewrite
- the commit adds status records only; the item's product commit stays `dc94520` and nothing product-side is
  staged, modified or claimed by the bookkeeping commit

## Progress log

2026-11-10T15:30:33Z | DO | revision r1 | attempt n1 | item #1 DO started in this cycle record (continuation of attempt n1, not a new attempt); collection row #1 pending→in-progress, group G1 pending→in-progress, attempts 0→1; on-disk state re-verified: cargo build --lib exit 0, fixture sha256 and model yaml sha256 match the values left by the previous DO execution, checkpoint download still in flight and unusable | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/cargo-build-lib.log
2026-11-10T15:35:00Z | DO | revision r1 | attempt n1 | checkpoint verified complete: 639,331,213 B, sha256 5b84f37e8d444c8cb30c79d77f613a41c05868ff9c9ac6c7049c00aefae115aa = pinned value (the stale curl finished; no partial file was ever used) | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/checkpoint-verify.log
2026-11-10T15:36:45Z | DO | revision r1 | attempt n1 | BLOCKED defect CONC-1: a second execution process is running the same item attempt in the same working tree — it rewrote scripts/convert_ckpt.py (6424→6864 B), created logs/hash-verify.log, logs/install-safetensors.log, logs/convert*.log, models/…safetensors, logs/load-smoke.log, ran python PIDs 19032/21824 at 20:32:15 and changed src/model/mod.rs (1010→992 lines), src/model/bs_roformer.rs (18825→18435 B) and src/config.rs after this session's 20:30:39 build; no product edit and no measurement were made by this session; returned to the driving role for session-level recovery (interrupt the duplicate executor), fixes applied 0 | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/concurrency-conflict.log
2026-11-10T16:08:00Z | DO | revision r1 | attempt n1 | DO resumed by the single remaining writer after `CONC-1`: no python/cargo/rustc process running and no product mtime change in the previous 60 s; the on-disk implementation, checkpoint, converted safetensors and reference/candidate artefacts were re-verified; no new attempt, no re-plan, no new CHECK | .pdca/open-issues-r4/rv1/item-1-bs-roformer/
2026-11-10T16:12:37Z | DO | revision r1 | attempt n1 | hygiene on the final tree: rustfmt on the 11 task-owned Rust files and `rustfmt --check --config skip_children=true` exit 0; `cargo fmt --check` over the whole crate exit 1 with diffs only in `src/dsp.rs`, `src/wav.rs`, `tests/s1_spikes.rs` (untouched, byte-identical to HEAD - the crate was never rustfmt-clean at baseline); a rustfmt pass that followed `mod` declarations into `src/dsp.rs`/`src/wav.rs` was reverted with `git checkout --` before any measurement | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/rustfmt-item-files.log
2026-11-10T16:14:39Z | DO | revision r1 | attempt n1 | final-tree measurement re-bound: release example rebuilt (exit 0) and the candidate re-run on the frozen fixture - exit 0, load_secs 0.457, separation_secs 95.753, both stems 352800 frames; stems byte-identical to the earlier pre-rustfmt run (sha256 vocals 4d9e3ab9…bfcd, instrumental b37a10d4…ebc1) | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/candidate-run-final.log
2026-11-10T16:14:50Z | DO | revision r1 | attempt n1 | parity re-compared against reference run1: vocals corr 0.9999999999999561 / max abs 4.768e-7 / RMSE 3.753e-8 / SDR 130.302 dB, instrumental corr 0.9999999999999664 / max abs 3.651e-7 / RMSE 2.860e-8 / SDR 131.729 dB, all channels inside the gate; comparison.json and comparison-instrumental.json regenerated from this run | .pdca/open-issues-r4/rv1/item-1-bs-roformer/compare/comparison.json
2026-11-10T16:15:15Z | DO | revision r1 | attempt n1 | crate-schema strict validation against the real converted checkpoint: opt-in `#[ignore]` test with env-var paths prints expected=699 actual=699 missing=0 unexpected=0 shape_mismatches=0 and then `Separator::load` ok; negative path (missing key, unexpected key) passes | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/strict-validation-real-ckpt.log
2026-11-10T16:16:37Z | DO | revision r1 | attempt n1 | gates: `cargo clippy --all-targets` exit 0 (0 errors; lib 51 / lib-test 52 / core_tests 7 / library_api 11 / streaming_chunked 6 warnings, none in the new module files); `cargo test` exit 0 - 87 passed, 0 failed, 1 ignored; `cargo test --release --test bs_roformer` exit 0 - 8 passed, 1 ignored | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/cargo-test-full-final.log
2026-11-10T16:18:00Z | DO | revision r1 | attempt n1 | evidence packaged for CHECK: `snapshot/` 14 byte copies, `snapshot-sha256.txt`, `snapshot.diff` (165,921 B vs bf2cab6), `manifest.json`, `index.md`; README `## Model files` table and `## Status and limits` verified complete, CUDA/Metal still "not tested" | .pdca/open-issues-r4/rv1/item-1-bs-roformer/index.md
2026-11-10T16:56:00Z | DO | revision r1 | attempt n2 | attempt n2 opened by the CHECK→DO routing for item #1: CHECK returned `not accepted` with `BS-EVID-001` (blocking, unverified), `BS-DOC-003` and `BS-PROV-002` (minor). Pre-start quiet check: no python/cargo/rustc process running and no watched product path changed in the previous 60 s. Note recorded as instructed: the re-gather alone would not be a counted attempt, but the README correction is a product-documentation change, so this execution increments the attempt to `n=2` (`r=1` unchanged, no replan) | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/refreeze-report.json
2026-11-10T16:58:00Z | DO | revision r1 | attempt n2 | `BS-DOC-003` fixed in `README.md`: the paragraph no longer claims strict dtype validation for the BS converter — it now says the converter checks the key set and every tensor shape and then casts to float32 (`tools/convert_bs_roformer.py:194-211`), with the v7 converter named as the one that also checks dtypes; family/config table, CUDA/Metal "not tested" and the parity wording untouched. `rustfmt --check --config skip_children=true` on the same 11 task-owned Rust files → rc 0 | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/rustfmt-item-files-recheck.log
2026-11-10T16:58:40Z | DO | revision r1 | attempt n2 | `BS-PROV-002` fixed in `manifest.json`: `safetensors.converter` now records `scripts/convert_ckpt.py` as the producer of `logs/convert-report.json` (report-only fields `audio_separator`, `reference_state_dict_keys`, `strict_load_into_reference`, `roundtrip_spot_check`; its `out_sha256` equals the shipped safetensors), and describes `tools/convert_bs_roformer.py` as the in-tree converter (pinned size/SHA-256, key/shape check, float32 cast, no dtype validation) that did not produce the file, with an explicit not-cross-compared note | .pdca/open-issues-r4/rv1/item-1-bs-roformer/manifest.json
2026-11-10T16:59:00Z | DO | revision r1 | attempt n2 | candidate re-frozen from the live tree after the doc fix: `snapshot/` 14 files (9 modified + 5 added), `snapshot-sha256.txt`, `snapshot.diff` 161,968 B vs bf2cab6 (git diff --no-index against a baseline-version tree; no index, no staging). Only README.md differs from the attempt-1 freeze (ea9b9e35…74f94 → 90c7a890…62514); machine report: 14/14 snapshot files equal their live counterparts, manifest changed_files ↔ snapshot list equal, HEAD still bf2cab62e1350eec59a4e5283d18e818d8ed3c54, git status delta exactly the 9 M + 5 ?? product paths | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/refreeze-report.json
2026-11-10T17:00:30Z | DO | revision r1 | attempt n2 | materialization for BS-EVID-001: `git archive bf2cab6` piped into `tar -x -C snapshot-build/tree` rc 0 (264 files; no branch, no worktree) with the `snapshot/` overlay at relative paths; all 14 overlaid files re-hashed against `snapshot-sha256.txt` before any build → 0 mismatches | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/snapshot-build-materialize.log
2026-11-10T17:01:10Z | DO | revision r1 | attempt n2 | snapshot-tree build: `cargo build --release --example bs_roformer_separate` → rc 0, finished in 1m02s, rustc 1.99.1 / cargo 1.99.1, fresh `CARGO_TARGET_DIR=snapshot-build/target` | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/snapshot-build-build.log
2026-11-10T17:02:00Z | DO | revision r1 | attempt n2 | snapshot-tree strict validation: `cargo test --release --test bs_roformer -- --ignored --nocapture real_checkpoint_passes_strict_validation` with CANDLE_ROFORMER_BS_SAFETENSORS/CANDLE_ROFORMER_BS_CONFIG on the frozen evidence models → rc 0, 1 passed / 0 failed, expected=699 actual=699 missing=0 unexpected=0 shape_mismatches=0, `strict-load: ok` | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/snapshot-build-strict-validation.log
2026-11-10T17:03:40Z | DO | revision r1 | attempt n2 | snapshot-built executable run on the frozen fixture into a fresh empty output dir → rc 0, load_secs 0.452, separation_secs 91.678, both stems 352800 frames × 2 ch × 44100 Hz; executable sha256 4fe007ef25e51953f73db23c1827c50d4c3a3691bc6979be54a164062e89dbe8 (6,445,568 B), vocals sha256 4d9e3ab94ecc1b8cf973739bec54e5eb07f5c450d3f0cd7a3ca888b5acf6bfcd, instrumental b37a10d4313033f161bad26cd0fe3424f36e3875a7f59978842a8c0fbd5cebc1 — byte-identical to candidate/final, so only the README wording moved | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/snapshot-build-candidate-run.log
2026-11-10T17:04:20Z | DO | revision r1 | attempt n2 | parity from the frozen candidate with explicit assertions added around `scripts/compare_stems.py` (`scripts/assert_stem_gate.py`: corr ≥ 0.9999999, max abs ≤ 3.5e-4 overall and per channel, equal sample rate/shape, all samples finite, reference determinism, non-zero exit on a miss): both rcs 0, 12/12 checks pass per stem. vocals corr 0.9999999999999561 / max abs 4.76837e-07 / RMSE 3.75262e-08 / SDR 130.302177 dB; instrumental corr 0.9999999999999664 / max abs 3.65078e-07 / RMSE 2.8601e-08 / SDR 131.728898 dB; reference run1/run2 still bitwise identical | .pdca/open-issues-r4/rv1/item-1-bs-roformer/compare/comparison-snapshot.json
2026-11-10T17:06:00Z | DO | revision r1 | attempt n2 | end-of-execution quiet check: no python/cargo/rustc process running, no product write in the previous 460 s, snapshot vs live tree 14/14 hash-equal, HEAD unchanged, no CONC-4 condition observed. Note: the comparison had to run under the pinned reference venv `…\Temp\opencode\mdx-ref-venv\Scripts\python.exe` (Python 3.12.10 + numpy 2.5.4 + soundfile 0.14.0 + torch 2.15.0+cpu) because the bare Python312 interpreter no longer has numpy/soundfile; recorded as a reproduction prerequisite, not frozen evidence | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/snapshot-build-report.json
2026-11-10T17:24:00Z | ACT | revision r1 | attempt n2 | CHECK #3 (final) returned accepted for AC-1 (met) and routed the item to ACT; no new proving run, counters unchanged (N=1, r=1, n=2/3). The complete report is recorded verbatim in the CHECK record and the retrospective in Lessons learned | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/provenance-facts.txt
2026-11-10T17:24:00Z | ACT | revision r1 | attempt n2 | BS-PROV-004 fixed: the build-root wording corrected in manifest.json (new provenance_correction block plus candidate/parity/gates/repo fields), index.md (Provenance correction block, AC-1 rows, build section) and this record; the attribution is restated as build-input equality (269/269 shared files, 21/21 compiler inputs, no extra build-affecting input), raw logs kept and the false annotations designated refuted; manifest.json re-validated with json.load -> OK | .pdca/open-issues-r4/rv1/item-1-bs-roformer/manifest.json
2026-11-10T17:24:00Z | ACT | revision r1 | attempt n2 | single atomic item commit on main (commit time 2026-11-10T17:21:45Z): dc94520550418df3f5d583523d04251bf86859bc '#1 feat: support BS-RoFormer and the general MDXC/RoFormer family' - exactly the 14 authorized product paths, 2500 insertions / 343 deletions, no weights, no .pdca/**, no .openidea/**, no target/**; pre-commit 14/14 product files hash-equal to snapshot-sha256.txt and quiet tree (no CONC-6); no push, no amend, no branch switch | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/commit-verify.log
2026-11-10T17:24:00Z | ACT | revision r1 | attempt n2 | post-commit verification: git log -1 = dc94520, git show --stat = exactly 14 paths, git status --porcelain = only untracked .pdca/status/*.md and .openidea/ (no product path), git rev-parse --abbrev-ref HEAD = main; ACT record completed with outcome accepted, remaining work none, limitations CPU/f32 + one 8 s window + no GPU claim, handoff = collection integrated-tree CHECK | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/commit-verify.log
2026-11-10T17:38:31Z | ACT | revision r1 | attempt n2 | GitHub issue `#1` closed as completed via the GitHub API — closure comment id 6774583625, referencing commit dc94520; external closure of the accepted item, not a new acceptance: outcome stays accepted/completed, AC-1 stays met, counters stay N=1 / r=1 / n=2 of 3, and no product file, criterion, measurement or evidence artefact changed | .pdca/status/issue-1-r4.md §ACT record
2026-11-10T17:38:31Z | ACT | revision r1 | attempt n2 | bookkeeping: this note and the matching note in `.pdca/status/collection-open-issues-r4.md` are the only writes; the r4 status records are then committed as one bookkeeping commit on `main` (`chore: record PDCA r4 status and close #1 evidence`), no push, nothing product-side staged | .pdca/status/collection-open-issues-r4.md §Git state
