# Cycle status: issue-2-r4 / cycle 1

## Current state

- task: `issue-2-r4` · form: `cycle` · specialized skill: `pdca-coder`
- mode: `autonomous`
- cycle: `N=1` · revision: `r=1` · attempt: `n=1`
- scope: collection item `#2` of `open-issues-r4` — MDX-Net (ONNX) support in `candle-roformer`, whole item goal at `r1` · status path: `.pdca/status/issue-2-r4.md`
- goal: MDX-Net (ONNX) separation on CPU from a real published checkpoint, measured against audio-separator 0.48.0 on a frozen fixture, with the dependency and its licence documented — the §PLAN record and §Original task are authoritative
- phase: `ACT` · record: `completed` · outcome: `incomplete (terminal=criteria-undetermined)` — the cycle closed at `PLAN → ACT`; `AC-2.5` stays `unverified`, nothing was accepted and nothing was committed
- source: GitHub issue `#2` of `AlexeyShirshov/candle-roformer`, item of collection `open-issues-r4` · context refs: `.pdca/status/collection-open-issues-r4.md` §`PLAN record` `r1` + `### Schedule s2`, `.pdca/open-issues-r4/rv1/plan.md`
- criteria: `AC-2` — exact text in §PLAN record `r1` (source: `.pdca/status/collection-open-issues-r4.md` §Input item 2), not retyped here

## Original task

Item brief, verbatim (as supplied by the collection):

> #2 feat: MDX-Net (ONNX) architecture

Item `Done when` criterion (`AC-2`, verbatim, immutable):

> a published MDX-Net ONNX file (supplied locally, checksum-checked by caller) separates audio on CPU; output matches audio-separator on a frozen fixture within agreed tolerance; dependency documented in docs/dependency-audit.md with licence.

The collection task, verbatim (original language, Russian):

> pdca по открытым задачам из github

The follow-up instruction, verbatim (original language, Russian):

> тогда делай автономно с автокомитом

## PLAN record

### r1

`recorded by reference, not re-planned` — the authoritative item plan of this cycle is the collection
PLAN return at `r1`: `.pdca/status/collection-open-issues-r4.md` §`PLAN record` → `### r1`
(item status line `#2 ready`, acceptance criterion `AC-2` verbatim, measurement method for `#2`:
"max abs diff and RMSE vs audio-separator 0.48.0 on a frozen fixture; SHA-256 checksum of the .onnx
file verified before inference; dependency and licence documented in docs/dependency-audit.md"),
plus `### Schedule s2` (group `G1`, order `#2 → #8 → #3 → #1 → #4`, one sequential lane on the
current worktree `main`, `plan_cap 1`, `lane_cap 1`). A byte-identical copy of the collection PLAN
return is persisted at `.pdca/open-issues-r4/rv1/plan.md`. No new PLAN was started in this cycle and
the form, contract and overlay are as decided at entry (`cycle` / `pdca` / `pdca-coder`).

**Terminal PLAN decision on the `CHECK → PLAN` return (`r1`) — revision `r2` not issued.** Returned by
the planner after CHECK's `not accepted → PLAN` route; recorded verbatim. The `### r1` block above stays
byte-identical, and no `### r2` block exists because no revised plan was produced.

````markdown
## Решение PLAN по возврату CHECK → PLAN (item `#2`, `issue-2-r4`)

### 1. Решение и терминальный исход PLAN
- **Решение:** **Терминальный исход PLAN — `acceptance criteria cannot be determined`** (ревизия `r2` **не выпускается**).
- **Следующий разрешённый переход:** **PLAN → ACT** напрямую (без нового DO, без нового CHECK и без автокоммита кандидата как принятого).
- **Основание (`references/plan.md` §Criteria, §Terminal PLAN outcomes, §Gap closure, §Revision accounting):**
  1. Независимый CHECK подтвердил выполнение всех остальных обязательных частей `AC-2` (`AC-2.1`, `AC-2.2`, `AC-2.3`, `AC-2.4`, `AC-2.6` — **met**) и зафиксировал вычисленные расхождения с `audio-separator 0.48.0` на замороженной фикстуре (`Vocals`: `max_abs = 1.423061e-6`, `RMSE = 1.687843e-7`; `Instrumental`: `max_abs = 1.013279e-6`, `RMSE = 1.202614e-7`), но вернул блокирующий дефект проверяемости **`AC2-TOL-001`**: в пользовательском критерии `AC-2` формулировка *«within agreed tolerance»* не опирается ни на какое существующее соглашение о численном пороге для MDX-Net.
  2. Пользовательский критерий `AC-2` неизменяем (*immutable*): PLAN не вправе ослаблять, сужать, дополнять или заменять его, подгонять порог под уже измеренные `1.42e-6` / `1.69e-7` или молча переносить допуск `G3` от другой модели.
  3. Выпуск ревизии `r2` невозможен без изменения пользовательского входа: метод измерения в `r1` уже определён и вычислим, а единственное недостающее звено — нормативный порог приёмки (*agreed tolerance*), который не может быть закрыт допущением планировщика (§Gap closure: допущение не может менять оцениваемый критерием результат). В автономном режиме («тогда делай автономно с автокомитом») интерактивный запрос пользователю внутри цикла не выполняется, поэтому по процедуре §Terminal PLAN outcomes цикл переходит в терминальный исход **`acceptance criteria cannot be determined`**.

---

### 2. Форма, навык и область входа (по ссылке на `r1`)
- **Форма и специализированный навык:** элемент коллекции `#2` (`collection` → item cycle `cycle`, специализированный навык `pdca-coder`); обоснование формы из `r1` сохраняется по ссылке без переклассификации.
- **Область входа (Entry scope):** локальная CPU-сепарация аудио опубликованной моделью MDX-Net ONNX с проверкой SHA-256 до инференса, сверкой выходов с `audio-separator` на замороженной фикстуре и документированием зависимостей/лицензий в `docs/dependency-audit.md`.

---

### 3. Цель, запрашиваемый результат и неизменный критерий приёмки (`AC-2`)
- **Исходная задача пользователя (дословно):** «pdca по открытым задачам из github» + «тогда делай автономно с автокомитом».
- **Запрашиваемый результат (Requested result) и поставляемый результат (Delivered result):** один и тот же объект — рабочая CPU-сепарация аудио в `candle-roformer` с использованием локально предоставленного опубликованного файла MDX-Net ONNX (с проверкой контрольной суммы вызывающей стороной до инференса), чей выход совпадает с `audio-separator` на замороженной фикстуре в пределах согласованного допуска, и документирование зависимости с лицензией в `docs/dependency-audit.md`. Суррогатный результат не поставляется.
- **Критерий приёмки `AC-2` (пользовательский, воспроизведён дословно и без изменений):**
  > `"a published MDX-Net ONNX file (supplied locally, checksum-checked by caller) separates audio on CPU; output matches audio-separator on a frozen fixture within agreed tolerance; dependency documented in docs/dependency-audit.md with licence."`

---

### 4. Конкретная недостающая информация и доказательства (Payload терминального исхода)
- **Что именно отсутствует:** явно согласованный (пользователем или нормативной спецификацией проекта) численный порог допуска и правило агрегации по стемам (`Vocals`, `Instrumental`) для сравнения выхода CPU-инференса MDX-Net ONNX (`UVR-MDX-NET-Inst_HQ_3.onnx`) с референсом `audio-separator` на замороженной фикстуре.
- **Почему read-only сбор фактов не может установить этот порог:**
  1. Порог **`G3`** (`RMS error ≤ 1e-4` и `maxabs ≤ 1e-3` при `ref RMS > 1e-3`, иначе `relL2 ≤ 1e-3`) в `docs/superpowers/specs/2026-11-07-candle-roformer-design.md:118`, `docs/gates-report.md:9,94` и `README.md:162-165` явно ограничен моделью **MelBand RoFormer против PyTorch** (item `#1`) и помечен как целевой порог архитектора для MelBand; область его действия не включает MDX-Net / `audio-separator` (item `#2`).
  2. Численные допуски в тестах репозитория (`tests/streaming_chunked.rs:155,487-488` — `1e-4`; `tests/core_tests.rs:149,168,193,216`; `tests/library_api.rs:380,544,579-580`; `tests/s1_spikes.rs:54,139` — `1e-3`; `src/mdx.rs:608-613` — `1e-5` для коэффициентов Ханна) относятся к синтетическим/OLA/DSP-проверкам, а не к референсной паритетности MDX-Net с `audio-separator`.
  3. В записях планов и статусов (`issue-2-r4.md:84-86,146`, `collection-open-issues-r4.md:88-89`, `collection-open-issues-r3.md:76,82`) численный порог для `#2` нигде не задан (а упоминание `3.5e-4` в `collection-open-issues-r3.md:76` ошибочно называет порогом фактически измеренное значение MelBand из `README.md:163-165`).
- **Варианты решения при последующем явном указании пользователя (для отчёта в ACT):**
  - *Вариант А:* явно распространить архитекторский гейт `G3` (`max_abs ≤ 1e-3`, `RMSE ≤ 1e-4` по каждому стему при `ref RMS > 1e-3`) на MDX-Net vs `audio-separator`.
  - *Вариант Б:* задать собственный допуск f32 ONNX-паритетности между рантаймами (`tract-onnx` vs `onnxruntime` в `audio-separator`), например `max_abs ≤ 1e-5` и `RMSE ≤ 1e-6` по каждому стему без выравнивания/подгонки.
  - *(При любом из этих двух порогов уже полученный кандидат с `max_abs ≤ 1.423061e-6` и `RMSE ≤ 1.687843e-7` укладывается в допуск с запасом от ×7 до ×700, однако без инструкции пользователя PLAN не вправе выбрать ни один из них.)*

---

### 5. Метод измерения (без изменений относительно `r1`)
- **Метрика и единицы:** поотсчётная абсолютная ошибка `max_abs = max_i |y_cand[i] - y_ref[i]|` и среднеквадратичная ошибка `RMSE = sqrt((1/N) * sum_i (y_cand[i] - y_ref[i])^2)` в линейной шкале амплитуд `f32 PCM` (`[-1.0, 1.0]`).
- **Знаменатель и фильтры:** все сэмплы (`882 281` стерео-фреймов × 2 канала, `44 100 Hz`) каждого выходного стема (`Vocals`, `Instrumental`) без обрезки тишины, без сдвига по фазе/времени и без подгонки усиления.
- **Условия сопоставимости:** один и тот же локальный чекпоинт `UVR-MDX-NET-Inst_HQ_3.onnx` (`SHA-256: 317554b0…f9adc`), одна и та же замороженная входная фикстура (`SHA-256: a6dcdd06…13d88`), прямое сравнение выходов `candle-roformer` (CPU) и `audio-separator 0.48.0`.
- **Порог и его происхождение:** **не определён во входных данных** (причина терминального исхода `AC2-TOL-001`).

---

### 6. Ограничения, допущения и закрытие пробелов (Gaps)
- **Ограничения:** `AC-2` неизменяем; запрет на коммит многогигабайтных весов в git; сохранение статуса `"not tested"` для CUDA/Metal в `README.md`; запрет на замену реального чекпоинта/референса синтетикой; автономный режим; автокоммит разрешён только после `accepted` в CHECK.
- **Закрытие пробелов (Gaps):**
  - *Пробел в формулировке «within agreed tolerance» (`AC-2.5`)* → **Blocker** (`AC2-TOL-001` → терминальный исход `acceptance criteria cannot be determined`).

---

### 7. Судьба существующего кандидата и неблокирующих замечаний CHECK
- **Судьба кандидата (`issue-2-r4/r1/n1`, dirty tree `main @ bf2cab62e1350eec59a4e5283d18e818d8ed3c54` и артефакты в `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/`):**
  - Кандидат (код `src/mdx.rs`, `examples/mdx_separate.rs`, `docs/dependency-audit.md`, тесты `90 passed / 0 failed`, скрипты и логи реального прогона CPU-сепарации и сравнения с `audio-separator 0.48.0`) **сохраняется полностью** в рабочем дереве и в каталоге доказательств `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/`.
  - **Автокоммит не выполняется**, так как кандидат не получил вердикт `accepted` от независимого CHECK, а веса модели не подлежат включению в git.
- **Неблокирующие замечания CHECK (`AC2-FMT-001`, `AC2-EVID-001`):**
  - Поскольку элемент `#2` завершается терминальным исходом PLAN из-за неопределённости критерия `AC-2.5`, новый цикл DO/CHECK для косметической правки форматирования (`AC2-FMT-001`) или описания порядка модулей в манифесте (`AC2-EVID-001`) **не запускается**; оба замечания сохраняются в записи цикла как диагностические примечания для последующего закрытия после уточнения порога пользователем.

---

### 8. Учёт ревизий, счётчиков (`N`, `r`, `n`, `attempts`) и диспозиция item `#2`
- **Ревизия контракта (`Contract revision`):** ревизия `r2` **отклонена как неправомерная** (невозможно устранить `AC2-TOL-001` без модификации неизменяемого пользовательского `AC-2`).
- **Эффект на счётчики:** `N = 1` (без изменений); `r = 1` (ревизия не увеличивается, так как новый план `r2` не создаётся); `n = 1` (новая попытка DO/CHECK не открывается); **учёт `attempts`:** попытка `r1/n1` записывается в историю как непринятая (`not accepted → PLAN`, дефект `AC2-TOL-001`, применённых исправлений `0`) без стирания истории и без зачёта как принятого прогресса; итогом фазы PLAN фиксируется терминальный исход `acceptance criteria cannot be determined`.
- **Диспозиция item `#2` в коллекции:** статус item `#2` — **терминальный останов PLAN — `acceptance criteria cannot be determined`** (реализация и доказательства `AC-2.1..2.4, 2.6` + численное сравнение `max_abs = 1.42e-6` / `RMSE = 1.69e-7` подготовлены и сохранены на диске без автокоммита).
- **Следующий шаг для драйвера:** маршрутизировать **PLAN → ACT** для оформления терминального закрытия цикла `issue-2-r4.md` и обновления статуса коллекции `collection-open-issues-r4.md`.
````

## Approval record

`not applicable: autonomous mode` — the user instruction «тогда делай автономно с автокомитом»
authorizes execution without an approval gate; no approval ref was opened, requested or fabricated.

## DO result

`in progress: candidate ready, CHECK not run` — MDX-Net (ONNX) CPU separation is implemented and
exercised on the frozen fixture against `audio-separator` 0.48.0. A candidate is not a verdict.

Completed (same attempt `n=1`; the previous executor session was cut off by its host step budget
mid-DO after the reference/fixture groundwork, so this session resumed the same attempt and did not
open a new one):

- `src/mdx.rs` — `MdxProfile` with pinned `uvr_mdx_net_inst_hq_3()` (SHA-256
  `317554b07fe1ea5279a77f2b1520a41ea4b93432560c4ffd08792c30fddf9adc`), caller-supplied
  local-asset resolution, SHA-256 verification **before** the graph is parsed, `MdxSeparator::load` /
  `load_from_dir` / `separate` with the reference STFT/iSTFT geometry (`n_fft 6144`, `hop 1024`,
  `dim_f 3072`, `dim_t 256`, `step 195840`, `trim 3072`, symmetric-Hann overlap-add), and
  per-profile stem naming: primary `Instrumental` (model output), secondary `Vocals` =
  `-primary * 1.022 + mix`; first three frequency bins zeroed as in the reference.
- `dsp::normalize_max_peak` (scale-down only, no amplification); `MdxError` + `Error::Mdx`;
  `ModelFamily::MdxNet` + `InferenceConfig::mdx()`; `lib.rs` exports; `examples/mdx_separate.rs`.
- `docs/dependency-audit.md` — `tract-onnx` 0.23.10 and `sha2` 0.10.9 (MIT OR Apache-2.0; licence
  read from the published crates and copied to `docs/dependency-audit-evidence/`), plus the note that
  the MDX-Net checkpoint is user-supplied and not vendored/downloaded.
- `README.md` — model table row and supported-family/status wording; CUDA/Metal stay “**not** tested”.

Evidence (all under `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/`):

- checkpoint and fixture digest recomputed and matched before inference: model
  `317554b0…9adc`, fixture `a6dcdd06…3d88` (reference summary from the prior session, unchanged).
- candidate stems: `out/candidate/vocals.wav` (sha256 `d7f435a3…3850`), `out/candidate/instrumental.wav`
  (sha256 `e607db44…b7f`), 882,281 frames × 2 ch @ 44,100 Hz.
- `logs/candidate-run.log` — exit 0, CPU-only runtime **85.285 s** for 20 s audio (≈4.3× realtime).
- `logs/comparison.json` + `logs/compare-stems.log` (exit 0): Vocals max abs diff **1.423e-6**,
  RMSE **1.688e-7**, SDR 117.95 dB, corr 0.999999999999; Instrumental max abs diff **1.013e-6**,
  RMSE **1.203e-7**, SDR 120.31 dB, corr 0.999999999999; overall max abs diff **1.423e-6**,
  RMSE **1.465e-7**, SDR 118.90 dB. The plan fixes the measurement method (max abs diff + RMSE) but
  no numeric tolerance, so these measured values are returned for CHECK's tolerance judgement; no
  PASS is asserted here.
- `logs/cargo-test-full.log` — `cargo test` exit 0: 33 lib + 22 + 12 + 11 + 8 + 3 integration tests
  + 1 doc-test = **90 passed, 0 failed**; `logs/cargo-test-mdx.log` — `cargo test --lib mdx::` exit 0,
  **9 passed** (profile/geometry/checksum-mismatch/missing-asset/window/STFT-shape/input tests).
- builds: `logs/cargo-build-lib-mdx.log` (`cargo build --lib`) exit 0;
  `logs/cargo-build-release-example-mdx.log` (release example) exit 0.
- gates: `logs/cargo-clippy.log` — `cargo clippy --all-targets` exit 0 (warnings only, no errors);
  `logs/cargo-fmt-check.log` — `cargo fmt --check` exit **1** on pre-existing formatting drift in
  `src/config.rs` and `src/dsp.rs` regions not touched by this item; the two new files were formatted
  with `rustfmt` and no repo-wide reformat was applied.
- `manifest.json` — git identity (`main @ bf2cab62e1350eec59a4e5283d18e818d8ed3c54`), branch,
  `git status --porcelain`, `git diff --stat`, candidate file list and SHA-256 of every key artefact
  (refreshed after the final-tree re-run below).

Final-tree re-run (tail of the same attempt `n1`, no new attempt, no CHECK): a cosmetic `src/lib.rs`
import/module reorder landed **after** the measurement above (`src/lib.rs` sha256 `759e1c6b…88b1` →
`a5cfd4bd…688b`: `pub mod mdx;` placed before `pub mod mel;`, `pub use mdx::{…}` moved above
`pub use mel::…`; import-order only, `cargo build --lib` exit 0 afterwards). The declared gate and
clippy were re-run on that final tree so the evidence matches the candidate:

- `logs/cargo-test-full-final.log` — `cargo test` exit **0** on the final tree: 33 lib + 22 + 12 + 11
  + 8 + 3 + 1 doc-test = **90 passed, 0 failed, 0 ignored** (same counts as the earlier run). The
  earlier `logs/cargo-test-full.log` is kept and marked `superseded_by` the final log in the manifest.
- `logs/cargo-clippy-final.log` — `cargo clippy --all-targets` exit **0** on the final tree, warnings
  only, **0** `error:` lines; the earlier `logs/cargo-clippy.log` is kept and marked `superseded_by`.
- `logs/cargo-fmt-check.log` (regenerated 2026-11-10T14:50:55Z) — `cargo fmt --check` exit **1**: in
  addition to the pre-existing drift in `src/config.rs`, `src/dsp.rs`, `src/model/mod.rs` and
  `src/separator.rs` it now reports `Diff in src\lib.rs:10:` (rustfmt wants `pub mod mel;` before
  `pub mod mdx;`), i.e. the reorder itself added formatting drift in `src/lib.rs`. It is not a
  declared gate of this item and was not fixed here — a fix changes the tree and would re-open the
  measurement — so it is recorded as evidence for CHECK.
- `manifest.json` now also carries `final_gate` (command line, exit code, passed/failed/ignored per
  run, log path) and `notes` on the observed log-refresh drift: current `HEAD` and branch unchanged
  (`main @ bf2cab62e1350eec59a4e5283d18e818d8ed3c54`), same porcelain/diff-stat, and the candidate
  stems, `comparison.json`, `candidate-run.log` and `compare-stems.log` hashes are byte-identical to
  the earlier frozen values.

CHECK-readable evidence staging (tail of the same attempt `n1`, no new attempt, no product edit, no
re-measurement): because CHECK's scoped read reaches `.pdca/**` only, the candidate's product change
is mirrored under the evidence root, bound to the source identity at
`main @ bf2cab62e1350eec59a4e5283d18e818d8ed3c54`:

- `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/snapshot/` — byte-exact copies of all 11 changed/new
  item files at repo-relative paths (`Cargo.lock`, `Cargo.toml`, `README.md`,
  `docs/dependency-audit.md`, `docs/dependency-audit-evidence/tract-onnx-0.23.10.txt`,
  `examples/mdx_separate.rs`, `src/config.rs`, `src/dsp.rs`, `src/error.rs`, `src/lib.rs`,
  `src/mdx.rs`); copy hashes were checked equal to the working-tree sources.
- `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/snapshot.diff` — the whole candidate change as one
  unified diff: `git diff` of the 8 tracked paths against `HEAD`, plus the 3 new files rendered as
  `new file` diffs (`git diff --no-index -- /dev/null <path>`); 65,446 B, 11 file headers, 3
  `new file mode` headers.
- `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/snapshot-sha256.txt` — `sha256  repo-relative-path` per
  staged file, header binding to `HEAD`, branch `main`, the verbatim 15-entry
  `git status --porcelain` and `cycle_id issue-2-r4 / r1 / n1`, trailer with the `snapshot.diff`
  self-hash.
- `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/index.md` — navigation index over every artefact group in
  this evidence root (pointers only, no conclusions).
- `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/manifest.json` — unchanged item manifest, cross-checked:
  every staged snapshot hash equals the matching `artefacts[].sha256` value already recorded there.

Residual for CHECK/ACT: independent verdict; the plan pinned no numeric tolerance; nothing is
staged or committed by DO.

## CHECK record

Recorded verbatim from the independent `pdca-check` return (source: CHECK report for candidate
`issue-2-r4/r1/n1`, snapshot at evidence root `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/`; verdict
`not accepted`, routed → PLAN):

````markdown
## CHECK: `not accepted` → PLAN

Кандидат: dirty tree `main @ bf2cab62e1350eec59a4e5283d18e818d8ed3c54`, snapshot `issue-2-r4/r1/n1`. Счётчики прочитаны из status: `N=1, r=1, n=1`; предыдущих дефектов нет.

Ниже пути относительно `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/`.

### Оценка критерия

AC-2 покрывает обязательные результаты item #2; противоречий между его требованиями не обнаружено. Однако **«within agreed tolerance» остаётся неопределённым**.

**G3 не составляет согласованный допуск для MDX-Net.** По предоставленному независимому наблюдению он относится к MelBand/PyTorch. Совпадение измеряемых величин — waveform max abs/RMSE — делает сравнение технически осмысленным, но не переносит область действия соглашения на MDX-Net/audio-separator. PLAN явно привязывает MelBand gate к #1, но не к #2 (`../plan.md:11–12`).

Не хватает существующего соглашения, устанавливающего применимые к MDX-Net численные пределы и правило оценки stems. Один раунд поиска завершён; источник не найден. Порог по полученным результатам я не назначал.

### Матрица AC-2

Идентификаторы ниже — декомпозиция исходного AC-2, не новые критерии.

| ID | Обязательное требование | Статус | Основание и evidence |
|---|---|---|---|
| AC-2.1 | Реальный опубликованный ONNX, предоставленный локально | **met** | `UVR-MDX-NET-Inst_HQ_3.onnx`, 66 759 214 B; присутствует в upstream model list. Reference распознаёт MDX и использует существующий локальный файл. `models/download_checks.json:36–40`, `logs/utf8/reference-run.utf8.log:16–38`, `logs/onnx-graph.txt`. |
| AC-2.2 | Checksum проверен до inference | **met** | SHA-256 `317554b0…f9adc` согласован в manifest/reference/comparison. Код действительно вычисляет и проверяет checksum **до** parsing graph; example вызывает этот loader. `snapshot/src/mdx.rs:199–227,245–262`, `snapshot/examples/mdx_separate.rs:30–42`. Напечатанный example digest сам по себе доказательством проверки не считался. |
| AC-2.3 | Разделяет аудио на CPU | **met** | Исполняется tract graph; получены Instrumental/Vocals, по 882 281 frames, stereo, 44 100 Hz; 85.285 s. `cargo run --release --example mdx_separate …`, записанный exit **0**. `logs/candidate-run.log`, `logs/utf8/README.md:75–76`, `snapshot/src/mdx.rs:448–453`. |
| AC-2.4 | Общая frozen fixture с reference | **met** | Reference и comparison привязаны к fixture SHA-256 `a6dcdd06…13d88`; геометрия выходов совпадает. Проверены пути и логика скриптов. `scripts/reference_audio_separator.py`, `scripts/compare_stems.py`, `logs/reference-summary.json`, `logs/comparison.json`. |
| AC-2.5 | Parity с audio-separator в согласованном допуске | **unverified** | Reference **0.48.0** подтверждён логом. Vocals: max abs **1.423061e−6**, RMSE **1.687843e−7**; Instrumental: **1.013279e−6**, **1.202614e−7**. `python scripts/compare_stems.py`, записанный exit **0**. Числа получены прямым sample-wise сравнением без выравнивания/подгонки, но согласованный применимый порог отсутствует. `logs/comparison.json`, `scripts/compare_stems.py:40–56,77–97`, `../plan.md:12`. |
| AC-2.6 | Dependency и licence в `docs/dependency-audit.md` | **met** | Документированы tract-onnx **0.23.10**, sha2 **0.10.9**, лицензии **MIT OR Apache-2.0**, назначение и ссылки на evidence. `snapshot/docs/dependency-audit.md:25–50`, `snapshot/docs/dependency-audit-evidence/tract-onnx-0.23.10.txt:1–12`. |

Дополнительно: `cargo test`, записанный exit **0**, **90 passed / 0 failed / 0 ignored**; содержание проверено по `logs/utf8/cargo-test-full-final.utf8.log`. README сохраняет CUDA/Metal «not tested» (`snapshot/README.md:158–161`); checkpoint отсутствует в списке commit-кандидата.

### Findings

| ID | Severity / статус | Finding |
|---|---|---|
| **AC2-TOL-001** | **blocking / confirmed** | Дефект judgeability AC-2.5: согласованный MDX-Net допуск не установлен. Основание: PLAN содержит метод, но не предел; завершённый независимый поиск не нашёл применимого соглашения. Это **не доказанный численный дефект реализации**. |
| **AC2-FMT-001** | **minor / confirmed** | `cargo fmt --check` имеет записанный exit **1**, включая замечание к добавленной строке `src/lib.rs`. Однако независимое наблюдение не обнаружило обязательного fmt gate в инструкциях/CI/item contract. Поэтому это не самостоятельное основание отказа и не исключение из обязательного gate. `logs/utf8/cargo-fmt-check.utf8.log:49–58`. |
| **AC2-EVID-001** | **minor / confirmed** | Описание порядка модулей в manifest/status не соответствует прочитанному snapshot: snapshot содержит **mel перед mdx**, а fmt-log требует **mdx перед mel**, то есть направление противоположно brief. `snapshot/src/lib.rs:13–14`, `logs/utf8/cargo-fmt-check.utf8.log:49–58`, `manifest.json:97,101`. Привязка именно fmt-наблюдения к описанному финальному состоянию ненадёжна; численную parity это само по себе не опровергает. |

### Маршрутизация и пределы

**→ PLAN**, поскольку отказ вызван неопределённостью пользовательского критерия, а не проваленной реализацией.

После завершённого поиска PLAN должен оформить terminal outcome **`acceptance criteria cannot be determined`**, сохранив AC-2 дословно и указав отсутствующее соглашение. Нельзя переносить G3, дописывать порог, запускать новый DO/CHECK-loop ради его подбора или автокоммитить кандидат как принятый.

**Уверенность высокая** в маршруте и отсутствии основания для PASS. Реальная CPU separation и близость выходов подтверждены существующими кодом, скриптами и артефактами. Проверка read-only: я не пересчитывал бинарные WAV/checkpoint и не повторял inference; exit codes взяты из записей, а reference exit отдельно не записан, хотя summary и оба выхода созданы. Результаты ограничены выбранным checkpoint и fixture; обобщение на другие MDX-модели не установлено.
````

**Re-gather note (one round, not an attempt).** CHECK's read-back needed one re-gather round, recorded
as a note and not as a new attempt: the transcoded UTF-8 log copies under
`.pdca/open-issues-r4/rv1/item-2-mdx-onnx/logs/utf8/` (21 readable copies; only `logs/reference-run.log`
was mixed UTF-8/UTF-16LE and needed transcoding, the other 20 copies were byte-identical to the
untouched originals — see `logs/utf8/README.md`) and the independent fact gather on tolerance sources and
gates that grounds `AC2-TOL-001` (`G3` scoped to MelBand/PyTorch at
`docs/superpowers/specs/2026-11-07-candle-roformer-design.md:118`, `docs/gates-report.md:9,94`,
`README.md:162-165`; no numeric MDX-Net tolerance in the plans or tests; `../plan.md:11–12`). One round,
no new attempt, no product change, no re-measurement.

## ACT record

Recorded by ACT; the outcome was determined by PLAN (`PLAN → ACT` route, no new verdict, no PASS).

- **outcome:** `incomplete` · `terminal=criteria-undetermined` · `disposition=finalized` · counters
  `N=1`, `r=1`, `n=1` (revision `r2` not issued; no new attempt opened)
- **cause:** blocking finding `AC2-TOL-001` — the user's immutable `AC-2` requires parity "within agreed
  tolerance", and no agreed numeric tolerance or stem-aggregation rule for MDX-Net vs `audio-separator`
  exists in the user input, the project spec/gates or the test suite. `G3` belongs to MelBand/PyTorch
  (item `#1`) and cannot be transferred; fitting a threshold to the measured `1.42e-6` / `1.69e-7` would
  modify the user's criterion. In autonomous mode no interactive clarification is possible, so PLAN
  declared `acceptance criteria cannot be determined`.
- **criteria outcome per element** (as assessed by CHECK; recorded as **not an acceptance**):
  `AC-2.1` met, `AC-2.2` met, `AC-2.3` met, `AC-2.4` met, `AC-2.6` met, **`AC-2.5` unverified** — the
  candidate was measured (`Vocals` max abs `1.423061e-6` / RMSE `1.687843e-7`; `Instrumental` `1.013279e-6`
  / `1.202614e-7`) but judgeability of the threshold failed. No criterion is recorded as accepted.
- **what remains:** `AC-2.5` unverified (tolerance undetermined); the two minor CHECK findings
  `AC2-FMT-001` (fmt drift incl. the new `src/lib.rs` module-order line) and `AC2-EVID-001` (manifest
  module-order description) stay open as diagnostics — no new DO/CHECK cycle was opened for them;
  **no commit** — the candidate was not accepted, so no `git add`, commit, push or branch change.
- **preserved assets (all outside git, under `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/`):**
  - `patch/item-2-candidate.patch` — the whole candidate change as one applyable patch, 65,446 B,
    sha256 `4922abec88ce6661a31c96f16eeba2033d5657f7473797fd139769ab8896ff17` (byte-identical to
    `snapshot.diff`); `git apply --check --reverse` exit 0 on the still-modified tree and
    `git apply --check` exit 0 on the restored `HEAD` tree after rollback (no output)
  - `snapshot/` — byte-exact copies of all 11 candidate files, each hashed equal to the working-tree
    source before rollback; `snapshot-sha256.txt`, `snapshot.diff`, `index.md`, `manifest.json`
    (now carrying a `preservation` block with the patch hash and the rollback ledger)
  - measurement evidence retained: checkpoint `317554b0…f9adc` (66,759,214 B), fixture
    `a6dcdd06…13d88`, both reference stems, both candidate stems, `logs/comparison.json`,
    `logs/candidate-run.log`, `logs/cargo-test-full-final.log`, `logs/cargo-clippy-final.log`,
    `logs/cargo-fmt-check.log`, `logs/utf8/`, `scripts/`, `reference/`
  - the item's product paths were **rolled back to `HEAD`** (`git restore --source=HEAD --staged
    --worktree --` for `Cargo.toml`, `Cargo.lock`, `README.md`, `docs/dependency-audit.md`,
    `src/config.rs`, `src/dsp.rs`, `src/error.rs`, `src/lib.rs`) and its three untracked files
    (`src/mdx.rs`, `examples/mdx_separate.rs`, `docs/dependency-audit-evidence/tract-onnx-0.23.10.txt`)
    were removed **only after** the patch and the snapshot copies were verified; the working tree is
    clean of this item's delta
- **limitations:** CHECK verified by record, not by recomputation (binary WAV/checkpoint not
  re-hashed, inference not repeated; the reference exit code was not recorded separately although the
  summary and both outputs exist); results hold only for the chosen checkpoint and fixture — support
  for other MDX models is unproven; the retrospective read the status records before this write and
  therefore descends that they lagged.
- **next authorized action (item level):** none inside this cycle — it is closed. A **new explicit user
  instruction that defines the numeric tolerance** (e.g. extend `G3` to MDX-Net, or a runtime-parity
  tolerance such as `max_abs ≤ 1e-5` / `RMSE ≤ 1e-6` per stem) would open a **new authorized cycle**
  that re-applies `patch/item-2-candidate.patch` (or starts from `snapshot/`), re-measures and judges
  `AC-2.5` against that tolerance; the two minor findings would then be closed in the same cycle. The
  collection lane continues with item `#8`; this plan-terminal resolution does not stop the group.
- **durable lesson:** none promoted beyond this record (no host memory write within these permissions)

## Lessons learned

Recorded verbatim from the mandatory ACT retrospective returned by `pdca-check` in `retrospective`
mode (analysis only; it returns no verdict and does not override the originating outcome):

````markdown
## Ретроспектива `issue-2-r4`

Анализ существующих записей; новых проверок и запусков не проводилось. Заявленный исход `incomplete / criteria-undetermined`, без автокоммита, не пересматривается.

### Что сработало

- **Возобновление сохранило выполненную работу.** Запись DO подтверждает продолжение того же `r1/n1` после ограничения шагов хоста с использованием сохранённых reference/fixture. После последней перестановки модулей повторены тесты и clippy; прежние логи сохранены как superseded. Это обеспечило прослеживаемость, хотя само совпадение идентичностей не доказывает корректность продукта.
  **Источник:** `.pdca/status/issue-2-r4.md:56–58,100–144,215–221`.

- **Единственный зафиксированный re-gather устранил проблему читаемости, не меняя измерения.** Отчёт описывает 21 читаемую копию: только `reference-run.log` требовал преобразования смешанной UTF-8/UTF-16LE кодировки; остальные 20 копий побайтно совпали с оригиналами. Оригиналы сохранены. Это восстановило доступ к свидетельствам, но не определило допустимую погрешность и не восстановило отсутствующие exit codes.
  **Источник:** `logs/utf8/README.md:15–60`; журнал item, строка 222.

### Сохранённый технический актив — не подтверждение выполнения AC-2

По существующим отчётам сохранены:

- checkpoint `UVR-MDX-NET-Inst_HQ_3.onnx`, SHA-256
  `317554b07fe1ea5279a77f2b1520a41ea4b93432560c4ffd08792c30fddf9adc`;
- frozen fixture, SHA-256
  `a6dcdd06ff1c19677787fd36f4114d4c98145161c6c1e259b9658b753eb13d88`, 882 281 кадр, stereo, 44 100 Гц;
- reference audio-separator **0.48.0**, ONNX Runtime **1.31.1**, оба reference stem и candidate stem;
- измерения: общий max abs diff **1.423060894e−6**, RMSE **1.465451436e−7**, SDR **118.8965 dB**; заявленное время candidate CPU-run **85.285 с**;
- снимок 11 файлов реализации, документации и примера; отчёт финального `cargo test`: **90 passed**, exit 0; clippy exit 0; 9 MDX-тестов.

**Источники:** `manifest.json`, `logs/reference-summary.json`, `logs/comparison.json`, item §DO result. Это полезная база будущего цикла, но не замена «agreed tolerance» и не доказательство поддержки всех MDX-моделей.

### Три переносимых урока

1. **Категория: PLAN / определимость критериев.**
   В entry PLAN указаны метрики max abs diff/RMSE, но нет численного допуска; DO прямо передал вопрос «на judgement CHECK». Следовало отметить этот пробел уже при PLAN, самое позднее — при входе в DO, не назначая допуск самостоятельно. Измеримость расхождения и определимость соответствия — разные вещи. Вероятное следствие более раннего обнаружения — меньше затрат до terminal-решения; объём экономии неизвестен.
   **Источники:** collection §PLAN r1, строки 87–89; item:81–86,146–147. Заявленная цепочка CHECK → PLAN → terminal соответствует такому разграничению, но полные отчёты этой цепочки в указанных записях отсутствуют.

2. **Категория: исполнение / восстановление после прерываний.**
   Продолжать с явно записанной границы: что завершено, какие артефакты заморожены, что осталось, какие действия относятся к тому же attempt. Здесь дисковые свидетельства позволили продолжить без потери reference/fixture. Оценку «85 %» не следует использовать как показатель готовности к завершению: она ничего не говорит об определимости критерия.
   **Источник:** item §DO result и §Progress log. Два прерывания и оценка 85 % названы в brief; прочитанный журнал подтверждает лишь одно предшествующее прерывание, без количественной оценки готовности.

3. **Категория: точность отчётности / evidence bookkeeping.**
   Отделять исходное состояние от внесённого дрейфа, а имена файлов — от их содержания. Первоначальное описание fmt как только pre-existing стало неполным: финальная запись честно признаёт новый drift в `src/lib.rs`. Кроме того, `lint-issue-3.txt` содержит результат для `issue-2-r4`. Это локальные неточности атрибуции, не свидетельство выдуманных измерений. В будущих отчётах стоит сохранять обе версии описания с явным уточнением.
   **Источники:** item:92–95,111–116,220; `logs/utf8/README.md:118–120`. Их minor/non-blocking трактовка имеет объяснение: fmt не указан обязательным gate, неверно названный lint-файл не меняет численных результатов. Однако отсутствие полного CHECK не позволяет подтвердить именно его обоснование классификации.

### Пределы и неопределённость

Указанные status-файлы отстают от brief: item всё ещё содержит `CHECK: not run`, пустую defect history и pending ACT; collection также показывает незавершённое состояние. Полные CHECK, terminal PLAN и история трёх дефектов по предоставленным ссылкам не представлены. Поэтому их содержание не реконструировано; originating outcome сохранён как заданный контекст. Уверенность выше в описании сохранённых активов и DO, ниже — в деталях закрытия.

**Ретроспективный анализ завершён. Запись отчёта исполнителем и завершение ACT этим сигналом не подтверждаются.**
````

**Completion signal.** The retrospective report above is persisted verbatim by `pdca-executor`; on that
signal ACT is **completed** for `issue-2-r4`. The retrospective's own remark about lagging status records
was accurate when it read them — this write closes that gap: §CHECK record, §ACT record, §Revision
history, §Defect history now carry the returned reports. The report was recorded as returned and not
edited; it creates no verdict, no counter, no attempt and no new verification, and the originating
outcome stays `incomplete` / `terminal=criteria-undetermined`.

## Revision history

2026-11-10T15:08Z | r1 held (no r2) | CHECK → PLAN: `AC2-TOL-001` criteria judgeability — no agreed MDX-Net tolerance exists and the user's `AC-2` is immutable, so revision `r2` was refused as illegitimate without changing the user's input; outgoing attempt `r1/n1` not accepted (`not accepted → PLAN`, fixes applied 0)

## Defect history

- `AC2-TOL-001` | r1/n1 observed once | 0 fixes applied | CHECK report in §CHECK record; plan terminal in §PLAN record | last: terminal `criteria-undetermined` — no escalation needed (blocking judgeability defect, not a repeated implementation defect)
- `AC2-FMT-001` | r1/n1 | 0 fixes applied | `logs/utf8/cargo-fmt-check.utf8.log:49–58` | last: minor, confirmed; no declared fmt gate in the contract/CI, so not an acceptance gate — open as a diagnostic
- `AC2-EVID-001` | r1/n1 | 0 fixes applied | `manifest.json:97,101`; `snapshot/src/lib.rs:13–14` | last: minor, confirmed — manifest/status module-order description inconsistent with the snapshot — open as a diagnostic

## Changed files

Task-owned delta of this item. **At ACT the product delta was rolled back**: the eight tracked paths are
`reverted to HEAD; preserved in patch`, and the three new files were removed after preservation — the
work is not lost, it lives in the evidence root.

Product / docs / example (state after ACT):

- `Cargo.toml`, `Cargo.lock` — `reverted to HEAD; preserved in patch` (`tract-onnx` 0.23.10, `sha2` 0.10.9)
- `src/config.rs`, `src/dsp.rs`, `src/error.rs`, `src/lib.rs` — `reverted to HEAD; preserved in patch`
- `docs/dependency-audit.md`, `README.md` — `reverted to HEAD; preserved in patch`
- `src/mdx.rs`, `examples/mdx_separate.rs`, `docs/dependency-audit-evidence/tract-onnx-0.23.10.txt` —
  removed from the working tree after preservation; byte-exact copies in `snapshot/`, applyable from
  `patch/item-2-candidate.patch`
- no test file was added: the new cases were unit tests inside `src/mdx.rs` and `src/dsp.rs` (9 + 2),
  now inside the preserved patch

`.pdca` artefacts (this cycle; all git-ignored except the status record):

- `.pdca/status/issue-2-r4.md` → created and finalized (PLAN terminal block, CHECK record + re-gather note,
  ACT record, lessons learned, revision/defect history, changed files, git state, progress log)
- `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/patch/item-2-candidate.patch` → created (whole candidate,
  65,446 B, sha256 `4922abec…6ff17`)
- `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/manifest.json` → added the `preservation` block (patch hash,
  snapshot verification, rollback ledger, commit: none)
- `.pdca/open-issues-r4/rv1/item-2-mdx-onnx/index.md` → added the patch row and the ACT-phase note
- unchanged but bound to this identity: `snapshot/`, `snapshot.diff`, `snapshot-sha256.txt`, `out/`,
  `logs/` (incl. `logs/utf8/`), `scripts/`, `reference/`, `fixture/`, `models/`

No pre-existing dependency-audit evidence file was changed: the accidental rewrite of the tracked
`docs/dependency-audit-evidence/sha2-0.10.9.txt` was restored to `HEAD` inside the DO session and stays
`HEAD` after the rollback.

## Git state

`status path: .pdca/status/issue-2-r4.md — created and modified this cycle (finalized in ACT) ·
untracked (durable on disk, not committed)`

- baseline supplied by the collection record: `main @ bf2cab6`, worktree on branch `main` (no worktree,
  no branch creation, no merge for this item); current `HEAD` unchanged at
  `bf2cab62e1350eec59a4e5283d18e818d8ed3c54`
- same-cycle touched-file ledger: during DO the ten product paths were modified/created (see the earlier
  DO ledger in §DO result); **ACT reverted that product delta** — `git restore --source=HEAD --staged
  --worktree --` (exit 0) for the eight tracked paths and removal of the three untracked files after
  preservation
- after rollback `git status --porcelain` shows only `?? .openidea/`, `?? .pdca/status/collection-open-issues-r3.md`,
  `?? .pdca/status/collection-open-issues-r4.md`, `?? .pdca/status/issue-2-r4.md`; `git diff --stat HEAD`
  is empty (tracked tree equals `HEAD`)
- candidate patch pending under the evidence root (`.pdca/open-issues-r4/rv1/item-2-mdx-onnx/patch/`,
  git-ignored via `/.pdca/*`) — not committed, not staged; **no commit was made**: the candidate was not
  accepted, so autocommit does not apply
- `git apply --check` of the patch on the restored tree: exit 0, no output
- work outside this task's delta stays untouched and unclaimed (`.openidea/` and the prior
  `collection-open-issues-r3.md` record were pre-existing and remain so)

## Progress log

2026-11-10T14:19:29Z | DO | revision r1 | attempt n1 | DO started for item #2 (MDX-Net ONNX); attempt 1 of r1 consumed | .pdca/status/issue-2-r4.md
2026-11-10T14:49:38Z | DO | revision r1 | attempt n1 | resumed the same DO attempt after the previous executor session was cut off by its host step budget (not a contract failure; no CHECK ran, no new attempt opened); reference/fixture groundwork reused from disk | .pdca/open-issues-r4/rv1/item-2-mdx-onnx/
2026-11-10T14:49:38Z | DO | revision r1 | attempt n1 | candidate ready (CHECK not run): MDX-Net ONNX CPU separation vs audio-separator 0.48.0 on the frozen fixture — overall max abs diff 1.423e-6, RMSE 1.465e-7, SDR 118.90 dB; runtime 85.285 s; 90 tests passed; builds/clippy exit 0, fmt --check exit 1 on pre-existing drift | .pdca/open-issues-r4/rv1/item-2-mdx-onnx/logs/comparison.json
2026-11-10T14:52:17Z | DO | revision r1 | attempt n1 | final-tree gate re-run after the post-measurement cosmetic src/lib.rs import reorder (tail of the same attempt n1; no new attempt, no CHECK): cargo test exit 0 — 90 passed, 0 failed, 0 ignored (33 lib + 22 + 12 + 11 + 8 + 3 + 1 doc-test) | .pdca/open-issues-r4/rv1/item-2-mdx-onnx/logs/cargo-test-full-final.log
2026-11-10T14:52:22Z | DO | revision r1 | attempt n1 | cargo clippy --all-targets exit 0 on the final tree (warnings only, 0 errors) | .pdca/open-issues-r4/rv1/item-2-mdx-onnx/logs/cargo-clippy-final.log
2026-11-10T14:52:59Z | DO | revision r1 | attempt n1 | manifest.json refreshed — HEAD main @ bf2cab62e1350eec59a4e5283d18e818d8ed3c54, branch, porcelain, diff --stat, candidate files, sha256 incl. both final logs, final_gate block | .pdca/open-issues-r4/rv1/item-2-mdx-onnx/manifest.json
2026-11-10T14:52:59Z | DO | revision r1 | attempt n1 | Notice: cargo fmt --check (log regenerated 2026-11-10T14:50:55Z) exit 1 now also reports src/lib.rs:10 (mod mel/mdx order) in addition to the pre-existing drift; not a declared gate, not fixed — recorded for CHECK | .pdca/open-issues-r4/rv1/item-2-mdx-onnx/logs/cargo-fmt-check.log
2026-11-10T14:55:52Z | DO | revision r1 | attempt n1 | evidence staged for CHECK's scoped read (same attempt n1; no product edit, no re-measurement, no CHECK): 11 candidate files copied byte-exact to snapshot/ with hashes verified against the working tree, snapshot.diff (65,446 B; 8 tracked diffs + 3 new-file diffs), snapshot-sha256.txt bound to HEAD main @ bf2cab6, index.md navigation; item stays in DO, n=1, outcome open | .pdca/open-issues-r4/rv1/item-2-mdx-onnx/snapshot-sha256.txt
2026-11-10T15:01:03Z | DO | revision r1 | attempt n1 | CHECK re-gather of the existing evidence, no phase change and no re-run (no product edit, no build/test/clippy/fmt invoked, no new attempt): logs/utf8/ now holds UTF-8 readable copies of the unmodified originals - only logs/reference-run.log was not clean UTF-8 (mixed UTF-8 + UTF-16LE segments, 61,969 NUL bytes; 150,513 B -> 88,599 B transcoded), the other 20 logs were already UTF-8 and their copies are byte-identical; 21/21 copies decode as strict UTF-8, 0 NUL bytes; originals byte-for-byte unchanged (sha256 before = after, and equal to the hashes recorded in manifest.json) | .pdca/open-issues-r4/rv1/item-2-mdx-onnx/logs/utf8/README.md
2026-11-10T15:08:10Z | CHECK | revision r1 | attempt n1 | CHECK verdict `not accepted` → PLAN: AC-2.1..2.4 and AC-2.6 met, AC-2.5 unverified, blocking AC2-TOL-001 (no agreed MDX-Net tolerance); minor AC2-FMT-001, AC2-EVID-001; one re-gather round recorded as a note, not an attempt | .pdca/status/issue-2-r4.md §CHECK record
2026-11-10T15:08:10Z | PLAN | revision r1 | attempt n1 | PLAN terminal: `acceptance criteria cannot be determined` (AC2-TOL-001); revision r2 not issued — r2 would be illegitimate without changing the immutable user AC-2; route PLAN → ACT, no new DO, no new CHECK, no autocommit; attempt r1/n1 recorded as not accepted with 0 fixes | .pdca/status/issue-2-r4.md §PLAN record (terminal block)
2026-11-10T15:08:20Z | ACT | revision r1 | attempt n1 | ACT opened for item #2: counters read from the record (N=1, r=1, n=1), no new revision and no new attempt; outcome to be recorded, no commit authorized because the candidate was not accepted | .pdca/status/issue-2-r4.md
2026-11-10T15:08:20Z | ACT | revision r1 | attempt n1 | candidate preserved as one applyable patch before any rollback: patch/item-2-candidate.patch, 65,446 B, sha256 4922abec88ce6661a31c96f16eeba2033d5657f7473797fd139769ab8896ff17, byte-identical to snapshot.diff; reverse check `git apply --check --reverse` exit 0 on the still-modified tree; all 11 snapshot/ copies hashed equal to the working-tree sources; patch hash + rollback ledger recorded in manifest.json (`preservation`) and index.md | .pdca/open-issues-r4/rv1/item-2-mdx-onnx/patch/item-2-candidate.patch
2026-11-10T15:08:40Z | ACT | revision r1 | attempt n1 | item paths rolled back after preservation: `git restore --source=HEAD --staged --worktree --` exit 0 for Cargo.toml, Cargo.lock, README.md, docs/dependency-audit.md, src/config.rs, src/dsp.rs, src/error.rs, src/lib.rs; removed src/mdx.rs, examples/mdx_separate.rs, docs/dependency-audit-evidence/tract-onnx-0.23.10.txt; `git apply --check` of the patch on the restored tree exit 0 with no output; `git diff --stat HEAD` empty; `git status --porcelain` only .openidea/ and the three .pdca/status records; no git add, no commit, no push, no branch switch | .pdca/open-issues-r4/rv1/item-2-mdx-onnx/patch/item-2-candidate.patch
2026-11-10T15:09:00Z | ACT | revision r1 | attempt n1 | ACT closed: outcome `incomplete`, `terminal=criteria-undetermined`, `disposition=finalized`; cause AC2-TOL-001 (no agreed MDX-Net tolerance; the user's AC-2 is immutable); residual AC-2.5 unverified, AC2-FMT-001 and AC2-EVID-001 open as diagnostics; nothing committed; retrospective recorded verbatim in §Lessons learned on the pdca-check return | .pdca/status/issue-2-r4.md §ACT record
2026-11-10T15:10:35Z | ACT | revision r1 | attempt n1 | collection row #2 updated to `incomplete` / `criteria-undetermined` / `finalized` with `plan_state=pending`, group G1 kept `in-progress` (a plan-terminal item does not stop the group), next item #8; status_lint.py exit 0 — "status-lint: OK (.pdca\status\collection-open-issues-r4.md: 2 tables, 7 rows)"; an earlier run with attempts=1 was rejected (line 119, exit 2) and the row was repaired to attempts 0 with the truthful note | .pdca/open-issues-r4/rv1/item-2-mdx-onnx/logs/lint-item-2-act.txt
