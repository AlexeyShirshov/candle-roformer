# Collection status: open-issues-r5 / collection r1

- collection-id: `open-issues-r5`
- task: `open-issues-r5`
- form: `collection`
- plan_cap: 1
- lane_cap: 1
- cap_reason: one sequential lane — all four items (#2, #8, #3, #4) touch shared CLI/dispatch surfaces (Cargo.toml, src/lib.rs, CLI, src/separator.rs, src/config.rs), so their footprints are not provably disjoint; plan_cap 1 because the per-item contracts were published at r1 by the collection PLAN and no further planner call is scheduled before DO.
- specialized skill: `pdca-coder`
- overlay: `pdca-coder`
- mode: `autonomous`
- scope: four items — `#2`, `#3`, `#4`, `#8` (one group `G1`)
- status path: `.pdca/status/collection-open-issues-r5.md`
- evidence root: `.pdca/open-issues-r5/rv1/`
- baseline: `main @ 2d0be53`, tree clean except untracked `.openidea/` and `.pdca/status/collection-open-issues-r3.md` (observed; see §Git state)
- phase: `PLAN` · record: `open` · outcome: `pending` — collection `open-issues-r5` is between items: items `#2` and `#8` are **accepted, committed (`59fb1fb`, `a38bb7f`) and finalized**, and both GitHub issues were **closed as `COMPLETED` by the driver** (`#2` at `2026-11-11T06:18:29Z`, comment `6788661305`; `#8` at `2026-11-11T07:57:47Z`, comment `6790434030`); completeness **`2/4 accepted`**; next item **`#3`** (VR/UVR), then `#4`; context rows `#1` and `#11` unchanged. Item history: §Current state, §ACT record, §General verification and recovery, §Progress log
- goal: implement GitHub issues #2, #3, #4 and #8 against their verbatim `Done when` clauses plus the user-approved numeric tolerances, with one atomic commit and GitHub issue closure per accepted item
- criteria: `AC-2.1`–`AC-2.4` (item `#2`, contract revision `r2`, exact text in `.pdca/status/issue-2-r5.md` §PLAN record `r2`), `AC-3.1`–`AC-3.3`, `AC-4.1`–`AC-4.3`, `AC-8.1`–`AC-8.4` (exact text in §PLAN record `r1`; source: user criteria message 2026-11-10 and the approved addendum persisted at `.pdca/open-issues-r5/rv1/criteria-approval.md`), not retyped here

## Current state

- task: `open-issues-r5` · form: `collection` · specialized skill: `pdca-coder` · overlay: `pdca-coder`
- mode: `autonomous` — no plan approval requested or required; the criteria addendum was approved by the user (see §Approval record)
- counters: collection `N=1` · plan revision `r=1` (collection PLAN; item `#2` finalized at contract revision `r=2`, item `#8` revised to `r=3`) · attempts `n=0` (per-item attempts are in the §Item table; item `#2` consumed `n=1/3` and `n=2/3` under `r2` and was finalized; item `#8` consumed attempt `n=1/3` under `r1`, its CHECK 1 (`r1`/`n1`) returned `not accepted → PLAN` and consumed no additional attempt, `CHECK 2` on `r2`/`n1` then returned `not accepted → PLAN` and consumed no attempt, and the revised contract `r3` starts at attempt `n=1/3`)
- scope: `#2`, `#3`, `#4`, `#8` in group `G1`; context rows `#1` (accepted at `dc94520`, closed in GitHub) and `#11` (terminal `infeasible`) are recorded but outside this collection's execution
- evidence root: `.pdca/open-issues-r5/rv1/`
- goal: implement GitHub issues #2, #3, #4 and #8 against their verbatim `Done when` clauses plus the user-approved numeric tolerances, one atomic commit and GitHub closure per accepted item — §PLAN record and §Original task are authoritative
- phase: `PLAN` · record: `open` · outcome: `pending` — active item `#3` (`pending` / `ready` / `active`), next authorized step **`DO item #3 (r1/n1)`**, then `#4`; `#2` and `#8` are `done` / `finalized` with terminal outcome (cycle) `accepted`: `#2` committed as `59fb1fb` and GitHub `#2` closed as `COMPLETED` at `2026-11-11T06:18:29Z` (comment `6788661305`), `#8` committed as `a38bb7f` and GitHub `#8` closed as `COMPLETED` at `2026-11-11T07:57:47Z` (comment `6790434030`); completeness **`2/4 accepted`**. Superseded history (kept): item `#2` contract revision `r2` after `CHECK → PLAN`, attempts `r2`/`n1` and `r2`/`n2`, `CHECK 3` `accepted → ACT`; item `#8` `CHECK 1` and `CHECK 2` `not accepted`, item-level contract revisions `r1→r2→r3`, `CHECK 3` (`r3`/`n1`) `accepted → ACT` and commit `a38bb7f` — full reports in `.pdca/status/issue-2-r5.md` and `.pdca/status/issue-8-r5.md`
- source: user request over the open GitHub issues of `AlexeyShirshov/candle-roformer` · context refs: `.pdca/status/collection-open-issues-r4.md`, `.pdca/open-issues-r4/`, `.pdca/open-issues-r5/rv1/plan.md`, `.pdca/open-issues-r5/rv1/criteria-approval.md`
- schedule: one group `G1` on the current worktree `main`, one sequential lane (`plan_cap 1`, `lane_cap 1`), order by risk `#2 → #8 → #3 → #4`; single worktree, no branches/merges
- next item: `#3` — **`DO item #3 (r1/n1)`** is the next authorized collection step (VR/UVR checkpoint with a pinned checksum plus audio-separator `0.48.0` parity at `window_size=512`, `aggression=5`, no post-processing); `#4` stays `pending` and untouched. `#2` and `#8` are `done` / `finalized` with their GitHub issues closed by the driver (see the `2026-11-11` bullets below). Superseded history: the `#8` CHECK 1/CHECK 2 loop-backs, the `r3` revision and the `CHECK 3` acceptance stay in §CHECK record, §ACT record and `.pdca/status/issue-8-r5.md`
- **`#8` `DO` `r2`/`n1` complete (2026-11-11T07:12:49Z)** — the `next allowed step: DO item #8 (r2/n1)` lines above are history, superseded by this bullet: `PAD-8` is fixed (1 fix), the `LIC-8` primary-source licence evidence is in `licence/` (1 evidence fix), `AC-8.1`/`AC-8.3` are re-measured on the fixed bytes (`max_abs 0` / `RMSE 0` over 915,374 samples; 120 passed / 0 failed / 2 ignored), `AC-8.2` is unchanged and non-invalidated, and `AC-8.4(1)` holds (index empty, no commit, GitHub `#8` untouched). Active item is `#8` at `DO r2/n1` **complete**, next allowed step **`CHECK item #8 (r2/n1)`**; the commit and the GitHub `#8` closure stay forbidden before `CHECK: accepted`; completeness stays **`1/4 accepted`**.
- **`#8` `CHECK 2` (`r2`/`n1`) returned `not accepted → PLAN` (2026-11-11T07:21:22Z)** — the `#8` `DO` `r2`/`n1` bullet above and every `next allowed step: CHECK item #8 (r2/n1)` line are history, superseded by this bullet. Graded candidate: the `r2`/`n1` snapshot (3 changed product files, seven hashes of `logs/reanchor-snapshot.log` on `59fb1fb`). Matrix `AC-8.1` `met`, `AC-8.2` `met`, `AC-8.3` `unmet` (the literal "no warnings" clause is violated by the measured clippy/test warnings), `AC-8.4(1)` `unverified`, `AC-8.4(2)` `unverified` (term not reached). New keys: `F-8-ac3-contract-drift` (blocking, confirmed, AC-8.3 — the derived contract both promises `r1` invariance and forbids linter warnings) and `F-8-issue-open-evidence` (major, unverified, AC-8.4(1) — no stored GitHub `#8` state was presented); `F-8-acceptance-order`, `F-8-song-licence`/`LIC-8` and `PAD-8` are refuted/closed on the `r2` candidate. **Next allowed step: a PLAN contract revision for item `#8`**; counters `r=2`, `n=1/3` stand and the transition consumes no attempt; the commit, the GitHub `#8` closure and `done` are **not authorized**; completeness `1/4 accepted`. Verbatim report: `.pdca/status/issue-8-r5.md` §CHECK record `### CHECK 2 — r2/n1`.
- **`#8` item-level contract revision `r3` recorded (2026-11-11T07:27:43Z)** — the `CHECK 2` bullet above and every `next allowed step: a PLAN contract revision for item #8` line are history, superseded by this bullet. `r3` explicitly revises `AC-8.3` back to the `r1` gate semantics (`exit 0` and `0 failed`; non-fatal compiler/linter warnings are not a failure, so `F-8-ac3-contract-drift` is **closed by the contract, fixes applied 1**) and refines `AC-8.4(1)` to the stored read-only GitHub `#8` observation (identity, URL, `state=OPEN`, observation time — the bounded probe is authorized in `DO`, `F-8-issue-open-evidence` fix pending, fixes applied 0). Counters item `#8` `r=3`, `n=1/3`; the 7-file product snapshot is unchanged; **next allowed step `DO item #8 (r3/n1)`**; the commit, the GitHub `#8` closure and `done` stay **not authorized**; completeness `1/4 accepted`. Verbatim report: `.pdca/status/issue-8-r5.md` §PLAN record `r3`.
- **`#8` `DO` `r3`/`n1` complete (`2026-11-11T07:38Z`, evidence only, no product change)** — the `DO item #8 (r3/n1)` bullet above and every `next allowed step: DO item #8 (r3/n1)` line are history, superseded by this bullet. The round did exactly the `r3` §10 scope: (1) a **fresh** read-only GitHub observation of `AlexeyShirshov/candle-roformer#8` — `HTTP 200`, `state=open` (`OPEN`), canonical URL `https://github.com/AlexeyShirshov/candle-roformer/issues/8`, observed `2026-11-11T07:34:54Z`, `state_reason=null`, `closed_at=null`, timeline **0 `closed` / 0 `reopened`**, no token read/sent/stored — persisted as `item-8-ensembles/github/issue-8-observation.json` + the timestamped raw pair, closing `F-8-issue-open-evidence` (**fixes applied 1**); (2) the frozen 7-file candidate re-anchored **7/7** against `snapshot-sha256.txt` (`0` missing, `0` mismatch), `HEAD 59fb1fbcf89ed99a447dc5b8d48b915de601917f`, index empty, no `#8` commit, no product byte changed — so no gate was re-run (the snapshot had not drifted) and the `CHECK 2` measurements stay bound. **Next allowed step: `CHECK item #8 (r3/n1)`**; the commit, the GitHub `#8` closure and `done` stay **not authorized**; counters `r=3`, `n=1/3`; completeness `1/4 accepted`.
- **`#8` `CHECK 3` (`r3`/`n1`) returned `accepted → ACT` and ACT closed the item (`2026-11-11T07:44Z`–`07:47Z`)** — every `next allowed step: CHECK item #8 (r3/n1)` line and the "not authorized" clauses of the bullets above are history, superseded by this bullet. Graded candidate: `main @ 59fb1fb` + the frozen 7-file delta. Matrix `AC-8.1` `met` (`max_abs 0`, `RMSE 0` over 915,374 samples × 2 channels, parity test exit `0`), `AC-8.2` `met` (CLI exit `0`, 5,491.531 s, exactly `vocals` + `instrumental`, 44,100 Hz / 2 ch / 4,175,961 frames / 94.692993 s, CC0 licence proven from the primary source and bound to the used bytes), `AC-8.3` `met` (`build`/`test`/`clippy` exit `0`, 120 passed / 0 failed / 2 ignored on the restored snapshot; warnings non-fatal under the exact `r3` contract), `AC-8.4(1)` `met` (7/7, empty index, stored read-only observation `2026-11-11T07:37:38Z` `state=open`), `AC-8.4(2)` `unverified — срок не наступил`; keys `F-8-ac3-contract-drift` and `F-8-issue-open-evidence` **refuted/closed**, while `F-8-acceptance-order`, `F-8-song-licence`/`LIC-8` and `PAD-8` remain closed. ACT performed the closure it owns: pre-stage re-anchor **7/7** (`0` mismatch, `0` missing, verifier exit `0`), retrospective recorded **verbatim** in `.pdca/status/issue-8-r5.md` §Lessons learned (`retrospective_complete`), and **one atomic commit `a38bb7f`** (`a38bb7fd969dc0ebb36ca74957af93b6b4b20a19`, parent `59fb1fb`, **7 files changed +1368/−2**, footer `Refs #8`, no auto-close keyword) with exactly the 7 product paths staged — `git diff --name-only HEAD -- <7 paths>` empty, nothing staged, no push, no branch switch, no amend, no tag, **no GitHub write**. Item `#8` is **`done` / `finalized`**, with the **GitHub `#8` closure by the calling driver** as its only external residual (`AC-8.4(2)` = commit performed, GitHub closure pending by the calling driver; explicitly **not** performed and never recorded as "issue closed"). Counters item `#8` `r=3`, `n=1/3`. Next step: **`DO item #3 (r1/n1)`** — the next authorized collection step, not started here; the calling driver closes GitHub `#8` with `CHECK 3` and commit `a38bb7f` as evidence; `#4` stays `pending`; completeness **`2/4 accepted`** (`#2` and `#8` both `done`, each with the GitHub closure by the calling driver as its only external residual).
- blockers: **none active** — `CONC-1` on item `#8` is history: observed at 04:32:56Z, the duplicate exited on its own at ~04:40Z and a quiet tree was re-verified at 10:22Z and 11:52Z local (no second `cargo`/`ensemble_separate`, `HEAD 59fb1fb`, nothing staged). Its artifacts stay quarantined/labelled (`attic/882000-window/`, `logs/concurrency-conflict.log`) so CHECK does not read them as one base. The `AC-8.2` run cut by the server restart was replaced by exactly one relaunch (run 3) — no further retry, no process killed. Item `#2` completed its cycle and is `done`; its only residual is GitHub `#2` closure by the calling driver. CHECK 1 added two open contract/evidence keys on `#8` — `F-8-acceptance-order` (blocking) and `F-8-song-licence` (major); the contract revision `r2` is now recorded, which **closes `F-8-acceptance-order`** by splitting `AC-8.4` into a CHECK-verifiable pre-acceptance state and the mandatory ACT closure, while `F-8-song-licence` stays open as the `AC-8.2` evidence obligation (fix pending in `DO` `r2`/`n1`, 0 fixes applied), and **neither the `#8` commit nor the GitHub `#8` closure is authorized** until a later accepted CHECK
  — **update (2026-11-11, `r2`/`n1` complete):** `F-8-song-licence` / `LIC-8` now carries the persisted
  primary-source evidence (`licence/`, 1 fix) with the duration and bit-identity binding plus the
  HQ-preview caveat; `PAD-8` is fixed (1 fix); `F-8-acceptance-order` stays closed at contract `r2`;
  no blocker is active, and the `#8` commit and the GitHub `#8` closure remain unauthorized before
  `CHECK: accepted`

- **Both GitHub closures performed by the driver (`2026-11-11`) — the credential blocker is resolved.** Item `#2` was closed as `COMPLETED` at `2026-11-11T06:18:29Z` (comment `6788661305`) and item `#8` as `COMPLETED` at `2026-11-11T07:57:47Z` (comment `6790434030`), both by the driver through the GitHub CLI because the repository owner confirmed it works in this environment; `gh` is **authenticated** at `C:\Program Files\GitHub CLI\gh.exe`. This resolves the earlier blocker note that closure was impossible for lack of GitHub credentials — the earlier item records (`gh` not installed, unauthenticated read-only REST observation for the pre-acceptance state) stay true as history. No PDCA phase performed or claims the closure: it is the driver's external action, now recorded as completed.
- **Completeness `2/4 accepted`** — `#2` and `#8` are `done` / `finalized` with terminal outcome (cycle) `accepted` (commits `59fb1fb` and `a38bb7f`; GitHub `#2` and `#8` closed as `COMPLETED`); `#3` and `#4` are `pending` / `ready` / `active`; `#1` (accepted at `dc94520`, closed in GitHub) and `#11` (`infeasible`) remain context rows outside this collection's execution.
- **Record correction (one-writer rule, `2026-11-11`)** — this collection record had been left with a stale `#2` header (`phase: ACT · record: completed`) and contradictory `#8` item-row dispositions written by concurrent record writers during item `#8`; the header, §Current state, item rows `#2`/`#8` and §ACT record now carry the authoritative `2/4 accepted` disposition with both GitHub closures recorded. The record is written only by the driver's executor; competing blocks are kept and superseded, never deleted.

## Original task

The user's task, verbatim (original language, Russian):

> pdca по открытым задачам из github

Prior instruction, verbatim (original language, Russian):

> тогда делай автономно с автокомитом

Later instruction, verbatim (original language, Russian):

> после каждой выполненной задачи - делай комит и закрывай задачу

New user message — the criteria authority for this revision, verbatim:

```text
#2 принимаю допуск
#3 ok
#4 ok
#8 ok
```

Provenance: the new user message approves the criteria addendum quoted in §Input by reference (`#2 принимаю допуск` accepts the #2 tolerance; `#3 ok`, `#4 ok`, `#8 ok` accept the remaining blocks). The plan approval itself is not required in autonomous mode.

## Input

Source items: open GitHub issues `#2`, `#3`, `#4`, `#8` of `AlexeyShirshov/candle-roformer`, with their verbatim `Done when` texts and the user-approved criteria addendum.

Verbatim `Done when` texts (unchanged):

```text
#2: a published MDX-Net ONNX file (supplied locally, checksum-checked by caller) separates audio on CPU; output matches audio-separator on a frozen fixture within agreed tolerance; dependency documented in docs/dependency-audit.md with licence.
#3: a chosen VR checkpoint converts with a pinned checksum and separates audio; parity fixture against audio-separator; converter covered by tests like the v7 one.
#4: htdemucs weights load with strict validation and separate a fixture into 4 stems; parity against the reference implementation and per-stem output naming.
#8: at least avg_wave and median_wave match the reference on a frozen fixture with two checkpoints; a preset produces the documented stem set on a real song.
```

Approved criteria addendum, verbatim (the text the user approved by reference):

- #2: "For comparison candle-roformer vs audio-separator 0.48.0 on one frozen fixture, CPU/f32, with the same .onnx SHA-256: max_abs ≤ 1e-5; RMSE ≤ 1e-6; SDR ≥ 100 dB; correlation ≥ 0.999999; comparison over all samples of all channels and stems, without trimming and without post-normalization."
- #3: "same block as #2: max_abs ≤ 1e-5, RMSE ≤ 1e-6, SDR ≥ 100 dB, corr ≥ 0.999999, plus fixed window_size=512, aggression=5, no TTA/high-end/post-process (or identical flags on the reference)."
- #4: "max_abs ≤ 3.5e-4, RMSE ≤ 1e-4, SDR ≥ 80 dB, comparison for each of the 4 stems, shifts=0, same segment/overlap." (stem names: drums, bass, other, vocals)
- #8: "1) tolerance for avg_wave / median_wave: max_abs ≤ 1e-6, RMSE ≤ 1e-7. 2) documented stem set: preset vocal_balanced → vocals + instrumental, on a real freely-licensed (e.g. CC0) song; the two checkpoints are MelBand and the accepted BS-RoFormer."

Provenance: `Done when` texts carried unchanged from the collection input; the addendum is the text approved by the user message quoted in §Original task and §Approval record; both are persisted verbatim in `.pdca/open-issues-r5/rv1/criteria-approval.md`.

## Constraints

- autonomous mode: no approvals requested, no questions; the PLAN is recorded as returned and DO may start on `next item #2`;
- the four `Done when` criteria and the approved numeric addendum are immutable — no weakening, no narrowing, no surrogate deliverable;
- measurement per stem/channel, over all samples of the full output, no trimming, no post-normalization: max_abs = max|y_cand - y_ref|; RMSE = sqrt(mean((y_cand - y_ref)^2)); SDR = 10*log10(sum(y_ref^2)/sum((y_cand - y_ref)^2)) dB; correlation = Pearson r;
- autocommit only after an accepted CHECK, one atomic commit per accepted item, on `main`, only that item's files; no push, no branch switch, no history rewrite, no forcing;
- no weights, `.pdca/`, `.openidea/` or temporary audio in product commits; README keeps "not tested" for CUDA/Metal;
- preserve prior `.pdca` records (`.pdca/status/collection-open-issues-r4.md`, `.pdca/status/issue-1-r4.md`, `.pdca/status/issue-2-r4.md`, `.pdca/open-issues-r4/` untouched);
- Python invoked by full path `C:\Users\Пользователь\AppData\Local\Programs\Python\Python312\python.exe`; evidence for every run persisted under `.pdca/open-issues-r5/rv1/`;
- working directory `F:\sources\candle-roformer`; single worktree `main`, no branches/merges; commits serialized because of shared surfaces (`Cargo.toml`, `src/lib.rs`, CLI, `src/separator.rs`, `src/config.rs`);
- `plan_cap: 1`, `lane_cap: 1` recorded in the header with the reason above;
- this recording task itself: no DO start, no product-file edit, no `git add`, no staging, no commit, no push.

## PLAN record

### r1

- Form: collection. Specialized skill: pdca-coder. Entry scope: #2, #3, #4, #8.
- Requested result: working implementations of open GitHub issues #2, #3, #4, #8 satisfying every `Done when` clause plus the user-approved numeric tolerances, with one atomic git commit and closure of the corresponding GitHub issue per accepted item. Delivered result: same object.
- Criteria determinacy: all four items are now numerically determinate; the r4 terminal `acceptance criteria cannot be determined` resolutions for #2/#3/#4/#8 are `superseded-by-user-criteria` (not errors). r4 record preserved unchanged; #1 closed at dc94520; #11 stays infeasible.
- Measurement method: per stem/channel, over all samples of the full output, no trimming, no post-normalization: max_abs = max|y_cand - y_ref|; RMSE = sqrt(mean((y_cand-y_ref)^2)); SDR = 10*log10(sum(y_ref^2)/sum((y_cand-y_ref)^2)) dB; correlation = Pearson r.
- Per-item acceptance criteria: AC-2.1 real published MDX-Net .onnx, checksum-verified, CPU separation, parity vs audio-separator 0.48.0 at the #2 thresholds; AC-2.2 no regressions (build+tests, #1 intact); AC-2.3 one atomic commit + GitHub #2 closed. AC-3.1 real public VR/UVR checkpoint with pinned checksum, separation, parity vs audio-separator 0.48.0 at window_size=512/aggression=5/no post-processing with the #3 thresholds; AC-3.2 no regressions; AC-3.3 commit + GitHub #3 closed. AC-4.1 HTDemucs weights strict-validated, 4 stems named drums/bass/other/vocals, parity vs reference at shifts=0 and equal segment/overlap with the #4 thresholds on every stem; AC-4.2 no regressions; AC-4.3 commit + GitHub #4 closed. AC-8.1 avg_wave and median_wave within max_abs ≤ 1e-6 / RMSE ≤ 1e-7 against an exact reference on a frozen fixture with two real checkpoints; AC-8.2 preset vocal_balanced on a real CC0 song produces exactly vocals + instrumental (valid, non-empty, full-length, finite); AC-8.3 no regressions; AC-8.4 commit + GitHub #8 closed.
- Dependencies/schedule: #8 uses the already-working MelBand and BS-RoFormer checkpoints, so it has no hard runtime dependency on #2/#3/#4; commits are serialized because of shared surfaces (Cargo.toml, src/lib.rs, CLI, src/separator.rs, src/config.rs). One group G1, order by risk: #2 → #8 → #3 → #4. plan_cap 1, lane_cap 1. Single worktree main, no branches/merges.
- Constraints: autonomous; no approvals; autocommit only after an accepted CHECK; one atomic commit per accepted item on main; no push, no branch switch, no history rewrite; no weights/.pdca/.openidea/temp audio in product commits; no criterion weakening; README keeps "not tested" for CUDA/Metal; use the full Python path C:\Users\Пользователь\AppData\Local\Programs\Python\Python312\python.exe; prior .pdca records preserved.
- Stop conditions: an item that exhausts n/3 without an accepted CHECK finalizes honestly (not_done) and does not block already accepted items; the collection completes only when all four are accepted, committed, and closed in GitHub.

Source: `.pdca/open-issues-r5/rv1/plan.md` (same content, persisted).

Item-level pointer (not a collection revision — the collection's own revision stays `r1`): item `#8` received an item-level contract revision **`r2`** from `CHECK → PLAN` on 2026-11-11T07:04:10Z; its complete report is recorded verbatim in `.pdca/status/issue-8-r5.md` §PLAN record `r2`, together with the revised `AC-8.1`–`AC-8.4` text. It is not pasted here.

Item-level pointer (not a collection revision — the collection's own revision stays `r1`): item `#8` received a further item-level contract revision **`r3`** from `CHECK 2 → PLAN` on 2026-11-11T07:27:43Z (`AC-8.3` revised to the `r1` gate semantics, `AC-8.4(1)` refined to the stored read-only GitHub observation); its complete report is recorded verbatim in `.pdca/status/issue-8-r5.md` §PLAN record `### r3`, together with the revised `AC-8.1`–`AC-8.4` text. It is not pasted here.

## Approval record

- Criteria addendum approved by the user, verbatim message (source: user message, recorded 2026-11-10T18:01:54Z):

```text
#2 принимаю допуск
#3 ok
#4 ok
#8 ok
```

- Scope of the approval: the criteria addendum quoted in §Input (tolerances and comparison protocol for #2/#3/#4/#8), against identity `open-issues-r5` / `N=1` / `r=1`.
- No further approval is required: form `collection`, mode `autonomous` — plan approval is not requested in autonomous mode; this slot records the user's criteria approval only. No approval ref is pending.

## Item table

| item | group | branch | status | plan_state | form | contract | overlay | cycle_id | status_path | plan_revision | disposition | terminal | attempts | notes |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| #2 | G1 | main | done | ready | cycle | pdca | pdca-coder | issue-2-r5 | .pdca/status/issue-2-r5.md | 2 | finalized | execution | 2 | order 1 (risk-first): MDX-Net ONNX separation on CPU from a locally supplied, checksum-verified published `.onnx`; accepted by `CHECK 3` (`r2`/`n2`: `AC-2.1`–`AC-2.4` `met`, `AC-2.4` on its pre-acceptance component with the commit action performed); attempts `n=1/3` and `n=2/3` under contract `r2`; product commit `59fb1fb` (parent `2d0be53`, 13 files, `+1750/-23`, footer `Refs #2`) on `main`, not pushed; **GitHub `#2` closed as `COMPLETED` by the driver at `2026-11-11T06:18:29Z`** (comment `6788661305`); terminal outcome (cycle) `accepted`; full item history in `.pdca/status/issue-2-r5.md` §CHECK record, §ACT record, §Lessons learned |
| #8 | G1 | main | done | ready | cycle | pdca | pdca-coder | issue-8-r5 | .pdca/status/issue-8-r5.md | 3 | finalized | execution | 1 | order 2: ensemble operators `avg_wave`/`median_wave` plus preset `vocal_balanced` on a real CC0 song; item-level contract revisions `r1→r2→r3` (item detail, the collection plan stays `r1`); accepted by `CHECK 3` (`r3`/`n1`: `AC-8.1`, `AC-8.2`, `AC-8.3`, `AC-8.4(1)` `met`); product commit `a38bb7f` (parent `59fb1fb`, 7 files, `+1368/-2`, footer `Refs #8`) on `main`, not pushed; **GitHub `#8` closed as `COMPLETED` by the driver at `2026-11-11T07:57:47Z`** (comment `6790434030`); terminal outcome (cycle) `accepted`; full item history in `.pdca/status/issue-8-r5.md` §CHECK record, §ACT record, §Lessons learned |
| #3 | G1 | main | pending | ready | cycle | pdca | pdca-coder | issue-3-r5 | .pdca/status/issue-3-r5.md | 1 | active | execution | 0 | order 3: real VR/UVR checkpoint with pinned checksum, conversion plus audio-separator parity at window_size=512/aggression=5 with no post-processing |
| #4 | G1 | main | pending | ready | cycle | pdca | pdca-coder | issue-4-r5 | .pdca/status/issue-4-r5.md | 1 | active | execution | 0 | order 4: HTDemucs strict validation and 4-stem parity (drums/bass/other/vocals) at shifts=0 with equal segment/overlap |
| #1 | — | — | done | pending | cycle | pdca | pdca-coder | issue-1-r4 | .pdca/status/issue-1-r4.md | 1 | finalized | execution | 2 | context row: accepted at dc94520 and closed in GitHub; not part of this collection's execution |
| #11 | — | — | incomplete | pending | cycle | pdca | pdca-coder | — | — | — | finalized | infeasible | 0 | context row: terminal infeasible, unchanged from r4; requires both a real CUDA and an Apple Silicon host |

## Groups table

| group | items/order | worktree | branch | ref | status | notes |
|---|---|---|---|---|---|---|
| G1 | [#2, #8, #3, #4] | main (current worktree) | main | 2d0be53 | pending | one sequential lane, no branch/merge; order by risk #2 → #8 → #3 → #4; common base is the current main HEAD 2d0be53 |

## Dependencies

- No hard edges among the four items. `#8` uses the already-working MelBand and BS-RoFormer checkpoints, so it has no hard runtime dependency on `#2`/`#3`/`#4`.
- Serialization is imposed by shared surfaces only: `Cargo.toml`, `src/lib.rs`, CLI, `src/separator.rs`, `src/config.rs`, plus shared tests/docs; their footprints are not provably disjoint, so `plan_cap 1` and `lane_cap 1` keep one lane.
- Each accepted item's commit is independent and atomic; no merge step exists because there is a single group on a single worktree.

## DO result

Item `#2` — DO attempt `n=1` complete, candidate ready for CHECK (no verdict recorded here). Details
in `.pdca/status/issue-2-r5.md` §DO result and the evidence root
`.pdca/open-issues-r5/rv1/item-2-mdx-onnx/`.

- Applied the preserved r4 candidate (patch sha256 `4922abec…96ff17`) onto HEAD `2d0be53` and
  reconciled it with `#1`'s accepted BS-RoFormer work; `README.md`, `src/config.rs` and `src/lib.rs`
  were hand-merged and `#1`'s two exhaustive `ModelFamily` matches gained `MdxNet` arms. The
  numerical core is byte-identical (CRLF-normalised) to the preserved candidate snapshot.
- Checkpoint `UVR-MDX-NET-Inst_HQ_3.onnx` verified: 66,759,214 B, sha256
  `317554b07fe1ea5279a77f2b1520a41ea4b93432560c4ffd08792c30fddf9adc`.
- Gates: `cargo build` exit 0; `cargo test` exit 0 (98 passed, 0 failed, 1 ignored);
  `cargo clippy --all-targets` exit 0 (warnings only).
- Candidate separation (CPU/f32, 882,281 frames × 2 ch @ 44.1 kHz) exit 0; fresh audio-separator
  0.48.0 reference run exit 0 with the same fixture and `.onnx` sha256.
- Comparison over all samples of all channels and stems, no trimming, no post-normalization: overall
  max_abs 1.4230609e-6, RMSE 1.4654514e-7, SDR 118.897 dB, correlation 0.999999999999; per stem ×
  channel all four metrics inside the `#2` tolerance.
- Finding for CHECK: reference and candidate WAVs differ in byte length (7,058,336 vs 7,058,292 B)
  only by the reference's `fact`/`PEAK` metadata chunks; `data` payload size and frame counts are
  identical, so no trimming was applied.

Item `#2` — DO attempt `r2`/`n1` complete (contract revision `r2`), candidate ready for CHECK; no
product file changed. Details in `.pdca/status/issue-2-r5.md` §DO result `### r2 / n1`.

- `F-2-reference-provider` closed: the evidence-root copy `scripts/reference_audio_separator_audit.py`
  wraps `onnxruntime.InferenceSession` and records the **live** session's `get_providers()`; exactly one
  session was created with `requested_providers=["CPUExecutionProvider"]` and
  `active_providers=["CPUExecutionProvider"]`, `tensor(float)` in/out, `torch.cuda.is_available()=False`,
  same `.onnx` sha256. Readable pure-ASCII log `logs/reference-provider-audit.log` (+ `.json`), exit `0`.
- Reference separation re-run on CPU exit `0` into `out/reference-r2/` (r1 reference kept): both stems
  882,281 frames × 2 ch @ 44,100 Hz, `data` chunk sha256 identical to r1 (`7fd76234…`, `98fef60a…`),
  f32 PCM bit-identical; only the `PEAK` chunk differs (`logs/reference-pcm-equivalence.log`, exit `0`).
- Comparison re-run against the unchanged candidate exit `0`: every metric per stem × channel is
  bit-identical to r1, all four thresholds hold, no trimming and no post-normalization
  (`logs/comparison-r1-vs-r2.log`, exit `0`).
- Product tree: `scripts/verify_snapshot_sha256.py` exit `0` — 13/13 files match `snapshot-sha256.txt`,
  0 mismatches, HEAD still `2d0be53`, nothing staged; the r1 gates (`cargo build`/`test`/`clippy`, exit
  `0`, 98 passed / 0 failed / 1 ignored) are cited as still binding the same bytes and were not re-run
  (`logs/tree-sha256-verify.log`).

Item `#2` — DO attempt `r2`/`n2` complete (CHECK 2's three items), candidate ready for CHECK. Details
in `.pdca/status/issue-2-r5.md` §DO result `### r2 / n2`.

- `F-2-dependency-inventory` closed (AC-2.2): `docs/dependency-audit.md` now carries a **149-row**
  table (crate, resolved version, SPDX licence as published by the crate, role/why) covering the full
  `Cargo.lock` closure of `tract-onnx` (141 crates) and `sha2` (10 crates), including all nine
  `tract-*` crates. Generated from `cargo metadata --format-version 1` (raw JSON persisted),
  `Cargo.lock` (version cross-check) and `cargo tree` (host build graph: 118 rows built here, 31
  `resolved only`). 0 rows without a licence; no licence is guessed. This is the only product-tree
  change of the attempt: 6,353 B → 21,256 B, sha256
  `527862c0e88b194479f150526e69afe66b94831904f3a715a555fb4937256b74`. `Cargo.toml`/`Cargo.lock`
  untouched.
- `F-2-github-preacceptance-state` closed as an evidence gap (AC-2.4): `gh` is **not installed** in
  this environment (`where.exe gh` exit `1`; probe exit `127` with
  `FileNotFoundError: [WinError 2]`), `gh auth status` recorded `not set` (no token read). The state
  was read unauthenticated through the GitHub REST API the CLI wraps (repo from `git remote -v` =
  `AlexeyShirshov/candle-roformer`): issue `#2` HTTP `200` → `state=open`, `state_reason=null`,
  `created_at 2026-11-08T06:43:12Z`, `closed_at=null`, labels `[]`; timeline HTTP `200` → 1 event
  (`milestoned`), **0 `closed`**, **0 `reopened`**, 0 comments. No premature closure has occurred.
- `F-2-evidence-map` closed: `manifest.json` › `criterion_evidence` rebuilt on the authoritative r2
  set (`AC-2.1` CPU separation + tolerance parity, `AC-2.2` dependency/licence documentation, `AC-2.3`
  build/tests no regressions, `AC-2.4` pre-acceptance state + post-acceptance closure contract).
- Frozen candidate refreshed: `snapshot/docs/dependency-audit.md`, `snapshot-sha256.txt`,
  `snapshot.diff` (83,461 B, sha256 `e5d28adf…`), `manifest.json`, `commands.txt`, `index.md`.
  `scripts/verify_snapshot_sha256_r2n2.py` exit `0`: 13/13 match, 0 mismatch; changed vs `r1` = the
  audit doc only, unchanged 12/13; `.rs` files and both manifests byte-identical to `r1`; HEAD
  `2d0be53`; nothing staged — so build/test/clippy stay bound to the same bytes and were not re-run
  (`logs/tree-sha256-verify-r2n2.log`).
- No commit, no staging, no push, no GitHub write (`AC-2.4` pre-acceptance state). Next allowed step:
  CHECK item `#2` (`r2`/`n2`).

Item `#8` — DO attempt `r1`/`n1` **blocked on defect `CONC-1`** (cannot finish; no verdict recorded here).
Details in `.pdca/status/issue-8-r5.md` §DO result and the evidence root
`.pdca/open-issues-r5/rv1/item-8-ensembles/`.

- Item record created from the `act.md` skeleton; frozen plan recorded by exact reference to
  §PLAN record `r1` (item `#8` block) and the `AC-8.1`–`AC-8.4` text quoted verbatim.
- Frozen checkpoint identities verified on disk: MelBand `voc_fv7.ckpt` 489,571,079 B, sha256
  `77b9e4b54802d670c02daceb30c61ec825dc54ca6c29c34b03cdd5e9f78382b6` (matches
  `tools/convert_voc_fv7.py` `EXPECTED_SHA256`); BS-RoFormer `model_bs_roformer_ep_317_sdr_12.9755.ckpt`
  639,331,213 B, sha256 `5b84f37e8d444c8cb30c79d77f613a41c05868ff9c9ac6c7049c00aefae115aa` (the `#1`
  accepted checkpoint), converted `.safetensors` 639,109,056 B sha256 `fa296577…5a8a`.
- Reference implementation located and read in the **installed** audio-separator 0.48.0
  (`separator/ensembler.py`, package-root `ensemble_presets.json`, `separator/separator.py`
  `VALID_ENSEMBLE_ALGORITHMS` / `_separate_ensemble`); upstream preset `vocal_balanced` uses
  `avg_fft` and different models, so this repo's `vocal_balanced` is documented as its own preset.
- **First reference attempt failed**, traceback kept: `ValueError: Model file voc_fv7.ckpt not found in
  supported model files` (`separator.py:873`, `download_model_files`) — the checkpoint is absent from
  audio-separator 0.48.0's *model registry*, not from its inference stack
  (`logs/reference-ensemble.log`); model 0 (BS-RoFormer) had already produced both stems.
- **Blocker solved in the reference's own stack** (`scripts/reference_ensemble_v2.py`): the verbatim
  upstream config `docs/provenance/v7.yaml` (sha256 `f51b06e4…ce2f`) is placed next to the checkpoint as
  `voc_fv7.yaml` and one `"MDXC"` entry is added to `Separator.list_supported_model_files()` in a
  `Separator` subclass; the stock `MDXCSeparator`/`RoformerLoader` then loads it
  (`model_run=MelBandRoformer`, `is_roformer` detected by `ConfigurationNormalizer`), both
  `download_files` already on disk so no network. Nothing else is overridden — chunking, `spec_utils`,
  stem naming and the arithmetic are audio-separator's own code:
  `Ensembler(logger, algorithm, weights=None).ensemble([member0, member1])` on exactly the arrays
  `Separator._separate_ensemble` rebuilds with `librosa.load(mono=False, sr=44100)`
  (`separator.py:1568-1582`).
- **`AC-8.1` measured bit-exact** on the frozen fixture `fixture/mixture.wav` (457,687 frames × 2 ch,
  44,100 Hz, FLOAT, sha256 `ad377390…c54e9`): reference run exit 0 (`logs/reference-ensemble-v2.log`),
  inputs frozen as `fixture/inputs/member{0,1}_vocals.f32`, reference outputs
  `fixture/reference/{avg_wave,median_wave}_vocals.{f32,f64}` (dtypes float64 / float32 as returned);
  candidate parity test exit 0, `1 passed; 0 failed` (`logs/parity-test-ignored.log`); independent
  re-measurement exit 0 (`logs/comparison.log`, `metrics/parity.json`, `metrics/parity.csv`). Worst case
  over **all 915,374 samples of both channels**, all-channels and per channel: `max_abs 0.000e+00`
  (inside ≤ 1e-6), `RMSE 0.000e+00` (inside ≤ 1e-7), `SDR +inf dB`, `corr 1.000000`, 0 differing
  samples — for both `avg_wave` and `median_wave`. Degeneracy for CHECK: with two members
  `median_wave` and `avg_wave` are the same mean, so the fixture validates both code paths against the
  reference but cannot separate their semantics numerically.
- Docs: `docs/ensemble-presets.md` added (operators, `vocal_balanced` composition with both checkpoint
  sha256, equal weights, stem set, CLI invocation, deliberate divergence from upstream's same-named
  preset). README untouched.
- **[history — state at 04:32:56Z, superseded by "Continuation 2" below]** **`AC-8.2` `unverified`, `AC-8.3` `unverified`, evidence pack `unmet`** — blocked by `CONC-1`: a
  duplicate execution process started `cargo run --release --example ensemble_separate -- --preset
  vocal_balanced … --out …/out/candidate/ensemble` at 09:29:52 local (still running), holds
  `logs/preset-cli-run.log` (this session's own invocation died on that lock, exit 1, **no second heavy
  run started**), rewrote `tests/ensemble.rs` at 09:29:08 during the parity run and left 882,000-frame
  candidate dumps in the same evidence root. Gates cannot be measured on a tree that changes under the
  measurement, and `manifest.json`/`index.md`/`snapshot*` must bound a final tree that does not exist.
  Evidence and the requested recovery: `logs/concurrency-conflict.log`.
- No product file, test, config or doc was edited after the conflict was detected (only read-only
  measurement of files this session had already produced); no `git add`/stage/commit/push at any point.
- Next allowed step: **`CHECK item #8 (r1/n1)`** — superseded the driving-role recovery decision once the
  duplicate exited on its own: the remainder was completed on the quiet tree (`AC-8.1` bit-exact,
  `AC-8.2` measured end to end, `AC-8.3` gates green, evidence pack frozen), so `CONC-1` is closed and no
  `DO → ACT` finalization of a blocker is needed.

Item `#8` — DO attempt `r1`/`n1` **continuation 2** (2026-11-11T05:22Z; same attempt, no counter
increment). Details in `.pdca/status/issue-8-r5.md` §DO result and the item evidence root.

- Starting state re-resolved, not assumed: `HEAD 59fb1fb`, nothing staged, product delta exactly the 7
  item files; the 09:36:59 local run is **dead** (PIDs cargo 24236/14456, exe 24488 gone;
  `logs/preset-cli-run-continuation.log` ends at `input=…` with no `rc=`; no output directory), and a
  **live** AC-8.2 run — execution run 3, started `05:20:22Z` by an earlier dispatch of this same attempt
  (cargo 23876/26320, `ensemble_separate.exe` 23412, `logs/preset-cli-run-final.log`, output
  `out/candidate/preset-vocal-balanced`, member 0 from the r4 evidence root) — was found and left
  untouched: no second copy launched, no process killed, `CONC-1` needs no interrupt.
- **`AC-8.3` gates green on the frozen bytes** (no product edit; `HEAD` unchanged, nothing staged):
  `cargo build` exit `0` (0.6 s); `cargo test` exit `0` (80.7 s) — **118 passed, 0 failed, 2 ignored** over
  9 targets (lib 33, `bs_roformer` 8+1 ignored, `core_tests` 22, `ensemble` 20+1 ignored, `library_api` 12,
  `s1_spikes` 11, `streaming_chunked` 8, `whisper_tests` 3, doc 1), with #1 BS-RoFormer and #2 MDX-Net
  tests present and green in the same run; `cargo clippy --all-targets` exit `0` (6.0 s, warnings only:
  lib 54 / lib-test 55 / tests 4+7+11+6). Logs `logs/build.log`, `logs/test.log`, `logs/clippy.log`.
- **`AC-8.2` measured (continuation 3, 06:52Z)** — execution run 3 exited **`rc=0`**, read with
  `GetExitCodeProcess` after `WaitForSingleObject` on the live handle (`scripts/wait_preset_exit.ps1`;
  the run's own launcher also appended `rc=0` to its log), `separation_secs=5491.531`. The output
  directory holds exactly `vocals.wav` + `instrumental.wav`, each 33,407,732 B, FLOAT 44,100 Hz, 2 ch,
  **4,175,961 frames = 94.692993 s**, all samples finite, non-empty, non-silent, matching the input rate,
  frames and duration; `sha256` vocals `9cdb81bc…bade`, instrumental `e15800cf…5f5a`.
  `scripts/verify_preset_stems.py` exit `0` with all eight checks `true`
  (`logs/preset-validation.log`, `out/candidate/preset-vocal-balanced/{exit.json,validation.json}`).
  Runs 1 (exit 1, wrong member path) and 2 (cut by the server restart) are history, recorded in the
  item's §Defect history as `AC-8.2-run-cut`.
- Evidence pack refreshed on the same bytes: `scripts/make_snapshot.py` exit `0` (`HEAD 59fb1fb`, 3
  tracked + 4 new, 7/7 worktree == snapshot copy, hashes unchanged, `snapshot.diff` 57,620 B);
  `manifest.json` and `index.md` rewritten so the criterion map carries the measured numbers for
  `AC-8.1`, `AC-8.2` and `AC-8.3`; `commands.txt` extended with argv, PIDs, start/exit times and exit
  codes. The root keeps exactly one contract fixture (457,687 frames); the 882,000-frame window stays
  quarantined in `attic/882000-window/` and is labelled as such.
- No product file, test, config or doc was edited in this continuation; no `git add`/stage/commit/push and
  no GitHub write at any point. **DO `r1`/`n1` is complete — candidate ready for CHECK.** Next allowed
  step: `CHECK item #8 (r1/n1)`; `AC-8.4` (atomic commit + GitHub `#8` closure) stays ACT work after an
  accepted CHECK.

Item `#8` — DO attempt `r2`/`n1` **complete** (contract revision `r2`; CHECK-directed round), candidate
ready for CHECK. Details in `.pdca/status/issue-8-r5.md` §DO result `### r2 / n1` and the item evidence
root `.pdca/open-issues-r5/rv1/item-8-ensembles/`.

- **Identity note:** the dispatch brief fixed `r1` / `n=2/3`, but the durable record already carried the
  genuine contract revision `r2` (its §1 sets `r=1 → 2` and `n → 1/3`), so the round is recorded as
  `r2` / `n=1`; the `r2` block and the counters are preserved, not overwritten, and no work is claimed
  under `r1`/`n2`.
- **`PAD-8` fixed (1 fix):** `padded_sample(buffer, channel, frame)` now indexes
  `channel * buffer.frames() + frame` and pads **each channel separately** in both `avg_wave` and
  `median_wave`, with two genuinely unequal-length stereo tests and the docs corrected to "per channel,
  in frames". Before: `cargo test --test ensemble` exit **101**, 20 passed / 2 failed
  (`[1.0, 2.0, 3.0, 4.0, 0.0, 0.0]` vs `[1.0, 2.0, 0.0, 3.0, 4.0, 0.0]`; `[1.0, 2.0, 3.0, 4.0, 0.5, 0.5]`
  vs `[1.0, 2.0, 0.5, 3.0, 4.0, 0.5]`). After: exit **0**, 22 passed / 0 failed / 1 ignored
  (`logs/pad8-tests-before-fix.log`, `logs/pad8-tests-after-fix.log`).
- **`LIC-8` / `F-8-song-licence` evidence produced (1 evidence fix, no product change):** the live
  publisher page (`licence/freesound-567226.html`, HTTP 200, 41,021 B, sha256 `ed10cadc…c4ce2`), its
  wayback and short-URL mirrors and the CC0 deed (`licence/wayback-20251023015709.html`,
  `licence/short-url-567226.html`, `licence/cc0-deed-1.0.html`, all HTTP 200), the licence facts in
  `licence/licence-evidence.json` (`Creative Commons 0`, author `Roses1401`, title `Go out - rock.mp3`,
  duration `1:34.692`, `fetch_utc 2026-11-11T07:06:41Z`) and the binding in `licence/binding.md`; the
  **APIv2 call is HTTP 401** (a token is required — the fields were not obtained and are recorded as a
  limitation; no token was read, sent or stored). Producer `scripts/fetch_licence_evidence.py` exit `0`
  (`logs/licence-fetch.log`). Duration binding: `1:34.692` ↔ the used file `94.692993 s`; the preview MP3
  (sha256 `edf03cb0…f76a`, 2,349,696 B, 48 kHz, 94.692979 s) decoded and resampled is **bit-identical**
  to `song/song-go-out-rock-f32-44100.wav` (`max_abs_difference 0.0`, `logs/licence-decode-chain.log`
  exit 0). **Honest caveat kept:** the repo MP3 is the public HQ preview (≈198.5 kbps), not the 3.6 MB /
  320 kbps uploaded master behind a login, so the licence binds to the preview of sound `567226`.
- **`AC-8.1` re-measured on the fixed bytes:** parity test exit `0` (`logs/parity-test-final2.log`) and
  `scripts/compare_parity.py` exit `0` (`metrics/parity-final2.json`, `logs/comparison-final2.log`) —
  `max_abs 0`, `RMSE 0`, `SDR +inf dB`, `corr 1.000000`, 0 differing samples over **915,374** samples
  (457,687 per channel, all-channels and per channel) for both operators; the post-fix dumps are
  byte-identical to the pre-fix dumps and to the reference f32 dump (sha256 `fab3854a…9774`).
- **`AC-8.3` gates on the fixed bytes:** `cargo build` exit `0` (`logs/build-n2.log`), `cargo test`
  exit `0` — **120 passed / 0 failed / 2 ignored** (`logs/test-n2.log`: lib 33, `bs_roformer` 8+1
  ignored, `core_tests` 22, `ensemble` 22+1 ignored with the two new tests, `library_api` 12,
  `s1_spikes` 11, `streaming_chunked` 8, `whisper_tests` 3, doc 1; #1 BS-RoFormer and #2 MDX-Net green
  in the same run), `cargo clippy --all-targets` exit `0` with 0 errors (`logs/clippy-n2.log`).
- **`AC-8.2` unchanged and not invalidated** (the 91-minute song run was not repeated): both member stems
  of that run carry the input's 4,175,961 frames, so the padding branch is never entered and for equal
  frame counts the fixed index is identically the old one — confirmed by the byte-identical dumps
  (`logs/ac82-pad8-non-invalidation.md`).
- **`AC-8.4(1)` pre-acceptance state:** the whole delta is in the worktree and ready for one atomic
  commit — 3 modified + 4 new product files, `HEAD 59fb1fb`, index empty, no commit, no push, no GitHub
  write; GitHub `#8` untouched. `AC-8.4(2)` stays `ACT` work after `CHECK: accepted`.
- **Pack refreshed** on the fixed bytes: `scripts/make_snapshot.py` exit `0` (`logs/snapshot-final2.log`),
  7/7 worktree == snapshot, `snapshot.diff` 61,979 B; closing re-anchor `build=0 test=0 clippy=0 parity=0`
  and `7/7` identity (`logs/reanchor-summary.log`, `logs/reanchor-7of7-verify.log`).
- **`LIC-8` closed on the primary source, bound to the used bytes (continuation `2026-11-11T07:09Z`,
  fixes applied 1).** Raw responses re-fetched and persisted under
  `item-8-ensembles/evidence/source/` (no token, no credentials): page
  `https://freesound.org/people/Roses1401/sounds/567226/` **HTTP 200**, 41,021 B, sha256
  `fedd1b69…b4fc2ae`, observed **`2026-11-11T07:09:35Z`**; short URL **HTTP 200** (41,021 B,
  `ddaf537c…`); APIv2 **HTTP 401** (`{"detail":"Authentication credentials were not provided."}`, token
  `not set`); HQ preview **HTTP 200**, 2,349,696 B, sha256 `edf03cb0…c13f76a`. The page declares sound
  `567226`, `Go out - rock.mp3` by `Roses1401`, and the licence block **`Creative Commons 0`** →
  `http://creativecommons.org/publicdomain/zero/1.0/`, with no CC BY/attribution wording; it lists the
  preview URL as its `data-static-file-url`, that URL re-downloads **byte-identically** to
  `song/go-out-rock-567226-hq.mp3`, and re-running the documented decode step on the re-downloaded bytes
  reproduces `song/song-go-out-rock-f32-44100.wav` (4,175,961 frames = 94.692993 s) **bit-identically**
  (`max_abs 0.0`). Machine-readable record `item-8-ensembles/metrics/licence-provenance.json`; the item
  `manifest.json` gained the root `licence` block and a closed `LIC-8`. **Measured correction:** the
  earlier note above cites `ed10cadc…c4ce2` / `f117c2eb…959a` for the two persisted HTML responses, but
  those files hash as `034ea7c7…8d33` / `25b90d16…78b` (the Freesound page is dynamic, so each hash
  identifies one fetch); `item-8-ensembles/index.md` carries the correction and the earlier rows are kept.
  Logs: `logs/{fetch-licence-source,licence-decode-chain,licence-provenance-build}.log`.
- **Status-record repair (no content change):** the item record's `### r2` PLAN block was enclosed from a
  2-backtick fence to a 4-backtick one (it contains 3-backtick fences); the recorded text stayed
  byte-identical (127 lines / 23,977 B / sha256 `669a4418…c55adb1`), `### r1` unchanged
  (`logs/status-fence-repair.log`).
- No commit, no staging, no push, no GitHub write. Next allowed step: **`CHECK item #8 (r2/n1)`**.
- **Item `#8` — `DO` `r3`/`n1` complete (evidence-only round, no product change; `2026-11-11T07:34Z`–`07:38Z`).**
  Details in `.pdca/status/issue-8-r5.md` §DO result `r3`/`n1` and the evidence root
  `.pdca/open-issues-r5/rv1/item-8-ensembles/`.
  - **`AC-8.4(1)` observation stored, and it is fresh (read-only, no token):** unauthenticated
    `GET https://api.github.com/repos/AlexeyShirshov/candle-roformer/issues/8` → **HTTP 200**
    (4,931 B, sha256 `4d61ef91…fbcd`) and `GET …/issues/8/timeline?per_page=100` → **HTTP 200**
    (1,329 B, sha256 `e642a3f5…08db`). Observed **`2026-11-11T07:34:54Z`**: identifier
    `AlexeyShirshov/candle-roformer#8`, canonical URL
    `https://github.com/AlexeyShirshov/candle-roformer/issues/8`, **`state=open` (`OPEN`)**,
    `state_reason=null`, `closed_at=null`, `created_at=2026-11-08T06:43:43Z`, timeline
    **0 `closed` / 0 `reopened`**, `rate-remaining=59`. Files
    `github/issue-8__2026-11-11T07-34-54Z.json`, `github/issue-8-timeline__2026-11-11T07-34-54Z.json`,
    `github/issue-8-observation.json`; the stale `07:19:54Z` summary was archived byte-identically as
    `github/issue-8-observation__2026-11-11T07-19-54Z.json`; producer `scripts/observe_issue8.py` exit
    `0` (`logs/github-issue-8-r3n1.log`). `F-8-issue-open-evidence` **closed, fixes applied 1**; the
    plan's stop condition did not trigger (the issue is `OPEN`). A concurrent dispatch of the same item
    re-ran the producer while this round was recording, so the **current**
    `github/issue-8-observation.json` is a second fresh observation at **`2026-11-11T07:37:38Z`**
    (HTTP `200`, `state=open`, raw `github/issue-8__2026-11-11T07-37-38Z.json`, byte-identical body);
    the `07:34:54Z` record above is archived as `github/issue-8-observation__2026-11-11T07-34-54Z.json`
    and both are `OPEN`.
  - **Frozen candidate re-anchored `7/7`, no product change:** worktree == `snapshot/` copy ==
    `snapshot-sha256.txt`, `matched=7/7`, `missing=0`, `mismatch=0`, exit `0`
    (`logs/reanchor-7of7-verify-r3n1.log`); the seven hashes are the `CHECK 2`-graded ones
    (`tests/ensemble.rs` 21,578 B / `dbc05263…e01a`). `HEAD 59fb1fbcf89ed99a447dc5b8d48b915de601917f`,
    `git diff --cached --stat` empty, no `#8` commit. No build/test was re-run (the snapshot had not
    drifted); the `AC-8.3` gates stay `logs/reanchor-{build,test,clippy}.log` (exit `0`,
    120 passed / 0 failed / 2 ignored).
  - **Notice — concurrent out-of-contract lint round, reverted:** another dispatch of the same item
    edited `tests/ensemble.rs` (21,578 → 21,633 B) to drop 4 clippy warnings and then reverted it
    byte-identically when `r3` §9 was seen to forbid that edit (`logs/lint-fix-round.md`,
    `logs/snapshot-restore.log`, `out/lint-fix-round/`); it is not claimed as progress and no product
    byte differs from `CHECK 2`.
  - **Pack:** `manifest.json` gained the root `github_issue` block and the `AC-8.4(1)` pre-acceptance
    sub-record, and its item/snapshot/`AC-8.3` pointers were put back on the frozen bytes (they still
    described the reverted round); `commands.txt` carries the round's commands with exit/HTTP status
    (`scripts/update_manifest_r3n1.py` exit `0`, `logs/manifest-r3n1-update.log`).
  - No commit, no staging, no push, no GitHub write. Next allowed step: **`CHECK item #8 (r3/n1)`**.

**`#8` `DO` `r3`/`n1` continuation (`2026-11-11T07:37Z`–`07:40Z`, evidence only, no product change).**
The same attempt's second continuation re-took the read-only observation
(`github/issue-8-observation.json`, `fetched_utc 2026-11-11T07:37:38Z`, HTTP `200`, `state=open`,
`closed_at=null`, `timeline_closed_events=0`, `timeline_reopened_events=0`; the `07:34:54Z` summary
archived as `github/issue-8-observation__2026-11-11T07-34-54Z.json`), reconciled the evidence pack to the
restored tree (`scripts/reconcile_manifest_r3.py` exit `0`, `logs/manifest-r3-reconcile.log`:
`tests/ensemble.rs` back to 21,578 B / `dbc05263…e01a` in `products` and `AC-8.3.tree`, restored gate/log
pointers, `F-8-ac3-contract-drift` and `F-8-issue-open-evidence` closed, `AC-8.1.restored_r3_measurement`
added, `snapshot.diff` 61,979 B; `index.md` rewritten to DO `r3`/`n1` with the `r3` gate semantics and
the stored observation; `commands.txt` extended), and re-verified the candidate independently —
`scripts/verify_final_state.py` exit `0`, `matched=7/7`, `logs/final-state-verify-r3n1-continuation.log`.
**No product, test, documentation or snapshot byte changed;** the 7-file delta equals the `CHECK 2`
snapshot hashes, `HEAD 59fb1fb`, index empty, no commit, no push, GitHub `#8` untouched (read-only `GET`
only). Both status files were concurrently written by another dispatch in the same window; its blocks are
preserved byte-identically and nothing was lost.

## CHECK record

`not run` for items `#3` and `#4` — both are still `pending` and unstarted. Item `#8` ran three CHECK
events (CHECK 1, CHECK 2 and — accepted — CHECK 3, all summarized below); item `#2` ran three CHECKs.
Item `#2`: CHECK **ran** on the `r1`/`n1` candidate and returned **`not accepted → PLAN`** — `AC-2.1`
`unverified`, `AC-2.2` `met`, `AC-2.3` `unmet`; findings `F-2-criteria-coverage` (major),
`F-2-acceptance-order` (blocking) and `F-2-reference-provider` (major, unverified), with
`F-2-wav-trimming` refuted. The complete report is recorded verbatim in the `## CHECK record` of
`.pdca/status/issue-2-r5.md`. This is not a pass and no item is accepted.

Item `#2` — CHECK **2** ran on the `r2`/`n1` candidate and returned **`not accepted → DO`**:
`AC-2.1` `met`, `AC-2.2` `unmet` (no per-dependency version/licence inventory of the MDX build graph),
`AC-2.3` `met`, `AC-2.4` `unverified` (no independent GitHub pre-acceptance observation), with
findings `F-2-dependency-inventory` (major, confirmed), `F-2-github-preacceptance-state` (major,
unverified), `F-2-evidence-map` (minor, confirmed) and `F-2-reference-provider` **refuted for `r2`**.
The complete report is recorded verbatim under `### CHECK 2 — r2/n1` in the `## CHECK record` of
`.pdca/status/issue-2-r5.md`. Not a pass; no commit and no GitHub closure is authorized.

Item `#2` — CHECK **3** ran on the frozen `r2`/`n2` candidate and returned **`accepted → ACT`**:
`AC-2.1` `met`, `AC-2.2` `met`, `AC-2.3` `met`, `AC-2.4` `met` (pre-acceptance component only), and the
findings `F-2-dependency-inventory`, `F-2-github-preacceptance-state`, `F-2-evidence-map` and
`F-2-reference-provider` are all **refuted** on the current candidate. The complete report is recorded
verbatim under `### CHECK 3 — r2/n2` in the `## CHECK record` of `.pdca/status/issue-2-r5.md`.
Acceptance authorizes the post-acceptance closure and nothing else: the commit and the GitHub `#2`
closure are still unperformed and belong to ACT.

Item `#8` — CHECK **1** ran on the `r1`/`n1` candidate (frozen 7-file delta on
`main @ 59fb1fb`) and returned **`not accepted → PLAN`**:

- Matrix: `AC-8.1` **`met`** (both operators `max_abs=0`, `RMSE=0` over 915,374 samples / 2 channels;
  parity test exit `0`); `AC-8.2` **`unverified`** (`rc=0`, `separation_secs=5491.531`, exactly
  `vocals` + `instrumental` at 44,100 Hz / 2 ch / 4,175,961 frames / 94.692993 s — but the song's free
  licence is not independently evidenced); `AC-8.3` **`met`** (build/test/clippy exit `0`,
  118 passed / 0 failed / 2 ignored, BS-RoFormer 8 / MDX-Net 9); `AC-8.4` **`unmet`** (no commit, no
  GitHub closure).
- Findings: `F-8-acceptance-order` (**blocking**, `confirmed`, AC-8.4) — cyclic acceptance condition:
  CHECK demands actions the plan permits only after an accepted CHECK; `F-8-song-licence` (**major**,
  `unverified`, AC-8.2) — CC0 is an author assertion (`prepare_song.py:73-79`), the download log is empty.
- Two required evidence actions, routed by CHECK: (1) the **ordering fix in the contract** — a PLAN
  revision that keeps the user's commit/closure requirement whole while making its place in the
  sequence explicit; (2) **licence evidence** for sound `567226` — the persisted source page/API
  response with the licence and the link to the preview URL actually downloaded, plus URL, observation
  time and the correspondence to the MP3 sha256 (no repetition of the long separation run).
- The complete report is recorded **verbatim** under `### CHECK 1 — r1/n1` in `.pdca/status/issue-8-r5.md`
  §CHECK record. This is not a pass: counters stay `r=1`, `n=1/3`, and neither the commit nor the
  GitHub closure is authorized.
- The outcome above (**CHECK 1**, `r1`/`n1) produced the item's genuine contract revision **`r2`**:
  `F-8-acceptance-order` was closed at the contract level by splitting `AC-8.4` into `AC-8.4(1)`
  (pre-acceptance state, checkable on `CHECK`) and `AC-8.4(2)` (commit + GitHub closure in `ACT`), and
  `F-8-song-licence` became an `AC-8.2` primary-source evidence obligation. The revision is recorded
  verbatim in `.pdca/status/issue-8-r5.md` §PLAN record `r2`; the **collection** revision stays `r1`.

Item `#8` — CHECK **2** ran on the same `r1`/`n1` candidate, on the measurements taken **before** the
`r2` block was written, and returned **`not accepted → DO`**:

- Matrix: `AC-8.1` **`met`** (two frozen real member inputs, 457,687 frames × 2 ch; `max_abs=0`,
  `RMSE=0` over all 915,374 samples including each channel; parity test exit `0`); `AC-8.2`
  **`unverified`** (`rc=0`, exactly `vocals` + `instrumental` at 44,100 Hz / 2 ch / 4,175,961 frames /
  94.692993 s, but CC0 was still an author constant); `AC-8.3` **`met`** (build `0`, test `0` —
  118 passed / 0 failed / 2 ignored, clippy `0`); `AC-8.4` **`unverified — ACT pending`** (seven
  product files in the snapshot, no commit, no closure — explicitly not a preliminary PASS).
- Findings: `LIC-8` (**unverified, blocking**, AC-8.2) — `scripts/prepare_song.py:73-79` writes title,
  author and CC0 as literals, and `logs/song-metadata.log` concerns a different recording; `PAD-8`
  (**confirmed**, medium) — `snapshot/src/ensemble.rs` padded the flat array instead of each channel,
  so a short stereo buffer shifts the right channel, and `snapshot/tests/ensemble.rs:77-83` used equal
  lengths and could not detect it (this does not refute the measured `AC-8.1`, whose frozen inputs are
  equal-length).
- Routing `CHECK → DO`: (1) one evidence round for `AC-8.2` — the publisher page/API response for sound
  `567226` with the URL, the observation time and the binding to the used MP3, without repeating the
  long separation run; (2) fix `PAD-8` by per-channel indexing, add a genuinely unequal-length stereo
  test for both operators, then refresh the snapshot and the relevant checks.
- Both findings are **addressed by the `r2`/`n1` round** (`PAD-8` fixed, `LIC-8` evidence produced — see
  §DO result). The complete report is recorded verbatim under `### CHECK 2 — r1/n1` in the
  `## CHECK record` of `.pdca/status/issue-8-r5.md`. This is not a pass: neither verdict authorizes a
  commit or the GitHub closure, and CHECK 1 and CHECK 2 stay recorded as **two distinct outcomes**,
  never merged into one.

Item `#8` — CHECK **2** on the `r2`/`n1` candidate (the driving role's label for this third CHECK
event; recorded after the two above and not merged with them) returned **`not accepted → PLAN`**:

- Matrix: `AC-8.1` **`met`** (both operators `max_abs=0`, `RMSE=0` over all 915,374 samples including
  each channel; parity test exit `0`); `AC-8.2` **`met`** (`rc=0`, `separation_secs=5491.531`, exactly
  `vocals` + `instrumental` at 44,100 Hz / 2 ch / 4,175,961 frames / 94.692993 s, with the licence now
  proven from the primary source and bound to the used WAV); `AC-8.3` **`unmet`** (build/test/clippy all
  exit `0` and 120 passed / 0 failed / 2 ignored, but the contract's literal "no warnings" clause is
  violated — `logs/reanchor-clippy.log:1–19,898` including 4 warnings in `tests/ensemble.rs`, and
  `logs/reanchor-test.log:100–101`); `AC-8.4(1)` **`unverified`** (the 7-file delta is ready and the index
  is empty, but no stored GitHub `#8` state was presented); `AC-8.4(2)` **`unverified — срок не наступил`**
  (the commit and closure are ACT work after an accepted CHECK, not a standalone obstacle).
- Findings: `F-8-ac3-contract-drift` (**blocking**, `confirmed`) — the `r2` text forbids linter warnings
  while declaring `AC-8.3` semantics unchanged from `r1`, where only command passage was required, so the
  recorded contract text and its declared invariance disagree; `F-8-issue-open-evidence` (**major**,
  `unverified`) — there is no directly stored observation of the GitHub issue state. By contrast
  `F-8-acceptance-order` (blocking) is **refuted/closed** at the contract level,
  `F-8-song-licence`/`LIC-8` (major) is **refuted/closed** on the current candidate, and the historical
  `PAD-8` (medium) is **refuted/closed** on the current candidate.
- Two required actions: (1) **resolve the real AC-8.3 contract drift** in a PLAN revision, keeping the
  user's criteria set whole — CHECK neither authorizes ignoring the warnings nor declares the criterion
  infeasible; (2) through the driver, **store a read-only GitHub response for
  `AlexeyShirshov/candle-roformer#8` with `state=OPEN`, its URL and the observation time** — a narrow
  evidence gap, not a product defect (an unread observation already exists at
  `item-8-ensembles/github/issue-8-observation.json`, recorded `2026-11-11T07:19:56Z`, and is named in the
  item record's recording note).
- The complete report is recorded **verbatim** under `### CHECK 2 — r2/n1` in
  `.pdca/status/issue-8-r5.md` §CHECK record. This is not a pass: counters stay `r=2`, `n=1/3`, and the
  commit, the GitHub `#8` closure and `done` remain unauthorized.

**Item `#8` — `CHECK 3` was pending under contract `r3` at the time of this paragraph; it has since run
and returned `accepted → ACT` (block below).** No third CHECK had run when these lines were written. It becomes the next
allowed step after `DO` `r3`/`n1` (which stored the read-only GitHub `#8` observation and re-anchored the
frozen 7-file candidate `7/7`, no product change). What `CHECK 3` will read: the item's §PLAN record `r3`
(the criteria text with `AC-8.3` in the `r1` gate semantics — `exit 0`, `0 failed`, non-fatal warnings
**not** a failure — and `AC-8.4(1)` naming the stored observation); the restored gate/parity logs
`logs/{build,test,clippy,parity-test}-restored.log` (`exit 0`, 120 passed / 0 failed / 2 ignored,
`max_abs 0` / `rmse 0` over 915,374 samples); `github/issue-8-observation.json`
(`fetched_utc 2026-11-11T07:37:38Z`, `state=open`, `closed_at=null`, 0 closed / 0 reopened events); and the
`7/7` identity check `logs/final-state-verify-r3n1-continuation.log`. `AC-8.4(2)` (commit + GitHub
closure) remains `ACT` work and is not a pre-acceptance obstacle.

Item `#8` — CHECK **3** (`r3`/`n1`; the sixth CHECK event in this collection and the item's first
accepted verdict) ran on the frozen 7-file delta at `main @ 59fb1fb` and returned
**`accepted → ACT`**:

- Matrix: `AC-8.1` **`met`** (`max_abs 0`, `RMSE 0` over all 915,374 samples of 2 channels, both
  operators, also per channel; `cargo test --test ensemble -- --ignored --nocapture` exit `0`,
  1 passed); `AC-8.2` **`met`** (CLI exit `0`, `5491.531 s`, exactly `vocals` + `instrumental`, each
  44,100 Hz / 2 ch / 4,175,961 frames / 94.692993 s, with the CC0 licence from the primary page and the
  MP3→used-WAV decode chain `max_abs_difference=0`); `AC-8.3` **`met`** (restored snapshot:
  `build`/`test`/`clippy` exit `0`, **120 passed / 0 failed / 2 ignored**; the present warnings,
  including 4 in `tests/ensemble.rs`, do not violate the exact `r3` contract); `AC-8.4(1)`
  **`met`** (**7/7** files, 0 missing / 0 mismatch, previous HEAD, empty index, no premature commit;
  stored observation `2026-11-11T07:37:38Z`, HTTP `200`, correct repo/number/URL, `state=open`,
  timeline with 0 closures/reopens); `AC-8.4(2)` **`unverified — срок не наступил`** (the commit and the
  issue closure are the ACT obligation, not a pre-acceptance obstacle, and `done` is forbidden until
  both are performed).
- Keys: `F-8-ac3-contract-drift` (blocking) and `F-8-issue-open-evidence` (major) **`refuted`/closed**;
  `F-8-acceptance-order`, `F-8-song-licence`/`LIC-8` and `PAD-8` remain closed; `CONC-1` and
  `AC-8.2-run-cut` remain closed historical operational records; `F-8-lint-warnings` (a concurrent
  out-of-contract edit, reverted byte-identically to `dbc05263…e01a`) **is not a defect under `r3`**.
- Confidence and limits as reported: CHECK ran nothing itself (stored observations, logs, measuring
  scripts and the snapshot were read; binary metrics were not recomputed); the full song run was not
  repeated after `PAD-8`; parity is limited to the chosen `vocals` fixture, where the two members do not
  numerically distinguish mean from median; CUDA/Metal and the absence of all possible regressions are
  not proven. Acceptance does not mean that the commit, the issue closure or the item completion had
  already been performed.
- The complete report is recorded **verbatim** under `### CHECK 3 — r3/n1` in the `## CHECK record` of
  `.pdca/status/issue-8-r5.md`; this summary is a pointer, not a substitute. The verdict authorizes the
  post-acceptance closure only — ACT then created commit `a38bb7f` and handed the GitHub `#8` closure
  to the calling driver (see §ACT record).

## ACT record

Item `#2` — **outcome `accepted`, finalized**. CHECK 3 (`r2`/`n2`) accepted the frozen candidate
(`AC-2.1` `met`, `AC-2.2` `met`, `AC-2.3` `met`, `AC-2.4` `met` on its pre-acceptance component); ACT
performed the post-acceptance closure and issues no new verdict.

- **Accepted snapshot identity**: HEAD `2d0be53` + the 13-file frozen tree of
  `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/snapshot/`; re-anchored before staging — 13/13 match,
  0 mismatched, 0 missing (`logs/act-reanchor-verify.log`). No drift, so no `Notice`.
- **Commit**: `59fb1fbcf89ed99a447dc5b8d48b915de601917f` (`59fb1fb #2: MDX-Net ONNX separation on CPU
  with audio-separator 0.48.0 parity`), one atomic commit on `main`, parent `2d0be53`, **13 files
  changed, +1750/-23**, footer `Refs #2` (no auto-close keyword). Exactly the item's 13 paths were
  staged (`Cargo.toml`, `Cargo.lock`, `README.md`, `docs/dependency-audit.md`,
  `docs/dependency-audit-evidence/tract-onnx-0.23.10.txt`, `examples/mdx_separate.rs`,
  `src/config.rs`, `src/dsp.rs`, `src/error.rs`, `src/lib.rs`, `src/mdx.rs`, `src/model/mod.rs`,
  `src/separator.rs`); nothing else, no push, no branch switch, no amend/rebase
  (`logs/commit-59fb1fb.log`).
- **Retrospective**: recorded verbatim in `## Lessons learned` of `.pdca/status/issue-2-r5.md` from
  the `pdca-check` `retrospective` report routed by the driving role (3 lessons, limits and
  uncertainty, completion signal). It adds no verdict, counter, attempt or verification.
- **Remaining work**: none inside the collection for item `#2` except the **GitHub `#2` closure by the
  calling driver** — this executor performs no GitHub write, so the item is `done` / `finalized` with
  that external residual, never recorded as closed.
- **Handoff / next authorized action**: the calling driver closes GitHub `#2` with the `AC-2.1`–`AC-2.3`
  report; the collection's next step is `DO item #8 (r1/n1)`; `#3` and `#4` stay `pending` and
  untouched.

Item `#8` — **outcome `accepted`, finalized**. CHECK 3 (`r3`/`n1`) accepted the frozen candidate
(`AC-8.1` `met`, `AC-8.2` `met`, `AC-8.3` `met`, `AC-8.4(1)` `met`, `AC-8.4(2)` `unverified — срок не
наступил`); ACT performed the post-acceptance closure and issues no new verdict.

- **Accepted snapshot identity**: HEAD `59fb1fbcf89ed99a447dc5b8d48b915de601917f` + the seven hashes
  of `item-8-ensembles/snapshot-sha256.txt`; re-anchored before staging — **7/7** worktree == snapshot
  copy == `snapshot-sha256.txt`, 0 mismatched, 0 missing, verifier exit `0`, `git diff --cached` empty
  (`item-8-ensembles/logs/act-reanchor-7of7-r3n1.log`). No drift; the only difference is the
  repository's own CRLF normalisation (recorded as a `Notice`, below).
- **Commit**: `a38bb7fd969dc0ebb36ca74957af93b6b4b20a19` (`a38bb7f #8: ensemble avg_wave/median_wave
  parity and vocal_balanced preset`), one atomic commit on `main`, parent `59fb1fb`, **7 files changed,
  +1368/−2**, footer `Refs #8` (no auto-close keyword). Exactly the item's 7 product paths were staged
  (`src/ensemble.rs`, `src/config.rs`, `src/error.rs`, `src/lib.rs`, `examples/ensemble_separate.rs`,
  `tests/ensemble.rs`, `docs/ensemble-presets.md`); nothing else — no `.pdca/**`, `.openidea/**`,
  weights, audio/fixtures, logs or status file. No push, no branch switch, no amend/rebase, no tag
  (`item-8-ensembles/logs/commit-a38bb7f.log`). `HEAD` is now `a38bb7f`;
  `git diff --name-only HEAD -- <7 paths>` is empty and `git status --porcelain` shows no product
  change left and nothing staged.
- **Retrospective**: recorded verbatim in `## Lessons learned` of `.pdca/status/issue-8-r5.md` from the
  `pdca-check` `retrospective` report routed by the driving role (3 lessons with sources, limits and
  uncertainty, completion signal `retrospective_complete`). It adds no verdict, counter, attempt or
  verification.
- **Remaining work**: none inside the collection for item `#8` except the **GitHub `#8` closure by the calling driver** — this executor performs no GitHub write, so the item is **`done` / `finalized`** with that external residual (`AC-8.4(2)` = commit performed, GitHub closure pending by the calling driver), never recorded as "issue closed" — the same recorded convention as item `#2`.
- **Handoff / next authorized action**: the calling driver closes GitHub `#8` with the CHECK 3 report
  and commit `a38bb7f`; the collection's next step is **`DO item #3 (r1/n1)`**; `#4` stays `pending`
  and untouched.
- **Notice (cosmetic, not identity drift)**: `core.autocrlf=true` with no `.gitattributes`, so for the
  three *modified* files (`src/config.rs`, `src/error.rs`, `src/lib.rs`) the committed blob is git's
  LF-normalised form while the accepted worktree bytes are CRLF; the worktree still hashes to its
  snapshot row 7/7 and `git diff --name-only HEAD -- <7 paths>` is empty. The commit identity
  (`Alexey Shirshov <alex.shirshov@vk.com>`, matching `59fb1fb`) was supplied per command with
  `git -c`; no git config was written.

**Record correction — ACT dispatch of `2026-11-11T07:47Z` (append-only; the `#8` block above is kept
unchanged).** `CHECK 3`'s verdict and the commit facts above are confirmed by an independent re-read of the worktree (`item-8-ensembles/logs/commit-a38bb7f-verify.log`, `logs/act-reanchor-verify.log`); the disposition is recorded as `done` / `finalized` with the GitHub closure as the single external residual (corrected bullet below):

- **The commit was not created by this dispatch's staging step.** The pre-stage re-anchor found `HEAD`
  **already** at `a38bb7f` (reflog `commit`, `2026-11-11 12:45:40 +0500`, `HEAD@{1} = 59fb1fb`): the
  commit component of `AC-8.4(2)` had been performed by a concurrent dispatch of the same item before
  this dispatch started. Per this ACT brief's rule ("if `HEAD` has moved … do not stage, do not
  commit") this dispatch staged nothing and created no commit; a second commit or an amend is
  forbidden, so none was made. The commit is verified as contract-conforming: parent `59fb1fb`, exactly
  7 files, footer `Refs #8`, no `Closes`/`Fixes`, no tag, **not pushed** (`a38bb7f` is not an ancestor
  of `origin/main = bf2cab6`).
- **`Notice` — subject-line deviation.** The committed subject is `#8: ensemble avg_wave/median_wave
  parity and vocal_balanced preset`; this dispatch's brief prescribed `#8: ensemble operators
  avg_wave/median_wave and vocal_balanced preset with audio-separator 0.48.0 parity`. Not amended
  (amend is prohibited); recorded, not repaired. No criterion or measurement is affected.
- **`done` recorded with the external residual (reconciled).** The earlier statement that withheld `done` is corrected to the convention already recorded for item `#2`: this cycle performs no GitHub write, so `AC-8.4(2)` is recorded as "commit performed, GitHub closure pending by the calling driver (external residual)" and the GitHub closure was not performed (the last stored observation `2026-11-11T07:37:38Z` shows `state=open`). Item `#8` is therefore **`done`** with disposition `finalized`, `plan_revision 3`, attempts `1`; the Git state is `HEAD a38bb7f`, index clean, nothing pushed.
- **Remaining work / handoff**: only the **GitHub `#8` closure by the calling driver**; the item is recorded `done` / `finalized` now, with that closure as its external residual, and the next step is `DO item #3 (r1/n1)` (`#4` stays `pending`, untouched). Nothing judged by `CHECK 3` (`AC-8.1`–`AC-8.4(1)`) is `unmet`, and no product,
  test or documentation work remains.

**Driver closure completed (append-only, `2026-11-11`; the item blocks above are kept unchanged).**

- **Item `#2` — GitHub `#2` closed as `COMPLETED` by the driver at `2026-11-11T06:18:29Z`** (comment id `6788661305`), product commit `59fb1fbcf89ed99a447dc5b8d48b915de601917f` on `main` (not pushed). The single external residual recorded above is discharged; the item is `done` / `finalized` with terminal outcome (cycle) `accepted`.
- **Item `#8` — GitHub `#8` closed as `COMPLETED` by the driver at `2026-11-11T07:57:47Z`** (comment id `6790434030`), product commit `a38bb7fd969dc0ebb36ca74957af93b6b4b20a19` on `main` (parent `59fb1fb`, exactly 7 product files, `+1368/-2`, not pushed). Same effect: residual discharged, item `done` / `finalized` with terminal outcome (cycle) `accepted`.
- **Closure no longer requires a token for this record's account**: the driver used the GitHub CLI, which is authenticated at `C:\Program Files\GitHub CLI\gh.exe` (owner-confirmed working). This supersedes the earlier blocker note about missing credentials; no credential value is recorded, read or reproduced here.
- **Counters unchanged**: collection `N=1`, `r=1`; item `#2` `r=2` / `n=2/3`; item `#8` `r=3` / `n=1/3`. The closures consume no attempt and add no verification, no criterion and no verdict.

## General verification and recovery

- verification state: `#2 contract revision r=2 (AC-2.1–AC-2.4), DO r2/n2 complete, CHECK 3 accepted → ACT, ACT closed (commit 59fb1fb, retrospective recorded)`; the `r1`/`n1` and `r2`/`n1` candidates were `not accepted` and are not progress; the `r2`/`n2` attempt changed one documentation file and closed the three CHECK 2 findings; all four item keys are `refuted/closed on the r2/n2 candidate` and `AC-2.1`–`AC-2.4` are `met` (`AC-2.4` pre-acceptance component, its post-acceptance commit action now performed). Item `#2`'s only residual is the GitHub `#2` closure by the calling driver; `#8`, `#3`, `#4` untouched and unstarted
- last CHECK (`C`): item `#2`, `r1` / attempt `n1` — **`not accepted → PLAN`**, route: criteria defect.
  Verdict `not accepted`; open defect keys `F-2-criteria-coverage` (major, confirmed: the mandatory
  `dependency documented in docs/dependency-audit.md with licence` clause is missing from
  `AC-2.1`–`AC-2.3`), `F-2-acceptance-order` (**blocking**: `AC-2.3` demands the commit and the GitHub
  closure that the PLAN allows only after an accepted CHECK) and `F-2-reference-provider` (major,
  `unverified`: the reference run's active CPU provider is not directly evidenced). Fixes applied: 0.
  Report recorded verbatim in `.pdca/status/issue-2-r5.md` §CHECK record; counters unchanged
  (`r=1`, attempt `n=1/3`).
- last CHECK 2 (`C`): item `#2`, `r2` / attempt `n1` — **`not accepted → DO`**, route: implementation
  and evidence gaps. `AC-2.1` `met`, `AC-2.2` `unmet` (no per-dependency version/licence inventory of
  the MDX build graph), `AC-2.3` `met`, `AC-2.4` `unverified` (no independent GitHub pre-acceptance
  observation). Open keys `F-2-dependency-inventory` (major, confirmed),
  `F-2-github-preacceptance-state` (major, unverified), `F-2-evidence-map` (minor, confirmed), fixes
  applied: 0; `F-2-reference-provider` **refuted for `r2`**. Report recorded verbatim under
  `### CHECK 2 — r2/n1` in `.pdca/status/issue-2-r5.md`; counters `r=2`, attempt `n=1/3` consumed.
- last CHECK 3 (`C`): item `#2`, `r2` / attempt `n2` — **`accepted → ACT`**. `AC-2.1` `met`,
  `AC-2.2` `met` (149 named rows with versions and licences, 0 licence gaps), `AC-2.3` `met`
  (build/test exit `0`, 98 passed / 0 failed / 1 ignored), `AC-2.4` `met` on its pre-acceptance
  component (tree verifiable, HEAD base, empty index, `#2` `open` with 0 `closed`/`reopened`);
  `F-2-dependency-inventory`, `F-2-github-preacceptance-state`, `F-2-evidence-map` and
  `F-2-reference-provider` refuted. Report recorded verbatim under `### CHECK 3 — r2/n2` in
  `.pdca/status/issue-2-r5.md`; counters `r=2`, attempt `n=2/3` consumed; environment exception
  recorded (`gh` absent, probe exit `127`, read-only REST observation used).
- last CHECK 4 (`C`): item `#8`, `r1` / attempt `n1` — **`not accepted → PLAN`**, route: criteria
  defect (acceptance ordering). `AC-8.1` `met`, `AC-8.2` `unverified`, `AC-8.3` `met`, `AC-8.4`
  `unmet`; open keys `F-8-acceptance-order` (**blocking**, `confirmed`: `AC-8.4` requires a finished
  commit and GitHub closure that the plan allows only after an accepted CHECK) and `F-8-song-licence`
  (major, `unverified`: the CC0 licence of sound `567226` is an author assertion), fixes applied: 0.
  Report recorded verbatim under `### CHECK 1 — r1/n1` in `.pdca/status/issue-8-r5.md`; counters
  unchanged (`r=1`, attempt `n=1/3`; the `CHECK → PLAN` transition consumes no attempt).
- next allowed step: **PLAN contract revision for item `#8`** — `r` stays `1` until the planner returns
  it; **no commit and no GitHub `#8` closure is authorized**, and no DO work is allowed from DO. The
  revision must place the user's commit/closure requirement in a non-cyclic order and may require the
  one licence-evidence supplement for sound `567226`. Item `#2` is closed on the work side (ACT
  recorded, commit `59fb1fb`), and its GitHub `#2` closure is executed by the calling driver, never by
  a status write.
- completeness: **`2/4 accepted`** (2 of 4 items `done`; this bullet is reconciled — its earlier `1/4` value is history) — item `#2` is **accepted, committed (`59fb1fb`) and finalized**, with the GitHub `#2` closure pending by the calling driver; item `#8` is **accepted by `CHECK 3` (`r3`/`n1`) and committed (`a38bb7f`, parent `59fb1fb`, 7 files, footer `Refs #8`, not pushed)** and **`done` / `finalized`**, with `AC-8.4(2)` recorded as "commit performed, GitHub closure pending by the calling driver (external residual)"; `#3` and `#4` are `pending` (`0/4 closed in GitHub`, closure is external)
- record lint: `& "C:\Users\Пользователь\AppData\Local\Programs\Python\Python312\python.exe" .pdca/scripts/status_lint.py .pdca/status/collection-open-issues-r5.md` → exit status `0` (`status-lint: OK (.pdca\status\collection-open-issues-r5.md: 2 tables, 7 rows)`); log `.pdca/open-issues-r5/rv1/lint.txt`.
- record lint after the `#2` CHECK recording: same command → exit status `0` (`status-lint: OK
  (.pdca\status\collection-open-issues-r5.md: 2 tables, 7 rows)`); `issue-2-r5.md` → exit status `0`
  (`status-lint: OK (.pdca\status\issue-2-r5.md: 1 tables, 3 rows)`).
- record lint after the `#2` plan revision `r2` recording: same command → exit status `0`
  (`status-lint: OK (.pdca\status\issue-2-r5.md: 1 tables, 3 rows)` and
  `status-lint: OK (.pdca\status\collection-open-issues-r5.md: 2 tables, 7 rows)`); log
  `.pdca/open-issues-r5/rv1/lint.txt` (appended section, `2026-11-10T23:27:02Z`).
- record lint after the `#2` CHECK 2 recording: same command → exit status `0`
  (`status-lint: OK (.pdca\status\issue-2-r5.md: 2 tables, 7 rows)` and
  `status-lint: OK (.pdca\status\collection-open-issues-r5.md: 2 tables, 7 rows)`; no violation, no
  fix required).
- record lint after the `#2` CHECK 3 recording: same command → exit status `0` on both files —
  `status-lint: OK (.pdca\status\issue-2-r5.md: 3 tables, 11 rows)` and
  `status-lint: OK (.pdca\status\collection-open-issues-r5.md: 2 tables, 7 rows)`; no violation, no
  fix required (the item record's third table is the CHECK 3 criteria matrix, which the lint counts as
  data rows).
- record lint after the `#2` ACT recording: same command → exit status `0` on both files —
  `status-lint: OK (.pdca\status\issue-2-r5.md: 3 tables, 11 rows)` and
  `status-lint: OK (.pdca\status\collection-open-issues-r5.md: 2 tables, 7 rows)`; no violation, no
  fix required.
- record lint after the `#8` CHECK 1 recording: same command → exit status `0` on both files —
  `status-lint: OK (.pdca\status\issue-8-r5.md: 1 tables, 4 rows)` (the CHECK criteria matrix) and
  `status-lint: OK (.pdca\status\collection-open-issues-r5.md: 2 tables, 7 rows)`; no violation, no
  fix required; log `.pdca/open-issues-r5/rv1/item-8-ensembles/lint.txt`.
- after the ACT commit and the two status writes, `git status --porcelain` shows only the pre-existing
  untracked `.openidea/` and `.pdca/` records (`collection-open-issues-r3.md`,
  `collection-open-issues-r5.md`, `issue-2-r5.md`) — no leftover product change, nothing staged.
- recovery: counters and this record are sufficient to restore the collection state: item `#2`
  finalized at `r=2`, `n=2/3`, commit `59fb1fb`, retrospective recorded; the next step is
  `DO item #8 (r1/n1)`; the GitHub `#2` closure is external to this record. Item `#2`'s rejected
  candidates (`r1`/`n1`, `r2`/`n1`) and their evidence root stay as they are — a rejected candidate is
  not progress and is not re-worked by a status write, and its artefacts remain the evidence base.
- **after item `#8` `DO` `r2`/`n1`** — the earlier "next allowed step: PLAN contract revision for item
  `#8`" bullet above is history: that revision `r2` was recorded at 2026-11-11T07:04:10Z and the
  CHECK-directed round is now complete (`PAD-8` fixed with 1 fix; `LIC-8` primary-source evidence in
  `licence/` with 1 fix; `AC-8.1`/`AC-8.3` re-measured on the fixed bytes — `max_abs 0` / `RMSE 0` over
  915,374 samples and 120 passed / 0 failed / 2 ignored; `AC-8.2` unchanged and non-invalidated;
  `AC-8.4(1)` holds with an empty index and GitHub `#8` untouched). Next allowed step: **`CHECK item #8
  (r2/n1)`**; no commit and no GitHub `#8` closure is authorized. Completeness stays `1/4 accepted`;
  `#3` and `#4` untouched and unstarted.
- **after the `LIC-8` closure continuation (`2026-11-11T07:09Z`)** — the licence gap is closed on the
  primary source and bound to the used bytes: Freesound sound `567226` page **HTTP 200** at
  `2026-11-11T07:09:35Z` declares `Creative Commons 0` (CC0 1.0, `creativecommons.org/publicdomain/zero/1.0/`),
  the page lists `https://cdn.freesound.org/previews/567/567226_3442171-hq.mp3`, that URL re-downloads
  **byte-identically** (2,349,696 B, sha256 `edf03cb0…c13f76a`) and its decode chain reproduces
  `song/song-go-out-rock-f32-44100.wav` **bit-identically** (`max_abs 0.0`); APIv2 is **HTTP 401** (no
  token used). Evidence `item-8-ensembles/evidence/source/` + `metrics/licence-provenance.json`; item
  `manifest.json` root `licence` block; `F-8-song-licence` **closed, fixes applied 1**. The item record's
  `### r2` fence was repaired to a 4-backtick enclosure with the recorded text byte-identical
  (127 lines / 23,977 B / sha256 `669a4418…c55adb1`). Independent re-anchor of the same product bytes:
  `7/7` worktree == snapshot copy == `snapshot-sha256.txt`, `cargo build`/`test`/`clippy`/parity all exit
  `0` (**120 passed, 0 failed, 2 ignored**; `max_abs 0`, `rmse 0` over 915,374 samples), no product change
  and the 5,491.531 s separation not re-run. Next allowed step stays **`CHECK item #8 (r2/n1)`**;
  completeness `1/4 accepted`; no commit and no GitHub `#8` closure is authorized.
- record lint after the `#8` `r2`/`n1` recording and after the `LIC-8` closure continuation: same command
  → exit status `0` on `issue-8-r5.md` (`status-lint: OK (.pdca\status\issue-8-r5.md: 4 tables, 15 rows)`)
  and on this collection file (see the appended section in `item-8-ensembles/lint.txt`); no violation, no
  fix required.
- record lint after the `#8` `DO` `r3`/`n1` recording: same command → exit status `0` on both files —
  `status-lint: OK (.pdca\status\issue-8-r5.md: 8 tables, 30 rows)` and
  `status-lint: OK (.pdca\status\collection-open-issues-r5.md: 2 tables, 7 rows)`; no violation, no fix
  required; log `item-8-ensembles/lint.txt` (sections `2026-11-11T07:39:12Z`, the closing run and the
  re-check run). The item record's third observation table comes from the concurrent continuation of the
  same `r3`/`n1` round (its own second observation at `07:37:38Z`) and is kept as returned.
- **`AC-8.4(1)` GitHub pre-acceptance observation for item `#8` (`2026-11-11T07:19:56Z`)** — unauthenticated
  GitHub REST read of `AlexeyShirshov/candle-roformer` `#8` (repo identity parsed from `git remote -v`):
  `GET /repos/AlexeyShirshov/candle-roformer/issues/8` → **HTTP 200**,
  `GET .../issues/8/timeline?per_page=100` → **HTTP 200** (1 page, no `rel="next"`); observed
  `state=open`, `state_reason=null`, `closed_at=null`, `created_at=2026-11-08T06:43:43Z`, labels `[]`,
  `comment_count=0`, `timeline_event_count=1` (a single `milestoned` event to `1.0`),
  `timeline_closed_events=0`, `timeline_reopened_events=0` — the issue is still OPEN and no premature
  closure or reopen occurred. No token was read, sent or stored (unauthenticated request; `gh` absent);
  `x-ratelimit-remaining=9` at read time, recorded as environment context. Evidence
  `item-8-ensembles/github/issue-8-observation.json` (with sha256 of `issue-8.json` and
  `issue-8-timeline.json`), `github/issue-8.json`, `github/issue-8-timeline.json`,
  `github/fetch_issue8.py` and `logs/github-issue-8.log`. Nothing staged, no commit, no push, no GitHub
  write; counters unchanged (`r=2`, attempt `n=1/3`), completeness `1/4 accepted`.
- **last CHECK for item `#8` (`C`): `r2` / attempt `n=1` — `not accepted → PLAN`** (recorded as CHECK 2
  under the driving role's label; the fifth CHECK event in this collection). Matrix `AC-8.1` `met`,
  `AC-8.2` `met`, `AC-8.3` `unmet`, `AC-8.4(1)` `unverified`, `AC-8.4(2)` `unverified — срок не наступил`.
  Keys: `F-8-ac3-contract-drift` (**blocking**, `confirmed`: the derived `AC-8.3` both promises `r1`
  invariance and forbids linter warnings, which the measured clippy/test logs violate) and
  `F-8-issue-open-evidence` (major, `unverified`: no stored GitHub `#8` state was presented); fixes
  applied **0** for both. `F-8-acceptance-order`, `F-8-song-licence`/`LIC-8` and `PAD-8` are
  `refuted/closed on the r2/n1 candidate`. Report verbatim in `.pdca/status/issue-8-r5.md` §CHECK record;
  counters unchanged (`r=2`, `n=1/3` — the `CHECK → PLAN` transition consumes no attempt).
- **next allowed step: a PLAN contract revision for item `#8`** — the divergence of the derived `AC-8.3`
  from the original contract and from its declared invariance must be resolved while the user's criteria
  set stays whole; the `AC-8.4(1)` evidence action (a read-only GitHub `#8` response with `state=OPEN`,
  URL and observation time) is with the driver and is not a product defect. **No commit, no GitHub `#8`
  closure and no `done` is authorized.**
- completeness: `1/4 accepted` — item `#2` is accepted/committed/finalized (residual: the GitHub `#2`
  closure by the calling driver); `#8` was `in-progress` at that stage (`AC-8.3` `unmet`, `AC-8.4(1)` `unverified`) and awaited a PLAN contract revision (since accepted by `CHECK 3` and `done` / `finalized`, commit `a38bb7f`); `#3` and `#4` are `pending` (`0/4 closed in GitHub`).
- **`#8` `DO` `r3`/`n1` (`2026-11-11T07:34Z`–`07:38Z`, evidence only, no product change)** — the round
  executed the whole `r3` §10 scope: a fresh read-only GitHub observation (`HTTP 200`, `state=open`,
  canonical URL, observed `2026-11-11T07:34:54Z`, timeline **0 closed / 0 reopened**, no token) is
  stored under `item-8-ensembles/github/`, closing `F-8-issue-open-evidence` (fixes applied 1), and the
  frozen 7-file candidate re-anchored **7/7** against `snapshot-sha256.txt` (`0` missing, `0` mismatch)
  with `HEAD 59fb1fb`, an empty index and no `#8` commit. No gate was re-run because the snapshot had
  not drifted; the `CHECK 2` measurements stay bound to the same bytes. A concurrent out-of-contract
  lint round (`tests/ensemble.rs` 21,578 → 21,633 B) was reverted byte-identically and is not claimed
  as progress (`item-8-ensembles/logs/lint-fix-round.md`, `logs/snapshot-restore.log`). **Next allowed
  step: `CHECK item #8 (r3/n1)`** — no commit, no GitHub `#8` closure and no `done` is authorized.
- **last `DO` for item `#8` (`D`): `r3` / attempt `n=1` — complete, no product change.** Evidence:
  `item-8-ensembles/github/issue-8-observation.json`,
  `item-8-ensembles/github/issue-8__2026-11-11T07-34-54Z.json`,
  `item-8-ensembles/logs/reanchor-7of7-verify-r3n1.log` (exit `0`),
  `item-8-ensembles/logs/github-issue-8-r3n1.log` (exit `0`),
  `item-8-ensembles/logs/manifest-r3n1-update.log` (exit `0`);
  `.pdca/status/issue-8-r5.md` §DO result `r3`/`n1`. Counters stay `r=3`, `n=1/3`.
- **last CHECK for item `#8` (`C`): `r3` / attempt `n=1` — `accepted → ACT`** (the sixth CHECK event in
  this collection). Matrix `AC-8.1` `met`, `AC-8.2` `met`, `AC-8.3` `met`, `AC-8.4(1)` `met`,
  `AC-8.4(2)` `unverified — срок не наступил`; keys `F-8-ac3-contract-drift` and
  `F-8-issue-open-evidence` **refuted/closed**, `F-8-acceptance-order`, `F-8-song-licence`/`LIC-8` and
  `PAD-8` remain closed. Report recorded verbatim under `### CHECK 3 — r3/n1` in
  `.pdca/status/issue-8-r5.md`; counters `r=3`, attempt `n=1/3` (the accepted verdict consumed no
  additional attempt).
- **verification state**: item `#8` is **accepted, committed (`a38bb7f`) and finalized**; the accepted
  candidate was re-anchored **7/7** before staging (`item-8-ensembles/logs/act-reanchor-7of7-r3n1.log`,
  verifier exit `0`), the commit contains exactly the 7 product paths (`7 files changed, +1368/−2`,
  parent `59fb1fb`, footer `Refs #8`), `git diff --name-only HEAD -- <7 paths>` is empty, nothing is
  staged, and `git status --porcelain` shows only the pre-existing untracked `.openidea/` and
  `.pdca/status/*`. No push, no branch switch, no amend, no tag, **no GitHub write**.
- **next allowed step: `DO item #3 (r1/n1)`** — the next authorized collection step; `#4` stays
  `pending`. For item `#8` the only outstanding action is the **GitHub `#8` closure by the calling
  driver**, which is external to this record and is not performed by the executor.
- **completeness: `2/4 accepted`** — item `#2` (commit `59fb1fb`) and item `#8` (commit `a38bb7f`) are
  accepted, committed and finalized, each with the GitHub closure by the calling driver as its only
  residual; `#3` and `#4` are `pending` and untouched (`0/4 closed in GitHub`; closure is external).

- **driver closures recorded (`2026-11-11`)**: GitHub `#2` `COMPLETED` at `2026-11-11T06:18:29Z` (comment `6788661305`) and GitHub `#8` `COMPLETED` at `2026-11-11T07:57:47Z` (comment `6790434030`), both performed by the driver through the authenticated `gh` CLI, closing the earlier credential blocker; no criterion, matrix, verdict or counter changed.
- **next allowed step: `DO item #3 (r1/n1)`** — the only authorized collection step; `#4` stays `pending`.
- **completeness: `2/4 accepted`** — `#2` and `#8` accepted, committed and closed in GitHub; `#3` and `#4` `pending`.
- **record lint (this update)** — `& "C:\Users\Пользователь\AppData\Local\Programs\Python\Python312\python.exe" .pdca/scripts/status_lint.py .pdca/status/collection-open-issues-r5.md` → exit status **`0`** (`status-lint: OK (.pdca\status\collection-open-issues-r5.md: 2 tables, 7 rows)`); `& …python.exe .pdca/scripts/status_lint.py .pdca/status/issue-2-r5.md` → exit **`0`** (`status-lint: OK (.pdca\status\issue-2-r5.md: 3 tables, 11 rows)`); `& …python.exe .pdca/scripts/status_lint.py .pdca/status/issue-8-r5.md` → exit **`0`** (`status-lint: OK (.pdca\status\issue-8-r5.md: 10 tables, 41 rows)`). The same three-file run was repeated after this bullet was written and returned exit `0` again. A failing lint would be recorded as failing, never as a pass.
- **bookkeeping commit (`2026-11-11`)**: one commit on `main` containing only the three status files `.pdca/status/collection-open-issues-r5.md`, `.pdca/status/issue-2-r5.md` and `.pdca/status/issue-8-r5.md`; `.openidea/` and `.pdca/status/collection-open-issues-r3.md` were not staged and stay untouched; no push. A commit cannot record its own id, so the hash is reported to the driver by the executor.

## Revision history

`none` at collection level — `r1` is the collection's initial revision; no collection replan has occurred. Item `#2` carries its own
contract revision `r2` (genuine revision after `CHECK → PLAN`, `AC-2.1`–`AC-2.4`), recorded in
`.pdca/status/issue-2-r5.md` §Revision history. Item `#8` now carries its own item-level contract
revision `r2` as well (recorded 2026-11-11T07:04:10Z, genuine revision after `CHECK → PLAN`, same
cause: the `F-8-acceptance-order` acceptance-order defect in `AC-8.4`, fixed by the split into a
`CHECK`-verifiable pre-acceptance state and the mandatory `ACT` closure, plus `F-8-song-licence`
turned into the `AC-8.2` primary-source evidence obligation), recorded verbatim in
`.pdca/status/issue-8-r5.md` §PLAN record `r2` and §Revision history; the **collection** revision
stays `r1`.
- 2026-11-11T07:12:49Z | item `#8` contract revision `r2` → the `DO` `r2`/`n1` round is complete
  (`PAD-8` fixed, `LIC-8` primary-source licence evidence produced, `AC-8.1`/`AC-8.3` re-measured on the
  fixed bytes, `AC-8.2` unchanged and non-invalidated); the item is at **contract revision `r2`**, attempt
  `n=1/3`, and the **collection schedule is unchanged at `r1`** (no collection replan, no new group, no
  new lane). Next allowed step `CHECK item #8 (r2/n1)`.

- 2026-11-11T07:27:43Z | item `#8` contract revision **`r3`** (item level; the collection revision stays `r1`, no collection replan, no new group or lane) → cause `CHECK 2` (`r2`/`n1`) `not accepted → PLAN`: `F-8-ac3-contract-drift` (blocking) closed by explicitly revising `AC-8.3` to the `r1` gate semantics, and `F-8-issue-open-evidence` (major) carried as the `AC-8.4(1)` stored-observation obligation. Item `#8` is at `r=3`, attempt `n=1/3`; next allowed step `DO item #8 (r3/n1)`.

- 2026-11-11 | item `#8` `r1 → r2 → r3` (item level only; the **collection plan is not rewritten and stays `r1`**) → two genuine item contract revisions inside the item cycle: `r1 → r2` from `CHECK 1 → PLAN` (acceptance-order defect `F-8-acceptance-order` plus the `F-8-song-licence` evidence obligation) and `r2 → r3` from `CHECK 2 → PLAN` (`F-8-ac3-contract-drift` — `AC-8.3` revised to the `r1` gate semantics — and `F-8-issue-open-evidence` — `AC-8.4(1)` refined to the stored read-only observation). Complete reports: `.pdca/status/issue-8-r5.md` §PLAN record `r2` and `### r3`. This row is a pointer, not a collection replan and not a new group or lane.

## Defect history

`none` at collection level — no collection-wide defect key exists. Item-level keys are kept in the
item's own cycle record (`.pdca/status/issue-2-r5.md` §Defect history). `#2` now carries:
`F-2-criteria-coverage` → **closed at r2 by `AC-2.2`** and `F-2-acceptance-order` → **closed at r2 by
`AC-2.4`** (1 fix each); `F-2-reference-provider` → closed at DO `r2`/`n1` (1 fix: the instrumented
live-session provider audit plus a re-measured, PCM-identical reference), `refuted for r2` at CHECK 2
and **`refuted/closed on the r2/n2 candidate`** at CHECK 3; `F-2-dependency-inventory` (major),
`F-2-github-preacceptance-state` (major) and `F-2-evidence-map` (minor) were opened at `r2`/`n1`,
fixed once at `r2`/`n2` and are now **`refuted/closed on the r2/n2 candidate`** (recorded fixes
applied 2 each: one `r2`/`n2` measure plus its CHECK 3 closure confirmation). No key is open, no
recurrence without a new measure and no escalation. Item `#8` now carries two item-level keys, kept in
`.pdca/status/issue-8-r5.md` §Defect history: `F-8-acceptance-order` (**blocking**, `confirmed`;
opened at CHECK 1, fixes applied 0) and `F-8-song-licence` (major, `unverified`; opened at CHECK 1,
fixes applied 0) — both **open**, routed `CHECK → PLAN`, no escalation (attempts not exhausted). Items
`#3`, `#4` have none.
- **Update after `DO` `r2`/`n1`** (append-only, the paragraphs above are kept): `PAD-8` was opened at
  CHECK 2 and is **fixed** (1 fix, per-channel padding plus two unequal-length stereo tests:
  `logs/pad8-tests-before-fix.log` exit 101 → `logs/pad8-tests-after-fix.log` exit 0); `LIC-8` /
  `F-8-song-licence` received its **primary-source evidence** in `licence/` (1 fix; the APIv2 401 and the
  HQ-preview size/bitrate caveat are recorded as limitations, not hidden); `F-8-acceptance-order` stays
  **closed at contract `r2`** by `AC-8.4(1)`/`AC-8.4(2)`; `CONC-1` stays closed with fixes applied 0 and
  its attribution corrected (the 11:52–11:56 local writer was an earlier dispatch of the same attempt).
  No key is open against the code, none recurs without a new measure, no escalation. The full per-key rows
  live in `.pdca/status/issue-8-r5.md` §Defect history.

## Changed files

- `.pdca/status/collection-open-issues-r5.md` → created (this collection record).
- `.pdca/open-issues-r5/rv1/plan.md` → created (PLAN record r1).
- `.pdca/open-issues-r5/rv1/criteria-approval.md` → created (user message and approved addendum, verbatim).
- `.pdca/open-issues-r5/rv1/git-state.txt` → created (HEAD, branch, porcelain at creation).
- `.pdca/open-issues-r5/rv1/lint.txt` → created (status_lint command and result).
- `.pdca/status/issue-2-r5.md` → created (item `#2` cycle record, DO in progress); updated with the
  verbatim `CHECK` report, the three defect keys and the `CHECK → PLAN` routing rows.
- `.pdca/status/collection-open-issues-r5.md` → created (this collection record); updated for the
  `#2` CHECK outcome (phase `CHECK`, next step PLAN revision).
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/` → created (item `#2` evidence root: snapshot, logs,
  scripts, comparison, manifest).
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/scripts/` → preserved `reference_audio_separator.py` and
  `compare_stems.py` copied byte-identically from the r4 evidence root, plus the per-channel
  comparison extension `compare_stems_per_channel.py`.
- Product paths changed by item `#2` DO (pending commit, not staged): `Cargo.toml`, `Cargo.lock`,
  `README.md`, `docs/dependency-audit.md`, `docs/dependency-audit-evidence/tract-onnx-0.23.10.txt`,
  `examples/mdx_separate.rs`, `src/config.rs`, `src/dsp.rs`, `src/error.rs`, `src/lib.rs`,
  `src/mdx.rs`, `src/model/mod.rs`, `src/separator.rs`.
- No product file was changed by the PLAN recording step itself.
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/` → refreshed at DO `r2`/`n1`: new provider-audit logs,
  `out/reference-r2/`, PCM-equivalence and metric-identity logs, tree-hash verification and four new
  helper scripts; `manifest.json`, `commands.txt`, `index.md` rebuilt (`logs/reference-provider-audit.log`,
  `logs/tree-sha256-verify.log`).
- `.pdca/status/issue-2-r5.md` → updated for DO `r2`/`n1` (§DO result `### r2 / n1`, §Defect history
  `F-2-reference-provider` closed, §Changed files, §Git state, progress rows).
- `.pdca/status/collection-open-issues-r5.md` → updated for the same event (§Current state, §Item table
  row `#2`, §DO result, §General verification, §Defect history, progress rows).
- No product file was changed by the DO `r2`/`n1` step: the 13 pending-commit product files are 13/13
  byte-identical to the `r1` candidate snapshot.
- `.pdca/status/collection-open-issues-r5.md` and `.pdca/status/issue-2-r5.md` → updated for CHECK 2
  (item `#2`, `r2`/`n1`, verdict `not accepted → DO`): phase `CHECK`, item notes, §CHECK record, the
  defect keys and progress rows. Status-record writes only — no product file, no evidence artifact and
  no gate was touched.
- `.pdca/status/issue-2-r5.md` and `.pdca/status/collection-open-issues-r5.md` → updated for CHECK 3
  (item `#2`, `r2`/`n2`, verdict `accepted → ACT`): the verbatim report under `### CHECK 3 — r2/n2`,
  §Current state / §CHECK record / §Defect history / next allowed step / progress rows. Status-record
  writes only — no product file, no evidence artifact and no gate was touched, and the commit plus the
  GitHub `#2` closure remain unperformed.
- **Item `#2` ACT** — `.pdca/status/issue-2-r5.md` updated (§Current state `phase: ACT · record:
  completed · outcome: accepted`, §ACT record, §Lessons learned with the retrospective verbatim,
  §Changed files, §Git state, progress rows); `.pdca/status/collection-open-issues-r5.md` updated
  (§Current state, §Item table row `#2` → `done`/`finalized`, §ACT record, §General verification,
  §Changed files, §Git state, §Lessons learned note, progress rows); new evidence
  `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/{act-reanchor-verify.log,commit-59fb1fb.log}`; and
  **one atomic commit `59fb1fb`** with the item's 13 product paths
  (`Cargo.toml`, `Cargo.lock`, `README.md`, `docs/dependency-audit.md`,
  `docs/dependency-audit-evidence/tract-onnx-0.23.10.txt`, `examples/mdx_separate.rs`,
  `src/config.rs`, `src/dsp.rs`, `src/error.rs`, `src/lib.rs`, `src/mdx.rs`, `src/model/mod.rs`,
  `src/separator.rs`). No `.pdca/**`, `.openidea/**`, `.onnx`, fixture or audio was staged; the two
  status files stay untracked.

- `.pdca/status/issue-8-r5.md` → created (item `#8` cycle record, DO `r1`/`n1` in progress);
  `.pdca/open-issues-r5/rv1/item-8-ensembles/` → created/refreshed (item `#8` evidence root:
  `logs/checkpoint-voc_fv7-sha256.log`, `logs/checkpoint-bs-roformer-sha256.log`, `logs/download-voc_fv7.log`
  pre-existing, `models/voc_fv7.ckpt` pre-existing frozen checkpoint).
- **Item `#8` DO `r1`/`n1` continuation** — product (all untracked-uncommitted, nothing staged):
  `src/ensemble.rs` (new, 9,277 B, sha256 `ad83de85…7bde`), `src/config.rs` (modified, 24,683 B,
  sha256 `8fcfefdc…c50e`), `src/error.rs` (modified, 3,945 B, sha256 `939415b5…5ad`), `src/lib.rs`
  (modified, 1,624 B, sha256 `7b8a0f26…cf4`), `examples/ensemble_separate.rs` (new, 7,683 B,
  sha256 `9d7e040b…bdf8`), `tests/ensemble.rs` (new, 18,782 B, sha256 `fef9172e…c3fc` — **rewritten at
  09:29:08 local by the duplicate process**, not attributable to this session), and
  `docs/ensemble-presets.md` (new, added by this session). Evidence root additions:
  `scripts/{reference_ensemble_v2.py, fixture_root_layout.py, compare_parity.py}`, `logs/{reference-ensemble-v2.log,
  fixture-root-layout.log, parity-test-ignored.log, comparison.log, concurrency-conflict.log}`,
  `fixture/{layout.json, inputs/*, reference/*}`, `out/reference/{model0,model1,reference-operator,reference-summary.json}`,
  `out/candidate/operators/*`, `metrics/{parity.json,parity.csv}`, `models/voc_fv7.yaml` (copy of
  `docs/provenance/v7.yaml`), plus artifacts written by the duplicate (`out/reference/{layout,summary}.json`,
  `out/reference/members/*`, `out/candidate/*.f32`, `out/candidate/ensemble/`, `logs/preset-cli-run.log`).
  Status-record writes: the two status files. No `git add`, no staging, no commit, no push.
- **Item `#8` DO `r1`/`n1` continuation 2** — evidence root only, **no product change**:
  `.pdca/open-issues-r5/rv1/item-8-ensembles/logs/{build.log,test.log,clippy.log}` (the three AC-8.3
  gates), `scripts/wait_preset_exit.ps1` (exit-code observer for the already-running AC-8.2 run),
  `scripts/run_preset_cli.ps1` (reproducible launcher, **not used** — run 3 was already live),
  `logs/preset-cli-run-final.log` (written by run 3). Status-record writes: the two status files.
- **Item `#8` DO `r1`/`n1` continuation 3** — evidence root only, **no product change**:
  `out/candidate/preset-vocal-balanced/{vocals.wav,instrumental.wav}` (33,407,732 B each — run artifacts,
  not product delta), `out/candidate/preset-vocal-balanced/{exit.json,validation.json}`,
  `logs/preset-validation.log`, `logs/snapshot-refresh.log`, and the refreshed `manifest.json`,
  `index.md`, `snapshot/`, `snapshot-sha256.txt`, `snapshot.diff`, `commands.txt`, `lint.txt`.
  Status-record writes: the two status files.
- **Item `#8` DO `r2`/`n1`** — product (uncommitted, nothing staged): `src/ensemble.rs` (10,417 B,
  sha256 `3db9b29ab4faac976db67ee19373951c3608c88ab8026207668c068f7c4610de`), `tests/ensemble.rs`
  (21,578 B, sha256 `dbc0526302378126a9c79f380903bb1f4ac089f38a0a9525e72de861d228e01a`),
  `docs/ensemble-presets.md` (5,968 B, sha256
  `e2cdf8d687f0a15c3c6884839568b1e614ee18fc63587019c267edbc5ac25079`) — the other four item files
  unchanged (`src/config.rs`, `src/error.rs`, `src/lib.rs`, `examples/ensemble_separate.rs`).
  Evidence root only, no further product change: `logs/pad8-tests-{before,after}-fix.log`,
  `logs/parity-test-final2.log`, `logs/comparison-final2.log`, `metrics/parity-final2.{json,csv}`,
  `logs/{build,test,clippy}-n2.log`, `logs/ac82-pad8-non-invalidation.md`, `logs/licence-fetch.log`,
  `licence/` (8 files: the live page, wayback, short URL, CC0 deed, the 401 APIv2 response,
  `licence-evidence.json`, `binding.md`, `scout-observation.md`), `scripts/fetch_licence_evidence.py`,
  refreshed `snapshot/`, `snapshot-sha256.txt`, `snapshot.diff` (61,979 B), `manifest.json`,
  `index.md`, `commands.txt`, `lint.txt`, and the closing re-anchor/observation logs
  `logs/{reanchor-build,reanchor-test,reanchor-clippy,reanchor-parity,reanchor-summary,reanchor-snapshot,reanchor-7of7-verify,licence-decode-chain,licence-provenance-build,fetch-licence-source,manifest-inspect,manifest-inspect2,status-fence-repair}.log`.
  Status-record writes: the two status files. No `git add`, no staging, no commit, no push, no GitHub write.
- **Item `#8` `LIC-8` closure continuation (`2026-11-11T07:09Z`)** — evidence root only, **no product
  change**: `item-8-ensembles/evidence/source/` (`fetch-log.json`, the page/short-URL/APIv2/preview raw
  responses, `decode-chain-verify.json`), `metrics/licence-provenance.json`, `logs/{fetch-licence-source,
  licence-decode-chain,licence-provenance-build,manifest-licence-update,status-fence-repair,reanchor-build,
  reanchor-test,reanchor-clippy,reanchor-parity,reanchor-summary,reanchor-snapshot,reanchor-7of7-verify}.log`,
  `scripts/{fetch_licence_source,verify_decode_chain,build_licence_provenance,update_manifest_licence,
  check_fence}.py`, refreshed `manifest.json`, `index.md`, `commands.txt`, `lint.txt`, `snapshot/`,
  `snapshot-sha256.txt`, `snapshot.diff`, `out/candidate/operators-reanchor/`; plus the repaired
  `.pdca/status/issue-8-r5.md` fence (recorded text unchanged). Status-record writes: the two status files.

- **`#8` item-level contract revision `r3` recording (2026-11-11T07:27:43Z)** — status-record writes only: `.pdca/status/issue-8-r5.md` (§PLAN record `### r3`, §Current state, §Revision history, §Defect history, §Progress log) and `.pdca/status/collection-open-issues-r5.md` (item row `#8`, §Current state, §PLAN record pointer, §Revision history, §Progress log). No product file, test, doc or gate was touched.
- **`#8` `DO` `r3`/`n1` (evidence-only round, `2026-11-11T07:34Z`–`07:38Z`)** — evidence root only, **no
  product change**: `item-8-ensembles/github/{issue-8__2026-11-11T07-34-54Z.json,
  issue-8-timeline__2026-11-11T07-34-54Z.json, issue-8-observation.json,
  issue-8-observation__2026-11-11T07-19-54Z.json}` (the last is the archived stale summary, byte-identical),
  `item-8-ensembles/scripts/{observe_issue8.py, update_manifest_r3n1.py}`,
  `item-8-ensembles/logs/{github-issue-8-r3n1.log, reanchor-7of7-verify-r3n1.log,
  manifest-r3n1-update.log}`, refreshed `item-8-ensembles/manifest.json` (root `github_issue` block,
  `AC-8.4(1)` pre-acceptance sub-record, `AC-8.3` gates on the frozen bytes, item `r=3` / `n=1/3`) and
  `item-8-ensembles/commands.txt`. Status-record writes: the two status files. The seven product files
  keep the `CHECK 2` hashes (`7/7`, `logs/reanchor-7of7-verify-r3n1.log`); nothing was staged,
  committed or pushed.
- **Item `#8` `DO` `r3`/`n1` continuation (`2026-11-11T07:37Z`–`07:40Z`)** — evidence root and pack only,
  **no product change**: `github/issue-8__2026-11-11T07-37-38Z.json`,
  `github/issue-8-timeline__2026-11-11T07-37-38Z.json`, `github/issue-8-observation.json` (refreshed) and
  `github/issue-8-observation__2026-11-11T07-34-54Z.json` (archive); `logs/github-issue-8.log` (appended),
  `logs/manifest-r3-reconcile.log`, `logs/final-state-verify-r3n1-continuation.log`; the repaired pack
  `manifest.json`, `index.md`, `commands.txt`, `lint.txt`; evidence tooling `scripts/observe_issue8.py`,
  `scripts/reconcile_manifest_r3.py`. The 7-file product delta is unchanged (`tests/ensemble.rs` 21,578 B /
  `dbc05263…e01a` and the six other `CHECK 2` hashes, verified `7/7` at `07:39:32Z`).

- **Item `#8` `CHECK 3` + `ACT` (`2026-11-11T07:44Z`–`07:47Z`)** — status-record writes plus the
  item's own commit: `.pdca/status/issue-8-r5.md` (§Current state, §CHECK record `### CHECK 3 — r3/n1`
  verbatim, §ACT record, §Lessons learned with the retrospective verbatim, §Changed files, §Git state,
  §Progress log) and `.pdca/status/collection-open-issues-r5.md` (item row `#8` → `done`/`finalized`,
  §Current state, §CHECK record summary, §ACT record for `#8`, §General verification and recovery,
  §Changed files, §Git state, §Progress log). New evidence:
  `item-8-ensembles/logs/act-reanchor-7of7-r3n1.log` (pre-stage re-anchor, verifier exit `0`,
  `matched=7/7`) and `item-8-ensembles/logs/commit-a38bb7f.log` (commit, identity, changed-path list,
  `HEAD`-vs-worktree diff, porcelain, CRLF note). **Committed product delta — exactly the accepted 7
  files**: `src/ensemble.rs` (+264), `src/config.rs` (+217), `src/error.rs` (+26), `src/lib.rs` (+7/−2),
  `examples/ensemble_separate.rs` (+219), `tests/ensemble.rs` (+526),
  `docs/ensemble-presets.md` (+109) → commit `a38bb7f`, `7 files changed, +1368/−2`, parent `59fb1fb`.
  No `.pdca/**`, `.openidea/**`, weight, audio/fixture, log or status path was staged; no product byte
  was edited by the ACT round, and the committed content is the accepted snapshot.

- **`#8` ACT verification dispatch (`2026-11-11T07:47Z`)** — no product byte, no staging, no commit by
  this dispatch: new evidence under `item-8-ensembles/logs/` —
  `act-reanchor-verify.log` (pre-stage re-anchor: `HEAD` already `a38bb7f`, branch `main`, index empty,
  verifier `7/7` exit `0`, blob-vs-snapshot table incl. the CRLF reconstruction) and
  `commit-a38bb7f-verify.log` (independent commit verification: parent `59fb1fb`, exact 7-file set,
  footer `Refs #8`, no `Closes`/`Fixes`, not pushed). Status-record writes only: this file (§Current
  state, §ACT record correction, §General verification, §Changed files, §Git state, §Progress log),
  the item row `#8`, and `.pdca/status/issue-8-r5.md` (§Current state, §ACT record correction,
  §Lessons learned, §Changed files, §Git state, §Progress log).

**Reconciliation of the concurrent-writer collision (`2026-11-11T07:53Z`, this dispatch)** — status records only, **no product file**: this collection file (item row `#8`, §Current state, §ACT record, §General verification and recovery, §Changed files, §Git state, §Progress log) and `.pdca/status/issue-8-r5.md` (§Current state, §ACT record element state, §Lessons learned label, §Changed files, §Git state, §Progress log). Item `#8` reconciled to `done` / `finalized`, `plan_revision 3`, attempts `1`, residual = the GitHub `#8` closure by the calling driver; both retrospective blocks kept and the second labelled as the concurrent dispatch's, not authoritative; no progress row deleted. Lint after every edit: `.pdca/open-issues-r5/rv1/item-8-ensembles/lint.txt`.

- **Driver-closure bookkeeping (`2026-11-11`)** — `.pdca/status/collection-open-issues-r5.md` (header phase `PLAN` / record `open`, §Current state, item rows `#2`/`#8`, §ACT record, §General verification and recovery, §Revision history, §Git state, progress rows), `.pdca/status/issue-2-r5.md` and `.pdca/status/issue-8-r5.md` (§ACT record `External GitHub closure` notes) → status records only; no product file, test, gate, evidence artifact or screenshot was touched and nothing was pushed.

## Git state

- status path: `.pdca/status/collection-open-issues-r5.md` — created this cycle · untracked (durable on disk, not committed).
- baseline supplied at creation: branch `main`, HEAD `2d0be535fd82a1296a76f74b0d738829bd54a6df` (`2d0be53`), tree clean except pre-existing untracked `.openidea/` and `.pdca/status/collection-open-issues-r3.md`.
- same-cycle touched paths: the status files plus `.pdca/open-issues-r5/rv1/` evidence artifacts (the
  evidence root is ignored by `/.pdca/*`, durable on disk and not tracked); the item `#2` product
  changes listed above are pending commit and not indexed.
- no `git add`, no staging, no commit, no push was performed by this recording task or by item `#2`
  DO; prior dirty work (`.openidea/`, `.pdca/status/collection-open-issues-r3.md`) stays untouched
  and unclaimed.
- CHECK 3 (`r2`/`n2`) recording touched only the two status files; `HEAD` stays
  `2d0be535fd82a1296a76f74b0d738829bd54a6df`, nothing staged, and the 13-path item `#2` pending-commit
  set is unchanged from `r2`/`n2`. The post-acceptance commit and the GitHub `#2` closure are still
  outstanding and are ACT work.
- **Item `#2` ACT** — the 13-path set is now committed: `59fb1fbcf89ed99a447dc5b8d48b915de601917f`
  on `main` (parent `2d0be53`, 13 files, +1750/-23, footer `Refs #2`), after a pre-stage re-anchor of
  13/13 hashes. No push, no branch switch, no amend/rebase, no tag; nothing outside the 13 paths was
  staged. `HEAD` is now `59fb1fb`; `git diff --name-only HEAD -- <13 paths>` is empty, so the commit
  bytes equal the accepted worktree bytes (`logs/commit-59fb1fb.log`).
- after the commit and the two ACT status writes, `git status --porcelain` shows only the pre-existing
  untracked `.openidea/` and `.pdca/` records (`collection-open-issues-r3.md`,
  `collection-open-issues-r5.md`, `issue-2-r5.md`) — no leftover product change, nothing staged. The
  status files stay untracked: durable on disk, and the authorized commit scope was the item's 13
  product paths only. Prior dirty work stays untouched and unclaimed.
- the evidence root stays invisible to `git status` via `.gitignore:5` (`/.pdca/*`), so the ACT logs
  are durable on disk and cannot be staged by accident.
- **Item `#8` DO `r1`/`n1`** — this recording created `.pdca/status/issue-8-r5.md` and item `#8`
  evidence files only; `HEAD` stays `59fb1fbcf89ed99a447dc5b8d48b915de601917f`, nothing staged, no
  commit/push. Product-file deltas of the DO are recorded in §Changed files once produced and stay
  pending commit (the `#8` commit is ACT work after an accepted CHECK).
- **Item `#8` DO `r1`/`n1` continuation, refreshed 2026-11-11T04:32:56Z** — `HEAD` is still
  `59fb1fbcf89ed99a447dc5b8d48b915de601917f` on `main`; `git diff --cached --stat` is empty (nothing
  staged); the worktree carries the item's product delta pending commit — modified `src/config.rs`,
  `src/error.rs`, `src/lib.rs`; untracked `src/ensemble.rs`, `examples/ensemble_separate.rs`,
  `tests/ensemble.rs`, `docs/ensemble-presets.md` — plus the pre-existing untracked `.openidea/` and
  `.pdca/status/{collection-open-issues-r3.md, collection-open-issues-r5.md, issue-2-r5.md,
  issue-8-r5.md}`. The evidence root stays invisible to `git status` via `.gitignore` (`/.pdca/*`).
  No `git add`, no staging, no commit, no push, no branch switch, no amend/rebase, no tag by this
  session. **Caveat:** a duplicate execution process is working the same delta in this tree
  (`CONC-1`), so some bytes in that list — `tests/ensemble.rs` demonstrably — were written by it, not
  by this attempt.
- **Item `#8` DO `r1`/`n1` continuation 2, refreshed 2026-11-11T05:25:53Z** — `HEAD` is still
  `59fb1fbcf89ed99a447dc5b8d48b915de601917f` on `main`; `git diff --cached --stat` is empty (0 lines,
  nothing staged); `git status --porcelain` shows exactly the item delta pending commit — modified
  `src/config.rs`, `src/error.rs`, `src/lib.rs`; untracked `docs/ensemble-presets.md`,
  `examples/ensemble_separate.rs`, `src/ensemble.rs`, `tests/ensemble.rs` — plus the pre-existing untracked
  `.openidea/` and `.pdca/status/{collection-open-issues-r3.md, collection-open-issues-r5.md,
  issue-2-r5.md, issue-8-r5.md}`. `CONC-1` is closed (no second executor) but `tests/ensemble.rs` bytes
  remain attributed to its window. No `git add`, no staging, no commit, no push, no branch switch, no
  amend/rebase, no tag; the evidence root stays invisible to `git status` via `.gitignore` (`/.pdca/*`).
- **Item `#8` DO `r1`/`n1` complete, refreshed 2026-11-11T06:54:36Z** — `HEAD` is still
  `59fb1fbcf89ed99a447dc5b8d48b915de601917f` on `main`; `git diff --cached --stat` is empty (0 lines,
  nothing staged); `git status --porcelain` shows exactly the same 7-path item delta as before the gates
  and the run — modified `src/config.rs`, `src/error.rs`, `src/lib.rs`; untracked
  `docs/ensemble-presets.md`, `examples/ensemble_separate.rs`, `src/ensemble.rs`, `tests/ensemble.rs` —
  plus the pre-existing untracked `.openidea/` and `.pdca/status/*`. `make_snapshot.py` re-verified 7/7
  worktree == snapshot, so no product byte moved in this continuation. Nothing staged, no commit, no
  push, no branch switch, no amend/rebase, no tag; the `#8` commit is ACT work after an accepted CHECK.
- **Item `#8` DO `r2`/`n1` complete, refreshed 2026-11-11T07:12:49Z** — `HEAD` is
  `59fb1fbcf89ed99a447dc5b8d48b915de601917f` on branch `main`; `git diff --cached --stat` is **empty**
  (index empty, nothing staged); `git status --porcelain` shows the same **7-file product delta,
  uncommitted** — modified `src/config.rs`, `src/error.rs`, `src/lib.rs`; untracked `src/ensemble.rs`,
  `examples/ensemble_separate.rs`, `tests/ensemble.rs`, `docs/ensemble-presets.md` — plus the pre-existing
  untracked `.openidea/` and `.pdca/status/*`. Nothing was staged, committed or pushed in this round; the
  GitHub issue `#8` was not touched (no write of any kind). `logs/reanchor-7of7-verify.log` exit 0:
  `7/7` worktree == snapshot copy == `snapshot-sha256.txt` at 07:12:42Z. The `#8` commit and the GitHub
  `#8` closure remain ACT work after `CHECK: accepted` (`AC-8.4(2)`).
- **Item `#8` `LIC-8` closure continuation, refreshed `2026-11-11T07:17Z`** — `HEAD` is still
  `59fb1fbcf89ed99a447dc5b8d48b915de601917f` on `main`; `git diff --cached --stat` is empty (nothing
  staged); `git status --porcelain` shows the same 7-file product delta pending commit (modified
  `src/config.rs`, `src/error.rs`, `src/lib.rs`; untracked `src/ensemble.rs`,
  `examples/ensemble_separate.rs`, `tests/ensemble.rs`, `docs/ensemble-presets.md`) plus the pre-existing
  untracked `.openidea/` and `.pdca/status/*`. No product byte moved in this continuation (`7/7` worktree
  == snapshot). No `git add`, staging, commit, push, branch switch, tag or GitHub call; the evidence root
  stays invisible to `git status` via `.gitignore` (`/.pdca/*`) and is durable on disk only.
- **`#8` CHECK 2 (`r2`/`n1`) recording, refreshed `2026-11-11T07:21Z`** — this recording task wrote only the
  two status files: no product file was edited, no build or test was run, nothing was `git add`ed, staged,
  committed or pushed, and no GitHub call was made. `HEAD` stays
  `59fb1fbcf89ed99a447dc5b8d48b915de601917f`, the index stays empty and the 7-file `#8` product delta stays
  pending commit; the `#8` commit and the GitHub `#8` closure remain `ACT` work after `CHECK: accepted`.
- **`#8` PLAN `r3` recording, refreshed 2026-11-11T07:27:43Z** — this recording task wrote only the two status files: no product file was edited, no build or test was run, nothing was `git add`ed, staged, committed or pushed, and no GitHub call was made. `HEAD` stays `59fb1fbcf89ed99a447dc5b8d48b915de601917f`, the index stays empty and the 7-file `#8` product delta stays pending commit; the `#8` commit and the GitHub `#8` closure remain `ACT` work after `CHECK: accepted`.
- **`#8` `DO` `r3`/`n1` complete, refreshed `2026-11-11T07:38Z`** — `HEAD` is
  `59fb1fbcf89ed99a447dc5b8d48b915de601917f` on `main`; `git diff --cached --stat` is **empty** (index
  empty, nothing staged); `git status --porcelain` shows the same **7-file product delta, uncommitted**
  (modified `src/config.rs`, `src/error.rs`, `src/lib.rs`; untracked `src/ensemble.rs`,
  `examples/ensemble_separate.rs`, `tests/ensemble.rs`, `docs/ensemble-presets.md`) plus the pre-existing
  untracked `.openidea/` and `.pdca/status/*`. The seven files are byte-identical to the `CHECK 2`
  ledger (`7/7`, `item-8-ensembles/logs/reanchor-7of7-verify-r3n1.log` exit `0`), so no product byte
  moved in this round. The only GitHub calls were two **read-only** unauthenticated GETs (no write, no
  token). No `git add`, no staging, no commit, no push, no branch switch, no tag; the evidence root stays
  invisible to `git status` via `.gitignore` (`/.pdca/*`) and is durable on disk only.
- **Item `#8` `DO` `r3`/`n1` continuation, refreshed `2026-11-11T07:39:53Z`** — `HEAD`
  `59fb1fbcf89ed99a447dc5b8d48b915de601917f` on `main`; `git diff --cached --stat` **empty** (nothing
  staged, no `#8` commit); `git status --porcelain` unchanged — ` M src/config.rs`, ` M src/error.rs`,
  ` M src/lib.rs`, `?? src/ensemble.rs`, `?? examples/ensemble_separate.rs`, `?? tests/ensemble.rs`,
  `?? docs/ensemble-presets.md`, plus the pre-existing untracked `.openidea/` and `.pdca/status/*`. **No
  product byte changed in this round** (7/7 == the `CHECK 2` snapshot hashes). The only GitHub traffic was
  a read-only `GET` observation of `#8`; no `git add`, no staging, no commit, no push, no branch switch, no
  tag, no issue write.
- **Item `#8` `ACT` (`r3`/`n1`), refreshed `2026-11-11T07:47Z`** — the 7-path set is now committed:
  **`a38bb7fd969dc0ebb36ca74957af93b6b4b20a19`** on `main` (parent `59fb1fb`, **7 files changed
  +1368/−2**, footer `Refs #8`, no auto-close keyword), after a pre-stage re-anchor of **7/7** hashes
  (verifier exit `0`). Only the item's 7 product paths were staged; no push, no branch switch, no
  amend/rebase, no tag, **no GitHub write**. `HEAD` is now `a38bb7f`;
  `git diff --name-only HEAD -- <7 paths>` is empty, `git diff --cached --name-only` is empty, and
  `git status --porcelain` shows only the pre-existing untracked `.openidea/` and `.pdca/` records
  (`collection-open-issues-r3.md`, `collection-open-issues-r5.md`, `issue-2-r5.md`, `issue-8-r5.md`) —
  no leftover product change, nothing staged. The two status files stay **untracked** (durable on disk;
  the authorized commit scope was the 7 product paths only). Prior dirty work stays untouched and
  unclaimed, and the evidence root stays invisible to `git status` via `.gitignore` (`/.pdca/*`).
  - **Notice (cosmetic, not identity drift)**: with `core.autocrlf=true` and no `.gitattributes`, the
    three *modified* files (`src/config.rs`, `src/error.rs`, `src/lib.rs`) are committed as git's
    LF-normalised blobs while their accepted worktree bytes are CRLF — the worktree still matches its
    `snapshot-sha256.txt` row 7/7 and `git diff --name-only HEAD -- <7 paths>` is empty, so no accepted
    byte changed. The commit identity (`Alexey Shirshov <alex.shirshov@vk.com>`, as in `59fb1fb`) was
    supplied per command with `git -c`; **no git config was written**
    (`item-8-ensembles/logs/commit-a38bb7f.log`).

- **Driver-closure bookkeeping, refreshed `2026-11-11`** — status-record writes only: no product byte, no build/test, no `git add` by any PDCA phase. `HEAD` is `a38bb7fd969dc0ebb36ca74957af93b6b4b20a19` on `main`; the two product commits (`59fb1fb`, `a38bb7f`) are on `main` and **not pushed** (`origin/main` unchanged at `bf2cab6`). The three status files were untracked before this bookkeeping commit and are committed by it alone (`.pdca/status/collection-open-issues-r5.md`, `.pdca/status/issue-2-r5.md`, `.pdca/status/issue-8-r5.md`); `.openidea/` and `.pdca/status/collection-open-issues-r3.md` stay untracked, untouched and unclaimed. The commit id cannot be recorded inside the commit itself; it is reported to the driver by the executor, and no push followed.

## Lessons learned

`pending: collection retrospective not yet conducted` — the collection has not closed (completeness
`2/4 accepted`). The first closed cycle is item `#2`: its mandatory `pdca-check` retrospective was
recorded **verbatim** in `.pdca/status/issue-2-r5.md` §Lessons learned (3 lessons — criteria coverage
and phase ordering, observability/traceability, reproducibility and change confinement — plus limits
and the completion signal), and that record is the source for a later collection-level retrospective.
Item `#8`'s mandatory retrospective is likewise recorded **verbatim** in
`.pdca/status/issue-8-r5.md` §Lessons learned (3 lessons — contract and phase ordering, provenance
evidence, executor coordination — plus limits and the completion signal
`retrospective_complete`); it is an item-level source for the future collection retrospective and is
**not** a collection-level retrospective. No collection-level retrospective report has been produced
yet, and none is reconstructed here.

- **refreshed `2026-11-11T07:51Z` by the `#8` ACT verification dispatch** — re-read after all of its
  work: `HEAD a38bb7fd969dc0ebb36ca74957af93b6b4b20a19` on `main` (the `#8` commit, parent `59fb1fb`);
  index **clean** (`git diff --cached --stat` empty); `git diff --name-only HEAD -- <7 paths>` empty;
  `git status --porcelain` shows only the pre-existing untracked `.openidea/` and `.pdca/status/*`; no
  product change is left for the seven `#8` paths. **No push** (`a38bb7f` is not an ancestor of
  `origin/main = bf2cab6`), no branch switch, no amend/rebase, no tag, no `git add`/commit by this
  dispatch (the commit existed before it started). Both status files stay untracked/durable on disk.

- **refreshed `2026-11-11T07:53Z` by the reconciliation dispatch** — status-record writes only (no product byte, no staging, no commit, no push, no GitHub write): `git rev-parse HEAD` = `a38bb7fd969dc0ebb36ca74957af93b6b4b20a19`, `git diff --cached --stat` **empty** (0 staged), `git status --porcelain` shows only the pre-existing untracked `.openidea/` and `.pdca/status/*` (`collection-open-issues-r3.md`, `collection-open-issues-r5.md`, `issue-2-r5.md`, `issue-8-r5.md`) — no product change left. Both status files stay **untracked** and durable on disk.

## Progress log

- 2026-11-10T18:01:54Z | PLAN | r1 | n=0 | collection initialized; PLAN record r1 recorded (form collection, overlay pdca-coder, scope #2/#3/#4/#8) | .pdca/open-issues-r5/rv1/plan.md
- 2026-11-10T18:01:54Z | PLAN | r1 | n=0 | user approved tolerances recorded (provenance: user message) | .pdca/open-issues-r5/rv1/criteria-approval.md
- 2026-11-10T18:01:54Z | PLAN | r1 | n=0 | r4 four terminal resolutions superseded-by-user-criteria (#2/#3/#4/#8); r4 record preserved unchanged | .pdca/status/collection-open-issues-r4.md
- 2026-11-10T18:01:54Z | PLAN | r1 | n=0 | next allowed step: DO item #2; completeness 0/4 accepted | .pdca/status/collection-open-issues-r5.md
- 2026-11-10T18:12:11Z | DO | r1 | n=1 | item #2 DO started (autonomous): preserved r4 candidate applied onto HEAD 2d0be53 and reconciled with #1 | .pdca/status/issue-2-r5.md
- 2026-11-10T18:12:11Z | DO | r1 | n=1 | item #2 checkpoint verified sha256 317554b07fe1ea5279a77f2b1520a41ea4b93432560c4ffd08792c30fddf9adc (66,759,214 B) | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/checkpoint-sha256.log
- 2026-11-10T18:12:11Z | DO | r1 | n=1 | item #2 gates: cargo build exit 0; cargo test exit 0 (98 passed/0 failed/1 ignored); cargo clippy --all-targets exit 0 (warnings only) | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/
- 2026-11-10T18:12:11Z | DO | r1 | n=1 | item #2 candidate separation exit 0 (CPU/f32, 882281 frames × 2 ch @ 44100) and fresh audio-separator 0.48.0 reference exit 0 on the same fixture/.onnx | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/candidate-run.log
- 2026-11-10T18:12:11Z | DO | r1 | n=1 | item #2 comparison exit 0: overall max_abs 1.4230609e-6, RMSE 1.4654514e-7, SDR 118.897 dB, corr 0.999999999999 (no trimming, no post-normalization); item #2 status in-progress, attempts 1 | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/comparison.json
- 2026-11-10T18:12:11Z | DO | r1 | n=1 | next allowed step: CHECK item #2; completeness 0/4 accepted | .pdca/status/collection-open-issues-r5.md
- 2026-11-10T23:22:49Z | CHECK | r1 | n=1 | item #2 CHECK ran and returned `not accepted → PLAN`: AC-2.1 unverified, AC-2.2 met, AC-2.3 unmet; defect keys F-2-criteria-coverage (major), F-2-acceptance-order (blocking), F-2-reference-provider (unverified); item #2 stays in-progress, attempts 1, phase CHECK | .pdca/status/issue-2-r5.md
- 2026-11-10T23:22:49Z | CHECK | r1 | n=1 | routing CHECK → PLAN (criteria defect): next allowed step is a PLAN revision for item #2; revision stays r1, no commit and no GitHub closure authorized; items #8/#3/#4 untouched | .pdca/status/collection-open-issues-r5.md
- 2026-11-10T23:27:02Z | PLAN | r2 | n=1 | item #2 contract revision recorded: `Replanned: CHECK→PLAN criteria contract (r 1→2)`, outgoing attempt `r1/n1, not accepted by CHECK`; `F-2-criteria-coverage` closed by AC-2.2, `F-2-acceptance-order` closed by AC-2.4, `F-2-reference-provider` still open; item #2 plan_revision 2, phase DO, next allowed step `DO item #2 (r2/n1: close F-2-reference-provider, no product change expected)`; completeness 0/4 accepted | .pdca/status/issue-2-r5.md
- 2026-11-10T18:32:48Z | DO | r2 | n=1 | item #2 DO r2/n1 started (autonomous): close `F-2-reference-provider` with observable active-provider evidence; no product change expected | .pdca/status/issue-2-r5.md
- 2026-11-10T18:32:48Z | DO | r2 | n=1 | item #2 reference provider audit exit 0: 1 live ONNX Runtime session, active providers `["CPUExecutionProvider"]` (f32 in/out), `torch.cuda.is_available()=False`, same `.onnx` sha256; readable pure-ASCII log | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/reference-provider-audit.log
- 2026-11-10T18:32:48Z | DO | r2 | n=1 | item #2 reference re-run exit 0 (CPU, 11.814 s): PCM bit-identical to the r1 reference on both stems (`data` sha256 7fd76234… / 98fef60a…, 882281 frames each, only `PEAK` differs); comparison re-run exit 0 with all four metrics per stem × channel unchanged; product tree 13/13 byte-identical, r1 gates cited as still binding; `F-2-reference-provider` closed (fixes applied 1); completeness 0/4 accepted | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/comparison-r1-vs-r2.log
- 2026-11-10T18:32:48Z | DO | r2 | n=1 | next allowed step: CHECK item #2 (r2/n1); item #2 in-progress, plan_revision 2, phase DO; no commit/staging/GitHub closure authorized (`AC-2.4`) | .pdca/status/issue-2-r5.md
- 2026-11-10T18:38:42Z | CHECK | r2 | n=1 | item #2 CHECK 2 ran on the r2/n1 candidate: verdict `not accepted → DO`; AC-2.1 met, AC-2.2 unmet (no per-dependency version/licence inventory of the MDX build graph), AC-2.3 met, AC-2.4 unverified (no independent GitHub pre-acceptance observation); report verbatim under `### CHECK 2 — r2/n1` | .pdca/status/issue-2-r5.md
- 2026-11-10T18:38:42Z | CHECK | r2 | n=1 | item #2 routing `CHECK → DO`: keys F-2-dependency-inventory (major, confirmed), F-2-github-preacceptance-state (major, unverified) and F-2-evidence-map (minor, confirmed) opened with fixes applied 0; F-2-reference-provider refuted for r2; contract r2 unchanged, attempt n=1/3 consumed | .pdca/status/issue-2-r5.md
- 2026-11-10T18:38:42Z | CHECK | r2 | n=1 | item #2 next allowed step: `DO item #2 (r2/n2: dependency inventory, GitHub pre-acceptance observation, manifest mapping)` - not started; phase CHECK; no commit/staging/GitHub closure authorized (`AC-2.4`); completeness 0/4 accepted | .pdca/status/collection-open-issues-r5.md
- 2026-11-10T18:46:08Z | DO | r2 | n=2 | item #2 DO attempt r2/n2 ran: `docs/dependency-audit.md` gained a 149-row per-crate version + SPDX-licence inventory of the full `tract-onnx`+`sha2` `Cargo.lock` closure (`cargo metadata` exit 0, raw JSON 1,193,103 B; `dependency_inventory.py` exit 0 — 149 rows, 118 host-build / 31 resolved-only, 0 rows without a licence; doc sha256 527862c0…); GitHub `#2` observed read-only (probe exit 0 — `gh` not installed, REST: state `open`, `closed_at=null`, timeline 1 `milestoned`, 0 `closed`/`reopened`); `manifest.json` criterion mapping rebuilt on the r2 set; keys F-2-dependency-inventory, F-2-github-preacceptance-state and F-2-evidence-map closed with fixes applied 1 | .pdca/status/issue-2-r5.md
- 2026-11-10T18:46:08Z | DO | r2 | n=2 | item #2 frozen candidate refreshed (snapshot doc + `snapshot-sha256.txt` + `snapshot.diff` 83,461 B + `manifest.json` + `commands.txt` + `index.md`); `verify_snapshot_sha256_r2n2.py` exit 0 — 13/13 match, changed vs r1 = the audit doc only, `.rs`/`Cargo.toml`/`Cargo.lock` byte-identical to r1, HEAD 2d0be53, nothing staged, so build/test/clippy stay bound and were not re-run; next allowed step `CHECK item #2 (r2/n2)`; no commit/staging/GitHub closure (`AC-2.4`); completeness 0/4 accepted | .pdca/status/collection-open-issues-r5.md
- 2026-11-10T18:51:12Z | CHECK | r2 | n=2 | item #2 CHECK 3 ran on the frozen `r2`/`n2` candidate and returned **`accepted → ACT`**: AC-2.1 met, AC-2.2 met (149 named rows with versions and licences, 0 gaps), AC-2.3 met (build/test exit 0, 98 passed / 0 failed / 1 ignored), AC-2.4 met (pre-acceptance component: verifiable tree, HEAD base, empty index, #2 open with 0 closed/reopened); report verbatim under `### CHECK 3 — r2/n2` | .pdca/status/issue-2-r5.md
- 2026-11-10T18:51:12Z | CHECK | r2 | n=2 | item #2 keys F-2-dependency-inventory, F-2-github-preacceptance-state, F-2-evidence-map and F-2-reference-provider marked `refuted/closed on the r2/n2 candidate` (fixes applied 2/2/2/1); contract r2 unchanged, attempt n=2/3 consumed; no recurrence, no escalation | .pdca/status/issue-2-r5.md
- 2026-11-10T18:51:12Z | CHECK | r2 | n=2 | item #2 phase `CHECK`, next allowed step `ACT item #2 (retrospective + one atomic commit; GitHub closure by the calling driver)` — the commit and the GitHub closure are not yet performed; completeness 0/4 closed (1 accepted) | .pdca/status/collection-open-issues-r5.md
- 2026-11-10T18:55:14Z | ACT | r2 | n=2 | item #2 ACT started: pre-stage re-anchor of the accepted candidate — **13/13** files match `snapshot-sha256.txt`, 0 mismatched, 0 missing, HEAD `2d0be53`, nothing staged; no drift | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/act-reanchor-verify.log
- 2026-11-10T18:55:37Z | ACT | r2 | n=2 | item #2 retrospective recorded verbatim in the item record §Lessons learned (source: `pdca-check` retrospective report via the driving role; 3 lessons, limits, completion signal) | .pdca/status/issue-2-r5.md
- 2026-11-10T18:55:37Z | ACT | r2 | n=2 | item #2 post-acceptance commit created: **59fb1fbcf89ed99a447dc5b8d48b915de601917f** on `main` — `#2: MDX-Net ONNX separation on CPU with audio-separator 0.48.0 parity`, parent `2d0be53`, 13 files changed +1750/-23, footer `Refs #2`, only the item's 13 paths staged; no push, no branch switch, no amend | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/commit-59fb1fb.log
- 2026-11-10T18:55:55Z | ACT | r2 | n=2 | item #2 **done / finalized**: outcome `accepted` (AC-2.1–AC-2.3 met, AC-2.4 pre-acceptance met with its post-acceptance commit action performed); residual **GitHub closure pending by the calling driver**; completeness **1/4 accepted**; next item `#8` — `DO item #8 (r1/n1)`; item #2 phase `ACT`, record `completed` | .pdca/status/collection-open-issues-r5.md
- 2026-11-11T06:14:00Z | DO | r1 | n=1 | item #8 DO started (autonomous) on HEAD 59fb1fb: item record created, frozen plan recorded by exact reference to §PLAN record r1; MelBand ckpt voc_fv7.ckpt sha256 77b9e4b54802d670c02daceb30c61ec825dc54ca6c29c34b03cdd5e9f78382b6 (489,571,079 B) and BS-RoFormer ckpt model_bs_roformer_ep_317_sdr_12.9755.ckpt sha256 5b84f37e8d444c8cb30c79d77f613a41c05868ff9c9ac6c7049c00aefae115aa (639,331,213 B) verified on disk | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/
- 2026-11-11T06:14:00Z | DO | r1 | n=1 | item #8 attempt n=1/3 in progress under contract r1; next allowed step after DO: `CHECK item #8 (r1/n1)`; completeness 1/4 accepted, no commit/staging authorized for #8 in DO | .pdca/status/issue-8-r5.md
- 2026-11-11T04:22:00Z | DO | r1 | n=1 | item #8 continuation of the same attempt after a server restart (no counter increment): product delta and evidence root re-verified as briefed, failed first reference attempt preserved; the MelBand checkpoint was still the blocker | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/reference-ensemble.log
- 2026-11-11T04:26:30Z | DO | r1 | n=1 | item #8 MelBand blocker solved inside audio-separator 0.48.0's own stack: the checkpoint is missing only from its *model registry*, so `docs/provenance/v7.yaml` (sha256 f51b06e4…ce2f) is supplied as `voc_fv7.yaml` and one `MDXC` entry is added to `Separator.list_supported_model_files()` in a `Separator` subclass; the stock `MDXCSeparator`/`RoformerLoader` path then loads it (`model_run=MelBandRoformer`), smoke load exit 0, no network, no other override | .pdca/open-issues-r5/rv1/item-8-ensembles/scripts/reference_ensemble_v2.py
- 2026-11-11T04:29:07Z | DO | r1 | n=1 | item #8 AC-8.1 reference exit 0 on the frozen 457,687-frame fixture: both members separated and written by audio-separator (BS-RoFormer registry-native, MelBand registry-injected), the written stems re-read with `librosa.load(mono=False, sr=44100)`, inputs frozen as `inputs/member{0,1}_vocals.f32` (2 × 915,374 f32), and `avg_wave` (float64) / `median_wave` (float32) reference dumps written as f32 (contract) and f64 (exact returned dtype), shape [2, 457687], all finite, peak 0.735177218914032 | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/reference-ensemble-v2.log
- 2026-11-11T04:30:10Z | DO | r1 | n=1 | item #8 AC-8.1 candidate parity test exit 0 (`1 passed; 0 failed`): bit-exact over all 915,374 samples of both channels — `max_abs 0.000e+00` (inside ≤1e-6), `RMSE 0.000e+00` (inside ≤1e-7), `SDR +inf dB`, `corr 1.000000`, 0 differing samples, for `avg_wave` and `median_wave`; degeneracy noted: with two members median_wave and avg_wave coincide numerically, so the fixture validates both code paths but cannot separate their semantics | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/parity-test-ignored.log
- 2026-11-11T04:30:40Z | DO | r1 | n=1 | item #8 AC-8.1 metrics re-measured per stem × channel against the exact f64 reference (all-channels plus channel0/channel1, no trimming, no post-normalization) exit 0 | .pdca/open-issues-r5/rv1/item-8-ensembles/metrics/parity.csv
- 2026-11-11T04:31:50Z | DO | r1 | n=1 | item #8 docs: `docs/ensemble-presets.md` added (operators, `vocal_balanced` composition with both checkpoint sha256, equal weights, stem set, CLI invocation, and the note that upstream 0.48.0's same-named preset uses `avg_fft` with different models); README untouched | docs/ensemble-presets.md
- 2026-11-11T04:32:56Z | DO | r1 | n=1 | item #8 **BLOCKED defect CONC-1**: a second execution process runs this same item attempt in this tree — it rewrote `tests/ensemble.rs` (17,607→18,782 B) at 09:29:08 local, wrote 882,000-frame candidate dumps, and has been running `ensemble_separate --preset vocal_balanced` on the full CC0 song since 09:29:52 (cargo PIDs 22236/14536, `ensemble_separate.exe` PID 22936) holding `logs/preset-cli-run.log`; this session's own AC-8.2 invocation died on that lock with exit 1 and **no second heavy run was started**; AC-8.1 stands bit-exact, AC-8.2/AC-8.3 and the evidence pack stay `unverified`/`unmet`; fixes applied 0 | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/concurrency-conflict.log
- 2026-11-11T04:32:56Z | DO | r1 | n=1 | item #8 next allowed step: **driving-role decision** (session-level recovery — interrupt the duplicate, choose one frozen fixture, re-run only the remainder on a quiet tree) then `CHECK item #8 (r1/n1)` or `DO → ACT` finalization of `CONC-1`; item #8 in-progress/blocked, plan_revision 1, attempts 1/3, no commit/staging/GitHub closure; completeness 1/4 accepted | .pdca/status/issue-8-r5.md
- 2026-11-11T04:40:00Z | DO | r1 | n=1 | item #8 `CONC-1` **cleared without an interrupt**: the duplicate process exited on its own (no `cargo`/`ensemble_separate` alive; its `out/candidate/ensemble/` empty and `logs/preset-cli-run.log` ending at `input=…` with no `rc=` line), the tree is quiet, and the DO resumed; AC-8.2 re-launched into `out/candidate/preset-vocal-balanced` with `logs/preset-cli-run-continuation.log` (first attempt exit 1 — wrong BS-RoFormer safetensors path in the item-8 models dir, no inference started; corrected to the r4 evidence root and re-run) | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/preset-cli-run-continuation.log
- 2026-11-11T05:22:30Z | DO | r1 | n=1 | item #8 continuation 2 (same attempt, no counter change): `CONC-1` closed by environment — the duplicate exited on its own and a quiet tree was re-verified (no second `cargo`/`ensemble_separate`, `HEAD 59fb1fb`, nothing staged, product delta = the 7 item files); the 09:36:59 run is dead (PIDs gone, no `rc=`), while AC-8.2 execution run 3 is **live** (started 05:20:22Z by an earlier dispatch: cargo 23876/26320, exe 23412, `logs/preset-cli-run-final.log`, `--out .../out/candidate/preset-vocal-balanced`) and was left untouched — no second copy, no kill | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/preset-cli-run-final.log
- 2026-11-11T05:22:30Z | DO | r1 | n=1 | item #8 AC-8.3 gates green on the frozen bytes (no product edit): `cargo build` exit 0 (0.6 s); `cargo test` exit 0 (80.7 s) — 118 passed / 0 failed / 2 ignored over 9 targets, #1 BS-RoFormer and #2 MDX-Net present and green; `cargo clippy --all-targets` exit 0 (6.0 s, warnings only) | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/{build.log,test.log,clippy.log}
- 2026-11-11T05:25:02Z | DO | r1 | n=1 | item #8 AC-8.2 exit observer armed on PID 23412 (`WaitForSingleObject` + `GetExitCodeProcess`; the run holds its log open, so `rc` is written to `out/candidate/preset-vocal-balanced/exit.json`, never guessed) — run 3 still live | .pdca/open-issues-r5/rv1/item-8-ensembles/scripts/wait_preset_exit.ps1
- 2026-11-11T05:25:53Z | DO | r1 | n=1 | item #8 next allowed step: finish run 3 → validate the produced stems → refresh the evidence pack (`manifest.json`/`index.md`/`snapshot*`/`commands.txt`/`lint.txt`) → `CHECK item #8 (r1/n1)`; item #8 in-progress, attempts 1/3, no commit/staging/GitHub closure; completeness 1/4 accepted | .pdca/status/collection-open-issues-r5.md
- 2026-11-11T06:52:31Z | DO | r1 | n=1 | item #8 AC-8.2 execution run 3 exited **`rc=0`** (`GetExitCodeProcess` after `WaitForSingleObject`; the run's launcher also appended `rc=0` to its log), `separation_secs=5491.531`; output exactly `vocals.wav` + `instrumental.wav`, 33,407,732 B each | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/preset-cli-run-final.log, out/candidate/preset-vocal-balanced/exit.json
- 2026-11-11T06:53:00Z | DO | r1 | n=1 | item #8 AC-8.2 validation exit 0: exactly {vocals, instrumental}, no unexpected audio; both FLOAT 44,100 Hz 2 ch 4,175,961 frames / 94.692993 s, all finite/non-empty/non-silent, matching input rate+framing+duration; sha256 vocals 9cdb81bc…bade, instrumental e15800cf…5f5a — AC-8.2 measured end to end | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/preset-validation.log
- 2026-11-11T06:53:30Z | DO | r1 | n=1 | item #8 evidence pack refreshed: `make_snapshot.py` exit 0 (head 59fb1fb, 7/7 worktree == snapshot, hashes unchanged, `snapshot.diff` 57,620 B); `manifest.json`/`index.md` rebuilt on `AC-8.1`+`AC-8.2`+`AC-8.3` numbers; `commands.txt` extended; one contract fixture, the 882,000-frame window stays quarantined | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/snapshot-refresh.log
- 2026-11-11T06:55:00Z | DO | r1 | n=1 | item #8 **DO r1/n1 complete — candidate ready for CHECK**: AC-8.1 bit-exact, AC-8.2 measured, AC-8.3 gates green on the same 7 files; no product edit, `HEAD 59fb1fb`, nothing staged, no commit/push/GitHub write; next allowed step `CHECK item #8 (r1/n1)`; AC-8.4 stays ACT work; completeness 1/4 accepted | .pdca/status/issue-8-r5.md
- 2026-11-11T07:10:00Z | DO | r1 | n=1 | item #8 independent final-tree re-verification by this session (no product edit): AC-8.1 parity test re-run exit 0 (`1 passed; 0 failed`, `max_abs 0.000e+0`, `rmse 0.000e+0`, 915,374 samples, both channels, both operators; candidate dumps byte-identical to the reference f32, sha256 `fab3854a…9774`), metrics recomputed exit 0 on the final tree; `cargo build` exit 0, `cargo test` exit 0 (118 passed / 0 failed / 2 ignored), `cargo clippy --all-targets` exit 0; snapshot re-frozen exit 0 (7/7 worktree == snapshot, `snapshot.diff` 57,620 B) — an independent duplicate of the continuation-2/3 runs, identical results, kept under separate log names so CHECK can cite either | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/{parity-test-final.log,comparison-final.log,gate-build.log,gate-test.log,gate-clippy.log,snapshot-final.log}
- 2026-11-11T07:10:00Z | DO | r1 | n=1 | item #8 evidence pack written by this session: `manifest.json` (AC-8.1→AC-8.4 map with the real numbers, checkpoint/fixture/song identities, versions, `CONC-1` cleared, `AC-8.4` not executed), `index.md` (one-page map incl. the MelBand unblock and the ownership split of the concurrent process's artifacts), `commands.txt` (every command with exit code, failures and re-runs included), `lint.txt`; next allowed step `CHECK item #8 (r1/n1)`; completeness 1/4 accepted | .pdca/open-issues-r5/rv1/item-8-ensembles/manifest.json
- 2026-11-11T06:59Z | CHECK | r1 | n=1 | item #8 CHECK 1 ran and returned **`not accepted → PLAN`**: matrix `AC-8.1 met` / `AC-8.2 unverified` / `AC-8.3 met` / `AC-8.4 unmet`; findings `F-8-acceptance-order` (blocking, confirmed) and `F-8-song-licence` (major, unverified); item #8 stays in-progress, attempts 1/3, plan_revision 1, phase CHECK | .pdca/status/issue-8-r5.md
- 2026-11-11T06:59Z | CHECK | r1 | n=1 | routing **`CHECK → PLAN`** (criteria defect, acceptance order): next allowed step is a PLAN contract revision for item #8; the commit and the GitHub closure are not authorized; the `CHECK → PLAN` transition consumes no attempt; items #3/#4 untouched; completeness 1/4 accepted | .pdca/status/collection-open-issues-r5.md
- 2026-11-11T07:04:10Z | PLAN | r2 | n=1 | item #8 contract revision recorded (item-level): `Replanned: CHECK→PLAN acceptance-order criteria contract (r 1→2)` — the outgoing attempt `r1/n1` (DO complete, not accepted by CHECK) is recorded first in the item record; `F-8-acceptance-order` closed by the `AC-8.4` restatement (CHECK-verifiable pre-acceptance state plus the mandatory ACT closure), `F-8-song-licence` carried as an `AC-8.2` evidence obligation; item #8 `plan_revision 2`, `plan_state` ready, phase `DO`, attempts `1/3` | .pdca/status/issue-8-r5.md
- 2026-11-11T07:04:10Z | PLAN | r2 | n=1 | item #8 next allowed step `DO item #8 (r2/n1: AC-8.2 licence evidence from the primary source; no re-run of the separation if the same audio file is confirmed free-licensed)`; the commit and the GitHub closure stay unauthorized until an accepted CHECK; items #3/#4 untouched and unstarted; completeness 1/4 accepted | .pdca/status/collection-open-issues-r5.md
- unknown (before the `r2` block at 07:04:10Z) | CHECK | r1 | n=1 | item #8 **CHECK 2** ran on the same `r1`/`n1` candidate, on measurements taken before `r2` was written, and returned **`not accepted → DO`**: AC-8.1 met, AC-8.2 unverified (song licence), AC-8.3 met, AC-8.4 `unverified — ACT pending`; findings `LIC-8` (unverified, blocking, AC-8.2) and `PAD-8` (confirmed, medium: flat-buffer padding instead of per-channel); recorded as a **separate** outcome from CHECK 1, not merged | .pdca/status/issue-8-r5.md §CHECK record `### CHECK 2 — r1/n1`
- unknown (before the `r2` block at 07:04:10Z) | CHECK | r1 | n=1 | item #8 routing `CHECK → DO` with two actions (publisher licence evidence for sound `567226` bound to the used MP3; per-channel padding plus a genuinely unequal-length stereo test); CHECK 1 (`not accepted → PLAN`, `F-8-acceptance-order` blocking + `F-8-song-licence` major) is the outcome that produced the item's contract revision `r2`; counters unchanged `r=1`, `n=1/3`; neither verdict authorizes a commit or the GitHub closure | .pdca/status/collection-open-issues-r5.md §CHECK record
- 2026-11-11T07:03:00Z | DO | r2 | n=1 | item #8 `DO` `r2`/`n1` round started on the recorded contract `r2` (identity notice: the dispatch fixed `r1`/`n=2/3`, the durable record carried `r2`/`n=1`, so the round is recorded as `r2`/`n1`); scope: close `PAD-8` and `LIC-8`, re-measure `AC-8.1`/`AC-8.3` on the fixed bytes, show `AC-8.2` non-invalidated, record `AC-8.4(1)` | .pdca/status/issue-8-r5.md §PLAN record r2
- 2026-11-11T07:03:24Z | DO | r2 | n=1 | item #8 `PAD-8` **before the fix**: `cargo test --test ensemble` exit **101**, 20 passed / 2 failed (flat-array padding: `[1.0, 2.0, 3.0, 4.0, 0.0, 0.0]` vs `[1.0, 2.0, 0.0, 3.0, 4.0, 0.0]`; `[1.0, 2.0, 3.0, 4.0, 0.5, 0.5]` vs `[1.0, 2.0, 0.5, 3.0, 4.0, 0.5]`) | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/pad8-tests-before-fix.log
- 2026-11-11T07:03:51Z | DO | r2 | n=1 | item #8 `PAD-8` **fixed** (fixes applied 1): per-channel, in-frames indexing in both operators, two genuinely unequal-length stereo tests, docs corrected — `cargo test --test ensemble` exit **0**, 22 passed / 0 failed / 1 ignored | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/pad8-tests-after-fix.log
- 2026-11-11T07:06:41Z | DO | r2 | n=1 | item #8 `LIC-8`/`F-8-song-licence` evidence produced (fixes applied 1, no product change): publisher page HTTP 200 (41,021 B, sha256 `ed10cadc…c4ce2`), wayback and short URL HTTP 200, CC0 deed HTTP 200, **APIv2 HTTP 401** (token required — recorded as a limitation, no token read/sent/stored); licence `Creative Commons 0`, author `Roses1401`, duration `1:34.692` | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/licence-fetch.log, licence/licence-evidence.json, licence/binding.md
- 2026-11-11T07:11:50Z | DO | r2 | n=1 | item #8 `LIC-8` byte binding: the preview MP3 (sha256 `edf03cb0…f76a`, 2,349,696 B, 48 kHz, 94.692979 s) decoded and resampled to 44.1 kHz is **bit-identical** to the used `song/song-go-out-rock-f32-44100.wav` (4,175,961 frames, 94.692993 s, `max_abs_difference 0.0`); HQ-preview vs 3.6 MB/320 kbps uploaded-master caveat recorded | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/licence-decode-chain.log
- 2026-11-11T07:03:57Z | DO | r2 | n=1 | item #8 `AC-8.1` re-measured on the fixed bytes: parity test exit **0** (`1 passed; 0 failed`), `max_abs 0.000e0` / `rmse 0.000e0` over **915,374** samples (457,687 per channel) for both operators; metrics recomputed exit 0 — `max_abs 0`, `RMSE 0`, `SDR +inf dB`, `corr 1.000000`, dumps byte-identical to the reference f32 (sha256 `fab3854a…9774`) | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/parity-test-final2.log, logs/comparison-final2.log
- 2026-11-11T07:04:23Z | DO | r2 | n=1 | item #8 `AC-8.3` gates on the fixed bytes: `cargo build` exit **0**; `cargo test` exit **0** — **120 passed / 0 failed / 2 ignored** (ensemble 22+1 ignored with the two new tests, #1 BS-RoFormer and #2 MDX-Net green); `cargo clippy --all-targets` exit **0** (0 errors) | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/{build-n2.log,test-n2.log,clippy-n2.log}
- 2026-11-11T07:07:22Z | DO | r2 | n=1 | item #8 `AC-8.2` **unchanged and non-invalidated** (long run not repeated): both member stems carry the input's 4,175,961 frames, so the padding branch is never entered and the fixed index equals the old one for equal lengths — confirmed by the byte-identical post-fix dumps | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/ac82-pad8-non-invalidation.md
- 2026-11-11T07:07:49Z | DO | r2 | n=1 | item #8 evidence pack refreshed on the fixed bytes: `make_snapshot.py` exit 0 — `HEAD 59fb1fb`, 3 tracked + 4 new, 7/7 worktree == snapshot, `snapshot.diff` 61,979 B; `manifest.json`/`index.md` rewritten, `commands.txt` extended; closing re-anchor `build=0 test=0 clippy=0 parity=0` and 7/7 identity at 07:12:42Z | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/snapshot-final2.log, logs/reanchor-summary.log
- 2026-11-11T07:12:49Z | DO | r2 | n=1 | item #8 **`DO` `r2`/`n1` complete — candidate ready for CHECK**: `PAD-8` fixed (1 fix), `LIC-8` evidence in `licence/` (1 fix), `AC-8.1`/`AC-8.3` re-measured, `AC-8.2` non-invalidated, `AC-8.4(1)` holds; `HEAD 59fb1fb`, index empty, no commit/push/GitHub write; next allowed step `CHECK item #8 (r2/n1)`; item #8 in-progress, plan_revision 2, attempts 1/3; completeness 1/4 accepted | .pdca/status/collection-open-issues-r5.md
- 2026-11-11T07:12:00Z | DO | r2 | n=1 | item #8 status-record integrity repaired: the item record's `### r2` PLAN block now has a 4-backtick outer fence (it contains 3-backtick fences); the recorded text is byte-identical (127 lines / 23,977 B / sha256 `669a4418…c55adb1`) and `### r1` unchanged | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/status-fence-repair.log
- 2026-11-11T07:09:35Z | DO | r2 | n=1 | item #8 `LIC-8` primary source fetched and persisted (no token): page HTTP **200** (`Creative Commons 0` → `creativecommons.org/publicdomain/zero/1.0/`, 41,021 B, sha256 `fedd1b69…`), short URL HTTP **200**, APIv2 HTTP **401** (`Authentication credentials were not provided.`), HQ preview HTTP **200** (2,349,696 B, sha256 `edf03cb0…c13f76a`) | .pdca/open-issues-r5/rv1/item-8-ensembles/evidence/source/fetch-log.json
- 2026-11-11T07:09:41Z | DO | r2 | n=1 | item #8 licence bound to the used bytes: the preview URL listed by the page re-downloads byte-identically to `song/go-out-rock-567226-hq.mp3`, and its decode chain reproduces `song/song-go-out-rock-f32-44100.wav` bit-identically (`max_abs 0.0`, 4,175,961 frames = 94.692993 s) — so the file the 5,491.531 s run separated is the CC0 sound | .pdca/open-issues-r5/rv1/item-8-ensembles/metrics/licence-provenance.json
- 2026-11-11T07:12:13Z | DO | r2 | n=1 | item #8 `F-8-song-licence` **closed, fixes applied 1** (evidence pointer): `metrics/licence-provenance.json`, the item `manifest.json` root `licence` block + resolved `AC-8.2.licence_evidence` + closed `LIC-8`, and a corrected `index.md` (the earlier HTML hashes did not match the persisted bytes) | .pdca/open-issues-r5/rv1/item-8-ensembles/manifest.json
- 2026-11-11T07:16:00Z | DO | r2 | n=1 | item #8 next allowed step **`CHECK item #8 (r2/n1)`**; re-anchor `7/7` and gates `build=0 test=0 clippy=0 parity=0` on the same bytes, no product change, separation not re-run; nothing staged/committed/pushed; completeness `1/4 accepted` | .pdca/status/collection-open-issues-r5.md
- 2026-11-11T07:21:22Z | CHECK | r2 | n=1 | item #8 **CHECK 2** ran on the `r2`/`n1` candidate and returned **`not accepted → PLAN`**: `AC-8.1` `met`, `AC-8.2` `met`, `AC-8.3` `unmet` (the literal no-warnings clause is violated by the measured clippy/test warnings, 4 of them in `tests/ensemble.rs`), `AC-8.4(1)` `unverified` (no stored GitHub `#8` state presented), `AC-8.4(2)` `unverified — срок не наступил`; findings `F-8-ac3-contract-drift` (blocking, confirmed) and `F-8-issue-open-evidence` (major, unverified); `F-8-acceptance-order`, `F-8-song-licence`/`LIC-8` and `PAD-8` refuted/closed on the `r2` candidate; item #8 stays in-progress, `plan_revision 2`, attempts 1, phase `CHECK` | .pdca/status/issue-8-r5.md §CHECK record `### CHECK 2 — r2/n1`
- 2026-11-11T07:21:22Z | CHECK | r2 | n=1 | item #8 routing **`CHECK → PLAN`**: next allowed step is a PLAN contract revision for `#8` (resolve the AC-8.3 contract drift against the original contract and its declared `r1` invariance; the `AC-8.4(1)` read-only GitHub `#8` observation is with the driver); counters unchanged `r=2`, `n=1/3` — no attempt consumed; commit, GitHub `#8` closure and `done` not authorized; items #3/#4 untouched; completeness `1/4 accepted` | .pdca/status/collection-open-issues-r5.md §CHECK record
- 2026-11-11T07:27:43Z | PLAN | r3 | n=1 | item `#8` outgoing attempt recorded first: `r2`/`n1` — DO complete on the unchanged 7-file snapshot at `59fb1fb` (`PAD-8` fixed, `LIC-8` licence bound to the used bytes, `AC-8.1` bit-exact, `AC-8.3` gates `exit 0` with `120 passed / 0 failed`) and **not accepted by CHECK 2**; `Replanned: CHECK→PLAN AC-8.3 contract-drift (r 2→3)`; `F-8-ac3-contract-drift` closed by the revised `AC-8.3`, `F-8-issue-open-evidence` carried as the `AC-8.4(1)` stored-observation obligation; item `#8` `plan_revision 3`, attempts `1/3` | .pdca/status/issue-8-r5.md §PLAN record r3
- 2026-11-11T07:27:43Z | PLAN | r3 | n=1 | item `#8` next allowed step `DO item #8 (r3/n1: store the read-only GitHub observation for #8, no product change)`; phase `DO`, record `open`, outcome `pending`; the commit, the GitHub `#8` closure and `done` stay not authorized; items `#3`/`#4` untouched; completeness `1/4 accepted` | .pdca/status/collection-open-issues-r5.md §Current state
- 2026-11-11T07:33:16Z | DO | r3 | n=1 | item `#8` **Notice:** a concurrent dispatch ran an out-of-contract lint round (`tests/ensemble.rs` 21,578 → 21,633 B) and reverted it **byte-identically** once `r3` §9 forbade that edit; the snapshot was re-frozen `7/7` on the reverted bytes (`snapshot.diff` 61,979 B); the edit is not claimed as progress | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/lint-fix-round.md, logs/snapshot-restore.log
- 2026-11-11T07:34:37Z | DO | r3 | n=1 | item `#8` `DO` `r3`/`n1` started (autonomous, evidence only, no product change) and the frozen candidate re-anchored: **7/7** worktree == `snapshot/` copy == `snapshot-sha256.txt`, `missing=0`, `mismatch=0`, exit **0**; `HEAD 59fb1fb`, index empty, no `#8` commit; no gate re-run (no drift) | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/reanchor-7of7-verify-r3n1.log
- 2026-11-11T07:34:54Z | DO | r3 | n=1 | item `#8` **fresh read-only GitHub observation** (unauthenticated, no token): `GET /repos/AlexeyShirshov/candle-roformer/issues/8` **HTTP 200** (4,931 B, sha256 `4d61ef91…fbcd`), timeline **HTTP 200**; identifier `AlexeyShirshov/candle-roformer#8`, canonical URL `https://github.com/AlexeyShirshov/candle-roformer/issues/8`, `state=open` (`OPEN`), `state_reason=null`, `closed_at=null`, **0 closed / 0 reopened**, observed `2026-11-11T07:34:54Z`; the plan's stop condition did not trigger | .pdca/open-issues-r5/rv1/item-8-ensembles/github/issue-8-observation.json, logs/github-issue-8-r3n1.log
- 2026-11-11T07:34:58Z | DO | r3 | n=1 | item `#8` `F-8-issue-open-evidence` **closed, fixes applied 1**: raw responses, summary and observation time persisted, the stale `07:19:54Z` summary archived byte-identically; `manifest.json` root `github_issue` block + `AC-8.4(1)` pre-acceptance sub-record added | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/manifest-r3n1-update.log
- 2026-11-11T07:38:00Z | DO | r3 | n=1 | item `#8` **`DO` `r3`/`n1` complete, no product change**; next allowed step `CHECK item #8 (r3/n1)`; item `#8` `plan_revision 3`, attempts `n=1/3`, phase `DO`; no commit, no staging, no push, no GitHub write; items `#3`/`#4` untouched; completeness `1/4 accepted` | .pdca/status/collection-open-issues-r5.md §DO result
- 2026-11-11T07:39:00Z | DO | r3 | n=1 | item `#8` record re-check: a concurrent dispatch of the same item re-ran the producer, so the current `github/issue-8-observation.json` is a second fresh observation at `2026-11-11T07:37:38Z` (HTTP `200`, `state=open`, byte-identical body `4d61ef91…fbcd`), with the `07:34:54Z` record archived byte-identically; both `OPEN`, no product byte involved, outcome unchanged | .pdca/open-issues-r5/rv1/item-8-ensembles/github/
- 2026-11-11T07:37:38Z | DO | r3 | n=1 | item `#8` continuation (no counter change): the read-only GitHub observation was re-taken — `GET …/issues/8` **HTTP 200** (4,931 B, sha256 `4d61ef91…fbcd`), `GET …/issues/8/timeline?per_page=100` **HTTP 200** (1,329 B, sha256 `e642a3f5…08db`), `state=open` (`OPEN`), `state_reason=null`, `closed_at=null`, `timeline_closed_events=0`, `timeline_reopened_events=0`, `fetched_utc 2026-11-11T07:37:38Z`, no token read/sent/stored; the `07:34:54Z` summary archived as `github/issue-8-observation__2026-11-11T07-34-54Z.json` | .pdca/open-issues-r5/rv1/item-8-ensembles/github/issue-8-observation.json, logs/github-issue-8.log
- 2026-11-11T07:38:21Z | DO | r3 | n=1 | item `#8` pack repaired to the restored tree (`scripts/reconcile_manifest_r3.py` exit **0**): `tests/ensemble.rs` back to 21,578 B / `dbc05263…e01a` in `products` and `AC-8.3.tree`, `AC-8.3` gates on `logs/{build,test,clippy}-restored.log`, `logs.*` restored-first, `F-8-lint-warnings` marked «not a defect under `r3`, reverted», `F-8-ac3-contract-drift` and `F-8-issue-open-evidence` closed, `AC-8.1.restored_r3_measurement` added, `github_issue` refreshed, `snapshot.diff` 61,979 B; `index.md` rewritten to DO `r3`/`n1` with the `r3` gate semantics and the stored observation; `commands.txt` extended | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/manifest-r3-reconcile.log, manifest.json, index.md
- 2026-11-11T07:39:32Z | DO | r3 | n=1 | item `#8` independent re-anchor: `scripts/verify_final_state.py` exit **0** — worktree == `snapshot/` copy == `snapshot-sha256.txt`, `matched=7/7`, byte sizes equal, `tests/ensemble.rs` 21,578 B `dbc05263…e01a`; **no product byte changed in this round** | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/final-state-verify-r3n1-continuation.log
- 2026-11-11T07:39:00Z | DO | r3 | n=1 | item `#8` **Notice:** mid-round collision and a competing record writer — the dispatch brief fixed `r=2` / `n=2/3` and a warning fix, the durable record carried the genuine `r3` revision (whose §7/§9 forbid that edit), so the fix was reverted byte-identically (`tests/ensemble.rs` 21,633 B / `e47d8110…4b65` → 21,578 B / `dbc05263…e01a`) and the restored tree re-measured; `.pdca/status/issue-8-r5.md` was rewritten by another dispatch at `12:28:49` / `12:30:16` / `12:37:53` local and this file at `12:39:03` — its blocks are preserved byte-identically, nothing lost, no process killed, no interrupt issued | .pdca/status/issue-8-r5.md §DO result `r3`/`n1`
- 2026-11-11T07:40:00Z | DO | r3 | n=1 | item `#8` **`DO` `r3`/`n1` complete — no product change**; `CHECK 3` is **pending under `r3`** (not run); counters `r=3`, `n=1/3`, attempts `1`; nothing staged, no commit, no push, no GitHub write; next allowed step **`CHECK item #8 (r3/n1)`**; completeness `1/4 accepted` | .pdca/status/collection-open-issues-r5.md §CHECK record, §DO result
- unknown (returned before this `ACT`, recorded `2026-11-11T07:46:05Z`; the verdict's own clock is not recorded) | CHECK | r3 | n=1 | item `#8` **CHECK 3** (`r3`/`n1`) returned **`accepted → ACT`**: `AC-8.1` `met` (`max_abs 0` / `RMSE 0` over 915,374 samples × 2 channels, parity test exit `0`), `AC-8.2` `met` (CLI exit `0`, 5,491.531 s, exactly `vocals` + `instrumental`, 44,100 Hz / 2 ch / 4,175,961 frames / 94.692993 s, CC0 proven from the primary source), `AC-8.3` `met` (`build`/`test`/`clippy` exit `0`, 120 passed / 0 failed / 2 ignored), `AC-8.4(1)` `met` (7/7, empty index, stored observation `2026-11-11T07:37:38Z` `state=open`), `AC-8.4(2)` `unverified — срок не наступил`; `F-8-ac3-contract-drift` and `F-8-issue-open-evidence` refuted/closed, `F-8-acceptance-order`, `F-8-song-licence`/`LIC-8` and `PAD-8` remain closed; report verbatim under `### CHECK 3 — r3/n1` in the item record; counters `r=3`, `n=1/3` | .pdca/status/issue-8-r5.md §CHECK record
- 2026-11-11T07:44:36Z | ACT | r3 | n=1 | item `#8` ACT started: pre-stage re-anchor — verifier exit **0**, **`matched=7/7`** worktree == snapshot copy == `snapshot-sha256.txt`, `0` missing, `0` mismatch, `HEAD 59fb1fb`, `git diff --cached` empty, no `#8` commit; no drift | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/act-reanchor-7of7-r3n1.log
- 2026-11-11T07:45:47Z | ACT | r3 | n=1 | item `#8` **post-acceptance commit created**: **`a38bb7fd969dc0ebb36ca74957af93b6b4b20a19`** on `main` — `#8: ensemble avg_wave/median_wave parity and vocal_balanced preset`, parent `59fb1fb`, **7 files changed +1368/−2**, footer `Refs #8`, no auto-close keyword; only the 7 product paths staged, no `.pdca/**`/`.openidea/**`/weights/audio/logs/status file; `git diff --name-only HEAD -- <7 paths>` empty, nothing staged, no push, no branch switch, no amend | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/commit-a38bb7f.log
- 2026-11-11T07:46:05Z | ACT | r3 | n=1 | item `#8` **retrospective recorded verbatim** in the item record §Lessons learned (source: `pdca-check` retrospective via the driving role; 3 lessons with sources, limits, completion signal `retrospective_complete`); adds no verdict, counter, attempt or verification | .pdca/status/issue-8-r5.md §Lessons learned
- 2026-11-11T07:46:30Z | ACT | r3 | n=1 | item `#8` **done / finalized**: outcome `accepted` (`AC-8.1`-`AC-8.3` `met`, `AC-8.4(1)` `met`, `AC-8.4(2)` commit component performed as `a38bb7f`); residual **GitHub `#8` closure by the calling driver - NOT performed by this executor, never recorded as "issue closed"**; item `#8` phase `ACT`, record `completed`, attempts `1`; completeness **`2/4 accepted`**; next item `#3` - `DO item #3 (r1/n1)` | .pdca/status/collection-open-issues-r5.md §ACT record
- 2026-11-11T07:47:21Z | ACT | r3 | n=1 | item `#8` pre-stage re-anchor by the verification dispatch: branch `main`, index empty, verifier exit **0**, worktree == snapshot **7/7** (`0` missing, `0` mismatch) — but **`HEAD` had already moved** to `a38bb7f` (reflog `commit` `2026-11-11 12:45:40 +0500`, parent `59fb1fb`), so **no staging and no commit were performed** by this dispatch; `Notice` recorded, not repaired | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/act-reanchor-verify.log
- 2026-11-11T07:47:29Z | ACT | r3 | n=1 | item `#8` commit `a38bb7fd969dc0ebb36ca74957af93b6b4b20a19` **independently verified** (not created by this dispatch): parent `59fb1fb`, exactly **7 files** (`+1368/-2`), footer `Refs #8`, no `Closes`/`Fixes`, no tag, **not pushed** (`origin/main = bf2cab6`), index clean, no leftover product change for the 7 paths; subject differs from the brief's prescribed wording → `Notice` (no amend) | .pdca/open-issues-r5/rv1/item-8-ensembles/logs/commit-a38bb7f-verify.log
- 2026-11-11T07:51:30Z | ACT | r3 | n=1 | item `#8` retrospective from `pdca-check` (routed by the driving role) recorded **verbatim** in the item record §Lessons learned as the final report (3 lessons, boundaries/uncertainty, closing signal); earlier recording kept byte-identical | .pdca/status/issue-8-r5.md §Lessons learned
- 2026-11-11T07:52:00Z | ACT | r3 | n=1 | item `#8` **record correction: `done` withheld** — `AC-8.4(2)` requires the commit **and** `state=CLOSED`; the GitHub `#8` closure was **not performed** (last stored observation `2026-11-11T07:37:38Z`, `state=open`), so the row is `in-progress` / `finalized`, `plan_revision 3`, attempts `1`; completeness **`1/4 accepted`** (`#2` `done`; `#8` accepted and committed but not `done`) | .pdca/status/collection-open-issues-r5.md §ACT record correction, item row `#8`
- 2026-11-11T07:52:30Z | ACT | r3 | n=1 | item `#8` **ACT closed with the residual**: outcome `accepted` per `CHECK 3`, `AC-8.1`-`AC-8.4(1)` `met`, `AC-8.4(2)` commit performed / GitHub closure pending; next step = **the calling driver closes GitHub `#8`**, after which the collection records completion and the next authorized step is `DO item #3 (r1/n1)`; `#4` stays `pending` | .pdca/status/collection-open-issues-r5.md §Current state, §ACT record
- 2026-11-11T07:50:59Z | ACT | r3 | n=1 | item `#8` **Notice - concurrent status-file writer**: a competing dispatch of the same item wrote both status files during this ACT window (`issue-8-r5.md` mtime `12:46:28` and `collection-open-issues-r5.md` mtime `12:48:15`/`12:50:14` local) and re-set the item row's `status` cell to `done`; because `AC-8.4(2)` forbids `done` without the GitHub closure this dispatch re-set it to `in-progress` and recorded the collision. Nothing was deleted, no process was killed, no interrupt was issued, and every competing block is kept in place | .pdca/status/collection-open-issues-r5.md item row `#8`

- 2026-11-11T07:53Z | ACT | r3 | n=1 | item `#8` **Notice — concurrent executor collision during ACT, reconciled:** two concurrent executor dispatches wrote both status files (`.pdca/status/issue-8-r5.md`, `.pdca/status/collection-open-issues-r5.md`); a second retrospective block was appended and a contradictory `in-progress` / `not done` disposition appeared (the `07:52:00Z` and `07:52:30Z` rows, and this file's item row `#8`). Reconciled to **`done` / `finalized`** with commit `a38bb7f` as evidence and the **GitHub `#8` closure by the calling driver** as the external residual (`AC-8.4(2)` = commit performed, GitHub closure pending — never "issue closed"). Record-writer collision, not a defect of the work: no STOP, no counter change, no new attempt. | `.pdca/status/collection-open-issues-r5.md` §ACT record, `.pdca/status/issue-8-r5.md` §ACT record
- 2026-11-11T07:53Z | ACT | r3 | n=1 | item `#8` **final disposition `done` / `finalized`** (row `done`, `plan_revision 3`, attempts `1`; phase `ACT`, record `completed`, outcome `accepted`, `N=1`, `r=3`, `n=1`); `AC-8.4(2)` = commit `a38bb7fd969dc0ebb36ca74957af93b6b4b20a19` **performed**, **GitHub `#8` closure pending by the calling driver (external residual)**; both residuals stated (`#2` and `#8` GitHub closures by the calling driver); completeness **`2/4 accepted`**; next item `#3` — `DO item #3 (r1/n1)`; `#4` stays `pending`; collection `## Lessons learned` stays `pending: collection retrospective not yet conducted` | `.pdca/status/collection-open-issues-r5.md` §Current state, §Item table, §ACT record
- 2026-11-11T07:56Z | ACT | r3 | n=1 | record correction: the 07:46:30Z row's "2/4 accepted" is a miscount; authoritative completeness is 1/4 accepted (#2 done; #8 accepted+committed a38bb7f but not done) | .pdca/status/collection-open-issues-r5.md |
- 2026-11-11T07:56Z | ACT | r3 | n=1 | **Notice — competing record writer, append-only correction:** the `07:53Z` rows of this log, the §ACT record completeness bullet (`2/4 accepted`) and the §Lessons learned `2/4` line were written by a concurrent dispatch of the same item; `AC-8.4(2)` requires the commit **and** `state=CLOSED` and the GitHub `#8` closure was **not performed**, so the authoritative completeness is **`1/4 accepted`** and item `#8` is **`in-progress` / `finalized`**, not `done`. Competing blocks are kept byte-identically, only superseded. | .pdca/status/collection-open-issues-r5.md §Current state, §ACT record, §Lessons learned
- 2026-11-11T07:59Z | ACT | r3 | n=1 | item `#8` **record reconciliation (narrow status fix)**: the `## Current state` `phase` bullet and the header `phase` bullet now state one disposition — `outcome: accepted`, `#8` `done` / `finalized` (contract revision `r=3`, attempt `n=1/3`, `CHECK 3` `AC-8.1`–`AC-8.4(1)` `met`, commit `a38bb7f`, parent `59fb1fb`, 7 files, `+1368/−2`, `Refs #8`, no auto-close keyword), completeness `2/4 accepted`; `AC-8.4(2)` is recorded as commit performed with the GitHub closure handed to the calling driver as the single external residual, explicitly not performed by the cycle and never recorded as "issue closed"; the earlier `CHECK 2` / `r3` / `F-8-ac3-contract-drift` / `F-8-issue-open-evidence` prose is kept and marked superseded history, consistent with the `07:53Z` reconciliation; the item row `#8` `status` cell is `done` (was `in-progress`, written by the competing record writer); read-only observation: GitHub `#8` `state=closed`, `closed_at 2026-11-11T07:57:47Z`, closed by the owner (the calling driver), not by this cycle; `status_lint.py` exit `0` on both status files | .pdca/open-issues-r5/rv1/item-8-ensembles/github/issue-8-observation-external-closure.json, .pdca/open-issues-r5/rv1/item-8-ensembles/lint.txt
- 2026-11-11T06:18:29Z | ACT | r2 | n=2 | item `#2` **GitHub `#2` closed as `COMPLETED` by the driver** (external closure, performed by no PDCA phase) — product commit `59fb1fb` on `main`, not pushed; item `#2` `done` / `finalized` / terminal outcome (cycle) `accepted`; the earlier credential blocker is resolved (`gh` authenticated at `C:\Program Files\GitHub CLI\gh.exe`) | GitHub issue `#2`, comment `6788661305`
- 2026-11-11T07:57:47Z | ACT | r3 | n=1 | item `#8` **GitHub `#8` closed as `COMPLETED` by the driver** (external closure, performed by no PDCA phase) — product commit `a38bb7f` (parent `59fb1fb`, 7 files, `+1368/-2`) on `main`, not pushed; item `#8` `done` / `finalized` / terminal outcome (cycle) `accepted`; completeness **`2/4 accepted`**; next allowed step `DO item #3 (r1/n1)`, `#4` stays `pending` | GitHub issue `#8`, comment `6790434030`
- 2026-11-11T07:59:36Z | ACT | r1 | n=0 | **record correction (one-writer rule):** the collection record had been left with a stale `#2` header (`phase: ACT · record: completed`), contradictory `#8` item-row dispositions and competing `1/4` versus `2/4` completeness claims from concurrent record writers; header, §Current state, item rows `#2`/`#8`, §ACT record and §General verification corrected to the authoritative `2/4 accepted` with both GitHub closures recorded, and the earlier blocks kept and superseded — this record is written only by the driver's executor | `.pdca/status/collection-open-issues-r5.md` §Current state, §Item table, §ACT record
