# Cycle status: issue-4-r5 / cycle 1

## Current state

- task: `issue-4-r5` (item `#4`, HTDemucs) · form: `cycle` · specialized skill: `pdca-coder`
- mode: `autonomous`
- cycle: `N=1` · revision: `r=2` · attempt: `n=1/3` (attempt `n=1` of contract `r2`; the `r1` attempt
  `n=1` is closed by `CHECK 1` `not accepted → PLAN`, which consumed no new attempt)
- scope: collection `open-issues-r5` item `#4` (order 4, group `G1`, branch `main`) — HTDemucs weights loaded with strict validation, a frozen fixture separated into exactly 4 stems `drums`/`bass`/`other`/`vocals`, and per-stem parity against the reference implementation at `shifts=0` and equal segment/overlap · status path: `.pdca/status/issue-4-r5.md`
- goal: `#4` is done when "htdemucs weights load with strict validation and separate a fixture into 4 stems; parity against the reference implementation and per-stem output naming" holds at the user-approved `#4` tolerance — the §PLAN record and §Original task are authoritative
- phase: `ACT` · record: `completed` · outcome: `accepted` (finalized `2026-11-12T17:13Z`: `CHECK 3`
  (`r2`/`n1`) returned **`accepted → ACT`**, the post-acceptance commit `d042c26` was performed, and the
  mandatory `pdca-check` `retrospective` report is now persisted verbatim in §Lessons learned with the
  completion signal `retrospective_complete`; item `#4` is `done` / `finalized`) — contract `r2`
  (item-level revision; the collection plan stays `r1`), attempt `n=1/3` of `r2`. `CHECK 1`
  (`r1`/`n1`) `not accepted → PLAN` and `CHECK 3` (`r2`/`n1`) `accepted → ACT` are recorded in §CHECK
  record; the committed closure, the finalization and the external GitHub residual are in §ACT record.
  No GitHub write is recorded here and the issue is **not** recorded as closed
- source: collection `open-issues-r5` item `#4` (order 4) · context refs: `.pdca/status/collection-open-issues-r5.md`, `.pdca/open-issues-r5/rv1/plan.md`, `.pdca/open-issues-r5/rv1/criteria-approval.md`, `.pdca/open-issues-r5/rv1/item-4-demucs/` (this item's evidence root)
- criteria: `AC-4.1`, `AC-4.2`, `AC-4.3(1)`, `AC-4.3(2)` — exact text in §PLAN record `r2` (current
  contract; `r1` stays recorded above it as history). Source: `.pdca/open-issues-r5/rv1/plan.md:9`
  per-item acceptance-criteria line for the `r1` text, the collection §PLAN record `r1`, and the
  planner's `r2` revision; the approved numeric addendum is quoted verbatim in §Original task, not
  retyped in the current-state line
- **Known state recorded (`2026-11-12T15:17Z`, same attempt `r1`/`n1`, continuation dispatch; appends to
  the fields above, supersedes nothing):** `HEAD 8b03cfd`, branch `main`, nothing staged, no
  commit/push/GitHub write. `GRN-4` (GroupNorm pooled the variance over trailing dims only) is
  **fixed** with red↔green evidence. `CTR-4` (`Tensor::conv_transpose1d` in candle 0.11.1 does not
  implement the `ConvTranspose` definition for kernel > 1) is **open**: `FreqConvTranspose::forward`
  was rewritten as zero-insert + flipped-kernel `conv1d`, the unit test is still red, and a
  `one_hot_stride()` helper edit carrying a leftover unused `OnceLock` stub was applied **without
  compiling or testing**. The product tree therefore does **not compile** in its current state.
  `PAR-4` is still red after the `GroupNorm` fix (`logs/57`). Next allowed step: `finish CTR-4, re-run
  parity`.
- **Update (`2026-11-12T16:01Z`, supersedes the `CTR-4`/compile clause of the bullet above; that bullet
  is kept as history):** `CTR-4` is **fixed and verified** — the dead `OnceLock` stub and the
  `one_hot_stride()` helper are removed, `FreqConvTranspose::forward` inserts the `s - 1` zeros with
  `pad_with_zeros` and convolves with the flipped kernel; `cargo test --release --lib
  model::demucs::tests` exit **`0`**, `5 passed; 0 failed` (`logs/61-ctr4-lib-tests.log`), and the tree
  **compiles**. `PAR-4` was re-measured and is **inside the approved thresholds on all 8 stem × channel
  pairs**, exit **`0`**, `1 passed; 0 failed` (`logs/62-parity-after-ctr4.log`) — a measured result only,
  **not** a verdict and not an acceptance claim. Gates done so far: `cargo build` exit `0` (`logs/63`),
  `cargo clippy --all-targets` exit `0` (`logs/65`); the full `cargo test` gate is **in progress** and
  recorded as unfinished. Next allowed step: `complete the full cargo test gate, then the converter
  tests on the fixed bytes, then record`.
- **Final state of this attempt (`2026-11-12T16:42Z`, supersedes the gate-status clause of the bullet
  above; that bullet is kept as history):** every gate ran on the fixed bytes — `cargo build` exit
  **`0`**, full `cargo test` exit **`0`** (**144 passed / 0 failed / 4 ignored**, 25 targets,
  `logs/66`), `cargo clippy --all-targets` exit **`0`** (`logs/65`), converter tests exit **`0`**
  (`Ran 10 tests` / `OK`, `logs/67`), parity exit **`0`** (`1 passed`, `logs/62`). **candidate
  numerically ready** — a DO-side statement about the candidate only: **no verdict, no PASS, no STOP,
  no acceptance, no commit, no staging, no push, no GitHub write**. Checklist outstanding (next round /
  CHECK): the evidence pack, the pre-stage re-anchor and the single atomic commit with the GitHub `#4`
  closure are **not** performed here. Next allowed step: `CHECK item #4 (r1/n1)`.
- **Update (`2026-11-12T16:15Z`, attempt complete — supersedes the "in progress / full `cargo test`
  unfinished / converter tests not re-run" clauses of the bullets above, which are kept as history):**
  `DO` attempt `r1`/`n1` is **complete** and the candidate is **ready for CHECK**. Full `cargo test`
  gate: `cargo test` (debug, whole workspace) exit **`0`** — **144 passed / 0 failed / 4 ignored** over 24 targets + doc-tests (lib 42; bs_roformer 8+1 ignored; core_tests 22; demucs_parity 1; ensemble 22+1 ignored; fp16_decision 0+2 ignored; gpu_smoke 1; library_api 12; s1_spikes 11; streaming_chunked 8; vr_arch 1; vr_pipeline_parity 1; vr_resample 1; vr_roundtrip 1; vr_samplerate 1; vr_stage_band_specs 1; vr_stage_bands 1; vr_stage_combined 1; vr_stage_mag 1; vr_stage_mask 1; vr_stage_waves 1; vr_stage_yspec 1; vr_stft_roundtrip 1; whisper_tests 3; doc-tests 1), 41m54s wall, launcher `EXIT=0` (`logs/66-gate-test-full.log`). Converter tests `Ran 10 tests` `OK` exit **`0`** (`logs/68`) and the pinned
  checkpoint re-verified by the converter itself (`8726e21a…faa8b4`, 84,141,911 B, 533 `F32`
  tensors, `logs/69`). `AC-4.1` non-numeric: strict-load rejection measured for missing / unexpected
  / mis-shaped / non-f32 (`logs/75`, exit `101` ×4) and the four stems written on disk under exactly
  `drums`/`bass`/`other`/`vocals` with an independent read-back (`logs/80`, `logs/82`, exit `0`).
  Evidence pack written (`manifest.json`, `index.md`, `snapshot/` + `snapshot-sha256.txt` 8/8,
  `snapshot.diff`, `metrics/parity.{json,csv}`, `commands.txt`, `lint.txt`). `PAR-4` closed on the
  measured candidate. No product byte was edited by this continuation; `HEAD 8b03cfd`, index empty,
  nothing staged, no commit/push/GitHub write. Next allowed step: **`CHECK item #4 (r1/n1)`**; no
  commit, no GitHub `#4` closure and no `done` is authorized before an accepted CHECK.
- **Transition recorded (`2026-11-12T16:48Z`, contract revision `r1 → r2`):** the `r1`/`n1` `DO` was
  complete with the parity measurement passing, and `CHECK 1` (`r1`/`n1`) returned **`not accepted →
  PLAN`** — `AC-4.1` `met`, `AC-4.2` `met`, `AC-4.3` `unmet`, finding `ORD-4` (blocking, confirmed: the
  acceptance ordering of `AC-4.3` against the collection's pre-`CHECK` commit prohibition). The
  planner's item-level revision `r2` preserves `AC-4.1`/`AC-4.2` verbatim and splits `AC-4.3` into
  `AC-4.3(1)` (pre-acceptance `CHECK` gate) and `AC-4.3(2)` (post-acceptance `ACT` commit + GitHub `#4`
  closure); no user requirement or numeric threshold was added, weakened or narrowed (complete report
  verbatim in §PLAN record `### r2`). The `CHECK → PLAN` transition consumed **no new attempt**:
  contract `r2` starts at attempt `n=1/3`. `DO` `r2`/`n1` then re-anchored the unchanged 8-path
  candidate **8/8** against `snapshot-sha256.txt` (`0` missing, `0` mismatch) on
  `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685`, index empty, no premature commit or push and no
  GitHub `#4` read or write — `AC-4.3(1)` readiness confirmed, no product byte edited
  (`logs/83-reanchor-r2n1.log`, exit `0`). Next allowed step: **`CHECK item #4 (r2/n1)`**.
- **Update (`2026-11-12T16:57Z`, `GH-4-PRE` evidence round under `r2` / `n=1/3` — no new attempt, no
  product byte):** `CHECK 2` routed `GH-4-PRE` (major, unverified, `AC-4.3(1)`) back to `DO` for one
  read-only observation of GitHub `#4`, because no independent observation of the absence of a premature
  closure had been stored — an evidence gap, not a finding of violation. The observation was made and
  stored: `HTTP 200` on both endpoints, `state=open`, `state_reason=null`, `closed_at=null`, observed
  `2026-11-12T16:57:05Z`, timeline `0` `closed` / `0` `reopened` (one `milestoned` event only, no
  `rel="next"` page). No premature closure exists, so none can predate this cycle's CHECK. The evidence
  pack was updated by reference only (root `github_issue` block + `AC-4.3(1)` sub-record in
  `manifest.json`, a section in `index.md`, rows in `commands.txt`); no product byte, no gate re-run, no
  commit/staging/push, **no GitHub write of any kind**. Next allowed step: **`CHECK item #4 (r2/n1)` —
  CHECK 3**; the commit and the GitHub `#4` closure stay `ACT` work under `AC-4.3(2)`.
- **`ACT` post-acceptance closure performed (`2026-11-12T17:06Z`, contract `r2` / attempt `n=1/3`)** —
  appends to the fields above and supersedes their "next allowed step `CHECK item #4 (r2/n1)`" /
  "commit not authorized" clauses (kept as history): `CHECK 3` (`r2`/`n1`) returned **`accepted → ACT`**
  (recorded as returned in §CHECK record `### CHECK 3 - r2/n1`). `AC-4.3(2)` **commit performed**:
  `d042c2628edb52deab42118e18ac69974e5ad75d` (`d042c26`) on `main`, parent
  `8b03cfda15a485b115f8b694d07c03e98fcdf685`, **8 files changed, `+3767/−4`**, subject
  `#4: HTDemucs 4-stem separation with audio-separator 0.48.0 parity`, footer `Refs #4` (no
  `Closes`/`Fixes`/auto-close keyword), **not pushed**, no branch switch, no amend/rebase, no tag. Exactly
  the 8 item paths were staged; the commit carries **no** `.pdca/**`, `.openidea/**`, weight,
  audio/fixture, log or status path. `HEAD` is now `d042c26`, the index is empty and
  `git diff --name-only HEAD -- <8 paths>` is empty (`logs/86-act-reanchor-r2n1.log`,
  `logs/87-act-commit-verify.log`). **GitHub `#4` closure = external residual handed to the calling
  driver** — no PDCA phase performs a GitHub write in this collection; the issue is **not** recorded as
  closed here. Record stays **`open`**: §Lessons learned is `pending` until the mandatory retrospective
  report is persisted. Counters unchanged `N=1 · r=2 · n=1/3`; item `#4` attempts `1`.
- **`ACT` finalization performed (`2026-11-12T17:13Z`, contract `r2` / attempt `n=1/3`)** — appends to
  the fields above and supersedes the "record stays `open`" clause of the bullet above (kept as
  history): the mandatory `pdca-check` `retrospective` report for cycle `issue-4-r5` (`N=1`, `r=2`,
  `n=1`) was routed by the calling driver in `mode: retrospective` and is persisted **verbatim** in
  §Lessons learned (26-line report block + the dispatch's own recording notes outside it), completion
  signal `retrospective_complete`. The record is therefore **`phase: ACT · record: completed ·
  outcome: accepted`** — `accepted` is `CHECK 3`'s outcome, not a new verdict. `AC-4.3(2)` commit
  component performed (`d042c26`); the **GitHub `#4` closure stays an external residual handed to the
  calling driver — NOT performed, never recorded as "issue closed"**. Counters unchanged
  `N=1 · r=2 · n=1/3`; item `#4` attempts `1`; item status `done` / `finalized`. Next allowed step:
  **collection integration `CHECK`**.

## Original task

The collection's top-level task, verbatim (original language, Russian), by exact reference — recorded
unchanged in `.pdca/status/collection-open-issues-r5.md` §Original task:

> pdca по открытым задачам из github

Prior instruction, verbatim (original language, Russian):

> тогда делай автономно с автокомитом

Later instruction, verbatim (original language, Russian):

> после каждой выполненной задачи - делай комит и закрывай задачу

Item `#4` goal and `Done when`, verbatim (unchanged from `.pdca/status/collection-open-issues-r5.md`
§Input):

```text
#4: htdemucs weights load with strict validation and separate a fixture into 4 stems; parity against the reference implementation and per-stem output naming.
```

Approved criteria addendum for `#4`, verbatim and immutable (user-approved — message `#4 ok`,
collection §Approval record, source `.pdca/open-issues-r5/rv1/criteria-approval.md:19`):

```text
- #4: "max_abs ≤ 3.5e-4, RMSE ≤ 1e-4, SDR ≥ 80 dB, comparison for each of the 4 stems, shifts=0, same segment/overlap." (stem names: drums, bass, other, vocals)
```

Source pointers: the user's task and its two follow-up instructions are the collection's §Original
task; the `Done when` line and the addendum are the collection's §Input, itself persisted from the
same sources at `.pdca/open-issues-r5/rv1/criteria-approval.md`.

## PLAN record

### r1

`Recorded by exact reference — this record was missing and is created from the persisted plan. No
criterion is authored, corrected, completed, weakened or reinterpreted by this write.`

Persisted sources, both readable and identical in content:

- `.pdca/open-issues-r5/rv1/plan.md` — the complete collection PLAN `r1` (form `collection`,
  specialized skill `pdca-coder`, recorded `2026-11-10T18:01:54Z`); line 8 holds the measurement
  method, line 9 the per-item acceptance-criteria block, line 10 the dependencies/schedule, line 11
  the constraints, line 12 the stop conditions.
- `.pdca/status/collection-open-issues-r5.md` §PLAN record `r1` — the same content as recorded in the
  collection record.

The item `#4` acceptance-criteria block, quoted verbatim from `.pdca/open-issues-r5/rv1/plan.md:9`
(the `AC-4.x` clauses; the line also carries the `#4` item clause):

```text
AC-4.1 HTDemucs weights strict-validated, 4 stems named drums/bass/other/vocals, parity vs reference at shifts=0 and equal segment/overlap with the #4 thresholds on every stem; AC-4.2 no regressions; AC-4.3 commit + GitHub #4 closed.
```

The same three criteria, as the saved collection PLAN states them, so the block is reproduced without
either source being rewritten:

- `AC-4.1` — HTDemucs weights strict-validated, 4 stems named drums/bass/other/vocals, parity vs
  reference at `shifts=0` and equal segment/overlap with the `#4` thresholds on every stem.
- `AC-4.2` — no regressions.
- `AC-4.3` — commit + GitHub `#4` closed.

The `#4` thresholds and comparison protocol, verbatim from the approved addendum
(`.pdca/open-issues-r5/rv1/criteria-approval.md:19`, approved by the user message `#4 ok` quoted in
§Original task):

```text
max_abs ≤ 3.5e-4, RMSE ≤ 1e-4, SDR ≥ 80 dB, comparison for each of the 4 stems, shifts=0, same segment/overlap.
```

Stem names, verbatim from the same addendum line: `drums`, `bass`, `other`, `vocals`.

Measurement method, verbatim from `.pdca/open-issues-r5/rv1/plan.md:8` (applies unchanged to `#4`):

```text
per stem/channel, over all samples of the full output, no trimming, no post-normalization: max_abs = max|y_cand - y_ref|; RMSE = sqrt(mean((y_cand-y_ref)^2)); SDR = 10*log10(sum(y_ref^2)/sum((y_cand-y_ref)^2)) dB; correlation = Pearson r.
```

Constraints carried by the saved PLAN (`plan.md:11`), verbatim, in so far as they bind this item:
`autonomous; no approvals; autocommit only after an accepted CHECK; one atomic commit per accepted
item on main; no push, no branch switch, no history rewrite; no weights/.pdca/.openidea/temp audio in
product commits; no criterion weakening; README keeps "not tested" for CUDA/Metal;` full Python path
`C:\Users\Пользователь\AppData\Local\Programs\Python\Python312\python.exe`; prior `.pdca` records
preserved. Schedule position (`plan.md:10`): one group `G1`, order by risk `#2 → #8 → #3 → #4`,
single worktree `main`, `plan_cap 1`, `lane_cap 1`.

Routes recorded elsewhere (not re-typed here): the item-level revisions of `#8` (`r2`, `r3`) and of
`#3` (`r2`) are item-level pointers in the collection record; item `#4` has received **no** contract
revision — its contract is `r1` as quoted above.

`Identity note:` the dispatch and the collection record label this item's evidence root
`item-4-htdemucs`; the directory that actually exists and holds this item's work is
`.pdca/open-issues-r5/rv1/item-4-demucs/`. The label mismatch is a naming difference only; this record
uses the real path and claims no second root.

### r2

`Recorded as returned by pdca-planner (item-level contract revision r1 → r2; the collection plan stays
r1). The complete report follows byte-identically; the outer fence is a container only and changes no
byte of the recorded text.`

````text
# PLAN Record — Item `#4` (`issue-4-r5`), Contract Revision `r2`

## 1. Decision Summary

- **Decision:** **Revise item `#4` contract (`r1 → r2`)** to resolve the planner-created phase-ordering contradiction (`ORD-4`) in `AC-4.3`.
- **Terminal outcome check:** **None** of the four terminal PLAN outcomes applies:
  - Not `original task is infeasible under the stated constraints`: `ORD-4` is a planner-authored phase-ordering contradiction between `AC-4.3` (`r1`) and the collection's pre-CHECK commit prohibition (`.pdca/open-issues-r5/rv1/plan.md:9,11`), not an infeasibility of the user's task.
  - Not `acceptance criteria cannot be determined` or `goal cannot be measured under the available means`: the user's requirements and the approved `#4` numerical thresholds and measurement basis are determinate, complete, and already computable under available means.
  - Not `requested result already established`: the atomic commit on `main` and closure of GitHub issue `#4` remain to be executed in `ACT` after an accepted `CHECK`, and the candidate must first pass `CHECK` under `r2`.

---

## 2. Form, Overlay, Scope, and Anchor (carried by reference)

- **Collection / Item identity:** Collection `open-issues-r5` (collection revision remains `r1`), item `#4` (HTDemucs), cycle `issue-4-r5`, group `G1`, mode `autonomous`, worktree/branch `main`.
- **Form and specialized skill (carried by reference from collection `r1`):** `collection` (independent issue goals bounded per item; item `#4` method and volume are known and bounded by the reference HTDemucs architecture, converter, and approved parity basis), overlay `pdca-coder`.
- **Entry / cycle scope:** Item `#4` only. Items `#1` and `#11` are out of scope and untouched.
- **Re-anchor snapshot:** `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685` on `main` plus the 8-path item `#4` product delta (`src/config.rs`, `src/model/mod.rs`, `src/separator.rs`, `src/model/demucs.rs`, `src/model/demucs_pipeline.rs`, `tests/demucs_parity.rs`, `tools/convert_demucs.py`, `tools/tests/test_convert_demucs.py`), snapshot `.pdca/open-issues-r5/rv1/item-4-demucs/snapshot/`, evidence root `.pdca/open-issues-r5/rv1/item-4-demucs/`.

---

## 3. Unchanged Original Task, Requested/Delivered Result, and Measurement Method

- **Original task and user-approved `#4` addendum (by exact verbatim reference, preserved without reformulation):**
  - `.pdca/status/issue-4-r5.md` §Original task (top-level collection task, both follow-up instructions, item `#4` verbatim `Done when`, and the user-approved `#4` addendum).
  - `.pdca/status/collection-open-issues-r5.md` §Original task, §Input, §Approval record, and §Constraints; `.pdca/open-issues-r5/rv1/criteria-approval.md` (user approval `#4 ok`).
- **Requested result and delivered result (identical object):** Working native Rust/Candle HTDemucs v4 4-stem separation (`drums`, `bass`, `other`, `vocals`) with converter and strict weight validation meeting the user-approved numerical parity thresholds against `audio-separator 0.48.0` without regressions, committed atomically on `main` with GitHub issue `#4` closed.
- **Measurement method (unchanged from `r1` and user-approved `#4` addendum):**
  - **Metrics and units:**
    1. Per-stem, per-channel maximum absolute sample difference `max_abs` (dimensionless float amplitude in `[-1, 1]` scale).
    2. Per-stem, per-channel root-mean-square error `RMSE` (dimensionless float amplitude).
    3. Per-stem, per-channel signal-to-distortion ratio `SDR` in decibels (`dB`), with the reference stem output as signal and `(candidate - reference)` as distortion.
  - **Denominator and inclusion/exclusion filters:** Evaluated across every stem (`drums`, `bass`, `other`, `vocals`) and every channel over **all output samples** of the full-length separated audio; no sample trimming, no window exclusion, and no post-normalization.
  - **Comparability conditions:** Same checkpoint weights, `audio-separator 0.48.0` reference run in `CPU` / `f32`, `shifts = 0`, equal chunking/overlap (`segment = 7.8 s`, `overlap = 0.25`).
  - **Thresholds and provenance (user-approved in `.pdca/open-issues-r5/rv1/criteria-approval.md`):**
    - `max_abs ≤ 3.5e-4`
    - `RMSE ≤ 1e-4`
    - `SDR ≥ 80 dB`

---

## 4. Contract Revision `r2` (`r 1 → 2`)

### Reason for revision (`ORD-4`)

In `r1`, `AC-4.3` (`commit + GitHub #4 closed`) was stated as a flat condition for `CHECK` acceptance while `.pdca/open-issues-r5/rv1/plan.md:9,11` and the collection constraints forbid committing or closing the GitHub issue until after an accepted `CHECK`. `r2` resolves this phase-ordering contradiction—matching the collection's resolution pattern for item `#8` (`F-8-acceptance-order`)—by preserving the verbatim `AC-4.3` requirement in full and explicitly partitioning it by phase order into a **pre-acceptance `CHECK` gate (`AC-4.3(1)`)** and a **post-acceptance `ACT` closure obligation (`AC-4.3(2)`)**. No user requirement or numerical threshold is added, weakened, or narrowed.

### Complete Revised Acceptance Criteria Contract (`r2`)

- **`AC-4.1` (unchanged verbatim from `r1`):**
  HTDemucs weights strict-validated, 4 stems named `drums`/`bass`/`other`/`vocals`, parity vs reference at `shifts=0` and equal segment/overlap with the `#4` thresholds (`max_abs ≤ 3.5e-4`, `RMSE ≤ 1e-4`, `SDR ≥ 80 dB`, per stem, `shifts=0`, `segment 7.8 s` / `overlap 0.25`, `audio-separator 0.48.0` `CPU/f32`, all samples, no trimming, no post-normalization) on every stem.
  - *Negative conditions:* Must reject missing, unexpected, wrong-shape, or non-`F32` tensors; must not drop or rename any of the 4 stems; must not exceed `max_abs` or `RMSE` bounds or fall below the `SDR` floor on any stem/channel; must not trim samples or apply post-normalization.
  - *Phase & evidence expectation:* Evaluated at `CHECK` from the candidate implementation and execution evidence produced in `DO` (CHECK selects its own verification means and evidence sources).

- **`AC-4.2` (unchanged verbatim from `r1`):**
  No regressions.
  - *Negative conditions:* Existing build, test suite, lints, and converter tests must not regress.
  - *Phase & evidence expectation:* Evaluated at `CHECK` from the candidate state and regression evidence produced in `DO`.

- **`AC-4.3` (`commit + GitHub #4 closed` — preserved in full, phase-ordered into `AC-4.3(1)` and `AC-4.3(2)`):**
  - **`AC-4.3(1)` — Pre-acceptance `CHECK` gate (delivery readiness & ordering compliance):**
    The item `#4` candidate delta on `main` is complete, self-contained, and isolated exclusively to item `#4`'s product files (with no out-of-scope product paths, no `.pdca/` artefacts staged for product commit, no premature git commit/push prior to `CHECK` acceptance, and no premature GitHub `#4` closure prior to `CHECK` acceptance), ready for a single atomic commit in `ACT`.
    - *Phase & evidence expectation:* `DO` leaves the working tree/snapshot in this clean, uncommitted, item-isolated state; `CHECK` verifies `AC-4.3(1)` as the sole `AC-4.3` gate required to issue an `accepted` verdict (`CHECK → ACT`).
  - **`AC-4.3(2)` — Post-acceptance `ACT` obligation (`commit + GitHub #4 closed`):**
    Immediately upon an `accepted` `CHECK` verdict (`CHECK → ACT`), execute one atomic git commit on `main` containing only item `#4`'s product files and close GitHub issue `#4` (or record an explicit external/environment blocker if GitHub write access fails). Item `#4`'s cycle cannot record terminal `done` in `ACT` until both the atomic commit on `main` and GitHub `#4` closure are completed and recorded.
    - *Phase & evidence expectation:* Executed and recorded in `ACT` after `CHECK` acceptance; not a pre-condition for `CHECK` to accept the candidate.

---

## 5. Outgoing Attempt Record & Revision Accounting

*(Reported as planner data for `pdca-executor` to book in `.pdca/status/issue-4-r5.md` and `.pdca/status/collection-open-issues-r5.md`.)*

- **Outgoing attempt (`r1` / `n=1/3`):**
  - `DO` (`r1`/`n1`) completed on `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685` + 8-path delta; fixed `GRN-4` and `CTR-4`; parity passed (`max_abs = 8.583e-6`, `RMSE = 1.774e-7`, min `SDR = 84.36 dB`); regression gates passed.
  - `CHECK 1` (`r1`/`n1`) verdict: **`not accepted` (`CHECK → PLAN`)** — `AC-4.1` `met`, `AC-4.2` `met`, `AC-4.3` `unmet` solely due to `ORD-4` (phase-ordering contradiction between `AC-4.3` and the pre-CHECK commit prohibition).
- **Revision transition and counters:**
  - Item contract revision: **`r: 1 → 2`** (collection revision stays `r1`).
  - Attempt counter reset on genuine contract revision: **`n: 1/3`** (the `CHECK → PLAN` return consumed no attempt; `r2` starts at attempt `n=1/3`).
  - Iteration counter: `N=1` (carried).
- **Defect history carried across to `r2`:**
  - `GRN-4`: GroupNorm variance pooling axis defect — **fixed in `r1`/`n1`** (closed/verified).
  - `CTR-4`: `conv_transpose1d` kernel > 1 unsupported in candle 0.11.1 — **fixed in `r1`/`n1`** via zero-insert + flipped-kernel `conv1d` (closed/verified).
  - `PAR-4`: Parity failure vs HTDemucs reference — **2 fixes applied (`GRN-4`, `CTR-4`), refuted on the `r1`/`n1` candidate** (`AC-4.1` `met`).
  - `ORD-4`: Planner-created phase-ordering contradiction on `AC-4.3` — **1 contract revision applied (`r1 → r2`)** by splitting `AC-4.3` into `AC-4.3(1)` (pre-acceptance `CHECK` gate) and `AC-4.3(2)` (post-acceptance `ACT` commit + GitHub `#4` closure).

---

## 6. Constraints, Assumptions, and Gaps

- **Constraints:**
  - Autocommit only after an accepted `CHECK`, one atomic commit per accepted item, on `main`, containing only item `#4`'s product files (`.pdca/` stays untracked/uncommitted).
  - Scope strictly limited to item `#4`; items `#1` and `#11` must not be touched or re-decided.
  - Preserve user-approved `#4` thresholds and comparability conditions verbatim.
- **Known resource and access constraints:**
  - Local CPU execution and reference fixture artefacts under `.pdca/open-issues-r5/rv1/item-4-demucs/` are available and already verified.
  - GitHub issue `#4` closure in `ACT` (`AC-4.3(2)`) requires network/auth access to GitHub at `ACT` time.
- **Assumptions:** None affecting the accepted outcome.
- **Statement gaps:**
  - Phase ordering of `AC-4.3` vs pre-CHECK commit prohibition (`ORD-4`) → **Closed by contract revision `r2` (`AC-4.3(1)` / `AC-4.3(2)`)**.

---

## 7. Minimal Solution, Next Authorized Step (`Plan-to-DO` Handoff), and Stop Conditions

- **Minimal solution:** Keep the verified 8-file item `#4` candidate at `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685` unchanged, validate in `DO` (`r2`/`n1`) that the candidate remains anchored and satisfies `AC-4.1`, `AC-4.2`, and `AC-4.3(1)` without premature commit or GitHub mutation, pass `CHECK` (`r2`/`n1`), and perform the atomic commit and GitHub `#4` closure (`AC-4.3(2)`) in `ACT`.
- **Next authorized transition:** **`PLAN → DO` (`r2`, `n=1/3`)**.
- **Plan-to-DO handoff (`r2`/`n1` scope):**
  - **Original task & criteria:** Carried by reference to `.pdca/status/issue-4-r5.md` §Original task and §4 of this `r2` record (`AC-4.1`, `AC-4.2`, `AC-4.3(1)`, `AC-4.3(2)`).
  - **What `DO` (`r2`/`n1`) must do:** Re-anchor on `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685` + the 8-path item `#4` product delta (`src/config.rs`, `src/model/mod.rs`, `src/separator.rs`, `src/model/demucs.rs`, `src/model/demucs_pipeline.rs`, `tests/demucs_parity.rs`, `tools/convert_demucs.py`, `tools/tests/test_convert_demucs.py`). Because `CHECK 1` already judged `AC-4.1` and `AC-4.2` `met` on this exact snapshot and `AC-4.3(1)` requires the candidate to remain uncommitted and isolated to item `#4` files prior to `CHECK`, **no new product code edits are required** unless semantic drift is found on re-anchor; `DO` confirms `AC-4.3(1)` readiness (clean uncommitted 8-path delta, no premature commit/push or GitHub `#4` mutation) and hands the candidate to `CHECK` (`r2`/`n1`).
  - **Prohibitions:** Do not commit, push, stage `.pdca/`, touch `#1` or `#11`, or close GitHub `#4` during `DO` or `CHECK`. Do not freeze or mandate a verification script or command for `CHECK`.
  - **What belongs purely to `ACT` closure (`AC-4.3(2)`):** Atomic git commit of item `#4`'s 8 product files on `main` and closing GitHub issue `#4` after `CHECK` (`r2`/`n1`) returns `accepted`.
- **Risks and stop conditions:**
  - **Stop / escalation conditions:**
    1. If semantic drift has altered `HEAD` or the 8-path candidate delta and cannot be reconciled within `r2` attempts (`n ≤ 3`), escalate via `ACT`.
    2. If `CHECK` (`r2`/`n1`) accepts `AC-4.1`, `AC-4.2`, and `AC-4.3(1)`, transition to `ACT`, execute `AC-4.3(2)` (atomic commit + GitHub `#4` closure), and stop item `#4` with terminal outcome `done`.
    3. If GitHub `#4` closure fails in `ACT` due to an external auth/network blocker after the atomic commit succeeds, record the concrete resource blocker in `ACT` rather than fabricating closure.

---

## 8. Unresolved Items from CHECK & Confidence

- **Anything left unresolved by `CHECK 1`?**
  - **For `PLAN`:** Nothing remains unresolved. `CHECK`'s notes on hash recomputation, ignored tests, GPU behavior, and arbitrary inputs beyond the approved fixture are explicitly noted by `CHECK` as non-blocking scope bounds of the approved contract (`AC-4.1` and `AC-4.2` were both judged **`met`** with high confidence).
  - **Live GitHub `#4` state:** `CHECK 1` noted that GitHub `#4` closure was unverified at `CHECK` time. Under `r2`, live GitHub closure is explicitly assigned to `ACT` (`AC-4.3(2)`) after `CHECK` acceptance, while `AC-4.3(1)` only requires that `DO` has not prematurely closed the issue before `CHECK`.

- **Confidence:** **High** — `AC-4.1` and `AC-4.2` are already verified as `met` on the frozen 8-file snapshot, and `r2` cleanly resolves `ORD-4` without altering any user requirement or alteration of any user requirement or numerical threshold.
````

## Approval record

`not applicable: autonomous mode` — the collection runs `autonomous`; no plan approval was requested
or is required. The user's criteria approval is recorded verbatim in §Original task and in
`.pdca/open-issues-r5/rv1/criteria-approval.md` (identity `open-issues-r5` / `N=1` / `r=1`).

## DO result

### Attempt `r2` / `n1` — complete, candidate unchanged and ready for `CHECK` (`2026-11-12T16:48Z`)

Scope executed: exactly the planner's `r2` §7 handoff — re-anchor and `AC-4.3(1)` readiness. **No
product byte was edited**, no gate was re-run (the snapshot had not drifted, so the `CHECK 1`
measurements stay bound to the same bytes), and nothing was staged, committed, pushed or sent to
GitHub.

- **Re-anchor, 8/8 (`AC-4.3(1)`):** `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685` on `main`;
  `git diff --cached --name-only` prints nothing (**index empty, nothing staged**); `git status
  --porcelain` shows exactly the item's delta — `M src/config.rs`, `M src/model/mod.rs`,
  `M src/separator.rs`, `?? src/model/demucs.rs`, `?? src/model/demucs_pipeline.rs`,
  `?? tests/demucs_parity.rs`, `?? tools/convert_demucs.py`, `?? tools/tests/test_convert_demucs.py`
  — plus the pre-existing untracked `.openidea/` and `.pdca/status/collection-open-issues-r3.md` and
  the two cycle status files. Every one of the 8 product paths hashes equal to
  `snapshot-sha256.txt` **and** to its `snapshot/` copy: **8 matched / 0 mismatched / 0 missing**
  (`logs/83-reanchor-r2n1.log`, exit **`0`**, recorded `2026-11-12T16:48:56Z`). Worst-case identities:
  `src/model/demucs.rs` 88,355 B sha256 `b1e04d0f…b40d07`; `src/model/demucs_pipeline.rs` 12,273 B
  `4079e974…f121`; `tests/demucs_parity.rs` 5,383 B `486d4320…b78d`; `tools/convert_demucs.py` 16,195 B
  `89fb606f…17de`; `tools/tests/test_convert_demucs.py` 7,920 B `25c2b549…12ca6`.
- **No drift, therefore no reconciliation and no re-measure:** no product path changed, none was added
  or removed from the task-owned set, and no `CHECK 1` measurement is invalidated.
- **Isolation of the delta (`AC-4.3(1)`):** the dirty product set is exactly item `#4`'s 8 files; no
  out-of-scope product path is dirty, so a single atomic commit can contain only these 8 paths. No
  `.pdca/` artefact is inside that set — the `.pdca/status/*` records are separate paths that ACT must
  keep out of the product commit; `.pdca/` is not staged here and would not be picked up by a commit
  that names the 8 paths.
- **Ordering compliance:** no `git add`, no staging, no commit, no push, no branch switch, no amend, no
  tag; **no GitHub `#4` write of any kind** and no GitHub read performed. No evidence pack was rebuilt
  and no criterion was changed.
- **Carried measurement (not re-run, unchanged bytes):** parity `logs/62` exit `0`, `1 passed; 0
  failed`, 8/8 stem × channel inside `max_abs ≤ 3.5e-4` / `RMSE ≤ 1e-4` / `SDR ≥ 80 dB` (worst
  `max_abs 8.583e-6`, worst `RMSE 1.774e-7`, lowest `SDR 84.36 dB`); gates `logs/63` (build), `logs/65`
  (clippy), `logs/66` (`cargo test`, 144 passed / 0 failed / 4 ignored), `logs/68` (converter `Ran 10
  tests` / `OK`), `logs/75` (strict-load rejection ×4) and `logs/80`/`logs/82` (on-disk stem naming).
- **State handed to CHECK:** `HEAD 8b03cfd`, branch `main`, index empty, clean uncommitted and
  item-isolated 8-path delta. Next allowed step: **`CHECK item #4 (r2/n1)`**; the atomic commit and the
  GitHub `#4` closure remain `ACT` work under `AC-4.3(2)` and are **not authorized** before an accepted
  `CHECK`.

`History (attempt r1/n1, kept as recorded): the line below was written while the repair was still in
progress and is superseded for the current attempt by the `r2`/`n1` block above.`

`in progress: attempt n=1/3 of contract r1 — mid-repair, the product tree does not compile yet; no
candidate, no verdict, no acceptance.`

- **Fresh baseline re-established on the unchanged tree** (before any edit of this attempt): converter
  tests `Ran 10 tests` / `OK`, exit **`0`** (`logs/50-converter-tests.log`); `cargo build --release`
  exit **`0`** (`logs/51-cargo-build.log`); parity test exit **`101`**, `0 passed; 1 failed`, per-stem
  `max_abs ≈ 2.6e3…4.0e3`, `RMSE ≈ 1.5e2…3.7e2`, `SDR ≈ −73…−123 dB`
  (`logs/52-parity-baseline-trace.log`).
- **`GRN-4` fixed.** Root cause confirmed by a formula cross-check: the reference `GroupNorm` divides by
  the variance pooled over the **whole group** (all channels of the group × trailing dims), the port
  pooled only over the trailing dims (`logs/54-groupnorm-check.log`: port-formula vs port dump
  `max_abs 1.32e-4`, torch formula vs port dump `6.94e1`, per-channel/group scale factors
  `0.507…1.238`). New unit test `model::demucs::tests::group_norm_pools_variance_over_the_whole_group`
  red exit **`101`** (`logs/55-groupnorm-test-red.log`, `got -2.0701897 expected -1.4638476`) → green
  exit **`0`**, `3 passed` (`logs/56-groupnorm-test-green.log`).
- **Bisection after `GRN-4`:** the boundary moved from `encoder.0.dconv.layers.0-norm1`
  (`max_abs 6.94e1`, `corr 0.897` — `logs/53-bisect-encoder0.log`, probe dumps in `probe-r5/`) down to
  the decoder: parity re-measured at `max_abs 6.2e-1…1.09`, `RMSE 1.3e-2…5.3e-2`, `SDR −41.7…+2.7 dB`,
  exit **`101`** (`logs/57-parity-after-groupnorm-fix.log`), and the stage replay puts the first
  decoder divergence at `decoder-0` (`max_abs 8.14`, `corr 0.031`, input peak 0.806 —
  `logs/58-bisect-decoder.log`, dumps in `probe-r5b/`).
- **`CTR-4` open.** `Tensor::conv_transpose1d` (candle 0.11.1) does not reproduce
  `nn.ConvTranspose2d(kernel (k,1), stride (s,1))` for `k > 1`, so `FreqConvTranspose::forward` was
  rewritten as zero insertion plus a flipped-kernel `conv1d`. The new unit test
  `freq_transposed_conv_matches_the_definition` is **still red** — exit **`101`**, `4 passed; 1 failed`
  (`logs/59-decoder-primitive-tests.log`: `conv_tr[0][0][1]: got 1 expected 0.25`;
  `logs/60-convtr-fix-tests.log`: `conv_tr[0][1][0]: got 0 expected -0.5`).
- **Unverified edit, flagged as unverified:** the `one_hot_stride()` helper as it stands on disk
  (`src/model/demucs.rs:350-358`) contains a leftover `static CACHE: std::sync::OnceLock<Tensor>` stub
  and was **applied without compiling or testing** — no exit code exists for it. The tree's
  compile status is therefore unknown from evidence and is to be established by the next run, not
  assumed.
- **Product bytes touched so far** (`src/model/demucs.rs`, untracked in Git): `GroupNorm` variance over
  the whole group; `FreqConvTranspose::forward` + `one_hot_stride()` rewrite; three new unit tests
  (`group_norm_pools_variance_over_the_whole_group`,
  `freq_transposed_conv_matches_the_definition`, `square_rewrite_conv_matches_the_definition`). None of
  this is claimed as compiled, verified or ready.

### Fixes applied after the step-A recording (measured)

- **`CTR-4` fixed.** The leftover `static CACHE: OnceLock<Tensor>` stub and the whole `one_hot_stride()`
  helper were deleted; the zero insertion is now `y.unsqueeze(3)` + `pad_with_zeros(D::Minus1, 0, s-1)`
  (each sample `[x, 0, …, 0]`), reshaped to `f*s` and narrowed to `(f-1)*s + 1`, then convolved with the
  flipped kernel (`k-1` padding). `cargo test --release --lib model::demucs::tests` exit **`0`**,
  `5 passed; 0 failed; 0 ignored` (`logs/61-ctr4-lib-tests.log`); the same test was red at exit **`101`**
  before the fix (`logs/59`, `logs/60`). The tree compiles.
- **`PAR-4` re-measured against the frozen reference** (`audio-separator 0.48.0`, CPU/f32, `shifts=0`,
  segment 7.8 s = 343,980 samples, `overlap=0.25`; fixture sha256
  `583a41eef4f9e09d3f615cf5bbebd1e9f84a5b99130de12a55207f698f37f0d4`, 441,000 frames, 10 s; stems
  `Bass 6cf454b2…02eacffc`, `Drums 4a8c9408…e5abc09`, `Other e00dcea6…9bf7901a87`,
  `Vocals b7663f10…1b4f06709`): `cargo test --release --test demucs_parity -- --nocapture` exit
  **`0`**, **`1 passed; 0 failed; 0 ignored`** (the test really executed — it printed all 8 comparison
  lines; it did not take the `skipping: … is not available` branch), log
  `logs/62-parity-after-ctr4.log`:

  | stem | ch | `max_abs` | `RMSE` | `SDR` |
  |---|---|---|---|---|
  | drums | 0 | `3.949e-7` | `9.037e-9` | `93.90 dB` |
  | drums | 1 | `3.036e-7` | `8.759e-9` | `94.45 dB` |
  | bass | 0 | `3.019e-7` | `6.484e-9` | `86.07 dB` |
  | bass | 1 | `2.778e-7` | `6.306e-9` | `84.36 dB` |
  | other | 0 | `6.508e-6` | `1.674e-7` | `112.75 dB` |
  | other | 1 | `5.299e-6` | `1.381e-7` | `112.41 dB` |
  | vocals | 0 | `8.583e-6` | `1.774e-7` | `89.51 dB` |
  | vocals | 1 | `6.534e-6` | `1.414e-7` | `89.20 dB` |

  Thresholds are unchanged (`max_abs ≤ 3.5e-4`, `RMSE ≤ 1e-4`, `SDR ≥ 80 dB`); the fixture, reference
  stems and comparison protocol were not touched.
- **Gates run on the fixed bytes so far:** `cargo build` exit **`0`** (`logs/63-gate-build.log`);
  `cargo clippy --all-targets` exit **`0`**, warnings only, none in the edited region
  (`logs/65-gate-clippy.log`). **Full `cargo test`: in progress, unmeasured as a whole** — a first
  attempt hit the dispatch's command timeout after `vr_stage_waves` (per-target results up to that point
  all `ok`), and the clean re-run is `logs/66-gate-test-full.log` (still running when this line was
  written); the target-level counts are therefore **not yet recorded as a gate result**. Converter
  tests on the fixed bytes: **not yet re-run** in this step (`logs/50` is the pre-fix baseline, exit
  `0`, `Ran 10 tests` `OK`).

### Gates, complete (`2026-11-12T16:42Z`, all on the fixed bytes)

| gate | command | exit | counts | log |
|---|---|---|---|---|
| build | `cargo build` | **`0`** | 22 warnings (pre-existing) | `logs/63-gate-build.log` |
| tests | `cargo test` | **`0`** | **144 passed / 0 failed / 4 ignored** over 25 targets (lib 42, bs_roformer 8+1i, core_tests 22, demucs_parity 1, ensemble 22+1i, fp16_decision 0+2i, gpu_smoke 1, library_api 12, s1_spikes 11, streaming_chunked 8, vr_arch 1, vr_pipeline_parity 1, vr_resample 1, vr_roundtrip 1, vr_samplerate 1, vr_stage_band_specs 1, vr_stage_bands 1, vr_stage_combined 1, vr_stage_mag 1, vr_stage_mask 1, vr_stage_waves 1, vr_stage_yspec 1, vr_stft_roundtrip 1, whisper_tests 3, doctests 1) | `logs/66-gate-test-full.log` |
| lint | `cargo clippy --all-targets` | **`0`** | warnings only, none in the `FreqConvTranspose`/`GroupNorm` region | `logs/65-gate-clippy.log` |
| converter | `python -m unittest tools.tests.test_convert_demucs` (mdx-ref-venv, Python 3.12.10) | **`0`** | `Ran 10 tests` / `OK` | `logs/67-gate-converter-tests.log` |
| parity | `cargo test --release --test demucs_parity -- --nocapture` | **`0`** | `1 passed; 0 failed; 0 ignored`, 8/8 stem × channel inside threshold | `logs/62-parity-after-ctr4.log` |

**Candidate numerically ready** (DO-side statement about the candidate; not a verdict): the parity
measurement meets `max_abs ≤ 3.5e-4` / `RMSE ≤ 1e-4` / `SDR ≥ 80 dB` on every stem × channel and no
other gate regressed. **Outstanding, not done here:** the evidence pack, a pre-stage re-anchor and the
atomic commit with the GitHub `#4` closure — next round / CHECK scope.

- Pre-existing workspace work under item `#4` (recorded before any edit, see §Defect history and
  §Progress log): `HEAD 8b03cfd` on `main`, nothing staged; `M src/config.rs`, `M src/model/mod.rs`,
  `M src/separator.rs`, untracked `src/model/demucs.rs`, `src/model/demucs_pipeline.rs`,
  `tests/demucs_parity.rs`, `tools/convert_demucs.py`, `tools/tests/test_convert_demucs.py`; plus
  pre-existing `?? .openidea/` and `?? .pdca/status/collection-open-issues-r3.md`. These are
  pre-existing user/workspace work: inspected, not overwritten, not reverted, not reformatted, and no
  `logs/`, `probe*/`, `ref/` or `out/` artefact was deleted.
- Known measured state at the start of this attempt (pre-existing diagnostics, to be re-established by
  fresh runs, not treated as durable evidence): the recorded parity measurement **fails**
  (`logs/19-parity-trace.log:191-198`, `logs/18-parity-run-7.log:189-196`: per-stem `max_abs ≈
  2.6e3…4.0e3`, `RMSE ≈ 1.5e2…3.7e2`, `SDR ≈ −73…−123 dB`), with the first divergence localised inside
  `encoder.0` (`progress.log:12-13`).

### Attempt `n=1` complete — candidate ready for CHECK (`2026-11-12T16:15Z`, continuation 3)

No product byte was edited by this continuation: every number below was measured on the same bytes
that `logs/62` measured (`src/model/demucs.rs` 88,355 B sha256 `b1e04d0f…b40d07`, mtime `20:19:10`
local; the release parity binary `logs/62` ran is `demucs_parity-bf7640aa…exe`, 25,666,560 B, sha256
`c8759b66…561027`, built `20:19:33` — after the last source edit). The "full `cargo test` unfinished /
converter tests not re-run" clauses above are superseded here and kept as history.

- **Full-suite gate (`AC-4.2`).** `cargo test` (debug, whole workspace): `cargo test` (debug, whole workspace) exit **`0`** — **144 passed / 0 failed / 4 ignored** over 24 targets + doc-tests (lib 42; bs_roformer 8+1 ignored; core_tests 22; demucs_parity 1; ensemble 22+1 ignored; fp16_decision 0+2 ignored; gpu_smoke 1; library_api 12; s1_spikes 11; streaming_chunked 8; vr_arch 1; vr_pipeline_parity 1; vr_resample 1; vr_roundtrip 1; vr_samplerate 1; vr_stage_band_specs 1; vr_stage_bands 1; vr_stage_combined 1; vr_stage_mag 1; vr_stage_mask 1; vr_stage_waves 1; vr_stage_yspec 1; vr_stft_roundtrip 1; whisper_tests 3; doc-tests 1), 41m54s wall, launcher `EXIT=0` (`logs/66-gate-test-full.log`).
- **Converter tests on the fixed bytes.** `& $py -m unittest -v tools.tests.test_convert_demucs`
  exit **`0`**, `Ran 10 tests in 9.090s`, `OK` (10/10, none skipped)
  (`logs/68-converter-tests-fixed-bytes.log`).
- **Pinned checkpoint, re-verified by the converter itself.** `& $py tools/convert_demucs.py
  models/955717e8-8726e21a.th --check-only` exit **`0`**: source sha256
  `8726e21a993978c7ba086d3872e7608d7d5bfca646ca4aca459ffda844faa8b4`, size `84,141,911`,
  `tensor_count 533`, every tensor `F32`, key-set sha256 `33ddc01d…5fbe`, `weight_norm_resolved []`
  (`logs/69-converter-check-only.log`).
- **`AC-4.1` strict-load rejection, measured on the real artifact.** Four mutants of the converted
  artifact were rebuilt canonically (header order == offset order, contiguous, full coverage) so the
  container stays valid and only one schema property differs; each was loaded through
  `DemucsSeparator::load` → `HTDemucs::validate_state` and each was rejected, `cargo test` exit
  **`101`**, `0 passed / 1 failed` (`logs/75-strict-load-probe-v3.log`):

  | mutant | rejection (verbatim) |
  |---|---|
  | missing | `1 missing, 0 unexpected, 0 mis-shaped … first missing: Some("decoder.3.conv_tr.weight")` |
  | unexpected | `0 missing, 1 unexpected … first unexpected: Some("strict.load.probe")` |
  | mis-shaped | `… 1 mis-shaped … Some("decoder.3.conv_tr.weight: [6144] != [48, 16, 8, 1]")` |
  | non-f32 | `… 1 mis-shaped … Some("decoder.3.conv_tr.weight: dtype I32 != F32")` |

  Two earlier probe rounds are kept as history: `logs/71` (header-only mutants — rejected by the
  safetensors reader before the validator, so not accepted as evidence) and `logs/73` (canonical
  mutants, but a shallow-copy bug in the probe generator made the `non_f32` row report the shape
  mismatch). Generator: `scripts/make_strict_load_mutants.py`; accepted round: `logs/74` + `logs/75`.
- **`AC-4.1` per-stem naming, end to end on disk.** An evidence-only harness
  (`scripts/stem_names_e2e.rs`, compiled with `rustc` against `target/debug/libcandle_roformer.rlib`;
  no product file and no crate target added) loaded the artifact, separated the frozen fixture and
  wrote every returned stem under the pipeline's own name: `names=["drums", "bass", "other",
  "vocals"] len=4`, four files in `out/stems-candidate/`, `3,528,044 B` each, 441,000 frames × 2 ch @
  44,100 Hz, all finite (`logs/80`, exit **`0`**). Independent read-back
  (`scripts/check_stem_names.py`, manual RIFF parse — stdlib `wave` rejects IEEE-float format tag 3,
  that first attempt is kept in the log): exactly `{bass.wav, drums.wav, other.wav, vocals.wav}`,
  `f32`, `44100 Hz`, `2 ch`, `441000` frames, all samples finite, non-silent; peaks `drums
  0.043429870 / bass 0.007832416 / other 0.999377787 / vocals 0.208965346` (`logs/82`, exit **`0`**).
- **Evidence pack** in `.pdca/open-issues-r5/rv1/item-4-demucs/`: `manifest.json` (criterion →
  evidence → numbers, identities, protocol, gates, runs), `index.md`, `snapshot/` +
  `snapshot-sha256.txt` (8/8 worktree == snapshot) + `snapshot.diff` (148,596 B, 8 `diff --git`
  headers = exactly the item delta), `metrics/parity.json` + `.csv` (the 8 measured rows),
  `commands.txt` (every command, failures and re-runs included) and `lint.txt`.
- **`PAR-4` closed on the measured candidate**: 8/8 stem × channel rows inside `max_abs ≤ 3.5e-4`,
  `RMSE ≤ 1e-4`, `SDR ≥ 80 dB`; worst `max_abs 8.583e-6`, worst `RMSE 1.774e-7`, lowest `SDR
  84.36 dB` (`logs/62`, `metrics/parity.json`) — **a measurement, not a verdict and not an
  acceptance claim**.
- **State:** `HEAD 8b03cfd`, branch `main`, index empty, nothing staged, no commit, no push, no
  branch switch, no amend, no tag, no GitHub read or write. Next allowed step:
  **`CHECK item #4 (r1/n1)`**.

### Evidence round `r2` / `n1` — `GH-4-PRE` GitHub `#4` observation (`2026-11-12T16:57Z`)

Evidence-only re-gather routed by `CHECK 2 → DO`. **No new attempt** (`n` stays `1/3`), no contract
change, **no product byte edited**, no gate re-run, nothing staged, no commit, no push and **no GitHub
write of any kind**. The routed finding is recorded as supplied by the dispatch brief:

> **GH-4-PRE — major, unverified; AC-4.3(1):** отсутствует независимое наблюдение отсутствия
> преждевременного закрытия GitHub `#4`. Это пробел доказательств, **не установленное нарушение**.

- **One read-only observation** (`scripts/observe_issue4.py`, unauthenticated; no token read, sent or
  stored), repo identity taken from `git remote -v` (`origin git@github.com:AlexeyShirshov/candle-roformer.git`
  → `AlexeyShirshov/candle-roformer`), observation clock `2026-11-12T16:57:05Z` (UTC):
  - `GET https://api.github.com/repos/AlexeyShirshov/candle-roformer/issues/4` → **`HTTP 200`**, 4,635 B,
    sha256 `c9734a32…f18117`; `number 4`, `html_url
    https://github.com/AlexeyShirshov/candle-roformer/issues/4`, title `feat: Demucs (4-stem and 6-stem)
    architecture`, `state` **`open`**, `state_reason` `null`, `closed_at` `null`, `created_at`
    `2026-11-08T06:43:20Z`, labels `[]`, comment count `0`.
  - No product drift in the round: the 8 item product paths re-hashed against `snapshot-sha256.txt` at
    `2026-11-12T17:01:42Z` → **8 matched / 0 mismatched / 0 missing** (`logs/85-no-product-drift-gh4pre.log`,
    exit **`0`**).
  - `GET https://api.github.com/repos/AlexeyShirshov/candle-roformer/issues/4/timeline?per_page=100` →
    **`HTTP 200`**, 1,329 B, sha256 `9087d84f…93848f`; `1` event, a `milestoned` (`1.0`) at
    `2026-11-08T06:43:20Z` by `AlexeyShirshov`; **`0` `closed` and `0` `reopened` events**; no `Link`
    header / no `rel="next"` page announced, so the single page **is** the whole history and no further
    page was requested.
  - Raw bodies `github/issue-4__2026-11-12T16-57-05Z.json` and
    `github/issue-4-timeline__2026-11-12T16-57-05Z.json`, summary `github/issue-4-observation.json`,
    producer log `logs/84-github-issue-4-observation.log`; run exit **`0`**.
- **Facts only, no verdict:** the issue was **`open`** at observation time and its history contains **no
  closure at all**. There is therefore no premature closure of GitHub `#4` by this cycle or by anyone
  else, and none can predate this cycle's `CHECK 1` (`2026-11-12T16:48Z`) or `CHECK 2`. Nothing closed,
  reopened or relabelled anything: the observation is read-only and the single `milestoned` event
  predates the cycle by four days.
- **Evidence pack updated by reference, evidence only:** root `github_issue` block in `manifest.json`
  (identity, raw paths + sha256, observation UTC, timeline counts, `rel="next"` status) and the
  `AC-4.3(1)` / `AC-4.3(2)` sub-records under `criterion_evidence.AC-4.3`; a `## GitHub #4 observation`
  section in `index.md`; rows 7 in `commands.txt`. The pack states no criterion as met — that decision
  belongs to `pdca-check`.
- **State handed to CHECK:** `HEAD 8b03cfd`, branch `main`, index empty, the unchanged 8-path product
  delta, **no GitHub write**. Next allowed step: **`CHECK item #4 (r2/n1)` — CHECK 3**.

## CHECK record

### CHECK 1 — `r1` / `n1`

`Recorded as returned. The complete original CHECK report was not supplied to this write as a separate
persisted artefact; the elements below are the ones the dispatch brief and the planner's `r2` return
(§5, §8) carry as CHECK 1's own result. Nothing is reconstructed beyond them, and the missing full
original is recorded as missing rather than filled in.`

- **Verdict:** **`not accepted` → `PLAN`** (loop-back to the planner, not to `DO`).
- **Per-criterion matrix:**

  | criterion | result |
  |---|---|
  | `AC-4.1` (strict-validated weights, 4 stems, parity at the approved thresholds) | **`met`** |
  | `AC-4.2` (no regressions) | **`met`** |
  | `AC-4.3` (commit + GitHub `#4` closed) | **`unmet`** |

- **Finding `ORD-4`** (`AC-4.3`, **blocking**, **confirmed**): acceptance ordering — `AC-4.3` was stated
  as a flat `CHECK`-acceptance condition while the collection constraint and
  `.pdca/open-issues-r5/rv1/plan.md:9,11` forbid committing or closing GitHub `#4` before an accepted
  `CHECK`. The contract, not the candidate, is the cause; no product change was requested.
- **Notes recorded by CHECK 1 as non-blocking scope bounds** (carried by the planner's `r2` §8): hash
  recomputation, ignored tests, GPU behaviour, and arbitrary inputs beyond the approved fixture are
  outside the approved contract's bounds; the live GitHub `#4` state was **unverified** at `CHECK` time.
- **Transition:** `CHECK → PLAN`, which consumes **no new attempt**; the outgoing attempt `r1`/`n1` is
  booked first and then `Replanned: CHECK→PLAN acceptance-ordering contract (r 1→2)` — both in
  §Progress log and §Revision history.
- **Source:** dispatch brief (CHECK 1 summary: verdict, matrix, `ORD-4`) and `.pdca/status/issue-4-r5.md`
  §PLAN record `### r2` §5/§8 (the planner's quotation of the same CHECK result).

### CHECK 2 - `r2` / `n1`

`not recorded as a separate report: CHECK 2 routed finding GH-4-PRE (major, unverified, AC-4.3(1)) back to
DO for evidence only, and its complete original report was not supplied to this write as a persisted
artefact. The routing, the finding text and the evidence round it caused are recorded in §Progress log
(rows stamped 2026-11-12T16:57:05Z / 16:59Z) and in the §DO result block of the same date; nothing is
reconstructed beyond them.`

### CHECK 3 - `r2` / `n1`

`Recorded as returned by the ACT dispatch brief. The complete original CHECK 3 report was not supplied to
this write as a separate persisted artefact, so only the verdict it carries is recorded; a per-criterion
matrix for CHECK 3 was not supplied and is recorded as absent, never reconstructed.`

- **Verdict:** **`accepted → ACT`** (CHECK's verdict; this record issues no verdict of its own).
- **Accepted snapshot graded:** `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685` + the 8-path product
  delta — modified `src/config.rs`, `src/model/mod.rs`, `src/separator.rs`; untracked/new
  `src/model/demucs.rs`, `src/model/demucs_pipeline.rs`, `tests/demucs_parity.rs`,
  `tools/convert_demucs.py`, `tools/tests/test_convert_demucs.py` — frozen as `item-4-demucs/snapshot/` +
  `snapshot-sha256.txt` (8/8).
- **Transition:** `CHECK → ACT`. Only the post-acceptance closure of `AC-4.3(2)` (atomic commit +
  GitHub `#4` closure) moves to ACT; no attempt is consumed and the counters stay `N=1 · r=2 · n=1/3`.
- **Source:** ACT dispatch brief (verdict line); the full CHECK 3 report is absent, as stated above.

## ACT record

`CHECK 3` (`r2`/`n1`) returned **`accepted → ACT`**; ACT performs only the post-acceptance closure and
issues no new verdict. The record is **`completed`** (`2026-11-12T17:13Z`): the mandatory `pdca-check`
`retrospective` report is persisted in §Lessons learned, so the finalization is closed with outcome
`accepted`.

- **Outcome determined by the ending phase:** `accepted` (`CHECK 3`, `r2`/`n1`). The criteria outcome is
  CHECK's; this record does not re-derive, extend or weaken it.
- **Completed by ACT — `AC-4.3(2)` commit component.** Pre-stage re-anchor first: worktree ==
  `snapshot/` copy == `snapshot-sha256.txt`, **8/8 matched, `0` mismatched, `0` missing**, verifier exit
  **`0`**, on `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685`, branch `main`, index empty
  (`logs/86-act-reanchor-r2n1.log`). Then exactly the 8 item paths were staged (`git add -- <8 paths>`,
  never `git add -A`) and **one atomic commit** created:
  `d042c2628edb52deab42118e18ac69974e5ad75d` (`d042c26`), parent `8b03cfd`, **8 files changed,
  `+3767/−4`**, subject `#4: HTDemucs 4-stem separation with audio-separator 0.48.0 parity`, footer
  `Refs #4` (no `Closes`/`Fixes`/auto-close keyword), author/committer `Alexey Shirshov
  <alex.shirshov@vk.com>` supplied per command with `git -c` (**no `git config` write**); no `--amend`,
  no push, no branch switch, no tag, no rebase (`logs/87-act-commit-verify.log`).
- **Verified after the commit (`logs/87`):** `HEAD = d042c26…`; `git show --name-only HEAD` is exactly
  the 8 item paths (`file_count=8`); the commit contains **no** `.pdca/**`, no `.openidea/**`, no weight,
  audio/fixture, log or status path; `git diff --name-only HEAD -- <8 paths>` **empty**;
  `git diff --cached --name-only` **empty**; the worktree still hashes to its snapshot row **8/8** after
  the commit (no product byte moved).
- **Unfinished / external residual:** the **GitHub `#4` closure is handed to the calling driver** — no
  PDCA phase performs a GitHub write in this collection, so the closure component of `AC-4.3(2)` is an
  external residual: **not performed here** and **never recorded as "issue closed"**.
- **Retrospective recorded — record closed (`2026-11-12T17:13Z`):** the mandatory `pdca-check`
  `retrospective` report for cycle `issue-4-r5` (`N=1`, `r=2`, `n=1`) was routed by the calling driver
  in `mode: retrospective` and is persisted **verbatim** in §Lessons learned (completion signal
  `retrospective_complete`) with the dispatch's own recording notes kept outside the quoted block. The
  previous `pending` content of that slot and the explicit closure blocker it carried are superseded
  (kept as history, nothing deleted). The retrospective adds no verdict, criteria result, counter,
  attempt or new verification and reopens nothing.
- **Completion:** `phase: ACT · record: completed · outcome: accepted` (the `accepted` outcome is
  `CHECK 3`'s, not a new verdict); item `#4` is `done` / `finalized`, `plan_revision 2`, attempts `1`.
  Nothing remains inside item `#4` except the external residual above.
- **Handoff / next authorized action:** the calling driver performs the **GitHub `#4` closure** with the
  accepted CHECK result and commit `d042c26`; inside the collection the next step is the **collection
  integration `CHECK`** (all four items are terminal: `#2` `59fb1fb`, `#8` `a38bb7f`, `#3` `3130f55`,
  `#4` `d042c26`; one group, no merge). No further `DO` work is authorized for this item.
- **Counters:** `N=1 · r=2 · n=1/3`; item `#4` attempts `1` — unchanged by ACT (the closure and the
  retrospective consume no attempt and add no verification or criterion).

**GitHub `#4` closure — performed (`2026-11-12T17:26Z`, evidence-only round; the bullets above that hand the
closure to the calling driver are superseded here in that clause only, kept as history and not rewritten).**
Authorization: item `#4` was accepted by the independent `CHECK 3`; the standing user instruction is
"после каждой выполненной задачи - делай комит и закрывай задачу"; the frozen contract `r2` (`AC-4.3(2)`)
assigns the closure to ACT; `#2`/`#8`/`#3` were closed the same way. No product change, no commit, no new
attempt.

- **Command and result:** `gh issue close 4 --repo AlexeyShirshov/candle-roformer --reason completed
  --comment <text of github/issue-4-close-comment.md>` with the authenticated GitHub CLI at
  `C:\Program Files\GitHub CLI\gh.exe` — **exit `0`**, output
  `✓ Closed issue AlexeyShirshov/candle-roformer#4 (feat: Demucs (4-stem and 6-stem) architecture)`. No
  credential value was read, printed, copied or recorded (`logs/89-github-issue-4-close.log`).
- **Comment:** id **`6830882059`** (`2026-11-12T17:26:05Z`), body sha256
  `da4be8f5b415484c3902a5288cead349013982be7db45c71e257ca7324eb4167`; the stored body compares
  **byte-identical** (after trailing-newline trim) with the text sent, so no encoding loss
  (`github/issue-4-close-comment.md`).
- **Read-only verification (unauthenticated, no token, `HTTP 200`):** `state=closed`,
  `state_reason=completed`, `closed_at=2026-11-12T17:26:06Z`, `comment_count=1`; timeline **1 `closed` / 0
  `reopened`** (`closed` event `2026-11-12T17:26:06Z`). Raw bodies + summary in
  `github/issue-4-observation-closed.json`, `github/issue-4__2026-11-12T17-26-24Z.json`,
  `github/issue-4-timeline__2026-11-12T17-26-24Z.json`; log `logs/90-github-issue-4-observation-closed.log`.
- **Read-lag note:** the first post-write issue read at `17:26:13Z` returned a stale `state=open` body
  (byte-identical to the pre-write read) while the comments endpoint already returned the closure comment;
  a `Cache-Control: no-cache` re-read at `17:26:24Z` returned the closed state. Both reads are persisted;
  the stale one is recorded as an observation artefact, not used as the result.
- **`AC-4.3(2)` is performed in both components:** commit
  `d042c2628edb52deab42118e18ac69974e5ad75d` **and** GitHub `#4` closed `COMPLETED` with comment
  `6830882059`. Item `#4` therefore stays **`done` / `finalized`** (outcome `accepted` — `CHECK 3`'s, not
  re-derived) with **no external residual left**; the closure consumes no attempt and adds no verification
  or criterion. Nothing was pushed, amended, staged or committed by this round.

## Lessons learned

### Ретроспектива `issue-4-r5`

1. **Категория: численная диагностика и тестирование.**  
   Сработали поиск первого расходящегося слоя и локальные red→green тесты: исправлены `GRN-4` и `CTR-4`, после чего parity уложилась в допуски на 8/8 парах stem × channel. Первого исправления было недостаточно — повторное измерение выявило расхождение декодера.  
   **На будущее:** проверять семантику примитивов небольшими тестами по определению; после каждого исправления повторять сквозное сравнение, не подменяя его успешным unit-тестом.  
   **Основание:** `.pdca/status/issue-4-r5.md` §DO result; `.pdca/open-issues-r5/rv1/item-4-demucs/logs/53`–`62`.

2. **Категория: согласованность фазового контракта.**  
   Не сработал исходный порядок требований: `AC-4.3` требовал коммит до решения CHECK, хотя ограничения разрешали его только после. Ревизия `r2` разделила предварительную готовность и обязательства ACT без изменения численных допусков.  
   **На будущее:** уже в PLAN явно указывать фазу исполнения каждого требования и проверять отсутствие циклических зависимостей между решением CHECK и действиями ACT.  
   **Основание:** `.pdca/status/issue-4-r5.md` §PLAN record `r1`, `r2`; §CHECK record, CHECK 1 (`ORD-4`).

3. **Категория: доказательства и передача результатов.**  
   Снимок и повторная привязка 8/8 файлов позволили сохранить результаты измерений при раундах без изменения продукта и выполнить изолированный коммит `d042c26`. Однако наблюдение GitHub потребовало отдельного раунда, а полные исходные отчёты CHECK в статусе отмечены отсутствующими.  
   **На будущее:** заранее предусматривать сохраняемые наблюдения внешнего состояния и передавать полные исходные отчёты фаз вместе с результатом; различать доказательственный пробел и дефект продукта.  
   **Основание:** тот же статус, §DO result — evidence round, §CHECK record, §ACT record; evidence root: `github/issue-4-observation.json`, `logs/83`–`87`.

#### Ограничения и неопределённость

- Анализ использует существующие записи и предоставленные сведения; новых запусков, исследований и пересчёта хешей не было.
- Полные исходные отчёты CHECK 1–3 в прочитанном статусе отсутствуют; предоставленные в dispatch сведения использованы с этим ограничением.
- Численные результаты относятся к замороженной fixture, CPU/f32, а не к произвольным входам или GPU.
- Закрытие GitHub `#4` остаётся внешним остаточным обязательством; подтверждения его выполнения в предоставленных материалах нет.
- Рекомендации выше — переносимые выводы, а не доказательство того, что они гарантированно устранят повторение проблем.

**Сигнал завершения ретроспективы:** `retrospective_complete`

- **Recording notes of this ACT dispatch (`2026-11-12T17:13Z`), outside the quoted report.** The block
  above is the raw `pdca-check` `retrospective` report for cycle `issue-4-r5` (`N=1`, `r=2`, `n=1`),
  routed by the calling driver in `mode: retrospective` and recorded here as delivered: wording,
  punctuation, numbering, bullets, hard line breaks and the closing signal unchanged. The only
  mechanical change is that its two headings sit one level deeper (`##`→`###`, `###`→`####`) so the
  block nests inside this section instead of forming a competing top-level section
  (`references/act.md` §Status file: headings stay stable). No word, line break or character of the
  text was altered, translated or summarised; nothing was re-run, re-measured or re-verified.
- **Superseded, kept as history:** the previous content of this slot —
  `` `pending: retrospective not yet conducted` `` with its explicit closure blocker (`pdca-check`
  report not routed) — is superseded by the recorded report above. It is not rewritten or deleted.
- **Originating outcome preserved unchanged:** `CHECK 3` (`r2`/`n1`) `accepted`; contract `r2`
  attempt `n=1/3`; `AC-4.3(2)` commit `d042c26` performed. The retrospective adds no verdict,
  criteria result, counter, attempt or new verification, and reopens nothing.
- **Residual (unchanged, external):** the **GitHub `#4` closure** remains an external residual handed
  to the calling driver — not performed by any PDCA phase and never recorded as "issue closed".

## Revision history

- `2026-11-12T16:48Z` | `r1 → r2` | acceptance-ordering contract revision from `CHECK → PLAN` — finding
  `ORD-4` (blocking, confirmed: `AC-4.3` stated as a flat `CHECK`-acceptance condition against the
  collection's pre-`CHECK` commit prohibition); outgoing attempt `r1`/`n1` (DO complete, parity
  passing, `CHECK 1` `not accepted → PLAN`) booked first and consuming no new attempt; `AC-4.1`/`AC-4.2`
  verbatim, `AC-4.3` partitioned into `AC-4.3(1)` (pre-acceptance `CHECK` gate) and `AC-4.3(2)`
  (post-acceptance `ACT` commit + GitHub `#4` closure); item-level only — the collection plan stays `r1`.

## Defect history

| key | revisions/attempts observed | fixes applied | evidence | last recurrence/escalation |
|---|---|---|---|---|
| `PAR-4` (four-stem parity vs the reference) | `r1`/`n1` (observed on the pre-existing candidate before this attempt's own runs) | 0 | `.pdca/open-issues-r5/rv1/item-4-demucs/logs/19-parity-trace.log:191-198`, `logs/18-parity-run-7.log:189-196`, `logs/26-parity-trace7.log:189-198`, `progress.log:12-13` | first divergence localised inside `encoder.0` (`reference peak 5.3438 / std 0.2328` vs `ours peak 153.9751 / std 1.0126`, per-channel corr 0.33–0.55); no fix applied yet, no escalation |
| `GRN-4` (`GroupNorm` pooled the variance over trailing dims only, not over the whole group) | `r1`/`n1` | 1 (`GroupNorm::forward` + unit test) | red `logs/53-bisect-encoder0.log` (`encoder.0.dconv.layers.0-norm1 max_abs 6.94e1`, `corr 0.897`), `logs/54-groupnorm-check.log` (torch formula vs port dump `6.94e1`; per-channel/group scale `0.507…1.238`), red `logs/55-groupnorm-test-red.log` exit `101` → green `logs/56-groupnorm-test-green.log` exit `0` | boundary moved out of the encoder; not re-opened by any later measure; no escalation |
| `CTR-4` (`Tensor::conv_transpose1d` is not the `ConvTranspose` definition for kernel > 1) | `r1`/`n1` | 2 (rewrite, then the verified `pad_with_zeros` form) — **now green** | `logs/58-bisect-decoder.log` (`decoder-0 max_abs 8.14`, `corr 0.031`), red `logs/59-decoder-primitive-tests.log` exit `101`, red `logs/60-convtr-fix-tests.log` exit `101`, **green `logs/61-ctr4-lib-tests.log` exit `0` (`5 passed`)** | fixed in the same attempt; the deleted `one_hot_stride()`/`OnceLock` edit had been **unverified — never compiled or tested**, and is now gone rather than verified; no escalation |
| `PAR-4` (four-stem parity) — measured state after the fixes | `r1`/`n1` | 2 (`GRN-4`, `CTR-4`) | `logs/62-parity-after-ctr4.log` exit `0`, `1 passed`, 8/8 stem × channel inside threshold (worst `max_abs 8.583e-6`, `RMSE 1.774e-7`, `SDR 84.36 dB`) | measured green on the current candidate; **no acceptance is recorded here** - the verdict belongs to CHECK |
| `GRN-4` - final state on the candidate (`2026-11-12T16:15Z`) | `r1`/`n1` | 1 | green `logs/56` (`3 passed`), never re-opened by `logs/57`, `logs/61`, `logs/62`, `logs/75` | **fixed**; no later measurement contradicts it, no recurrence, no escalation |
| `CTR-4` - final state on the candidate (`2026-11-12T16:15Z`) | `r1`/`n1` | 2 | green `logs/61` (`5 passed`), parity `logs/62` exit `0`, strict-load probe `logs/75` (4/4 rejected by `validate_state`) | **fixed and verified**; the earlier `one_hot_stride()`/`OnceLock` edit was deleted unverified and is recorded as removed, never as verified; no escalation |
| `PAR-4` - **closed** on the measured candidate (`2026-11-12T16:15Z`) | `r1`/`n1` | 2 (`GRN-4`, `CTR-4`) | `logs/62-parity-after-ctr4.log` exit `0`, `1 passed; 0 failed`, 8/8 rows inside `max_abs <= 3.5e-4` / `RMSE <= 1e-4` / `SDR >= 80 dB` (worst `max_abs 8.583e-6`, worst `RMSE 1.774e-7`, lowest `SDR 84.36 dB`); projection `metrics/parity.json` + `.csv`; identities `logs/78` | **closed** on the measured candidate - measurement only; the verdict and any acceptance belong to CHECK; no escalation |
| `ORD-4` (planner-created phase-ordering contradiction: `AC-4.3` as a flat `CHECK`-acceptance condition against the collection's pre-`CHECK` commit/closure prohibition) | `r1`/`n1` at `CHECK 1` (`AC-4.3` `unmet`); closed by contract `r2` at attempt `n=1/3` | 1 (contract revision `r1 → r2`; no product fix, no criterion weakened) | `CHECK 1` verdict `not accepted → PLAN` with `AC-4.1` `met`, `AC-4.2` `met`, `AC-4.3` `unmet`; revised contract in §PLAN record `### r2` §4; provenance `.pdca/open-issues-r5/rv1/plan.md:9,11` | closed by the contract revision; the `r2`/`n1` re-anchor re-confirmed the same candidate (`logs/83-reanchor-r2n1.log`, exit `0`, 8/8, no drift) and did not re-open it; no escalation, attempts not exhausted |
| `GH-4-PRE` (evidence gap: no independent observation that GitHub `#4` was not prematurely closed) | `r2`/`n1` at `CHECK 2` (`AC-4.3(1)` `unverified`) — evidence round under the same attempt `n=1/3`, no new attempt consumed | 1 (evidence produced: one read-only, unauthenticated GitHub observation plus the pack reference; **no product fix, no criterion or threshold change**) | `github/issue-4-observation.json` + raw `github/issue-4__2026-11-12T16-57-05Z.json` and `github/issue-4-timeline__2026-11-12T16-57-05Z.json`, `logs/84-github-issue-4-observation.log` — `HTTP 200` on both endpoints at `2026-11-12T16:57:05Z`, `state=open`, `state_reason=null`, `closed_at=null`, `0` `closed` / `0` `reopened` events | evidence produced and recorded; **no verdict stated** — the `AC-4.3(1)` decision belongs to CHECK; no recurrence, no escalation, attempts not exhausted (`n=1/3`) |

`r2` note (append-only): the attempt `r2`/`n1` re-anchor edited no product byte, so `GRN-4` (1 fix),
`CTR-4` (2 fixes) and `PAR-4` (closed on the measured candidate, 2 underlying fixes) carry into `r2`
unchanged — no recurrence, no new measure required, no escalation. `ORD-4` is the only key whose state
changed in this transition, and its fix is the contract revision itself (1 revision applied).

`GH-4-PRE` note (append-only, `2026-11-12T16:57Z`): the evidence round added **evidence only** — one
read-only GitHub observation and its reference in the pack. It changed no product byte, no gate result,
no criterion, no threshold and no counter (`N=1 · r=2 · n=1/3`, attempts `1`), and it states no verdict:
whether `AC-4.3(1)` is now satisfied is a `pdca-check` decision. The earlier keys are unaffected.

## Changed files

- created this cycle: `.pdca/status/issue-4-r5.md` (this record, from the persisted PLAN `r1`).
- no product, test, tool, config or doc file was edited by the creation of this record.
- `src/model/demucs.rs` (untracked in Git, pre-existing item file) → modified in this attempt `r1`/`n1`:
  `GroupNorm::forward` variance over the whole group; `FreqConvTranspose::one_hot_stride()` +
  `forward` zero-insert/flipped-kernel rewrite (**unverified, not compiled**); three new unit tests
  (`group_norm_pools_variance_over_the_whole_group`,
  `freq_transposed_conv_matches_the_definition`, `square_rewrite_conv_matches_the_definition`).
- `.pdca/status/issue-4-r5.md` → modified (this continuation: §Current state known-state line, §DO
  result in-progress block, §Defect history `GRN-4`/`CTR-4`, §Changed files, §Git state, §Progress log).
- `.pdca/open-issues-r5/rv1/item-4-demucs/logs/50…60-*.log`, `probe-r5/`, `probe-r5b/` → created as the
  attempt's evidence (run logs, bisection JSON and stage dumps).
- `.pdca/open-issues-r5/rv1/item-4-demucs/logs/61-ctr4-lib-tests.log`, `62-parity-after-ctr4.log`,
  `63-gate-build.log`, `65-gate-clippy.log`, `66-gate-test-full.log`, `67-gate-converter-tests.log` →
  created after the step-A recording (all measured: exit `0` each).
- **Not created by this dispatch:** `logs/68`-`82`, `scripts/make_strict_load_mutants.py`,
  `scripts/stem_names_e2e.rs`, `scripts/check_stem_names.py`, `scripts/extract_parity_metrics.py`,
  `out/stems-candidate/`, `snapshot/`, `snapshot-sha256.txt`, `snapshot.diff`, `metrics/parity.*`,
  `commands.txt`, `manifest.json`, `index.md`, `lint.txt` — these belong to a **concurrent dispatch of
  the same attempt** (recorded `2026-11-12T16:15Z`, see §Progress log Notice); they are listed here for
  attribution only and are not claimed as this dispatch's work.
- `src/model/demucs.rs` → further modified at `2026-11-12T15:2xZ` with the `CTR-4` fix (helper deleted,
  `pad_with_zeros` zero insertion); current bytes 88,355 B, sha256
  `b1e04d0fc3cf12ff6329a80b455b014020b21927e211b478dccba0c9dab40d07`. Untracked in Git, not staged,
  not committed.
- **This continuation (attempt complete, `2026-11-12T16:15Z`) — no product byte edited.** Created under
  the evidence root `.pdca/open-issues-r5/rv1/item-4-demucs/`:
  `scripts/make_strict_load_mutants.py`, `scripts/stem_names_e2e.rs`, `scripts/check_stem_names.py`,
  `scripts/extract_parity_metrics.py`, `logs/68`–`logs/82`, `out/stems-candidate/{drums,bass,other,vocals}.wav`
  (4 × 3,528,044 B), `metrics/parity.json`, `metrics/parity.csv`, `snapshot/` (8 files),
  `snapshot-sha256.txt`, `snapshot.diff` (148,596 B), `commands.txt`, `manifest.json`, `index.md`,
  `lint.txt`. Mutant artifacts were built in scratch
  `C:\Users\<user>\AppData\Local\Temp\opencode\demucs-strict-load\` and are identified by sha256 in
  `logs/74`/`logs/75`; they are inputs, not evidence files, so only the logs are persisted.
- **This continuation — status files:** `.pdca/status/issue-4-r5.md` (this file: §Current state,
  §DO result, §Defect history, §Changed files, §Git state, §Progress log) and
  `.pdca/status/collection-open-issues-r5.md` (§Current state, §Item table row `#4`, §DO result,
  §General verification and recovery, §Changed files, §Git state, §Progress log).
- **This dispatch (`2026-11-12T16:48Z`, PLAN recording + `DO` `r2`/`n1`) — no product byte edited.**
  - `.pdca/status/issue-4-r5.md` → modified: §Current state (`r=2`, `n=1/3`, transition bullet),
    §PLAN record `### r2` (planner's complete return, verbatim), §CHECK record `### CHECK 1 — r1/n1`,
    §DO result `### Attempt r2/n1`, §Revision history `r1 → r2`, §Defect history (`ORD-4` row + the `r2`
    note), §Changed files, §Git state, §Progress log.
  - `.pdca/status/collection-open-issues-r5.md` → modified: header `criteria` pointer, §Current state,
    §PLAN record pointer, §Item table row `#4` (`plan_revision 2`), §Revision history pointer,
    §Defect history pointer, §DO result, §Changed files, §Git state, §Progress log.
  - `.pdca/open-issues-r5/rv1/item-4-demucs/logs/83-reanchor-r2n1.log` → **created** (the `r2`/`n1`
    re-anchor: HEAD/branch/index/porcelain identity plus the 8-row sha256 comparison, 8/8, exit `0`).
  - No product, test, tool, config or doc file was edited; no evidence pack was rebuilt; no criterion
    was changed.

- **This evidence round (`2026-11-12T16:57Z`, `GH-4-PRE` under `r2`/`n1`) — no product byte edited.**
  - `.pdca/open-issues-r5/rv1/item-4-demucs/scripts/observe_issue4.py` → **created** (read-only
    observation producer; no credential read, sent or stored).
  - `.pdca/open-issues-r5/rv1/item-4-demucs/github/issue-4__2026-11-12T16-57-05Z.json` (4,635 B),
    `github/issue-4-timeline__2026-11-12T16-57-05Z.json` (1,329 B) and
    `github/issue-4-observation.json` (2,587 B) → **created** (raw bodies + summary).
  - `.pdca/open-issues-r5/rv1/item-4-demucs/logs/84-github-issue-4-observation.log` → **created**
    (producer log, exit `0`); `logs/85-no-product-drift-gh4pre.log` → **created** (8/8 product hashes
    equal to `snapshot-sha256.txt`, 0 mismatch, exit `0`).
  - `.pdca/open-issues-r5/rv1/item-4-demucs/manifest.json` → modified: root `github_issue` block,
    `criterion_evidence.AC-4.3` `sub_records` (`AC-4.3(1)` evidence recorded, `AC-4.3(2)` not reached),
    `git.github_read_or_write` corrected to the read-only observation, artefacts table.
  - `.pdca/open-issues-r5/rv1/item-4-demucs/index.md` → modified: header revision `r=2`, new
    `## GitHub #4 observation` section, file map row for `github/`.
  - `.pdca/open-issues-r5/rv1/item-4-demucs/commands.txt` → modified: section 7 with the exact requests,
    HTTP statuses, sizes, sha256 values and exits.
  - status records: `.pdca/status/issue-4-r5.md` (§Current state, §DO result evidence round,
    §Defect history `GH-4-PRE`, §Changed files, §Git state, §Progress log) and
    `.pdca/status/collection-open-issues-r5.md` (§Current state/next allowed step, §DO result,
    §General verification and recovery, §Progress log).
  - **Not touched:** every product/test/tool file of the item delta, item `#11`, `.openidea/`,
    `.pdca/status/collection-open-issues-r3.md`, thresholds and criteria.

- **This ACT dispatch (`2026-11-12T17:06Z`, contract `r2` / attempt `n=1/3`) — the post-acceptance
  commit was performed; no further product byte was edited.**
  - `.pdca/open-issues-r5/rv1/item-4-demucs/scripts/act_reanchor_verify.py` — **created** (read-only
    re-anchor verifier: worktree hash + snapshot hash + byte size against `snapshot-sha256.txt`).
  - `.pdca/open-issues-r5/rv1/item-4-demucs/logs/86-act-reanchor-r2n1.log`,
    `logs/87-act-commit-verify.log` — **created** (re-anchor result 8/8; commit identity, file list,
    foreign-path scan, HEAD-vs-worktree diff, index emptiness, post-commit re-hash).
  - `.pdca/open-issues-r5/rv1/item-4-demucs/commands.txt`, `lint.txt` — **appended** (ACT section and the
    ACT lint runs with their exit codes).
  - `.pdca/status/issue-4-r5.md` (this record) and `.pdca/status/collection-open-issues-r5.md` —
    **modified** (Current state, CHECK record `### CHECK 2`/`### CHECK 3`, ACT record, Lessons-learned
    closure blocker, Changed files, Git state, Progress log; collection: Current state, item row `#4`
    note, ACT record, Git state, Progress log).
  - **Committed product delta (the only files this dispatch staged):** the item's 8 paths — `src/config.rs`,
    `src/model/mod.rs`, `src/separator.rs` (modified) and `src/model/demucs.rs`, `src/model/demucs_pipeline.rs`,
    `tests/demucs_parity.rs`, `tools/convert_demucs.py`, `tools/tests/test_convert_demucs.py` (new) — as
    commit `d042c26`.

- **This finalization dispatch (`2026-11-12T17:13Z`, contract `r2` / attempt `n=1/3`) — no product byte
  edited, no staging, no commit.**
  - `.pdca/status/issue-4-r5.md` (this record) → **modified**: §Current state (header `record: completed`
    / `outcome: accepted` + finalization bullet), §ACT record (retrospective recorded, completion,
    handoff), §Lessons learned (the raw `pdca-check` retrospective persisted verbatim + recording notes),
    §Changed files, §Git state, §Progress log.
  - `.pdca/status/collection-open-issues-r5.md` → **modified**: §Current state, §Item table row `#4`
    (`done` / `finalized`, `plan_revision 2`, attempts `1`), §ACT record `#4` block, §General
    verification and recovery, §Progress log.
  - `.pdca/open-issues-r5/rv1/integration/` → **created** by the same dispatch as the *collection's*
    integration staging (`facts.md`, `snapshot/` 43 files, `snapshot-sha256.txt`, `index.md`,
    `lint.txt`, `scripts/`, `logs/`). It is **not** part of item `#4`'s task-owned delta and no product
    path was moved or edited by it; it is listed here for attribution only.
  - **Not touched:** every product/test/tool file of the item delta, item `#11`, `.openidea/`,
    `.pdca/status/collection-open-issues-r3.md`, thresholds and criteria.

## Git state

- status path: `.pdca/status/issue-4-r5.md` — created this cycle · untracked (durable on disk, not
  committed; `.gitignore` `/.pdca/*` keeps the whole `.pdca/` tree out of `git status`).
- baseline supplied at creation: branch `main`, `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685`
  (`8b03cfd`), `git diff --cached --stat` empty (nothing staged).
- task-owned delta pending commit, not indexed and not claimed as staged: the `#4` product set —
  `M src/config.rs`, `M src/model/mod.rs`, `M src/separator.rs`, `?? src/model/demucs.rs`,
  `?? src/model/demucs_pipeline.rs`, `?? tests/demucs_parity.rs`, `?? tools/convert_demucs.py`,
  `?? tools/tests/test_convert_demucs.py` (pre-existing work, bytes to be recorded once measured).
- untouched and unclaimed: `.openidea/`, `.pdca/status/collection-open-issues-r3.md`, and every other
  `.pdca/status/*.md` record.
- no `git add`, no staging, no commit, no push, no branch switch, no amend, no tag, no GitHub read or
  write by this record.
- refreshed at `2026-11-12T15:17Z`: `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685` (`8b03cfd`), branch
  `main`, `git diff --cached --stat` empty (nothing staged); the delta above is unchanged in identity —
  no file was added to or removed from the task-owned set, and no commit, push, branch switch, amend,
  tag or GitHub read/write was performed.
- refreshed at `2026-11-12T16:42Z`: `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685` (`8b03cfd`), branch
  `main`, `git diff --cached --name-only` empty (**nothing staged**), `git status --porcelain` 12 lines
  — the same item set plus ` M .pdca/status/collection-open-issues-r5.md` and the untracked
  `?? .pdca/status/issue-4-r5.md`; `src/model/demucs.rs` 88,355 B, sha256
  `b1e04d0fc3cf12ff6329a80b455b014020b21927e211b478dccba0c9dab40d07`, untracked, unmodified since the
  gate runs. No `git add`, no staging, no commit, no push, no branch switch, no amend, no tag, no
  GitHub read or write. Attribution note: the **collection** record is tracked-and-modified, the
  **item** record is untracked — the earlier "both modified in the worktree only" phrasing is imprecise
  and is superseded here.
- **Refreshed at `2026-11-12T16:15Z` (attempt complete), with one earlier imprecision corrected:**
  `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685` (`8b03cfd`) on `main`; index **empty** —
  `git diff --cached --name-only` prints nothing, nothing staged. Tracked/untracked state of the two
  status files, verified by `git status --porcelain`: `.pdca/status/issue-4-r5.md` is **untracked**
  (`?? .pdca/status/issue-4-r5.md`; the file was created this cycle and `.pdca/status/` is not
  covered by the `/.pdca/*` ignore for this path because sibling records were committed explicitly),
  while `.pdca/status/collection-open-issues-r5.md` is **tracked and modified** (` M`). Earlier
  wording that implied either file was committed in this cycle is imprecise and is corrected here;
  the file is durable on disk, not by Git. Task-owned delta unchanged in identity: `M src/config.rs`,
  `M src/model/mod.rs`, `M src/separator.rs`, `?? src/model/demucs.rs`, `?? src/model/demucs_pipeline.rs`,
  `?? tests/demucs_parity.rs`, `?? tools/convert_demucs.py`, `?? tools/tests/test_convert_demucs.py`
  (sha256 recorded in `snapshot-sha256.txt`, 8/8 worktree == snapshot). Untouched and unclaimed:
  `.openidea/`, `.pdca/status/collection-open-issues-r3.md`, every other `.pdca/status/*.md`, item
  `#11`. No `git add`, no staging, no commit, no push, no branch switch, no amend, no tag, **no
  GitHub read or write**; the evidence root stays local (`.pdca/` is ignored).
- **Refreshed at `2026-11-12T16:48Z` (contract `r2`, attempt `n=1/3`, after the re-anchor):**
  `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685` (`8b03cfd`) on `main`; index **empty** —
  `git diff --cached --name-only` prints nothing, nothing staged; `git status --porcelain` shows the
  same 8-path item delta (`M src/config.rs`, `M src/model/mod.rs`, `M src/separator.rs`,
  `?? src/model/demucs.rs`, `?? src/model/demucs_pipeline.rs`, `?? tests/demucs_parity.rs`,
  `?? tools/convert_demucs.py`, `?? tools/tests/test_convert_demucs.py`), ` M
  .pdca/status/collection-open-issues-r5.md` (tracked-and-modified), `?? .pdca/status/issue-4-r5.md`
  (untracked), and the pre-existing `?? .openidea/`, `?? .pdca/status/collection-open-issues-r3.md`;
  worktree == snapshot for all 8 product paths (`logs/83-reanchor-r2n1.log`, 8/8, `0` missing). No
  `git add`, no staging, no commit, no push, no branch switch, no amend, no tag, **no GitHub `#4` read
  or write**. The `.pdca/status/*` records are durable on disk only and are **not** part of the
  task-owned product delta, so they must stay out of the `ACT` product commit.

- **Refreshed at `2026-11-12T16:58Z` (GH-4-PRE evidence round, `r2`/`n1`):**
  `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685` (`8b03cfd`) on `main`; `git diff --cached --name-only`
  prints nothing (**index empty, nothing staged**); `git status --porcelain` = 12 lines — the same 8-path
  product delta (`M src/config.rs`, `M src/model/mod.rs`, `M src/separator.rs`, `?? src/model/demucs.rs`,
  `?? src/model/demucs_pipeline.rs`, `?? tests/demucs_parity.rs`, `?? tools/convert_demucs.py`,
  `?? tools/tests/test_convert_demucs.py`), ` M .pdca/status/collection-open-issues-r5.md`
  (tracked-and-modified), `?? .pdca/status/issue-4-r5.md` (untracked), plus the pre-existing
  `?? .openidea/` and `?? .pdca/status/collection-open-issues-r3.md`. No product byte changed in this
  round; the new files are evidence under the ignored `.pdca/` tree. No `git add`, no staging, no commit,
  no push, no branch switch, no amend, no tag. **GitHub traffic in this round: read-only only** — two
  unauthenticated GETs of issue `#4` (issue + timeline); **no write of any kind** (no comment, close,
  label or state change), no token read, sent or stored. The task-owned product delta is unchanged, so
  nothing here alters the bytes a later `ACT` commit would carry; the `.pdca/status/*` records stay
  durable on disk only and outside that commit.

- **Refreshed at `2026-11-12T17:06Z` (ACT, after the post-acceptance commit):** status path
  `.pdca/status/issue-4-r5.md` — **modified this cycle**, **untracked** (`?? .pdca/status/issue-4-r5.md`;
  durable on disk, not committed — no `git add` of any `.pdca` path in this dispatch).
  `.pdca/status/collection-open-issues-r5.md` — **modified this cycle**, **tracked and modified** (` M`).
  Baseline carried from creation: `main`, `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685`, nothing staged.
  Now: **`HEAD = d042c2628edb52deab42118e18ac69974e5ad75d` (`d042c26`)** on `main`, parent `8b03cfd`;
  index **empty** (`git diff --cached --name-only` prints nothing); **not pushed** (`d042c26` is not an
  ancestor of `origin/main`; no push, no branch switch, no amend, no rebase, no tag was performed);
  `git diff --name-only HEAD -- <8 item paths>` **empty** — the worktree bytes still equal their snapshot
  rows and are now also the committed bytes; `git status --porcelain` shows only
  ` M .pdca/status/collection-open-issues-r5.md`, `?? .pdca/status/issue-4-r5.md` and the pre-existing
  `?? .openidea/`, `?? .pdca/status/collection-open-issues-r3.md` — no product path remains dirty.
  `core.autocrlf=true` and no `.gitattributes`: `git add` printed "LF will be replaced by CRLF the next
  time Git touches it" for six paths, yet `git diff --name-only HEAD` is empty and the worktree still
  hashes 8/8, i.e. no byte divergence between worktree and committed blob — recorded as a **`Notice`
  (cosmetic, no identity drift)**, nothing amended. Untouched and unclaimed: `.openidea/`,
  `.pdca/status/collection-open-issues-r3.md`, item `#11`, every other `.pdca/status/*.md` record.
  **GitHub traffic: none** — no read and **no write of any kind** in this dispatch.

- **Refreshed at `2026-11-12T17:14Z` (ACT finalization — retrospective recorded, record `completed`):**
  status path `.pdca/status/issue-4-r5.md` — **modified this cycle**, **untracked**
  (`?? .pdca/status/issue-4-r5.md`; durable on disk, not committed — no `git add` of any `.pdca` path in
  this dispatch). `.pdca/status/collection-open-issues-r5.md` — **modified this cycle**, **tracked and
  modified** (` M`). Baseline carried from creation: `main`,
  `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685`, nothing staged. Now: **`HEAD =
  d042c2628edb52deab42118e18ac69974e5ad75d` (`d042c26`)** on `main`; index **empty**
  (`git diff --cached --name-only` prints nothing); **not pushed** (`origin/main` is `8b03cfd` and
  `git merge-base --is-ancestor HEAD origin/main` exits `1`); `git diff --name-only HEAD -- <8 item
  paths>` **empty** — no product path is dirty. `git status --porcelain` is exactly four lines:
  ` M .pdca/status/collection-open-issues-r5.md`, `?? .pdca/status/issue-4-r5.md`, plus the
  **pre-existing, untouched, unclaimed** `?? .openidea/` and `?? .pdca/status/collection-open-issues-r3.md`.
  No `git add`, no staging, no commit, no push, no branch switch, no amend, no tag, no rebase in this
  dispatch. **GitHub traffic: none** — no read and no write; the `#4` closure stays the calling
  driver's external action.

- **Refreshed at `2026-11-12T17:33Z` (GitHub `#4` closure round — evidence and one authorized GitHub
  write, no product change):** status path `.pdca/status/issue-4-r5.md` — **modified this cycle**,
  **untracked** (`?? .pdca/status/issue-4-r5.md`; durable on disk, not committed — no `git add` of any
  `.pdca` path). `.pdca/status/collection-open-issues-r5.md` — **modified this cycle**, **tracked and
  modified** (` M`). Baseline carried from creation unchanged: `main`,
  `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685`, nothing staged. Now: **`HEAD =
  d042c2628edb52deab42118e18ac69974e5ad75d` (`d042c26`)** on `main`; index **empty**
  (`git diff --cached --name-only` prints nothing); no product path dirty (`git diff --name-only HEAD -- <8
  item paths>` empty). **Push state:** `d042c26` stays unpublished — `git merge-base --is-ancestor d042c26
  origin/main` exits `1` and `git ls-remote origin` returns only `HEAD` and `refs/heads/main @ 8b03cfd`;
  the three earlier item commits are ancestors of that ref (published by an earlier non-PDCA push, recorded
  honestly in the collection record's `INT-F03` note). `git status --porcelain` still exactly four lines
  (` M` collection record, `??` this record, plus the pre-existing `?? .openidea/`, `?? .pdca/status/collection-open-issues-r3.md`).
  Untouched and unclaimed: `.openidea/`, `.pdca/status/collection-open-issues-r3.md`, item `#11`, every
  other `.pdca/status/*.md`. **GitHub traffic: one authorized write** (`gh issue close 4 --reason
  completed`) plus unauthenticated read-only verification GETs; no credential value read, printed or
  recorded.

## Progress log

- 2026-11-12T15:05:06Z | DO | r1 | n=1 | item `#4` record created from the persisted PLAN `r1` (the file did not exist); contract `AC-4.1`–`AC-4.3` quoted verbatim from `.pdca/open-issues-r5/rv1/plan.md:9`, thresholds from the approved addendum; **no criterion authored, corrected or completed by this write**; phase `DO`, `DO started`, counters `N=1 · r=1 · n=1/3` | `.pdca/open-issues-r5/rv1/plan.md`, `.pdca/open-issues-r5/rv1/criteria-approval.md`
- 2026-11-12T15:05:06Z | DO | r1 | n=1 | pre-existing workspace work re-confirmed before any edit: `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685` on `main`, nothing staged, `M src/config.rs` `M src/model/mod.rs` `M src/separator.rs` plus 5 untracked item files, pre-existing untracked `.openidea/` and `.pdca/status/collection-open-issues-r3.md` untouched | `git status --porcelain`, `git rev-parse HEAD`
- 2026-11-12T15:05:06Z | DO | r1 | n=1 | **Notice — evidence-root label mismatch:** the recorded label `item-4-htdemucs` does not exist; the item's real evidence root is `.pdca/open-issues-r5/rv1/item-4-demucs/` (39 logs, `ref/`, `probe/`, `probe-ours/`, `probe-ours2/`, `analysis/`, `convert/`, `models/`, `fixture/`, `scripts/`, `env.json`, `progress.log`); this record uses the real path | `.pdca/open-issues-r5/rv1/item-4-demucs/`
- 2026-11-12T15:17:10Z | DO | r1 | n=1 | fresh baseline re-established before any edit: converter tests `Ran 10 tests` `OK` exit `0`; `cargo build --release` exit `0`; parity exit `101`, `0 passed; 1 failed`, `max_abs 2.6e3…4.0e3` | `logs/50-converter-tests.log`, `logs/51-cargo-build.log`, `logs/52-parity-baseline-trace.log`
- 2026-11-12T15:17:10Z | DO | r1 | n=1 | **`GRN-4` fixed** — `GroupNorm` now pools the mean and variance over the whole group, not per channel; unit test red exit `101` → green exit `0` (`3 passed`); bisection boundary moved from `encoder.0.dconv.layers.0-norm1` to the decoder (parity `max_abs 0.62…1.09`, `SDR −41.7…+2.7 dB`, exit `101`) | `logs/54-groupnorm-check.log`, `logs/55-groupnorm-test-red.log`, `logs/56-groupnorm-test-green.log`, `logs/57-parity-after-groupnorm-fix.log`, `logs/58-bisect-decoder.log`
- 2026-11-12T15:17:10Z | DO | r1 | n=1 | **`CTR-4` open** — `Tensor::conv_transpose1d` is not the `ConvTranspose` definition for kernel > 1; `FreqConvTranspose::forward` rewritten as zero-insert + flipped-kernel `conv1d`, unit test still red exit `101` (`got 1 expected 0.25`, then `got 0 expected -0.5`); the `one_hot_stride()` edit with the leftover unused `OnceLock` stub is **unverified — never compiled or tested** | `logs/59-decoder-primitive-tests.log`, `logs/60-convtr-fix-tests.log`
- 2026-11-12T15:17:10Z | DO | r1 | n=1 | **recorded the durable state before touching code** (this continuation dispatch): attempt `r1`/`n1` in progress, `GRN-4` fixed / `CTR-4` open / `PAR-4` still red, the product delta and its unverified status, `HEAD 8b03cfd`, nothing staged, no commit/push/GitHub write; next allowed step `finish CTR-4, re-run parity`; counters `N=1 · r=1 · n=1/3`; lint exit codes: `status_lint.py` on this file and on the collection record → see §Progress log tail of the collection record | `.pdca/status/issue-4-r5.md`
- 2026-11-12T15:18:43Z | DO | r1 | n=1 | `status_lint.py` run on both records after the step-A write: item `status-lint: OK (.pdca\status\issue-4-r5.md: 1 tables, 3 rows)` exit **`0`**; collection `status-lint: OK (.pdca\status\collection-open-issues-r5.md: 2 tables, 7 rows)` exit **`0`** | `.pdca/scripts/status_lint.py`
- 2026-11-12T16:01:20Z | DO | r1 | n=1 | **`CTR-4` fixed and verified** — dead `OnceLock` stub + `one_hot_stride()` removed, zero insertion via `pad_with_zeros`, flipped-kernel `conv1d`; `cargo test --release --lib model::demucs::tests` exit **`0`**, `5 passed; 0 failed` (test was red exit `101` before) | `logs/61-ctr4-lib-tests.log`
- 2026-11-12T16:01:20Z | DO | r1 | n=1 | **parity re-measured, exit `0`, `1 passed; 0 failed`**, 8/8 stem × channel inside `max_abs ≤ 3.5e-4` / `RMSE ≤ 1e-4` / `SDR ≥ 80 dB` (worst `max_abs 8.583e-6`, worst `RMSE 1.774e-7`, lowest `SDR 84.36 dB`); frozen reference unchanged | `logs/62-parity-after-ctr4.log`
- 2026-11-12T16:01:20Z | DO | r1 | n=1 | gates on the fixed bytes: `cargo build` exit **`0`**; `cargo clippy --all-targets` exit **`0`** (warnings only, none in the edited region); full `cargo test` **in progress** (first attempt killed by a command timeout after `vr_stage_waves`, all per-target results up to that point `ok`); converter tests **not yet re-run** after the fix | `logs/63-gate-build.log`, `logs/65-gate-clippy.log`, `logs/66-gate-test-full.log`
- 2026-11-12T16:42:56Z | DO | r1 | n=1 | **gates complete on the fixed bytes**: `cargo test` exit **`0`**, **144 passed / 0 failed / 4 ignored** over 25 targets; converter tests exit **`0`** (`Ran 10 tests` / `OK`); `cargo build` exit `0`; `cargo clippy --all-targets` exit `0`; parity exit `0` (`1 passed`). **candidate numerically ready** — no verdict, no PASS/STOP, no acceptance, no commit/staging/push/GitHub write; next allowed step `CHECK item #4 (r1/n1)` | `logs/66-gate-test-full.log`, `logs/67-gate-converter-tests.log`, `logs/62-parity-after-ctr4.log`
- 2026-11-12T16:15:00Z | DO | r1 | n=1 | **full-suite gate:** `cargo test` (debug, whole workspace) exit **`0`** — `144 passed / 0 failed / 4 ignored` over 24 targets + doc-tests, 41m54s wall; the launcher's filtered log ends with `EXIT=0` (`logs/66-gate-test-full.log`); the earlier attempt killed by the command timeout after `vr_stage_waves` is history (`logs/64`) and was never counted as a pass | `.pdca/open-issues-r5/rv1/item-4-demucs/logs/66-gate-test-full.log`
- 2026-11-12T16:15:00Z | DO | r1 | n=1 | converter tests on the fixed bytes: `& $py -m unittest -v tools.tests.test_convert_demucs` exit **`0`**, `Ran 10 tests in 9.090s`, `OK`; pinned checkpoint re-verified by the converter itself (`--check-only` exit **`0`**): source sha256 `8726e21a…faa8b4`, size `84,141,911`, `tensor_count 533`, all `F32` | `logs/68-converter-tests-fixed-bytes.log`, `logs/69-converter-check-only.log`
- 2026-11-12T16:15:00Z | DO | r1 | n=1 | **`AC-4.1` strict-load rejection measured**: four canonically rebuilt mutants of the real 167,993,600 B artifact, each rejected by `HTDemucs::validate_state` — missing (`1 missing … decoder.3.conv_tr.weight`), unexpected (`1 unexpected … strict.load.probe`), mis-shaped (`[6144] != [48, 16, 8, 1]`), non-f32 (`dtype I32 != F32`); `cargo test` exit **`101`**, `0 passed / 1 failed` each. Two earlier probe rounds kept as history: `logs/71` (header-only mutants killed by the safetensors container before the validator — **not** accepted as evidence) and `logs/73` (shallow-copy bug in the probe generator; the `non_f32` row was wrong) | `logs/70`–`logs/75`, `scripts/make_strict_load_mutants.py`
- 2026-11-12T16:15:00Z | DO | r1 | n=1 | **`AC-4.1` per-stem naming end to end on disk**: evidence-only harness (`rustc` against `target/debug/libcandle_roformer.rlib`, no product file or crate target added) wrote `names=["drums", "bass", "other", "vocals"] len=4` to `out/stems-candidate/`, 4 × 3,528,044 B, 441,000 frames × 2 ch @ 44,100 Hz, all finite, exit **`0`**; independent RIFF read-back exit **`0`** — exactly the four names, `f32`, 44100 Hz, 2 ch, 441000 frames, all finite, non-silent | `logs/80-stem-names-e2e.log`, `logs/82-stem-names-check.log`, `scripts/stem_names_e2e.rs`, `scripts/check_stem_names.py`
- 2026-11-12T16:15:00Z | DO | r1 | n=1 | **evidence pack written** (no product byte edited): `manifest.json`, `index.md`, `snapshot/` + `snapshot-sha256.txt` (8/8 worktree == snapshot) + `snapshot.diff` (148,596 B, 8 `diff --git` headers = exactly the item delta), `metrics/parity.json` + `.csv` (8 measured rows, all inside the limits), `commands.txt`, `lint.txt` | `.pdca/open-issues-r5/rv1/item-4-demucs/`
- 2026-11-12T16:15:00Z | DO | r1 | n=1 | **`DO` attempt `r1`/`n1` complete - candidate ready for CHECK**; `HEAD 8b03cfd`, index empty, nothing staged, no commit/push/branch switch/amend/tag/GitHub read-or-write; counters unchanged `N=1 · r=1 · n=1/3`, attempts `1`; next allowed step **`CHECK item #4 (r1/n1)`** | `.pdca/status/issue-4-r5.md`
- 2026-11-12T16:45Z | DO | r1 | n=1 | **Notice - two concurrent executor dispatches on the same attempt (reconciled, not a defect):** this dispatch produced `logs/61`-`67` (CTR-4 fix, parity, build, clippy, full `cargo test`, converter tests) and the rows stamped `15:17`-`16:42`; the other dispatch produced `logs/68`-`82`, the harness scripts, `out/stems-candidate/`, the snapshot, `metrics/` and the evidence pack, and the rows stamped `16:15:00Z`. Both report the same measured facts (lib `5 passed`; parity exit `0`, 8/8 inside threshold; `cargo test` 144/0/4 exit `0`; converter `Ran 10 tests` `OK`; `HEAD 8b03cfd`, index empty) and no product byte was changed by either after the `CTR-4` fix (`src/model/demucs.rs` sha256 `b1e04d0f…40d07`, unchanged). **Attribution:** the evidence pack (`manifest.json`, `index.md`, `snapshot/`, `metrics/`, `commands.txt`, `lint.txt`) and `out/stems-candidate/` were **created by the other dispatch, not by this one**; this dispatch did not build, extend or validate them. Row order in the log is append order, not chronology (`16:42` precedes `16:15`); no chronology is inferred and no earlier row was rewritten | `.pdca/open-issues-r5/rv1/item-4-demucs/logs/61`-`82`
- 2026-11-12T16:46:00Z | DO | r1 | n=1 | `status_lint.py` on both records after the attempt-complete write: this file `status-lint: OK (.pdca\status\issue-4-r5.md: 4 tables, 24 rows)` exit **`0`**; collection `status-lint: OK (.pdca\status\collection-open-issues-r5.md: 2 tables, 7 rows)` exit **`0`**; both files re-linted on their final bytes, runs appended to `lint.txt` | `.pdca/open-issues-r5/rv1/item-4-demucs/lint.txt`
- 2026-11-12T16:48Z | CHECK | r1 | n=1 | **`CHECK 1` (`r1`/`n1`) returned `not accepted → PLAN`** — matrix `AC-4.1` `met`, `AC-4.2` `met`, `AC-4.3` `unmet`; finding `ORD-4` (blocking, confirmed: acceptance ordering of `AC-4.3` against the collection's pre-`CHECK` commit/closure prohibition); non-blocking notes recorded by CHECK on hash recomputation, ignored tests, GPU behaviour and inputs beyond the approved fixture, with the live GitHub `#4` state unverified at CHECK time; the complete CHECK report was **not** supplied as a separate artefact and its recorded elements are stated as such | §CHECK record `### CHECK 1 — r1/n1`; §PLAN record `### r2` §5/§8
- 2026-11-12T16:48Z | PLAN | r1 → r2 | n=1/3 | **outgoing attempt booked first:** `DO` `r1`/`n1` complete (parity passing — `max_abs 8.583e-6`, `RMSE 1.774e-7`, lowest `SDR 84.36 dB`; gates green) and `CHECK 1` `not accepted → PLAN`; attempts consumed `1`, and the transition consumed **no** new attempt · **Replanned: CHECK→PLAN acceptance-ordering contract (r 1→2)** — `AC-4.1`/`AC-4.2` preserved verbatim, `AC-4.3` partitioned into `AC-4.3(1)` (pre-acceptance `CHECK` gate) and `AC-4.3(2)` (post-acceptance `ACT` commit + GitHub `#4` closure); item-level only — the collection plan stays `r1`; counters `N=1 · r=2 · n=1/3` | §PLAN record `### r2`; §Revision history
- 2026-11-12T16:48Z | DO | r2 | n=1 | **`DO` `r2`/`n1` executed and completed (`AC-4.3(1)` readiness; no product edit):** `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685` on `main`, index empty, 8/8 product hashes equal to `snapshot-sha256.txt` (`0` missing, `0` mismatch), delta isolated to item `#4`'s 8 product paths, no premature commit or push, **no GitHub `#4` read or write**, no evidence-pack rebuild, no criterion change; run exit `0` | `logs/83-reanchor-r2n1.log`; §DO result `### Attempt r2 / n1`
- 2026-11-12T16:48Z | DO | r2 | n=1 | **`DO` `r2`/`n1` marked complete — candidate ready for CHECK**; counters `N=1 · r=2 · n=1/3`, item `#4` attempts `1`; next allowed step **`CHECK item #4 (r2/n1)`**; the atomic commit and the GitHub `#4` closure remain `ACT` work under `AC-4.3(2)` and are **not authorized** before an accepted CHECK | §Current state; §Git state
- 2026-11-12T16:48Z | DO | r2 | n=1 | **`status_lint.py` after the `r2` write:** this file `status-lint: OK (.pdca\status\issue-4-r5.md: 5 tables, 28 rows)` exit **`0`**; collection `status-lint: OK (.pdca\status\collection-open-issues-r5.md: 2 tables, 7 rows)` exit **`0`**; both files re-linted on their post-write bytes with the same exit codes | `.pdca/scripts/status_lint.py`
- 2026-11-12T16:57:05Z | CHECK | r2 | n=1 | **`CHECK 2` routed `GH-4-PRE` (`AC-4.3(1)`) back to `DO` for evidence only** — recorded as supplied by the dispatch brief, verbatim: «GH-4-PRE — major, unverified; AC-4.3(1): отсутствует независимое наблюдение отсутствия преждевременного закрытия GitHub `#4`. Это пробел доказательств, **не установленное нарушение**.» Next transition: → `DO`, evidence gathering only; the complete `CHECK 2` report was **not** supplied to this write as a separate persisted artefact and is therefore not reconstructed here. Counters unchanged `N=1 · r=2 · n=1/3`; no attempt consumed | dispatch brief; §DO result `### Evidence round r2 / n1`
- 2026-11-12T16:57:05Z | DO | r2 | n=1 | **read-only GitHub `#4` observation done** (`scripts/observe_issue4.py`, unauthenticated, no token read/sent/stored, repo identity from `git remote -v`): `GET https://api.github.com/repos/AlexeyShirshov/candle-roformer/issues/4` → `HTTP 200`, 4,635 B, sha256 `c9734a32…f18117` — `state=open`, `state_reason=null`, `closed_at=null`, `created_at=2026-11-08T06:43:20Z`, `number=4`, `html_url=https://github.com/AlexeyShirshov/candle-roformer/issues/4`, labels `[]`, comments `0`; `GET …/issues/4/timeline?per_page=100` → `HTTP 200`, 1,329 B, sha256 `9087d84f…93848f` — `1` event (`milestoned 1.0` at `2026-11-08T06:43:20Z`), **`0` `closed` / `0` `reopened` events**, no `Link` header / no `rel="next"` page, so the one page is the whole history; **no premature closure exists**; exit `0` | `github/issue-4-observation.json`, `github/issue-4__2026-11-12T16-57-05Z.json`, `github/issue-4-timeline__2026-11-12T16-57-05Z.json`, `logs/84-github-issue-4-observation.log`
- 2026-11-12T16:58Z | DO | r2 | n=1 | **evidence pack updated by reference only (no product byte, no gate re-run, no verdict):** `manifest.json` root `github_issue` block + `criterion_evidence.AC-4.3` sub-records (`AC-4.3(1)` evidence recorded, `AC-4.3(2)` not reached), `index.md` `## GitHub #4 observation` section, `commands.txt` section 7 with the exact requests and HTTP statuses; `HEAD 8b03cfd`, index empty, no commit/staging/push, **no GitHub write of any kind**; counters unchanged `N=1 · r=2 · n=1/3`, item `#4` attempts `1`; next allowed step **`CHECK item #4 (r2/n1)` — CHECK 3** | `.pdca/open-issues-r5/rv1/item-4-demucs/manifest.json`, `index.md`, `commands.txt`
- 2026-11-12T16:59Z | DO | r2 | n=1 | **`status_lint.py` after the `GH-4-PRE` write:** this file exit **`0`** (`status-lint: OK`) and the collection record exit **`0`** (`status-lint: OK`); the final-byte re-lint, with its table/row counts, is appended to `lint.txt`; counters unchanged `N=1 · r=2 · n=1/3`, item `#4` attempts `1`; next allowed step **`CHECK item #4 (r2/n1)` — CHECK 3** | `.pdca/scripts/status_lint.py`, `.pdca/open-issues-r5/rv1/item-4-demucs/lint.txt`
- 2026-11-12T17:06Z | ACT | r2 | n=1 | **ACT dispatch started: `CHECK 3` (`r2`/`n1`) returned `accepted → ACT`** — recorded as returned by the dispatch brief (verdict line only; the complete CHECK 3 report was not supplied as a persisted artefact and is recorded as absent, not reconstructed). Accepted snapshot: `HEAD 8b03cfd` + the 8-path product delta frozen in `item-4-demucs/snapshot/` + `snapshot-sha256.txt` (8/8). Only the post-acceptance closure of `AC-4.3(2)` moves to ACT; no attempt consumed, counters unchanged `N=1 · r=2 · n=1/3` | dispatch brief; §CHECK record `### CHECK 3 - r2/n1`
- 2026-11-12T17:06:04Z | ACT | r2 | n=1 | **pre-stage re-anchor OK** — worktree == `snapshot/` copy == `snapshot-sha256.txt`: **matched=8, mismatched=0, missing=0 of 8**, verifier exit **`0`**; `HEAD 8b03cfda15a485b115f8b694d07c03e98fcdf685` (unchanged), branch `main`, `git diff --cached --name-only` empty. No drift, so staging proceeded; every snapshot row and the earlier `GH-4-PRE` no-drift result are unchanged | `.pdca/open-issues-r5/rv1/item-4-demucs/logs/86-act-reanchor-r2n1.log`, `scripts/act_reanchor_verify.py`
- 2026-11-12T17:06:17Z | ACT | r2 | n=1 | **`AC-4.3(2)` commit performed — one atomic commit** — `git add -- <8 paths>` (exactly the item delta, never `git add -A`; the staged list was checked = 8 before committing), then commit **`d042c2628edb52deab42118e18ac69974e5ad75d`** (`d042c26`), parent `8b03cfda15a485b115f8b694d07c03e98fcdf685`, **8 files changed, `+3767/−4`**, subject `#4: HTDemucs 4-stem separation with audio-separator 0.48.0 parity`, footer `Refs #4` (no `Closes`/`Fixes`), identity `Alexey Shirshov <alex.shirshov@vk.com>` via `git -c` (no `git config` write); no `--amend`, no push, no branch switch, no tag, no rebase | `git show --stat --oneline d042c26`; `.pdca/open-issues-r5/rv1/item-4-demucs/logs/87-act-commit-verify.log`
- 2026-11-12T17:06:17Z | ACT | r2 | n=1 | **commit verified after creation** — `HEAD d042c26`; `git show --name-only --format='' HEAD` = exactly the 8 item paths (`file_count=8`); **foreign-path scan: none** (no `.pdca/**`, no `.openidea/**`, no weight, audio/fixture, log or status path); `git diff --name-only HEAD -- <8 paths>` **empty**; `git diff --cached --name-only` **empty**; post-commit re-hash of the worktree against `snapshot-sha256.txt` **8/8** (no product byte moved); **not pushed** (`origin/main` is still `8b03cfd`, `git merge-base --is-ancestor d042c26 origin/main` exit `1`) | `logs/87-act-commit-verify.log`
- 2026-11-12T17:06:17Z | ACT | r2 | n=1 | **Notice (cosmetic, no identity drift):** `core.autocrlf=true` with no `.gitattributes`; `git add` warned “LF will be replaced by CRLF the next time Git touches it” for six paths, yet `git diff --name-only HEAD -- <8 paths>` is empty and the worktree still hashes 8/8, so the committed blobs equal the accepted worktree bytes in Git's comparison. Recorded, nothing amended | `logs/87-act-commit-verify.log`
- 2026-11-12T17:06Z | ACT | r2 | n=1 | **record stays `open` — closure blocker (explicit):** the mandatory `pdca-check` `retrospective` report was not supplied with this ACT brief, so §Lessons learned stays `pending` and the record is **not** marked `completed`. **GitHub `#4` closure = external residual handed to the calling driver** — no PDCA phase performs a GitHub write in this collection, the issue is **not** recorded as closed, and no fallback analysis was written. Next authorized action: `pdca-check` in `retrospective` mode for cycle `issue-4-r5` (`N=1`, `r=2`, `n=1`), then persist its raw report verbatim | §ACT record, §Lessons learned
- 2026-11-12T17:08Z | ACT | r2 | n=1 | **record lint after the ACT write:** `.pdca/status/issue-4-r5.md` exit **`0`** (`status-lint: OK (.pdca\status\issue-4-r5.md: 5 tables, 29 rows)`) and `.pdca/status/collection-open-issues-r5.md` exit **`0`** (`status-lint: OK (.pdca\status\collection-open-issues-r5.md: 2 tables, 7 rows)`); both re-linted on their final bytes with the same exit codes, runs appended to `item-4-demucs/lint.txt` | `.pdca/scripts/status_lint.py`
- 2026-11-12T17:13Z | ACT | r2 | n=1 | **mandatory `pdca-check` retrospective recorded verbatim** — the raw report for cycle `issue-4-r5` (`N=1`, `r=2`, `n=1`), `mode: retrospective`, routed by the calling driver, is persisted in §Lessons learned: 26-line report block (3 lessons with categories, "next time" and evidence pointers; limits and uncertainty; closing signal `retrospective_complete`) recorded unchanged in wording, punctuation and line breaks, with only its two headings shifted one level deeper so the block nests inside this section; the dispatch's own notes stay outside the quoted block; the previous `pending` content is superseded, kept as history. No verdict, criteria result, counter, attempt or new verification; nothing re-run | `.pdca/status/issue-4-r5.md` §Lessons learned
- 2026-11-12T17:13Z | ACT | r2 | n=1 | **item `#4` finalized — `phase: ACT · record: completed · outcome: accepted`** (the `accepted` outcome is `CHECK 3`'s `r2`/`n1` verdict, not a new one); §ACT record completed with the retrospective record, the completion and the handoff; item status `done` / `finalized`, `plan_revision 2`, attempts `1`; counters `N=1 · r=2 · n=1/3` unchanged. **GitHub `#4` closure = external residual handed to the calling driver — NOT performed by any PDCA phase and never recorded as "issue closed"**; no GitHub read or write in this dispatch | `.pdca/status/issue-4-r5.md` §ACT record, §Current state
- 2026-11-12T17:14Z | ACT | r2 | n=1 | state after the finalization write: `HEAD d042c2628edb52deab42118e18ac69974e5ad75d` on `main`, index **empty**, `git diff --name-only HEAD -- <8 item paths>` **empty**, not pushed (ancestor check against `origin/main = 8b03cfd` exits `1`); `git status --porcelain` four lines — the two status records plus the pre-existing untouched `?? .openidea/` and `?? .pdca/status/collection-open-issues-r3.md`; no `git add`/commit/push/branch switch/amend/tag/rebase | `git status --porcelain`, `git rev-parse HEAD`
- 2026-11-12T17:19Z | ACT | r2 | n=1 | **final `status_lint.py` on the final bytes:** this record exit **`0`** (`status-lint: OK (.pdca\status\issue-4-r5.md: 5 tables, 29 rows)`) and `.pdca/status/collection-open-issues-r5.md` exit **`0`** (`status-lint: OK (.pdca\status\collection-open-issues-r5.md: 2 tables, 7 rows)`) — exact runs in `.pdca/open-issues-r5/rv1/integration/lint.txt` and `item-4-demucs/lint.txt`; next allowed step **collection integration `CHECK`**; item `#4` has nothing left inside its cycle except the driver's GitHub closure | `.pdca/scripts/status_lint.py`, `.pdca/open-issues-r5/rv1/integration/lint.txt`
- 2026-11-12T17:26:02Z | ACT | r2 | n=1 | **GitHub `#4` closure performed** (authorized by the accepted `CHECK 3` and `AC-4.3(2)`): `gh issue close 4 --repo AlexeyShirshov/candle-roformer --reason completed --comment <file>` exit **`0`** (output `Closed issue AlexeyShirshov/candle-roformer#4`); no credential value read/printed/recorded; no product change, no commit, no push, no new attempt; counters unchanged `N=1 · r=2 · n=1/3` | `item-4-demucs/logs/89-github-issue-4-close.log`
- 2026-11-12T17:26:24Z | ACT | r2 | n=1 | **GitHub `#4` post-closure observation (unauthenticated, read-only):** `HTTP 200`, `state=closed`, `state_reason=completed`, `closed_at=2026-11-12T17:26:06Z`, `comment_count=1` (comment id `6830882059`, `2026-11-12T17:26:05Z`, body sha256 `da4be8f5…4167`, byte-identical to the text sent), timeline **1 closed / 0 reopened**; the 17:26:13Z read returned a stale `state=open` body and is recorded as such; raw + summary persisted; **`AC-4.3(2)` now performed in both components** (commit `d042c26` + closure) | `item-4-demucs/github/issue-4-observation-closed.json`, `item-4-demucs/logs/90-github-issue-4-observation-closed.log`
- 2026-11-12T17:33Z | ACT | r2 | n=1 | **record consistency after the closure:** item `#4` stays `done` / `finalized` (outcome `accepted`), `plan_revision 2`, attempts `1`; the external residual clause is superseded in this section; §Git state refreshed (`HEAD d042c26`, index empty, `d042c26` unpublished, the three earlier item commits published through `refs/heads/main`); no new count, no new verification | `.pdca/status/issue-4-r5.md` §ACT record, §Git state
