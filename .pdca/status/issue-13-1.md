# Cycle status: issue-13 / cycle 1

## Current state

- task: `issue-13` · form: `cycle` · specialized skill: `pdca-coder`
- mode: `autonomous`
- cycle: `N=1` · revision: `r=1` · attempt: `n=3/3`
- scope: `item #13 of collection issues-milestone-1.0-r2` · status path: `.pdca/status/issue-13-1.md`
- goal: `feat: library-level level processing (normalization, amplification, spectral inversion)` — the §PLAN record and §Original task are authoritative
- phase: `ACT` · record: `completed` · outcome: `incomplete/STOP — not accepted`
- blocking defect: `13-D6`
- source: `collection entry PLAN (r1) + item PLAN (pdca-planner)` · context refs: `collection .pdca/status/collection-issues-milestone-1.0-r2.md; issue #13`
- criteria: `AC-1, AC-2 — exact text in §PLAN record r1 (source: planner PLAN)`
- patch: `sha256 6f6d27c706db1d1dfee65124b76974237acfdda1634eedbedb638e132c340df8` (attempts/n3/diff.patch)

## Original task

Title: `feat: library-level level processing (normalization, amplification, spectral inversion)`

Body:
```markdown
Follow-up split out of the level-processing issue that moved to the CLI repo (`AlexeyShirshov/karaoke-cli#7`). The CLI flag surface (`--normalization`, `--amplification`, `--invert_spect` semantics and defaults) is tracked there; the **library** part belongs to this crate.

`audio-separator` applies `--normalization 0.9` by default, plus `--amplification` and `--invert_spect`. This crate touches neither the input level nor the output level, so levels and peak values differ from the reference tool even when the separation itself is identical.

**Missing**

- Peak normalization of input and output to a configurable threshold (default 0.9 to match the reference).
- Amplification of quiet inputs below a threshold (default 0.0 = off).
- Spectral inversion of the secondary stem instead of the waveform residual (`invert_spect`).
- A decision on residual exactness: normalization must be applied consistently to both stems, or the `vocals + instrumental = input` invariant breaks.

**Done when**

- The invariant is re-stated precisely in the docs, with a test for both "exact residual" and "normalized" modes.
- Library-level API implements normalization, amplification and spectral inversion with the reference defaults.

Cross-repo: `AlexeyShirshov/karaoke-cli#7` (CLI flags, defaults and help text).
```

## PLAN record

### r1

# План PDCA — Элемент коллекции #13 (`feat: library-level level processing (normalization, amplification, spectral inversion)`)

## 1. Форма, специализированный навык (overlay) и входной объём (entry scope)

- **Форма коллекции (ограничение пользователя):** `collection` (`issues-milestone-1.0-r2`, элемент #13).
- **Форма элемента:** `cycle` (обычный цикл PDCA).
- **Специализированный навык (overlay):** `pdca-coder` (`coder`).
- **Обоснование формы (в одну строку):** Метод реализации библиотечной обработки уровней (пиковая нормализация входа/выхода с порогом по умолчанию `0.9`, усиление тихих сигналов с порогом по умолчанию `0.0`, спектральная инверсия вторичного стема `invert_spect` через существующий STFT/iSTFT-тракт и явная фиксация инварианта суммы стемов) полностью определён сигнальной математикой и референсным поведением `audio-separator`, а объём работ строго ограничен API/конфигурацией, DSP-обработкой, документацией и тестами внутри одного крейта `candle-roformer`.
- **Entry scope (входной объём цикла):** Библиотечный API крейта `candle-roformer` с поддержкой нормализации пиков (по умолчанию `0.9`), усиления тихих входов (по умолчанию `0.0 = off`) и спектральной инверсии вторичного стема (`invert_spect`), обновлённая документация с точной формулировкой инварианта суммы стемов и тесты для обоих режимов — «exact residual» и «normalized».
- **Привязка к снимку (Re-anchor):** Репозиторий `/home/alex/sources/candle-roformer`, ветка `main`, коммит `b4cb290` (чистое дерево).
- **Расположение доказательств (Evidence location):** Первичный чекаут `/home/alex/sources/candle-roformer`, каталог доказательств элемента `.pdca/issues-milestone-1.0-r2/rv1/item-13/`.

---

## 2. Цель и критерии приёмки (Result Contract)

### Исходная постановка задачи (verbatim)

Title: `feat: library-level level processing (normalization, amplification, spectral inversion)`

Body:
```markdown
Follow-up split out of the level-processing issue that moved to the CLI repo (`AlexeyShirshov/karaoke-cli#7`). The CLI flag surface (`--normalization`, `--amplification`, `--invert_spect` semantics and defaults) is tracked there; the **library** part belongs to this crate.

`audio-separator` applies `--normalization 0.9` by default, plus `--amplification` and `--invert_spect`. This crate touches neither the input level nor the output level, so levels and peak values differ from the reference tool even when the separation itself is identical.

**Missing**

- Peak normalization of input and output to a configurable threshold (default 0.9 to match the reference).
- Amplification of quiet inputs below a threshold (default 0.0 = off).
- Spectral inversion of the secondary stem instead of the waveform residual (`invert_spect`).
- A decision on residual exactness: normalization must be applied consistently to both stems, or the `vocals + instrumental = input` invariant breaks.

**Done when**

- The invariant is re-stated precisely in the docs, with a test for both "exact residual" and "normalized" modes.
- Library-level API implements normalization, amplification and spectral inversion with the reference defaults.

Cross-repo: `AlexeyShirshov/karaoke-cli#7` (CLI flags, defaults and help text).
```

### Запрошенный и поставляемый результат

- **Requested result (запрошенный результат):** Библиотечный API крейта `candle-roformer`, реализующий пиковую нормализацию входа и выхода (порог по умолчанию `0.9`), усиление тихих входов ниже порога (по умолчанию `0.0` = выключено) и спектральную инверсию вторичного стема (`invert_spect`, по умолчанию выключено), вместе с точной формулировкой инварианта остатка (`vocals + instrumental = input`) в документации и тестами для режимов «exact residual» и «normalized».
- **Delivered result (поставляемый результат):** Библиотечный API крейта `candle-roformer`, реализующий пиковую нормализацию входа и выхода (порог по умолчанию `0.9`), усиление тихих входов ниже порога (по умолчанию `0.0` = выключено) и спектральной инверсии вторичного стема (`invert_spect`, по умолчанию выключено), вместе с точной формулировкой инварианта остатка (`vocals + instrumental = input`) в документации и тестами для режимов «exact residual» и «normalized» (тот же объект).

### Критерии приёмки (воспроизведены дословно из раздела `Done when` постановки пользователя)

- **`AC-1`:** `The invariant is re-stated precisely in the docs, with a test for both "exact residual" and "normalized" modes.`
- **`AC-2`:** `Library-level API implements normalization, amplification and spectral inversion with the reference defaults.`

*(Пользователь явно задал набор критериев в `Done when`; согласно контракту PLAN они сохранены verbatim без добавления новых критериев и оцениваются в контексте обязательных требований из раздела `Missing` исходного issue).*

## 3. Метод измерения (Measurement method)

Задача функционально-контрактная, однако включает численные пороги и инварианты из условия задачи:

- **Метрика пикового уровня (для нормализации и усиления):** максимальная абсолютная амплитуда сигнала $\text{peak}(x) = \max_i |x_i|$ (безразмерная величина в линейной шкале амплитуд `f32`).
  - **Знаменатель и фильтры:** все отсчёты выходного или входного аудио-буфера (по всем каналам и кадрам); для нулевого сигнала ($\text{peak}(x) == 0$) деление на ноль не производится (сигнал остаётся нулевым).
  - **Пороги и их происхождение:** порог нормализации по умолчанию `0.9` (задан в задаче по референсу `audio-separator`); порог усиления по умолчанию `0.0` = выключено (задан в задаче по референсу `audio-separator`).
- **Метрика точности остатка (residual exactness):** поточечное отклонение суммы первичного и вторичного стемов от входного сигнала $\max_i |(v_i + inst_i) - x_i|$ в режиме «exact residual» (без независимой пост-нормализации каждого стема и без спектральной инверсии) в пределах численной точности `f32`, против режима «normalized», где пик каждого ненулевого выходного стема приводится к настроенному порогу нормализации (или согласованному правилу нормализации, явно зафиксированному в документации).
- **Условия сопоставимости:** одинаковый входной буфер, частота дискретизации и параметры конфигурации разделения при сравнении режимов «exact residual» и «normalized», а также волнового вычитания и спектральной инверсии (`invert_spect`).

## 4. Ограничения и допущения (Constraints & Assumptions)

### Ограничения (Constraints)

- **Автономный режим:** без вопросов пользователю и без интерактивного гейта одобрения плана.
- **Границы репозитория:** все изменения строго внутри `/home/alex/sources/candle-roformer`; любые действия или запись в `/home/alex/sources/karaoke-cli` (где отслеживаются CLI-флаги `karaoke-cli#7`) запрещены.
- **Git и GitHub:** не более одного атомарного коммита Conventional Commits на ветке `main` при приёмке элемента, включающего только файлы этого элемента; `git push`, переключение веток, `rebase`, `reset --hard`, `--force` запрещены; мутации через `gh` (close/edit/comment/create/transfer) запрещены.
- **Защищённые пути:** не изменять `AGENTS.md`, `LICENSE`, `docs/superpowers/**`, а также чужие артефакты `.pdca/status/candle-roformer-impl.md`, `.pdca/candle-roformer-impl/**`, `.pdca/status/collection-issues-milestone-1.0.md`, `.pdca/status/issue-9-1.md`, `.pdca/status/issue-10-1.md`, `.pdca/issues-milestone-1.0/**`.
- **Модели и ресурсы:** запрещено скачивание, бандлинг или кэширование весов моделей; на хосте CPU-only (`x86_64`, без CUDA/Metal), локальный чекпойнт `/home/alex/models/MelBandRoformer_fp32.safetensors` доступен только для чтения.
- **Лимиты времени:** любой вызов shell ≤ 300 с (`cargo build|test|check` ≤ 600 с), `sleep` ≤ 60 с, без фоновых процессов.

### Допущения (Assumptions)

- **A-1 (Семантика референсного `audio-separator` для обработки уровней):**
  - Пиковая нормализация (`normalization`, по умолчанию `0.9`, при отключении/значении `None` или `<= 0.0` / выключенном режиме — без масштабирования пиков): масштабирует буфер с коэффициентом `threshold / max_abs`, если `max_abs > 0`. В референсе `audio-separator` нормализация применяется к входному миксу перед инференсом и к каждому выходному стему при формировании результата.
  - Усиление (`amplification`, по умолчанию `0.0` = выключено): если задан положительный порог и пик входного сигнала `0 < max_abs < amplification_threshold`, входной сигнал масштабируется до порога (или в соответствии с проверкой порога тихого сигнала до разделения).
  - Спектральная инверсия (`invert_spect`, по умолчанию `false`): вместо вычисления вторичного стема во временной области (`input - primary`) вторичный стем вычисляется вычитанием комплексной спектрограммы (STFT первичного стема из STFT входного сигнала с последующим iSTFT).
- **A-2 (Соотношение инварианта `vocals + instrumental = input` и нормализации):**
  - Когда независимая нормализация выходных стемов включена (референсный дефолт `0.9`) или включена `invert_spect`, поточечное тождество `vocals + instrumental == input` в исходном масштабе входа не сохраняется (каждый стем масштабируется до собственного пика `0.9`); когда нормализация отключена ("exact residual" режим, без `invert_spect` и без изменения масштаба), вторичный стем определяется как точный волновой остаток `input - vocals`, и инвариант `vocals + instrumental = input` выполняется точно. Оба режима должны быть явно поддержаны в API и точно описаны в документации.

## 5. Закрытие пробелов (Gaps)

| Пробел в постановке | Способ закрытия | Решение |
| --- | --- | --- |
| Как совместить референсный дефолт нормализации `0.9` с существующими тестами и документацией, проверяющими точный остаток `vocals + instrumental = input`? | Evidence + Assumption (`A-2`) | В самом issue явно указано: «A decision on residual exactness: normalization must be applied consistently to both stems, or the `vocals + instrumental = input` invariant breaks» и в `Done when`: «The invariant is re-stated precisely in the docs, with a test for both "exact residual" and "normalized" modes». Документация явно разделяет режим точного остатка (без нормализации/спектральной инверсии) и нормализованный режим (по умолчанию `0.9`), а API позволяет явно выбрать оба режима. |
| Как задаются параметры обработки уровней в публичном API крейта? | Assumption (`A-1`) | Через конфигурационную структуру параметров разделения (например, расширение `SeparationParams` / соответствующего конфига в `src/config.rs` и использование в `separate.rs` / `separator.rs`) с референсными значениями по умолчанию (`normalization = 0.9`, `amplification = 0.0`, `invert_spect = false`) и конструкторами/билдерами для явного включения режима точного остатка. |
| Допустимы ли некорректные значения порогов (`NaN`, отрицательные, бесконечные)? | Assumption (`A-1`) | Валидация параметров конфигурации должна отклонять невалидные значения (например, `NaN`, бесконечность, отрицательные пороги или пороги вне допустимого диапазона) через типизированную ошибку крейта, а также безопасно обрабатывать нулевой (тихий) буфер без `NaN`/`Inf`. |

## 6. Минимальное решение и альтернативы (Minimal solution & alternatives)

### Рассмотренные альтернативы

1. **Альтернатива 1 (выбрана): Встроить параметры обработки уровней (`normalization`, `amplification`, `invert_spect`) в существующую конфигурацию разделения (`SeparationParams` / DSP-пайплайн) с референсными дефолтами и явным режимом точного остатка.**
   - *Плюсы:* Единая поверхность настроек инференса и постобработки; автоматическая работа как в высокоуровневом `Separator`, так и в функциях разделения; чистые DSP-функции доступны для модульного тестирования без загрузки 912 МБ чекпойнта на каждом тесте.
2. **Альтернатива 2: Добавить только отдельные свободные функции пост-обработки в `dsp.rs`, не интегрируя их в пайплайн разделения.**
   - *Причина отказа:* Противоречит формулировке задачи («This crate touches neither the input level nor the output level, so levels and peak values differ from the reference tool even when the separation itself is identical» и «Library-level API implements normalization, amplification and spectral inversion with the reference defaults»).

### Выбранное минимальное решение

- Реализовать операции пиковой нормализации, порогового усиления тихого входа и спектральной инверсии вторичного стема (через STFT/iSTFT) с безопасной обработкой нулевого сигнала и проверкой валидности параметров.
- Интегрировать их в библиотечный API разделения с референсными значениями по умолчанию (`normalization: 0.9`, `amplification: 0.0`, `invert_spect: false`) и удобным способом отключения нормализации для режима точного остатка («exact residual»).
- Точно сформулировать инвариант `vocals + instrumental = input` в документации (указав, в каком режиме он выполняется поточечно, а как ведут себя уровни стемов в нормализованном режиме и при `invert_spect`) и покрыть тестами как «exact residual», так и «normalized» режим, а также усиление и спектральную инверсию.

## 7. Передача в DO (Plan-to-DO handoff)

Каждый запуск DO получает по ссылке:
- **Исходную задачу verbatim** (раздел 2 выше);
- **Критерии приёмки `AC-1`, `AC-2` verbatim** и метод измерения (разделы 2–3);
- **Ограничения и запреты** (раздел 4: только репозиторий `/home/alex/sources/candle-roformer`, запрет трогать `karaoke-cli`, запрет `git push` и `gh`-мутаций, запрет скачивания моделей, защита `AGENTS.md`, `LICENSE`, `docs/superpowers/**` и прошлых артефактов `.pdca/**`, лимиты времени shell ≤ 300 с / cargo ≤ 600 с);
- **Область (Scope):** публичный API и реализация обработки уровней в `src/`, документация крейта (`src/lib.rs` / doc-комментарии / `README.md` при наличии описания инварианта) и тесты в `tests/` (или модульные тесты в `src/`), с сохранением зелёного прогона всех существующих тестов крейта (при необходимости обновив тесты, которые неявно полагались на отсутствие дефолтной нормализации, так чтобы проверялись оба режима — «exact residual» и «normalized»).

## 8. Риски и условия остановки (Risks & Stop conditions)

### Риски

- **Риск 1 (Обратная совместимость существующих тестов на инвариант остатка):** Включение `normalization = 0.9` по умолчанию изменит пиковые значения выходных стемов в существующих тестах, если они вызывали API с параметрами по умолчанию и проверяли поточечное равенство `vocals + instrumental == input`.
  - *Митигация:* Проверить все существующие тесты; явно разделить и протестировать режим «exact residual» (нормализация отключена) и режим «normalized» (дефолтная нормализация `0.9`), как того требует `AC-1`.
- **Риск 2 (Деление на ноль на тишине):** Буфер из одних нулей при нормализации или усилении может дать `NaN`/`Inf`.
  - *Митигация:* При `max_abs == 0.0` (или `<= f32::EPSILON`) пропускать масштабирование, возвращая буфер без изменений.
- **Риск 3 (Несовпадение длины/граничных отсчётов при спектральной инверсии `invert_spect`):** STFT и iSTFT могут иметь паддинг на краях.
  - *Митигация:* Использовать согласованные параметры окна/паддинга и обрезку до исходной длины входного сигнала `num_samples` по каждому каналу.

### Условия остановки (Stop conditions)

- Достижение `PASS` по `AC-1` и `AC-2` на независимом CHECK с атомарным коммитом в `main`.
- Исчерпание лимита попыток (`n/3`) на один и тот же ключ дефекта → эскалация (`pdca-escalate`) или терминальный исход элемента коллекции с доказательствами.
- Обнаружение блокера окружения или нарушения неизменяемых ограничений прогона.

## 9. Уверенность и открытые вопросы (Confidence & Gaps)

- **Уверенность:** Высокая. Все операции лежат в сигнальной области (DSP + конфигурация API + документация + тесты), не требуют внешних зависимостей, GPU или новых весов моделей.
- **Верифицировано:** Структура крейта (`src/{lib.rs,config.rs,separate.rs,separator.rs,dsp.rs,...}`, `tests/`), чистое состояние git (`b4cb290`), точный текст issue #13 и разделение ответственности с `karaoke-cli#7`.
- **Остаётся допущением:** Точная сигнатура полей в `SeparationParams` / `src/config.rs` (решается на фазе DO в соответствии с текущим кодом `src/config.rs` and `src/dsp.rs`).

## Approval record

`not applicable: autonomous mode`

## DO result

`completed (n=2/3): correction attempt for 13-D2 and 13-D3. Unified file/streaming path: separate_file now decodes the whole input and delegates to in-memory separate(), so amplification/normalization/invert_spect and residual semantics apply regardless of sample rate. Qualified all unqualified residual guarantees in src/separator.rs and README.md. Added 3 same-rate streaming tests (default normalization peaks, amplification reconstruction, spectral inversion). Full suite rc=0 (77 tests: 22 unit + 22 core + 12 library_api + 11 s1 + 6 streaming + 3 whisper + 1 doctest); build rc=0. Single patch sha256 4e1b498b54be3371cdcfd5b9dbb8b2150dac2301aa0bbad551a116befa0d292c (7 files). Evidence: .pdca/issues-milestone-1.0-r2/rv1/item-13/ (build.log, build.rc, test.log, test.rc, diff.patch, IDENTITY.md, MANIFEST.md, notes.md, RUN_RECORD.md, status.porcelain, src-context/).`

### n=3

completed (n=3/3): restored bounded-memory streaming in `Separator::separate_file` (FrameBuf + OLA, chunk_duration drives OLA) with amplification (global peak pre-pass) and normalization (second pass over disk-backed raw stem buffers); fallback (non-matching sr / multi-stem / invert_spect) now decodes via `read_audio` (correct planar), fixing 13-D5. Added multi-block stereo regression test. README updated. Full suite rc=0 (78 tests: 22 unit + 22 core + 12 library_api + 11 s1 + 7 streaming + 3 whisper + 1 doctest); build rc=0. Patch sha256 6f6d27c706db1d1dfee65124b76974237acfdda1634eedbedb638e132c340df8. Evidence: .pdca/issues-milestone-1.0-r2/rv1/item-13/attempts/n3/ (build.log, build.rc, test.log, test.rc, diff.patch, IDENTITY.md, MANIFEST.md, notes.md, status.porcelain, d5-probe/).
## CHECK record

`last verdict: not accepted → DO (recheck; confirmed findings 13-D2 major/AC-2, 13-D3 major/AC-1; refuted 13-E1/13-E2/13-E3/13-D1). Correction attempt n=2/3 completed: unified file/streaming path applies level processing regardless of sample rate; residual guarantees qualified in docs; 3 same-rate streaming tests added; full suite rc=0 (77 tests); build rc=0; patch sha256 4e1b498b54be3371cdcfd5b9dbb8b2150dac2301aa0bbad551a116befa0d292c. Candidate awaiting independent CHECK.`

### Final CHECK (n=3/3) + mandatory expert review

not accepted → ACT (incomplete/STOP). Final CHECK after mandatory expert review (n=3/3): AC-1 met; AC-2 unmet (confirmed major defect 13-D6 — normalized secondary stem computed as N(p−N(y)) in memory vs N(p−y) in streaming). No genuine criteria defect found; no fourth attempt or PLAN referral authorized. 13-D4 remains unverified (no bounded-memory measurement). 13-D2, 13-D3, 13-D5 refuted within checked scope. Evidence: attempts/n3/diff.patch:436–443, 570–580, 653–660, 892–923, 981–1015; README.md:90–93. build rc=0 (build.log); test rc=0 (test.log; 78 tests).

## ACT record

> **Item #13: incomplete / STOP; not accepted.** Revision r=1 exhausted n=3/3. Mandatory expert review and independent CHECK found no genuine criteria defect; no fourth attempt or PLAN referral is authorized. AC-1 remains met; AC-2 remains unmet because confirmed major defect 13-D6 computes the normalized secondary stem as N(p−N(y)) in memory versus N(p−y) in streaming. Evidence: attempts/n3/diff.patch:436–443, 570–580, 653–660, 892–923, 981–1015. README’s shared-semantics claim is overstated (:90–93); existing tests do not establish normalized-path parity. Divergence magnitude is unmeasured. 13-D4 remains unverified: bounded-memory behaviour has no current measurement. 13-D2, 13-D3 and 13-D5 are refuted within their checked scope. Preserve 13-R1 recovery drift as non-product history, not a counted attempt. No PASS, staging or commit; collection item #13 counts as incomplete.

- terminal: `execution` (an execution failure, not a plan-terminal resolution); attempts: 3; N=1, r=1.
- commit: none — nothing staged or committed; the rejected candidate is preserved as `.pdca/issues-milestone-1.0-r2/rv1/item-13/attempts/n3/diff.patch` and `.pdca/issues-milestone-1.0-r2/rv1/item-13/incomplete-13.patch` (sha256 `6f6d27c7…5df8`), and the working tree was returned to HEAD for item #13's 7 paths.
- residual requirements: AC-2 unmet (13-D6); 13-D4 unverified; magnitude of the 13-D6 divergence unmeasured.
- next: none for this cycle — a further attempt needs a new authorized cycle (<N>+1) or an explicit new user instruction; re-running the unchanged failed criterion is not permitted here.

## Revision history

`none`

## Defect history

- `13-D2` (major, AC-2): file/streaming API ignored level processing. Confirmed. Fix applied in n=2/3: `Separator::separate_file` now decodes the whole input and delegates to `Separator::separate`, so amplification/normalization/invert_spect apply regardless of sample rate. Evidence: src/separator.rs (separate_file), tests/streaming_chunked.rs (stream_*).
- `13-D3` (major, AC-1): unqualified exact-residual claims in docs. Confirmed. Fix applied in n=2/3: qualified residual guarantees in src/separator.rs (lines 51-71, 95-109, 118-123, 263-268) and README.md. Evidence: notes.md AC-1.
- Refuted (not fixed, not regressed): `13-E1`, `13-E2`, `13-E3`, `13-D1` — tests, identity record and doc fix retained.

- `13-R1` (recovery drift, not a product defect, not a counted attempt): after the n=2/3 evidence freeze, three files were modified in the working tree without a status record and without build/test evidence; detected by identity mismatch on recovery, preserved (drift-unverified.patch, drift-unverified/, DRIFT.md) and the tree restored to the frozen candidate. Not a defect key for the attempt budget.

- `13-D4` (major, AC-2): bounded-memory streaming removed in n=2 (whole-input decode). Confirmed by CHECK. Fix applied in n=3: restored OLA streaming with chunk_duration, amplification via global peak pre-pass, normalization via second pass; fallback honest+documented. Evidence: src/separator.rs (separate_file), tests/streaming_chunked.rs.
- `13-D5` (major, AC-2): multi-block stereo planar concatenation bug (`all.extend(c)`). Probe on frozen candidate confirmed: exact-mode max residual 0.47466984 (required <1e-5), file-vs-inmem diff vocals 0.0621/instr 0.4705. Fix applied in n=3: fallback uses read_audio (correct planar); streaming path never concatenates planar chunks. Post-fix probe: max residual 2.98e-8, file-vs-inmem diff 0. Regression test `chunked_multiblock_stereo_residual` added. Evidence: attempts/n3/d5-probe/.

- `13-D6` (major, AC-2 + issue requirement that normalization be applied consistently to both stems): normalized secondary stem computed as N(p−N(y)) in memory versus N(p−y) in streaming. In-memory path normalizes the primary before subtracting (N(p − N(y))); streaming path stores the raw residual and normalizes both stems afterwards (N(p − y)). Correct expansion of the in-memory form is N((p−y)+(1−g)y), so the expert's (p−y)+(1−g)y describes the residual before the outer normalization. Magnitude **not measured**. Fix **not applied** (budget exhausted, n=3/3). Evidence: attempts/n3/diff.patch:436–443, 570–580, 653–660, 892–923, 981–1015; README.md:90–93. The suite did not catch it because (1) normalization tests check peaks only, and (2) exact-mode parity disables normalization.

- `13-D4` stays `unverified`: bounded-memory behaviour has no current measurement (no bounded-memory measurement recorded for the n=3 streaming path).

- Final statuses: `13-D2` refuted, `13-D3` refuted, `13-D5` refuted (within their checked scope).

## Changed files

- README.md — rolled back, not shipped (returned to HEAD)
- src/config.rs — rolled back, not shipped (returned to HEAD)
- src/dsp.rs — rolled back, not shipped (returned to HEAD)
- src/separate.rs — rolled back, not shipped (returned to HEAD)
- src/separator.rs — rolled back, not shipped (returned to HEAD)
- tests/library_api.rs — rolled back, not shipped (returned to HEAD)
- tests/streaming_chunked.rs — rolled back, not shipped (returned to HEAD)
- .pdca/issues-milestone-1.0-r2/rv1/item-13/* — evidence, ignored PDCA state (preserved, not an untracked git path)
- .pdca/issues-milestone-1.0-r2/rv1/item-13/incomplete-13.patch — preserved rejected candidate (sha256 6f6d27c7…)
- .pdca/status/issue-13-1.md (durable on disk, gitignored)

## Git state

`status path: .pdca/status/issue-13-1.md — modified this cycle (durable on disk, gitignored, not committed); HEAD 16033332c7c4f94f00575ed1e868651e9a270cda; no commit, nothing staged; item #13's 7 paths returned to HEAD; git status --porcelain empty; the rejected candidate preserved as attempts/n3/diff.patch and incomplete-13.patch (sha256 6f6d27c7…); .pdca/ is gitignored PDCA state, not an untracked git path.`

## Progress log

- `2026-11-09T05:46:17Z | PLAN | revision r1 | attempt n0 | item PLAN persisted (pdca-planner, form=cycle, overlay=pdca-coder); all acceptance criteria fixed as AC-1..AC-2`
- `2026-11-09T07:43:48Z | DO | revision r1 | attempt n1 | DO started (implementation in progress)`
- `2026-11-09T07:45:28Z | DO | revision r1 | attempt n1 | DO completed (build rc=0, test rc=0, 70 tests); single patch sha256 cc3e9c32...; evidence .pdca/issues-milestone-1.0-r2/rv1/item-13/`
- `2026-11-09T08:06:54Z | DO | revision r1 | attempt n=1 | DO re-gather | numerical API checks present and passing | exact residual max_abs_error=2.9802322e-8; normalized peaks vocals=0.9 instrumental=0.9; amplification before_peak=0.099999756 after_peak=0.9 maxerr=5.9604645e-8; spectral inversion maxdiff=0`
- `2026-11-09T08:13:52Z | DO | revision r1 | attempt n=1/3 | evidence re-gather completed (build rc=0, tests rc=0, 74); patch sha256 8572cce05aacde924477023522f10548b7f2759ceebff5df94971f2c4f9c1303; identity pre/post MATCH; src-context/ written`
- `2026-11-09T08:26:17Z | DO | revision r1 | attempt n=2/3 | correction for 13-D2/13-D3: unified file/streaming path; qualified residual docs; 3 same-rate streaming tests; build rc=0; tests rc=0 (77); patch sha256 4e1b498b54be3371cdcfd5b9dbb8b2150dac2301aa0bbad551a116befa0d292c`
- `2026-11-10T05:11:32Z | RECOVERY | revision r1 | attempt n=2/3 | Recovered after compaction/interruption — state restored from this file; phase DO-completed awaiting independent CHECK | .pdca/status/issue-13-1.md`
- `2026-11-10T05:19:22Z | RECOVERY | revision r1 | attempt n=2/3 | identity drift detected on recovery: working tree no longer matched the frozen candidate (README.md, src/separator.rs, tests/streaming_chunked.rs modified at 2026-11-09T13:55–13:58+05:00, unrecorded, unverified, no build/test evidence); drift preserved as drift-unverified.patch + drift-unverified/ + DRIFT.md; tree restored to the recorded candidate; restored identity verified: 7/7 file sha256 match MANIFEST.yes, git diff sha256 4e1b498b54be3371cdcfd5b9dbb8b2150dac2301aa0bbad551a116befa0d292c (frozen 4e1b498b…); fresh run build rc=0 tests rc=0 (77 tests) | .pdca/issues-milestone-1.0-r2/rv1/item-13/`

- `2026-11-10T05:46:57Z | DO | revision r1 | attempt n=3/3 | DO started (restoring bounded streaming + fixing 13-D5)`
- `2026-11-10T05:46:57Z | DO | revision r1 | attempt n=3/3 | DO completed (build rc=0, tests rc=0, 78); patch sha256 6f6d27c706db1d1dfee65124b76974237acfdda1634eedbedb638e132c340df8; evidence .pdca/issues-milestone-1.0-r2/rv1/item-13/attempts/n3/`
- `2026-11-10T06:02:09Z | CHECK | revision r1 | attempt n=3/3 | final CHECK after mandatory expert review: not accepted — AC-1 met, AC-2 unmet (13-D6 confirmed); no criteria defect; no fourth attempt | .pdca/status/issue-13-1.md`
- `2026-11-10T06:02:09Z | ACT | revision r1 | attempt n=3/3 | item #13 finalized incomplete/STOP; no commit; candidate preserved (incomplete-13.patch sha256 6f6d27c7…); tree returned to HEAD | .pdca/issues-milestone-1.0-r2/rv1/item-13/`
