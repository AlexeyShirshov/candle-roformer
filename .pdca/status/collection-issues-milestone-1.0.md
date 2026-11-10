# Collection status: issues-milestone-1.0

- collection-id: `issues-milestone-1.0`
- form: `collection`
- plan_cap: 4
- lane_cap: 1
- cap_reason: single workspace on branch main, no isolated worktrees, direct write-set overlap of the only two live items
- mode: `autonomous`
- cycle/campaign counters: untouched (collection adds no counter of its own)

## Current state

- task: `issues-milestone-1.0` · form: `collection` · specialized skill: `pdca-coder`
- mode: `autonomous`
- cycle: `N=n/a (collection)` · revision: `r=1` · attempt: `n=0`
- scope: `all 11 open issues #1–#11 of AlexeyShirshov/candle-roformer (milestone 1.0)`
- status path: `.pdca/status/collection-issues-milestone-1.0.md`
- goal: `each item ends either accepted+committed against its own issue criteria, or with a documented terminal outcome`
- phase: `ACT` · record: `completed` · outcome: `closed — collection incomplete; integration CHECK PASS on 3c1eec0 (tree 2c0b911c…); accepted #9 (3c1eec0); 9 PLAN-terminal infeasible (#1–#8,#11); #10 execution-incomplete (AC-10-2 unverified); g1 stopped; tree clean; no push`
- superseded: previous `phase: PLAN` line (stated DO had not started / CHECK never ran) superseded by this line on 2026-11-08 (integration evidence round)
- superseded: previous `phase: integration verification` line superseded by this line on 2026-11-08 (ACT finalization: integration CHECK PASS, collection incomplete)
- source: `entry PLAN (pdca-planner, form=collection, overlay=pdca-coder)`
- criteria: `AC-01…AC-11 = verbatim "Done when" text of issues #1…#11 (see PLAN record r1)`

## Original task

verbatim:
> pdca по https://github.com/AlexeyShirshov/candle-roformer/issues/ автономно автокомит

interpretation (caller, not replacing the original):
Run the collection of all 11 open issues #1–#11 of `AlexeyShirshov/candle-roformer` (milestone 1.0): each item ends either accepted+committed against its own issue criteria, or with a documented terminal outcome `blocked`/`infeasible` with evidence. Autonomous mode, no push, no branch switch, no history rewrite, no download/bundling/caching of model weights, no faked PASS.

## PLAN record

### r1

[ENTRY PLAN — pdca-planner return, form=collection, overlay=pdca-coder]

Form: `collection` (`pdca-collection`). Specialized skill/overlay: `pdca-coder`.
Entry scope: all 11 open issues #1–#11 of `AlexeyShirshov/candle-roformer` (milestone 1.0); each item ends either accepted+committed against its own issue criteria, or with a documented terminal outcome `blocked`/`infeasible` with evidence.
Snapshot anchor: `/home/alex/sources/candle-roformer`, branch `main`, HEAD `1f823c4`, clean tree at entry.
Evidence/status location: new paths under `.pdca/` only — status `.pdca/status/issues-milestone-1.0.md` and evidence `.pdca/issues-milestone-1.0/`; the pre-existing `.pdca/status/candle-roformer-impl.md` and `.pdca/candle-roformer-impl/**` are never touched.
Acceptance criteria: AC-01…AC-11 = the verbatim "Done when" text of issues #1…#11 (reproduced in the input section above).
Measurement method (for measurable criteria): (1) residual invariant / peak normalization — metric max_n |(vocals+instrumental) − input| in linear f32 amplitude, all samples of all channels, same sample rate and channel count, tolerance ≤1e-5 for exact residual; peak-normalization threshold default T=0.9 and amplification default 0.0 come from AC-07; AC-09 defaults must reproduce current numbers bit-for-bit or within 1e-6. (2) FLAC/MP3 round-trip — output sample rate (Hz) and decoded duration (samples/channel); exact sample-rate equality, exact sample count for FLAC, ≤2304 samples deviation for MP3 (LAME frame padding). (3) Peak RSS vs input length at fixed `--chunk_duration` — VmHWM in MiB for a series of inputs including a 2 h fixture, same host/model/rate/channels; growth ≤20 % when input length grows several-fold, with exact output length and residual invariant preserved. (4) Numeric parity / hardware runs for #1, #2, #3, #4, #8, #11 — require absent weights, reference fixtures and hardware; these items take the terminal outcome `original task is infeasible under the stated constraints` with no proxy metric or synthetic weights.
Per-item feasibility: #1, #2, #3, #4, #8, #11 → PLAN-terminal `infeasible` (blocked) with the evidence listed above; #5, #6, #7, #9, #10 → achievable, full DO→CHECK→ACT with one atomic Conventional-Commits commit per accepted item.
Constraints: workspace only; commits allowed (one atomic commit per accepted item, item's own files only), no push, no branch switch, no history rewrite, no --force; do not modify `AGENTS.md`, `LICENSE`, `docs/superpowers/**`, the pre-existing PDCA record; no downloading/bundling/caching of model weights; do not fake a PASS; `.pdca/` is gitignored; fully autonomous (no user questions).
Assumptions: A-01 duration/sample-rate "match the reference tool" means target sample rate and input duration within codec framing padding; A-02 "4-stem model returns four named AudioBuffers" is proven through the public multi-stem contract without external htdemucs weights (those belong to AC-04); A-03 `flacenc`/`mp3lame-encoder` are fetchable through cargo in this environment.
Gaps: missing weights/refs/hardware → per-item terminal resolutions (above); encoder-crate availability → bounded probe at #6 start; non-overlap with the old campaign file → separate status/evidence paths (above).
Minimal solution: run as a collection; 6 items resolved as PLAN-terminal with evidence, 5 items executed sequentially on the current branch with per-item CHECK and commit, because #5/#7/#9/#10 share the `Separator` public API, the OLA adapter and `karaoke-cli`.
Risks: R-01 mp3lame C build toolchain; R-02 OLA/normalization/multi-stem/chunking share the same code paths and must keep the two-stem API and MelBand default numbers green; R-03 2 h fixture runtime under the full MelBand transformer on 8 CPU cores.
Stop conditions: per item — 3 attempts on the same defect or an irreducible resource blocker, tree left clean; per collection — every item has a final outcome, tree clean, summary in the collection status file.
Confidence: verified — issue set/criteria, repo state, single local checkpoint, absent PyTorch/audio-separator, no CUDA/Metal host; assumption — encoder crates buildable.
Plan-to-DO handoff: each DO receives this plan by reference, the item's AC-XX verbatim, the measurement method, the prohibitions, and one atomic commit per accepted item; DO chooses decomposition/implementation, CHECK chooses verification.

### Schedule (revision s1)

- DAG: no edges (no item needs another's output).
- Groups: `g0` = items #1,#2,#3,#4,#5,#6,#7,#8,#11 — all PLAN-terminal resolutions, no DO, no write surface; `g1` = items #9 → #10, serialized on `main` because #9 writes `src/config.rs`, `src/separate.rs`, `src/separator.rs`, `src/lib.rs`, `tests/core_tests.rs` and #10 writes `src/wav.rs`, `src/separate.rs`, `src/separator.rs`, `src/lib.rs`, tests/examples, `README.md`/`docs/` — overlapping surfaces, so one lane, order #9 then #10.
- Caps: `plan_cap: 4`, `lane_cap: 1`, `cap_reason`: single workspace on branch `main`, no isolated worktrees, direct write-set overlap of the only two live items.
- Merge strategy: none (serialized in place on `main`); no merge barrier.
- Integration-CHECK scope: full workspace verification on `main` after #9 and #10 are accepted and committed, covering both item contracts and preserving the pre-existing 41-test baseline.
- Schedule PLAN revision vector: entry plan `r1`; schedule revision `s1`.

### ALL-barrier

Satisfied — 9 items have a PLAN-terminal resolution (#1,#2,#3,#4,#5,#6,#7,#8,#11) and #9/#10 have `plan_state=ready`; the next allowed item is `#9`.

## Approval record

`not applicable: autonomous mode`

## DO result

`not reached: no DO yet` — superseded by 2026-11-08 integration evidence round: DO ran for #9 (accepted+committed 3c1eec0) and #10 (finalized incomplete, terminal=execution); 9 items PLAN-terminal infeasible (no DO). See item table and General verification. (original statement retained for audit)

### Item table

| item | group | branch | status | plan_state | form | contract | overlay | cycle_id | status_path | plan_revision | disposition | terminal | attempts | notes |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| #1 | g0 | main (current; no task branch) | incomplete | pending | cycle | pdca | pdca-coder | issue-1-1 | .pdca/status/issue-1-1.md | 1 | active | infeasible | 0 | PLAN-terminal resolution — original task is infeasible under the stated constraints (entry PLAN §4); no DO, no CHECK, no goal credit |
| #2 | g0 | main (current; no task branch) | incomplete | pending | cycle | pdca | pdca-coder | issue-2-1 | .pdca/status/issue-2-1.md | 1 | active | infeasible | 0 | PLAN-terminal resolution — original task is infeasible under the stated constraints (entry PLAN §4); no DO, no CHECK, no goal credit |
| #3 | g0 | main (current; no task branch) | incomplete | pending | cycle | pdca | pdca-coder | issue-3-1 | .pdca/status/issue-3-1.md | 1 | active | infeasible | 0 | PLAN-terminal resolution — original task is infeasible under the stated constraints (entry PLAN §4); no DO, no CHECK, no goal credit |
| #4 | g0 | main (current; no task branch) | incomplete | pending | cycle | pdca | pdca-coder | issue-4-1 | .pdca/status/issue-4-1.md | 1 | active | infeasible | 0 | PLAN-terminal resolution — original task is infeasible under the stated constraints (entry PLAN §4); no DO, no CHECK, no goal credit |
| #5 | g0 | main (current; no task branch) | incomplete | pending | cycle | pdca | pdca-coder | issue-5-1 | .pdca/status/issue-5-1.md | 1 | active | infeasible | 0 | PLAN-terminal resolution — original task is infeasible under the stated constraints; no DO, no CHECK, no goal credit |
| #6 | g0 | main (current; no task branch) | incomplete | pending | cycle | pdca | pdca-coder | issue-6-1 | .pdca/status/issue-6-1.md | 1 | active | infeasible | 0 | PLAN-terminal resolution — original task is infeasible under the stated constraints; no DO, no CHECK, no goal credit |
| #7 | g0 | main (current; no task branch) | incomplete | pending | cycle | pdca | pdca-coder | issue-7-1 | .pdca/status/issue-7-1.md | 1 | active | infeasible | 0 | PLAN-terminal resolution — original task is infeasible under the stated constraints; no DO, no CHECK, no goal credit |
| #8 | g0 | main (current; no task branch) | incomplete | pending | cycle | pdca | pdca-coder | issue-8-1 | .pdca/status/issue-8-1.md | 1 | active | infeasible | 0 | PLAN-terminal resolution — original task is infeasible under the stated constraints (entry PLAN §4); no DO, no CHECK, no goal credit |
| #9 | g1 | main (current; no task branch) | done | ready | cycle | pdca | pdca-coder | issue-9-1 | .pdca/status/issue-9-1.md | 1 | finalized | execution | 2 | accepted at n=2 after one CHECK fix round (D9-PITCH/D9-BATCH/D9-CONFIG-ERROR/D9-FADE-RANGE all refuted); commit 3c1eec0 feat: typed inference parameters with configurable overlap/step (#9) |
| #10 | g1 | main (current; no task branch) | incomplete | ready | cycle | pdca | pdca-coder | issue-10-1 | .pdca/status/issue-10-1.md | 1 | finalized | execution | 1 | CHECK not accepted: AC-10-2 long-scale peak-RSS evidence not measured (run-time budget); AC-10-1/3/4 met; no commit; patch preserved at .pdca/issues-milestone-1.0/rv1/item10/incomplete-patch/ |
| #11 | g0 | main (current; no task branch) | incomplete | pending | cycle | pdca | pdca-coder | issue-11-1 | .pdca/status/issue-11-1.md | 1 | active | infeasible | 0 | PLAN-terminal resolution — original task is infeasible under the stated constraints (entry PLAN §4); no DO, no CHECK, no goal credit |

### Groups table

| group | items (order) | worktree | branch/ref | status |
|---|---|---|---|---|
| g0 | [#1, #2, #3, #4, #5, #6, #7, #8, #11] | — (no write surface) | main (current branch) | pending |
| g1 | [#9, #10] | — (one group: current worktree) | main (current branch) | incomplete (stopped) |

## CHECK record

`not run: PLAN → ACT (6 PLAN-terminal resolutions) / pending items not yet planned` — superseded by 2026-11-08 integration evidence round: CHECK ran for #9 (accepted) and #10 (not accepted; AC-10-2 unverified); integrated-tree CHECK pending/not-run. Six is wrong: there are 9 PLAN-terminal resolutions. (original statement retained for audit)

### Integration CHECK (2026-11-08) — integrated tree `3c1eec0` (tree `2c0b911c…`)

Verdict: **PASS**. IC-1…IC-4 met. Previous findings refuted independently:
- IC-SNAPSHOT-RUN — refuted
- IC-RECORD-STALE — refuted
- IC-ORIGINAL-TEXT — refuted

Evidence pointers:
- `.pdca/issues-milestone-1.0/rv1/snapshot/runs/cargo-build.log` (exit 0; `.rc` = `snapshot/runs/cargo-build.rc`)
- `.pdca/issues-milestone-1.0/rv1/snapshot/runs/cargo-test.log` (exit 0; `.rc` = `snapshot/runs/cargo-test.rc`; 51 tests passed: unittests 11, core_tests 22, library_api 3, s1_spikes 11, whisper_tests 3, doc-tests 1)
- `.pdca/issues-milestone-1.0/rv1/snapshot/MANIFEST.md` (identity anchor HEAD = 3c1eec0, tree = 2c0b911c…)

Note: integrated-tree PASS verifies only the included work (accepted item #9 + pre-existing baseline); it does not verify plan-terminal items or #10.

## ACT record

ACT finalizes the 6 PLAN-terminal resolutions (no DO, no CHECK, no acceptance, attempts=0): — superseded by 2026-11-08 integration evidence round: there are 9 PLAN-terminal resolutions, not 6; #9 accepted+committed, #10 finalized incomplete. (original statement retained for audit)

- #1 `infeasible` — only weight file on host is `/home/alex/models/MelBandRoformer_fp32.safetensors` (sha256 bdc1643f…6327); no BS-RoFormer checkpoint; PyTorch reference not installed; downloading weights is forbidden by the user's constraints.
- #2 `infeasible` — no MDX-Net `.onnx` file and no audio-separator fixture anywhere on the host (searched repo, /home/alex/models, /home/alex/sources, ~/.cache, /tmp/opencode); downloading weights forbidden.
- #3 `infeasible` — no VR `.pth` checkpoint and no audio-separator parity fixture on the host; downloading weights forbidden.
- #4 `infeasible` — no `htdemucs` weights and no reference fixture on the host; downloading weights forbidden.
- #5 `infeasible` — the verbatim criterion requires `--single_stem` and `<input>_(Drums)_<model>.wav` naming to work in `karaoke-cli`; the CLI lives in the separate repo `/home/alex/sources/karaoke-cli` (HEAD a1ea0a4, dirty tree) which is off-limits, and `/home/alex/sources/candle-roformer` contains exactly one `Cargo.toml` with no bin target and no `src/main.rs`; narrowing the criterion to the library half is forbidden.
- #6 `infeasible` — the verbatim criterion requires `--output_format`, `--output_bitrate`, `--sample_rate` to exist in `karaoke-cli` and be rejected clearly when unsupported; that repo is off-limits (its `src/main.rs:70-182,221-227` accepts only `wav` and has no bitrate/rate flags), no CLI target exists in the allowed repo, and the library-only half cannot be substituted.
- #7 `infeasible` — the verbatim criterion requires `CLI flags match reference semantics and defaults` (including `--invert_spect`); flags live only in the off-limits `/home/alex/sources/karaoke-cli`, no CLI target exists in the allowed repo, and the library-only half cannot be substituted.
- #8 `infeasible` — criterion requires two checkpoints on a frozen fixture; exactly one checkpoint exists locally; downloading a second is forbidden.
- #11 `infeasible` — criterion requires a real CUDA host and a real Apple Silicon host; this host has neither (`docs/gates-report.md:59`, `README.md`), and the criterion itself says README must keep saying "not tested" until then.

Residual: #9, #10 remain `pending` with item PLANs ready (plan_state=ready); #5, #6, #7 are PLAN-terminal infeasible (attempts=0). — superseded by 2026-11-08 integration evidence round: #9 done/committed 3c1eec0 (not pending); #10 finalized incomplete (terminal=execution); #5/#6/#7 remain PLAN-terminal infeasible. (original statement retained for audit)

### ACT finalization (2026-11-08) — collection outcome `incomplete`

Collection outcome: **incomplete** (not full completion).

Accepted goals:
- #9 — accepted+committed at `3c1eec0` (feat: typed inference parameters with configurable overlap/step (#9)); 5 files.

Plan-terminal resolutions (no goal credit, attempts=0, no DO, no CHECK):
- #1, #2, #3, #4, #5, #6, #7, #8, #11 — `infeasible` (original task infeasible under stated constraints).

Execution failure:
- #10 — `terminal=execution`; AC-10-2 (long-scale peak-RSS) unverified; AC-10-1/3/4 met; patch preserved at `.pdca/issues-milestone-1.0/rv1/item10/incomplete-patch/` (tracked.patch + new/* + MANIFEST.md); not committed, not integrated.

Full completion is **not** claimed. The integrated-tree PASS (above) verifies only the included work (accepted #9 + pre-existing baseline), not plan-terminal items or #10. No push was performed.

Group statuses:
- `g0` = the nine plan-terminal resolutions (#1,#2,#3,#4,#5,#6,#7,#8,#11): no write surface, no merge.
- `g1` = `incomplete` (#9 done; #10 incomplete; group stopped).

No merge applies (single group, current branch `main`). No worktrees, branches or refs were created; nothing was deleted.

Preserved results (kept in place):
- item #10 patch: `.pdca/issues-milestone-1.0/rv1/item10/incomplete-patch/`
- evidence root: `.pdca/issues-milestone-1.0/rv1/` (with subpaths `checkpack/`, `attempt2/`, `item10/`, `snapshot/`, `baseline.log`, `do-test*.log`)

## Revision history

`none`

## Defect history

`none`

## Changed files

`.pdca/status/collection-issues-milestone-1.0.md → created this cycle`

## Git state

`status path: .pdca/status/collection-issues-milestone-1.0.md — modified this cycle · untracked (inside gitignored .pdca/); durable on disk, not committed. Single commit on branch main: 3c1eec0f231a34f1fe58c05f140bda0fabc6be77 "feat: typed inference parameters with configurable overlap/step (#9)". No other commits, no push. Working tree clean.`

## Progress log

- 2026-11-08T08:47:41Z | PLAN | revision r1 | attempt n0 | entry PLAN completed: 6 PLAN-terminal infeasible resolutions (#1,#2,#3,#4,#8,#11) and 5 pending items (#5,#6,#7,#9,#10) | this file
- 2026-11-08T08:47:41Z | ACT | revision r1 | attempt n0 | ACT finalized the 6 PLAN-terminal resolutions; no DO, no CHECK, no acceptance | this file
- 2026-11-08T09:01:48Z | PLAN | revision r1 | attempt n0 | item PLANs ready for #9,#10; #5,#6,#7 reclassified PLAN-terminal infeasible; ALL-barrier satisfied; schedule s1 persisted (g0 9 terminal, g1 #9→#10) | this file
- 2026-11-08T11:15:00Z | ACT | revision r1 | attempt n2 | item #9 accepted+committed: CHECK accepted (all criteria met, 4 defect keys refuted); commit 3c1eec0 feat: typed inference parameters with configurable overlap/step (#9); 5 files; tree clean; next allowed item #10 | this file

- 2026-11-08T13:04:48Z | ACT | revision r1 | attempt n=1 | item #10 finalized incomplete (terminal=execution): CHECK not accepted (AC-10-2 unverified, run-time budget); #9 done at 3c1eec0; g1 stopped; no commit; patch preserved at .pdca/issues-milestone-1.0/rv1/item10/incomplete-patch/ | .pdca/status/issue-10-1.md
- 2026-11-08T13:12:50Z | ACT | revision r1 | attempt n=0 | integration evidence round: corrected stale record statements (DO not started / CHECK never ran / "six" → 9 resolutions / #9,#10 pending) and recorded snapshot build/test runs (rc 0/0, 51 tests); see Current state and snapshot/runs/README.md | this file
- 2026-11-08T13:17:49Z | ACT | revision r1 | attempt n=0 | collection closed: outcome incomplete; integration CHECK PASS on 3c1eec0 (tree 2c0b911c…, IC-1…IC-4 met; IC-SNAPSHOT-RUN/IC-RECORD-STALE/IC-ORIGINAL-TEXT refuted); accepted #9 at 3c1eec0; 9 PLAN-terminal infeasible; #10 execution-incomplete (AC-10-2 unverified, patch preserved); g1 stopped; tree clean; no push | this file

## Input (verbatim briefs)

1. #1 `feat: support BS-RoFormer and the general MDXC/RoFormer family` — Done when: a real BS-RoFormer checkpoint loads, passes strict validation and separates audio; numerical parity against a pinned PyTorch reference (same thresholds as the MelBand parity gate); README lists supported families and config files.
2. #2 `feat: MDX-Net (ONNX) architecture` — Done when: a published MDX-Net ONNX file (supplied locally, checksum-checked by caller) separates audio on CPU; output matches audio-separator on a frozen fixture within agreed tolerance; dependency documented in docs/dependency-audit.md with licence.
3. #3 `feat: VR/UVR architecture (.pth models)` — Done when: a chosen VR checkpoint converts with a pinned checksum and separates audio; parity fixture against audio-separator; converter covered by tests like the v7 one.
4. #4 `feat: Demucs (4-stem and 6-stem) architecture` — Done when: htdemucs weights load with strict validation and separate a fixture into 4 stems; parity against the reference implementation and per-stem output naming.
5. #5 `feat: expose multi-stem output (drums, bass, piano, guitar, other)` — Done when: a 4-stem model returns four named AudioBuffers through the library API; naming and `--single_stem` work in karaoke-cli.
6. #6 `feat: output encoders beyond WAV f32 (FLAC/MP3/M4A, bitrate, sample rate)` — Done when: FLAC and MP3 round-trip through `read_audio` and match the reference tool's duration/sample rate; `--output_format`, `--output_bitrate`, `--sample_rate` exist in `karaoke-cli` and are rejected with a clear error when unsupported; new dependencies added to docs/dependency-audit.md.
7. #7 `feat: level processing (normalization, amplification, spectral inversion)` — Done when: the invariant is re-stated precisely with tests for both "exact residual" and "normalized" modes; CLI flags match reference semantics and defaults.
8. #8 `feat: multi-model ensembles and ensemble presets` — Done when: at least avg_wave and median_wave match the reference on a frozen fixture with two checkpoints; a preset produces the documented stem set on a real song.
9. #9 `feat: model inference parameters (segment, overlap, batch, pitch shift, denoise, TTA)` — Done when: overlap/step configurable and a sweep test shows expected pass counts; defaults reproduce current numbers when unset.
10. #10 `feat: bounded-memory processing for long inputs (--chunk_duration)` — Done when: peak RSS roughly constant in input length for a fixed chunk duration; output length and residual invariant hold and boundary quality documented.
11. #11 `test: CUDA/Metal smoke tests and a validated reduced-precision path (gate G2)` — Done when: at least one real CUDA host and one Apple Silicon host run the smoke suite, logs committed as evidence, README/gates report state exactly what passed; until then README keeps saying "not tested".

## General verification and recovery

- Verification state: `partial` (#9 accepted+committed; #10 finalized incomplete) — integrated-tree CHECK PASS on 3c1eec0 (included work only)
- Last common C: 3c1eec0 (item #9 commit; accepted work only)
- Blocking reason: none
- Defect history: none
- Related corrective task status file: none
- Next allowed step: none — collection closed (incomplete); no further items; no push performed.
- Completeness (all original goals accepted): no — 9 PLAN-terminal resolutions + 1 execution-incomplete item (#10).
- Integrated-tree CHECK verdict: PASS (IC-1…IC-4 met; IC-SNAPSHOT-RUN/IC-RECORD-STALE/IC-ORIGINAL-TEXT refuted); evidence `snapshot/runs/cargo-build.log`, `snapshot/runs/cargo-test.log`, their `.rc`, `snapshot/MANIFEST.md`.
- Incomplete items: `#1`, `#2`, `#3`, `#4`, `#5`, `#6`, `#7`, `#8`, `#11` (PLAN-terminal infeasible); `#10` (finalized incomplete, terminal=execution, AC-10-2 unverified)
- Preserved results: item #9 accepted+committed at 3c1eec0 (5 files: src/config.rs, src/separate.rs, src/separator.rs, src/lib.rs, tests/core_tests.rs); item #10 patch preserved at `.pdca/issues-milestone-1.0/rv1/item10/incomplete-patch/` (tracked.patch + new/* + MANIFEST.md; base 3c1eec0); no commit for #10; evidence root `.pdca/issues-milestone-1.0/rv1/` (with subpaths) kept in place.
- Group g1 (`#9` → `#10`): stopped — item #9 done (commit 3c1eec0), item #10 incomplete (execution); no further items, no merge (single group, current branch). No worktrees, branches or refs were created; nothing was deleted.

## Done / Verified / Incomplete

- Done: #9 (accepted+committed at 3c1eec0)
- Verified: PASS — integrated-tree CHECK on 3c1eec0 (included work only); 51 tests passed; evidence `snapshot/runs/cargo-build.log`, `snapshot/runs/cargo-test.log`, their `.rc`, `snapshot/MANIFEST.md`
- Incomplete: #1, #2, #3, #4, #5, #6, #7, #8, #11 (PLAN-terminal infeasible, attempts=0, no goal credit); #10 (finalized incomplete, terminal=execution, AC-10-2 unverified, not committed, not integrated)

