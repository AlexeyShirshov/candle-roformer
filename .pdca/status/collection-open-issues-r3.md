# Collection status: open-issues-r3 / collection

- collection-id: `open-issues-r3`
- slug: `open-issues-r3`
- form: `collection`
- plan_cap: 4
- lane_cap: 1
- cap_reason: all six item goals are PLAN-terminal, so no item can enter DO and no lane can be allocated; 1 is the nominal value for an empty schedule and is recorded here as an explicit choice, not a quiet default.
- specialized skill: `pdca-coder`
- mode: `normal`
- status path: `.pdca/status/collection-open-issues-r3.md`
- evidence root: `.pdca/open-issues-r3/rv1/`
- baseline: `main @ bf2cab6`, clean tree
- phase: `ACT` · record: `completed` · outcome: `incomplete/STOP — all six items PLAN-terminal infeasible; completeness=no; integrated-tree CHECK not run; retrospective recorded`

## Current state

- task: `open-issues-r3` · form: `collection` · specialized skill: `pdca-coder`
- mode: `normal`
- collection: `N=1` · plan revision `r=1` · schedule `s1` · attempts `n=0` (no item consumed an attempt)
- scope: the 6 open GitHub issues #1, #2, #3, #4, #8, #11 · status path: `.pdca/status/collection-open-issues-r3.md`
- goal: implement the six open issues so every verbatim `Done when` criterion is satisfied — the §PLAN record and §Original task are authoritative
- phase: `ACT` · record: `completed` · outcome: `incomplete/STOP — all six items PLAN-terminal infeasible; no DO, no CHECK, no accepted goal, no product change`
- source: GitHub issues of `AlexeyShirshov/candle-roformer`, fetched 2026-11-10 · context refs: `.pdca/status/collection-issues-milestone-1.0.md`, `.pdca/status/collection-issues-milestone-1.0-r2.md`
- criteria: `AC-ISSUE-1`, `AC-ISSUE-2`, `AC-ISSUE-3`, `AC-ISSUE-4`, `AC-ISSUE-8`, `AC-ISSUE-11` — verbatim in §PLAN record `r1` and §Input, not retyped here
- schedule: one group `g0` holding all six items; every item resolved at PLAN, so no worktree, task branch, merge, autocommit or push exists for this collection
- blockers: no real checkpoints, frozen reference fixtures, pinned checksums or numeric tolerances; no real CUDA host and no Apple Silicon host; downloads forbidden by the constraints

## Original task

The user's task, verbatim (original language, Russian):

> pdca по открытым задачам из github

Caller framing supplied with the task (not a replacement of it): run the PDCA collection over the six open GitHub issues #1, #2, #3, #4, #8, #11 of `AlexeyShirshov/candle-roformer`, with the issue bodies' own `Done when` criteria as the acceptance criteria; no downloads, no fabricated substitutes, no false PASS.

## Input (verbatim briefs)

Every item brief below is the verbatim GitHub API text fetched 2026-11-10; all six issues are open.

```text
1. #1 feat: support BS-RoFormer and the general MDXC/RoFormer family — Done when: "a real BS-RoFormer checkpoint loads, passes strict validation and separates audio; numerical parity against a pinned PyTorch reference (same thresholds as MelBand parity gate); README lists supported families and config files."
2. #2 feat: MDX-Net (ONNX) architecture — Done when: "a published MDX-Net ONNX file (supplied locally, checksum-checked by caller) separates audio on CPU; output matches audio-separator on a frozen fixture within agreed tolerance; dependency documented in docs/dependency-audit.md with licence."
3. #3 feat: VR/UVR architecture (.pth models) — Done when: "a chosen VR checkpoint converts with a pinned checksum and separates audio; parity fixture against audio-separator; converter covered by tests like the v7 one."
4. #4 feat: Demucs (4-stem and 6-stem) architecture — Done when: "htdemucs weights load with strict validation and separate a fixture into 4 stems; parity against the reference implementation and per-stem output naming."
5. #8 feat: multi-model ensembles and ensemble presets — Done when: "at least avg_wave and median_wave match the reference on a frozen fixture with two checkpoints; a preset produces the documented stem set on a real song."
6. #11 test: CUDA/Metal smoke tests and a validated reduced-precision path (gate G2) — Done when: "at least one real CUDA host and one Apple Silicon host run the smoke suite, logs committed as evidence, README/gates report state exactly what passed; until then README keeps saying 'not tested'."
```

## Constraints

- normal mode; PLAN-terminal outcomes go to ACT without DO approval, so no approval is requested or recorded;
- the user-supplied `Done when` criteria are immutable; no criterion weakening and no surrogate deliverable (synthetic weights or CPU-only substitution is explicitly prohibited by the PLAN);
- no real checkpoints, frozen reference fixtures, pinned checksums, numeric tolerances, or real CUDA / Apple Silicon hosts are available; no downloads;
- no false PASS, no `done` without recorded evidence;
- working directory `F:\sources\candle-roformer`; `.pdca` is the state root;
- this recording task itself: no product-file edit, no `git add`, no staging, no commit, no push;
- `plan_cap: 4`, `lane_cap: 1` recorded in the header with the reason above.

## PLAN record

### r1

```text
Form: collection. Specialized skill: pdca-coder. Entry scope: the 6 open issues #1, #2, #3, #4, #8, #11.
Requested result: the 6 open GitHub issues implemented in candle-roformer, satisfying every verbatim `Done when` criterion. Delivered result: the same object; partial implementation on synthetic weights or CPU-only is prohibited as a substitute. If the object cannot be produced under the stated constraints, PLAN returns `original task is infeasible under the stated constraints` and goes PLAN → ACT without DO and CHECK.

Acceptance criteria (user-supplied, verbatim from the issue bodies):
- AC-ISSUE-1 (#1): a real BS-RoFormer checkpoint loads, passes strict validation and separates audio; numerical parity against a pinned PyTorch reference (same thresholds as MelBand parity gate); README lists supported families and config files.
- AC-ISSUE-2 (#2): a published MDX-Net ONNX file (supplied locally, checksum-checked by caller) separates audio on CPU; output matches audio-separator on a frozen fixture within agreed tolerance; dependency documented in docs/dependency-audit.md with licence.
- AC-ISSUE-3 (#3): a chosen VR checkpoint converts with a pinned checksum and separates audio; parity fixture against audio-separator; converter covered by tests like the v7 one.
- AC-ISSUE-4 (#4): htdemucs weights load with strict validation and separate a fixture into 4 stems; parity against the reference implementation and per-stem output naming.
- AC-ISSUE-8 (#8): at least avg_wave and median_wave match the reference on a frozen fixture with two checkpoints; a preset produces the documented stem set on a real song.
- AC-ISSUE-11 (#11): at least one real CUDA host and one Apple Silicon host run the smoke suite, logs committed as evidence, README/gates report state exactly what passed; until then README keeps saying "not tested".

Measurement method: numeric parity = max absolute per-sample error (PCM float32) and Pearson correlation across all channels/stems on the full frozen fixture; peak memory in MiB per backend. Thresholds #1 anchored to the existing MelBand gate (correlation 0.9999999, max abs error 3.5e-4, chunk parity 1e-4 in tests/streaming_chunked.rs); #2/#3/#4/#8/#11 have no numeric tolerance supplied by the user. The required measurement sources (real checkpoints, frozen reference fixtures from PyTorch/audio-separator/htdemucs, real CUDA and Apple Silicon hosts) are absent from this environment.

Terminal outcome: original task is infeasible under the stated constraints. Transition: PLAN → ACT, without DO and CHECK, no surrogate artifacts, no criterion weakening.

Per-item terminal causes:
- #1: no real BS-RoFormer checkpoint, no pinned PyTorch reference outputs, no frozen audio fixture; even the base MelBand gate G1 is blocked by missing voc_fv7.ckpt (docs/gates-report.md:39). Tests only synthesize tiny MelBand checkpoints (tests/library_api.rs:184, tests/streaming_chunked.rs:43).
- #2: no published MDX-Net .onnx file, no checksum, no audio-separator frozen fixture, no agreed numeric tolerance; no ONNX runtime dependency in Cargo.toml/Cargo.lock.
- #3: no real VR .pth checkpoint, no pinned checksum, no audio-separator parity fixture.
- #4: no htdemucs weights, no audio fixture, no reference 4-stem output; multi-stem API prerequisite #12 is already committed.
- #8: requires two real checkpoints and a frozen ensemble reference; only synthetic fixtures exist; also depends on a second loadable model family that is not implemented.
- #11: requires a real NVIDIA CUDA host and a real Apple Silicon Metal host; docs/gates-report.md:57-59 records G2 BLOCKED — no NVIDIA GPU / no Metal host; README.md:157-168 says CUDA/Metal not tested.

Why not `requested result already established`: the prior collection records (.pdca/status/collection-issues-milestone-1.0.md:121-129 and collection-issues-milestone-1.0-r2.md:50-56,124-129) record infeasibility, not completion; the issues remain open and the code still lacks these families (src/model/mod.rs only MelBandRoformer; src/separator.rs:233 hard-codes it).

Constraints: normal mode; working directory F:\sources\candle-roformer; .pdca state root; the user-supplied `Done when` criteria are immutable; no real checkpoints, reference fixtures, checksums, numeric tolerances or CUDA/Apple Silicon hosts are available; no downloads; no false PASS.

Risks/stop condition: any DO in bypass of these blockers would fabricate the `Done when` criteria or fail CHECK.
```

Persisted verbatim copy: `.pdca/open-issues-r3/rv1/plan.md` (identity: same bytes as this block's content).

### Item resolution #1

- item: `#1` — support BS-RoFormer and the general MDXC/RoFormer family
- outcome: `PLAN → ACT` — `original task is infeasible under the stated constraints`
- cause: no real BS-RoFormer checkpoint, no pinned PyTorch reference outputs and no frozen audio fixture; even the base MelBand gate G1 is blocked by the missing `voc_fv7.ckpt` (`docs/gates-report.md:39`); tests only synthesise tiny MelBand checkpoints (`tests/library_api.rs:184`, `tests/streaming_chunked.rs:43`).
- evidence: `.pdca/open-issues-r3/rv1/plan.md`, `.pdca/open-issues-r3/rv1/scout.md`; `docs/gates-report.md:39`; `tests/library_api.rs:184`; `tests/streaming_chunked.rs:43`.
- attempts: `0`; no DO, no CHECK, no goal credit.

### Item resolution #2

- item: `#2` — MDX-Net (ONNX) architecture
- outcome: `PLAN → ACT` — `original task is infeasible under the stated constraints`
- cause: no published MDX-Net `.onnx` file, no checksum, no audio-separator frozen fixture and no agreed numeric tolerance; no ONNX runtime dependency in `Cargo.toml`/`Cargo.lock`.
- evidence: `.pdca/open-issues-r3/rv1/plan.md`, `.pdca/open-issues-r3/rv1/scout.md`; `Cargo.toml`; `Cargo.lock`.
- attempts: `0`; no DO, no CHECK, no goal credit.

### Item resolution #3

- item: `#3` — VR/UVR architecture (.pth models)
- outcome: `PLAN → ACT` — `original task is infeasible under the stated constraints`
- cause: no real VR `.pth` checkpoint, no pinned checksum and no audio-separator parity fixture.
- evidence: `.pdca/open-issues-r3/rv1/plan.md`, `.pdca/open-issues-r3/rv1/scout.md`.
- attempts: `0`; no DO, no CHECK, no goal credit.

### Item resolution #4

- item: `#4` — Demucs (4-stem and 6-stem) architecture
- outcome: `PLAN → ACT` — `original task is infeasible under the stated constraints`
- cause: no `htdemucs` weights, no audio fixture and no reference 4-stem output; the multi-stem API prerequisite #12 is already committed but does not supply the model or the reference.
- evidence: `.pdca/open-issues-r3/rv1/plan.md`, `.pdca/open-issues-r3/rv1/scout.md`; commit `1603333` (#12).
- attempts: `0`; no DO, no CHECK, no goal credit.

### Item resolution #8

- item: `#8` — multi-model ensembles and ensemble presets
- outcome: `PLAN → ACT` — `original task is infeasible under the stated constraints`
- cause: the criterion requires two real checkpoints and a frozen ensemble reference; only synthetic fixtures exist, and the second loadable model family is not implemented.
- evidence: `.pdca/open-issues-r3/rv1/plan.md`, `.pdca/open-issues-r3/rv1/scout.md`; `src/config.rs:152`.
- attempts: `0`; no DO, no CHECK, no goal credit.

### Item resolution #11

- item: `#11` — CUDA/Metal smoke tests and a validated reduced-precision path (gate G2)
- outcome: `PLAN → ACT` — `original task is infeasible under the stated constraints`
- cause: the criterion requires a real NVIDIA CUDA host and a real Apple Silicon Metal host; `docs/gates-report.md:57-59` records G2 BLOCKED with no NVIDIA GPU and no Metal host, and `README.md:157-168` records CUDA/Metal as not tested.
- evidence: `.pdca/open-issues-r3/rv1/plan.md`, `.pdca/open-issues-r3/rv1/scout.md`; `docs/gates-report.md:57-59`; `README.md:157-168`.
- attempts: `0`; no DO, no CHECK, no goal credit.

### Schedule s1 (planner decision)

- DAG: no dependency edges — every one of `#1`, `#2`, `#3`, `#4`, `#8`, `#11` is resolved at PLAN and none enters execution.
- groups: `g0 = [#1, #2, #3, #4, #8, #11]` (six PLAN-terminal resolutions, no write surface).
- caps: `plan_cap: 4` (six independent item PLANs could be dispatched in parallel), `lane_cap: 1` with the recorded `cap_reason` (no executable lane exists).
- merge strategy: `none` — one group, no worktree, no task branch, no merge.
- integration-CHECK scope: the whole-collection integrated tree; not run because no group entered DO and the tree is unchanged (recorded as a limitation, never as a PASS).
- revision vector: schedule `s1`, item plan revisions `r1`.

## Approval record

`not applicable: PLAN → ACT for all six items — no ready plan entered DO and no approval was requested or possible; a normal-mode approval slot exists only for a ready plan that would start DO.`

## Item table

| item | group | branch | status | plan_state | form | contract | overlay | cycle_id | status_path | plan_revision | disposition | terminal | attempts | notes |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| #1 | g0 | main (current; no task branch) | incomplete | pending | cycle | pdca | pdca-coder | issue-1-3 | .pdca/status/collection-open-issues-r3.md (Item resolution #1) | 1 | finalized | infeasible | 0 | PLAN-terminal — no real BS-RoFormer checkpoint, no pinned PyTorch reference, no frozen fixture; no DO, no CHECK, no goal credit |
| #2 | g0 | main (current; no task branch) | incomplete | pending | cycle | pdca | pdca-coder | issue-2-3 | .pdca/status/collection-open-issues-r3.md (Item resolution #2) | 1 | finalized | infeasible | 0 | PLAN-terminal — no MDX-Net ONNX file, no checksum, no audio-separator fixture, no tolerance; no DO, no CHECK, no goal credit |
| #3 | g0 | main (current; no task branch) | incomplete | pending | cycle | pdca | pdca-coder | issue-3-3 | .pdca/status/collection-open-issues-r3.md (Item resolution #3) | 1 | finalized | infeasible | 0 | PLAN-terminal — no VR checkpoint, no pinned checksum, no audio-separator parity fixture; no DO, no CHECK, no goal credit |
| #4 | g0 | main (current; no task branch) | incomplete | pending | cycle | pdca | pdca-coder | issue-4-3 | .pdca/status/collection-open-issues-r3.md (Item resolution #4) | 1 | finalized | infeasible | 0 | PLAN-terminal — no htdemucs weights, no audio fixture, no reference 4-stem output; no DO, no CHECK, no goal credit |
| #8 | g0 | main (current; no task branch) | incomplete | pending | cycle | pdca | pdca-coder | issue-8-3 | .pdca/status/collection-open-issues-r3.md (Item resolution #8) | 1 | finalized | infeasible | 0 | PLAN-terminal — needs two real checkpoints and a frozen ensemble reference plus a second family; no DO, no CHECK, no goal credit |
| #11 | g0 | main (current; no task branch) | incomplete | pending | cycle | pdca | pdca-coder | issue-11-3 | .pdca/status/collection-open-issues-r3.md (Item resolution #11) | 1 | finalized | infeasible | 0 | PLAN-terminal — requires a real CUDA host and an Apple Silicon Metal host; no DO, no CHECK, no goal credit |

## Groups table

| group | items (order) | worktree | branch/ref | status | notes |
|---|---|---|---|---|---|
| g0 | [#1, #2, #3, #4, #8, #11] | — (no write surface; no lane allocated) | main @ bf2cab6 | pending | six PLAN-terminal infeasible resolutions; no executable item, no lane process, no group stop, no merge; the group is not done and grants no goal credit; a terminal resolution never makes a group done — a group needs executed items, so g0 stays `pending` |

## DO result

`not reached: no DO — every item is a PLAN-terminal resolution (PLAN → ACT); no candidate, no product change, no attempt consumed.`

## CHECK record

`not run: PLAN → ACT for all six items; no candidate existed to check. The collection integration CHECK did not run either, because no group entered DO and the tree is unchanged — this is an explicit limitation, not a PASS, and no criterion is marked met.`

## ACT record

- **Collection closed: incomplete / STOP** — completeness: **no**; all six items are PLAN-terminal `infeasible`, `attempts=0`; no DO, no CHECK, no accepted goal, no product change.
- **Accepted goals:** none.
- **Result clarification (requested vs delivered):** the requested result was implementation of the six issues; the actual delivered result was a terminal infeasibility report. This is a wording clarification only — it changes neither §PLAN record nor §Original task and adds no criterion.
- **Retrospective closure:** the previously missing `pdca-check` retrospective report was supplied and is persisted verbatim in §Lessons learned; that closure blocker is resolved and this record is `completed`. The STOP outcome, `completeness=no` and every item status are unchanged.
- **PLAN-terminal resolutions:** #1, #2, #3, #4, #8, #11 — `original task is infeasible under the stated constraints`, `attempts=0`, no goal credit.
- **Execution failures:** none (no item reached execution).
- **Integrated-tree CHECK:** not run, because no item entered DO and the tree is unchanged; recorded explicitly as a limitation, not as a PASS.
- **Next step:** a new explicit user instruction is required to (a) relax the `Done when` criteria (for example implement architectures/converters against synthetic fixtures, with no real checkpoints or hardware), or (b) supply the missing checkpoints, reference fixtures and numeric tolerances plus the CUDA and Apple Silicon hosts, or (c) choose one issue and provide its required artifacts. Until then the issues stay open and README must keep CUDA/Metal as "not tested".
- **Bookkeeping:** no commit, no push, no staging; product files untouched.
- **Notice (premise correction):** the brief states ".pdca is gitignored" — refuted by the tree: `.gitignore:5-7` tracks `.pdca/status/` and `.pdca/scripts/` via exceptions, while `.pdca/open-issues-r3/` is ignored. Pointer: `/.pdca/*` at `.gitignore:5` → exceptions at `.gitignore:6-7`. Recorded honestly in §Git state; no contract effect.
- **Notice (concurrent write):** a second process replaced this status path at 13:45:42Z and again at 13:46:03Z with a record whose §PLAN record was paraphrased and whose §Input titles were shortened; single-writer was violated by that dispatch, not by this record. Evidence and timestamps: `.pdca/open-issues-r3/rv1/concurrent-write.txt`.
- **Notice (concurrent write, ACT close):** the path kept changing across this close — a read at 13:47:11Z returned a 199-line variant, the 227-line record was in place by 13:47:26Z (sha256 `5D67069D…`), and it changed again at 13:48:00Z (sha256 `EC53F39B…`, same 227 lines). This close was applied to that compliant record and verified afterwards; if the path is replaced again, `concurrent-write.txt` is the conflict pointer. No product file was affected.

## General verification and recovery

verification state `unverified` (no common CHECK ran); last common C result `none — no CHECK was possible from a PLAN-terminal outcome`; blocking reason `six PLAN-terminal infeasible resolutions — required measurement sources (real checkpoints, frozen references, pins/tolerances, CUDA and Apple Silicon hosts) absent from the environment`; defect history `none`; linked corrective cycle `none`; next allowed step `none — collection closed; further work needs a new authorised task or the missing artifacts`; completeness `no`; integrated-tree CHECK verdict `not run`; incomplete items `#1, #2, #3, #4, #8, #11 (PLAN-terminal infeasible)`; preserved results `PLAN and scout evidence under .pdca/open-issues-r3/rv1/ (plan.md, scout.md, git-state.txt, lint.txt, concurrent-write.txt, self-check/)`.

## Revision history

`none — initial record at r1 (schedule s1); no replan occurred.`

## Defect history

`none — no attempt, no defect observed, no fix applied; the six resolutions are PLAN-terminal, not failures.`

## Changed files

`.pdca/status/collection-open-issues-r3.md → created this cycle; .pdca/open-issues-r3/rv1/plan.md → created (verbatim PLAN record); .pdca/open-issues-r3/rv1/scout.md → created; .pdca/open-issues-r3/rv1/git-state.txt → created; .pdca/open-issues-r3/rv1/lint.txt → created; .pdca/open-issues-r3/rv1/concurrent-write.txt → created; .pdca/open-issues-r3/rv1/self-check/check_tables.mjs → created (structural self-check script); .pdca/scripts/status_lint.py → unchanged (byte-identical to the skill copy, sha256 52a86088…); no product file changed.`

## Git state

status path: `.pdca/status/collection-open-issues-r3.md` — created earlier in this cycle and modified now (ACT retrospective close) · untracked and unstaged, not indexed (durable on disk, not committed) — baseline `main @ bf2cab6`, tree clean when the brief was issued; `git status --porcelain -- .pdca` reports exactly `?? .pdca/status/collection-open-issues-r3.md`. The containing `.pdca/status/` is a negated (non-ignored) tracked surface — `.gitignore:5` ignores `/.pdca/*` and `.gitignore:6-7` negate `/.pdca/status/` and `/.pdca/scripts/` — while this new file itself is untracked and was never staged. Evidence `.pdca/open-issues-r3/rv1/*` is untracked and ignored by `.gitignore:5`. Task-owed delta: the paths in §Changed files, none staged or committed; nothing added, committed or pushed. Work outside this delta stays untouched and unclaimed.

## Lessons learned

Source: raw `pdca-check` retrospective report (parent-only collection scope), received and persisted verbatim on 2026-11-10T13:48:42Z.

```text
### Ретроспектива родительской коллекции

**Что сработало.** Шесть задач перечислены отдельно с причинами терминального завершения и нулевыми попытками. Отсутствие DO, CHECK и интеграционной проверки явно обозначено, зачёт целей не присвоен. Одна полоса обоснована отсутствием исполнения.  
Основание: `.pdca/status/collection-open-issues-r3.md:71–146`.

**Выявленные дефекты учёта:**

1. **Противоречивые признаки завершения.** Запись одновременно содержит `record: open`, блокер незавершённой ретроспективы и утверждения «collection closed», включая журнал ACT. Элементы помечены `finalized`, но их `plan_state` и группа остаются `pending`.  
   **Эффект:** неоднозначность состояния для следующего исполнителя или автоматизации.  
   **Изменение на будущее:** различать зафиксированный терминальный исход и завершение ACT; согласовывать таблицы и журнал перед окончательным закрытием.  
   **Основание:** status:17, 127–138, 158, 169–173, 195.  
   **Уверенность:** высокая в противоречии записей; фактический сбой передачи не установлен.

2. **Формулировка результата противоречит исходу.** PLAN называет реализацию шести задач одновременно запрошенным и доставленным результатом, хотя далее фиксирует невозможность выполнения.  
   **Эффект:** риск ошибочно прочитать сводку как сообщение о выполнении.  
   **Изменение на будущее:** раздельно указывать запрошенный результат и фактически полученный терминальный отчёт.  
   **Основание:** `.pdca/open-issues-r3/rv1/plan.md:6`; status:54.  
   **Уверенность:** высокая; происхождение формулировки внутри родительского handoff не установлено.

### Переносимые уроки

- **Учёт состояния:** терминальный исход не означает завершённый ACT; все представления состояния должны различать эти события.
- **Передача результатов:** отделять цель от фактически доставленного результата, особенно при STOP.
- **Инструментальные ограничения:** сохранять команду, ошибку и статус `unverified`, не заменяя отсутствующий lint ручной проверкой. Здесь это корректно отражено в `lint.txt:6–16`.

**Пределы:** анализ основан только на существующих отчётах. Полный журнал handoff и отдельный DAG не представлены; ошибок распределения зависимостей или интеграции не установлено. Доменные результаты и дочерние уроки не оценивались; исход STOP не пересматривался.

**Сигнал: ретроспективный анализ завершён; сохранение отчёта исполнителем ещё требуется.**
```

Retrospective completion: report persisted verbatim; ACT bookkeeping complete. Evidence: .pdca/open-issues-r3/rv1/plan.md, .pdca/status/collection-open-issues-r3.md.

## Progress log

- `2026-11-10T13:44:32Z | PLAN | revision r1 | attempt n0 | collection open-issues-r3 recorded: six open issues, all PLAN-terminal infeasible; barrier satisfied by resolutions, no item is readiness or done | .pdca/open-issues-r3/rv1/plan.md`
- `2026-11-10T13:44:32Z | PLAN | revision r1 | attempt n0 | schedule s1 persisted: plan_cap 4, lane_cap 1 (no executable lane), single group g0 with six terminal items, merge none | this file`
- `2026-11-10T13:44:32Z | ACT | revision r1 | attempt n0 | collection closed incomplete/STOP; completeness=no; no DO, no CHECK, no accepted goal, no product change; nothing staged, committed or pushed | this file`
- `2026-11-10T13:44:32Z | ACT | revision r1 | attempt n0 | Lessons learned left pending: retrospective report not received (closure blocker) | this file`
- `2026-11-10T13:44:32Z | NOTE | revision r1 | attempt n0 | premise correction: .pdca is not wholly gitignored — .pdca/status and .pdca/scripts are tracked, .pdca/open-issues-r3 is ignored | this file`
- `2026-11-10T13:44:32Z | NOTE | revision r1 | attempt n0 | status_lint unverified: Python unavailable on host (exit 9009, Microsoft Store alias stub); exact error and the Node structural self-check in .pdca/open-issues-r3/rv1/lint.txt | .pdca/open-issues-r3/rv1/lint.txt`
- `2026-11-10T13:46:29Z | NOTICE | revision r1 | attempt n0 | concurrent duplicate executor replaced this status path at 13:45:42Z and 13:46:03Z with a paraphrased PLAN record; single-writer conflict recorded, compliant record rewritten | .pdca/open-issues-r3/rv1/concurrent-write.txt`
- `2026-11-10T13:48:42Z | ACT | revision r1 | attempt n0 | retrospective persisted; Lessons learned completed; record remains completed | this file`
- `2026-11-10T13:49:33Z | ACT | revision r1 | attempt n0 | retrospective persisted; record completed; outcome incomplete/STOP | .pdca/status/collection-open-issues-r3.md`
- `2026-11-10T13:49:33Z | ACT | revision r1 | attempt n0 | collection closed; no product change; no commit/push/stage | .pdca/open-issues-r3/rv1/plan.md`
