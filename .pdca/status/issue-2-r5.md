# Cycle status: issue-2-r5 / cycle 1

## Current state

- task: `issue-2-r5` · form: `cycle` · specialized skill: `pdca-coder`
- mode: `autonomous`
- cycle: `N=1` · revision: `r=2` · attempt: `n=2/3`
- scope: collection item `#2` — MDX-Net (ONNX) separation on CPU from a caller-supplied, checksum-verified published checkpoint, with audio-separator 0.48.0 parity on the frozen fixture · status path: `.pdca/status/issue-2-r5.md`
- goal: `#2` separates a frozen stereo fixture on CPU with the published `UVR-MDX-NET-Inst_HQ_3.onnx` and matches audio-separator 0.48.0 within the user-approved tolerance — the §PLAN record and §Original task are authoritative
- phase: `ACT` · record: `completed` · outcome: `accepted` — CHECK 3 (`r2`/`n2`) returned **`accepted → ACT`**; before it, DO attempt `r2`/`n2` completed the candidate: the audit now carries a 149-row per-crate version/licence inventory of the `tract-onnx`+`sha2` closure (`F-2-dependency-inventory`, fixes applied 1), the GitHub `#2` pre-acceptance state was observed read-only (`F-2-github-preacceptance-state`, fixes applied 1: `state=open`, `closed_at=null`, timeline = one `milestoned` event, 0 `closed`, 0 `reopened`; `gh` itself is not installed, so the observation used the GitHub REST API and that is recorded as an environment finding), and the criterion mapping in `manifest.json` was rebuilt on the authoritative `r2` set (`F-2-evidence-map`, fixes applied 1). The product tree changed in exactly one documentation file; `Cargo.toml`, `Cargo.lock` and every `.rs` file are byte-identical to the `r1` snapshot. No commit, no staging, no GitHub write had occurred when the candidate was handed over. CHECK 3 (`r2`/`n2`) then returned **`accepted → ACT`**: `AC-2.1`–`AC-2.4` all `met` (`AC-2.4` in its pre-acceptance component only) and the four item keys `F-2-dependency-inventory`, `F-2-github-preacceptance-state`, `F-2-evidence-map` and `F-2-reference-provider` refuted on the frozen candidate. The post-acceptance closure was therefore the only outstanding obligation — one atomic commit referencing `#2`, then the GitHub `#2` closure — and it belonged to ACT, which has now run. **ACT completed**: the mandatory retrospective was recorded verbatim (source: the `pdca-check` retrospective report routed by the driving role), the 13-path accepted tree was re-anchored 13/13 before staging, and one atomic commit `59fb1fb` (parent `2d0be53`, 13 files, +1750/−23, footer `Refs #2`) was created on `main` — no push, no branch switch, no amend/rebase, nothing outside the 13 item paths staged. Accepted snapshot identity: HEAD `2d0be53` + the frozen 13-file tree (`docs/dependency-audit.md` sha256 `527862c0…`, 21,256 B); criteria outcome `AC-2.1` met, `AC-2.2` met, `AC-2.3` met, `AC-2.4` met (pre-acceptance component) with its post-acceptance commit action now performed. **Residual**: the GitHub `#2` closure is executed by the calling driver after this commit — this executor does not write to GitHub — and the item is `accepted`/`done` pending that closure. Next authorized action: the calling driver closes GitHub `#2` with the `AC-2.1`–`AC-2.3` report; then the collection proceeds to `DO item #8 (r1/n1)`
- source: collection `open-issues-r5` item `#2` (order 1, risk-first) · context refs: `.pdca/status/collection-open-issues-r5.md`, `.pdca/open-issues-r5/rv1/plan.md`, `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/` (preserved r4 candidate), `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/` (this cycle's evidence root)
- criteria: `AC-2.1`–`AC-2.4` — exact text in §PLAN record `r2` (source: the planner's revision return of 2026-11-10, appended verbatim as §PLAN record r2); the user's `Done when` and the approved tolerance are unchanged and quoted in §Original task

## Original task

The collection's top-level task, verbatim (original language, Russian), by exact reference — it is
recorded unchanged in `.pdca/status/collection-open-issues-r5.md` §Original task and §Input:

> pdca по открытым задачам из github

> тогда делай автономно с автокомитом

> после каждой выполненной задачи - делай комит и закрывай задачу

```text
#2 принимаю допуск
#3 ok
#4 ok
#8 ok
```

Item `#2` `Done when`, verbatim (unchanged from the collection §Input):

```text
a published MDX-Net ONNX file (supplied locally, checksum-checked by caller) separates audio on CPU; output matches audio-separator on a frozen fixture within agreed tolerance; dependency documented in docs/dependency-audit.md with licence.
```

Approved tolerance for `#2`, verbatim and immutable (user-approved, collection §Input and
`.pdca/open-issues-r5/rv1/criteria-approval.md`):

```text
For comparison candle-roformer vs audio-separator 0.48.0 on one frozen fixture, CPU/f32, with the same .onnx SHA-256: max_abs ≤ 1e-5; RMSE ≤ 1e-6; SDR ≥ 100 dB; correlation ≥ 0.999999; comparison over all samples of all channels and stems, without trimming and without post-normalization.
```

## PLAN record

### r1

`recorded by exact reference`: the item contract `AC-2.1`–`AC-2.3` is the collection PLAN return r1
persisted complete at `.pdca/open-issues-r5/rv1/plan.md` (line 9 is the per-item acceptance criteria
block; line 8 the measurement method; line 10 the schedule) and quoted in §PLAN record `r1` of
`.pdca/status/collection-open-issues-r5.md`. The criteria are not retyped here.

- `AC-2.1` — real published MDX-Net `.onnx`, checksum-verified, CPU separation, parity vs
  audio-separator 0.48.0 at the `#2` thresholds (max_abs ≤ 1e-5; RMSE ≤ 1e-6; SDR ≥ 100 dB;
  correlation ≥ 0.999999; all samples of all channels and stems, no trimming, no post-normalization).
- `AC-2.2` — no regressions: build + full tests (#1's accepted BS-RoFormer work intact).
- `AC-2.3` — one atomic commit + GitHub `#2` closed (ACT only; not performed in DO).

### r2

`recorded verbatim` — the planner's revision return of 2026-11-10 for cycle `issue-2-r5`, `N=1`, `r=2`,
`n=1/3` (route `CHECK → PLAN`). Complete original, unchanged:

# Решение PLAN — пересмотр контракта результата (item #2: MDX-Net ONNX, цикл `issue-2-r5`)

## 1. Решение о пересмотре и учёт ревизии (Contract revision)
- **Решение:** подтверждён **подлинный пересмотр контракта (genuine revision)** по возврату `CHECK → PLAN` из-за двух подтверждённых дефектов производного набора критериев в `r=1`:
  1. `F-2-criteria-coverage` (major, confirmed): производные критерии `AC-2.1–AC-2.3` в `r=1` упустили обязательное требование из исходного `Done when`: `dependency documented in docs/dependency-audit.md with licence`.
  2. `F-2-acceptance-order` (blocking, confirmed): критерий `AC-2.3` в `r=1` требовал уже выполненных git-коммита и закрытия GitHub issue `#2` на момент оценки `CHECK`, тогда как политика коллекции разрешает коммит и закрытие issue только **после** `accepted CHECK`, что создавало циклическую блокировку фазового порядка.
- **Основание подлинности ревизии:** изменяются производные критерии и фазовый контракт приёмки/закрытия элемента коллекции при полном, дословном и неизменном сохранении исходной постановки пользователя (`Done when`) и утверждённого пользователем численного допуска (`criteria-approval.md`).
- **Счётчики (для фиксации `pdca-executor` в журнале цикла):**
  - Завершённая непринятая попытка `r=1, n=1/3` сохраняется в истории попыток без зачёта прогресса.
  - Переход к ревизии **`r=2`**, счётчик попыток внутри новой ревизии контракта — **`n=1/3`** (`N=1` без изменений).

---

## 2. Форма, специализация и входная область (Form, specialized skill, entry scope)
- **Форма и навык:** переносятся по ссылке из контракта коллекции `F:\sources\candle-roformer\.pdca\status\collection-open-issues-r5.md` и `F:\sources\candle-roformer\.pdca\open-issues-r5\rv1\plan.md` — элемент коллекции `open-issues-r5` (обычный цикл элемента `issue-2-r5`), специализированный навык `pdca-coder` (метод инференса ONNX на CPU и покомпонентной сверки с эталоном определён, объём ограничен одним элементом `#2` и одной замороженной фикстурой).
- **Входная область (Entry scope) для `r=2`:** полный приёмочный результат элемента `#2` (MDX-Net ONNX на CPU, совпадение с эталоном `audio-separator 0.48.0` в пределах утверждённого допуска с проверяемым подтверждением CPU-режима эталона, документирование зависимостей с лицензиями в `docs/dependency-audit.md`, отсутствие регрессий сборки/тестов и готовность к постприёмочному закрытию).

---

## 3. Цель, запрашиваемый/поставляемый результат и критерии приёмки (Goal and acceptance criteria)

### Исходная постановка задачи (неизменная, по точным ссылкам)
- Верхнеуровневая задача, таблица элементов и утверждённое дополнение критериев: `F:\sources\candle-roformer\.pdca\status\collection-open-issues-r5.md` (§Original task, §Input) и `.pdca\open-issues-r5\rv1\criteria-approval.md`.
- **`Done when` для элемента `#2` (дословно):**
  > "a published MDX-Net ONNX file (supplied locally, checksum-checked by caller) separates audio on CPU; output matches audio-separator on a frozen fixture within agreed tolerance; dependency documented in docs/dependency-audit.md with licence."
- **Утверждённый пользователем допуск для `#2` (дословно и неизменно):**
  > "For comparison candle-roformer vs audio-separator 0.48.0 on one frozen fixture, CPU/f32, with the same .onnx SHA-256: max_abs ≤ 1e-5; RMSE ≤ 1e-6; SDR ≥ 100 dB; correlation ≥ 0.999999; comparison over all samples of all channels and stems, without trimming and without post-normalization."

### Запрашиваемый и поставляемый результат
- **Запрашиваемый результат (requested result):** рабочая поддержка разделения аудио опубликованной локальной моделью MDX-Net ONNX (с проверкой контрольной суммы вызывающей стороной) на CPU в `candle-roformer`, дающая на замороженной фикстуре выход, совпадающий с `audio-separator 0.48.0` (CPU/f32, тот же `.onnx` SHA-256) в пределах утверждённого допуска по всем отсчётам всех каналов и стемов без подрезки и пост-нормализации, с документированием использованных зависимостей и их лицензий в `docs/dependency-audit.md`.
- **Поставляемый результат (delivered result):** тот же самый объект (код и тесты `candle-roformer`, запись в `docs/dependency-audit.md`, артефакты/данные сопоставления с эталоном для приёмки `CHECK`, и после `accepted CHECK` — git-коммит и закрытие issue `#2`).

### Полный пересмотренный набор критериев приёмки (`r=2`)
Каждый критерий оценивается на фазе `CHECK` до выполнения постприёмочных действий закрытия:

- **AC-2.1 (Разделение MDX-Net ONNX на CPU и численное совпадение с эталоном):**
  Опубликованный файл модели MDX-Net ONNX, переданный локально и проверенный вызывающей стороной по контрольной сумме SHA-256, разделяет аудио на CPU в `candle-roformer` без ошибок. На одной замороженной фикстуре выход удовлетворяет утверждённому пользователем допуску (дословно):
  *"For comparison candle-roformer vs audio-separator 0.48.0 on one frozen fixture, CPU/f32, with the same .onnx SHA-256: max_abs ≤ 1e-5; RMSE ≤ 1e-6; SDR ≥ 100 dB; correlation ≥ 0.999999; comparison over all samples of all channels and stems, without trimming and without post-normalization."*
  **Отрицательное условие:** не допускается пропуск проверки SHA-256, использование несовпадающего `.onnx` SHA-256 между `candle-roformer` и эталоном, выполнение разделения или эталонного прогона вне режима `CPU/f32` (активный CPU execution provider эталонного запуска должен быть наблюдаемо подтверждён, а не только перечислен в списке доступных провайдеров), подрезка (trimming) отсчётов, пост-нормализация, пропуск какого-либо канала или стема либо выход любой из четырёх метрик за указанный порог.

- **AC-2.2 (Документирование зависимости и лицензии):**
  Все добавленные/используемые для поддержки MDX-Net ONNX и проверки контрольной суммы зависимости документированы в `docs/dependency-audit.md` с явным указанием их лицензий (включая версию и лицензию каждой зависимости).
  **Отрицательное условие:** не допускается наличие в `Cargo.toml` / графе сборки функциональности MDX-Net ONNX зависимости, отсутствующей в `docs/dependency-audit.md` или указанной без лицензии.

- **AC-2.3 (Целостность сборки и отсутствие регрессий существующих моделей):**
  Проект компилируется без ошибок, все существующие и новые автоматические тесты (включая тесты существующей функциональности BS-RoFormer и новые проверки MDX-Net ONNX) проходят успешно.
  **Отрицательное условие:** не допускаются ошибки компиляции, падения существующих тестов или регрессии ранее работающих режимов разделения.

- **AC-2.4 (Контракт порядка приёмки и постприёмочного закрытия — устранение цикличности):**
  - **На момент оценки `CHECK` (pre-acceptance state):** все изменения по элементу `#2` находятся в проверяемом состоянии дерева, готовом к коммиту; коммит в git и закрытие GitHub issue `#2` **не должны** быть выполнены до вынесения вердикта `accepted` фазой `CHECK` (в полном соответствии с post-acceptance commit policy коллекции).
  - **Обязательство закрытия элемента после `accepted CHECK` (post-acceptance closure action):** непосредственно после вердикта `accepted` от `CHECK` (на фазе закрытия/ACT этого элемента) изменения фиксируются коммитом со ссылкой на `#2`, а GitHub issue `#2` закрывается с отчётом о выполнении всех критериев `AC-2.1–AC-2.3`.
  **Отрицательное условие:** не допускается как преждевременный коммит/закрытие issue `#2` до `accepted CHECK`, так и пропуск коммита или закрытия issue `#2` после получения `accepted CHECK`.

---

## 4. Метод измерения (Measurement method)
Полностью определяется утверждённым пользователем допуском (происхождение порогов — прямое указание пользователя в `.pdca\open-issues-r5\rv1\criteria-approval.md`, статус — фиксированный/неизменный):
- **Метрики и единицы:**
  - `max_abs` (максимальная абсолютная поточечная разность амплитуд, безразмерная в шкале `f32` PCM): порог `≤ 1e-5`;
  - `RMSE` (среднеквадратичная ошибка, безразмерная в шкале `f32` PCM): порог `≤ 1e-6`;
  - `SDR` (Signal-to-Distortion Ratio, `dB`): порог `≥ 100 dB`;
  - `correlation` (коэффициент корреляции Пирсона, безразмерный): порог `≥ 0.999999`.
- **Знаменатель и фильтры включения/исключения:** все отсчёты (`samples`) всех каналов (`channels`) и всех выходных стемов (`stems`) на одной замороженной входной аудио-фикстуре; исключение отсчётов, подрезка краёв (`trimming`) и пост-нормализация (`post-normalization`) запрещены (`0` исключённых отсчётов).
- **Условия сопоставимости:** сравнение выхода `candle-roformer` против `audio-separator 0.48.0` на одной и той же замороженной фикстуре, оба в режиме `CPU/f32`, на идентичном файле `.onnx` с совпадающим `SHA-256`.

---

## 5. Ограничения, допущения и закрытие пробелов (Constraints, assumptions, gaps)

### Ограничения (Constraints)
- Привязка к снапшоту: текущее рабочее состояние репозитория `F:\sources\candle-roformer` и доказательная база кандидата `issue-2-r5`.
- Запрет на изменение исходного текста `Done when` и утверждённого текста допусков.
- Соблюдение политики коллекции (`plan.md:9,11`): коммит и закрытие GitHub issue выполняются строго после `accepted CHECK`.
- Выполнение на CPU (`CPU/f32`), локальная подача опубликованного `.onnx` файла с проверкой контрольной суммы вызывающей стороной.

### Допущения (Assumptions)
- Уже полученные в попытке `r1/n1` результаты реализации и измерений (код, тесты, запись в `docs/dependency-audit.md:25–37`, численные результаты сравнения на 882,281 отсчётах на канал) остаются валидными, если исходные артефакты не менялись; единственным незакрытым доказательным пробелом по существу является `F-2-reference-provider`.

### Закрытие пробелов (Gaps)
1. **Пробел покрытия критериев (`F-2-criteria-coverage`):** закрыт в настоящем плане введением критерия `AC-2.2`, прямо отражающего `dependency documented in docs/dependency-audit.md with licence`.
2. **Пробел фазового порядка приёмки (`F-2-acceptance-order`):** закрыт в настоящем плане критерием `AC-2.4`, разделяющим предприёмочное состояние (проверяемое на `CHECK` без нарушения запрета на досрочный коммит) и обязательное постприёмочное закрытие (выполняемое после `accepted CHECK`).
3. **Доказательный пробел активного провайдера эталона (`F-2-reference-provider`):** закрывается на следующем шаге `DO` как требование к наблюдаемости условия сопоставимости `CPU/f32` в `AC-2.1` (см. раздел 7).

---

## 6. Минимальное решение и оценка трудоёмкости (Minimal solution & effort estimate)
- **Минимальное решение:** сохранить существующую кодовую реализацию MDX-Net ONNX, тесты и документацию в `docs/dependency-audit.md` (которые уже удовлетворили численные пороги, сборку/тесты и наличие лицензионной записи), и в рамках следующего `DO` устранить только пробел наблюдаемости `F-2-reference-provider` — обеспечить читаемое и однозначное доказательство того, что эталонный прогон `audio-separator 0.48.0` на замороженной фикстуре выполнялся именно с активным `CPUExecutionProvider` (`CPU/f32`) на том же `.onnx` SHA-256.
- **Оценка трудоёмкости (Effort estimate):** минимальная (1 короткая итерация `DO` + `CHECK` без переписывания продуктового кода разделения, если активный CPU-провайдер эталона подтверждается).

---

## 7. Передача в DO (Plan-to-DO handoff — что требуется от следующего DO как результат)
Следующий `DO` работает по контракту ревизии `r=2, n=1/3` (с передачей по ссылке исходного задания в `F:\sources\candle-roformer\.pdca\status\collection-open-issues-r5.md` и критериев `AC-2.1–AC-2.4` настоящего плана).

**Требуемый от `DO` результат (как состояние/исход, без предписания команд или способа реализации):**
1. **Закрытие `F-2-reference-provider` для `AC-2.1`:** представить в составе кандидата читаемое, непосредственно проверяемое подтверждение того, что эталонный выход `audio-separator 0.48.0`, с которым сравнивается `candle-roformer`, получен при **активно использованном** CPU execution provider (`CPU/f32`, а не просто при наличии CPU в списке доступных провайдеров и без нечитаемого бинарного лога), при том же `.onnx` SHA-256 и с соблюдением всех порогов (`max_abs ≤ 1e-5`, `RMSE ≤ 1e-6`, `SDR ≥ 100 dB`, `correlation ≥ 0.999999` на всех отсчётах всех каналов и стемов без подрезки и пост-нормализации).
2. **Сохранение выполнения `AC-2.2` и `AC-2.3`:** сохранить актуальную документацию зависимостей (`tract-onnx`, `sha2`) с лицензиями в `docs/dependency-audit.md` и безошибочное прохождение сборки и тестов.
3. **Соблюдение `AC-2.4`:** **не** создавать git-коммит и **не** закрывать GitHub issue `#2` во время `DO` — передать готовое рабочее состояние на проверку `CHECK`; коммит и закрытие issue `#2` будут выполнены только после получения вердикта `accepted` от `CHECK`.

---

## 8. Риски, стоп-условия и уверенность (Risks, stop conditions, confidence)
- **Риски:**
  - Если выяснится, что предыдущий эталонный результат `audio-separator 0.48.0` фактически выполнялся на не-CPU провайдере, потребуется пересчёт эталона в режиме `CPU/f32` и повторная сверка метрик против неизменных порогов `AC-2.1`.
- **Стоп-условия (Stop conditions):**
  - Исчерпание лимита попыток (`n=3/3`) в ревизии `r=2` без достижения `accepted CHECK`.
  - Обнаружение того, что при подтверждённом активном `CPU/f32` провайдере эталона `audio-separator 0.48.0` численные пороги из утверждённого допуска нарушаются и не могут быть достигнуты на CPU.
- **Уверенность и статус фактов (Confidence and gaps):**
  - **Подтверждено (verified):** численные метрики на 882,281 кадрах (`max_abs = 1.423061e-6`, `RMSE = 1.688632e-7`, `SDR = 117.3996 dB`, `corr = 0.9999999999991059`), отсутствие подрезки PCM (`F-2-wav-trimming` опровергнут), прохождение `cargo build` и `cargo test` (98 passed), наличие `tract-onnx 0.23.10` и `sha2 0.10.9` с лицензиями `MIT OR Apache-2.0` в `docs/dependency-audit.md:25–37`.
  - **Остаётся открытым до завершения `DO`:** верифицируемое подтверждение активного CPU provider для эталонного прогона (`F-2-reference-provider`).

## Approval record

`not applicable: autonomous mode` — no plan approval was requested or required; the criteria addendum
was approved by the user and is recorded in `.pdca/status/collection-open-issues-r5.md` §Approval
record.

## DO result

- **Candidate**: the preserved r4 MDX-Net candidate (`patch/item-2-candidate.patch`, sha256
  `4922abec88ce6661a31c96f16eeba2033d5657f7473797fd139769ab8896ff17`) applied onto HEAD `2d0be53`
  and reconciled with `#1`'s accepted BS-RoFormer work. Both features are present. The numerical core
  (`src/mdx.rs`, `src/dsp.rs`, the STFT/ISTFT + chunking + model I/O path) is byte-identical in
  behaviour to the preserved candidate: after normalising the repo's `core.autocrlf=true` CRLF
  checkout, `src/mdx.rs`, `src/dsp.rs`, `src/error.rs`, `examples/mdx_separate.rs`,
  `docs/dependency-audit.md`, `docs/dependency-audit-evidence/tract-onnx-0.23.10.txt`, `Cargo.toml`
  and `Cargo.lock` are byte-identical to `snapshot/` of the r4 evidence root (sha256 values recorded
  in `snapshot-sha256.txt` and `manifest.json`).
- **Conflict resolutions** (every one recorded in `manifest.json` › `conflict_resolutions`):
  `git apply --check` clean for `Cargo.toml`, `Cargo.lock`, `docs/dependency-audit.md`, `src/dsp.rs`,
  `src/error.rs`, `docs/dependency-audit-evidence/tract-onnx-0.23.10.txt`, `examples/mdx_separate.rs`,
  `src/mdx.rs`; `git apply` context failure for `src/config.rs` and `src/lib.rs` (both changed by
  `#1`), and `README.md` reported a 3-way conflict. `config.rs`/`lib.rs` were reconciled by hand
  applying the same hunks onto HEAD's version (keeping `#1`'s serde-renamed `ModelFamily` variants,
  `BSRoformer`/`RoFormerModel` re-exports); README by hand-merging the MDX row/bullet into `#1`'s
  4-column model table and status list. `ModelFamily::MdxNet` adds a new enum variant, so the two
  exhaustive matches introduced by `#1` gained arms: `src/separator.rs` (`Separator::load` returns a
  typed `ConfigError` directing MDX to `mdx::MdxSeparator`) and `src/model/mod.rs`
  (`expected_key_shapes` returns an empty set — ONNX has no safetensors key schema). No numerical
  default, threshold or geometry changed.
- **Checkpoint verification**: `UVR-MDX-NET-Inst_HQ_3.onnx`, 66,759,214 bytes, sha256
  `317554b07fe1ea5279a77f2b1520a41ea4b93432560c4ffd08792c30fddf9adc` (matches the pinned profile and
  the r4 record), computed with the full Python path — log `logs/checkpoint-sha256.log`.
- **Gates**: `cargo build` exit `0`; `cargo test` exit `0` — 98 passed / 0 failed / 1 ignored
  (lib 33, `bs_roformer` 8+1 ignored, `core_tests` 22, `library_api` 12, `s1_spikes` 11,
  `streaming_chunked` 8, `whisper_tests` 3, doc-test 1); `cargo clippy --all-targets` exit `0`,
  warnings only (52 lib warnings before dedup, pre-existing crate-wide style lints; no errors).
- **Candidate separation**: `cargo run --release --example mdx_separate -- <fixture> <models> <out>`,
  exit `0`, CPU only, f32, 882,281 frames × 2 ch @ 44,100 Hz, elapsed 80.307 s — log
  `logs/candidate-run.log`, outputs `out/candidate/{instrumental,vocals}.wav`.
- **Reference**: fresh audio-separator 0.48.0 run on CPU (pinned venv: torch 2.15.0+cpu, onnxruntime
  1.31.1, numpy 2.5.4, python 3.12.10), same fixture sha256 and same `.onnx` sha256, preserved
  `scripts/reference_audio_separator.py`, exit `0` — log `logs/reference-run.log`, summary
  `logs/reference-summary.json`, outputs `out/reference/…_(Vocals|Instrumental)_UVR-MDX-NET-Inst_HQ_3.wav`.
  The audio payload is bit-identical to the preserved r4 reference (`data` chunk sha256
  `7fd76234a2e7ef6eaea9567dae71ea7db83cdd4712ca5195c3764ac81a65ea93` on both); only the 2-byte `PEAK`
  chunk timestamp differs, which is why the file sha256 differs from r4's summary.
- **Comparison** (all samples of all channels, no trimming, no post-normalization; preserved
  `scripts/compare_stems.py` plus the per-channel extension `scripts/compare_stems_per_channel.py`,
  model/fixture sha256 re-checked by both): see `comparison.json` and
  `logs/comparison-per-channel.json`. Per stem × channel (max_abs / RMSE / SDR / correlation):
  Vocals L 1.423061e-6 / 1.687055e-7 / 118.439 dB / 0.999999999999305; Vocals R 1.341105e-6 /
  1.688632e-7 / 117.400 dB / 0.999999999999106; Instrumental L 1.013279e-6 / 1.200875e-7 /
  120.551 dB / 0.999999999999565; Instrumental R 9.536743e-7 / 1.204351e-7 / 120.064 dB /
  0.999999999999509. Pooled per stem: Vocals max_abs 1.423061e-6, RMSE 1.687843e-7, SDR 117.950 dB,
  corr 0.999999999999215; Instrumental max_abs 1.013279e-6, RMSE 1.202614e-7, SDR 120.314 dB,
  corr 0.999999999999538. Overall (both stems, 3,529,124 samples): max_abs 1.4230609e-6,
  RMSE 1.4654514e-7, SDR 118.897 dB; correlation is not pooled in the artefact — it is
  ≥ 0.999999 on every stem × channel (minimum 0.999999999999106). Every row is inside all four
  approved thresholds; no trimming and no post-normalization.
- **Findings to report (not trimmed, not normalised)**: the two WAV files differ in byte length —
  reference 7,058,336 B vs candidate 7,058,292 B — because the reference carries `fact` (12 B with
  header) and `PEAK` (32 B with header) metadata chunks while the candidate writes `fmt` + `data`
  only; the `data` chunks are both 7,058,248 B and both files are 882,281 frames × 2 ch @ 44,100 Hz,
  so nothing was trimmed — log `logs/wav-container-diff.log` (RIFF walk, exit `0`, delta +44 B =
  `fact` 12 B + `PEAK` 32 B, `data` delta 0).
- **Evidence root file map** (`.pdca/open-issues-r5/rv1/item-2-mdx-onnx/`): `manifest.json` (binding
  record: HEAD `2d0be53`, dirty worktree with nothing staged, checkpoint/fixture/output sha256,
  `criteria_ids` `AC-2.1`–`AC-2.3`, `conflict_resolutions`, `gates`, `runs`, `criterion_evidence`) ·
  `comparison.json` (root-level per-stem × channel projection of the approved thresholds) ·
  `index.md` (compact evidence index) · `commands.txt` (every command line, exit code, key numbers,
  log path) · `snapshot-sha256.txt` + `snapshot.diff` + `snapshot/` (13 changed files, CRLF worktree
  bytes) · `logs/` (cargo gates `-final`, `checkpoint-sha256`, `candidate-run`, `reference-run` +
  `reference-summary.json`, `compare-stems*`, `comparison*.json`, `wav-container-diff`,
  `stage-snapshot`, `transcode-logs`) with UTF-8 readable copies under `logs/utf8/` ·
  `scripts/` (7 staged helpers) · `models/` (published `.onnx` 66,759,214 B + checksum metadata) ·
  `fixture/` · `out/{candidate,reference}/`. No weights, fixtures or audio enter a tracked path.
- **Unit state**: candidate complete for `AC-2.1` and `AC-2.2`; `AC-2.3` (commit + GitHub closure)
  is ACT work and stays `pending`. No verdict is expressed here.

### r2 / n1

Attempt `r2`/`n1` under the revised contract `AC-2.1`–`AC-2.4`. No product file was changed: the `r1`
candidate tree is the accepted-shape tree and was verified byte-identical before and after this
attempt.

- **Gap closed — `F-2-reference-provider`.** The reference driver was instrumented **as an evidence-root
  copy only** (`scripts/reference_audio_separator_audit.py`; no product file, no preserved `r1` script
  modified). Before importing `audio_separator` it replaces `onnxruntime.InferenceSession` with a
  subclass that, after construction, records `session.get_providers()` of the **live session** together
  with the argument passed as `providers`, the model path and its sha256, and the input/output tensor
  types. It also records `onnxruntime.__version__`, `torch.__version__`,
  `torch.cuda.is_available()`, `torch.backends.mps.is_available()`, directml presence,
  `get_available_providers()` (availability, explicitly labelled as not selection) and the Separator's
  resolved device choices.
- **Provider evidence (one live session, CPU verified as *used*, not merely available).**
  `sessions_created=1`; `session[0].requested_providers=["CPUExecutionProvider"]` and
  `session[0].active_providers=["CPUExecutionProvider"]` from `session.get_providers()`; input and
  output `tensor(float)` (f32); `separator.torch_device=cpu`,
  `separator.onnx_execution_provider=["CPUExecutionProvider"]`, `separator.use_directml=False`;
  `torch.cuda.is_available()=False`, `torch.backends.mps.is_available()=False`, `torch_directml` not
  installed; `get_available_providers()=["AzureExecutionProvider","CPUExecutionProvider"]`. Stack:
  audio-separator 0.48.0 · onnxruntime 1.31.1 · torch 2.15.0+cpu · numpy 2.5.4 · python 3.12.10.
  Same `.onnx` sha256 `317554b07fe1ea5279a77f2b1520a41ea4b93432560c4ffd08792c30fddf9adc` on the frozen
  fixture sha256 `a6dcdd06ff1c19677787fd36f4114d4c98145161c6c1e259b9658b753eb13d88`.
  Artefacts: `logs/reference-provider-audit.log` (3,376 B, **pure ASCII, 0 non-ASCII bytes**),
  `logs/reference-provider-audit.json`; the same facts also appear in the r1 log, which is readable at
  `logs/utf8/reference-run.log.utf8.log:398` (`ONNX Runtime session providers:
  ['CPUExecutionProvider']`) — no encoding repair is needed for the new artefacts.
- **Reference re-run** (same fixture, same `.onnx`, same Separator/mdx settings; `CUDA_VISIBLE_DEVICES`
  emptied as a belt-and-braces guard): exit `0`, 11.814 s, outputs written to `out/reference-r2/` so the
  `r1` reference stays intact for the byte-level check. Vocals 7,058,336 B,
  file sha256 `64e8b93567e2100d5424cf90f59a6b85784d4c66baf99716e4421f5b8eb40225`, `data` chunk
  7,058,248 B sha256 `7fd76234a2e7ef6eaea9567dae71ea7db83cdd4712ca5195c3764ac81a65ea93`; Instrumental
  7,058,336 B, file sha256 `f2aaa743f60a1763c52b2eedd7456094ae38d1f927283381b7ade31d20648b9b`, `data`
  chunk 7,058,248 B sha256 `98fef60a2840991039ffcd6b1b3badfdb389f70a479c355ac690fb753ea86818`; both
  882,281 frames × 2 ch @ 44,100 Hz. Full stdout `logs/reference-r2-run.log` (154,920 B, 0 non-ASCII
  bytes — readable, no transcode needed).
- **Fresh reference ≡ `r1` reference (PCM).** `scripts/check_reference_pcm_equivalence.py` exit `0`: on
  both stems the `data` chunk byte length and sha256 are identical, the f32 PCM arrays are bit-identical
  sample by sample, and the frame/channel/rate geometry is equal; only the `PEAK` chunk differs (so the
  whole-file sha256 differs, which is expected and not a PCM finding).
  Log `logs/reference-pcm-equivalence.log` (+ `.json`).
- **Comparison re-run** against the **unchanged** candidate outputs with the fresh reference
  (`scripts/compare_r2.py` driving the preserved `compare_stems.py` /
  `compare_stems_per_channel.py` byte-identically, with `REF_DIR` redirected to `out/reference-r2/`):
  exit `0`. `scripts/compare_metrics_r1_r2.py` exit `0` — every measured field per stem × channel is
  **bit-identical** to `r1` (`logs/comparison-r1-vs-r2.log`, `.json`). Per stem × channel
  (max_abs / RMSE / SDR / correlation), 882,281 samples each:
  Vocals L `1.4230608940e-06` / `1.6870548336e-07` / `118.438951` dB / `0.9999999999993046`;
  Vocals R `1.3411045074e-06` / `1.6886315279e-07` / `117.399591` dB / `0.9999999999991059`;
  Instrumental L `1.0132789612e-06` / `1.2008750616e-07` / `120.551176` dB / `0.9999999999995649`;
  Instrumental R `9.5367431641e-07` / `1.2043507525e-07` / `120.063622` dB / `0.9999999999995094`.
  Aggregate: Vocals max_abs `1.4230608940e-06` / RMSE `1.6878432518e-07` / SDR `117.949807` dB /
  corr `0.9999999999992147`; Instrumental `1.0132789612e-06` / `1.2026141860e-07` / `120.313534` dB /
  `0.9999999999995383`; overall (both stems, 3,529,124 samples) max_abs `1.4230608940e-06`, RMSE
  `1.4654514365e-07`, SDR `118.896514` dB. All four thresholds hold on every row; no trimming, no
  post-normalization (0 excluded samples). `comparison.json` numbers did not change and were left
  as recorded; the re-run wrote `logs/comparison-r2.json` + `logs/comparison-per-channel-r2.json`.
- **Product tree unchanged.** `scripts/verify_snapshot_sha256.py` exit `0`: **13/13** files match
  `snapshot-sha256.txt`, 0 mismatches, 0 missing, `worktree==snapshot=True` for each; `HEAD` still
  `2d0be535fd82a1296a76f74b0d738829bd54a6df`, nothing staged. Log `logs/tree-sha256-verify.log`,
  `.json`.
- **Gate provenance (not re-run — same bytes).** Because no product file changed, the `r1` gates still
  bind exactly these files and were cited instead of re-executed: `cargo build` exit `0`
  (`logs/cargo-build-final.log`), `cargo test` exit `0` — 98 passed / 0 failed / 1 ignored
  (`logs/cargo-test-final.log`), `cargo clippy --all-targets` exit `0` — warnings only
  (`logs/cargo-clippy-final.log`). The binding of those logs to the verified hashes is recorded in
  `logs/tree-sha256-verify.json` and in `manifest.json` › `gate_provenance`.
- **Scope discipline.** No `src/`, `examples/`, `docs/`, `Cargo.*` or `README.md` edit; no `git add`,
  no staging, no commit, no push, no GitHub closure (`AC-2.4` pre-acceptance state). Only
  `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/` (evidence) and `.pdca/status/` (records) changed.

### r2 / n2

Attempt `r2`/`n2` answered the three items of CHECK 2. Product change: **one documentation file**
(`docs/dependency-audit.md`); `Cargo.toml`, `Cargo.lock` and all `.rs` files are byte-identical to
the `r1` snapshot (verified by sha256, below).

- **`F-2-dependency-inventory` (AC-2.2) — fixed.** `docs/dependency-audit.md` gained the subsection
  `#### Resolved dependency closure of tract-onnx and sha2` with a **149-row** table: crate name,
  resolved version, licence as the crate's own SPDX expression, and role/why. Rows were generated by
  tooling, not from memory: `cargo metadata --format-version 1` (raw JSON 1,193,103 B persisted) gave
  the `license` field and the `resolve` graph, which was walked from the two roots; `Cargo.lock` was
  the version cross-check for every row; `cargo tree` supplied the host build graph so nothing is
  presented as compiled that is not. Numbers: `tract-onnx` closure 141 crates, `sha2` closure 10,
  **union 149** (2 direct: `tract-onnx` 0.23.10, `sha2` 0.10.9); 118 rows in the host build graph,
  31 `resolved only`; **0 rows without a licence** (none is guessed); every row present in
  `Cargo.lock` at the same version. 15 distinct licence expressions; each offers a permissive option;
  the only copyleft mention is `LGPL-2.1-or-later` as one alternative of `r-efi`'s
  `MIT OR Apache-2.0 OR LGPL-2.1-or-later` (and `r-efi` is resolved-only here); no GPL-only/AGPL/SSPL
  crate. All nine `tract-*` crates are listed individually. `Cargo.toml`/`Cargo.lock` untouched.
  Document: 6,353 B → 21,256 B, sha256
  `527862c0e88b194479f150526e69afe66b94831904f3a715a555fb4937256b74`; CRLF preserved (262 CRLF,
  0 bare LF); a second splice run reproduced identical bytes.
- **`F-2-github-preacceptance-state` (AC-2.4) — observed, read-only.** Environment finding first:
  **`gh` is not installed** — `where.exe gh` exit `1`, `Get-Command gh` empty, and every probed
  install path (`Program Files\GitHub CLI`, `%LOCALAPPDATA%\GitHubCLI`, scoop/choco shims, WinGet
  links) absent; the probe records exit `127` with
  `FileNotFoundError: [WinError 2] The system cannot find the file specified`. `gh auth status` is
  therefore recorded as **`not set`** (set/not set only; no token read or printed). The state was
  then observed through the same GitHub API the CLI wraps, unauthenticated and read-only
  (`observation_channel` is labelled as such in the evidence; nothing was written to GitHub):
  repo derived from `git remote -v` = `AlexeyShirshov/candle-roformer`; `GET /repos/…/issues/2` →
  HTTP `200`, `GET /repos/…/issues/2/timeline` → HTTP `200`, 1 page. Result: **issue `#2` is
  `open`** (`state_reason=null`, `created_at` `2026-11-08T06:43:12Z`, `closed_at=null`, labels `[]`);
  timeline = **1 event, type `milestoned`**; **0 `closed`** and **0 `reopened`** events, 0 comments.
  So no premature closure has occurred, and the `AC-2.4` pre-acceptance condition is now evidenced
  by observation rather than by assertion.
- **`F-2-evidence-map` — fixed.** `manifest.json` › `criterion_evidence` rebuilt on the authoritative
  `r2` set: `AC-2.1` CPU separation + tolerance parity, `AC-2.2` dependency/licence documentation,
  `AC-2.3` build/tests no regressions, `AC-2.4` pre-acceptance tree state + post-acceptance closure
  contract. Each criterion now lists evidence that matches its actual text (the stale "regression
  gate"/"pre-acceptance state" mapping is gone), and `AC-2.4` separates the observed pre-acceptance
  state from the still-unperformed closure obligation.
- **Frozen candidate refreshed.** `snapshot/docs/dependency-audit.md` replaced, `snapshot-sha256.txt`
  rewritten (13 rows, `worktree==snapshot` on every row, new `snapshot.diff` sha256), `snapshot.diff`
  regenerated (83,461 B, sha256
  `e5d28adf7845aff00f3a872c577b7771bded26283a14d19aa63bd75a36e6628f`), `manifest.json` (new hashes,
  `attempt` `n2/3`, new blocks `dependency_inventory` and `preacceptance_github_observation`, r2/n2
  `tree_verification`, updated `gate_provenance`), `commands.txt` (PART 3) and `index.md` refreshed.
- **Ledger / gate provenance.** `scripts/verify_snapshot_sha256_r2n2.py` exit `0`: **13/13** files
  match `snapshot-sha256.txt`, 0 mismatch, 0 missing; **changed vs `r1` = 1 file**
  (`docs/dependency-audit.md`, `88ea8e93…` → `527862c0…`), **unchanged vs `r1` = 12 of 13**;
  `.rs` files identical to `r1` = true; `Cargo.toml` + `Cargo.lock` identical to `r1` = true; `HEAD`
  still `2d0be53`, nothing staged. Because the only change is documentation, `cargo build`, `cargo
  test` (98 passed / 0 failed / 1 ignored) and `cargo clippy` (exit `0`, warnings only) were **not**
  re-run: their `r1` logs still bind exactly these bytes. Log
  `logs/tree-sha256-verify-r2n2.log`.
- **New evidence (all pure ASCII)** in `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/`:
  `cargo-metadata-r2n2.json` (raw metadata) + `.err`; `cargo-tree-host-r2n2.log`/`.json`/`.stdout.log`;
  `dependency-inventory.json`/`.md`/`.log`/`.stdout.log`; `splice-audit-table.log`;
  `github-preacceptance-state.log`/`.json`, `github-issue-2-raw.json`,
  `github-issue-2-timeline-raw.json`, `github-preacceptance-stdout.log`; `stage-snapshot-r2n2.log`;
  `tree-sha256-verify-r2n2.log`/`.json`, `tree-verify-r2n2.stdout.log`. New scripts:
  `dependency_inventory.py`, `cargo_tree_host.py`, `splice_audit_table.py`,
  `github_preacceptance_probe.py`, `stage_snapshot_r2n2.py`, `verify_snapshot_sha256_r2n2.py`,
  `stage_manifest_r2n2.py`.
- **Scope discipline.** No `src/`, `examples/`, `Cargo.*` or `README.md` edit; no dependency change;
  no `git add`, no staging, no commit, no push, no GitHub write. `AC-2.4`'s post-acceptance closure
  remains unperformed by design. No verdict is expressed here.

## CHECK record

Recorded as returned by the independent `pdca-check` (source: the driving role's dispatch of
2026-11-10 for cycle `issue-2-r5`, `N=1`, `r=1`, `n=1/3`; evidence root
`.pdca/open-issues-r5/rv1/item-2-mdx-onnx/`). Verdict: **`not accepted → PLAN`**. The complete report,
verbatim in its original language:

````text
## CHECK: **not accepted → PLAN**

Кандидат: `issue-2-r5`, HEAD `2d0be535fd82a1296a76f74b0d738829bd54a6df` + зафиксированное незакоммиченное дерево. Счётчики подтверждены: `N=1`, `r=1`, `n=1/3`; история дефектов — `none` (`.pdca/status/issue-2-r5.md:7,155–157`).

Ниже `E` = `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/`.

### Оценка критериев

1. **Неполное покрытие:** AC-2.1–AC-2.3 не включают обязательную часть исходного Done when: `dependency documented in docs/dependency-audit.md with licence`. Наличие документа в кандидате не устраняет дефект критериев.
2. **Противоречие порядка приёмки:** AC-2.3 требует уже выполненных коммита и закрытия issue, но PLAN разрешает их только после accepted CHECK (`plan.md:9,11`). Нельзя признать будущее действие выполненным.
3. Численные допуски определены и проверяемы; оснований менять утверждённый пользователем текст нет.

### Матрица

| Критерий | Статус | Основание |
|---|---|---|
| AC-2.1 | **unverified** | Численная часть подтверждается сравнением и просмотренным скриптом: все четыре stem/channel проходят. Худшие значения: max_abs `1.423061e-6`, RMSE `1.688632e-7`, SDR `117.3996 dB`, correlation `0.9999999999991059`; каждый канал содержит `882281` отсчёт. Кандидат выполнил CPU-разделение, exit `0`. Однако непосредственное подтверждение активного CPU provider эталонного запуска не просмотрено: reference-log недоступен чтению как binary, а summary перечисляет **доступные**, не активные providers. Источники: `E/comparison.json:33–89`, `E/scripts/compare_stems_per_channel.py:97–123`, `E/logs/utf8/candidate-run.log.utf8.log:95–100`, `E/logs/reference-summary.json:8–15`. |
| AC-2.2 | **met** | `cargo build`, exit `0`; `cargo test`, exit `0`: **98 passed / 0 failed / 1 ignored**, включая **8 passed** BS-RoFormer. Просмотрены завершённые build/test логи; отсутствие всех возможных регрессий этим не доказывается. Источники: `E/commands.txt:67–93`, `E/logs/utf8/cargo-build-final.log.utf8.log:95–96`, `E/logs/utf8/cargo-test-final.log.utf8.log:106–249`. |
| AC-2.3 | **unmet** | Коммит не создан; закрытие #2 не подтверждено. Это ожидаемый остаток по принятой политике, но не выполненный критерий. Источники: `E/manifest.json:295–298`, `.pdca/status/issue-2-r5.md:185–190`. |

### Findings

- **F-2-criteria-coverage — major, confirmed.** Обязательное требование лицензированной зависимости пропущено в критериях (`plan.md:9` против исходного Done when). При этом документация действительно содержит `tract-onnx 0.23.10` и `sha2 0.10.9`, лицензии `MIT OR Apache-2.0`: `E/snapshot/docs/dependency-audit.md:25–37`. Это не утверждение о дефекте реализации.
- **F-2-acceptance-order — blocking, confirmed.** AC-2.3 и post-acceptance commit policy образуют циклическое условие принятия (`plan.md:9,11`). CHECK не вправе заменить его обещанием.
- **F-2-reference-provider — major, unverified.** CPU-режим эталона заявлен, но активный provider непосредственно не проверен.
- **F-2-wav-trimming — informational, refuted.** Разница WAV в 44 байта объясняется metadata chunks; длина PCM одинакова: `7058248` байт, `882281` кадров. Основание: `E/logs/utf8/wav-container-diff.log.utf8.log:48–52`.

### Переход и пределы проверки

**→ PLAN:** устранить дефекты покрытия и фазового порядка производных критериев, сохранив исходные требования и утверждённые допуски без изменений. Коммит и закрытие issue этим CHECK не разрешены.

Для последующей проверки AC-2.1 нужен читаемый фрагмент существующего reference-log с фактически выбранным session provider; если его нет — инструментированное наблюдение через драйвер. Повторный запуск сейчас не предписывается.

Метод: статическое чтение snapshot, скриптов, результатов измерений и журналов; команды не запускались, аудиометрики независимо не пересчитывались. Уверенность высокая в дефектах критериев и результатах просмотренных тестов; подтверждение CPU-provider эталона остаётся пробелом. Исключения не приняты.
````

Not a pass: the candidate was not accepted and no commit or GitHub closure is authorized.

### CHECK 2 — r2/n1

Recorded as returned by the independent `pdca-check` (source: the driving role's dispatch of
2026-11-10 for cycle `issue-2-r5`, `N=1`, `r=2`, `n=1/3`; evidence root
`.pdca/open-issues-r5/rv1/item-2-mdx-onnx/`). Verdict: **`not accepted → DO`**. The complete report,
verbatim in its original language:

````text
## CHECK: **not accepted → DO**

Кандидат: HEAD `2d0be535fd82a1296a76f74b0d738829bd54a6df` + snapshot из 13 файлов. Счётчики подтверждены по status: **N=1, r=2, n=1/3**.

Ниже `E` = `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/`.

### Оценка критериев

В r2 восстановлено покрытие требования лицензированной зависимости; AC-2.4 устраняет циклическое требование досрочного коммита. Утверждённый допуск сохранён. Критерии проверяемы; оснований для нового пересмотра PLAN не установлено.

### Матрица

| Критерий | Статус | Проверенное основание |
|---|---|---|
| AC-2.1 | **met** | Просмотрены реализация checksum/CPU-пути, инструментирование живой ORT-сессии и результаты сравнения. Активен только `CPUExecutionProvider`, вход/выход `tensor(float)`, audio-separator **0.48.0**; SHA-256 модели совпадает. Все четыре stem/channel содержат **882281** отсчёт. Худшие метрики: max_abs **1.423061e-6**, RMSE **1.688632e-7**, SDR **117.399591 dB**, correlation **0.9999999999991059** — проходят. Компаратор не подрезает и не перенормирует выходы. Источники: `E/snapshot/src/mdx.rs:184–263,419–484`; `E/scripts/reference_audio_separator_audit.py:112–135,202–228`; `E/logs/reference-provider-audit.json`; `E/scripts/compare_stems_per_channel.py:97–123`; `E/logs/comparison-per-channel-r2.json`. Reference audit и `compare_r2.py`: exit **0** (`E/commands.txt:223–284`). |
| AC-2.2 | **unmet** | Прямые зависимости `tract-onnx 0.23.10` и `sha2 0.10.9` документированы с лицензиями. Однако критерий требует **каждую** зависимость MDX build graph с версией и лицензией. Например, `tract-nnef`, `tract-onnx-opl`, `tract-transformers` присутствуют в lock-графе, но индивидуально отсутствуют в audit. Обобщённое утверждение о лицензиях графа этого не покрывает. Источники: `E/snapshot/Cargo.lock:2403–2469`; `E/snapshot/docs/dependency-audit.md:25–50,65–75`. |
| AC-2.3 | **met** | `cargo build`, exit **0**; `cargo test`, exit **0**: **98 passed / 0 failed / 1 ignored**, включая MDX и **8 passed** BS-RoFormer. Повторное использование r1-логов обосновано проверкой **13/13** неизменных файлов и неизменного HEAD. Источники: `E/commands.txt:68–103,287–296`; `E/logs/utf8/cargo-build-final.log.utf8.log:95–96`; `E/logs/utf8/cargo-test-final.log.utf8.log:103–249`; `E/logs/tree-sha256-verify.log`. |
| AC-2.4 | **unverified** | Проверяемое незакоммиченное дерево и пустой index подтверждаются hash/git-наблюдением. Но независимого наблюдения состояния GitHub #2 нет: заявление «closure не выполнялось» не доказывает отсутствие преждевременного закрытия. Будущий коммит/closure оценивается при закрытии, не считается уже выполненным. Источники: `E/logs/tree-sha256-verify.log:43–47`; `E/commands.txt:326–331`. |

### Findings

- **F-2-dependency-inventory — major, confirmed, AC-2.2.** Нет требуемого поимённого version/licence-инвентаря MDX-графа; конкретные пропуски приведены выше.
- **F-2-github-preacceptance-state — major, unverified, AC-2.4.** Отсутствует наблюдение GitHub-состояния и событий закрытия.
- **F-2-evidence-map — minor, confirmed.** В `manifest.json:296–315` соответствие критериев устарело: AC-2.2 описан как regression gate, AC-2.3 — как pre-acceptance state. Это не меняет авторитетный r2-контракт, но затрудняет трассировку.
- **F-2-reference-provider — major, refuted для r2.** Прежний доказательный пробел закрыт непосредственным `session.get_providers()`.
- Прежние **F-2-criteria-coverage** и **F-2-acceptance-order** устранены ревизией r2, что подтверждено чтением полного набора критериев.

### Следующий переход: **→ DO**

1. Дополнить audit поимёнными версиями и лицензиями зависимостей MDX-графа, сверив с разрешённым Cargo-графом; исправить mapping критериев в manifest.
2. Через драйвер получить read-only наблюдение `gh issue view 2` и timeline событий закрытия/переоткрытия, привязанные ко времени. Это запрос недостающего доказательства AC-2.4, не утверждение о преждевременном closure.
3. Передать обновлённый frozen snapshot и доказательства на CHECK. **Коммит и закрытие #2 пока не разрешены.**

### Пределы и уверенность

Проверка — чтение snapshot, скриптов, измерений и журналов; команды не запускались, метрики независимо не пересчитывались. Нативная peak-нормализация разделителей присутствует; подтверждено отсутствие **дополнительной нормализации при сравнении**, а не отсутствие level-processing внутри алгоритмов. Один opt-in BS-checkpoint test остался ignored.

Уверенность высокая в численной части, CPU-provider и конкретных пропусках audit. GitHub-состояние остаётся неизвестным. Исключения не приняты.
````

Not a pass: the candidate was not accepted; the second attempt `r2`/`n2` returns to DO, and no commit
or GitHub closure is authorized.

### CHECK 3 — r2/n2

Recorded as returned by the independent `pdca-check` (source: the driving role's dispatch of
2026-11-10 for cycle `issue-2-r5`, `N=1`, `r=2`, `n=2/3`; evidence root
`.pdca/open-issues-r5/rv1/item-2-mdx-onnx/`). Verdict: **`accepted → ACT`**. The complete report,
verbatim in its original language:

````text
## CHECK 3: **accepted → ACT**

Кандидат: HEAD `2d0be535fd82a1296a76f74b0d738829bd54a6df` + замороженное незакоммиченное дерево. Счётчики подтверждены непосредственно по status: **N=1, r=2, n=2/3**.

Ниже `E` = `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/`.

### Оценка критериев

Набор r2 покрывает исходный Done when, сохраняет утверждённые допуски и разделяет предприёмочную проверку и последующее закрытие. Противоречий и препятствующих оценке неоднозначностей не установлено. Постприёмочные действия не считаются уже выполненными.

### Матрица

| Критерий | Статус | Проверенное основание и доказательства |
|---|---|---|
| **AC-2.1** | **met** | Просмотрены checksum-gate и CPU-путь (`E/snapshot/src/mdx.rs:184–263`), инструментирование живой ORT-сессии и comparator. Активен только `CPUExecutionProvider`, вход/выход `tensor(float)`, audio-separator **0.48.0**, SHA-256 модели совпадает. Все **4 stem/channel × 882281 отсчёт** включены. Худшие значения: max_abs **1.423061e-6**, RMSE **1.688632e-7**, SDR **117.399591 dB**, correlation **0.9999999999991059** — проходят все пороги. Comparator не подрезает и не нормирует данные. Источники: `E/logs/reference-provider-audit.log:7–50`, `E/scripts/reference_audio_separator_audit.py:112–135,202–228`, `E/scripts/compare_stems_per_channel.py:54–123`, `E/logs/comparison-per-channel-r2.json`. Разделение кандидата и reference/comparison: exit **0**, `E/commands.txt`, `E/logs/utf8/candidate-run.log.utf8.log:95–100`. |
| **AC-2.2** | **met** | Audit содержит **149 поимённых строк** с версиями и лицензиями, включая прежние пропуски `tract-nnef`, `tract-onnx-opl`, `tract-transformers`; пустых лицензий **0**. Просмотренный генератор обходит dependency closure обоих корней и сверяет версии с lock-файлом. `cargo metadata --format-version 1` и `dependency_inventory.py`: exit **0**. Источники: `E/snapshot/docs/dependency-audit.md:52–237`, `E/scripts/dependency_inventory.py:78–158`, `E/logs/dependency-inventory.log`, `E/commands.txt:338–381`. |
| **AC-2.3** | **met** | `cargo build`: exit **0**; `cargo test`: exit **0**, **98 passed / 0 failed / 1 ignored**, включая MDX и **8 passed** BS-RoFormer. Просмотрены завершённые журналы. Повторное использование обосновано: **13/13** файлов соответствуют snapshot, изменён только audit-документ; код и Cargo-файлы неизменны. Источники: `E/logs/utf8/cargo-build-final.log.utf8.log:95–96`, `E/logs/utf8/cargo-test-final.log.utf8.log:103–249`, `E/logs/tree-sha256-verify-r2n2.log:43–49`, `E/commands.txt:69–95`. |
| **AC-2.4** | **met** — pre-acceptance | Дерево проверяемо, HEAD остаётся базовым, index пуст. Read-only GitHub-наблюдение: HTTP **200** для issue и timeline; #2 **open**, `closed_at=null`, одна страница timeline, **0 closed / 0 reopened**. Источники: `E/logs/tree-sha256-verify-r2n2.log:43–49`, `E/logs/github-preacceptance-state.json`, `E/logs/github-issue-2-timeline-raw.json`, просмотренный REST-путь `E/scripts/github_preacceptance_probe.py:164–225`. Коммит и closure остаются обязательством ACT. |

### Findings

- **F-2-dependency-inventory — major, refuted для текущего кандидата.** Поимённый version/licence-инвентарь теперь присутствует; прежние конкретные пропуски закрыты.
- **F-2-github-preacceptance-state — major, refuted.** Доказательный пробел закрыт наблюдением issue и полной полученной timeline, а не заявлением автора.
- **F-2-evidence-map — minor, refuted.** `E/manifest.json:281–322` теперь правильно сопоставляет все четыре критерия и отдельно обозначает будущее закрытие.
- **F-2-reference-provider — major, refuted.** Непосредственное `session.get_providers()` подтверждает активный CPU-provider.

**Исключение: environment/infrastructure.** Отсутствие `gh` подтверждено probe exit **127** и `where.exe gh` exit **1**. Read-only REST API покрывает требуемое наблюдение. Это не подтверждает наличие либо отсутствие иных credentials.

### Пределы и уверенность

Проверка выполнена чтением snapshot, скриптов и журналов; команды и независимый пересчёт аудиометрик не выполнялись. Вывод ограничен замороженной фикстурой и просмотренными тестами. Один opt-in тест реального BS-checkpoint остаётся ignored. Подтверждено отсутствие дополнительной нормализации **при сравнении**; штатная обработка уровней внутри разделителей присутствует.

Уверенность высокая; блокирующих пробелов не установлено.

### Следующий переход: **→ ACT**

Выполнить обязательное постприёмочное закрытие: коммит изменений #2 со ссылкой на `#2`, затем закрытие GitHub #2 с отчётом по AC-2.1–AC-2.3. Зафиксировать фактические результаты обоих действий; данный вердикт не объявляет их выполненными.
````

Accepted on the frozen `r2`/`n2` candidate; the commit and the GitHub `#2` closure remain unperformed
and are the ACT obligation. This record states acceptance only — it performs no closure.

## ACT record

- **Outcome**: `accepted`. The ending phase is `pdca-check` CHECK 3 (`r2`/`n2`) on the frozen
  candidate — HEAD `2d0be53` plus the 13-path uncommitted tree. This record states that outcome and
  performs its post-acceptance closure; it issues no new verdict and runs no CHECK.
- **Criteria outcome**: `AC-2.1` **met** · `AC-2.2` **met** · `AC-2.3` **met** · `AC-2.4` **met** on
  its pre-acceptance component, and its post-acceptance closure action is performed by this ACT
  (the commit here; the GitHub `#2` closure is the handoff below). No criterion was re-scored, waived
  or narrowed; the user's `Done when` and the approved tolerance stay as recorded in §Original task.
- **Accepted snapshot identity**: HEAD `2d0be535fd82a1296a76f74b0d738829bd54a6df` + the 13-file tree
  frozen at `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/snapshot/` and listed in `snapshot-sha256.txt`.
  Re-anchored immediately before the first staging action: **13/13 matched**, 0 mismatched,
  0 missing, 0 staged; `docs/dependency-audit.md` = sha256
  `527862c0e88b194479f150526e69afe66b94831904f3a715a555fb4937256b74` (21,256 B), `Cargo.lock`
  `0a1bb137…`, `Cargo.toml` `6a7209cd…`, `src/mdx.rs` `a3d38865…`, `README.md` `2aea14f0…`
  (log `logs/act-reanchor-verify.log`). No drift of any kind — no `Notice` is required.
- **Completed work — one atomic commit**:
  `59fb1fbcf89ed99a447dc5b8d48b915de601917f`
  (`59fb1fb #2: MDX-Net ONNX separation on CPU with audio-separator 0.48.0 parity`) on `main`,
  parent `2d0be535fd82a1296a76f74b0d738829bd54a6df`, **13 files changed, +1750 / −23**, footer
  `Refs #2` (no auto-close keyword — closure is the driver's explicit step). The staged set was
  verified with `git diff --cached --name-only` as exactly the 13 item paths and nothing else
  (`Cargo.lock`, `Cargo.toml`, `README.md`, `docs/dependency-audit-evidence/tract-onnx-0.23.10.txt`,
  `docs/dependency-audit.md`, `examples/mdx_separate.rs`, `src/config.rs`, `src/dsp.rs`,
  `src/error.rs`, `src/lib.rs`, `src/mdx.rs`, `src/model/mod.rs`, `src/separator.rs`). No push, no
  branch switch, no amend/rebase; `.pdca/**`, `.openidea/**`, any `.onnx`, any fixture or audio
  remained unstaged. Evidence: `logs/commit-59fb1fb.log` (HEAD, parent, author, message,
  `--name-status`, an empty `git diff --name-only HEAD -- <13 paths>`, porcelain).
- **Residual — GitHub issue `#2` closure is not performed here.** Writing to GitHub is outside this
  executor's authorization and belongs to the calling driver, which executes the closure after this
  commit and reports the `AC-2.1`–`AC-2.3` outcome. The item therefore stays `accepted` with exactly
  this one residual obligation, and the collection records it as `done` / `finalized` with
  `GitHub closure pending by the calling driver` — never as closed.
- **Handoff / next authorized action**: (a) the calling driver closes GitHub `#2`; (b) the collection
  then proceeds to item `#8`, `DO item #8 (r1/n1)`. Nothing else in item `#2` is outstanding.
- **Limitations carried from CHECK 3 (unchanged)**: the verdict is bounded by the frozen fixture and
  the reviewed tests — one opt-in real BS-RoFormer checkpoint test stays `ignored`; audio metrics
  were not independently recomputed by CHECK; `gh` is absent in this environment (`where.exe gh`
  exit `1`, probe exit `127`), so the pre-acceptance observation used the unauthenticated read-only
  GitHub REST API and this establishes nothing about other credentials. No exceptions were taken.

**External GitHub closure (append-only, recorded `2026-11-11`).** GitHub issue `#2` was closed as **`COMPLETED`** by the driver at **`2026-11-11T06:18:29Z`** (comment id `6788661305`), after the accepted verdict (`CHECK 3`, `r2`/`n2`) and product commit `59fb1fbcf89ed99a447dc5b8d48b915de601917f`. That commit is on `main` and **not pushed** (`origin/main` unchanged). This discharges the single residual recorded above: the item is `done` / `finalized` with terminal outcome (cycle) `accepted`. The accepted verdict and the criteria matrix (`AC-2.1`–`AC-2.4` `met`, `AC-2.4` on its pre-acceptance component with the commit action performed) are unchanged by this note. The closure was performed with the authenticated GitHub CLI at `C:\Program Files\GitHub CLI\gh.exe`, which supersedes the earlier `gh not installed` environment note as a current fact — that note remains the true record of the pre-acceptance observation and the read-only REST probe.

## Lessons learned

Recorded verbatim from the `pdca-check` retrospective report routed by the driving role (source: the
ACT dispatch of 2026-11-10 for cycle `issue-2-r5`, `N=1`, `r=2`, `n=2/3`). Complete original,
unchanged; recorded bytes are those of the report.

```text
## Ретроспектива `issue-2-r5`

Основание: сохранённые PLAN, DO и три полных отчёта CHECK в `.pdca/status/issue-2-r5.md`. Новые проверки не выполнялись.

### Уроки

1. **Категория: планирование и фазовый порядок.**
   В r1 пропущено требование лицензионной документации, а коммит и закрытие issue ошибочно поставлены условием предшествующей им проверки. В r2 эти дефекты устранены без изменения пользовательского допуска. **В следующий раз:** сопоставлять каждую часть `Done when` с производными критериями и сразу отделять проверяемое состояние от последующих действий закрытия.
   **Источники:** status:65–116, 411–434.

2. **Категория: наблюдаемость и трассируемость.**
   Доступность CPU-provider не доказывала его использование; заявление об отсутствии закрытия issue не заменяло наблюдения GitHub. Инструментирование живой сессии, read-only REST-наблюдение и сгенерированный инвентарь из 149 зависимостей закрыли конкретные пробелы. **В следующий раз:** заранее собирать читаемые доказательства фактического исполнения и состояния, а при пересмотре критериев одновременно обновлять manifest.
   **Источники:** status:261–313, 334–369, 465–475, 516–528.

3. **Категория: воспроизводимость и ограничение изменений.**
   Замороженные снимки и SHA-256 позволили сохранить численную реализацию: r2/n1 не менял продукт, r2/n2 изменил только документацию. Повторное использование build/test-логов было обосновано неизменностью кода и Cargo-файлов. **В следующий раз:** сохранять привязку журналов к снимку и отдельно фиксировать изменения документации и исполняемых файлов. Это могло бы сократить лишние прогоны; экономия времени здесь не измерялась.
   **Источники:** status:314–325, 370–383, 518.

### Пределы и неопределённость

- Выводы основаны на существующих отчётах; аудиометрики независимо не пересчитывались. Обобщение за пределы замороженной фикстуры не обосновано.
- Один тест реального BS-RoFormer checkpoint остался `ignored`.
- Отсутствие `gh` компенсировано REST-наблюдением; состояние иных credentials этим не установлено.
- Коммит и закрытие GitHub #2 на момент ретроспективы ещё не выполнены: ACT в status остаётся `pending` (строки 541–550).
- Эскалации и использования долговременной памяти хоста не было.

**Сигнал: retrospective complete — анализ завершён; сохранение отчёта исполнителем ещё требуется.**
```

Recorded by `pdca-executor` as the single-writer persistence step of `references/act.md` §Closing
order 1. The retrospective adds no verdict, criterion result, counter, attempt or verification. Its
one dated statement is resolved by this record: the commit exists as `59fb1fb`, and the ACT slot is
now `completed` rather than `pending`; the GitHub `#2` closure stays the recorded residual above.

## Revision history

- 2026-11-10T23:27:02Z | r1 → r2 | genuine contract revision on the `CHECK → PLAN` return: `F-2-criteria-coverage` (major, confirmed — the mandatory `dependency documented in docs/dependency-audit.md with licence` clause was missing from `AC-2.1`–`AC-2.3`) and `F-2-acceptance-order` (blocking, confirmed — `AC-2.3` demanded the commit and the GitHub `#2` closure that the plan allows only after an accepted CHECK). The revised contract adds `AC-2.2` (licensed dependency documentation) and `AC-2.4` (pre-acceptance state plus mandatory post-acceptance closure) and keeps the user's `Done when` and the approved tolerance unchanged. Outgoing attempt: `r1/n1, not accepted by CHECK`.

## Defect history

- `F-2-criteria-coverage` | observed at r1 / attempt `n1` (CHECK `not accepted`; major, confirmed) | fixes applied 1 — criteria revision `r2` adds `AC-2.2` (licensed dependency documentation) to the contract | cause: `AC-2.1`–`AC-2.3` of `r1` omitted the mandatory `dependency documented in docs/dependency-audit.md with licence` part of item `#2`'s `Done when` (`.pdca/open-issues-r5/rv1/plan.md:9`); the document itself is present in the candidate (`E/snapshot/docs/dependency-audit.md:25–37`) — a criteria-coverage defect, not a claim about implementation | evidence: this record's §CHECK record (report, findings); `.pdca/open-issues-r5/rv1/plan.md:9`; §PLAN record `r2` §3 `AC-2.2`, §5 gap 1 | last recurrence: first occurrence — routed `CHECK → PLAN`; **closed at r2 by `AC-2.2`** (contract gap closed, the criterion itself awaits CHECK re-verification; no escalation)
- `F-2-acceptance-order` | observed at r1 / attempt `n1` (CHECK `not accepted`; **blocking**, confirmed) | fixes applied 1 — criteria revision `r2` adds `AC-2.4`, splitting the pre-acceptance state (checked by CHECK, no commit) from the mandatory post-acceptance closure | cause: `AC-2.3` of `r1` required the commit and the GitHub `#2` closure to be already performed while the PLAN permits both only after an accepted CHECK (`.pdca/open-issues-r5/rv1/plan.md:9,11`) — a cyclic acceptance condition that CHECK may not replace with a promise | evidence: this record's §CHECK record (report, findings); `.pdca/open-issues-r5/rv1/plan.md:9,11`; §PLAN record `r2` §3 `AC-2.4`, §5 gap 2 | last recurrence: first occurrence — routed `CHECK → PLAN`; **closed at r2 by `AC-2.4`** (the cycle no longer depends on an action CHECK cannot authorize; no escalation)
- `F-2-reference-provider` | observed at r1 / attempt `n1` (CHECK `not accepted`; major, `unverified` — evidence-only gap) and re-observed at r2 / attempt `n1` only as a recorded-then-closed item | fixes applied 1 — at `r2`/`n1` the reference driver was instrumented **as an evidence-root copy** (`scripts/reference_audio_separator_audit.py`) to capture the **live session's** `get_providers()`, and the reference separation was re-run on the same fixture and the same `.onnx` sha256 | cause: the `r1` reference run's *active* CPU provider was not directly confirmed — the reference log was stored as unreadable binary and `logs/reference-summary.json:8–15` listed the **available**, not the selected, providers, which left `AC-2.1` `unverified` | closure evidence: `logs/reference-provider-audit.log` (pure ASCII, `sessions_created=1`, `requested_providers=["CPUExecutionProvider"]`, `active_providers=["CPUExecutionProvider"]` from `session.get_providers()`, `tensor(float)` in/out, `torch.cuda.is_available()=False`) with `.json`; `logs/reference-r2-run.log` exit `0`; PCM equivalence `logs/reference-pcm-equivalence.log` exit `0`; unchanged metrics `logs/comparison-r1-vs-r2.log` exit `0`; tree hashes `logs/tree-sha256-verify.log` exit `0`; corroborating readable `r1` excerpt `logs/utf8/reference-run.log.utf8.log:398` | last recurrence: first occurrence at `r1`/`n1` (CHECK remedy: a readable excerpt naming the session provider, else instrumented observation through the driver) — **closed at `r2`/`n1`** by the instrumented observation, with no product change required and no escalation; **re-evaluated at CHECK 2 (`r2`/`n1`): `refuted for r2`** — `AC-2.1` is `met` and the former evidence gap is closed by direct `session.get_providers()`; **re-evaluated at CHECK 3 (`r2`/`n2`): `refuted/closed on the r2/n2 candidate`** — `AC-2.1` stays `met`, the active CPU provider is confirmed by the live session and the four stem/channel rows stay inside every approved threshold, so the key stays closed, shows no recurrence and needs no new measure (fixes applied 1, unchanged); no escalation
- `F-2-dependency-inventory` | observed at r2 / attempt `n1` (CHECK 2 `not accepted`; major, confirmed) | fixes applied 2 — at `r2`/`n2` `docs/dependency-audit.md` gained the subsection `#### Resolved dependency closure of tract-onnx and sha2`: a 149-row table (crate, resolved version, SPDX licence from the crate's own metadata, role/why) covering the full `Cargo.lock` closure of `tract-onnx` (141 crates) and `sha2` (10), generated from `cargo metadata --format-version 1` + `Cargo.lock` + `cargo tree`, with 0 rows lacking a licence | cause: `docs/dependency-audit.md` recorded the direct dependencies (`tract-onnx 0.23.10`, `sha2 0.10.9`) with licences, but `AC-2.2` requires **every** dependency of the MDX build graph with version and licence, and `tract-nnef`, `tract-onnx-opl` and `tract-transformers` were in the Cargo lock graph without an individual entry; a general statement about graph licences did not cover it | evidence: §CHECK record `### CHECK 2 — r2/n1` (finding `F-2-dependency-inventory`); fix: `docs/dependency-audit.md` (sha256 `527862c0…`, 21,256 B), `E/logs/dependency-inventory.json`/`.md`/`.log`, `E/logs/cargo-metadata-r2n2.json`, `E/logs/cargo-tree-host-r2n2.log`, `E/logs/splice-audit-table.log`, `E/logs/tree-sha256-verify-r2n2.log` | last recurrence: first occurrence — routed `CHECK → DO`; **closed at `r2`/`n2`** (the named inventory now exists for the resolved graph; no dependency was added or changed, `Cargo.toml`/`Cargo.lock` byte-identical to `r1`); **CHECK 3 (`r2`/`n2`) refuted it on the frozen candidate — `refuted/closed on the r2/n2 candidate`** (`AC-2.2` `met`: 149 named rows with versions and licences, including the former `tract-nnef`/`tract-onnx-opl`/`tract-transformers` gaps, 0 rows without a licence); the two fixes are one `r2`/`n2` measure plus its CHECK 3 closure confirmation, not two product changes; no escalation
- `F-2-github-preacceptance-state` | observed at r2 / attempt `n1` (CHECK 2 `not accepted`; major, `unverified` — evidence-only gap) | fixes applied 2 — at `r2`/`n2` an independent read-only observation was captured: `gh` is not installed in this environment (`where.exe gh` exit `1`; probe exit `127`, `FileNotFoundError: [WinError 2]`), `gh auth status` recorded `not set` (no token read), and the state was then read unauthenticated through the GitHub REST API the CLI wraps — `GET /repos/AlexeyShirshov/candle-roformer/issues/2` HTTP `200` and its timeline HTTP `200`: `state=open`, `state_reason=null`, `closed_at=null`, `created_at 2026-11-08T06:43:12Z`, timeline 1 event (`milestoned`), **0 `closed`**, **0 `reopened`**, 0 comments | cause: there was no independent observation of the GitHub `#2` state and of its close/reopen timeline, so the `AC-2.4` pre-acceptance condition (no premature commit/closure) could not be evidenced; the DO statement that no closure was performed was a statement, not an observation | evidence: §CHECK record `### CHECK 2 — r2/n1` (finding `F-2-github-preacceptance-state`); fix: `E/logs/github-preacceptance-state.log`/`.json`, `E/logs/github-issue-2-raw.json`, `E/logs/github-issue-2-timeline-raw.json`, `E/scripts/github_preacceptance_probe.py`; `manifest.json` › `preacceptance_github_observation` | last recurrence: first occurrence — routed `CHECK → DO`; **closed at `r2`/`n2`** as an evidence gap (an observation now exists and shows no premature closure); **CHECK 3 (`r2`/`n2`) refuted it — `refuted/closed on the r2/n2 candidate`** (`AC-2.4` pre-acceptance `met`: HTTP `200` for the issue and its timeline, `#2` `open`, `closed_at=null`, one timeline page, 0 `closed`, 0 `reopened`); the two fixes are one `r2`/`n2` measure plus its CHECK 3 closure confirmation, not two product changes; the `gh` CLI absence stays recorded as an environment finding, not inferred away, and the post-acceptance closure stays unperformed by design and is judged at closure; no escalation
- `F-2-evidence-map` | observed at r2 / attempt `n1` (CHECK 2 `not accepted`; minor, confirmed) | fixes applied 2 — at `r2`/`n2` `manifest.json` › `criterion_evidence` was rebuilt on the authoritative `r2` set: `AC-2.1` CPU separation + tolerance parity, `AC-2.2` dependency/licence documentation, `AC-2.3` build/tests no regressions, `AC-2.4` pre-acceptance tree state + post-acceptance closure contract, each with evidence matching its actual text | cause: the criterion mapping in `manifest.json:296–315` was stale — `AC-2.2` was described as the regression gate and `AC-2.3` as the pre-acceptance state, which hampered tracing though it did not change the authoritative `r2` contract | evidence: §CHECK record `### CHECK 2 — r2/n1` (finding `F-2-evidence-map`); fix: `manifest.json` › `criterion_evidence` (rebuilt), `E/logs/tree-sha256-verify-r2n2.log` | last recurrence: first occurrence — routed `CHECK → DO`; **closed at `r2`/`n2`** (mapping now matches the `r2` contract); **CHECK 3 (`r2`/`n2`) refuted it — `refuted/closed on the r2/n2 candidate`** (`manifest.json:281–322` maps all four criteria and marks the future closure separately); the two fixes are one `r2`/`n2` measure plus its CHECK 3 closure confirmation, not two product changes; no escalation

`E` = `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/` (this cycle's evidence root).

## Changed files

- `src/mdx.rs` → new (MDX-Net ONNX runtime, pinned profile, checksum gate, demix/STFT path).
- `examples/mdx_separate.rs` → new (documented CPU separation example).
- `src/dsp.rs` → added STFT/ISTFT helpers used by the MDX path.
- `src/error.rs` → added `MdxError` and its `Error` variant.
- `src/lib.rs` → `pub mod mdx` + `MdxError`/`mdx` re-exports (merged with `#1`'s re-exports).
- `src/config.rs` → `ModelFamily::MdxNet` (serde rename + aliases), `InferenceConfig::mdx()`,
  `for_family` arm, `validate` arm (merged with `#1`'s serde-renamed variants).
- `src/separator.rs` → exhaustive `ModelFamily` match arm for `MdxNet` (conflict caused by `#1`).
- `src/model/mod.rs` → exhaustive `ModelFamily` match arm for `MdxNet` (conflict caused by `#1`).
- `Cargo.toml`, `Cargo.lock` → `tract-onnx`/`sha2` dependencies (byte-identical to the r4 candidate).
- `docs/dependency-audit.md`, `docs/dependency-audit-evidence/tract-onnx-0.23.10.txt` → licence audit.
- `README.md` → MDX-Net row/status entry merged into `#1`'s README; measured numbers restated from
  this run (SDR 118.9 dB).
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/` → evidence root (snapshot, logs, scripts, manifest).
- `.pdca/status/issue-2-r5.md` → CHECK report recorded verbatim in §CHECK record; §Defect history opened
  with three keys; this is a status-record write only, no product file touched.
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/scripts/reference_audio_separator_audit.py` → new (r2/n1:
  instrumented reference driver copy; records each live ONNX Runtime session's providers).
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/scripts/check_reference_pcm_equivalence.py` → new (r1 vs r2
  reference PCM equivalence).
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/scripts/compare_r2.py` → new (runs the preserved
  comparators byte-identically against the fresh reference directory).
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/scripts/compare_metrics_r1_r2.py` → new (metric-identity
  check between the r1 and r2 comparison artefacts).
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/scripts/verify_snapshot_sha256.py` → new (13-file tree
  hash verification against `snapshot-sha256.txt`).
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/{reference-provider-audit.log,reference-provider-audit.json,reference-summary-r2.json,reference-r2-run.log,reference-pcm-equivalence.log,reference-pcm-equivalence.json,comparison-r2.json,comparison-per-channel-r2.json,compare-r2.stdout.log,compare-metrics-r1-r2.stdout.log,comparison-r1-vs-r2.log,comparison-r1-vs-r2.json,tree-sha256-verify.log,tree-sha256-verify.json}`
  → new r2/n1 evidence (all pure ASCII).
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/out/reference-r2/` → new (fresh CPU reference stems; the r1
  reference in `out/reference/` untouched).
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/{manifest.json,commands.txt,index.md}` → refreshed for r2/n1.
- No product file was touched in `r2`/`n1`; `snapshot-sha256.txt`, `snapshot/`, `snapshot.diff`,
  `comparison.json` and the `r1` logs are unchanged.
- No weights, fixtures or audio enter a tracked path; the `.onnx` and the fixture live only under
  `.pdca/`.
- `.pdca/status/issue-2-r5.md` → updated for CHECK 2 (`r2`/`n1`, verdict `not accepted → DO`):
  §Current state (phase `CHECK`), the verbatim `### CHECK 2 — r2/n1` report, three new defect keys
  plus `F-2-reference-provider` recorded `refuted for r2`, §Changed files and progress rows. This is
  a status-record write only: no product file, no evidence artifact and no gate was touched.
- `docs/dependency-audit.md` → **r2/n2: the only product-tree change of this attempt.** Added
  `#### Resolved dependency closure of tract-onnx and sha2` (149-row table: crate, resolved version,
  SPDX licence, role) plus the generation note; 6,353 B → 21,256 B, sha256
  `88ea8e93…` → `527862c0e88b194479f150526e69afe66b94831904f3a715a555fb4937256b74`; CRLF preserved.
  `AC-2.2`.
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/{snapshot/docs/dependency-audit.md,snapshot-sha256.txt,snapshot.diff}`
  → refreshed for the new product bytes (diff 83,461 B, sha256
  `e5d28adf7845aff00f3a872c577b7771bded26283a14d19aa63bd75a36e6628f`); the other 12 `snapshot/` copies
  are untouched and still match the worktree.
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/scripts/` → r2/n2 new helpers:
  `dependency_inventory.py` (closure + licence table from `cargo metadata`/`Cargo.lock`),
  `cargo_tree_host.py` (host build graph), `splice_audit_table.py` (idempotent table insertion into
  the audit doc), `github_preacceptance_probe.py` (read-only issue/timeline observation),
  `stage_snapshot_r2n2.py` (snapshot + diff + ledger refresh), `verify_snapshot_sha256_r2n2.py`
  (13-file ledger + r1-baseline comparison), `stage_manifest_r2n2.py` (manifest refresh).
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/` → r2/n2 new evidence (all pure ASCII):
  `cargo-metadata-r2n2.json` (1,193,103 B raw `cargo metadata`) + `.err`;
  `cargo-tree-host-r2n2.log`/`.json`/`.stdout.log`; `dependency-inventory.json`/`.md`/`.log`/`.stdout.log`;
  `splice-audit-table.log`; `github-preacceptance-state.log`/`.json`, `github-issue-2-raw.json`,
  `github-issue-2-timeline-raw.json`, `github-preacceptance-stdout.log`; `stage-snapshot-r2n2.log`;
  `tree-sha256-verify-r2n2.log`/`.json`, `tree-verify-r2n2.stdout.log`.
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/{manifest.json,commands.txt,index.md}` → refreshed for
  `r2`/`n2` (new hashes, rebuilt criterion mapping, `dependency_inventory` and
  `preacceptance_github_observation` blocks, updated `gate_provenance`).
- No `src/`, `examples/`, `Cargo.toml`, `Cargo.lock` or `README.md` change in `r2`/`n2`; no dependency
  change; no staging, commit, push or GitHub write.
- `.pdca/status/issue-2-r5.md` → updated for CHECK 3 (`r2`/`n2`, verdict `accepted → ACT`): the verbatim
  report under §CHECK record `### CHECK 3 — r2/n2`, §Current state (phase `CHECK`, record `open`,
  outcome `pending`), the four item keys marked `refuted/closed on the r2/n2 candidate`, progress rows.
  Status-record write only — no product file, no evidence artifact and no gate was touched.
- **ACT commit** `59fb1fb` — the 13 item paths, and only those, now live in one atomic commit on
  `main` (parent `2d0be53`, 13 files, +1750/−23): `Cargo.lock`, `Cargo.toml`, `README.md`,
  `docs/dependency-audit.md`, `docs/dependency-audit-evidence/tract-onnx-0.23.10.txt`,
  `examples/mdx_separate.rs`, `src/config.rs`, `src/dsp.rs`, `src/error.rs`, `src/lib.rs`,
  `src/mdx.rs`, `src/model/mod.rs`, `src/separator.rs`. Content is byte-identical to the accepted
  snapshot (13/13 re-anchored before staging); the commit adds no product change of its own.
- `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/{act-reanchor-verify.log,commit-59fb1fb.log}` → new
  ACT evidence (pre-commit 13/13 hash re-anchor; commit identity, message, `--name-status`, empty
  post-commit diff for the 13 paths, porcelain). Evidence-root only — ignored by `/.pdca/*`.
- `.pdca/status/issue-2-r5.md` → ACT finalization: §Current state (`phase: ACT · record: completed ·
  outcome: accepted`), §ACT record, §Lessons learned (retrospective verbatim), §Changed files,
  §Git state and progress rows. Status-record write only — no product file, no evidence artifact and
  no gate was touched after the commit.

## Git state

- status path: `.pdca/status/issue-2-r5.md` — created and modified this cycle · untracked (durable on
  disk, not committed).
- baseline supplied at creation: branch `main`, HEAD `2d0be535fd82a1296a76f74b0d738829bd54a6df`
  (`2d0be53`); pre-existing untracked `.openidea/` and `.pdca/status/collection-open-issues-r3.md`
  left untouched and unclaimed.
- task changes at the DO stages, then **pending commit and not indexed** — superseded by the ACT
  commit entry below: `Cargo.lock`, `Cargo.toml`, `README.md`,
  `docs/dependency-audit.md`, `docs/dependency-audit-evidence/tract-onnx-0.23.10.txt`,
  `examples/mdx_separate.rs`, `src/config.rs`, `src/dsp.rs`, `src/error.rs`, `src/lib.rs`,
  `src/mdx.rs`, `src/model/mod.rs`, `src/separator.rs`.
- no `git add`, no staging, no commit, no push was performed in DO; the commit was ACT work after an
  accepted CHECK and is recorded above.
- `r2`/`n1` touched **no product path**: the 13 files pending commit were re-hashed and are 13/13
  identical to the `r1` candidate snapshot, so the pending-commit set and its bytes are unchanged and
  still not indexed (`logs/tree-sha256-verify.log`, exit `0`). Only evidence-root and status paths
  changed in this attempt.
- `r2`/`n2` touched **one product path**: `docs/dependency-audit.md` (documentation only). The
  pending-commit set is still the same 13 paths, still not indexed; 12 of them carry the `r1` bytes
  and `docs/dependency-audit.md` now carries sha256
  `527862c0e88b194479f150526e69afe66b94831904f3a715a555fb4937256b74` (21,256 B).
  `Cargo.toml`, `Cargo.lock` and every `.rs` file are byte-identical to `r1`; `HEAD` unchanged;
  nothing staged (`logs/tree-sha256-verify-r2n2.log`, exit `0`).
- the evidence root `.pdca/open-issues-r5/` is covered by `.gitignore:5` (`/.pdca/*`), so it never
  appears in `git status` and cannot be staged by accident; the status files under `.pdca/status/`
  are the only un-ignored records and they stay untracked.
- CHECK 3 (`r2`/`n2`) recording touched **only** `.pdca/status/issue-2-r5.md` (this file) and
  `.pdca/status/collection-open-issues-r5.md`; no product path, no evidence artifact, no gate. `HEAD`
  stays `2d0be53`, nothing staged or committed; the 13-path pending-commit set and its bytes are
  unchanged from `r2`/`n2`.
- **ACT (`r2`/`n2`, closure) — the pending-commit set is now committed.** Re-anchor before the first
  staging action: 13/13 files matched `snapshot-sha256.txt`, 0 mismatched, 0 missing, 0 staged
  (`logs/act-reanchor-verify.log`). Staged with explicit paths only and verified with
  `git diff --cached --name-only` as exactly the 13 item paths — nothing else
  (`.pdca/**`, `.openidea/**`, any `.onnx`, any fixture/audio and the two status files stayed out).
  Commit `59fb1fbcf89ed99a447dc5b8d48b915de601917f` (parent `2d0be53`, `main`, 13 files,
  +1750/−23, footer `Refs #2`); no push, no branch switch, no amend/rebase, no tag. `git diff
  --name-only HEAD -- <13 paths>` is empty afterwards, i.e. the committed bytes equal the accepted
  worktree bytes (`logs/commit-59fb1fb.log`).
- `HEAD` is now `59fb1fbcf89ed99a447dc5b8d48b915de601917f` on `main`; after the commit and the ACT
  status writes, `git status --porcelain` shows only the pre-existing untracked `.openidea/` and
  `.pdca/` records (`collection-open-issues-r3.md`, `collection-open-issues-r5.md`, `issue-2-r5.md`)
  and no leftover product change. The two status files remain untracked: they are durable on disk,
  and the authorized commit scope covered the accepted item's 13 product paths only.
- the evidence root stays out of `git status` via `.gitignore:5` (`/.pdca/*`), so the ACT logs
  (`act-reanchor-verify.log`, `commit-59fb1fb.log`) are durable on disk and cannot be staged by
  accident. Prior dirty work (`.openidea/`, `.pdca/status/collection-open-issues-r3.md`) stays
  untouched and unclaimed.

## Progress log

- 2026-11-10T18:12:11Z | DO | r1 | n=1/3 | DO started (autonomous): apply the preserved r4 candidate onto HEAD 2d0be53, reconcile with #1 | .pdca/open-issues-r4/rv1/item-2-mdx-onnx/patch/item-2-candidate.patch
- 2026-11-10T18:12:11Z | DO | r1 | n=1/3 | patch applied; conflicts: README.md (3-way), src/config.rs + src/lib.rs (context vs #1) reconciled by hand; separator.rs + model/mod.rs MdxNet arms added; numerical core byte-identical (CRLF-normalised) to the candidate snapshot | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/manifest.json
- 2026-11-10T18:12:11Z | DO | r1 | n=1/3 | checkpoint sha256 verified 317554b07fe1ea5279a77f2b1520a41ea4b93432560c4ffd08792c30fddf9adc (66,759,214 B) | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/checkpoint-sha256.log
- 2026-11-10T18:12:11Z | DO | r1 | n=1/3 | `cargo build` exit 0 | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/cargo-build-final.log
- 2026-11-10T18:12:11Z | DO | r1 | n=1/3 | `cargo test` exit 0 — 98 passed, 0 failed, 1 ignored | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/cargo-test-final.log
- 2026-11-10T18:12:11Z | DO | r1 | n=1/3 | `cargo clippy --all-targets` exit 0 — warnings only | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/cargo-clippy-final.log
- 2026-11-10T18:12:11Z | DO | r1 | n=1/3 | candidate separation exit 0 — 882281 frames × 2 ch @ 44100, CPU/f32, 80.307 s | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/candidate-run.log
- 2026-11-10T18:12:11Z | DO | r1 | n=1/3 | reference audio-separator 0.48.0 run exit 0 (fresh, CPU) — same fixture and .onnx sha256; audio payload bit-identical to r4 | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/reference-run.log
- 2026-11-10T18:12:11Z | DO | r1 | n=1/3 | comparison exit 0 — overall max_abs 1.4230609e-6, RMSE 1.4654514e-7, SDR 118.897 dB, corr 0.999999999999; no trimming, no normalisation | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/comparison.json
- 2026-11-10T18:20:16Z | DO | r1 | n=1/3 | evidence assembly completed: `wav_container_diff.py` exit 0 (delta +44 B = `fact` 12 B + `PEAK` 32 B, `data` payload 7,058,248 B both sides); `transcode_logs.py` exit 0 (18 UTF-8 copies); `commands.txt` + `index.md` written; `stage_manifest.py` exit 0 (comparison.json + manifest.json; HEAD 2d0be53, dirty, 0 staged, checkpoint/fixture/output sha256 and AC-2.1-AC-2.3 bound); log names corrected to `-final` in this record; status_lint exit 0 on issue-2-r5.md and collection-open-issues-r5.md | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/index.md
- 2026-11-10T23:22:49Z | CHECK | r1 | n=1/3 | CHECK ran on the r1/n1 candidate (HEAD 2d0be53 + recorded dirty tree): verdict **not accepted**, route → PLAN; AC-2.1 `unverified`, AC-2.2 `met`, AC-2.3 `unmet`; report recorded verbatim in this record | .pdca/status/issue-2-r5.md
- 2026-11-10T23:22:49Z | CHECK | r1 | n=1/3 | routing `CHECK → PLAN`: criteria defect (missing licensed-dependency coverage; cyclic acceptance order); defect keys F-2-criteria-coverage, F-2-acceptance-order (blocking) and F-2-reference-provider (unverified) recorded, fixes applied 0; revision stays r1, attempt n=1/3 | .pdca/status/issue-2-r5.md
- 2026-11-10T23:27:02Z | PLAN | r2 | n=1/3 | Replanned: CHECK→PLAN criteria contract (r 1→2) — outgoing attempt `r1/n1, not accepted by CHECK`; `F-2-criteria-coverage` closed by AC-2.2 and `F-2-acceptance-order` closed by AC-2.4, `F-2-reference-provider` still open | .pdca/status/issue-2-r5.md
- 2026-11-10T23:27:02Z | PLAN | r2 | n=1/3 | plan r2 ready — contract `AC-2.1`–`AC-2.4` recorded verbatim under §PLAN record `### r2`; counters N=1 · r=2 · n=1/3; phase DO, attempt `r2/n1` not started | .pdca/status/issue-2-r5.md
- 2026-11-10T18:32:48Z | DO | r2 | n=1/3 | DO started (autonomous): close `F-2-reference-provider` with observable active-provider evidence; no product change expected or made | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/scripts/reference_audio_separator_audit.py
- 2026-11-10T18:32:48Z | DO | r2 | n=1/3 | reference provider audit run exit 0 — 1 live session, requested+active `["CPUExecutionProvider"]`, `tensor(float)` f32 in/out, `torch.cuda.is_available()=False`, same `.onnx` sha256 317554b0…; pure-ASCII readable log (0 non-ASCII bytes) | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/reference-provider-audit.log
- 2026-11-10T18:32:48Z | DO | r2 | n=1/3 | reference separation re-run on CPU exit 0 (11.814 s) — outputs `out/reference-r2/`, 882281 frames × 2 ch @ 44100 each; PCM bit-identical to the r1 reference (`data` sha256 7fd76234… / 98fef60a…, frames equal; only `PEAK` differs) | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/reference-pcm-equivalence.log
- 2026-11-10T18:32:48Z | DO | r2 | n=1/3 | comparison re-run against the unchanged candidate exit 0 — all four metrics per stem × channel bit-identical to r1 (Vocals L 1.4230608940e-06 / 1.6870548336e-07 / 118.438951 dB / 0.9999999999993046 … Instrumental R 9.5367431641e-07 / 1.2043507525e-07 / 120.063622 dB / 0.9999999999995094); no trimming, no post-normalization | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/comparison-r1-vs-r2.log
- 2026-11-10T18:32:48Z | DO | r2 | n=1/3 | tree verify exit 0 — 13/13 files match `snapshot-sha256.txt`, 0 mismatches, HEAD still 2d0be53, nothing staged; r1 gates cited (not re-run) as still binding these bytes: cargo build/test/clippy exit 0, 98 passed / 0 failed / 1 ignored | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/tree-sha256-verify.log
- 2026-11-10T18:32:48Z | DO | r2 | n=1/3 | `F-2-reference-provider` closed (fixes applied 1); evidence root `manifest.json`/`commands.txt`/`index.md` refreshed; candidate ready, no commit/staging/GitHub closure (`AC-2.4`); next authorized step CHECK item #2 (r2/n1) | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/manifest.json
- 2026-11-10T18:38:42Z | CHECK | r2 | n=1/3 | CHECK 2 ran on the `r2`/`n1` candidate (HEAD 2d0be53 + 13-file snapshot): verdict **not accepted**, route → DO; AC-2.1 `met`, AC-2.2 `unmet`, AC-2.3 `met`, AC-2.4 `unverified`; complete report recorded verbatim under `### CHECK 2 — r2/n1` | .pdca/status/issue-2-r5.md
- 2026-11-10T18:38:42Z | CHECK | r2 | n=1/3 | routing `CHECK → DO`: findings `F-2-dependency-inventory` (major, confirmed), `F-2-github-preacceptance-state` (major, unverified) and `F-2-evidence-map` (minor, confirmed) recorded with fixes applied 0; `F-2-reference-provider` **refuted for r2**; contract `r2` unchanged, attempt `n=1/3` consumed | .pdca/status/issue-2-r5.md
- 2026-11-10T18:38:42Z | CHECK | r2 | n=1/3 | next authorized step: DO attempt `r2`/`n=2/3` (named dependency inventory of the MDX build graph, read-only GitHub pre-acceptance observation, manifest criterion mapping) — **not started**; no commit and no GitHub closure authorized | .pdca/status/issue-2-r5.md
- 2026-11-10T18:45:14Z | DO | r2 | n=2/3 | DO started (autonomous) on the three CHECK 2 items: named version/licence inventory of the `tract-onnx`+`sha2` closure, independent read-only GitHub `#2` observation, manifest criterion mapping | .pdca/status/issue-2-r5.md
- 2026-11-10T18:45:14Z | DO | r2 | n=2/3 | `cargo metadata --format-version 1` exit 0 — 1,193,103 B raw JSON persisted; resolve graph + per-package `license`; `Cargo.toml`/`Cargo.lock` untouched | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/cargo-metadata-r2n2.json
- 2026-11-10T18:45:14Z | DO | r2 | n=2/3 | `cargo_tree_host.py` exit 0 — host build graph per root (tract-onnx 110 unique, sha2 10) so `resolved only` rows are not presented as compiled | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/cargo-tree-host-r2n2.log
- 2026-11-10T18:45:14Z | DO | r2 | n=2/3 | `dependency_inventory.py` exit 0 — **149 rows** (tract-onnx closure 141 + sha2 closure 10, union), 2 direct, 118 host build / 31 resolved only, 15 distinct SPDX expressions, **0 rows without a licence**, every row in `Cargo.lock` at the same version | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/dependency-inventory.log
- 2026-11-10T18:45:14Z | DO | r2 | n=2/3 | `splice_audit_table.py` exit 0 — 149-row table inserted into `docs/dependency-audit.md` (6,353 B → 21,256 B, sha256 527862c0e88b194479f150526e69afe66b94831904f3a715a555fb4937256b74, CRLF preserved 262/0 bare LF); re-run byte-identical; **`F-2-dependency-inventory` closed** (fixes applied 1) | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/splice-audit-table.log
- 2026-11-10T18:45:14Z | DO | r2 | n=2/3 | `github_preacceptance_probe.py` exit 0 — `gh` **not installed** (probe exit 127, `FileNotFoundError: [WinError 2]`; `where.exe gh` exit 1), `gh auth status` = `not set` (no token read); observation via GitHub REST API (unauthenticated, read-only): repo `AlexeyShirshov/candle-roformer` from `git remote -v`, issue HTTP 200 + timeline HTTP 200 → **issue #2 `open`**, `state_reason=null`, `created_at 2026-11-08T06:43:12Z`, `closed_at=null`, timeline 1 event (`milestoned`), **0 `closed`**, **0 `reopened`**, 0 comments; **`F-2-github-preacceptance-state` closed** (fixes applied 1) | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/github-preacceptance-state.log
- 2026-11-10T18:45:14Z | DO | r2 | n=2/3 | `stage_snapshot_r2n2.py` exit 0 — `snapshot/docs/dependency-audit.md` refreshed, `snapshot.diff` 83,461 B sha256 e5d28adf7845aff00f3a872c577b7771bded26283a14d19aa63bd75a36e6628f, `snapshot-sha256.txt` 13 rows with `worktree==snapshot` on every row | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/stage-snapshot-r2n2.log
- 2026-11-10T18:45:14Z | DO | r2 | n=2/3 | `verify_snapshot_sha256_r2n2.py` exit 0 — **13/13** match, 0 mismatch, 0 missing; changed vs r1 = `docs/dependency-audit.md` only, unchanged 12/13; `.rs` files + `Cargo.toml` + `Cargo.lock` byte-identical to r1; HEAD 2d0be53; nothing staged → r1 cargo build/test/clippy cited, not re-run | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/tree-sha256-verify-r2n2.log
- 2026-11-10T18:46:08Z | DO | r2 | n=2/3 | `stage_manifest_r2n2.py` exit 0 — `manifest.json` rebuilt on the r2 criterion set (AC-2.1 CPU separation + tolerance parity, AC-2.2 dependency/licence documentation, AC-2.3 build/tests, AC-2.4 pre-acceptance state + post-acceptance closure), `attempt n2/3`, new `dependency_inventory` + `preacceptance_github_observation` blocks, re-hashed artefacts; **`F-2-evidence-map` closed** (fixes applied 1) | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/manifest.json
- 2026-11-10T18:46:08Z | DO | r2 | n=2/3 | candidate ready; no commit, no staging, no push, no GitHub write (AC-2.4 pre-acceptance state); next authorized step CHECK item #2 (r2/n2) | .pdca/status/issue-2-r5.md
- 2026-11-10T18:51:12Z | CHECK | r2 | n=2/3 | CHECK 3 ran on the frozen `r2`/`n2` candidate (HEAD 2d0be53 + uncommitted tree): verdict **accepted**, route → ACT; AC-2.1 `met`, AC-2.2 `met`, AC-2.3 `met`, AC-2.4 `met` (pre-acceptance component); complete report recorded verbatim under `### CHECK 3 — r2/n2` | .pdca/status/issue-2-r5.md
- 2026-11-10T18:51:12Z | CHECK | r2 | n=2/3 | item keys `F-2-dependency-inventory`, `F-2-github-preacceptance-state`, `F-2-evidence-map` and `F-2-reference-provider` marked `refuted/closed on the r2/n2 candidate` (fixes applied 2, 2, 2 and 1); contract `r2` unchanged, attempt `n=2/3` consumed; counter `n=3/3` not needed for a new candidate | .pdca/status/issue-2-r5.md
- 2026-11-10T18:51:12Z | CHECK | r2 | n=2/3 | next authorized step: ACT item #2 (mandatory retrospective + one atomic commit referencing `#2`; GitHub `#2` closure by the calling driver); no commit, no staging, no GitHub write performed by this recording | .pdca/status/issue-2-r5.md
- 2026-11-10T18:55:14Z | ACT | r2 | n=2/3 | ACT started (autonomous) after CHECK 3 `accepted → ACT`: re-anchored the frozen candidate before the first edit — **13/13** files match `snapshot-sha256.txt`, 0 mismatched, 0 missing, HEAD `2d0be53`, nothing staged, no drift (`Notice` not required) | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/act-reanchor-verify.log
- 2026-11-10T18:55:37Z | ACT | r2 | n=2/3 | retrospective recorded verbatim in §Lessons learned (source: the `pdca-check` retrospective report routed by the driving role; 3 lessons, limits and uncertainty, completion signal) — ACT is completed only on this persistence | .pdca/status/issue-2-r5.md
- 2026-11-10T18:55:37Z | ACT | r2 | n=2/3 | post-acceptance closure (commit part): staged exactly the 13 item paths with explicit `git add` and verified with `git diff --cached --name-only` (13 files, nothing else), then committed **59fb1fbcf89ed99a447dc5b8d48b915de601917f** — `#2: MDX-Net ONNX separation on CPU with audio-separator 0.48.0 parity`, parent `2d0be53`, 13 files changed +1750/−23, footer `Refs #2`; no push, no branch switch, no amend/rebase | .pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/commit-59fb1fb.log
- 2026-11-10T18:55:55Z | ACT | r2 | n=2/3 | ACT closed: outcome **accepted** (AC-2.1 met, AC-2.2 met, AC-2.3 met, AC-2.4 met on the pre-acceptance component; its post-acceptance commit action performed). Residual: GitHub `#2` closure is executed by the calling driver after this commit (no GitHub write by this executor). Next authorized action: driver closes `#2`, then item `#8` `DO (r1/n1)` | .pdca/status/issue-2-r5.md
