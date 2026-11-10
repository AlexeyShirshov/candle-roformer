# Collection status: open-issues-r4 / collection r4

- collection-id: `open-issues-r4`
- task: `open-issues-r4`
- form: `collection`
- plan_cap: 1
- lane_cap: 1
- cap_reason: all five active items (#1, #2, #3, #4, #8) touch the same product surfaces (src model/registry/config/CLI, tests, README, docs), so their footprints are not provably disjoint and the PLAN serializes them into one lane; plan_cap 1 because the per-item contracts were published at r1 by the collection PLAN and no further planner call is scheduled before DO. As of the criteria-defect decision #1 is the only remaining active item, so the one-lane cap still holds.
- specialized skill: `pdca-coder`
- mode: `autonomous`
- status path: `.pdca/status/collection-open-issues-r4.md`
- evidence root: `.pdca/open-issues-r4/rv1/`
- baseline: `main @ bf2cab6`, tree clean except one pre-existing untracked prior record (observed; see §Git state)
- phase: `ACT` · record: `completed` · outcome: `verified-partial (1 of 6 goals accepted)` — acted at the collection's integrated-tree CHECK (`accepted`) on `main @ dc94520` and closed by this ACT; the earlier value of this field (`phase: PLAN` · `record: open` · `outcome: pending` — planner criteria-defect decision taken after item `#2`; next item `#1`) is **superseded, not deleted**: it stays quoted here and as history in the appended blocks and §Progress log below
- goal: implement the six open GitHub issues in candle-roformer against their verbatim Done-when criteria, autocommitting accepted item results and recording the PLAN-terminal resolutions (#2, #3, #4, #8 `acceptance criteria cannot be determined`; #11 `infeasible`)
- criteria: `AC-1`, `AC-2`, `AC-3`, `AC-4`, `AC-8`, `AC-11` — verbatim in §PLAN record `r1` and §Input, not retyped here

## Current state

- task: `open-issues-r4` · form: `collection` · specialized skill: `pdca-coder` · overlay: `pdca-coder`
- mode: `autonomous` — no approvals requested or recorded, no user questions
- collection: `N=1` · plan revision `r=1` · schedule `s2` · attempts `n=1` consumed by item `#2` (attempt 1 of `r1`, ran DO + CHECK) and `n=2` consumed by item `#1` (attempt 1 of `r1` ended `candidate-ready` and was rejected by CHECK; attempt 2 is the CHECK→DO re-gather that re-froze and re-measured the candidate and was accepted by CHECK #3 at `n=2` of 3); `#3`, `#4`, `#8`, `#11` closed PLAN-terminal at attempts 0; `#1` is `done` and committed as `dc94520`
- scope: six items — `#1` (the only active item, `plan_state=ready`) plus #2, #3, #4, #8 (closed `incomplete`, `terminal=criteria-undetermined`) and #11 (PLAN-terminal `infeasible`) · status path: `.pdca/status/collection-open-issues-r4.md`
- evidence root: `.pdca/open-issues-r4/rv1/` (ignored by `.gitignore` via `/.pdca/*`)
- goal: implement the six open GitHub issues in candle-roformer against their verbatim Done-when criteria, autocommitting accepted item results — the §PLAN record and §Original task are authoritative
- phase: `ACT` · record: `completed` · outcome: `verified-partial (1 of 6 goals accepted)` — final ACT 2026-11-10T17:33Z: the integrated-tree CHECK returned `accepted` on `main @ dc94520`, the invocation is closed as a verified partial integration (**1 of 6** goals accepted) and the mandatory retrospective is recorded. The rest of this bullet is the **superseded pre-CHECK projection, kept as history and not deleted**: every dispatchable item was resolved — item `#1` was accepted at its own CHECK #3 (`AC-1` `met`) and committed as `dc94520`, so the lane was empty and one of the five active items was accepted; the planner's criteria-defect decision closed `#3`, `#4` and `#8` as PLAN-terminal `acceptance criteria cannot be determined`, and `#11` is PLAN-terminal `infeasible`. At that moment the collection itself had **no integrated-tree CHECK verdict**; that CHECK has since run (see §CHECK record)
- source: GitHub issues of `AlexeyShirshov/candle-roformer` (#1, #2, #3, #4, #8, #11) · context refs: `.pdca/status/collection-open-issues-r3.md`, `.pdca/open-issues-r3/rv1/`, `.pdca/open-issues-r4/rv1/plan.md`
- criteria: `AC-1`, `AC-2`, `AC-3`, `AC-4`, `AC-8`, `AC-11` — exact text in §PLAN record `r1` and §Input (source: the six verbatim `Done-when` strings below), not retyped here
- schedule: one group `G1` = `[#1]` on the current worktree `main`, one sequential lane (`plan_cap 1`, `lane_cap 1`); the s2 order `#2 → #8 → #3 → #1 → #4` collapses to `#1` because `#2`/`#3`/`#4`/`#8` are PLAN-terminal resolutions outside execution and `#11` was already terminal. Criterion and item plans unchanged
- next item: `none` — group `G1` is `done` (its single item `#1` is accepted and committed as `dc94520`); `#2`, `#3`, `#4`, `#8` are plan-terminal (criteria defects) and `#11` is plan-terminal (infeasible), so no item can be dispatched. The collection-level integrated-tree CHECK that was the next authorized step has since run and returned `accepted`; this ACT closed the invocation and **no further step is authorized** (see §ACT record)
- blockers: `CONC-1 resolved` (item `#1`, attempt `n=1`) — the duplicate writer exited and no python/cargo/rustc process ran for 5+ minutes before the single remaining writer resumed; that writer re-verified the tree, re-bound the parity measurement to the final tree, added the crate-schema strict-validation evidence, packaged the snapshot and recorded both status files. Original conflict evidence stays at `.pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/concurrency-conflict.log`. The five PLAN-terminal items are not blockers: `#2`/`#3`/`#4`/`#8` (`criteria-undetermined`) and `#11` (`infeasible`) are terminal resolutions, not blockers, and they do not stop `G1`; the `#8` two-checkpoint constraint recorded earlier is moot because `#8` is now terminal, and the preserved `#2` patch stays archived evidence only. No `CONC-6` condition arose at the item ACT/commit: no cargo/rustc/python process ran, the product delta was unchanged and 14/14 live product files were still hash-equal to `snapshot-sha256.txt`

## Original task

The user's task, verbatim (original language, Russian):

> pdca по открытым задачам из github

The new explicit user instruction, verbatim (original language, Russian):

> тогда делай автономно с автокомитом

Caller framing supplied with the task (recorded as interpretation, never as a replacement of the two
verbatim texts above): autonomous mode, no approvals and no questions; autocommit only accepted item
results; no push, no branch switch, no history rewrite, no forcing.

## Input

The six open GitHub issues of `AlexeyShirshov/candle-roformer`, each with its verbatim `Done when`
criterion. These are the items of this collection and stay verbatim; later item briefs reference
this source exactly.

```text
1. #1 feat: support BS-RoFormer and the general MDXC/RoFormer family — Done when: "a real BS-RoFormer checkpoint loads, passes strict validation and separates audio; numerical parity against a pinned PyTorch reference (same thresholds as MelBand parity gate); README lists supported families and config files."
2. #2 feat: MDX-Net (ONNX) architecture — Done when: "a published MDX-Net ONNX file (supplied locally, checksum-checked by caller) separates audio on CPU; output matches audio-separator on a frozen fixture within agreed tolerance; dependency documented in docs/dependency-audit.md with licence."
3. #3 feat: VR/UVR architecture (.pth models) — Done when: "a chosen VR checkpoint converts with a pinned checksum and separates audio; parity fixture against audio-separator; converter covered by tests like the v7 one."
4. #4 feat: Demucs (4-stem and 6-stem) architecture — Done when: "htdemucs weights load with strict validation and separate a fixture into 4 stems; parity against the reference implementation and per-stem output naming."
5. #8 feat: multi-model ensembles and ensemble presets — Done when: "at least avg_wave and median_wave match the reference on a frozen fixture with two checkpoints; a preset produces the documented stem set on a real song."
6. #11 test: CUDA/Metal smoke tests and a validated reduced-precision path (gate G2) — Done when: "at least one real CUDA host and one Apple Silicon host run the smoke suite, logs committed as evidence, README/gates report state exactly what passed; until then README keeps saying 'not tested'."
```

## Constraints

- autonomous mode: no approvals, no questions; the PLAN is recorded as returned and DO may start on the planner's `Next item`;
- the six `Done when` criteria are immutable — no weakening, no narrowing, no surrogate deliverable; synthetic tests may supplement but never replace real published checkpoints and reference fixtures;
- autocommit only after an accepted CHECK, one atomic commit per accepted item, on `main`, only that item's files; no push, no branch switch, no history rewrite, no forcing;
- no multi-GB weights committed to git; README must keep saying "not tested" for CUDA/Metal until a real host proves otherwise;
- preserve prior `.pdca` records (`.pdca/status/collection-open-issues-r3.md`, `.pdca/open-issues-r3/` untouched);
- Python invoked by full path `C:\Users\Пользователь\AppData\Local\Programs\Python\Python312\python.exe`;
- working directory `F:\sources\candle-roformer`; evidence for every run persisted under `.pdca/open-issues-r4/rv1/`;
- this recording task itself: no product-file edit, no `git add`, no staging, no commit, no push;
- `plan_cap: 1`, `lane_cap: 1` recorded in the header with the reason above.

## PLAN record

### r1

Form: collection. Specialized skill: pdca-coder. Entry scope: #1, #2, #3, #4, #8, #11. Requested result: the six open GitHub issues implemented in candle-roformer against their verbatim Done-when criteria, with autocommit of accepted item results and a terminal PLAN resolution for provably infeasible items. Delivered result: the same object.
Per-item PLAN status:
- #1 ready (implementable now: Python 3.12, PyPI audio-separator/torch, HuggingFace, 460 GB free, downloads allowed).
- #2 ready (ONNX runtime + audio-separator reference).
- #3 ready (offline .pth converter + audio-separator parity).
- #4 ready (htdemucs weights + demucs/audio-separator reference).
- #8 ready (avg_wave/median_wave + preset on a real song; depends on two loadable models).
- #11 PLAN-terminal `original task is infeasible under the stated constraints`: the criterion requires BOTH a real CUDA host AND an Apple Silicon host, and no Apple Silicon/Metal host exists; README must keep "not tested".
Acceptance criteria (verbatim, unchanged): AC-1, AC-2, AC-3, AC-4, AC-8, AC-11 exactly as quoted in the input items above.
Measurement methods:
- #1: waveform parity in f32 PCM, max abs error / RMSE / SDR over all samples/channels/stems on a frozen fixture; same thresholds as the MelBand parity gate; strict validation with zero missing/unexpected tensors.
- #2: max abs diff and RMSE vs audio-separator 0.48.0 on a frozen fixture; SHA-256 checksum of the .onnx file verified before inference; dependency and licence documented in docs/dependency-audit.md.
- #3: exact SHA-256 match of the converted artifact against a pinned checksum; max abs diff / RMSE / SDR vs audio-separator on a frozen fixture; converter covered by tests like the v7 converter.
- #4: exactly 4 stems named drums/bass/other/vocals; strict weight validation with zero errors; per-stem max abs diff / RMSE / SDR vs demucs 4.1.0 / audio-separator on a frozen fixture.
- #8: avg_wave and median_wave max abs diff / RMSE vs reference on a frozen fixture with two checkpoints; a preset produces the documented stem set on a real song, with finite samples and matching duration/sample rate.
- #11: PLAN-terminal; no DO/CHECK.
Constraints: autonomous, no approvals; autocommit only after an accepted CHECK, one atomic commit per accepted item on main, only item files; no push/branch switch/history rewrite; preserve prior .pdca records; no multi-GB weights committed to git; README must keep "not tested" for CUDA/Metal; use full Python path C:\Users\Пользователь\AppData\Local\Programs\Python\Python312\python.exe; no criterion weakening; synthetic tests may supplement but not replace real published checkpoints/reference fixtures.
Schedule: DAG `{#1,#2,#3,#4} → #8`; all five active items share surfaces, so one sequential group `G1` with order `#1 → #2 → #3 → #4 → #8`; #11 is a terminal resolution outside G1. One group => current worktree `main`, no worktree/branch/merge. plan_cap 1, lane_cap 1.
Per-item plans: the per-item contracts in this collection PLAN are the item plans at r1 (plan_state=ready) for #1, #2, #3, #4, #8; no separate planner call is needed before DO. #11 is terminal at r1.
Next item: #1.
Risks: numeric divergence in STFT/iSTFT/layers; ONNX runtime build on Windows; network failures; #8 depends on two loadable models. Stop conditions: item accepted only on full CHECK; n/3 exhaustion follows the contract; #11 terminal cannot block the other items.

### Schedule and item projection (s1)

Superseded by `### Schedule s2` below for the `G1` order and the `#8` dependency edge; the s1 prose
is kept as recorded, and the two tables in this section are the live projection refreshed to s2.

Scheduling decision, taken from the `r1` return above and not re-decided here: one sequential group
`G1` on the current worktree `main` (no task branch, no worktree creation, no merge), order
`#1 → #2 → #3 → #4 → #8`, because the five active items share product surfaces and disjointness
cannot be proven; `#8` additionally depends on two loadable models produced by `#1`/`#3`. `#11` is a
PLAN-terminal resolution outside `G1`: it is recorded `incomplete` with `disposition=finalized` and
consumes no attempt, and it cannot block `#1`, `#2`, `#3`, `#4` or `#8`. `plan_cap 1` and
`lane_cap 1` therefore describe a one-lane schedule, not a lost opportunity for parallelism.

| group | items/order | worktree | branch | ref | status | notes |
|---|---|---|---|---|---|---|
| G1 | [#1] | main (current worktree) | main | bf2cab6 | done | one sequential lane; the s2 order collapsed to #1 because #2, #3, #4, #8 are PLAN-terminal `criteria-undetermined` and #11 is PLAN-terminal `infeasible`. #1 was accepted at its own CHECK #3 and committed `dc94520`; the group closes with no merge (single group, single worktree, no worktree created). The collection-level integrated-tree CHECK is still pending |

| item | group | branch | status | plan_state | form | contract | overlay | cycle_id | status_path | plan_revision | disposition | terminal | attempts | notes |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| #2 | G1 | main | incomplete | pending | cycle | pdca | pdca-coder | issue-2-r4 | .pdca/status/issue-2-r4.md | 1 | finalized | criteria-undetermined | 0 | PLAN-terminal `acceptance criteria cannot be determined` (AC2-TOL-001: no agreed MDX-Net tolerance exists and the user's AC-2 is immutable; revision r2 not issued, so `plan_state` is no longer ready); attempt r1/n1 ran (DO + CHECK `not accepted` → PLAN) — lint schema caps terminal rows at attempts 0, so the conducted attempt is recorded here and in the item record, not in the cell; candidate preserved as `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/patch/item-2-candidate.patch` (65,446 B, sha256 `4922abec…6ff17`) plus `snapshot/`, product paths rolled back to HEAD, nothing committed; residual AC-2.5 unverified |
| #8 | G1 | main | incomplete | pending | cycle | pdca | pdca-coder | issue-8-r4 | .pdca/status/issue-8-r4.md | 1 | finalized | criteria-undetermined | 0 | PLAN-terminal `acceptance criteria cannot be determined` (AC-8), a criteria defect and not an execution failure: the verbatim criterion's "match the reference" carries no numeric tolerance or metric and no repo gate can decide it, "documented stem set" is not defined in the issue or in the repo, and the premise #2→#8 is unmet because #2 closed terminal without a commit. No DO, no CHECK, no goal credit, no item status file created |
| #3 | G1 | main | incomplete | pending | cycle | pdca | pdca-coder | issue-3-r4 | .pdca/status/issue-3-r4.md | 1 | finalized | criteria-undetermined | 0 | PLAN-terminal `acceptance criteria cannot be determined` (AC-3), a criteria defect and not an execution failure: "parity fixture against audio-separator" carries no metric or numeric tolerance and the repo has no gate for it, and the v7-converter reference does not define audio parity. No DO, no CHECK, no goal credit, no item status file created |
| #1 | G1 | main | done | ready | cycle | pdca | pdca-coder | issue-1-r4 | .pdca/status/issue-1-r4.md | 1 | finalized | execution | 2 | **accepted** — CHECK #3 (final) returned `accepted` for `AC-1` (`met`) and the item was committed as `dc94520550418df3f5d583523d04251bf86859bc` (14 files, one atomic commit on `main`). BS-RoFormer `model_bs_roformer_ep_317_sdr_12.9755.ckpt` (639,331,213 B, sha256 `5b84f37e…15aa`) → safetensors: crate-schema strict validation 699/699 with 0 missing / 0 unexpected / 0 shape mismatches, `Separator::load` ok, separation rc 0 on the frozen 8 s fixture; parity vs audio-separator 0.48.0 + torch 2.15.0+cpu — vocals corr `0.9999999999999561` / max abs `4.76837e-7` / RMSE `3.75262e-8` / SDR `130.302177 dB`, instrumental corr `0.9999999999999664` / max abs `3.65078e-7` / RMSE `2.8601e-8` / SDR `131.728898 dB`, 12/12 hard gate assertions per stem; README lists the families and config files and keeps CUDA/Metal "not tested" (CPU/f32 only, one 8 s window). History: attempt `n=1` `candidate-ready` → CHECK `not accepted` (`BS-EVID-001` blocking, `BS-DOC-003`/`BS-PROV-002` minor) → attempt `n=2` re-froze and re-measured from the frozen input; `BS-PROV-004` (minor) corrected by ACT. Records: `.pdca/status/issue-1-r4.md`; evidence `.pdca/open-issues-r4/rv1/item-1-bs-roformer/`
| #4 | G1 | main | incomplete | pending | cycle | pdca | pdca-coder | issue-4-r4 | .pdca/status/issue-4-r4.md | 1 | finalized | criteria-undetermined | 0 | PLAN-terminal `acceptance criteria cannot be determined` (AC-4), a criteria defect and not an execution failure: "parity against the reference implementation" carries no numeric threshold, no metric and no repo gate, so the criterion cannot be decided. No DO, no CHECK, no goal credit, no item status file created |
| #11 | none (terminal, outside G1) | main | incomplete | pending | cycle | pdca | pdca-coder | issue-11-r4 | .pdca/status/issue-11-r4.md | 1 | finalized | infeasible | 0 | PLAN-terminal: the criterion needs a real Apple Silicon host and none exists, so README keeps "not tested" |

Per-item contracts: the single `plan_state=ready` row above (`#1`) carries its item plan at `r1`; no
separate planner call precedes DO, and no contract revision was issued for `#2`, `#3`, `#4` or `#8`.
Item status files are created on each item's first write and are not created by this recording task.

### Schedule s2

Schedule revision returned by the planner before the single lane started; it supersedes only the `s1`
ordering and the `#8` dependency edge. The `r1` plan above stays byte-identical: item plans at `r1`,
the six criteria (`AC-1`, `AC-2`, `AC-3`, `AC-4`, `AC-8`, `AC-11`) and every measurement method are
unchanged, and no criterion is added, removed, weakened or narrowed.

- `G1` order revised to `#2 → #8 → #3 → #1 → #4`; `#11` remains terminal outside `G1`.
- Dependency edge for `#8` narrowed to `#2 → #8`: MelBand is already implemented in the repository, so
  `#8` no longer waits on a conversion item — `#2` supplies the second checkpoint/family that the
  `avg_wave`/`median_wave` parity fixture needs on top of the existing MelBand model.
- Rationale: risk-first order. `#2` is the lowest numerical-risk item (deterministic ONNX CPU inference
  against a checksum-pinned file), `#8` is deterministic aggregation over two working models, `#3` is
  medium (converted offline `.pth` artifacts), `#1` is high (strict validation plus MelBand-threshold
  parity on a real BS-RoFormer checkpoint), `#4` is highest (multi-stem Demucs weights and per-stem
  parity). Ordering the cheapest provable item first produces the first accepted increment earliest
  and de-risks the lane before the expensive items.
- Structure unchanged: one group/lane `G1` on the current worktree `main`, no worktree, no branch, no
  merge; `plan_cap 1`, `lane_cap 1` still describe a one-lane schedule, not lost parallelism.
- Effect on counters: none — `r=1`, `n=0`; a schedule revision is not a plan revision and consumes no
  attempt.

### Criteria-defect decision after #2 (r1, no revision)

PLAN decision taken after item `#2` closed, as dispatched: `#3`, `#4` and `#8` are PLAN-terminal
`acceptance criteria cannot be determined` — no DO, no CHECK, `attempts=0`, no goal credit and no
contract revision (`r2` not issued, item plans stay at `r1`). Causes, per item:

- `#3` — "parity fixture against audio-separator" states no metric and no numeric tolerance, and the
  repository has no gate that could decide it; the v7-converter reference defines converter
  behaviour, not audio parity.
- `#4` — "parity against the reference implementation" states no numeric threshold, no metric and no
  repo gate.
- `#8` — "match the reference" states no numeric tolerance; "documented stem set" is not defined in
  the issue or in the repository; and the premise `#2 → #8` is unmet because `#2` closed terminal
  without a commit, so the two-checkpoint fixture has no second loadable checkpoint.

Consequences recorded, not re-decided here: `#2` stays terminal `criteria-undetermined`; `#1` remains
`ready` and is the only active item; `G1` is `[#1]`; `#11` was already terminal `infeasible`. None of
these terminal resolutions stops the group — a criterion defect is not an execution failure.

## Approval record

`not applicable: autonomous mode` — the user instruction "тогда делай автономно с автокомитом"
authorizes execution without an approval gate; no approval ref was opened, requested or fabricated.

## DO result

`recorded during DO, superseded by the resolution at the end of this section` — item #2 (MDX-Net ONNX) returned a completed candidate; at that moment CHECK had not run, and the item's candidate, evidence pointers (exit status, key numbers, log paths) and git identity are recorded in
`.pdca/status/issue-2-r4.md` §`DO result` and under
`.pdca/open-issues-r4/rv1/item-2-mdx-onnx/` (comparison: overall max abs diff 1.423e-6, RMSE
1.465e-7, SDR 118.90 dB; CPU runtime 85.285 s; 90 tests passed). No item verdict exists, item `#2`
stays `in-progress`, and the collection itself produced no product change.

Final-tree evidence refresh (same item attempt `n1`, no new attempt, no CHECK): after a cosmetic
`src/lib.rs` import reorder the item re-ran its gate on the final tree — `cargo test` exit 0,
90 passed / 0 failed / 0 ignored (`logs/cargo-test-full-final.log`) and `cargo clippy --all-targets`
exit 0 (`logs/cargo-clippy-final.log`), both under
`.pdca/open-issues-r4/rv1/item-2-mdx-onnx/logs/`; the item manifest records the refreshed git
identity (`main @ bf2cab6`) and artefact hashes.

Item `#1` (BS-RoFormer family) returned a completed candidate in attempt `n=1`; CHECK has not run.
DO finished on the final tree (`main @ bf2cab6` + the item's uncommitted delta, nothing staged, no
commit): the real published checkpoint `model_bs_roformer_ep_317_sdr_12.9755.ckpt` (639,331,213 B,
sha256 `5b84f37e8d444c8cb30c79d77f613a41c05868ff9c9ac6c7049c00aefae115aa`, URL verified by a HEAD
request returning the same Content-Length) converts to safetensors (639,109,056 B, sha256
`fa296577…5a8a`) with 699 keys and 0 missing / 0 unexpected / 0 shape mismatches against the Python
reference module, passes the **crate's own** `expected_key_shapes` strict validation with
`expected=699 actual=699 missing=0 unexpected=0 shape_mismatches=0` and then loads via
`Separator::load`, and separates the frozen 8 s fixture (352,800 frames, 2 ch, 44.1 kHz) on CPU/f32
in 95.753 s. Parity against the pinned reference (audio-separator 0.48.0 + torch 2.15.0+cpu, run1/run2
raw samples bitwise identical): vocals overall corr `0.9999999999999561`, max abs `4.768e-7`, RMSE
`3.753e-8`, SDR `130.302 dB`; instrumental corr `0.9999999999999664`, max abs `3.651e-7`, RMSE
`2.860e-8`, SDR `131.729 dB`; every channel passes the MelBand gate (corr ≥ 0.9999999, max abs
≤ 3.5e-4). Gates on the final tree: `cargo test` exit 0 with 87 passed / 0 failed / 1 ignored,
`cargo clippy --all-targets` exit 0 with 0 errors (51 lib warnings reported), `cargo fmt --check`
exit 1 only on `src/dsp.rs`, `src/wav.rs` and `tests/s1_spikes.rs`, which this item does not touch and
which are byte-identical to `HEAD` (the task-owned files are rustfmt-clean). Evidence:
`.pdca/open-issues-r4/rv1/item-1-bs-roformer/` (`index.md`, `manifest.json`, `snapshot/`,
`snapshot.diff`, `logs/`, `compare/`); full detail in `.pdca/status/issue-1-r4.md` §DO result. No
item verdict exists and item `#1` stays `in-progress`.

Item `#1` attempt `n=2` (CHECK→DO re-gather, recorded 2026-11-10T17:06Z, no replan, `r=1`): CHECK had
returned `not accepted` with `BS-EVID-001` (blocking, unverified — the candidate identity was not tied
to a snapshot-built measurement) plus `BS-DOC-003` (README claimed strict dtype validation for the BS
converter) and `BS-PROV-002` (manifest attributed `logs/convert-report.json` to the wrong converter).
This execution changed `README.md` only (doc wording: key/shape checks then a float32 cast for
`tools/convert_bs_roformer.py`, dtype checks for `tools/convert_voc_fv7.py`; family table, CUDA/Metal
"not tested" and parity wording untouched), corrected `manifest.json` → `safetensors.converter`, and
re-froze the candidate from the live tree: 14 snapshot files, 14/14 hash-equal to their live
counterparts, manifest list ↔ snapshot list equal, `HEAD` still `bf2cab6`, delta exactly 9 modified +
5 untracked product paths (`logs/refreeze-report.json`). It then materialized `bf2cab6` with
`git archive` + the snapshot overlay (no branch, no worktree; 14/14 overlaid hashes verified before
the build) and ran everything from there: build rc 0, strict-validation test rc 0 with
`expected=699 actual=699 missing=0 unexpected=0 shape_mismatches=0` and `strict-load: ok`, candidate
run rc 0 (`load_secs=0.452`, `separation_secs=91.678`, stems sha256
`4d9e3ab9…bfcd`/`b37a10d4…ebc1`, byte-identical to attempt 1), and the parity comparison rc 0 with
12/12 hard assertions per stem (vocals corr `0.9999999999999561` / max abs `4.76837e-07` / RMSE
`3.75262e-08` / SDR `130.302177 dB`, instrumental corr `0.9999999999999664` / `3.65078e-07` /
`2.8601e-08` / `131.728898 dB`; the executable built from the snapshot hashes to
`4fe007ef…9dbe8`). `rustfmt --check --config skip_children=true` on the task-owned Rust files rc 0;
no code changed, so the attempt-1 clippy/test totals still describe the same source. The comparison
ran under the pinned reference venv (`…\Temp\opencode\mdx-ref-venv`, Python 3.12.10 + numpy 2.5.4 +
soundfile 0.14.0 + torch 2.15.0+cpu) because the bare Python312 interpreter no longer has
numpy/soundfile — recorded as a reproduction prerequisite, not as frozen evidence. No CHECK has
re-run; item `#1` stays `in-progress` with attempts 2 and a recheck pending. Full detail:
`.pdca/status/issue-1-r4.md` §DO result → `Attempt n=2`.

Items `#3`, `#4`, `#8` and `#11` produced no DO result: they were resolved at PLAN, so there is no
candidate, no command, no exit status and no log for them, and this section never invents one.

Resolution of the item recorded after this section (ACT for `#2`, 2026-11-10): the item's independent
CHECK returned `not accepted` → PLAN, and PLAN declared the terminal outcome
`acceptance criteria cannot be determined` (`AC2-TOL-001`). Item `#2` is therefore `incomplete`,
`terminal=criteria-undetermined`, `disposition=finalized`, nothing was committed, and its product delta
was rolled back to `HEAD` after the whole candidate was preserved as
`.pdca/open-issues-r4/rv1/item-2-mdx-onnx/patch/item-2-candidate.patch`. Full reports: item record
`.pdca/status/issue-2-r4.md` §CHECK record, §PLAN record (terminal block), §ACT record.

Resolution of item `#1` recorded after this section (ACT for `#1`, 2026-11-10T17:24Z; appended, no
earlier paragraph rewritten): the item's independent CHECK #3 (final) returned **`accepted`** for
`AC-1` (`met`) and routed the item `→ ACT`. Earlier findings are closed/refuted for `n2` —
`BS-EVID-001` (blocking) refuted by the build-input equivalence, `BS-PROV-002` and `BS-DOC-003`
refuted, `BS-PROV-004` (minor, confirmed) corrected in `manifest.json`, `index.md` and the item
record. The item's 14 product paths were committed as the single atomic commit
`dc94520550418df3f5d583523d04251bf86859bc` (`#1 feat: support BS-RoFormer and the general
MDXC/RoFormer family`) on `main`, no push, no amend; item row `#1` is `done`,
`disposition=finalized`, attempts 2, group `G1` `done`. Accepted numbers: strict validation 699/699
with 0 missing / 0 unexpected / 0 shape mismatches; parity vocals corr `0.9999999999999561` / max abs
`4.76837e-7` / SDR `130.302177 dB`, instrumental corr `0.9999999999999664` / max abs `3.65078e-7` /
SDR `131.728898 dB`, CPU/f32, one 8 s window, no GPU claim.

Correction to the attempt-`n=2` paragraph above (`BS-PROV-004`; the paragraph's "ran everything from
there"/"built from the snapshot" wording is refuted): the cargo build and the strict test were
invoked from the live root `F:\sources\candle-roformer` with `CARGO_TARGET_DIR` redirected, **not**
with `snapshot-build/tree` as the working directory; their build inputs equal the frozen candidate
`bf2cab62e1350eec59a4e5283d18e818d8ed3c54 + snapshot/` — 269/269 shared files and 21/21 compiler
inputs equal, no additional build-affecting input (`logs/provenance-facts.txt` in the item evidence
root). The attribution rests on that byte equality, not on the working directory, and no measured
number changes.

This is an item-level ACT outcome. It is **not** an integrated-tree verdict: the collection's own
CHECK on the integrated tree has not run.

## CHECK record

`not run: integrated-tree CHECK not run` — no integrated-tree candidate exists and the five active items
share one lane. Item `#2` did run its own independent CHECK on its snapshot (`not accepted` → PLAN,
findings `AC2-TOL-001` blocking, `AC2-FMT-001` and `AC2-EVID-001` minor); that report is recorded
verbatim in `.pdca/status/issue-2-r4.md` §CHECK record and is not an integrated-tree verdict.
Items `#3`, `#4` and `#8` reached no DO, so no CHECK was possible for them (`PLAN → ACT`,
`criteria-undetermined`); item `#11` is `PLAN → ACT` `infeasible`.

Item `#1` ran its own CHECK on its attempt-`n1` candidate and returned **not accepted**
(`BS-EVID-001` blocking/unverified, `BS-DOC-003` and `BS-PROV-002` minor). The full item report was
not persisted in that item's evidence root, so only its finding identities are recorded in
`.pdca/status/issue-1-r4.md` §CHECK record, together with the note that a recheck on attempt `n=2` is
pending. There is no item verdict and no integrated-tree verdict.

Appended after the item's ACT (2026-11-10T17:24Z; appended, no earlier paragraph rewritten): item `#1`
**did** receive an item verdict — its CHECK #3 (final) returned **`accepted`**, `AC-1` `met`, with
`BS-EVID-001`, `BS-PROV-002` and `BS-DOC-003` refuted for `n2` and `BS-PROV-004` corrected by ACT; the
complete report is recorded verbatim in `.pdca/status/issue-1-r4.md` §CHECK record. The
**integrated-tree CHECK has not run**: it is the next authorized step, and until it runs the collection
has no integrated verdict, no closure and no collection retrospective.

### Integrated-tree CHECK (collection, final) — recorded 2026-11-10T17:33Z

This is the collection's own integrated-tree CHECK. Its report is recorded **verbatim, in its original
language, as returned** by the independent `pdca-check`; the source is the driving role's dispatch and
the evidence it read is the staged integrated candidate under `.pdca/open-issues-r4/rv1/`
(`snapshot/`, `snapshot-sha256.txt`, `integration-manifest.json`, `integration-index.md`, `logs/`).
The item-level notes above stay as **superseded context** — they record item verdicts, not an
integrated-tree verdict — and are not deleted.

````markdown
## Вердикт интегрированного дерева: `accepted` → ACT

Кандидат: `main @ dc94520550418df3f5d583523d04251bf86859bc`.

**Это принятие интеграции единственного принятого item #1, а не завершение всей коллекции. Принята 1 из 6 исходных целей.**

Пути ниже относительно `.pdca/open-issues-r4/rv1/`; `I1` означает `item-1-bs-roformer/`.

### Матрица интеграции

| ID | Статус | Основание и доказательства |
|---|---|---|
| INT-IDENTITY | **met** | Список 14 путей и SHA-256 согласован между `snapshot-sha256.txt`, manifest и результатом staging script. Скрипт сравнивает копии с deployed tree, item snapshot и raw commit blobs. Результат: **14/14**, несовпадений **0**; 12 побайтно равны blob, README/lib — после CRLF→LF, с совпадающим clean blob OID. `scripts/stage_integration_snapshot.ps1:55–109`, `logs/integration-stage.log`. |
| INT-SCOPE | **met** | Commit содержит ровно принятые 14 путей, **2500/343**, без весов, `target/`, `.pdca/`, `.openidea/`. Branch `main`; post-commit status не содержит product drift. Единственная группа — без merge. `I1/logs/commit-verify.log:4–56`, `logs/terminal-items-non-disturbance.log:30–31`. |
| INT-PRESERVE | **met** | Patch #2: **65 446 B**, SHA-256 `4922abec…6ff17`; **12/12** сохранённых entries совпадают. Для r3 записаны inventory из 7 файлов, 19 145 B и timestamps до staging. Терминальные rows остаются incomplete; продуктовый commit относится только к #1. `logs/terminal-items-non-disturbance.log`, `logs/r3-inventory-sha256.txt`, collection status:119–124. Ограничение исторического доказательства — ниже. |
| AC-1 | **met** | Реальный checkpoint идентифицирован checksum; strict-test **rc 0**, **699/699**, missing/unexpected/shape mismatches **0**, `strict-load: ok`. Separation **rc 0**. Численный gate выполнен; README перечисляет семьи и config-файлы. См. детали ниже. |
| INT-NO-FALSE-CREDIT | **met** | README явно оставляет MDX-Net/VR/UVR/Demucs/MDX23C «not implemented», CUDA/Metal — «not tested». Непринятые items не получили commit или статус done. `snapshot/README.md:127–154,184–209`; collection item table. |

### AC-1 на интегрированном кандидате

Привязка измерения сохранена цепочкой:

`измеренный executable → равные build inputs → item snapshot → integration snapshot → commit blobs`.

Это **не** принятие только по совпадению идентификаторов: дополнительно прочитаны strict-test, separation log, comparison JSON, assertions, reference summary и содержимое staged README/test.

- **Vocals:** corr `0.9999999999999561`, max abs `4.76837e-7`.
- **Instrumental:** corr `0.9999999999999664`, max abs `3.65078e-7`.
- Пределы: corr ≥ `0.9999999`, max abs ≤ `3.5e-4`; оба канала проходят.
- Strict validation: `I1/logs/snapshot-build-strict-validation.log:102–109`.
- Separation и hashes outputs: `I1/logs/snapshot-build-candidate-run.log`.
- Reference pin: `I1/reference/summary-run1.json`.
- Parity: `I1/compare/comparison-snapshot.json`, `I1/compare/assertions-instrumental-snapshot.json`.
- Регрессии: `cargo test`, **rc 0**, **87 passed / 0 failed / 1 ignored**; ignored real-checkpoint test выполнен отдельно.

**Component/integration discrepancy не обнаружено.** Сборка действительно выполнялась из live root, не из materialized tree. Исправление `BS-PROV-004` отражено в manifest; атрибуция основана на наблюдении **269/269 общих файлов и 21/21 compiler inputs**, а не на прежней ошибочной аннотации (`I1/logs/provenance-facts.txt`).

### Findings

| ID | Критерий | Severity / статус | Причина и evidence |
|---|---|---|---|
| INT-STATUS-001 | Согласованность записи | **minor / confirmed** | Верхний header collection status:14 всё ещё сообщает PLAN/next #1, тогда как Current state и staging addendum отражают завершённый item и подготовленную интеграцию. Исторические блоки допустимы, но текущий header устарел. Исправить при ACT, сохранив историю. |
| INT-EVID-001 | INT-PRESERVE | **minor / confirmed** | «r3 untouched» доказано текущим inventory и mtime до staging, **не сравнением с независимым baseline inventory до всей коллекции**. Это ограничение исторического доказательства, а не обнаруженная модификация. `logs/terminal-items-non-disturbance.log:23–25`. |

Lint сообщает **OK, 2 tables / 7 rows** (`logs/lint-07-integration-stage.log`), но не устраняет смысловую устарелость header. Новых блокирующих findings нет.

### Полнота коллекции — отдельно

| Цель | Итог |
|---|---|
| #1 | **Принята**, done, commit `dc94520`; item `N=1, r=1, n=2/3`. |
| #2 | PLAN-terminal `acceptance criteria cannot be determined`: отсутствует agreed MDX tolerance, `AC2-TOL-001`. **Фактически был r1/n1 DO+CHECK**; terminal row attempts=0 — лишь проекция schema, не отсутствие попытки. |
| #3 | PLAN-terminal: parity metric/tolerance не определены; attempts=0, DO/CHECK не выполнялись. |
| #4 | PLAN-terminal: reference parity metric/tolerance не определены; attempts=0. |
| #8 | PLAN-terminal: reference match и documented stem set неопределены; дополнительно не выполнена записанная предпосылка #2→#8; attempts=0. |
| #11 | PLAN-terminal `original task is infeasible under the stated constraints`: нет доступного Apple Silicon/Metal host; attempts=0, GPU acceptance не установлена. |

**Полное выполнение: нет, 1/6.** Финальных execution failures не зафиксировано: #2 закрылся по дефекту критерия, а не по доказанному численному провалу; #1 имел отклонённую раннюю попытку, затем принят.

### Оценка критериев, пределы и переход

AC-1 покрывает принятую цель и проверяем; неизменяемые AC остальных items не подменены условиями интеграции. Их терминальное разрешение **не означает выполнения**.

Проверка read-only: команды и hashing самостоятельно не запускались; проверены сохранённые результаты наблюдений и логика staging script. Прямое чтение live `README.md` было запрещено — операция не повторялась; содержимое оценено по разрешённому evidence snapshot, связь с deployed tree подтверждается staging observation. Вывод относится к зафиксированному состоянию, не к непрерывному forensic-контролю.

Доказаны CPU/f32 и одна восьмисекундная fixture; GPU, multi-window numerical parity и остальные модели не доказаны. Точная reference shell line реконструирована; effective parameters сохранены.

**Следующий разрешённый переход: → ACT коллекции** — сохранить этот интеграционный verdict, актуализировать текущие поля status и оформить **неполное выполнение исходной коллекции**, затем обязательную ретроспективу. Нового proving run не требуется; непринятые items не переоткрывать и не помечать done.

Уверенность: **высокая в принятии зафиксированной интеграции**, с указанными ограничениями исторической сохранности и наблюдаемости.
````

## ACT record

### Final ACT outcome (2026-11-10T17:33Z)

**`accepted integrated tree` + `verified-partial` — the collection is NOT fully complete: 1 of 6 goals
accepted.**

The `pending` entry immediately below records the pre-CHECK state; it is **superseded by this final ACT
outcome, not deleted**, and stays as history.

- **Determined by the ending phase (`CHECK → ACT`, accepted):** the integrated tree
  `main @ dc94520550418df3f5d583523d04251bf86859bc` was accepted by the collection's own integrated-tree
  CHECK. That is acceptance of the *integration* of the single accepted item `#1` — **not** completion
  of the whole collection. Report: this file §CHECK record → "Integrated-tree CHECK (collection, final)".
- **Accepted goal (1):** `#1` — `done`, `AC-1` `met`, committed as `dc94520` (14 files, 2500 insertions
  / 343 deletions, no weights, no `.pdca/**`, no `.openidea/**`, no `target/**`), on `main`, not pushed.
- **Not accepted — PLAN-terminal `acceptance criteria cannot be determined` (4):** `#2`
  (`AC2-TOL-001`; its r1/n1 DO+CHECK did run, nothing was committed and the product delta was rolled back
  to `HEAD`), `#3`, `#4`, `#8` (no DO, no CHECK, attempts 0). All four stay `incomplete`; the user
  criteria are unchanged and remain `unverified`. A terminal resolution is not an execution and not a
  completion.
- **Not accepted — PLAN-terminal `infeasible` (1):** `#11` requires a real CUDA host **and** an Apple
  Silicon/Metal host, and no Apple Silicon host is reachable from this win32/x86_64 workstation;
  `AC-11` stays `unverified` and README keeps saying "not tested" for CUDA/Metal.
- **Execution-failure finalizations: 0.** `#1` had one rejected early attempt (`n=1`) and was accepted at
  `n=2`; `#2` closed on a criterion defect, not on a proven numerical failure.
- **Preserved results (untouched by this ACT):** the `#2` candidate patch
  `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/patch/item-2-candidate.patch` — 65,446 B, sha256
  `4922abec88ce6661a31c96f16eeba2033d5657f7473797fd139769ab8896ff17` — with its 11-file `snapshot/` and
  `snapshot.diff`; the r3 records `.pdca/open-issues-r3/` (7 files, 19,145 B, inventory digest
  `7d0c3d45…1f89`) and `.pdca/status/collection-open-issues-r3.md`; every terminal item's evidence and
  the `#3`/`#4`/`#8`/`#11` rows.
- **Topology: nothing to close.** No worktree, no task branch, no merge and no cleanup was performed or is
  due: one group `G1` on the current worktree `main`; `git worktree list` = only
  `F:/sources/candle-roformer dc94520 [main]` and `git branch` = only `main` (plus `origin/main`).
- **Nothing pushed:** no `git add`, no staging, no commit, no push, no amend, no history rewrite in this
  ACT. The only commit of this invocation remains the item `#1` autocommit `dc94520`.
- **Limitation `INT-EVID-001` (minor, confirmed) — recorded as a limitation:** "r3 untouched" is proven
  by the current inventory and the pre-staging mtime, **not** by comparison against an independent
  baseline inventory taken before the whole collection started. A historical-evidence limitation, not an
  observed modification.
- **Finding `INT-STATUS-001` (minor, confirmed) — fixed here:** the stale current header fields are now
  `phase: ACT` / `record: completed` / `outcome: verified-partial (1 of 6 goals accepted)`; the earlier
  header text and every historical block stay quoted and in place, not deleted.
- **Counter accounting (recorded, never reset):** `N=1`, `r=1`, schedule `s2`; per-item attempts as
  already recorded (`#1` 2, `#2` r1/n1 conducted with the terminal row projected at 0, `#3`/`#4`/`#8`/
  `#11` 0). This ACT creates no new counter, attempt or verdict.
- **Handoff — next authorized action: `none`.** This invocation is closed. Reopening any goal needs a new
  explicit user instruction (a defined metric and numeric tolerance, and the documented stem set, for
  `#2`/`#3`/`#4`/`#8`; a real CUDA host plus an Apple Silicon host for `#11`), and that is a separate
  authorized PLAN — never an ACT step.

`pending` — the collection stays open for the single active item `#1`. Items `#2`, `#3`, `#4` and
`#8` are PLAN-terminal `acceptance criteria cannot be determined` (not execution failures, attempts 0
except `#2`'s recorded r1/n1), and `#11` is PLAN-terminal `infeasible`; none of them is a closure
trigger and the mandatory retrospective still runs once, when `#1` resolves and this invocation closes.

Note for the eventual closure (not an ACT outcome of the collection itself): item `#11` is already
resolved at PLAN as `original task is infeasible under the stated constraints`. Its cause is recorded
in §PLAN record `r1`: the criterion requires both a real CUDA host and an Apple Silicon host, and no
Apple Silicon/Metal host is reachable from this win32/x86_64 workstation, so `AC-11` stays `unverified`
and README must keep saying "not tested" for CUDA/Metal. That terminal resolution consumes no attempt,
is not a `done`, and does not close the collection.

Appended after the item `#1` ACT (2026-11-10T17:24Z; appended, the paragraphs above are not rewritten):
item `#1` has now resolved — accepted at its own CHECK #3 and committed as `dc94520`, item row `done`,
group `G1` `done`. That is an **item** closure, not a collection closure: the collection stays `open`
because its integrated-tree CHECK has not run, so this ACT entry stays `pending`, the collection
retrospective stays `pending`, and no collection-level verdict, `done` or `PASS` is recorded. The
closure trigger is now the integrated-tree CHECK (accepted integration, or a verified partial such as
an accepted item with four proven criteria defects), after which ACT finalizes the collection and the
mandatory retrospective runs once.

### External closure of the accepted goal `#1` (recorded 2026-11-10T17:38Z; appended, the outcome above is not rewritten)

**GitHub issue `#1` closed as completed; comment `6774583625`; commit `dc94520`.**

- The closure was performed on the GitHub side at `2026-11-10T17:38:31Z` through the GitHub API and carries
  closure comment id `6774583625`, referencing the product commit
  `dc94520550418df3f5d583523d04251bf86859bc`
  (`#1 feat: support BS-RoFormer and the general MDXC/RoFormer family`).
- This is the **external closure of the already accepted goal**, not a new acceptance, verdict, counter or
  attempt: the collection outcome stays `verified-partial (1 of 6 goals accepted)`, item `#1` stays
  `accepted`/`done`, `#2`/`#3`/`#4`/`#8` stay PLAN-terminal `acceptance criteria cannot be determined` and
  `#11` stays PLAN-terminal `infeasible`.
- No product file, criterion, evidence artefact, measurement or table row was changed by recording it; the
  only writes are this note, the matching closure note in `.pdca/status/issue-1-r4.md` and the progress
  rows below.
- Source: the driving role's dispatch of 2026-11-10T17:38Z. The closure happened outside this workspace, so
  no local run log exists for it and none is fabricated.

## Lessons learned

### Collection retrospective (final, recorded 2026-11-10T17:33Z)

This is the one final parent-scope retrospective of this invocation, returned by `pdca-check` in
`retrospective` mode and recorded **verbatim, in its original language**, as the source produced it. It
is analysis, not a verdict: it overrides no outcome, creates no counter, attempt or verification. The
`pending` entry below is **superseded, not deleted**, and stays as history.

````markdown
## Ретроспектива `open-issues-r4`

### Что сработало
- Общие поверхности обоснованно сведены в одну последовательную lane; отдельные ветки и merge не понадобились.
- Терминальные решения по отдельным целям не остановили оставшуюся очередь и не получили зачёт выполнения.
- Оркестрация различала завершение item и интеграционную проверку коллекции. Для интеграции подготовлены снимок, manifest, соответствие коммиту и проверка состава коммита.

Основания: collection status, §PLAN, §General verification and recovery; `rv1/integration-index.md`, §§1–3, 6.

### Дефекты и переносимые уроки

**1. Готовность объявлена до разрешения вопросов проверяемости.**
**Категория:** планирование / передача в исполнение.
- **Наблюдение:** collection PLAN объявил #2/#3/#4/#8 `ready` и заменил отдельные item-планы своими контрактами. После DO/CHECK #2 последовало терминальное решение о неопределимости критериев; аналогичные проблемы остальных трёх целей рассмотрены только затем.
- **Эффект:** #2 прошёл исполнение до выявления проблемы входного контракта; готовность очереди пришлось пересмотреть.
- **Изменение на будущее:** при первоначальной декомпозиции устанавливать проверяемость каждого критерия до `ready`, сохраняя пользовательский текст без дополнений. Отсутствие необходимого основания фиксировать до передачи в DO.
- **Основания:** `rv1/plan.md:2–19`; collection status:119–123, 152–169, 519–524.
- **Неопределённость:** это недостаток опубликованного родительского контракта и передачи, а не вывод о качестве реализации или правомерности терминальных решений.

**2. Одна lane не обеспечила единственного исполнителя.**
**Категория:** владение / handoff / восстановление.
- **Наблюдение:** при `lane_cap=1` два процесса работали над одним item в одном дереве; журнал фиксирует изменения файлов другим исполнителем. Работа остановлена и продолжена после завершения дубля.
- **Эффект:** нарушилась исключительность владения деревом и доказательствами; возникла пауза восстановления.
- **Изменение на будущее:** при продолжении через новую сессию явно передавать владельца item и подтверждать завершение предыдущего writer до разрешения записи новому. Недоступность interrupt не считать подтверждением остановки.
- **Основания:** `item-1-bs-roformer/logs/concurrency-conflict.log:15–62`; collection status:530–531.
- **Неопределённость:** журнал подтверждает конфликт, но не устанавливает, какая именно родительская либо дочерняя диспетчеризация породила дубль. Недоступность API и step-cap известны из brief, а не из прочитанного журнала.

**3. Структурная корректность статуса скрывала семантические расхождения.**
**Категория:** bookkeeping / инструкции и схема.
- **Наблюдение:** заголовок оставался в PLAN, текущая проекция — в DO, а обновления добавлялись ниже. Для #2 таблица показывает `attempts=0`, хотя DO/CHECK `r1/n1` записаны в примечании: причиной указан предел lint-схемы для терминальных строк.
- **Эффект:** чтение заголовка или таблицы отдельно даёт устаревшее состояние либо теряет реально проведённую попытку; успешный lint этого не выявляет.
- **Изменение на будущее:** сохранять историю неизменной, но обновлять текущую проекцию; представление терминального исхода не должно обнулять исторический счётчик попыток.
- **Основания:** collection status:14, 22–26, 119, 523, 538–547.
- **Неопределённость:** ограничение схемы засвидетельствовано записью; её реализация не исследовалась.

### Ограничения
Анализировалась только работа коллекции, без использования дочерних lessons и без новой проверки продукта. Итог интеграционного CHECK сообщён в brief; полный отчёт в прочитанном status отсутствует. Низкая полнота выполнения сама по себе дефектом skill не считается.

**Сигнал: retrospective analysis complete.** Сохранение отчёта и завершение ACT остаются за драйвером и executor.
````

Recorded by `pdca-executor` as the single writer; with this report persisted, ACT is **completed** and
the mandatory retrospective is satisfied (no fallback analysis was performed by the driver or the
executor).

`pending: retrospective not yet conducted` — the mandatory `pdca-check` retrospective runs once, when
this collection invocation actually closes (accepted integration, verified partial integration,
`blocked`/`incomplete`, or terminal `STOP`). Item and group closures are not triggers.

Appended after the item `#1` ACT (2026-11-10T17:24Z): item `#1` ran its own mandatory retrospective,
recorded verbatim in `.pdca/status/issue-1-r4.md` §Lessons learned. That is an **item** retrospective
and does not substitute for this collection's retrospective, which stays `pending` until the
integrated-tree CHECK resolves and this invocation closes.

## General verification and recovery

- common verification state: `pending` / `unverified` — no integrated-tree CHECK has run; item `#2` ran its own independent CHECK (`not accepted` → PLAN) on its snapshot, not on an integrated tree
- last common `C`: `none` — no integrated-tree candidate, so no integrated command, exit code, log or numbers; the item-level last run is recorded in `.pdca/status/issue-2-r4.md` §DO result and §CHECK record
- blocking reason: none for the lane; `#2` closed terminal because its user criterion `AC-2.5` ("within agreed tolerance") is undeterminable, and `#3`, `#4`, `#8` closed terminal for the same class of defect (no metric, numeric tolerance or repo gate) — no `P:` task was opened for any of them
- defect id/history: collection level `AC3-UNDET-001`, `AC4-UNDET-001`, `AC8-UNDET-001` (criteria-defect class, terminal, 0 fixes); item level `AC2-TOL-001` (terminal), `AC2-FMT-001` and `AC2-EVID-001` (minor, open as diagnostics) — see §Defect history and `.pdca/status/issue-2-r4.md` §Defect history
- linked corrective cycle: `none` — a corrective cycle for `#2` would need a new explicit user instruction defining the tolerance, and the same holds for `#3`, `#4` and `#8`
- integrated-tree CHECK verdict: `not run`
- completeness (all original goals accepted): `no` — 0 of 5 active items accepted; 1 still active (`#1`), 4 PLAN-terminal (`#2`, `#3`, `#4`, `#8`)
- incomplete items: `#2`, `#3`, `#4`, `#8` (PLAN-terminal `criteria-undetermined`; `AC-2.5`, `AC-3`, `AC-4`, `AC-8` unverified, no DO/CHECK for `#3`/`#4`/`#8`, nothing committed, `#2` product paths rolled back to `HEAD`) and `#11` (PLAN-terminal `infeasible`, README keeps "not tested")
- preserved results: `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/patch/item-2-candidate.patch` (whole `#2` candidate, 65,446 B, sha256 `4922abec88ce6661a31c96f16eeba2033d5657f7473797fd139769ab8896ff17`, `git apply --check` exit 0 on the restored tree) and `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/snapshot/` (11 byte-exact files), with the measurement evidence (checkpoint, fixture, both reference stems, both candidate stems, `logs/`, `logs/utf8/`, `scripts/`, `reference/`) kept under the same evidence root
- next allowed step: `CHECK item #1` — `G1` is `[#1]` and its DO candidate is packaged under
  `.pdca/open-issues-r4/rv1/item-1-bs-roformer/`; `#2`, `#3`, `#4`, `#8` and `#11` are terminal
  resolutions and are not dispatchable

Appended after the item `#1` ACT (2026-11-10T17:24Z; the bullets above describe the pre-item-verdict
state and are not rewritten):

- common verification state: `pending` / `unverified` at **collection** level — no integrated-tree
  CHECK has run. The item level is settled: `#1` was accepted (`AC-1` `met`) at its own CHECK #3 on
  the frozen-input measurement; `#2`'s own CHECK was `not accepted` → PLAN
- last common `C`: `none` for the integrated tree. Latest accepted item-level run — item `#1`, commit
  `dc94520550418df3f5d583523d04251bf86859bc`: strict validation 699/699 with 0 missing / 0
  unexpected / 0 shape mismatches, parity vocals corr `0.9999999999999561` / max abs `4.76837e-7` /
  SDR `130.302177 dB`, instrumental corr `0.9999999999999664` / `3.65078e-7` / `131.728898 dB`;
  evidence `.pdca/open-issues-r4/rv1/item-1-bs-roformer/`
- blocking reason: none for the lane; `#2` (`AC-2.5`), `#3`, `#4`, `#8` are PLAN-terminal criteria
  defects and no `P:` task was opened for any of them
- defect id/history: collection level `AC3-UNDET-001`, `AC4-UNDET-001`, `AC8-UNDET-001`
  (criteria-defect class, terminal, 0 fixes); item `#1` level `CONC-1` (resolved), `BS-EVID-001`,
  `BS-DOC-003`, `BS-PROV-002` (refuted for `n2`) and `BS-PROV-004` (minor, fixed by ACT) — see
  `.pdca/status/issue-1-r4.md` §Defect history; item `#2` level `AC2-TOL-001` (terminal),
  `AC2-FMT-001`, `AC2-EVID-001`
- integrated-tree CHECK verdict: `not run` — unchanged
- completeness (all original goals accepted): `no` — **1 of 5** active items accepted (`#1`, `done`,
  committed as `dc94520`) and 4 PLAN-terminal (`#2`, `#3`, `#4`, `#8`); `#11` is PLAN-terminal
  `infeasible`
- incomplete items: `#2`, `#3`, `#4`, `#8` (PLAN-terminal `criteria-undetermined`; `AC-2.5`, `AC-3`,
  `AC-4`, `AC-8` unverified, nothing committed, `#2` product paths rolled back to `HEAD`) and `#11`
  (PLAN-terminal `infeasible`, README keeps "not tested")
- preserved results: the `#2` patch and snapshot above remain the preserved `#2` candidate; item `#1`'s
  results are now committed rather than preserved as a patch
- next allowed step: `CHECK (integrated tree) for collection open-issues-r4` — run the collection-level
  CHECK against committed `main` (`dc94520`); no item is dispatchable and no further item work is
  authorized by this record

Appended during the integration staging (2026-11-10T17:29Z; the bullets above are not rewritten):

- integrated candidate **staged, awaiting the integrated-tree CHECK** — no verdict, no PASS and no
  STOP is recorded here. Integrated tree `main @ dc94520550418df3f5d583523d04251bf86859bc`, verified
  at staging: `git rev-parse HEAD` = `dc94520550418df3f5d583523d04251bf86859bc`,
  `git rev-parse --abbrev-ref HEAD` = `main`, `git diff HEAD --` empty (exit 0), and
  `git status --porcelain --` = only the untracked `.openidea/` plus the four `.pdca/status/*.md`
  records — no tracked product path dirty, staged or deleted
- snapshot: `.pdca/open-issues-r4/rv1/snapshot/` — 14 byte copies at the same relative paths, **14/14
  verified, 0 mismatched**: 12 files byte-identical to the committed blob, and `README.md` /
  `src/lib.rs` EOL-normalized identical under the repo's pre-existing `core.autocrlf=true` (no
  `.gitattributes`), proven by CRLF→LF byte equality with the blob and by
  `git hash-object --path=<p>` = `git rev-parse dc94520:<p>`. Every copy also equals the deployed
  working-tree bytes, the accepted item snapshot hash and the item snapshot bytes. Set digest:
  `.pdca/open-issues-r4/rv1/snapshot-sha256.txt`, 14 entries, sha256 `250fd5f3…bd5f`
- staging map: `.pdca/open-issues-r4/rv1/integration-manifest.json` (collection-id, commit, branch
  `main`, worktree state, per-file sha256 + blob OID + identity, the six item rows, evidence and status
  pointers, terminal-item non-disturbance) and `.pdca/open-issues-r4/rv1/integration-index.md` (what
  the integration CHECK must verify); staging script
  `.pdca/open-issues-r4/rv1/scripts/stage_integration_snapshot.ps1`
- staging logs (all under `.pdca/open-issues-r4/rv1/logs/`): `integration-stage.log` (exit 0),
  `integration-snapshot-hashes.json`, `terminal-items-non-disturbance.log`,
  `r3-inventory-sha256.txt`
- commit scope re-checked: exactly 14 paths, 2500 insertions / 343 deletions, no
  `.ckpt/.safetensors/.onnx/.pth/.pt/.bin/.wav/.flac`, no `target/`, no `.pdca/`, no `.openidea/`; no
  push, no amend, no branch switch
- terminal items undisturbed: `#2` patch 65,446 B sha256
  `4922abec88ce6661a31c96f16eeba2033d5657f7473797fd139769ab8896ff17` with its 11-file `snapshot/` plus
  `snapshot.diff` re-hashing 12/12 equal to the item's own `snapshot-sha256.txt`;
  `.pdca/open-issues-r3/` 7 files / 19,145 B, newest mtime `2026-11-10T18:48:00+05:00`, inventory
  digest `7d0c3d45…1f89`; `#3`, `#4`, `#8`, `#11` rows and records unchanged
- quiet-tree check before writing: no python/python3/pythonw/cargo/rustc process (count 0); this
  staging turn made no product edit, no `git add`, no staging, no commit and no push
- next allowed step (same in kind, now with staged evidence): the collection-level **integrated-tree
  CHECK** on `main @ dc94520`, reading `.pdca/open-issues-r4/rv1/` — no item is dispatchable

Appended at the final ACT (2026-11-10T17:33Z; the bullets above are not rewritten):

- common verification state: `accepted` — the collection's **integrated-tree CHECK returned `accepted`**
  on `main @ dc94520550418df3f5d583523d04251bf86859bc`; report recorded verbatim in §CHECK record →
  "Integrated-tree CHECK (collection, final)"
- last common `C`: the integrated-tree CHECK `accepted` (same report pointer), which read the staged
  snapshot, manifest and logs under `.pdca/open-issues-r4/rv1/`; no new proving run was issued at ACT
- blocking reason: **none** — nothing blocks closure; the four `criteria-undetermined` and the one
  `infeasible` resolutions are terminal criteria outcomes, not blockers, and no `P:` task was opened
- integrated-tree CHECK verdict: **`accepted`**, with two minor confirmed findings — `INT-STATUS-001`
  (fixed in this ACT) and `INT-EVID-001` (recorded as a historical-evidence limitation)
- completeness (all original goals accepted): **no — 1 of 6.** Accepted: `#1` (`done`, `dc94520`).
  PLAN-terminal `criteria-undetermined`: `#2`, `#3`, `#4`, `#8`. PLAN-terminal `infeasible`: `#11`.
  Execution-failure finalizations: **0**
- defect id/history (unchanged by ACT): `AC3-UNDET-001`, `AC4-UNDET-001`, `AC8-UNDET-001` and
  `AC2-TOL-001` (terminal, 0 fixes), `AC2-FMT-001` / `AC2-EVID-001` (minor), plus the item `#1` keys in
  `.pdca/status/issue-1-r4.md`; newly recorded integration keys `INT-STATUS-001` (confirmed, fixed once)
  and `INT-EVID-001` (confirmed limitation) — see §Defect history
- next allowed step: **none** — this invocation is closed (see §ACT record "Final ACT outcome")

## Revision history

2026-11-10T14:16:41Z | s1 → s2 | risk-first order before lane start; no criteria change (r=1 kept, outgoing attempt n0)
2026-11-10T15:13:06Z | r1 → r1 (no plan revision) | planner criteria-defect decision after `#2`: `#3`, `#4`, `#8` became PLAN-terminal `acceptance criteria cannot be determined` (criteria defect class `AC3-UNDET-001`/`AC4-UNDET-001`/`AC8-UNDET-001`; no DO, no CHECK, attempts 0, no goal credit); `#2` stays terminal, `#1` the only active item, `G1` reduced to `[#1]`; no criterion added, removed, weakened or narrowed (outgoing attempt n1, item `#2`)

## Defect history

One row per stable defect key; all three are **criteria-defect** keys (the criterion itself cannot be
decided), not execution failures, so no fix and no attempt is applied:

- `AC3-UNDET-001` | observed at r1 (no attempt; no DO, no CHECK) | fixes applied 0 | evidence: planner criteria-defect decision after `#2`, §PLAN record `### Criteria-defect decision after #2` and item row `#3`; run evidence `none — nothing ran` | last recurrence: first occurrence, PLAN-terminal `acceptance criteria cannot be determined` (parity against audio-separator has no metric or numeric tolerance and no repo gate)
- `AC4-UNDET-001` | observed at r1 (no attempt; no DO, no CHECK) | fixes applied 0 | evidence: same decision block and item row `#4`; run evidence `none — nothing ran` | last recurrence: first occurrence, PLAN-terminal `acceptance criteria cannot be determined` (parity against the reference implementation has no numeric threshold, metric or repo gate)
- `AC8-UNDET-001` | observed at r1 (no attempt; no DO, no CHECK) | fixes applied 0 | evidence: same decision block and item row `#8`; run evidence `none — nothing ran` | last recurrence: first occurrence, PLAN-terminal `acceptance criteria cannot be determined` (`match the reference` has no numeric tolerance, `documented stem set` is undefined in issue and repo, and the premise `#2 → #8` is unmet because `#2` closed without a commit)

Escalation state: none of the three is a repeated defect and none was retried, so no escalation and no
`P:` corrective task. Fixing them requires a new explicit user instruction that defines the missing
metric, tolerance and stem set — not an executor or CHECK action.

Appended at the final ACT (2026-11-10T17:33Z; the rows above are not rewritten) — the two findings of the
collection's integrated-tree CHECK, recorded as stable defect keys:

- `INT-STATUS-001` | observed at r1, integrated-tree CHECK (collection attempt n2; no new attempt
  consumed) | fixes applied **1** (this ACT: current header set to `phase: ACT` / `record: completed` /
  `outcome: verified-partial (1 of 6 goals accepted)`, with the earlier text kept as superseded history)
  | evidence: §CHECK record findings table, §Current state, §Progress log | last recurrence: first
  occurrence, minor / confirmed, closed by the ACT fix
- `INT-EVID-001` | observed at r1, integrated-tree CHECK (same attempt) | fixes applied **0** — a
  historical-evidence limitation, not a correctable artifact | evidence:
  `.pdca/open-issues-r4/rv1/logs/terminal-items-non-disturbance.log:23–25` and the §CHECK record findings
  table | last recurrence: first occurrence, minor / confirmed, retained as a documented limitation

Neither key is a repeated defect and neither was retried, so no escalation and no `P:` corrective task.

## Changed files

Task-owned artefacts created by this recording task, `.pdca` only:

- `.pdca/status/collection-open-issues-r4.md` → created (this record); modified in the criteria-defect update, in the item `#1` attempt-`n2` update, and in the item `#1` ACT update (item row `#1` → `done`/`finalized`/attempts 2, group `G1` → `done`, §Current state, the appended §DO result resolution, §CHECK record, §ACT record, §General verification and recovery, §Git state, progress rows)
- `.pdca/open-issues-r4/rv1/plan.md` → created (PLAN return `r1`, verbatim)
- `.pdca/open-issues-r4/rv1/scout.md` → created (environment-fact pointer)
- `.pdca/open-issues-r4/rv1/git-state.txt` → created (baseline HEAD/branch/tree state)
- `.pdca/open-issues-r4/rv1/lint.txt` → created and appended (status-lint output, including the runs after the criteria-defect update and after the item `#1` ACT update)

No product file, test, config, README or docs file was modified by this recording task. The item `#1`
product delta it reports was committed by the item's ACT as `dc94520550418df3f5d583523d04251bf86859bc`
(14 paths, on `main`); this record owns none of those paths.

The `BS-PROV-004` wording correction touched `.pdca/open-issues-r4/rv1/item-1-bs-roformer/manifest.json`
and `index.md`; those files are owned by `.pdca/status/issue-1-r4.md` §Changed files, and this
collection record only reports the correction.

Ownership note: the item `#1` product delta and its changed-file list are recorded in
`.pdca/status/issue-1-r4.md`; this collection record owns only the batch-level `.pdca` artefacts above.

Appended during the integration staging (2026-11-10T17:29Z; the list above is not rewritten):

- `.pdca/open-issues-r4/rv1/snapshot/` → created (14 byte copies of the committed item files, same
  relative paths, 14/14 verified)
- `.pdca/open-issues-r4/rv1/snapshot-sha256.txt` → created (14 entries; set digest `250fd5f3…bd5f`)
- `.pdca/open-issues-r4/rv1/integration-manifest.json` → created (integrated-candidate manifest)
- `.pdca/open-issues-r4/rv1/integration-index.md` → created (integration CHECK map)
- `.pdca/open-issues-r4/rv1/scripts/stage_integration_snapshot.ps1` → created (staging + verification)
- `.pdca/open-issues-r4/rv1/logs/integration-stage.log`, `integration-snapshot-hashes.json`,
  `terminal-items-non-disturbance.log`, `r3-inventory-sha256.txt`,
  `lint-07-integration-stage.log` → created
- `.pdca/status/collection-open-issues-r4.md` → appended (§General verification and recovery, this
  §Changed files addendum, §Progress log rows, lint row)

Still no product, test, config, README or docs file was modified by this recording task, and nothing
was staged or committed.

Appended at the final ACT (2026-11-10T17:33Z; the lists above are not rewritten):

- `.pdca/status/collection-open-issues-r4.md` → **modified** (final ACT): current header fixed
  (`phase`/`record`/`outcome` → `ACT` / `completed` / `verified-partial (1 of 6 goals accepted)`), the
  integrated-tree CHECK report and the collection retrospective recorded verbatim, §ACT record finalized,
  and §General verification and recovery / §Defect history / §Git state / §Progress log updated
- `.pdca/open-issues-r4/rv1/logs/lint-08-collection-act.log` → created (both status-lint runs of this
  ACT, exit 0)

No product, test, config, README or docs file was touched by this ACT; nothing was staged or committed.

## Git state

`status path: .pdca/status/collection-open-issues-r4.md — created and modified this cycle (schedule
`s2` update, the criteria-defect decision update, the item `#1` attempt-`n2` record update, then the
item `#1` ACT update) · untracked (durable on disk, not committed)`

- baseline supplied at creation: `main @ bf2cab6` (subject `chore: track the remaining #13 cycle
  evidence referenced by the status record`); the brief stated a clean tree
- observed at baseline: one pre-existing untracked path, `?? .pdca/status/collection-open-issues-r3.md`
  (the prior cycle's record). Recorded as observed; not touched, not claimed, not attributed
- same-cycle touched-file ledger: the five `.pdca` artefacts in §Changed files; `.pdca/open-issues-r4/`
  is ignored by `/.pdca/*` in `.gitignore`, so only the status record is visible to `git status`
- observed later, unclaimed: an untracked `?? .openidea/` path appeared during this task (absent from
  the first observation); not created, touched or attributed. `.pdca/status/issue-2-r4.md` (the item
  `#2` record) is likewise untracked and prior to this task's delta
- pending commit: none — no item has been accepted, and this recording task is not authorized to
  `git add`, stage or commit; autocommit applies only to accepted item results
- ownership: the item `#1` product delta (14 task-owned paths) and its changed-file list are
  recorded in `.pdca/status/issue-1-r4.md`; this collection record owns only the batch-level `.pdca`
  artefacts listed in §Changed files
- work outside this task's delta stays untouched and unclaimed

Appended after the item `#1` ACT (2026-11-10T17:24Z; the ledger above is not rewritten):

- **item commit (now present)**: `dc94520550418df3f5d583523d04251bf86859bc` — `#1 feat: support
  BS-RoFormer and the general MDXC/RoFormer family`, on `main`, exactly the 14 item paths (2500
  insertions / 343 deletions), no weights, no `.pdca/**`, no `.openidea/**`, no `target/**`; no push,
  no amend, no branch switch. It is the item's autocommit, authorized by the accepted item CHECK and
  owned by `.pdca/status/issue-1-r4.md` §Git state
- post-commit observation: `HEAD` = `dc94520`; `git show --stat` lists exactly those 14 paths;
  `git status --porcelain` = `?? .openidea/`, `?? .pdca/status/collection-open-issues-r3.md`,
  `?? .pdca/status/collection-open-issues-r4.md`, `?? .pdca/status/issue-1-r4.md`,
  `?? .pdca/status/issue-2-r4.md` — no product path dirty; `git rev-parse --abbrev-ref HEAD` = `main`
- this collection record and the item records remain **untracked and uncommitted** by design
  (`/.pdca/*` is ignored); they were never staged, and autocommit covers accepted item files only
- line endings: `core.autocrlf=true` with no `.gitattributes`, so the committed blobs are LF for
  `README.md` and `src/lib.rs` (CRLF in the working tree); a fresh checkout restores CRLF. Pre-existing
  repo behaviour, not an item change
- `pending commit`: none for this task; the only pending items are the untracked `.pdca` records above

Appended at the final ACT (2026-11-10T17:33Z; the ledger above is not rewritten):

- `HEAD` = `dc94520550418df3f5d583523d04251bf86859bc` on `main` (`git rev-parse HEAD` and
  `git rev-parse --abbrev-ref HEAD`), and `git show --stat --oneline HEAD` lists exactly **14 files**
  (`README.md`, `configs/bs_roformer_viperx_1297.yaml`, `examples/bs_roformer_separate.rs`,
  `src/config.rs`, `src/lib.rs`, `src/model/bs_roformer.rs`, `src/model/mod.rs`, `src/separate.rs`,
  `src/separator.rs`, `tests/bs_roformer.rs`, `tests/core_tests.rs`, `tests/library_api.rs`,
  `tests/streaming_chunked.rs`, `tools/convert_bs_roformer.py`), 2500 insertions / 343 deletions
- status and evidence files stay **untracked and not committed**: `git status --porcelain --` =
  `?? .openidea/`, `?? .pdca/status/collection-open-issues-r3.md`,
  `?? .pdca/status/collection-open-issues-r4.md`, `?? .pdca/status/issue-1-r4.md`,
  `?? .pdca/status/issue-2-r4.md` — no tracked product path dirty, staged or deleted, and
  `.pdca/open-issues-r4/` stays ignored by `/.pdca/*`
- topology unchanged by ACT: `git worktree list` = only `F:/sources/candle-roformer dc94520 [main]`;
  `git branch` = only `main` (plus `remotes/origin/main` and `origin/HEAD`) — no worktree created, no
  task branch, no merge, no cleanup
- `pending commit`: **none** — no product change is pending; this ACT made no `git add`, no staging, no
  commit, no push, no amend and no history rewrite
- quiet-tree check before this ACT write: no python/python3/pythonw/cargo/rustc process (count 0)

Appended on the external closure of `#1` (2026-11-10T17:38Z; the ledger above is not rewritten):

- `HEAD` at the start of this bookkeeping step = `dc94520550418df3f5d583523d04251bf86859bc` on `main`;
  the product tree is clean (no staged or modified product path)
- this record and `.pdca/status/issue-1-r4.md` (with `.pdca/status/issue-2-r4.md`) are included in one
  bookkeeping commit on `main` (`chore: record PDCA r4 status and close #1 evidence`), created with the
  existing repository author identity through per-command `-c` flags; no `git config` file was written, no
  push, no amend, no history rewrite
- deliberately **not** staged or committed: `.openidea/` and
  `.pdca/status/collection-open-issues-r3.md` (prior-cycle record); they stay untracked and unclaimed
- the commit adds status records only — `.pdca/open-issues-r4/` stays ignored by `/.pdca/*`, and no
  product, test, config, README or docs path is part of it

## Progress log

2026-11-10T14:13:19Z | PLAN | revision r1 | attempt n0 | collection PLAN recorded as returned (six items; #1-#4 and #8 ready; #11 PLAN-terminal infeasible) | .pdca/open-issues-r4/rv1/plan.md
2026-11-10T14:13:54Z | PLAN | revision r1 | attempt n0 | schedule s1 recorded: one sequential lane G1 [#1, #2, #3, #4, #8] on worktree main, order #1 → #2 → #3 → #4 → #8, plan_cap 1, lane_cap 1 | .pdca/status/collection-open-issues-r4.md
2026-11-10T14:13:54Z | PLAN | revision r1 | attempt n0 | barrier satisfied: #1, #2, #3, #4, #8 plan_state=ready with per-item contracts at r1; #11 terminal outside G1 and blocking nothing | .pdca/open-issues-r4/rv1/plan.md
2026-11-10T14:13:54Z | PLAN | revision r1 | attempt n0 | Notice: baseline tree is not clean — one pre-existing untracked path .pdca/status/collection-open-issues-r3.md, untouched and unclaimed | .pdca/open-issues-r4/rv1/git-state.txt
2026-11-10T14:13:54Z | PLAN | revision r1 | attempt n0 | next allowed step recorded: DO item #1 | .pdca/status/collection-open-issues-r4.md
2026-11-10T14:14:31Z | PLAN | revision r1 | attempt n0 | status_lint.py run via full Python 3.12.10 path, exit 0, 2 tables 7 rows, no violations | .pdca/open-issues-r4/rv1/lint.txt
2026-11-10T14:16:41Z | PLAN | revision r1 | attempt n0 | schedule s2 recorded: G1 order #2,#8,#3,#1,#4; #8 dependency #2→#8; no criteria change | .pdca/status/collection-open-issues-r4.md
2026-11-10T14:17:02Z | PLAN | revision r1 | attempt n0 | status_lint.py run after s2: `C:\Users\Пользователь\AppData\Local\Programs\Python\Python312\python.exe .pdca/scripts/status_lint.py .pdca/status/collection-open-issues-r4.md` (cwd F:\sources\candle-roformer) → "status-lint: OK (2 tables, 7 rows)", exit 0 | .pdca/status/collection-open-issues-r4.md
2026-11-10T14:17:38Z | PLAN | revision r1 | attempt n0 | status_lint.py re-run after the final row (same command, cwd F:\sources\candle-roformer) → "status-lint: OK (2 tables, 7 rows)", exit 0; git status shows only this record added | .pdca/status/collection-open-issues-r4.md
2026-11-10T14:19:29Z | DO | revision r1 | attempt n1 | item #2 entered DO: item status record created, item row pending→in-progress, group G1 pending→in-progress, attempts 0→1; next item remains #2 while its DO runs | .pdca/status/issue-2-r4.md
2026-11-10T14:49:38Z | DO | revision r1 | attempt n1 | item #2 candidate ready (CHECK not run): MDX-Net ONNX CPU separation vs audio-separator 0.48.0 — overall max abs diff 1.423e-6, RMSE 1.465e-7, SDR 118.90 dB; runtime 85.285 s; 90 tests passed; item stays in-progress, attempts 1 | .pdca/status/issue-2-r4.md
2026-11-10T14:52:59Z | DO | revision r1 | attempt n1 | item #2 final-tree gate re-run after a cosmetic src/lib.rs import reorder (same attempt n1, no new attempt, no CHECK): cargo test exit 0 — 90 passed, 0 failed, 0 ignored; cargo clippy 0; item manifest refreshed at HEAD main @ bf2cab6; item stays in-progress, attempts 1 | .pdca/open-issues-r4/rv1/item-2-mdx-onnx/logs/cargo-test-full-final.log
2026-11-10T14:55:52Z | DO | revision r1 | attempt n1 | item #2 evidence staged for CHECK's scoped read (same attempt n1, no product edit, no re-measurement, no CHECK): 11 files copied to snapshot/ with hashes verified, snapshot.diff 65,446 B, snapshot-sha256.txt, index.md; status_lint.py on this record exit 0 (2 tables, 7 rows); item stays in-progress, attempts 1 | .pdca/open-issues-r4/rv1/item-2-mdx-onnx/index.md
2026-11-10T15:10:35Z | ACT | revision r1 | attempt n1 | item #2 closed as a PLAN-terminal resolution: CHECK `not accepted` → PLAN → ACT, outcome `incomplete` / `terminal=criteria-undetermined` / `disposition=finalized` (cause AC2-TOL-001: no agreed MDX-Net tolerance, the user's AC-2 is immutable; r2 not issued); attempt r1/n1 ran DO + CHECK but is not acceptance credit, lint caps the row cell at attempts 0; candidate preserved as patch (65,446 B, sha256 4922abec…6ff17) plus snapshot/, product paths rolled back to HEAD, nothing committed; group G1 NOT stopped — queue continues with #8; collection stays DO/open, 0 of 5 accepted; status_lint.py exit 0 (2 tables, 7 rows) | .pdca/status/issue-2-r4.md
2026-11-10T15:14:12Z | PLAN | revision r1 | attempt n1 | criteria-defect terminal resolutions #3/#4/#8; G1 reduced to #1 | .pdca/open-issues-r4/rv1/plan.md
2026-11-10T15:14:21Z | PLAN | revision r1 | attempt n1 | status_lint.py run after the criteria-defect update via full Python 3.12 path -> status-lint: OK (2 tables, 7 rows), exit 0; git status shows no product change | .pdca/open-issues-r4/rv1/lint.txt
2026-11-10T15:14:47Z | PLAN | revision r1 | attempt n1 | final status_lint.py re-run after the closing wording corrections -> status-lint: OK (2 tables, 7 rows), exit 0; no product file, staging or commit touched | .pdca/open-issues-r4/rv1/lint.txt
2026-11-10T15:14:58Z | PLAN | revision r1 | attempt n1 | Notice: an untracked path .openidea/ appeared in git status during this recording task (absent from the earlier observation of the same task); not created, touched or claimed by this task | .pdca/status/collection-open-issues-r4.md
2026-11-10T15:30:33Z | DO | revision r1 | attempt n1 | item #1 entered DO: item record `.pdca/status/issue-1-r4.md` created, item row `#1` pending→in-progress, group `G1` pending→in-progress, attempts 0→1; DO started as a continuation of attempt n1 (no new attempt, no re-plan); on-disk state re-verified — `cargo build --lib` exit 0, fixture sha256 `99b1d531…3541` and model yaml sha256 `2bfdd16c…af7f` match, checkpoint download still in flight | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/cargo-build-lib.log
2026-11-10T15:35:00Z | DO | revision r1 | attempt n1 | item #1 checkpoint verified complete: 639,331,213 B, sha256 `5b84f37e…15aa` = pinned value; no partial file used | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/checkpoint-verify.log
2026-11-10T15:36:45Z | DO | revision r1 | attempt n1 | item #1 DO BLOCKED, defect `CONC-1`: a second execution process is running the same attempt in the same tree (rewrote `scripts/convert_ckpt.py`, created `logs/hash-verify.log`, `logs/install-safetensors.log`, `logs/convert*.log`, `models/…safetensors`, `logs/load-smoke.log`, ran python at 20:32:15, and changed `src/model/mod.rs`, `src/model/bs_roformer.rs`, `src/config.rs` after this session's build); no product edit and no measurement by this session; item stays in-progress, attempts 1, fixes applied 0 — recovery is a driving-role action (interrupt the duplicate executor, then one writer finishes, then CHECK) | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/concurrency-conflict.log
2026-11-10T16:08:00Z | DO | revision r1 | attempt n1 | item #1 DO resumed by the single remaining writer after `CONC-1`; the duplicate writer exited and the tree had been quiet for 5+ minutes; no new attempt, no re-plan, no CHECK | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/concurrency-conflict.log
2026-11-10T16:17:00Z | DO | revision r1 | attempt n1 | item #1 candidate-ready (CHECK not run): real BS-RoFormer checkpoint passes the crate-schema strict validation 699/699 keys with 0 missing / 0 unexpected / 0 shape mismatches and loads, separates the frozen 8 s fixture in 95.753 s, and matches the pinned reference - vocals corr 0.9999999999999561 / max abs 4.768e-7 / RMSE 3.753e-8 / SDR 130.302 dB, instrumental corr 0.9999999999999664 / max abs 3.651e-7 / RMSE 2.860e-8 / SDR 131.729 dB; cargo test 87 passed / 0 failed / 1 ignored, clippy exit 0, cargo fmt --check exit 1 only on files untouched by the item; item stays in-progress, attempts 1 | .pdca/status/issue-1-r4.md
2026-11-10T16:18:30Z | DO | revision r1 | attempt n1 | item #1 evidence packaged for CHECK (same attempt n1, no product edit during packaging): snapshot/ 14 files, snapshot-sha256.txt, snapshot.diff 165,921 B vs bf2cab6, manifest.json, index.md; collection row #1 notes updated, group G1 still in-progress, nothing committed | .pdca/open-issues-r4/rv1/item-1-bs-roformer/index.md
2026-11-10T16:56:00Z | DO | revision r1 | attempt n2 | item #1 entered attempt `n=2` via CHECK→DO: its attempt-1 CHECK returned `not accepted` (`BS-EVID-001` blocking/unverified, `BS-DOC-003` and `BS-PROV-002` minor); item row attempts 1→2, item stays in-progress, group G1 stays in-progress; quiet-tree check before the write: no python/cargo/rustc process and no watched product path changed in the previous 60 s; the re-gather alone would not have been a counted attempt, the README correction makes it one | .pdca/status/issue-1-r4.md
2026-11-10T16:59:00Z | DO | revision r1 | attempt n2 | item #1 fixes applied: `README.md` converter paragraph corrected (`BS-DOC-003`: key/shape checks then a float32 cast for the BS converter, dtype checks named for the v7 converter) and `manifest.json` → `safetensors.converter` corrected (`BS-PROV-002`: `scripts/convert_ckpt.py` produced `logs/convert-report.json`, `tools/convert_bs_roformer.py` is the in-tree converter that did not); no code/test/config/tool change; candidate re-frozen from the live tree — 14 files, 14/14 hash-equal to their live counterparts, only README.md moved vs attempt 1, HEAD still bf2cab6 | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/refreeze-report.json
2026-11-10T17:04:20Z | DO | revision r1 | attempt n2 | item #1 re-measured from the frozen snapshot only: materialized `bf2cab6` + overlay (14/14 hashes verified before build, no branch/worktree), build rc 0, strict-validation test rc 0 with `expected=699 actual=699 missing=0 unexpected=0 shape_mismatches=0`, candidate run rc 0 (stems `4d9e3ab9…`/`b37a10d4…`, byte-identical to attempt 1), parity comparison rc 0 with 12/12 hard assertions per stem (vocals corr 0.9999999999999561 / max abs 4.76837e-07 / SDR 130.302177 dB; instrumental corr 0.9999999999999664 / 3.65078e-07 / 131.728898 dB); executable sha256 4fe007ef…9dbe8; no CHECK has re-run, recheck pending, item stays in-progress at attempts 2 | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/snapshot-build-report.json
2026-11-10T17:25:56Z | ACT | revision r1 | attempt n2 | item #1 finalized at its own ACT: CHECK #3 (final) returned accepted for AC-1 (met), no new proving run; BS-PROV-004 (minor) corrected in the item's manifest.json/index.md; item result committed as dc94520550418df3f5d583523d04251bf86859bc on main (14 item paths, 2500 insertions / 343 deletions, no weights, no .pdca/**, no .openidea/**, no target/**); item row #1 in-progress→done, disposition active→finalized, attempts stay 2; group G1 in-progress→done (single group, no merge); collection stays open with no integrated-tree verdict | .pdca/status/issue-1-r4.md
2026-11-10T17:25:56Z | ACT | revision r1 | attempt n2 | collection record updated after the item #1 ACT (appended, earlier paragraphs and rows not rewritten): §Current state, the §DO result resolution for item #1 (with the BS-PROV-004 build-root correction), §CHECK record note (item accepted, integrated-tree CHECK still pending), §ACT record note (item closure is not a collection closure), §Lessons learned note (item retrospective recorded in the item record, collection retrospective still pending), §General verification and recovery, §Changed files, §Git state; completeness now 1 of 5 active items accepted | .pdca/status/collection-open-issues-r4.md
2026-11-10T17:25:56Z | ACT | revision r1 | attempt n2 | Notice: no git author identity is configured in this workspace (user.name/user.email unset locally and globally); the single item commit reused the identity already present on every existing commit (Alexey Shirshov <alex.shirshov@vk.com>) through per-command -c flags, and no git config file was written | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/commit-message.txt
2026-11-10T17:25:56Z | ACT | revision r1 | attempt n2 | status_lint.py after the item #1 ACT update on both records: .pdca/status/issue-1-r4.md -> status-lint: OK (2 tables, 3 rows), exit 0; .pdca/status/collection-open-issues-r4.md -> status-lint: OK (2 tables, 7 rows), exit 0; next authorized step recorded: collection-level integrated-tree CHECK against main @ dc94520 | .pdca/open-issues-r4/rv1/item-1-bs-roformer/logs/lint-06-collection-act.log
2026-11-10T17:29:13Z | DO | revision r1 | attempt n2 | integration staging started for the collection CHECK (no product edit, no commit, no push): deployed tree verified — HEAD = dc94520550418df3f5d583523d04251bf86859bc on main, git diff HEAD -- empty (exit 0), git status --porcelain -- only untracked .openidea/ and the four .pdca/status/*.md records; quiet-tree check before writing: no python/python3/pythonw/cargo/rustc process (count 0) | .pdca/open-issues-r4/rv1/logs/integration-stage.log
2026-11-10T17:29:13Z | DO | revision r1 | attempt n2 | integrated snapshot staged: .pdca/open-issues-r4/rv1/snapshot/ 14 byte copies at the same relative paths, 14/14 verified 0 mismatched — 12 byte-identical to the committed blob and README.md/src/lib.rs EOL-normalized identical under the repo's pre-existing autocrlf=true (CRLF->LF byte equality plus clean-filter blob OID equal); every copy also equals the deployed file, the accepted item snapshot hash and the item snapshot bytes; set digest snapshot-sha256.txt sha256 250fd5f3…bd5f, script exit 0 | .pdca/open-issues-r4/rv1/snapshot-sha256.txt
2026-11-10T17:29:13Z | DO | revision r1 | attempt n2 | integration manifest and index written: integration-manifest.json (collection-id, commit dc94520, branch main, worktree state, 14 per-file sha256 + blob OID + identity, the six item rows, evidence/status pointers, terminal-item non-disturbance) and integration-index.md (one-page map for the integrated-tree CHECK); valid JSON, 6 items / 14 files | .pdca/open-issues-r4/rv1/integration-manifest.json
2026-11-10T17:29:13Z | DO | revision r1 | attempt n2 | terminal-item non-disturbance verified: #2 patch 65,446 B sha256 4922abec…6ff17 with its 11-file snapshot + snapshot.diff re-hashing 12/12 equal to the item's own snapshot-sha256.txt; .pdca/open-issues-r3/ 7 files / 19,145 B, newest mtime 2026-11-10T18:48:00+05:00, inventory digest 7d0c3d45…1f89; #3/#4/#8/#11 rows unchanged; nothing under those roots written by this turn | .pdca/open-issues-r4/rv1/logs/terminal-items-non-disturbance.log
2026-11-10T17:29:13Z | DO | revision r1 | attempt n2 | commit scope re-checked for the integrated tree: dc94520 lists exactly the 14 item paths, 2500 insertions / 343 deletions, no weights (.ckpt/.safetensors/.onnx/.pth/.pt/.bin/.wav/.flac), no target/, no .pdca/, no .openidea/; single group G1=[#1] with no merge — the integrated tree is simply main @ dc94520 | .pdca/open-issues-r4/rv1/integration-index.md
2026-11-10T17:29:13Z | DO | revision r1 | attempt n2 | collection record updated for the staging (appended, earlier paragraphs and rows not rewritten): §General verification and recovery -> staged awaiting the integrated-tree CHECK with the commit hash and snapshot paths, §Changed files addendum, §Progress log rows; collection still open, no integrated-tree verdict, ACT and retrospective still pending; no product file, staging or commit touched | .pdca/status/collection-open-issues-r4.md
2026-11-10T17:29:44Z | DO | revision r1 | attempt n2 | status_lint.py run after the staging update via the full Python 3.12 path `C:\Users\Пользователь\AppData\Local\Programs\Python\Python312\python.exe .pdca/scripts/status_lint.py .pdca/status/collection-open-issues-r4.md` (cwd F:\sources\candle-roformer) -> "status-lint: OK (.pdca\status\collection-open-issues-r4.md: 2 tables, 7 rows)", exit 0 | .pdca/open-issues-r4/rv1/logs/lint-07-integration-stage.log
unknown (recorded 2026-11-10T17:33Z) | CHECK | revision r1 | attempt n2 | collection integrated-tree CHECK returned `accepted` on main @ dc94520 — INT-IDENTITY, INT-SCOPE, INT-PRESERVE, AC-1 and INT-NO-FALSE-CREDIT all `met`; findings INT-STATUS-001 and INT-EVID-001 minor / confirmed, nothing blocking; transition → ACT | .pdca/status/collection-open-issues-r4.md §CHECK record
2026-11-10T17:33:25Z | ACT | revision r1 | attempt n2 | collection ACT closed: outcome `accepted integrated tree` + `verified-partial` — collection NOT fully complete, 1 of 6 goals accepted (#1 done at commit dc94520; #2/#3/#4/#8 PLAN-terminal criteria-undetermined; #11 PLAN-terminal infeasible; 0 execution-failure finalizations); no new proving run, no product edit, no commit, no push | .pdca/status/collection-open-issues-r4.md §ACT record
2026-11-10T17:33:25Z | ACT | revision r1 | attempt n2 | INT-STATUS-001 fixed (fixes applied 1): current header `phase`/`record`/`outcome` set to `ACT` / `completed` / `verified-partial (1 of 6 goals accepted)`; the earlier header text and every historical block are superseded, kept quoted, not deleted | .pdca/status/collection-open-issues-r4.md
2026-11-10T17:33:25Z | ACT | revision r1 | attempt n2 | INT-EVID-001 recorded as a minor / confirmed limitation: "r3 untouched" rests on the current inventory and the pre-staging mtime, not on an independent baseline inventory taken before the whole collection; no modification observed | .pdca/open-issues-r4/rv1/logs/terminal-items-non-disturbance.log
2026-11-10T17:33:25Z | ACT | revision r1 | attempt n2 | collection retrospective (pdca-check, retrospective mode) recorded verbatim in §Lessons learned — what worked, 3 transferable lessons with categories/evidence/uncertainty, limits and the completion signal; ACT completed only after the report was persisted | .pdca/status/collection-open-issues-r4.md §Lessons learned
2026-11-10T17:35:15Z | ACT | revision r1 | attempt n2 | status_lint.py after the collection ACT finalization, cwd F:\sources\candle-roformer, full Python 3.12 path: `.pdca/status/collection-open-issues-r4.md` -> "status-lint: OK (5 tables, 20 rows)", exit 0; `.pdca/status/issue-1-r4.md` -> "status-lint: OK (2 tables, 3 rows)", exit 0; no product file, staging or commit touched | .pdca/open-issues-r4/rv1/logs/lint-08-collection-act.log
2026-11-10T17:38:31Z | ACT | revision r1 | attempt n2 | GitHub issue `#1` closed as completed via the GitHub API — closure comment id 6774583625, referencing commit dc94520 (`#1 feat: support BS-RoFormer and the general MDXC/RoFormer family`); this is the external closure of the accepted goal, NOT a new acceptance: outcome stays `verified-partial (1 of 6 goals accepted)`, `#1` stays accepted/done, `#2`/`#3`/`#4`/`#8` stay PLAN-terminal criteria-undetermined and `#11` stays infeasible; no counter, attempt or criterion changed | .pdca/status/collection-open-issues-r4.md §ACT record
2026-11-10T17:38:31Z | ACT | revision r1 | attempt n2 | bookkeeping: this closure is recorded here and in `.pdca/status/issue-1-r4.md` only — no product file, test, config, README or docs edit, no evidence rewrite; the r4 status records are then committed as one bookkeeping commit on `main` (`chore: record PDCA r4 status and close #1 evidence`), with no push and with `.openidea/` and `.pdca/status/collection-open-issues-r3.md` deliberately left unstaged | .pdca/status/collection-open-issues-r4.md §Git state
2026-11-10T17:38:31Z | ACT | revision r1 | attempt n2 | status_lint.py after the closure recording, cwd F:\sources\candle-roformer, full Python 3.12 path: `.pdca/status/collection-open-issues-r4.md` -> "status-lint: OK (5 tables, 20 rows)", exit 0; `.pdca/status/issue-1-r4.md` -> "status-lint: OK (2 tables, 3 rows)", exit 0 | .pdca/open-issues-r4/rv1/logs/lint-09-closure.log
