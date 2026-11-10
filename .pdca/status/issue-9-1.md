# Cycle status: issue-9 / cycle 1

## Current state

- task: `issue-9` · form: `cycle` · specialized skill: `pdca-coder`
- mode: `autonomous`
- cycle: `N=1` · revision: `r=1` · attempt: `n=2`
- scope: item #9 of collection `issues-milestone-1.0` (model inference parameters)
- status path: `.pdca/status/issue-9-1.md`
- goal: `feat: model inference parameters (segment, overlap, batch, pitch shift, denoise, TTA)` — overlap/step configurable and a sweep test shows expected pass counts; defaults reproduce current numbers when unset.
- phase: `ACT` · record: `completed` · outcome: `accepted at n=2 (CHECK verdict accepted; all criteria met; all four defect keys D9-PITCH/D9-BATCH/D9-CONFIG-ERROR/D9-FADE-RANGE refuted); local acceptance for item #9 only — not collection completion`
- source: `collection entry PLAN (r1) + item PLAN (pdca-planner)` · context refs: `.pdca/status/collection-issues-milestone-1.0.md`
- criteria: `AC-9 = verbatim "Done when" of issue #9 (see §PLAN record r1)`

## Original task

Run the collection of all 11 open issues #1–#11 of `AlexeyShirshov/candle-roformer` (milestone 1.0): each item ends either accepted+committed against its own issue criteria, or with a documented terminal outcome `blocked`/`infeasible` with evidence. Autonomous mode, no push, no branch switch, no history rewrite, no download/bundling/caching of model weights, no faked PASS.

Item brief (verbatim): #9 `feat: model inference parameters (segment, overlap, batch, pitch shift, denoise, TTA)` — Done when: overlap/step configurable and a sweep test shows expected pass counts; defaults reproduce current numbers when unset.

## PLAN record

### r1

# PLAN (элемент коллекции `#9`, ревизия `r1`)

## 1. Форма, специализированный навык и входной объём (Entry scope)

- **Форма:** `cycle` (обычный цикл PDCA).
- **Специализированный навык:** `pdca-coder`.
- **Входной объём:** полная цель #9 — типизированный объект конфигурации инференса по семействам моделей с эталонными значениями по умолчанию, параметризованный OLA/инференс-адаптер вместо жёстко заданных констант (`chunk = hop*800`, `step = chunk/2`, `fade = chunk/10`), поддержка `pitch_shift` и `denoise`/`TTA` для поддерживающих их семейств, воспроизведение текущих числовых результатов по умолчанию и подтверждение ожидаемого числа проходов на sweep-проверке.
- **Привязка к снимку:** рабочее дерево `/home/alex/sources/candle-roformer`, текущий HEAD на момент DO элемента #9; доказательства в этом же checkout.

### Пользовательские критерии приёмки (verbatim)
- **AC-1 (из `Missing`):** `a typed inference-config object per model family with reference defaults; overlap/step wired into the OLA adapter instead of the constants (chunk = hop*800, step = chunk/2, fade = chunk/10); pitch_shift and denoise/TTA where the family supports them.`
- **AC-2 (из `Done when`):** `overlap/step configurable and a sweep test shows expected pass counts; defaults reproduce current numbers when unset.`

## 2. Метод измерения (Measurement method)

1. **Число проходов OLA (`pass_count`) при варьировании `overlap`/`step`:**
   - Метрика: число вызовов обработчика окна внутри OLA за один прогон сигнала длины `N` при `chunk` C≥1 и `step` S≥1 (при задании через `overlap` K: `S = max(1, floor(C/K))`), единица `usize`.
   - База: `pass_count(0,C,S)=0`; для `N>0`: `pass_count(N,C,S) = ceil(N/S) = floor((N+S-1)/S)`; при `denoise`/`TTA` число вызовов модели кратно числу аугментационных проходов на чанк.
   - Фильтры: все вызовы per-chunk обработчика от входа в `ola` до возврата тензора; конфигурационные проверки с ошибкой валидации исключаются.
   - Сопоставимость: фиксированная `N > C`, фиксированный `C`, серия `overlap`/`step`; уменьшение `step`/рост `overlap` строго монотонно увеличивает `pass_count`.
   - Порог: точное совпадение (расхождение 0) наблюдаемого числа вызовов с аналитическим на всём sweep.
2. **Воспроизведение текущих чисел по умолчанию:**
   - Метрики: производные целочисленные `chunk`, `step`, `fade` (в сэмплах) и `pass_count`; выходной тензор `[stems, C, N]` в `f32`.
   - База: текущие формулы `src/separate.rs:9-18,75-76` — `chunk = hop.max(1) * 800` (=352800 при hop=441), `step = (chunk/2).max(1)` (=176400), `fade = (chunk/10).max(1)` (=35280).
   - Сопоставимость: тот же вход `[C,N]`, детерминированный CPU-прогон, конфиг без `inference:` против явного вызова со старыми константными правилами.
   - Порог: для `(chunk, step, fade, pass_count)` — точное равенство; для выходного тензора — `max_abs_diff == 0.0`.

## 3. Ограничения и допущения

- Только `/home/alex/sources/candle-roformer`; сторонние репозитории недоступны для записи. Запрещены `git push`, переключение веток, переписывание истории, скачивание весов, правка `AGENTS.md`, `LICENSE`, `docs/superpowers/**`, существующих записей `.pdca`.
- Существующие `configs/*.yaml` (только корневой `model:`) должны парситься без ошибок и получать эталонные дефолты инференса.
- Обратная совместимость `CANDLE_ROFORMER_CHUNK_HOPS`: при незаданном явном размере чанка сохраняется `hop.max(1) * hops` (по умолчанию 800).
- Допущение: оба семейства крейта (`BSRoformer`, `MelBandRoformer`) — STFT/RoFormer-семейства с общим конвейером поверх OLA; типизированный конфиг даёт фабрику дефолтов по семейству (`chunk_hops = 800`/`dim_t = 801`, `num_overlap = 2` → `step = chunk/2`, `fade = chunk/10`, `batch_size = 1`, `pitch_shift = 0.0`, `denoise = false`, `tta = false`).

## 4. Закрытие пробелов

1. **`defaults reproduce current numbers when unset`** → Evidence + measurement spec: текущие числа заданы `src/separate.rs:9-18,75-76`; при отсутствии секции `inference:`/`Default` вычисляются те же `(chunk, step, fade)`, тот же `pass_count` и идентичный (`max_abs_diff == 0.0`) выходной тензор OLA.
2. **`a sweep test shows expected pass counts`** → OLA итерируется по окнам с шагом `step` (`src/separate.rs:57-137`); для `N>0` и `S = step.max(1)` (`или max(1, floor(chunk/overlap))`) `pass_count = ceil(N/S)`; счётчик вызовов в OLA делает число проходов строго проверяемым целым.
3. **`pitch_shift and denoise/TTA where the family supports them`** → оба семейства идут через аудио-интерфейс `[C,N] -> [stems,C,N]`; ничего не объявляется вне объёма: `denoise` — полуразность прямого и инвертированного прогона `(out(mix) - out(-mix))/2`; `TTA` — аугментация инверсией фазы/перестановкой стереоканалов с обратным приведением и усреднением; `pitch_shift` — сдвиг в полутонах ресемплингом входа и обратным ресемплингом к исходной длине `N`; при дефолтах (`pitch_shift = 0`, `denoise = false`, `tta = false`) дополнительные преобразования не выполняются, сохраняя побитовую идентичность.

## 5. Минимальное решение и альтернативы

1. Ввести в `src/config.rs` (и реэкспортировать) типизированную структуру параметров инференса с опциональной секцией `inference:` в YAML и эталонными дефолтами (`segment`/`chunk` или `dim_t`/`chunk_hops = 800`, `num_overlap = 2` / `step = chunk/2`, `fade = chunk/10`, `batch_size = 1`, `pitch_shift = 0.0`, `denoise = false`, `tta = false`).
2. Параметризовать OLA и разделение в `src/separate.rs` (и вызовы в `src/separator.rs`) вместо констант `chunk/2` и `chunk/10`, сохранив существующие точки входа как обёртки с дефолтной конфигурацией.
3. Реализовать `batch_size`, `denoise`, `tta`, `pitch_shift` в конвейере `src/separate.rs` и валидацию некорректных значений.
- Отклонено: только поля конфига без подключения в `src/separate.rs` (нарушает `pitch_shift and denoise/TTA where the family supports them`); сторонний тяжёлый крейт ресемплинга (не нужен, добавляет зависимость).

## 6. Передача в DO и метаданные расписания

- Границы для DO: не ломать публичный API и парсинг существующих `configs/*.yaml`; не удалять поддержку `CANDLE_ROFORMER_CHUNK_HOPS`; не править `AGENTS.md`, `LICENSE`, `docs/superpowers/**`, `.pdca/**`; не скачивать веса; один атомарный Conventional-Commits коммит только для файлов элемента после приёмки.
- Поверхность: `src/config.rs`, `src/separate.rs`, `src/separator.rs`, `src/lib.rs`, `tests/core_tests.rs`.

## 7. Риски и условия остановки

- Риск: нарушение обратной совместимости 41 теста при изменении сигнатур — сохранить существующие сигнатуры с дефолтной конфигурацией или добавить параметризованные варианты.
- Риск: краевые случаи длины при `pitch_shift != 0` — выход после обратного ресемплинга точно приводится к `N`.
- Остановка: некомпилируемость или нерегрессируемая регрессия тестов в пределах 3 попыток; противоречие требований → PLAN/эскалация.

## 8. Уверенность

- Подтверждено: текущая реализация OLA (`src/separate.rs:6-137`), `SeparatorConfig` (`src/config.rs:6-145`), эталонные параметры `docs/provenance/v7.yaml:70-73`, отсутствие `segment_size`/`overlap` в `src/` и тестов на число проходов.
- Допущение: конкретные имена Rust-типов/полей не заданы буквально и выбираются DO.

## Approval record

`not applicable: autonomous mode`

## DO result

DO correction (attempt n=2): all four CHECK defects addressed:
- D9-PITCH: pitch shift now resamples input to shifted rate, runs inference on that shifted-rate signal, then resamples output back to original length/rate (src/separate.rs, `separate_with_config` pitch branch); default 0.0 skips. Test `pitch_shift_changes_output` asserts output length preserved and max_abs_diff > 0.
- D9-BATCH: `batch_size` consumed via new `ola_with_batch` grouping `batch_size` chunks into one model batch and splitting results; `run` handles batch dimension `(b, channels, chunk)`; default `batch_size=1` bit-identical. Test `batch_size_changes_model_invocations` asserts fewer calls and observed batch dim > 1.
- D9-CONFIG-ERROR: `Separator::load` now propagates `InferenceConfig::from_yaml_file` error instead of `unwrap_or_default()` (src/separator.rs:59); defaults only when section absent. `from_yaml_str` validates parsed config. Tests `inference_config_yaml_absent_defaults`, `inference_config_yaml_malformed_errors`.
- D9-FADE-RANGE: validation rejects `fade_size > segment_size` with "fade must be <= chunk size" (src/config.rs validate); `ola_with_batch` clamps fade defensively to avoid unsigned underflow. Test `inference_config_invalid_fade_range`.
Pattern sweep: only `unwrap_or_default()` on explicit user config was separator.rs:59 (fixed); config.rs:280 remains intentional absent-section default. Only unchecked unsigned subtraction on configurable sizes was `chunk - fade` in src/separate.rs:219 (now guarded by validation + clamp).

Evidence:
- build: cargo build exit 0; log `.pdca/issues-milestone-1.0/rv1/attempt2/cargo-build.log`.
- tests: cargo test exit 0; 51 total (11 lib + 22 core + 3 library_api + 11 s1_spikes + 3 whisper + 1 doc), 0 failed; log `.pdca/issues-milestone-1.0/rv1/attempt2/cargo-test.log`.
- targeted: cargo test --test core_tests exit 0; 22 passed; log `.pdca/issues-milestone-1.0/rv1/attempt2/cargo-test-core_tests.log`.

Unit states: U-9.1 config type — done (fade validation added); U-9.2 OLA wiring — done (ola_with_batch); U-9.3 pitch_shift/denoise/TTA — done (pitch placement fixed, batched run); U-9.4 backward compat wrappers — done; U-9.5 tests — done (5 new). No blockers. Candidate awaits independent CHECK.

## CHECK record

CHECK verdict: `accepted` (independent pdca-check, N=1, r=1, n=2).

Per-criterion matrix:
- AC-1 (typed inference-config object per model family with reference defaults; overlap/step wired into OLA adapter; pitch_shift and denoise/TTA where supported): `met` — `InferenceConfig` in src/config.rs with reference defaults (chunk_hops=800, num_overlap=2, batch_size=1, pitch_shift=0.0, denoise=false, tta=false); `ola_with`/`ola_with_batch` consume overlap/step/fade; pitch_shift/denoise/TTA implemented in src/separate.rs.
- AC-2 (overlap/step configurable; sweep test shows expected pass counts; defaults reproduce current numbers when unset): `met` — `ola_pass_count_sweep` (probes/core_tests.rs) exact match on all 11 step/overlap cases; `defaults_reproduce_current_numbers` shows chunk=352800, step=176400, fade=35280 and max_abs_diff==0.0.

All four defect keys refuted at n=2:
- D9-PITCH: pitch shift resamples input to shifted rate, runs inference, resamples output back to original length/rate; default 0.0 skips; `pitch_shift_changes_output` asserts length preserved and max_abs_diff > 0.
- D9-BATCH: `batch_size` consumed via `ola_with_batch` grouping batch_size chunks; `run` handles (b, channels, chunk); default batch_size=1 bit-identical; `batch_size_changes_model_invocations` asserts fewer calls and batch dim > 1.
- D9-CONFIG-ERROR: `Separator::load` propagates `InferenceConfig::from_yaml_file` error (src/separator.rs:59); defaults only when section absent; `inference_config_yaml_absent_defaults`, `inference_config_yaml_malformed_errors`.
- D9-FADE-RANGE: validation rejects fade_size > segment_size with "fade must be <= chunk size" (src/config.rs validate); `ola_with_batch` clamps fade defensively; `inference_config_invalid_fade_range`.

Evidence pointers:
- build: cargo build exit 0; log `.pdca/issues-milestone-1.0/rv1/attempt2/cargo-build.log`.
- tests: cargo test exit 0; 51 total (11 lib + 22 core + 3 library_api + 11 s1_spikes + 3 whisper + 1 doc), 0 failed; log `.pdca/issues-milestone-1.0/rv1/attempt2/cargo-test.log`.
- targeted: cargo test --test core_tests exit 0; 22 passed; log `.pdca/issues-milestone-1.0/rv1/attempt2/cargo-test-core_tests.log`.
- CHECK evidence pack: `.pdca/issues-milestone-1.0/rv1/checkpack/` (MANIFEST.md, run-evidence.md, probes/observed.md).
- non-default path probe: exit 0; see probes/observed.md.
- clippy: no declared gate; skipped.

Source reference: CHECK evidence pack `.pdca/issues-milestone-1.0/rv1/checkpack/MANIFEST.md` (run-cargo-test.log sha b244fd4060c876af7852ca7b21ca2451b9357a3726c13546ded4fff37ac6860b for core_tests).

## ACT record

ACT outcome: **accepted** (local acceptance for item #9 only).

- Scope closed: issue #9's own criteria (AC-1, AC-2) — overlap/step configurable with a sweep test showing expected pass counts; defaults reproduce current numbers when unset.
- CHECK verdict: accepted, all criteria met; all four defect keys (D9-PITCH, D9-BATCH, D9-CONFIG-ERROR, D9-FADE-RANGE) refuted at n=2.
- Counters: N=1, r=1, n=2 (of 3 attempts).
- Evidence root: `.pdca/issues-milestone-1.0/rv1/` (attempt2 logs; checkpack evidence).
- This is a **local** acceptance: not collection completion, not `done` for the collection. Item #9 marked `done`/`finalized` in the collection record; next allowed item #10.
- Limitations: clippy not run (no declared gate); batch_size field present in non-default probe but OLA loop consumes it via ola_with_batch (verified by test). No push, no branch switch, no history rewrite performed.
- Handoff: single authorized commit of the five item files; then collection record update; then item #10.

## Revision history

`none`

## Defect history

- `D9-PITCH` | observed r1 n=1 | fix applied r1 n=2 | refuted at n=2 (CHECK accepted); evidence: src/separate.rs pitch branch + tests/core_tests.rs::pitch_shift_changes_output
- `D9-BATCH` | observed r1 n=1 | fix applied r1 n=2 | refuted at n=2 (CHECK accepted); evidence: src/separate.rs ola_with_batch + tests/core_tests.rs::batch_size_changes_model_invocations
- `D9-CONFIG-ERROR` | observed r1 n=1 | fix applied r1 n=2 | refuted at n=2 (CHECK accepted); evidence: src/separator.rs:59 propagation + tests/core_tests.rs::inference_config_yaml_malformed_errors
- `D9-FADE-RANGE` | observed r1 n=1 | fix applied r1 n=2 | refuted at n=2 (CHECK accepted); evidence: src/config.rs validate + tests/core_tests.rs::inference_config_invalid_fade_range

## Changed files

- `src/config.rs` → added fade-range validation (`fade must be <= chunk size`); `from_yaml_str` validates parsed config
- `src/separate.rs` → pitch shift placement fixed (shift input → inference on shifted rate → map outputs back); `ola_with_batch` consumes `batch_size`; `ola_with`/`ola` delegate with batch=1; defensive fade clamp
- `src/separator.rs` → propagate `InferenceConfig::from_yaml_file` error (no `unwrap_or_default`)
- `tests/core_tests.rs` → 5 new tests (pitch_shift_changes_output, batch_size_changes_model_invocations, inference_config_yaml_absent_defaults, inference_config_yaml_malformed_errors, inference_config_invalid_fade_range)
- `.pdca/issues-milestone-1.0/rv1/attempt2/*` → evidence logs (build/test)
- `.pdca/status/issue-9-1.md` → this cycle's status (modified)

## Git state

`status path: .pdca/status/issue-9-1.md — created+modified this cycle · untracked (durable on disk, not committed); baseline HEAD 1f823c4 → now committed at 3c1eec0; task changes committed (5 files: src/config.rs src/separate.rs src/separator.rs src/lib.rs tests/core_tests.rs); tree clean after commit`

## Progress log

- 2026-11-08T08:47:41Z | PLAN | revision r1 | attempt n0 | item PLAN ready (plan_state=ready); no DO yet | this file
- 2026-11-08T09:05:00Z | DO | revision r1 | attempt n1 | DO started | this file
- 2026-11-08T09:12:00Z | DO | revision r1 | attempt n1 | implementation complete (config/separate/separator/lib + tests) | this file
- 2026-11-08T09:15:00Z | DO | revision r1 | attempt n1 | cargo test exit 0; total 46 (41+5); candidate ready for CHECK | .pdca/issues-milestone-1.0/rv1/do-test4.log`
- 2026-11-08T09:27:08Z | DO | revision r1 | attempt n1 | CHECK evidence pack assembled (read-only; no candidate file changed) | .pdca/issues-milestone-1.0/rv1/checkpack/ (MANIFEST.md)
- 2026-11-08T10:00:00Z | DO | revision r1 | attempt n2 | correction started (D9-PITCH, D9-BATCH, D9-CONFIG-ERROR, D9-FADE-RANGE) | this file
- 2026-11-08T10:30:00Z | DO | revision r1 | attempt n2 | implementation complete (config/separate/separator/tests) | this file
- 2026-11-08T10:35:00Z | DO | revision r1 | attempt n2 | cargo test exit 0; total 51 (11+22+3+11+3+1); candidate ready for CHECK | .pdca/issues-milestone-1.0/rv1/attempt2/cargo-test.log
- 2026-11-08T09:38:00Z | DO | revision r1 | attempt n2 | n=2 candidate snapshot persisted | .pdca/issues-milestone-1.0/rv1/attempt2/candidate/ (MANIFEST.md)
- 2026-11-08T11:00:00Z | CHECK | revision r1 | attempt n2 | CHECK verdict accepted; all criteria met; all four defect keys refuted | .pdca/issues-milestone-1.0/rv1/checkpack/ (MANIFEST.md, run-evidence.md)
- 2026-11-08T11:15:00Z | ACT | revision r1 | attempt n2 | ACT closed: accepted (local); commit 3c1eec0 feat: typed inference parameters with configurable overlap/step (#9); 5 files | this file
