# Cycle status: open-issues-r5-int-followup-r6 / cycle 6

## Current state

- task: `open-issues-r5-int-followup-r6` · form: `cycle` · specialized skill: `none`
- mode: `autonomous`
- cycle: `N=6` · revision: `r=1` · attempt: `n=1`
- counters: `commands_run = 5` (финально) — четыре авторизованные read-only операции захвата, выполненные в пяти shell-вызовах (один повтор операции 2 из-за квотирования `^{tree}`; повтор дал `exit 0` и тот же результат, первый вызов — `exit 1`, пустой вывод, изменений состояния нет; см. §DO result, п. 1), плюс один разрешённый прогон `status_lint.py` по этой записи (`exit 0`, финальный — в `ACT`). Тестов, сборок, `cargo`, листингов, git-записей, `push`, обращений к GitHub — `0`
- scope: документарная финализация коллекции после `N=5`: обязательный pre-cycle `Notice`, один авторизованный read-only provenance capture, независимый `CHECK` десяти критериев, при `10/10 met` — append-only замещающий финальный блок в записи коллекции и `status_lint.py` в `ACT`; запись коллекции до `all met` не изменяется · status path: `.pdca/status/cycle-open-issues-r5-int-followup-r6.md`
- goal: получить и зафиксировать свежий read-only захват идентичностей кандидата и замороженных статусных записей, отдать его независимому `CHECK` десяти критериев и при `10/10 met` append-only закрыть коллекцию — §PLAN record и §Original task авторитетны
- phase: `ACT` · record: `completed` · outcome: `incomplete (STOP — F-R6-FROZEN-BASELINE)`
- source: бриф цикла `N=6`, выданный после терминального `ACT` цикла `cycle-open-issues-r5-int-followup-r5` (`incomplete (STOP — F-R5-CONTINUITY-EVIDENCE / F-R5-EXECUTION-EVIDENCE)`) · context refs: `.pdca/status/collection-open-issues-r5.md`, `.pdca/status/cycle-open-issues-r5-int-followup-r{3,4,5}.md`, `.pdca/cycle-open-issues-r5-int-followup-r3/`, `.pdca/cycle-open-issues-r5-int-followup-r4/`, `.pdca/cycle-open-issues-r5-int-followup-r6/`, `.pdca/open-issues-r5/rv1/`, `.pdca/status/issue-{2,8,3,4}-r5.md`
- criteria: `INT-BUILD-TESTS`, `INT-PARITY-AND-GATES`, `INT-HYGIENE`, `INT-TRACEABILITY`, `INT-LIMITS`, `FU-CANDIDATE-PIN`, `FU-ISOLATION`, `FU-R3-IMMUTABILITY`, `FU-HISTORY-PRESERVATION`, `FU-R6-EXECUTION-SCOPE` — дословно в §PLAN record `r1` (source: возврат планера, переданный драйвером), здесь не перенабираются

## Original task

Исходная задача — коллекционная задача `open-issues-r5` по точной ссылке (здесь не перенабирается): `.pdca/status/collection-open-issues-r5.md` §Original task, где дословно записаны `pdca по открытым задачам из github`, `тогда делай автономно с автокомитом`, `после каждой выполненной задачи - делай комит и закрывай задачу`; критерии — §PLAN record `r1` того же файла (четыре verbatim `Done when` и утверждённое численное дополнение), а также item-записи `.pdca/status/issue-{2,8,3,4}-r5.md` (все четыре — `accepted`).

Авторизация пользователя для цикла `N=6` — как передана в брифе (сырой текст сообщения пользователя в этой сессии не воспроизводится; ниже — дословная формулировка брифа, не пересказ):

- `N=6` — ровно один read-only capture provenance: `git rev-parse HEAD`, `git rev-parse HEAD^{tree}`, `git status --porcelain`, SHA-256 по явному списку замороженных статусных записей;
- append-only запись финального состояния коллекции (устаревший финальный блок замещается, не удаляется) и `status_lint.py`;
- без `cargo`, тестов, сборок, правок продукта, `commit`/`amend`/`reset`, `push`, GitHub;
- только необходимые записи `.pdca`;
- `0.00 s` Demucs не считается перепроверкой паритета;
- при недостаточности свидетельств — честный STOP.

Постоянные разрешения, действующие в этом цикле и переносимые в финальный блок:

- **`target/`:** временные артефакты сборки Cargo (`target/`) в изолированном detached candidate worktree, созданные единственным запуском `cargo test` цикла `N=3`, — `Признать build-артефакты допустимыми` (дословное решение пользователя, зафиксированное в `.pdca/status/cycle-open-issues-r5-int-followup-r5.md` §Original task);
- **amend:** единственное разрешённое историческое отклонение — `b15beb2 → 3130f55`; никакое иное переписывание истории не авторизовано.

## PLAN record

### r1

# План цикла `N=6` (`PLAN` `r1`) — документарная финализация коллекции после `N=5` с авторизованным read-only provenance capture

## 1. Форма, специализация, идентификаторы, пути и связность по указателям

- **Form:** `cycle` (обычный цикл; fresh post-ACT follow-up цикл `N=6` после завершения `N=5` в `ACT` с `outcome: incomplete (STOP — F-R5-CONTINUITY-EVIDENCE / F-R5-EXECUTION-EVIDENCE)`).
- **Specialized skill (overlay):** `none`.
- **One-line form rationale:** метод полностью определён прямым разрешением пользователя (ровно один read-only захват provenance из четырёх фиксированных операций, независимый `CHECK` всех 10 критериев по захвату и замороженным свидетельствам, затем в `ACT` append-only запись замещающего финального блока в запись коллекции и прогон `status_lint.py`), объём ограничен одним новым статусным файлом, одним каталогом свидетельств и одним append-only дополнением.
- **Cycle ID:** `cycle-open-issues-r5-int-followup-r6`; `N=6`, `r=1`, `n=1`.
- **New status path:** `.pdca/status/cycle-open-issues-r5-int-followup-r6.md`; **evidence root:** `.pdca/cycle-open-issues-r5-int-followup-r6/`.
- **Entry scope:** фиксация pre-cycle `Notice`; один авторизованный capture; независимый `CHECK` 10 критериев; при `all met` — append-only финализация коллекции в `ACT` + `status_lint.py`.
- **Re-anchor:** `HEAD = d042c2628edb52deab42118e18ac69974e5ad75d`, `HEAD^{tree} = 6cb258a028e0c3db51547a4a0b987fdb9e8e3e51`; цепочка `#2 → 59fb1fb`, `#8 → a38bb7f`, `#3 → 3130f55`, `#4 → d042c26`; коллекция до ACT: `phase: ACT`, `N=1`, `outcome: incomplete (STOP)`, `323295` B, sha256 `c2f02e49…`, blob `0e70f7b…`; устаревший финальный блок не удаляется и не перезаписывается.
- **Pointer-only:** коллекция; item-записи и их паритетные артефакты; `N=2`, `N=3`, `N=4`, `N=5` (frozen).

## 2. Pre-cycle `Notice`

Обязательная прозрачная фиксация инцидента `wc -l` (exit 1) в `pdca-scout` с атрибуцией и неопределённостью; запрет повторения и любых иных проб.

## 3. Порядок фаз

1. `PLAN` — контракт; коллекция не меняется.
2. `DO` — ровно четыре read-only операции захвата + сохранение сырых выходов и манифеста в `.pdca/cycle-open-issues-r5-int-followup-r6/`; запись §DO; коллекция не модифицируется.
3. `CHECK` — независимая оценка 10 критериев по захвату + замороженным свидетельствам + текущему состоянию коллекции + журналу команд `DO`.
4. `ACT` — при `10/10 met`: append-only замещающий блок + обновление `outcome`, закрытие `N=6` как `done (accepted)`, `status_lint.py` на обоих файлах; иначе коллекция не меняется, `N=6` → `incomplete (STOP)` с остатком, lint записи `N=6`.

## 4. Задача, результат, метод, критерии

- Исходная задача — по ссылке; item-записи `4/4 accepted`.
- Авторизация `N=6`: один read-only provenance capture (`git rev-parse HEAD`, `git rev-parse HEAD^{tree}`, `git status --porcelain`, SHA-256 замороженных статусных записей); append-only запись финального состояния коллекции; `status_lint.py`; без cargo/тестов/сборки/правок продукта/commit/amend/reset/push/GitHub; только необходимые `.pdca` записи; устаревший финальный блок замещается append-only; `target/` допустим; amend только `3130f55`; `0.00 s` Demucs не считается перепроверкой паритета; при недостаточности свидетельств — честный STOP.
- Метод: точное совпадение SHA-1 `HEAD`/`HEAD^{tree}`; разбор `git status --porcelain` (продуктовая грязь — любая строка вне `.pdca/` и `.openidea/`); точное совпадение SHA-256 всех замороженных статусных записей; подсчёт команд `N=6` (ровно 4 + lint).
- Критерии (10):
  1. `INT-BUILD-TESTS` — `cargo test` на `d042c26` (из `N=3`): exit 0, 25 `ok.`, `144/0/4`, `HEAD`/`tree` из захвата совпадают; `ZERO-EXIT-NOT-SUFFICIENT`; в `N=6` cargo не запускался.
  2. `INT-PARITY-AND-GATES` — 4 задачи `accepted` + численные пороги по принятым item-артефактам; **запрещено** выдавать `0.00 s` Demucs за перепроверку паритета.
  3. `INT-HYGIENE` — в `git status --porcelain` только `M .pdca/status/collection-open-issues-r5.md` и неотслеживаемые `.pdca/`/`.openidea/`; любые продуктовые строки — нарушение.
  4. `INT-TRACEABILITY` — цепочка задача→коммит→запись→закрытый issue→свидетельство; amend только `3130f55`.
  5. `INT-LIMITS` — явные оговорки: Demucs `0.00 s` не паритет; `target/` допустим; amend только `3130f55`; сохранены `F-R4-EXECUTION`, `F-R5-*` и pre-cycle `Notice`.
  6. `FU-CANDIDATE-PIN` — `HEAD`/`tree` из свежего захвата равны эталонам.
  7. `FU-ISOLATION` — `git status --porcelain` без продуктовых строк.
  8. `FU-R3-IMMUTABILITY` — SHA-256 всех замороженных записей (7 базовых + `N=3`/`N=4`/`N=5`) совпадают с эталонами; pre-ACT коллекция `c2f02e49…`/`323295` B.
  9. `FU-HISTORY-PRESERVATION` — устаревший блок сохранён; append-only замещение только в `ACT`; `N=1..N=5` нетронуты.
  10. `FU-R6-EXECUTION-SCOPE` — только 4 операции захвата + `status_lint.py`; 0 cargo/тестов/сборок/листингов/git-записей/GitHub; lint exit 0.

## 5. Точная спецификация capture

- Корень: `.pdca/cycle-open-issues-r5-int-followup-r6/` (`provenance-capture.txt` или `head.txt`, `tree.txt`, `status-porcelain.txt`, `frozen-status-sha256.txt`).
- Операция 1: `git rev-parse HEAD` → `d042c2628edb52deab42118e18ac69974e5ad75d`.
- Операция 2: `git rev-parse HEAD^{tree}` → `6cb258a028e0c3db51547a4a0b987fdb9e8e3e51`.
- Операция 3: `git status --porcelain` — читать буквально; допустимы ` M .pdca/status/collection-open-issues-r5.md` и `?? .pdca/…`/`?? .openidea/…`; любая строка с путём вне `.pdca/`/`.openidea/` = продуктовая грязь → STOP.
- Операция 4: SHA-256 по явному списку 10 файлов: `issue-{2,8,3,4}-r5.md`, `cycle-open-issues-r5-int-followup-r1.md`, `-r2.md`, `collection-open-issues-r5.md` (pre-ACT), `-r3.md`, `-r4.md`, `-r5.md`.
- Привязка: §DO/§CHECK записи `N=6` указывают пути артефактов, значения `HEAD`/`tree`, разбор `status` и таблицу SHA-256 с сопоставлением эталонам.

## 6. Спецификация append-only блока (только в `ACT` после `10/10 met`)

- Весь существующий текст коллекции, включая устаревший блок `N=1` `incomplete (STOP)`, остаётся без удаления/правки; в шапке синхронизируется только итоговый `outcome` (`done`/`accepted` по схеме lint); в конец файла строго append-only добавляется `## Post-ACT Superseding Finalization Record (N=6 — cycle-open-issues-r5-int-followup-r6)`.
- Содержание: декларация замещения; дословные решения/авторизации пользователя (capture, `target/`, amend `3130f55`); сводка 4/4 `accepted` с коммитами и паритетными артефактами; матрица `10/10 met` со ссылками; переносимые оговорки (Demucs `0.00 s` не паритет; `target/`; amend; сохранение `F-R4-EXECUTION`, `F-R5-*`, pre-cycle `Notice`); декларация `no history rewrite / no product edits / no commit / no amend / no reset / no push / no GitHub writes / no issue reopen`; финальный исход `done`.

## 7. Ограничения, допущения, пробелы, минимальное решение

- Только 4 операции + `.pdca` записи + `status_lint.py`; запрет cargo/тестов/сборки/правок/git-записей/push/GitHub/листингов; замороженные записи read-only.
- Допущений нет. Пробелы: непрерывность после `N=3` закрывается захватом; инцидент `wc -l` — прозрачным `Notice`.
- Минимальное решение: capture → CHECK → append-only блок и lint. Трудоёмкость ~10–15 минут.

## 8. Риски, стоп-условия, терминальные исходы

- STOP при: любой команде вне разрешённых; расхождении `HEAD`/`tree`; продуктовой строке в `status`; расхождении SHA-256 замороженной записи; необходимости переписать историю; расширении `target/`/amend-исключений; ошибке lint, требующей выхода за разрешённые правки.
- Ветка 1 (`10/10 met`): append-only блок + `outcome`, `N=6` → `done (accepted)`, lint обоих файлов (exit 0).
- Ветка 2: коллекция не меняется (`323295` B, `c2f02e49…`), `N=6` → `incomplete (STOP)` с остатком и вопросом пользователю, lint записи `N=6`.

## 9. Handoff и уверенность

- DO создаёт запись `N=6` (с планом и `Notice`), каталог свидетельств, выполняет ровно один capture из четырёх операций, документирует результаты; коллекцию не редактирует; иных команд не запускает.
- **Confidence and gaps:**
  - *Verified:* точные правила `references/plan.md`, структура и ограничения пользовательской авторизации `N=6`, эталонные идентификаторы `HEAD`/`HEAD^{tree}` и дайджест pre-ACT записи коллекции из брифа, а также состав замороженных артефактов `N=3`..`N=5`.
  - *Remains to be observed in `DO`/`CHECK`:* фактические выходные значения четырёх read-only операций захвата в `N=6` и прохождение `status_lint.py`.

## Approval record

`not applicable: autonomous mode`

## DO result

`DO` завершён: создана эта запись, зафиксирован pre-cycle `Notice`, создан каталог свидетельств и выполнены **ровно четыре** авторизованные read-only операции захвата с сохранением сырых выходов. Запись коллекции `.pdca/status/collection-open-issues-r5.md` **не тронута**. Это результат `DO`, **не вердикт**: независимый `CHECK` этого цикла не выполнялся, ни один критерий не оценён.

Единицы выполнения `DO`: не делились — один проход (`DO-1`), статус `в работе до CHECK`, критерии покрытия — все десять по указателям, зависимостей и замен нет.

### 0. Pre-cycle `Notice` (verbatim, до `PLAN`)

> **Pre-cycle `Notice` (`N=6`, pre-PLAN recovery):** в ходе предциклового read-only сбора состояния субагент `pdca-scout` самостоятельно сообщил, что выполнил одну неавторизованную его read-only задачей shell-пробу `wc -l` по файлу коллекции, завершившуюся кодом возврата `1` (ошибка исполнения/отсутствие вывода). Атрибуция и неопределённость: факт известен из самоотчёта `pdca-scout`; команда не произвела вывода и, по наблюдаемым данным состояния, не изменила ни одного файла рабочей копии или `.pdca/`. В рамках `N=6` эта команда (и любые другие shell-листинги/пробы вне четырёх разрешённых операций захвата и `status_lint.py`) строго запрещена и не повторяется.

Пометка записи: `Notice` возник **до** `PLAN` этого цикла; он воспроизведён дословно и должен быть перенесён в append-only финальный блок коллекции на `ACT`.

### 0a. `Notice` — исполнительская проба квотирования при операции 2 (не команда вне набора)

Первый вызов операции 2 был выполнен как `git rev-parse HEAD; git rev-parse HEAD^{tree}` в одной shell-строке; в `pwsh` последовательность `^{tree}` была разобрана как script block, `git` получил base64-строку `dAByAGUAZQA=` и вернул `fatal: ambiguous argument 'dAByAGUAZQA=': unknown revision or path not in the working tree` с итоговым `exit 1`. Вывод пуст, состояние не изменено. Операция 2 перезапущена в квотированном виде `git rev-parse "HEAD^{tree}"` → `exit 0`, эталонное значение. Это не выход за авторизованный набор команд: та же операция 2, исправлено только квотирование. Учтено в счётчике как отдельный shell-вызов (см. п. 1).

### 0b. `Notice` — короткие пути в плане против реальных корней

План (`## 4`, критерий 2) ссылается на «принятые item-артефакты», а критерий 9 — на записи `N=3`/`N=4`/`N=5`; в тексте плана пути даны сокращённо (`.pdca/item-*`-подобные указатели). Реальные корни, которыми оперирует цикл: `.pdca/open-issues-r5/rv1/item-{2-mdx-onnx,8-ensembles,3-vr-uvr,4-demucs}/` (item-доказательства) и `.pdca/cycle-open-issues-r5-int-followup-r{3,4}/` (прогон `N=3`, снимки `N=4`). Текст плана не правится; привязка критериев идёт по реальным корням, которые и являются управляющими.

### 1. Операции захвата — точный argv, exit-коды, значения

Все вызовы — read-only; рабочий каталог `F:\sources\candle-roformer`. Всего в `DO` — 5 shell-вызовов на 4 авторизованные операции (см. `Notice` 0a).

| # | argv | exit | значение | артефакт |
|---|---|---|---|---|
| 1 | `git rev-parse HEAD` | `0` | `d042c2628edb52deab42118e18ac69974e5ad75d` | `.pdca/cycle-open-issues-r5-int-followup-r6/head.txt` |
| 2 | `git rev-parse "HEAD^{tree}"` | `0` | `6cb258a028e0c3db51547a4a0b987fdb9e8e3e51` | `.pdca/cycle-open-issues-r5-int-followup-r6/tree.txt` |
| 3 | `git status --porcelain` | `0` | 9 строк, см. п. 2 | `.pdca/cycle-open-issues-r5-int-followup-r6/status-porcelain.txt` |
| 4 | `Get-FileHash -Algorithm SHA256` по явному списку 10 файлов (foreach, без glob и без листинга) | `0` | 10 строк, см. п. 3 | `.pdca/cycle-open-issues-r5-int-followup-r6/frozen-status-sha256.txt` |

Сопоставление с эталонами: результат операции 1 — `match` (`HEAD` эталон брифа и `N=4`-снимка); результат операции 2 — `match` (`HEAD^{tree}` эталон брифа и `N=4`-снимка).

### 2. Классификация `git status --porcelain`

Сырой вывод (9 строк) сохранён в `status-porcelain.txt`:

- ` M .pdca/status/collection-open-issues-r5.md` — ожидаемая, **предсуществующая** модификация записи коллекции (не этим циклом; атрибуция — `N=4/logs/frozen-record-hashes.txt:24–25`);
- `?? .openidea/` — неотслеживаемый каталог вне продукта, допустим;
- `?? .pdca/status/collection-open-issues-r3.md`, `?? .pdca/status/cycle-open-issues-r5-int-followup-r1.md`, `-r2.md`, `-r3.md`, `-r4.md`, `-r5.md`, `?? .pdca/status/issue-4-r5.md` — неотслеживаемые статусные записи, допустимы.

**Продуктовых строк нет (`0`).** Ни одна строка не указывает на путь вне `.pdca/` и `.openidea/`; отслеживаемых изменений продукта, индекса или удалений нет. Стоп-условие плана (`## 5`, операция 3) не сработало. Пометка честной атрибуции: собственные файлы этого цикла (эта запись и каталог `cycle-open-issues-r5-int-followup-r6/`) созданы **после** захвата и в этом выводе отсутствуют; их появление в `git status` следующим запуском не проверялось (команда вне авторизации).

### 3. Таблица SHA-256 замороженных записей и сопоставление с эталонами

Формат строки: `sha256  bytes  path`. Источник эталонов: `S4` = `.pdca/cycle-open-issues-r5-int-followup-r4/logs/frozen-record-hashes.txt` (снимок `2026-11-13T06:36:54Z`), `S3` = `.pdca/cycle-open-issues-r5-int-followup-r3/logs/frozen-identities-after-act.txt`.

| file | sha256 (this capture) | bytes | reference | match |
|---|---|---|---|---|
| `.pdca/status/issue-2-r5.md` | `510f5ab8943efe8078413423fdfd678a70c5f03d13e0a8d52bdd05ea571f14b8` | `109966` | `S4:10` | `match` |
| `.pdca/status/issue-8-r5.md` | `dae960b0120d4558629e779493b7ad6c3f76fb4aba2c881b6b3e478859d717e8` | `246554` | `S4:11` | `match` |
| `.pdca/status/issue-3-r5.md` | `1420cba8c854cef9c79b0bf943e89b143f1ac4f8c57554764dad64fa92a6ab3c` | `67539` | `S4:12` | `match` |
| `.pdca/status/issue-4-r5.md` | `d4b554b3e308381b2820ad4486fe7386971db703d1789fc4186a53c45712d6d8` | `118099` | `S4:13` | `match` |
| `.pdca/status/cycle-open-issues-r5-int-followup-r1.md` | `c8544323caf11243eb260dd73b30e809d4934d32fc372394f614eefa4efe24e9` | `48215` | `S4:16` | `match` |
| `.pdca/status/cycle-open-issues-r5-int-followup-r2.md` | `2219f4690df7fa289c3dea9bf572b949df76620586ebfbc57809e57fefce3b22` | `69803` | `S4:8` | `match` |
| `.pdca/status/collection-open-issues-r5.md` | `c2f02e495e6e9ae300c7a609038df8961b03664e49a3238191bcc6c4e5a0f063` | `323295` | `S4:7` (pre-ACT) | `match` |
| `.pdca/status/cycle-open-issues-r5-int-followup-r3.md` | `b65d88a81c47727455442d0c8a3ac88d174e59bb60270c9830753ad461acb769` | `57862` | `S4:9` | `match` |
| `.pdca/status/cycle-open-issues-r5-int-followup-r4.md` | `1de1cde56f79a4a4ee2ac89a094d0365455454542bd4506f5940f4ac827b8a05` | `75238` | нет эталона | `unverified` |
| `.pdca/status/cycle-open-issues-r5-int-followup-r5.md` | `151072aeba97b0049699dfef589c6dc28a6ba221095c2fcedb6f3d79d943d54b` | `85411` | нет эталона | `unverified` |

Итог: **8 `match`**, **0 `mismatch`**, **2 `unverified`**. `unverified` — не расхождение: для `-r4.md` (создан в `N=4` после последнего снимка) и `-r5.md` (создан в `N=5` после последнего снимка) записанного SHA-256-эталона в замороженных артефактах нет; вычислять «эталон» сравнением с самим собой запрещено, поэтому строка честно помечена `unverified`. Причина отсутствия эталона — те же пробелы `F-R5-CONTINUITY-EVIDENCE`/`F-R5-EXECUTION-EVIDENCE`, которые закрывает этот захват.

**Pre-ACT запись коллекции:** `sha256 = c2f02e495e6e9ae300c7a609038df8961b03664e49a3238191bcc6c4e5a0f063`, `bytes = 323295` — совпадает с перенесённым эталоном брифа (`c2f02e49…` / `323295` B) и со снимком `S4:7`; blob `0e70f7b774a89a4cde8d5080cc9e97a1f66f61f5` в этом захвате **не** вычислялся (`git hash-object` вне разрешённого набора) и приводится только как замороженный указатель.

### 4. Что в этом `DO` не выполнялось

Не выполнялось: `cargo`, тесты, сборки, любые `git`-записи (`commit`/`amend`/`reset`/`checkout`/`add`/`push`), `git write-tree`, `git log`, `wc`, `ls`/`dir`/`Get-ChildItem`, произвольный Python, сетевые вызовы и обращения к GitHub; ни одной правки продуктового или отслеживаемого файла; ни одной записи в `.pdca/status/collection-open-issues-r5.md` (файл остаётся `323295` B / `c2f02e49…`, что и подтверждено операцией 4) и ни в одну замороженную запись. Инспекция выполнялась файловыми инструментами чтения.

### 5. Учёт исполнения

`commands_run = 4` — четыре авторизованные операции захвата (§1), выполненные в 5 shell-вызовах (`Notice` 0a). Плюс один разрешённый прогон `status_lint.py` по этой записи — итого `commands_run = 5`; его stdout и `exit` фиксируются в §Progress log последней строкой.

## CHECK record

Записано дословно, без правок: источник — сырой отчёт `pdca-check` (обычный режим `CHECK`), переданный драйвером в бриф `ACT`. Отдельного сохранённого артефакта отчёта до этой записи не существовало; настоящий раздел является его полной записью.

## Вердикт: `not accepted`

**8 `met` / 0 `unmet` / 2 `unverified`. ACT пока не вправе выполнять append-only закрытие коллекции.** Остаток — недоказанная неизменность записей `N=4` и `N=5`, а не установленная мутация.

Счётчики прочитаны из статус-записи: **`N=6 · r=1 · n=1`**.

Обозначения: `S6` — `.pdca/status/cycle-open-issues-r5-int-followup-r6.md`; `E6`, `E4`, `E3` — `.pdca/cycle-open-issues-r5-int-followup-r{6,4,3}/`; `C` — `.pdca/status/collection-open-issues-r5.md`; `I` — `.pdca/open-issues-r5/rv1/`.

### Матрица

| Критерий | Статус | Основание и свидетельство |
|---|---|---|
| `INT-BUILD-TESTS` | `met` | Исторический `cargo test`: exit `0`, завершённая сборка, **25** итогов `ok.`, **144 passed / 0 failed / 4 ignored**. Прочитаны `E3/logs/cargo-test-d042c26.log:528–831`, `manifests/run-state-result.json`. Свежие HEAD/tree совпадают с идентичностью прогона из `E3/manifests/comparison.json`. Нового Cargo-прогона N=6 не зафиксировано. |
| `INT-PARITY-AND-GATES` | `met` | Четыре item-записи имеют `outcome: accepted`; непосредственно прочитанные принятые измерения укладываются в пороги. `#2`: max_abs `1.4231e-6`, худший RMSE `1.6879e-7`, SDR ≥`117.94 dB` (`I/item-2-mdx-onnx/logs/comparison.json`). `#8`: max_abs/RMSE `0/0` для обоих операторов (`item-8-ensembles/metrics/parity.json`). `#3`: max_abs ≤`1.40071e-6`, RMSE ≤`3.55737e-8`, SDR ≥`112.72 dB` (`item-3-vr-uvr/logs/41-check-parity-after-reference-rerun.log:118–125`). `#4`: **8/8**, max_abs ≤`8.583e-6`, RMSE ≤`1.774e-7`, SDR ≥`84.36 dB` (`item-4-demucs/logs/62-parity-after-ctr4.log:189–201`, `metrics/parity.json`, `logs/66-gate-test-full.log`, exit `0`). **Demucs `0.00 s` в N=3 не является измерением паритета.** |
| `INT-HYGIENE` | `met` | Буквально прочитан `E6/status-porcelain.txt`: **9 строк, 0 продуктовых**. Единственная tracked-модификация — ` M .pdca/status/collection-open-issues-r5.md`; остальные — untracked `.pdca/status/*` и `.openidea/`. Это не буквально чистый checkout, но допустимое состояние по данному критерию. |
| `INT-TRACEABILITY` | `met` | Согласованы `#2→59fb1fb`, `#8→a38bb7f`, `#3→3130f55`, `#4→d042c26`, принятые item-записи и сохранённые закрытия issues (`C:167–211,314–320`). Историческая amend-пара непосредственно присутствует в `I/integration/evidence/int-f03-no-push-reflog.txt:32–33`: только `b15beb2→3130f55` как разрешённое исключение. |
| `INT-LIMITS` | `met` | Требуемые оговорки явно сохранены: `target/` допустим по решению пользователя; amend-исключение узкое; `0.00 s` не паритет; `F-R4-EXECUTION`, `F-R5-*` и pre-cycle Notice не сняты (`S6:28–31,121–133,203–205`; записи N=4/N=5, §§CHECK/ACT/Defect history). |
| `FU-CANDIDATE-PIN` | `met` | `E6/head.txt` = `d042c2628edb52deab42118e18ac69974e5ad75d`; `E6/tree.txt` = `6cb258a028e0c3db51547a4a0b987fdb9e8e3e51`. Точное совпадение с `E3/manifests/comparison.json:2–8` и `E4/logs/frozen-record-hashes.txt:19–20`. |
| `FU-ISOLATION` | `met` | Свежий `E6/status-porcelain.txt:1–9` содержит **0 путей вне `.pdca/` и `.openidea/`**. Ожидаемая tracked-модификация C не является продуктовой грязью; её прежнее происхождение согласуется с `E4/logs/frozen-record-hashes.txt:24–25`, текущий digest совпадает. |
| `FU-R3-IMMUTABILITY` | `unverified` | Самостоятельно сопоставлены `E6/frozen-status-sha256.txt` и `E4/logs/frozen-record-hashes.txt`: **8 совпадений / 0 расхождений / 2 без базы сравнения**. C совпадает: `323295 B`, `c2f02e49…`. Для записей N=4 (`1de1cde5…`) и N=5 (`151072ae…`) есть только текущие значения, не исторические эталоны. Их неизменность этим не установлена. |
| `FU-HISTORY-PRESERVATION` | `unverified` | Старый финальный STOP-блок действительно сохранён (`C:183–211`), C совпадает с историческим digest; supersession ещё не применён. В N=4/N=5 непосредственно видны прежние нарушения и STOP. Но составное требование **«N=1..N=5 нетронуты»** полностью не доказано: отсутствует независимая база для побайтового сравнения N=4/N=5. Содержательная сохранность ключевых блоков не доказывает неизменность всего файла. |
| `FU-R6-EXECUTION-SCOPE` | `met` | По предоставленному журналу операций и сохранённым выходам выполнен один capture четырёх разрешённых операций, затем разрешённый lint — `exit 0` (`S6:127–144,183–185,227–233`). Исправление квотирования **той же** tree-операции после exit `1` не добавляет иной операции или нового capture; это не нарушение данного бюджета. `commands_run=5` обозначает логические операции вместе с lint, **не количество shell-вызовов**. Полнота скрытых действий отдельно не установлена. |

### Findings

- **`F-R6-FROZEN-BASELINE` — blocking, `confirmed` как доказательственный пробел.** Затрагивает `FU-R3-IMMUTABILITY` и часть `FU-HISTORY-PRESERVATION`. Текущий хэш двух файлов не подтверждает их историческую неизменность. Самоотчёты «не тронуты», сохранённые STOP-блоки, отсутствие продуктовой грязи и одинаковый HEAD этого не заменяют.
- **Нарушение scope из-за повторного квотирования — `refuted` в пределах предоставленного журнала.** Неудачный вызов и исправленный вызов относятся к одной разрешённой read-only операции. Ошибка не скрыта; отсутствие результата не выдано за успешный захват.
- **«N=3 перепроверил Demucs-паритет» — `refuted`.** Принятие соответствующей строки основано на item-level измерениях, не на быстром тестовом результате.
- Исторические `F-R4-EXECUTION` и `F-R5-*` остаются сохранёнными; этот CHECK не меняет исходов N=4/N=5.

### Критерии и покрытие

Десять критериев покрывают текущий документарный follow-up и его ограничения. Пороги не ослаблены; будущая ACT-запись не принята заранее. Отсутствие двух эталонов — **пробел свидетельств**, не доказанный дефект продукта и не основание менять критерии.

### Маршрутизация

**→ DO через драйвер: один документарный сбор уже существующей независимой базы для N=4/N=5**, без новых команд и без изменения замороженных файлов.

Закрыть остаток может сохранённый финальный снимок после соответствующего ACT: исторический digest либо независимая неизменяемая полная копия/полный журнал записи, позволяющие установить точное соответствие нынешним файлам. **Повторное хэширование текущих файлов историческую базу не создаст.**

Если такой базы нет — предусмотренный авторизацией **`incomplete (STOP)`**, коллекция остаётся без изменений; повторный раунд по тому же критерию в этой ревизии не запрашивается. **Append-only закрытие сейчас запрещено.**

### Пределы и уверенность

Проверка выполнена только чтением `.pdca`-артефактов; ничего не запускалось и не пересчитывалось. Capture устанавливает состояние на момент захвата, не полный аудит промежуточных или скрытых действий. Линт подтверждён записью stdout/exit, отдельного внешнего журнала запуска не предоставлено. Четыре ignored-теста не исполнялись; нового GPU/Python-покрытия и удалённого наблюдения нет.

**Уверенность высокая** в совпадении восьми строк, идентичности кандидата, буквальном status и наличии остатка; недостаточная для `10/10 met`.

## ACT record

`ACT` закрыт по терминальному маршруту `CHECK → ACT`, `not accepted`. Вердикта не выносится, принятия нет.

- `outcome: incomplete (STOP — F-R6-FROZEN-BASELINE)` · `record: completed` · phase `ACT` · `commands_run = 5` (четыре авторизованные read-only операции захвата в пяти shell-вызовах, `Notice` 0a, плюс один разрешённый прогон `status_lint.py` по этой записи; финальный прогон — в `ACT`, `exit 0`).

**Установлено — восемь критериев `met`:**

- `INT-BUILD-TESTS` — исторический `cargo test` на `d042c26`: exit `0`, 25 `ok.`, `144 passed / 0 failed / 4 ignored`; новый Cargo-прогон в `N=6` не выполнялся.
- `INT-PARITY-AND-GATES` — четыре item-записи `accepted`, принятые измерения в порогах; Demucs `0.00 s` из `N=3` **явно не является измерением паритета**, паритет `#4` принят на item-level свидетельстве.
- `INT-HYGIENE` — `E6/status-porcelain.txt`: 9 строк, **0 продуктовых**.
- `INT-TRACEABILITY` — цепочка `#2→59fb1fb`, `#8→a38bb7f`, `#3→3130f55`, `#4→d042c26`; amend только `b15beb2→3130f55`.
- `INT-LIMITS` — оговорки (`target/`, узкий amend, `0.00 s`, `F-R4-EXECUTION`, `F-R5-*`, pre-cycle `Notice`) сохранены.
- `FU-CANDIDATE-PIN` — свежие `HEAD = d042c2628edb52deab42118e18ac69974e5ad75d` и `HEAD^{tree} = 6cb258a028e0c3db51547a4a0b987fdb9e8e3e51`, точное совпадение с эталонами.
- `FU-ISOLATION` — 0 путей вне `.pdca/` и `.openidea/`.
- `FU-R6-EXECUTION-SCOPE` — один capture разрешённых операций + lint; неудачный вызов и его квотированный повтор относятся к **одной** операции, нарушением бюджета не признаны.

**Остаток `unverified` (2):** `FU-R3-IMMUTABILITY` и `FU-HISTORY-PRESERVATION`. Для записей `N=4` (`1de1cde5…`) и `N=5` (`151072ae…`) нет исторического digest или независимой неизменяемой копии, поэтому их побайтовая неизменность за интервал не устанавливается; повторное хэширование текущих файлов базу не создаёт. `F-R6-FROZEN-BASELINE` подтверждён как **пробел свидетельств**; **мутация не установлена**.

**Коллекция:** append-only закрытие и замещающий блок **не применены**; `.pdca/status/collection-open-issues-r5.md` остаётся byte-identical — `323295` B, sha256 `c2f02e49…`.

**Сохранено:** четыре item-принятия и их закрытия; записи `N=3`/`N=4`/`N=5` с их находками (`F-R4-EXECUTION`, `F-R5-*`); pre-cycle `Notice` (`wc -l`, exit 1); правок продукта, `commit`, `push`, обращений к GitHub и переоткрытий issue — нет.

**Handoff / следующее авторизованное действие:** решение **пользователя/планера** по двум свидетельственным строкам — либо (a) принять, что неизменность записей `N=4`/`N=5` не может быть установлена ретроспективно, и записать закрытие с этой явной оговоркой, либо (b) предоставить любую существующую независимую копию/журнал этих двух записей, пригодную как база сравнения. Здесь ничего из этого не выполняется.

## Lessons learned

Записано дословно, без правок: источник — сырой ретроспективный отчёт `pdca-check` (`retrospective` mode), переданный драйвером в бриф `ACT`. Отчёт был составлен до этой записи («отчёт не записывался»); настоящий раздел — его персистенция, и только после неё `ACT` считается завершённым.

## Ретроспектива

1. **Происхождение свидетельств.** Свежий хэш фиксирует текущее состояние, но без исторического эталона не доказывает неизменность. Для `-r4.md` и `-r5.md` таких эталонов нет. На будущее: сохранять независимый датированный снимок после завершения записи, до её заморозки.
   **Основание:** статус `N=6`, §DO result, п. 3; переданный итог CHECK, `F-R6-FROZEN-BASELINE`.

2. **Планирование доказательств.** План предполагал, что новый capture закроет непрерывность, хотя два сравнения не имели базы. На будущее: перед обещанием закрыть исторический пробел сопоставлять каждому требованию существующий источник сравнения; повторное хэширование не восстанавливает прошлое.
   **Основание:** §PLAN record, пп. 7–9; §DO result, п. 3.

3. **Ограничение исполнения и прозрачность.** Сработали явная фиксация ошибки квотирования, раздельный учёт четырёх операций и пяти shell-вызовов, сохранение оговорок и запрет изменения коллекции без полного подтверждения. На будущее: заранее учитывать синтаксис целевой оболочки и сохранять такое разделение счётчиков.
   **Основание:** §DO result, Notice 0a, пп. 1 и 5; §Progress log.

**Пределы и неопределённость:** новых проверок не проводилось. Прочитанная статус-запись ещё содержит `CHECK: not run` и `ACT: pending`; итог CHECK и терминальное завершение доступны только в сводке диспетчера, полный сырой отчёт не предоставлен. Отсутствие эталонов не доказывает изменение файлов.

**Сигнал:** ретроспективный анализ завершён; отчёт не записывался.

## Revision history

`none`

## Defect history

`none` для исправлений — переносимые ключи: `F-R4-EXECUTION`, `F-R4-NOTICE-BASIS` → `.pdca/status/cycle-open-issues-r5-int-followup-r4.md` §Defect history; `F-R5-CONTINUITY-EVIDENCE`, `F-R5-EXECUTION-EVIDENCE` → `.pdca/status/cycle-open-issues-r5-int-followup-r5.md` §CHECK record. В этом цикле правок не применялось (`fixes applied: 0`).

| key | revisions/attempts observed | fixes applied | evidence | last recurrence/escalation |
|---|---|---|---|---|
| `F-R6-FROZEN-BASELINE` | `r1` · `n=1` (единственное наблюдение) | `0` | §CHECK record (findings, `FU-R3-IMMUTABILITY`, `FU-HISTORY-PRESERVATION`); `E6/frozen-status-sha256.txt`; `E4/logs/frozen-record-hashes.txt`; коллекция `323295` B / `c2f02e49…` | новое; подтверждён как пробел свидетельств, мутация не установлена; эскалация — решение пользователя/планера (см. §ACT record, Handoff) |

## Changed files

- `.pdca/status/cycle-open-issues-r5-int-followup-r6.md` → создана в `DO` и финализирована в `ACT` (эта статус-запись цикла `N=6`)
- `.pdca/cycle-open-issues-r5-int-followup-r6/head.txt` → сырой выход операции 1
- `.pdca/cycle-open-issues-r5-int-followup-r6/tree.txt` → сырой выход операции 2
- `.pdca/cycle-open-issues-r5-int-followup-r6/status-porcelain.txt` → сырой выход операции 3
- `.pdca/cycle-open-issues-r5-int-followup-r6/frozen-status-sha256.txt` → сырой выход операции 4
- `.pdca/status/collection-open-issues-r5.md` → **не изменена** этим циклом (только чтение и хэширование)

## Git state

- status path: `.pdca/status/cycle-open-issues-r5-int-followup-r6.md` — created in `DO`, modified (finalized) in `ACT` this cycle · untracked (durable on disk, not committed)
- baseline, переданный циклом: `HEAD = d042c2628edb52deab42118e18ac69974e5ad75d`, `HEAD^{tree} = 6cb258a028e0c3db51547a4a0b987fdb9e8e3e51`, `origin/main = 8b03cfd`; снят операциями 1–2 этого `DO` и совпал с эталоном
- наблюдаемое состояние рабочей копии (операция 3): ` M .pdca/status/collection-open-issues-r5.md` (предсуществующая модификация, не этим циклом) плюс неотслеживаемые `.openidea/` и семь `.pdca/status/*.md`; продуктовых строк `0`
- same-cycle touched-file ledger: `.pdca/status/cycle-open-issues-r5-int-followup-r6.md` (untracked, создан) и четыре файла под `.pdca/cycle-open-issues-r5-int-followup-r6/` (untracked, созданы); в индекс ничего не добавлялось, коммитов не было; task changes pending commit: `0`

## Progress log

- `unknown (2026-11-13, часы недоступны без команды вне авторизованного набора) | PLAN | revision r1 | attempt n=1 | pre-cycle Notice зафиксирован; план планера записан дословно в §PLAN record r1; команды не запускались | .pdca/status/cycle-open-issues-r5-int-followup-r6.md`
- `unknown | DO | revision r1 | attempt n=1 | DO started: создан каталог свидетельств, начат единственный авторизованный capture | .pdca/cycle-open-issues-r5-int-followup-r6/`
- `unknown | DO | revision r1 | attempt n=1 | Notice: первая попытка операции 2 (`git rev-parse HEAD; git rev-parse HEAD^{tree}`) завершилась exit 1 из-за разбора `^{tree}` в pwsh (fatal: ambiguous argument), вывода нет, состояние не изменено; операция 2 перезапущена квотированной и дала exit 0 | .pdca/status/cycle-open-issues-r5-int-followup-r6.md §DO result 0a`
- `unknown | DO | revision r1 | attempt n=1 | операция 1 `git rev-parse HEAD` exit 0 → d042c2628edb52deab42118e18ac69974e5ad75d (match); операция 2 `git rev-parse "HEAD^{tree}"` exit 0 → 6cb258a028e0c3db51547a4a0b987fdb9e8e3e51 (match) | head.txt, tree.txt`
- `unknown | DO | revision r1 | attempt n=1 | операция 3 `git status --porcelain` exit 0: 9 строк, продуктовых строк 0 (только предсуществующая ` M .pdca/status/collection-open-issues-r5.md` и неотслеживаемые `.openidea/`/`.pdca/status/*`) | status-porcelain.txt`
- `unknown | DO | revision r1 | attempt n=1 | операция 4 SHA-256 по 10 файлам exit 0: 8 match, 0 mismatch, 2 unverified (-r4.md, -r5.md: эталон не записан); pre-ACT коллекция c2f02e49… / 323295 B — match | frozen-status-sha256.txt`
- `unknown | DO | revision r1 | attempt n=1 | DO завершён: commands_run = 4 (capture), запись коллекции не тронута, вердикта нет; следующий авторизованный шаг — независимый CHECK этого цикла | .pdca/status/cycle-open-issues-r5-int-followup-r6.md §DO result`
- `unknown | DO | revision r1 | attempt n=1 | lint: `python .pdca/scripts/status_lint.py .pdca/status/cycle-open-issues-r5-int-followup-r6.md` — см. строку ниже (stdout и exit добавлены после прогона) | —`
- `unknown | DO | revision r1 | attempt n=1 | lint result: status-lint: OK (.pdca\status\cycle-open-issues-r5-int-followup-r6.md: 2 tables, 14 rows), EXIT=0; commands_run = 5; цикл закрыт на границе DO, следующий авторизованный шаг — независимый CHECK | —`
- `unknown | CHECK | revision r1 | attempt n=1 | независимый CHECK выполнен: вердикт not accepted, 8 met / 0 unmet / 2 unverified (FU-R3-IMMUTABILITY, FU-HISTORY-PRESERVATION); новая находка F-R6-FROZEN-BASELINE — пробел свидетельств, мутация не установлена; отчёт записан дословно в §CHECK record | .pdca/status/cycle-open-issues-r5-int-followup-r6.md §CHECK record`
- `unknown | ACT | revision r1 | attempt n=1 | ACT закрыт по терминальному CHECK → ACT: outcome incomplete (STOP — F-R6-FROZEN-BASELINE), record completed; append-only закрытие и замещающий блок не применены; коллекция .pdca/status/collection-open-issues-r5.md не изменена (323295 B, c2f02e49…) | §ACT record`
- `unknown | ACT | revision r1 | attempt n=1 | ретроспектива pdca-check (retrospective mode) записана дословно, без правок; ACT завершён только после этой записи | §Lessons learned`
- `unknown | ACT | revision r1 | attempt n=1 | lint: python .pdca/scripts/status_lint.py .pdca/status/cycle-open-issues-r5-int-followup-r6.md — см. строку ниже (stdout и exit добавлены после прогона) | —`
- `unknown | ACT | revision r1 | attempt n=1 | lint result: status-lint: OK (.pdca\status\cycle-open-issues-r5-int-followup-r6.md: 4 tables, 25 rows), EXIT=0; прогон повторён после добавления этой строки журнала — тот же EXIT=0; учтён как одна разрешённая операция lint; commands_run = 5; ACT закрыт, дальнейших авторизованных шагов в этом цикле нет | —`
