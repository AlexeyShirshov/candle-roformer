# Cycle status: issue-10 / cycle 1

## Current state

- task: `issue-10` · form: `cycle` · specialized skill: `pdca-coder`
- mode: `autonomous`
- cycle: `N=1` · revision: `r=1` · attempt: `n=1/3`
- scope: item #10 of collection `issues-milestone-1.0` (bounded-memory processing for long inputs)
- status path: `.pdca/status/issue-10-1.md`
- goal: `feat: bounded-memory processing for long inputs (--chunk_duration)` — peak RSS roughly constant in input length for a fixed chunk duration; output length and residual invariant hold and boundary quality documented.
- phase: `ACT` · record: `completed` · outcome: `incomplete (terminal=execution)`
- source: `collection entry PLAN (r1) + item PLAN (pdca-planner)` · context refs: `.pdca/status/collection-issues-milestone-1.0.md`
- criteria: `AC-10 = verbatim "Done when" of issue #10 (see §PLAN record r1)`

## Original task

Run the collection of all 11 open issues #1–#11 of `AlexeyShirshov/candle-roformer` (milestone 1.0): each item ends either accepted+committed against its own issue criteria, or with a documented terminal outcome `blocked`/`infeasible` with evidence. Autonomous mode, no push, no branch switch, no history rewrite, no download/bundling/caching of model weights, no faked PASS.

Item brief (verbatim): #10 `feat: bounded-memory processing for long inputs (--chunk_duration)` — Done when: peak RSS roughly constant in input length for a fixed chunk duration; output length and residual invariant hold and boundary quality documented.

## PLAN record

### r1

# ITEM PLAN — Issue #10 (`feat: bounded-memory processing for long inputs (--chunk_duration)`)

## 1. Форма и входной объём

- Форма: `cycle`; overlay `pdca-coder`. Объём — полная цель #10: публичный потоковый/чанковый файловый путь разделения с параметром длительности чанка (`chunk_duration` / `--chunk_duration`), отсутствие устранимых граничных артефактов, инварианты длины выхода и остатка, задокументированное качество границ, примерно постоянный пиковый RSS при фиксированном `chunk_duration`, включая длинную фикстуру (например, 2 ч).
- Привязка: текущий HEAD `/home/alex/sources/candle-roformer` на момент DO; фикстуры — эфемерные в `/tmp/opencode`.

### Пользовательские критерии приёмки (verbatim)
> `Missing: a streaming/chunked file path (decode chunk N, separate, write, drop); chunk splitting without avoidable boundary artefacts; peak-RSS evidence on a long fixture (e.g. 2 h).`
> `Done when: peak RSS roughly constant in input length for a fixed chunk duration; output length and residual invariant hold and boundary quality documented.`

- **AC-1 (`streaming/chunked file path (--chunk_duration)`):** реализован потоковый файловый путь (декодирование чанка N, разделение, инкрементальная запись стемов, освобождение чанка N) с управлением длительностью чанка (`chunk_duration`, включая флаг `--chunk_duration` в исполняемом примере/раннере внутри репозитория). Негатив: полный вход и все полные выходные буферы не удерживаются одновременно.
- **AC-2 (`peak RSS roughly constant ... peak-RSS evidence on a long fixture (e.g. 2 h)`):** при фиксированном `chunk_duration` пиковый RSS примерно постоянен при кратном росте длины входа вплоть до ≥2 ч; прирост `VmHWM`/`ru_maxrss` ≤ 20 % (провизорный порог входного PLAN коллекции); нет OOM и линейного накопления.
- **AC-3 (`output length and residual invariant hold`):** длина каждого выходного стема в сэмплах на канал точно равна длине входа (отклонение 0), и `sum(stems) + residual = input` с `max |...| <= 1e-5`; нет обрезки, лишнего паддинга, `NaN`/`Inf`.
- **AC-4 (`chunk splitting without avoidable boundary artefacts`, `boundary quality documented`):** разбиение использует перекрытие и плавное сведение (overlap-add/crossfade или согласование с внутренним OLA-шагом), исключающее скачки на стыках; расхождение с монолитным `Separator::separate` в окрестности границ измерено и задокументировано вместе с алгоритмом склейки и параметрами перекрытия.

## 2. Метод измерения

### 2.1 Пиковый RSS как функция длины (AC-2)
- Метрика: `VmHWM` (`/proc/self/status`) или `ru_maxrss`, КиБ/МиБ; относительный прирост `Δ = (PeakRSS(L_long) - PeakRSS(L_short)) / PeakRSS(L_short) * 100 %`.
- Знаменатель: `PeakRSS(L_short)` при многочанковом входе `L_short >= 2 × chunk_duration` после аллокации чанка и буфера перекрытия.
- Фильтры: полный прогон потокового файлового разделения в отдельном процессе; кэш страниц ОС исключается.
- Сопоставимость: одинаковая конфигурация `Separator`, 44.1 кГц стерео, одинаковый `chunk_duration` и `CANDLE_ROFORMER_CHUNK_HOPS`, один и тот же `--release` бинарник, изолированный процесс на каждую длину. В набор входит фикстура ≥2 ч (7200 с, ≈317.52e6 фреймов) в `/tmp/opencode`; допускается прогон 2 ч через тот же потоковый конвейер с облегчённым `RoformerConfig` при идентичном конфиге на паре сравнения, плюс отдельная проверка постоянства RSS на реальных весах `MelBandRoformer_fp32.safetensors` на серии кратных длин (2×, 4×, 8× `chunk_duration`).
- Порог: `Δ ≤ 20 %` при росте длины ≥4×, включая 2 ч фикстуру; абсолютный Peak RSS на 2 ч меньше объёма несжатого входа+выходов в памяти (≈7.6 ГБ для 2 ч f32 стерео). Происхождение: провизорная операционализация «roughly constant in input length» из входного PLAN коллекции.

### 2.2 Длина выхода и остаток (AC-3)
- Метрики: `ΔL = |L_out,s - L_in|` (сэмплы/фреймы) для каждого стема; `E_res = max_{c,t} |(Σ_s Ŷ_s[c,t] + R[c,t]) - X[c,t]|`.
- Знаменатель: все каналы и все отсчёты без исключения граничных зон.
- Пороги: `ΔL = 0`; `E_res <= 1e-5` (точность f32 WAV).

### 2.3 Качество границ (AC-4)
- Метрика: `E_boundary = max |Ŷ_chunked - Ŷ_full|` и RMSE между чанковым и монолитным разделением на многочанковом сигнале, включая окрестности `k · step_samples`, в амплитудных единицах (и дБ).
- Сопоставимость: тот же вход, те же веса/конфиг, тот же `CANDLE_ROFORMER_CHUNK_HOPS`.
- Порог: при согласованном шаге `E_boundary <= 1e-5` (численная эквивалентность f32) либо при внешнем кроссфейде — отсутствие разрывов первого рода и документированная верхняя граница погрешности.

## 3. Ограничения и допущения

- Только `/home/alex/sources/candle-roformer`; `/home/alex/sources/karaoke-cli` и другие репозитории — off-limits. Запрещены `git push`, переключение веток, переписывание истории; один атомарный коммит для файлов элемента после приёмки; нельзя менять `AGENTS.md`, `LICENSE`, `docs/superpowers/**`, записи других кампаний `.pdca`; нельзя скачивать веса; фикстуры только в `/tmp/opencode` и не коммитятся.
- **A-1 (`--chunk_duration` в рамках библиотечного репозитория):** поскольку `candle-roformer` — библиотечный крейт, а `karaoke-cli` вне доступа, упоминание `(--chunk_duration)` покрывается публичным API потокового разделения с параметром `chunk_duration` (в секундах), готовым к вызову из любого CLI, плюс исполняемый пример/утилита/бенчмарк-таргет внутри `candle-roformer` (например, `examples/...`), принимающий `--chunk_duration <seconds>` и используемый для воспроизводимого замера RSS.
- **A-2 (время прогона 2 ч на CPU):** потоковая подсистема одинакова для любой конфигурации, поэтому 2 ч фикстура может прогоняться на компактной конфигурации за минуты, а реальные веса `MelBandRoformer_fp32.safetensors` — на серии многочанковых фикстур меньшей длины (16/32/64 с при `chunk_duration = 8 с`).

## 4. Закрытие пробелов

| Пробел | Закрытие |
|---|---|
| `--chunk_duration` при недоступном `karaoke-cli` | Evidence + A-1: ни `Missing:`, ни `Done when:` не требуют правок `karaoke-cli`; закрывается публичным API с `chunk_duration` + пример с флагом `--chunk_duration` в `candle-roformer`. |
| Операционализация «peak RSS roughly constant» и 2 ч на 8 CPU | Evidence + A-2: `VmHWM`/`ru_maxrss`, порог ≤20 %, 2 ч фикстура генерируется в `/tmp/opencode` и прогоняется потоковым путём (компактный конфиг для 2 ч + реальные веса на серии многочанковых длин). |
| Что является артефактом «boundary quality documented» | Deliverable: документация в репозитории (раздел `README.md` и/или профильный документ в `docs/`, плюс doc-комментарии API) с описанием схемы перекрытия, условиями отсутствия граничных артефактов, замерами `E_boundary` и пикового RSS (включая 2 ч). |

## 5. Минимальное решение и альтернативы

1. Отклонено: наивная нарезка на неперекрывающиеся куски и `Separator::separate` на каждом (краевые эффекты STFT, слышимые щелчки).
2. Отклонено как избыточное: внешнее перекрытие с собственным линейным кроссфейдом поверх полного `separate` (двойной OLA, расхождение на стыках).
3. Выбрано: потоковый файловый конвейер — потоковый декодер (инкрементальное чтение без загрузки файла в `AudioBuffer`), потоковый писатель WAV (`StreamingWavWriter`, запись f32 порциями, обновление размера в заголовке), потоковый OLA/чанковый цикл с размером окна из `chunk_duration` (`step = chunk/2`, `fade = chunk/10` или эквивалентное перекрытие); в памяти только текущий входной чанк, хвост перекрытия на стем и буфер готового шага; финализированные отсчёты сразу пишутся в файлы стемов (и при необходимости в аккумулятор проверки остатка) и освобождаются. Результат численно согласован с in-memory OLA (до f32), граничные артефакты отсутствуют по построению, пиковый RSS зависит от `chunk_duration`, не от `L`.

## 6. Передача в DO

- Исходная задача и AC-1…AC-4 (раздел 1), метод измерения (раздел 2), границы области: `src/wav.rs`, `src/separate.rs`, `src/separator.rs`, `src/lib.rs`, тесты/примеры, документация; фикстуры — только `/tmp/opencode`, не коммитить.
- Запреты: не трогать другие репозитории; не менять `AGENTS.md`, `LICENSE`, `docs/superpowers/**`; не скачивать модели; не делать `git push`; не фиксировать фейковый PASS.
- Декомпозиция — DO; команды верификации — CHECK.

## 7. Риски и условия остановки

- R-1: 2 ч стерео WAV ≈1.27 ГБ (i16) / ≈2.54 ГБ (f32), плюс выходы ≈5 ГБ; при ~800 ГиБ свободно, временные файлы очищать.
- R-2: скрытая аллокация полного файла при декодировании — чтение обязано быть потоковым.
- R-3: выравнивание последнего чанка и STFT-паддинга — точное отслеживание числа входных фреймов и обрезка ровно до `L_in`.
- Остановка: необходимость править `/home/alex/sources/karaoke-cli`; не более 3 попыток на один ключ дефекта.

## 8. Уверенность

- Подтверждено: `read_audio` (`src/wav.rs:64-204`), `write_wav_f32` (`:206-248`) и `ola` (`src/separate.rs:57-137`) работают в памяти (O(L)); 41 тест проходит; локальные веса и ~800 ГиБ свободны.
- Допущения: A-1, A-2 (выше).

## Approval record

`not applicable: autonomous mode`

## DO result

Implemented bounded-memory chunked separation path:
- `src/separate.rs`: extracted public `infer_chunk` helper; reused by monolithic path.
- `src/wav.rs`: `StreamingWavWriter` (incremental f32 WAV writer) and `AudioStreamReader` (incremental decoder).
- `src/separator.rs`: `Separator::separate_file` / `separate_streaming` (rolling OLA, chunk_duration-driven), `FrameBuf`.
- `src/lib.rs`: re-exports.
- `examples/separate.rs` (+ `separate_streaming.rs`, `chunked_separate.rs`) with `--chunk_duration`.
- `tests/streaming_chunked.rs`: 3 tests (length/residual, boundary parity, mono).
- `README.md`: boundary-quality + RSS section.

Candidate (not verdict): monolithic vs streaming vocals bit-identical on 6000-frame stereo (E_boundary max abs 0.0, RMSE 0.0); length exact; residual <=1e-5.

Evidence: full suite log `/tmp/opencode/item10_test_full.log` (exit 0); targeted `cargo test --test streaming_chunked` (exit 0, 3 passed); RSS series recorded in README.md.

## CHECK record

CHECK (independent) verdict: `not accepted` (AC-10-2 `unverified`).

Per-criterion matrix:
- AC-10-1 (streaming/chunked file path, --chunk_duration): `met`
- AC-10-2 (peak RSS roughly constant in input length; long-fixture peak-RSS evidence, e.g. 2 h): `unverified`
- AC-10-3 (output length and residual invariant): `met`
- AC-10-4 (chunk splitting without avoidable boundary artefacts; boundary quality documented): `met`

Findings:
- `E10-LONG-RSS` — blocking: long-scale (7200 s) peak-RSS evidence not measured; the 7200 s run was terminated under the 300 s-per-call budget (empty log `.pdca/issues-milestone-1.0/rv1/item10/rss_7200s.log`); extrapolated wall ~3.3 h. `SCALE-CAVEAT.md` records that the long scale (and the 4 s chunk_duration) is unmeasured.
- `E10-REPORT-CONSISTENCY` — medium: short-fixture RSS series (10–300 s) and boundary parity (E_boundary max_abs 0.0, RMSE 0.0) / residual (1.49e-8) reported in README vs measurements.

Evidence pointers: `.pdca/issues-milestone-1.0/rv1/item10/measurements.md`, `.pdca/issues-milestone-1.0/rv1/item10/SCALE-CAVEAT.md`, `.pdca/issues-milestone-1.0/rv1/item10/runs.md`, `.pdca/issues-milestone-1.0/rv1/item10/rss_short/*`, `.pdca/issues-milestone-1.0/rv1/item10/item10_test_full.log`, `.pdca/issues-milestone-1.0/rv1/item10/item10_streaming_chunked.log`.

Source: CHECK verdict supplied to ACT for item #10 (no separate persisted CHECK artefact in `.pdca/`; recorded as returned).

## ACT record

Outcome: `incomplete` with `terminal=execution` (CHECK not accepted; no acceptance, no PASS, no goal credit, no commit).

Cause: AC-10-2 (long-scale peak-RSS evidence) `unverified` — the 7200 s run was terminated under the 300 s-per-call budget; `SCALE-CAVEAT.md` records the long scale as unmeasured. Findings: `E10-LONG-RSS` (blocking), `E10-REPORT-CONSISTENCY` (medium). AC-10-1, AC-10-3, AC-10-4 met.

What was completed: streaming/chunked separation implementation + tests + README section (candidate), short-fixture RSS series (10–300 s, flat ~16–17 MiB), boundary parity (max_abs 0.0, RMSE 0.0), residual 1.49e-8. Candidate was NOT accepted and is NOT committed.

What remains (`unverified`): long-scale (2 h) peak-RSS evidence; 4 s chunk_duration exercise; real-weights RSS. Residual requirements AC-10-2 unmet/unverified.

Attempt counter: 1 (of 3). No commit made. No replan (single attempt spent; revision r1).

Preserved patch path: `.pdca/issues-milestone-1.0/rv1/item10/incomplete-patch/` (tracked.patch + new/* + MANIFEST.md; base commit 3c1eec0). Item's paths rolled back to accepted tip 3c1eec0; tree clean.

This is NOT a plan-terminal resolution and gives no credit. Terminal cause = execution (run-time budget), not PLAN.

### ACT (2026-11-09, user override)

Outcome: `accepted` by explicit user instruction («10 закрывай как выполненное и комить»). AC-10-2 accepted on short-scale measurements (10–300 s, tiny model, chunk_duration = 0.07256 s, peak RSS 16–17 MiB plateau, +3 % growth); long-scale (2 h) evidence NOT obtained — see `SCALE-CAVEAT.md`. Commit: `b4cb290ad1cf864f3edc76e3e5bf8e0ad1e93ccb`.

## Revision history

`none`

## Defect history

`none`

## Changed files

- `.pdca/status/issue-10-1.md` → this ACT finalization (modified)
- `.pdca/issues-milestone-1.0/rv1/item10/incomplete-patch/` → preserved candidate patch (tracked.patch + new/* + MANIFEST.md)
- Product files (README.md, src/lib.rs, src/separate.rs, src/separator.rs, src/wav.rs, examples/*, tests/streaming_chunked.rs): rolled back to accepted tip 3c1eec0; not committed.

## Git state

`status path: .pdca/status/issue-10-1.md — modified this cycle · untracked (durable on disk, not committed); baseline HEAD 3c1eec0; commit b4cb290ad1cf864f3edc76e3e5bf8e0ad1e93ccb; tree clean; preserved patch at .pdca/issues-milestone-1.0/rv1/item10/incomplete-patch/`

## Progress log

- 2026-11-08T09:01:48Z | PLAN | revision r1 | attempt n0 | item PLAN ready (plan_state=ready); no DO yet | this file
- 2026-11-08T15:00:00Z | DO | revision r1 | attempt n1 | DO started | this file
- 2026-11-08T16:00:00Z | DO | revision r1 | attempt n1 | candidate implemented; streaming==monolithic E_boundary 0.0; full suite exit 0 (55 tests); RSS plateau ~17 MiB | /tmp/opencode/item10_test_full.log
- 2026-11-08T16:30:00Z | DO | revision r1 | attempt n1 | evidence persisted under .pdca/issues-milestone-1.0/rv1/item10/ (runs.md, fixtures.md, MANIFEST.md) | .pdca/issues-milestone-1.0/rv1/item10/MANIFEST.md
- 2026-11-08T13:00:18Z | DO | revision r1 | attempt n=1/3 | killed leftover 7200 s run (pid 477784) and samplers; confirmed gone | n/a
- 2026-11-08T13:00:18Z | DO | revision r1 | attempt n=1/3 | short-fixture RSS series (10/30/60/120 s) complete; peak RSS 16192-17476 KiB, all exit 0, all under 280 s | .pdca/issues-milestone-1.0/rv1/item10/rss_short/run_120s.log
- 2026-11-08T13:00:18Z | DO | revision r1 | attempt n=1/3 | measurements.md + SCALE-CAVEAT.md + MANIFEST.md written; E_boundary max_abs=0.0 RMSE=0.0; residual 1.49e-8 | .pdca/issues-milestone-1.0/rv1/item10/measurements.md
- 2026-11-08T13:04:48Z | ACT | revision r1 | attempt n=1/3 | item #10 finalized incomplete (terminal=execution): CHECK not accepted (AC-10-2 unverified; E10-LONG-RSS blocking, E10-REPORT-CONSISTENCY medium); paths rolled back to 3c1eec0; tree clean; no commit; patch preserved at .pdca/issues-milestone-1.0/rv1/item10/incomplete-patch/ | .pdca/issues-milestone-1.0/rv1/item10/incomplete-patch/MANIFEST.md
- 2026-11-09 | ACT | revision r1 | attempt n=1/3 | item #10 accepted by explicit user instruction; committed b4cb290ad1cf864f3edc76e3e5bf8e0ad1e93ccb | .pdca/issues-milestone-1.0/rv1/item10/commit-verify/
