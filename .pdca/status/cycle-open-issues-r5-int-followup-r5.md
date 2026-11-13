# Cycle status: open-issues-r5-int-followup-r5 / cycle 5

## Current state

- task: `open-issues-r5-int-followup-r5` · form: `cycle` · specialized skill: `none`
- mode: `autonomous`
- cycle: `N=5` · revision: `r=1` · attempt: `n=1/3`
- counters: `commands_run = 2` по итогу цикла: (1) DO — единственный разрешённый линт этой статус-записи; (2) ACT — тот же линт на том же файле по явному разрешению финального диспетча (второй запуск разрешён только в этом финальном проходе; ретраев в DO — `0`, повтора внутри прохода — `0`). Продуктовых/тестовых/сборочных прогонов: `0`. Бюджет цикла — только `status_lint.py`; ограничение `N=4` (`F-R4-EXECUTION`) сохраняется в истории и не снимается
- scope: документарный post-ACT follow-up — дословная фиксация решения пользователя «Признать build-артефакты допустимыми», беззапусковая переоценка десяти интеграционных и follow-up критериев только по замороженным доказательствам `N=3`/`N=4`, item-записям и коллекционной записи, проект append-only блока закрытия коллекции; коллекционная запись не меняется до `CHECK: all met` · status path: `.pdca/status/cycle-open-issues-r5-int-followup-r5.md`
- goal: документарно переоценить десять критериев на существующих доказательствах с учётом решения пользователя о допустимости `target/` и подготовить (не применять) append-only блок закрытия коллекции — §PLAN record и §Original task авторитетны
- phase: `ACT` · record: `completed` · outcome: `incomplete (STOP — F-R5-CONTINUITY-EVIDENCE / F-R5-EXECUTION-EVIDENCE)`
- blockers: `CHECK` вернул `not accepted` — `4 met` / `0 unmet` / `6 unverified` (`INT-LIMITS`, `FU-CANDIDATE-PIN`, `FU-ISOLATION`, `FU-R3-IMMUTABILITY`, `FU-HISTORY-PRESERVATION`, `FU-R5-EXECUTION-SCOPE`); мутация **не установлена**, пробел доказательственный; коллекционная запись не закрыта — §CHECK record, §ACT record
- source: бриф цикла `N=5`, выданный после терминального `ACT` цикла `cycle-open-issues-r5-int-followup-r4` (`incomplete (STOP — F-R4-EXECUTION)`) и нового решения пользователя · context refs: `.pdca/status/collection-open-issues-r5.md`, `.pdca/status/cycle-open-issues-r5-int-followup-r3.md`, `.pdca/status/cycle-open-issues-r5-int-followup-r4.md`, `.pdca/cycle-open-issues-r5-int-followup-r3/`, `.pdca/cycle-open-issues-r5-int-followup-r4/`, `.pdca/open-issues-r5/rv1/`, `.pdca/status/issue-{2,8,3,4}-r5.md`
- criteria: `INT-BUILD-TESTS`, `INT-PARITY-AND-GATES`, `INT-HYGIENE`, `INT-TRACEABILITY`, `INT-LIMITS`, `FU-CANDIDATE-PIN`, `FU-ISOLATION`, `FU-R3-IMMUTABILITY`, `FU-HISTORY-PRESERVATION`, `FU-R5-EXECUTION-SCOPE` — дословно в §PLAN record `r1` (source: возврат планера, переданный драйвером), здесь не перенабираются

## Original task

Исходная задача — коллекционная задача `open-issues-r5` по точной ссылке (здесь не перенабирается): `.pdca/status/collection-open-issues-r5.md` §Original task, где дословно записаны `pdca по открытым задачам из github`, `тогда делай автономно с автокомитом`, `после каждой выполненной задачи - делай комит и закрывай задачу`; критерии — §Input и §PLAN record `r1` того же файла (четыре verbatim `Done when` и утверждённое численное дополнение), а также item-записи `.pdca/status/issue-{2,8,3,4}-r5.md` (все четыре — `accepted`).

Решение пользователя, действующее в этом цикле — дословно:

> Признать build-артефакты допустимыми

Узкая авторизация цикла `N=5` (как передана в брифе):

1. документарная запись решения пользователя, переоценка и (только при `CHECK: all met`) финализация коллекционной записи;
2. единственная разрешённая команда — `status_lint.py` на статус-файлах;
3. запрещены: тесты, сборки, shell-листинги (`dir`/`ls`/`Get-ChildItem`/`git`), любой `python` сверх `status_lint.py`, правки продуктовых файлов, коммиты, `push`, обращения к GitHub и переоткрытие issues;
4. запрещено утверждать, что Demucs-тест с `0.00 s` повторно проверил паритет: паритет `#4` опирается только на принятые свидетельства уровня задачи;
5. `F-R4-EXECUTION` и `incomplete (STOP)` цикла `N=4` сохраняются в истории и не снимаются.

## PLAN record

### r1

```markdown
# PLAN Report — Cycle `cycle-open-issues-r5-int-followup-r5` (`N=5`, `r1`, `1/3`)

## 1. Форма, специализированный навык, идентификация цикла и область входа

- **Форма:** `cycle` (обычный документарный post-ACT follow-up цикл).
- **Специализированный навык / оверлей:** `none` (чистый `pdca`: продуктовый код не изменяется, сборки и тесты не запускаются).
- **Обоснование формы:** метод полностью определён (документарная переоценка замороженных свидетельств `N=3`/`N=4`, записи коллекции и принятых записей задач с учётом решения пользователя `«Признать build-артефакты допустимыми»`, после чего — только при `CHECK: all met` — append-only финализация записи коллекции на фазе `ACT` и проверка статус-файлов командой `status_lint.py`), объём ограничен одним новым статус-файлом и одним append-only блоком закрытия.
- **Идентификатор:** `cycle-open-issues-r5-int-followup-r5`, `N=5`, `r1`, `1/3`; открывается после `N=4` (`incomplete (STOP — F-R4-EXECUTION)`); не переоткрывает `N=3`, `N=4`, запись коллекции и записи задач.
- **Новый статус-файл:** `.pdca/status/cycle-open-issues-r5-int-followup-r5.md`; **корень свидетельств:** `.pdca/cycle-open-issues-r5-int-followup-r5/`.
- **Ссылочные отношения (pointer-only):** `N=3` и `N=4` — frozen; записи задач и их свидетельства паритета — frozen; запись коллекции неизменна на `PLAN`/`DO`/`CHECK`, получает append-only блок закрытия только на `ACT` после `CHECK: all met`.
- **Область входа:** полный объём документарного цикла — фиксация разъяснения по `target/`, переоценка интеграционных и преемственных критериев, append-only финализация коллекции при `all met`, lint.

## 2. Ограничение порядка фаз

1. `PLAN` — контракт результата и критерии, запись коллекции не меняется.
2. `DO` — запись в новый статус-файл дословного разъяснения пользователя, документарной сводки по 10 критериям и **проекта** append-only блока; запись коллекции не меняется, команды не запускаются.
3. `CHECK` — независимая оценка всех 10 критериев по замороженным свидетельствам, текущему append-only состоянию коллекции и записи `N=5`; проверка соблюдения ограничений выполнения; вердикт `all met` только при `10/10 met`, без waiver'ов.
4. `ACT` — при `all met`: append-only блок закрытия + обновление `outcome` в шапке коллекции, `## ACT record`, lint обоих файлов; иначе коллекция не меняется, `incomplete (STOP)` с остатком.

## 3. Задача, результат, метод, критерии

- Исходная задача — по ссылке (четыре `Done when` + addendum); item-записи приняты/закрыты.
- Авторизация `N=5`: (1) `«Признать build-артефакты допустимыми»` — временные `target/` в изолированном detached worktree допустимы в рамках «no source edits»; (2) документарный цикл с фиксацией, переоценкой/финализацией коллекции и `status_lint.py`; (3) запреты: тесты, сборки, shell-листинги, правки кода, коммиты, push, GitHub, переоткрытие issues; только `status_lint.py`; (4) запрещено утверждать, что `0.00 s` Demucs-тест перепроверил паритет — паритет `#4` опирается на принятые свидетельства уровня задачи; (5) `F-R4-EXECUTION` и `incomplete (STOP)` `N=4` сохраняются в истории.
- Requested == Delivered: запись `N=5` с решением пользователя и полной переоценкой; при `all met` — append-only финализированная запись коллекции, прошедшая lint.
- Метод: метрики из лога `N=3` (`exit code`, 25 секций `test result: ok.`, `144/0/4`), принятые метрики паритета задач, инварианты чистоты (`HEAD`/`write-tree`/`0/0/0/0`/43-43/7-7), счётчик выполнения в `N=5` (0 неразрешённых команд, `status_lint.py` exit 0). Знаменатели: 25 секций, 4 задачи, 43 файла, все замороженные записи; `target/` исключён решением пользователя. Пороги: `exit 0`, `0 failed`, `0/0/0/0`, 0 расхождений SHA-256, 0 неразрешённых команд.
- Критерии (11 строк; ни один `unverified`/`unmet` не прощается; закрытие требует `10/10`, где `FU-R5-EXECUTION-SCOPE` — десятый счётный):
  1. `INT-BUILD-TESTS` — `cargo test` на `d042c26` (из `N=3`): `exit 0`, 25 `ok.`, `144/0/4`; в `N=5` сборок/тестов нет.
  2. `INT-PARITY-AND-GATES` — по всем четырём задачам `Done when` + addendum выполнены по принятым свидетельствам + интеграционный прогон; **запрещено** заявлять, что `0.00 s` Demucs-тест перепроверил паритет `#4` (паритет — `item-4-demucs/logs/62-parity-after-ctr4.log`, `metrics/parity.{json,csv}`, `logs/66-gate-test-full.log`).
  3. `INT-HYGIENE` — `HEAD`/`write-tree`/`0/0/0/0`/43-43 drift 0 (по манифестам `N=3`).
  4. `INT-TRACEABILITY` — задача → коммит → запись → закрытый issue → интегральное свидетельство; amend только `3130f55`.
  5. `INT-LIMITS` — `target/` допустим по решению пользователя; остальные инварианты подтверждены; `0` правок исходников; worktree удалён; amend узкий; оговорки области покрытия зафиксированы.
  6. `FU-CANDIDATE-PIN` — `HEAD`/`write-tree` без дрейфа между `N=3`/`N=4`/`N=5`.
  7. `FU-ISOLATION` — продуктовое дерево не менялось ни в одном цикле.
  8. `FU-R3-IMMUTABILITY` — все замороженные записи побайтово неизменны.
  9. `FU-HISTORY-PRESERVATION` — история append-only; `F-R4-EXECUTION` и `incomplete (STOP)` `N=4` сохранены.
  10. `FU-R5-EXECUTION-SCOPE` — единственная разрешённая команда — `status_lint.py`; 0 тестов/сборок/листингов/правок/коммитов/push/GitHub; запись коллекции не изменена до `CHECK all met`.

## 4. Спецификация append-only блока закрытия (только при `CHECK: all met`)

- **Не меняется:** записи задач и их свидетельства; записи `N=1..N=4`; все существующие строки/секции записи коллекции.
- **Где:** (1) в шапке коллекции обновляется только строка итогового статуса — `outcome: done (all 4 issues integrated on d042c26; post-ACT follow-up N=5 accepted after user clarification on N=3 worktree build artifacts)`, `phase: ACT`; (2) в конец файла добавляется блок `### Post-ACT Follow-up Closure Record (Cycle N=5: cycle-open-issues-r5-int-followup-r5)`.
- **Содержание блока:** дословная цитата решения пользователя и узкой авторизации; итоговая матрица `10/10 met` (перечисление всех строк с основаниями); переносимые оговорки (Demucs `0.00 s` не перепроверял паритет — паритет по принятым свидетельствам `#4`; 4 ignored; нет GPU/произвольных входов/Python; `d042c26` неопубликован; `F-R4-EXECUTION` и `F-R4-NOTICE-BASIS` сохранены в истории); точные ссылки на `N=3`, `N=4`, `N=5`; финальный исход `done`; заявление, что история не переписана, коммитов/push/GitHub не было, issues не переоткрывались.

## 5. Рамки верификации

- `CHECK` читает только встроенными файловыми инструментами: записи `N=3`/`N=4` и их артефакты (`logs/cargo-test-d042c26.log`, `manifests/`), запись коллекции, записи задач и принятые артефакты паритета, запись `N=5`.
- Единственная разрешённая команда — `status_lint.py` на статус-файлах (`.pdca/status/cycle-open-issues-r5-int-followup-r5.md` и, при `all met`, `.pdca/status/collection-open-issues-r5.md`). Никакие иные команды (включая `cargo`, `git`, `dir`/`ls`/`Get-ChildItem`, произвольные Python/PowerShell, сетевые вызовы) не разрешены.

## 6. Ограничения, пробелы, минимальное решение

- Ограничения: только документарный цикл; только `status_lint.py`; запрет любых иных команд/правок/публикаций; сохранение `F-R4-EXECUTION`; закрытие только append-only и только в `ACT` после `CHECK all met`.
- Допущений нет. Пробелы: трактовка `target/` закрыта решением пользователя; процедурный конфликт `N=4` закрыт явным разрешением на `status_lint.py` и запретом иных команд.
- Минимальное решение: DO/CHECK только чтением; запись `N=5`; при `all met` — append-only блок и lint обоих файлов. Отвергнуто: закрытие коллекции до независимого CHECK.
- Трудоёмкость: один компактный цикл.

## 7. Handoff, риски, стоп-условия, терминальные исходы

- DO получает задачу и авторизации по ссылке; пишет только запись `N=5` и при необходимости артефакт в её корне; коллекцию не меняет; команды не запускает.
- **STOP** при: любом неразрешённом действии (тест/сборка/листинг/иная команда/правка продукта/commit/push/GitHub/issue-write); невыводимости критерия из существующих свидетельств; необходимости переписать историю или замороженные записи; расширении amend-исключения или решения по `target/`; заявлении о повторном паритете Demucs; ошибке lint, требующей действий за пределами текущего статус-файла/append-only блока.
- **Ветка А (`all met`):** ACT обновляет `outcome` коллекции на `done (...)`, добавляет append-only блок, пишет `## ACT record` в `N=5` (`phase: ACT`, `outcome: done`), запускает `status_lint.py` на обоих файлах, фиксирует `exit 0`.
- **Ветка Б (иначе):** коллекция не меняется; `N=5` → `incomplete (STOP)` с остатком и вопросом пользователю; lint только записи `N=5`.

## 8. Уверенность и пробелы

- Верифицировано на входе: контракт PLAN прочитан; авторизации `N=5` получены дословно; указатели на замороженные свидетельства определены.
- Допущений нет; независимая проверка содержимого и соблюдения `FU-R5-EXECUTION-SCOPE` выполняется в `CHECK` до любого изменения коллекции.
```

## Approval record

`not applicable: autonomous mode`

## DO result

DO завершён: создана эта статус-запись, зафиксировано дословно решение пользователя, собрана беззапусковая документарная сводка по десяти критериям (только указатели на доказательства, **без вердиктов**) и подготовлен, но **не применён** проект append-only блока закрытия коллекции. Коллекционная запись `.pdca/status/collection-open-issues-r5.md` в этом DO **не тронута**. Это результат DO, **не вердикт**: независимый `CHECK` этого цикла ещё не выполнялся, критерии не оценены.

Единицы выполнения DO: не делились — один документарный проход (`DO-1`), статус `в работе до CHECK`, критерии покрытия — все десять по указателям, зависимостей и замен нет.

### 1. Решение пользователя (verbatim)

> Признать build-артефакты допустимыми

### 2. `Notice` — слипы путей в плане и формулировка `INT-HYGIENE` (текст плана не изменён)

План записан дословно выше; ниже — расхождения, зафиксированные **без правки** плана и **без переноса** идентичностей из его путей.

**(a) Подтверждённое расхождение — объявленный «корень свидетельств».** План объявляет корень свидетельств `.pdca/cycle-open-issues-r5-int-followup-r5/`. Реальные корни, которыми оперирует эта запись: `.pdca/open-issues-r5/rv1/item-{2-mdx-onnx,8-ensembles,3-vr-uvr,4-demucs}/` (принятые item-доказательства паритета) и `.pdca/cycle-open-issues-r5-int-followup-r3/` (прогон `N=3` и его манифесты). Follow-up-запись `N=1` — это сама коллекционная запись `.pdca/status/collection-open-issues-r5.md`; отдельного файла `cycle-open-issues-r5-int-followup.md` не существует. Ни одна идентичность и ни один путь в этом цикле из объявленного планом корня не взяты; собственных артефактов под этим путём цикл не создаёт — единственный созданный файл цикла это статус-запись.

**(b) Неподтверждённое основание брифа — `unconfirmed`.** Бриф цикла сообщает о слипах вида `.pdca/cycle-open-issues-r5/item-…` и файла `cycle-open-issues-r5-int-followup.md`. Чтением дословного блока `r1` (выше) эти две конкретные строки **не подтверждаются**: в блоке плана нет ни пути `.pdca/cycle-open-issues-r5/item-…`, ни имени `cycle-open-issues-r5-int-followup.md` без суффикса `-r5` (встречаются только `item-4-demucs/logs/62-parity-after-ctr4.log`, `metrics/parity.{json,csv}`, `logs/66-gate-test-full.log` — относительные имена без префикса `.pdca/cycle-open-issues-r5/`). Основание этой части Notice'а **не подтверждено** и помечено `unconfirmed` (по образцу `F-R4-NOTICE-BASIS`); Notice не удаляется, план не переписывается. Подтверждённая часть — только (a).

**(c) `INT-HYGIENE`: числа `0/0/0/0` относятся к candidate worktree, не к основному дереву.** Формулировка плана «`HEAD`/`write-tree`/`0/0/0/0`/43-43 drift 0 (по манифестам `N=3`)» верна для detached candidate worktree прогона `N=3`: `.pdca/cycle-open-issues-r5-int-followup-r3/manifests/comparison.json` даёт `modified_tracked_count=0`, `staged_count=0`, `deleted_count=0`, `untracked_count=0` и в `pre_counts`, и в `post_counts`, плюс `status_unchanged=true`. Основное дерево `F:\sources\candle-roformer` как `0/0/0/0` **не** описывается: по замороженным снимкам в нём есть untracked `.pdca/status/*` (записи `-r1`, `-r2`, `-r3`, `-r4`, `collection-open-issues-r3.md`, `issue-4-r5.md`), untracked `.openidea/` и ` M .pdca/status/collection-open-issues-r5.md` — модификация, существовавшая до цикла `N=3` и не менявшаяся с его ACT (`.pdca/cycle-open-issues-r5-int-followup-r3/logs/frozen-identities-after-act.txt`, `.pdca/cycle-open-issues-r5-int-followup-r4/logs/frozen-record-hashes.txt:25`). Отслеживаемого изменения **продуктового** кода в основном дереве нет.

### 3. Документарная переоценка — сырые указатели по десяти критериям (вердикта нет)

1. **`INT-BUILD-TESTS`** — лог `N=3`: `.pdca/cycle-open-issues-r5-int-followup-r3/logs/cargo-test-d042c26.log` (по `manifests/`: `30312` B, `832` строки, `exit_code = 0`); прочитано напрямую: ровно **25** строк `test result: ok.` (строки 575, 590, 618, 625, 654, 662, 669, 687, 704, 718, 726, 734, 741, 748, 755, 763, 770, 777, 784, 792, 800, 808, 815, 824, 831 — все со статусом `ok.`, секций `failures:` нет), сумма по строкам `passed = 144` (42+8+22+1+22+0+1+12+11+8+1+1+1+1+1+1+1+1+1+1+1+1+1+3+1), `failed = 0`, `ignored = 4`; `Finished test profile` в `:528`; сводка — `manifests/log-summary.json`, `manifests/run-state-result.json`. В `N=5` ни одной сборки и ни одного теста (см. п. 6).
2. **`INT-PARITY-AND-GATES`** — только принятые item-доказательства (повторного измерения нет): `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/comparison.json`; `item-8-ensembles/metrics/parity.json` (+ `parity.csv`); `item-3-vr-uvr/logs/41-check-parity-after-reference-rerun.log`; `item-4-demucs/logs/62-parity-after-ctr4.log` — прочитано: `:197` `test demucs_four_stems_match_audio_separator ... ok`, `:199` `test result: ok. 1 passed; 0 failed; … finished in 12.29s`, `:201` `EXIT=0`; плюс `item-4-demucs/metrics/parity.json`, `item-4-demucs/metrics/parity.csv`, `item-4-demucs/logs/66-gate-test-full.log`. Оговорка обязательна: Demucs-тест прогона `N=3` завершился за `0.00 s` (`cargo-test-d042c26.log:625`; baseline `item-4-demucs/logs/66-gate-test-full.log:13` — `250.64s`) и **паритет не перепроверял**; паритет `#4` держится только на `62-parity-after-ctr4.log` / `metrics/parity.*` / `66-gate-test-full.log`.
3. **`INT-HYGIENE`** — `.pdca/cycle-open-issues-r5-int-followup-r3/manifests/pre/pre-worktree-state.txt`, `manifests/post/post-worktree-state.txt`, `manifests/comparison.json`: `pre_head = post_head = d042c2628edb52deab42118e18ac69974e5ad75d`, `pre_write_tree = post_write_tree = candidate_tree = 6cb258a028e0c3db51547a4a0b987fdb9e8e3e51`, `head_unchanged = write_tree_unchanged = true`, `pre_union_path_count = post_union_path_count = 43`, `pre_matched_path_hashes = post_matched_path_hashes = 43`, `sha256_drift_count = 0`, `paths_only_in_pre/post = []`, `status_unchanged = true`; `manifests/frozen-compare.json` (`file_count = 7`, `identical_count = 7`, `frozen_status_byte_drift = 0`). Факты основного дерева — §2 (c).
4. **`INT-TRACEABILITY`** — цепочка «задача → коммит → запись → закрытый issue → интегральное свидетельство» по коллекционной записи (§Item table, §Current state, FINAL COLLECTION STATE): `#2` → `59fb1fb` → `.pdca/status/issue-2-r5.md` (`completed`/`accepted`) → GitHub `#2` closed `2026-11-11T06:18:29Z`, comment `6788661305`; `#8` → `a38bb7f` → `issue-8-r5.md` (`completed`/`accepted`) → `2026-11-11T07:57:47Z`, comment `6790434030`; `#3` → `3130f55` → `issue-3-r5.md` (`completed`/`accepted`) → `2026-11-12T07:22:14Z`, comment `6817426243`; `#4` → `d042c26` → `issue-4-r5.md` (`completed`/`accepted`) → `2026-11-12T17:26:06Z`, comment `6830882059`. Amend-исключение — только `b15beb2 → 3130f55` (указатель `.pdca/open-issues-r5/rv1/integration/evidence/int-f03-no-push-reflog.txt:23–38`, подтверждён `N=4` §CHECK record, `FU-AMEND-EXCEPTION`).
5. **`INT-LIMITS`** — решение пользователя (§1) снимает единственное историческое условие `F-R3-CANDIDATE-WRITE` (размещение `target/` внутри изолированного detached worktree): `manifests/command-and-env.txt:22–24` (`CARGO_TARGET_DIR` unset → target по умолчанию `<worktree>/target`, `no writes to the main checkout`); остальные инварианты — `manifests/frozen-compare.json` (7/7), `N=4/logs/frozen-record-hashes.txt` (`staged_entries = 0`), amend узкий (п. 4), worktree `N=3` удалён (`manifests/worktree-remove.txt`: `exit_code = 0`, `exists_after = False`), правок исходников `0` (43/43, drift `0`), коммитов/`push`/GitHub в follow-up циклах `0`. Оговорки области покрытия — п. 6 и §ACT record `N=4`.
6. **`FU-CANDIDATE-PIN`** — те же `HEAD`/`write-tree` в `N=3` (`manifests/pre/pre-worktree-state.txt:3–6`, `manifests/post/post-worktree-state.txt`, `manifests/comparison.json`), в `N=4` (§DO result 2 той записи) и в `N=5` (значения перенесены из замороженного снимка `N=4/logs/frozen-record-hashes.txt:19–22`, повторная git-инспекция не выполнялась — `git` вне разрешённого набора команд).
7. **`FU-ISOLATION`** — `manifests/pre|post/per-path-hashes.csv`, `manifests/pre|post/union-paths.txt`, `manifests/comparison.json` (`43/43`, `sha256_drift_count = 0`); `N=4` §CHECK record (`FU-NO-MUTATION-NO-PUBLISH` — `met`, с оговоркой «не полный аудит скрытых действий»); в `N=5` правок продуктовых файлов `0`.
8. **`FU-R3-IMMUTABILITY`** — `.pdca/cycle-open-issues-r5-int-followup-r3/logs/frozen-identities-after-act.txt` (`2026-11-13T06:21:08Z`) против `.pdca/cycle-open-issues-r5-int-followup-r4/logs/frozen-record-hashes.txt:7–16`: 7/7 замороженных записей `identical` (`collection-open-issues-r5.md` `323295` B / sha256 `c2f02e495e6e9ae300c7a609038df8961b03664e49a3238191bcc6c4e5a0f063` / blob `0e70f7b…`; `-r2` `69803` B / `2219f469…`; `-r3` `57862` B / `b65d88a8…`; `issue-2` `510f5ab8…`; `issue-8` `dae960b0…`; `issue-3` `1420cba8…`; `issue-4` `d4b554b3…`), плюс `-r1` `48215` B / `c8544323…` `identical`. Последнее измерение — `N=4`, `2026-11-13T06:36:54Z`. В `N=5` хэширование **не выполнялось** (вне разрешённого набора команд): байт-идентичность подтверждается только этим снимком и тем, что ни один цикл после него не писал эти файлы; коллекционная запись этим DO не тронута.
9. **`FU-HISTORY-PRESERVATION`** — `.pdca/status/cycle-open-issues-r5-int-followup-r4.md` §CHECK record (`not accepted`, 9 `met` / 1 `unmet`) и §ACT record (`outcome: incomplete (STOP — F-R4-EXECUTION)`, §Defect history `F-R4-EXECUTION` — `blocking`/`confirmed`, `F-R4-NOTICE-BASIS` — `minor`/`confirmed`, оба сохранены); §Current state `N=4` и §Progress log `N=4` не изменяются; эта запись только добавляет новый файл, историю не переписывает.
10. **`FU-R5-EXECUTION-SCOPE`** — счётчик этого цикла: `commands_run = 1`, единственная команда — `python .pdca/scripts/status_lint.py .pdca/status/cycle-open-issues-r5-int-followup-r5.md`; её полный stdout и `exit` зафиксированы в последней строке §Progress log. Тестов/сборок/shell-листингов/правок продукта/коммитов/`push`/обращений к GitHub — `0`; коллекционная запись не изменена (п. 4).

### 4. Текущее состояние коллекционной записи (не изменена)

`.pdca/status/collection-open-issues-r5.md` — **не тронута** этим DO: `phase: ACT`, `record: completed`, `outcome: incomplete (STOP)`; последнее измерение идентичности — `N=4`, `2026-11-13T06:36:54Z`: `323295` B, sha256 `c2f02e495e6e9ae300c7a609038df8961b03664e49a3238191bcc6c4e5a0f063`, blob `0e70f7b774a89a4cde8d5080cc9e97a1f66f61f5` (`.pdca/cycle-open-issues-r5-int-followup-r4/logs/frozen-record-hashes.txt:7`). В `N=5` хэш не пересчитывался (вне разрешённых команд) — значение приведено как замороженный указатель, а не как новое измерение.

### 5. Проект append-only блока закрытия — `draft, not applied; ACT only after CHECK all met`

Ничто из этого блока **не записано** в коллекционную запись и не будет записано до `CHECK: all met`. Строки статусов намеренно не заполнены: вердикт выносит `CHECK`, а не `DO`.

````markdown
### Post-ACT Follow-up Closure Record (Cycle N=5: cycle-open-issues-r5-int-followup-r5)

- **Решение пользователя (verbatim):** «Признать build-артефакты допустимыми» — временные артефакты сборки Cargo (`target/`) в изолированном detached candidate worktree, созданные единственным запуском `cargo test` цикла `N=3`, признаны допустимыми в рамках авторизации «без правок исходников»; переоценка выполнена без повторного запуска.
- **Узкая авторизация `N=5`:** документарная запись решения, переоценка и финализация коллекции, `status_lint.py`; запрещены тесты, сборки, shell-листинги, правки кода, коммиты, `push`, GitHub и переоткрытие issues.
- **Матрица 10/10:** `INT-BUILD-TESTS`, `INT-PARITY-AND-GATES`, `INT-HYGIENE`, `INT-TRACEABILITY`, `INT-LIMITS`, `FU-CANDIDATE-PIN`, `FU-ISOLATION`, `FU-R3-IMMUTABILITY`, `FU-HISTORY-PRESERVATION`, `FU-R5-EXECUTION-SCOPE` — статус каждой строки и её основание заполняются в `ACT` дословно из отчёта `CHECK` (в `DO` не заполняются: вердикта нет).
- **Переносимые оговорки:** Demucs-тест `0.00 s` в `N=3` паритет `#4` не перепроверял (паритет — по принятым свидетельствам `item-4-demucs/logs/62-parity-after-ctr4.log`, `metrics/parity.{json,csv}`, `logs/66-gate-test-full.log`); 4 `ignored`-теста не исполнялись; покрытия GPU/произвольных входов/Python-converter нет; `d042c26` неопубликован относительно `origin/main = 8b03cfd`; `F-R4-EXECUTION` и `F-R4-NOTICE-BASIS` сохранены в истории и не снимаются.
- **Ссылки:** `N=3` — `.pdca/status/cycle-open-issues-r5-int-followup-r3.md`, `.pdca/cycle-open-issues-r5-int-followup-r3/`; `N=4` — `.pdca/status/cycle-open-issues-r5-int-followup-r4.md`, `.pdca/cycle-open-issues-r5-int-followup-r4/`; `N=5` — `.pdca/status/cycle-open-issues-r5-int-followup-r5.md`; item-доказательства — `.pdca/open-issues-r5/rv1/item-{2-mdx-onnx,8-ensembles,3-vr-uvr,4-demucs}/`.
- **Итоговый исход коллекции:** `done (all 4 issues integrated on d042c26; post-ACT follow-up N=5 accepted after user clarification on N=3 worktree build artifacts)`.
- **Заявление:** история не переписана; коммитов, `push` и обращений к GitHub не было; issues не переоткрывались; записи `N=1..N=4`, item-записи и их доказательства остаются байт-идентичными.
````

Дополнительно, если `CHECK: all met`, в шапке коллекции обновляется **только** строка итогового статуса на `phase: ACT` / `outcome: done (all 4 issues integrated on d042c26; post-ACT follow-up N=5 accepted after user clarification on N=3 worktree build artifacts)` — остальные строки шапки и вся история остаются как есть.

### 6. Учёт исполнения и ограничений этого DO

- `commands_run = 1` — ровно один запуск: `python .pdca/scripts/status_lint.py .pdca/status/cycle-open-issues-r5-int-followup-r5.md` (единственный файл, единственный запуск; повтор не выполнялся). Полный stdout и `exit` — строка `lint` в §Progress log; строки журнала `read-back` и `lint` добавлены **после** запуска (append-only plain bullets), поэтому линт покрывал байты записи без этих двух финальных строк журнала.
- Не выполнялось: ни одного теста, сборки, `cargo`-команды, shell-листинга, `git`, произвольного `python`, сетевого вызова; ни одной правки продуктового файла; ни одного коммита, `push`, обращения к GitHub; ни одного нового worktree; ни одной записи в коллекционную запись или в замороженные записи. Вся инспекция выполнена файловыми инструментами чтения (чтение файлов, поиск по содержимому, чтение каталогов).
- Время записи строк журнала помечено `unknown`: часы недоступны без запуска команды, а разрешена единственная команда — `status_lint.py` (её stdout времени не содержит).
- Ограничение честной атрибуции: байт-идентичность замороженных записей и основного дерева в этом цикле **не перемерялась** (хэширование вне разрешённого набора команд); используются последние замороженные измерения `N=4` (`2026-11-13T06:36:54Z`). Это ограничение, а не подтверждение.

### 7. Evidence pointers

- собственных новых артефактов-файлов, кроме этой записи, цикл не создаёт: все доказательства — замороженные указатели из п. 3; линт — единственная команда, её вывод в §Progress log.
- запись цикла: `.pdca/status/cycle-open-issues-r5-int-followup-r5.md`; план — §PLAN record `r1` (read-back: §Progress log).
- замороженные корни: `.pdca/cycle-open-issues-r5-int-followup-r3/`, `.pdca/cycle-open-issues-r5-int-followup-r4/`, `.pdca/open-issues-r5/rv1/`.

### Documentary evidence round (CHECK-routed)

Выполнен после `CHECK`, по его маршрутизации: только сбор **уже существующих** свидетельств файловыми инструментами чтения (чтение файлов, чтение каталогов, поиск по содержимому). Ни одной команды в этом проходе не запускалось — ни `status_lint.py`, ни `git`, ни Python, ни листингов через shell. Вердикта нет.

**Что искалось:** (i) независимый журнал операций цикла `N=5` — полный перечень выполненных действий за пределами собственной записи цикла; (ii) независимый снимок непрерывности/идентичности, покрывающий интервал после последнего измерения (`N=4`, `2026-11-13T06:36:54Z`): текущие `HEAD`/`write-tree`, состояние отслеживаемых файлов, байт-идентичность замороженных записей.

**Что прочитано (пути):**

- корень `.pdca/`: каталога `.pdca/cycle-open-issues-r5-int-followup-r5/` (объявленный планом «корень свидетельств») **не существует**; присутствуют только `cycle-open-issues-r5-int-followup-r2/`, `-r3/`, `-r4/`, `open-issues-r5-int-followup-r1/`, `open-issues-r5/`, `issue-13/`, `open-issues-r3/`, `open-issues-r4/`, `scripts/`, `status/` и `tmp-session-audit.py` — ни одного артефакта цикла `N=5`, кроме самой этой записи, нет;
- `.pdca/status/` (22 записи): файла журнала/аудита операций нет; `.pdca/status/cycle-open-issues-r5-int-followup-r5.md` — единственный файл этого цикла;
- поиск по содержимому `.pdca/**` по `int-followup-r5`: 22 совпадения, **все** — внутри `.pdca/status/cycle-open-issues-r5-int-followup-r5.md`; ни один другой файл на `N=5` не ссылается;
- поиск по содержимому `.pdca/**` по `commands_run`, `журнал вызовов`, `operation log`: 4 совпадения, **все** — в этой записи (строки 8, 139, 165, 205);
- поиск `status_lint` в `.pdca/cycle-open-issues-r5-int-followup-r4/`: 1 совпадение — `logs/lint-do.txt:1` (линт цикла `N=4` над записью `N=4`), это не `N=5`;
- `.pdca/cycle-open-issues-r5-int-followup-r4/logs/` — 3 файла (`frozen-record-hashes.txt`, `lint-do.txt`, `plan-readback.txt`); `logs/frozen-record-hashes.txt` прочитан целиком (30 строк): единственный снимок идентичностей — `2026-11-13T06:36:54Z` (DO-этап `N=4`), после него снимков нет;
- `.pdca/status/cycle-open-issues-r5-int-followup-r4.md` §Progress log (строки `344–356`): `CHECK` и `ACT` цикла `N=4` (включая четыре служебных вызова и `lint` `06:42:00Z`) выполнены **после** единственного снимка и **без нового снимка**; там же записано, что финальный линт этой записи не запускался;
- `.pdca/cycle-open-issues-r5-int-followup-r3/logs/` — 9 файлов; последний снимок `frozen-identities-after-act.txt`, `2026-11-13T06:21:08Z`;
- `.pdca/open-issues-r5/rv1/integration/evidence/` — 12 файлов: item-интегральные свидетельства закрытия issues; снимка непрерывности текущего интервала среди них нет;
- `.pdca/scripts/` — 1 файл (`status_lint.py`): журнала запусков нет;
- `.pdca/tmp-session-audit.py` — черновой read-only скрипт чтения базы сессий OpenCode (`~/.local/share/opencode/opencode.db`); журналом операций `N=5` не является, а его вывод потребовал бы запуска Python — вне авторизации цикла.

**Что найдено:** ничего независимого.

- **Независимого журнала операций `N=5` не существует.** Строка §Progress log `:231` этой записи (линт → `status-lint: OK`, `exit 0`) остаётся **самоотчётом**: ни один артефакт вне этой записи запуск не фиксирует, поэтому полнота перечня действий `N=5`, отсутствие иных команд и неизменность коллекции независимо не подтверждены — это ровно то, что `CHECK` обозначил как `F-R5-EXECUTION-EVIDENCE`.
- **Снимка непрерывности после `N=4` не существует.** Последние измерения идентичности — `N=3`, `2026-11-13T06:21:08Z` (`.pdca/cycle-open-issues-r5-int-followup-r3/logs/frozen-identities-after-act.txt`) и `N=4`, `2026-11-13T06:36:54Z` (`.pdca/cycle-open-issues-r5-int-followup-r4/logs/frozen-record-hashes.txt:7–22`). Интервал «`CHECK` `N=4` → `ACT` `N=4` → `PLAN`/`DO` `N=5`» не покрыт ни одним снимком: `HEAD`, `write-tree`, состояние отслеживаемых файлов и байт-идентичность замороженных записей на текущий момент **не наблюдались**.

**Прямое заключение (без смягчений):** **независимого журнала операций `N=5` и снимка непрерывности после `N=4` не существует**; все утверждения этой записи о единственном запуске и о неизменности — самоотчёт, а свежий снимок потребовал бы команды вне авторизации этого цикла (`git rev-parse HEAD`, `git status --porcelain`, `git write-tree`, хэширование файлов замороженных записей). Из отсутствия свидетельств **не** выводится отсутствие мутаций: ни мутация, ни её отсутствие этим раундом не установлены.

**Ограничение раунда:** собственных артефактов он не создаёт (команд нет, записывать нечего), поэтому в объявленном корне `.pdca/cycle-open-issues-r5-int-followup-r5/` ничего не появляется, а единственный изменённый файл цикла — эта запись. Раунд не заменяет измерение и не закрывает ни одной строки матрицы.

## CHECK record

Отчёт независимого `CHECK` записан дословно, как возвращён драйвером (полный оригинал на языке автора; не переписан, не сокращён, не переведён). Служебная пометка записи: источник — возврат `pdca-check` для цикла `N=5 · r=1 · n=1/3`, переданный в финальном диспетче; текст отчёта ниже не изменён.

## Вердикт: `not accepted`

**`10/10 met` не установлено. ACT пока не вправе выполнять append-only закрытие коллекции.** Остаток — подтверждение инвариантов именно до CHECK `N=5`, а не содержательные результаты исторического прогона.

Счётчики прочитаны из записи: **`N=5 · r=1 · n=1/3`**. `commands_run=1` записан, но независимо полный перечень действий не подтверждён.

Обозначения: `E3` / `E4` — `.pdca/cycle-open-issues-r5-int-followup-r3/` / `-r4/`; `S4` / `S5` — соответствующие файлы `.pdca/status/cycle-open-issues-r5-int-followup-r{4,5}.md`; `C` — `.pdca/status/collection-open-issues-r5.md`; `I` — `.pdca/open-issues-r5/rv1/`.

### Матрица

| Критерий | Статус | Основание и доказательства |
|---|---|---|
| `INT-BUILD-TESTS` | `met` | Исторический `cargo test` на зафиксированном кандидате: exit `0`, завершённая сборка, **25** итогов `ok.`, **144 passed / 0 failed / 4 ignored**. Прочитаны `E3/logs/cargo-test-d042c26.log:528–831`, `manifests/command-and-env.txt:9–14`. Отсутствие новых запусков оценивается отдельно ниже. |
| `INT-PARITY-AND-GATES` | `met` | Четыре принятые item-записи и их исторические измерения согласуются с порогами: `I/item-2-mdx-onnx/logs/comparison.json`; `item-8-ensembles/metrics/parity.json`; `item-3-vr-uvr/logs/41-check-parity-after-reference-rerun.log:118–125`; `item-4-demucs/logs/62-parity-after-ctr4.log:189–201`, `metrics/parity.{json,csv}`, `logs/66-gate-test-full.log`. Для `#4`: **8/8**, худшие `max_abs=8.583e-6`, RMSE `1.774e-7`, минимальный SDR `84.36 dB`. **Demucs `0.00 s` в N=3 — не повторное измерение паритета.** |
| `INT-HYGIENE` | `met` | Для **candidate worktree N=3**: неизменные HEAD/tree, `0/0/0/0`, **43/43**, drift `0`; `E3/manifests/comparison.json:2–38`. Основное дерево буквально чистым не объявляется: прежняя модификация `C` отражена в `E4/logs/frozen-record-hashes.txt:25`. |
| `INT-TRACEABILITY` | `met` | `#2→59fb1fb`, `#8→a38bb7f`, `#3→3130f55`, `#4→d042c26`; принятые item-записи, сохранённые закрытия четырёх issues и интеграционное свидетельство связаны в `C:167–211,314–321`. Единственный amend — `b15beb2→3130f55`, непосредственно в `I/integration/evidence/int-f03-no-push-reflog.txt:32–37`. Свежего GitHub-наблюдения нет и не требуется. |
| `INT-LIMITS` | `unverified` | Пользователь снимает **только условие размещения `target/`**. Исторически отдельно подтверждены отсутствие дрейфа исходников, frozen-сравнение и удаление worktree: `E3/manifests/{comparison.json,frozen-compare.json,worktree-remove.txt}`. Однако сохранность остальных инвариантов до текущего CHECK N=5 независимо не подтверждена; старый снимок не закрывает этот интервал. |
| `FU-CANDIDATE-PIN` | `unverified` | N=3 и снимок N=4 согласованы: HEAD `d042c2628edb52deab42118e18ac69974e5ad75d`, tree `6cb258a028e0c3db51547a4a0b987fdb9e8e3e51`. `E3/manifests/comparison.json`; `E4/logs/frozen-record-hashes.txt:19–22`. Для N=5 значения **перенесены**, а не независимо наблюдены (`S5:135,216`). |
| `FU-ISOLATION` | `unverified` | N=3: **43/43**, drift `0`; N=4 имеет сохранённый CHECK с ограниченной оценкой отсутствия мутаций. Отсутствие продуктовых изменений в N=5 представлено заявлениями автора (`S5:136,166,211`), без независимого журнала операций или текущего снимка. Нарушение не установлено; выполнение полного критерия тоже не установлено. |
| `FU-R3-IMMUTABILITY` | `unverified` | Снимки `E3/logs/frozen-identities-after-act.txt` и `E4/logs/frozen-record-hashes.txt:7–16` согласованы. **Последнее измерение — N=4, 06:36:54Z.** `S5:137,168` прямо признаёт отсутствие измерения в N=5. Это подтверждает прошлое состояние, не текущую побайтовую неизменность всех записей. |
| `FU-HISTORY-PRESERVATION` | `unverified` | Содержательно нарушение N=4 сохранено: `S4:202–212,271–279,321–322` содержит `F-R4-EXECUTION`, четыре служебных вызова и `incomplete (STOP)`. У `C:16–21,183–211` прежний STOP остаётся. Но требуемая **побайтовая неизменность C до ACT** не подтверждена актуальным сравнением; `323295 B / c2f02e49… / 0e70f7b…` — исторический снимок. |
| `FU-R5-EXECUTION-SCOPE` | `unverified` | `S5:231` сохраняет команду `python .pdca/scripts/status_lint.py .pdca/status/cycle-open-issues-r5-int-followup-r5.md`, stdout `status-lint: OK`, exit `0`. Это запись автора о разрешённом запуске, **не независимое подтверждение полноты всех действий**, отсутствия других команд и неизменности коллекции. Нужен существующий журнал вызовов/записей N=5. |

### Findings

- **`F-R5-CONTINUITY-EVIDENCE` — blocking, `confirmed` как пробел доказательств.** Для `INT-LIMITS`, `FU-CANDIDATE-PIN`, `FU-ISOLATION`, `FU-R3-IMMUTABILITY`, `FU-HISTORY-PRESERVATION` исторические снимки перенесены через непроверенный интервал. Основание: `S5:135–143,168`. **Фактическая мутация не установлена.**
- **`F-R5-EXECUTION-EVIDENCE` — blocking, `unverified`.** Точное соблюдение бюджета и запретов N=5 пока опирается на самоотчёт `S5:165–166,231`, а не независимую запись действий.
- **`F-R4-EXECUTION` — historical blocking, `confirmed`, сохранён.** Новая авторизация линта не отменяет нарушения и STOP N=4.
- **Утверждение «N=3 повторно измерил Demucs-паритет» — `refuted`.** Оно не принимается; основание паритета `#4` — исключительно принятые issue-level артефакты, перечисленные в матрице.

### Критерии и покрытие

Десять критериев покрывают узкую документарную переоценку; пользовательские пороги не ослаблены. Ошибочный корень свидетельств не использован вместо реальных артефактов. Главный недостаток — **доказательственный**, не установленный дефект продукта и не основание менять критерии.

Предусмотренные планом будущие линты в ACT **не разрешают второй запуск**: по текущему брифу единственный бюджет уже использован.

### Маршрутизация

**→ DO: один документарный сбор уже существующих свидетельств через драйвер**, без новых команд:
- независимая выгрузка существующего журнала операций N=5, включая разрешённый линт и записи файлов;
- имеющееся независимое свидетельство непрерывности HEAD/tree, продуктовых файлов и frozen-записей до CHECK.

Это не запрос нового хэширования, git-инспекции или повторного линта. Если таких свидетельств нет, сохраняются `unverified` и предусмотренный STOP; **закрывать коллекцию нельзя**.

### Пределы и уверенность

Исторические результаты логов и сохранение формулировок STOP проверены непосредственно. Текущая побайтовая идентичность, полнота действий N=5 и отсутствие скрытых операций не установлены. Четыре ignored-теста не выполнялись; нового GPU, произвольного входного или Python-converter покрытия нет; свежая удалённая проверка не проводилась.

**Уверенность высокая** в исторических результатах и наличии доказательственного остатка; недостаточная для `all met`.

## ACT record

- outcome: `incomplete (STOP — F-R5-CONTINUITY-EVIDENCE / F-R5-EXECUTION-EVIDENCE)`
- record: `completed` · phase: `ACT` · cycle: `N=5` · revision: `r=1` · attempt: `n=1/3` (вступивший `CHECK → ACT` терминальный исход: `not accepted`, попыток больше не тратится)
- `commands_run = 2` — оба запуска разрешённый линт, без повтора внутри прохода:
  1. `DO`: `python .pdca/scripts/status_lint.py .pdca/status/cycle-open-issues-r5-int-followup-r5.md` → stdout `status-lint: OK (.pdca\status\cycle-open-issues-r5-int-followup-r5.md: 0 tables, 0 rows)`, `exit 0` (§Progress log, строка `DO … линт статус-записи`);
  2. `ACT` (этот финальный проход): та же команда на том же единственном файле → stdout/`exit` — §Progress log, строка `ACT … ACT lint`; значение фиксируется запуском, не предположением. Коллекционная запись в линт не включалась: она не изменялась и была провалидирована ранее.
- **Что установлено свидетельствами:** четыре строки матрицы — `met`: `INT-BUILD-TESTS`, `INT-PARITY-AND-GATES`, `INT-HYGIENE`, `INT-TRACEABILITY` (исторический `cargo test` на `d042c26`: `exit 0`, 25 × `test result: ok.`, `144 passed / 0 failed / 4 ignored`; пороги паритета по принятым item-свидетельствам; candidate-worktree `0/0/0/0`, 43/43, drift 0; цепочка задача→коммит→запись→issue→интегральное свидетельство). Результат Demucs `0.00 s` прямо **не** является измерением паритета, а паритет `#4` опирается исключительно на принятые issue-level свидетельства (`item-4-demucs/logs/62-parity-after-ctr4.log`, `metrics/parity.{json,csv}`, `logs/66-gate-test-full.log`).
- **Что блокирует закрытие:** шесть строк `unverified` — `INT-LIMITS`, `FU-CANDIDATE-PIN`, `FU-ISOLATION`, `FU-R3-IMMUTABILITY`, `FU-HISTORY-PRESERVATION`, `FU-R5-EXECUTION-SCOPE`. Исторические снимки не доказывают непрерывность через ненаблюдавшийся интервал, а авторизация этого цикла не разрешает ни одной команды, способной дать свежий снимок; документарный раунд (§DO result `Documentary evidence round (CHECK-routed)`) установил, что независимого журнала операций `N=5` и снимка непрерывности после `N=4` **не существует**. Итог матрицы: **4 `met` / 0 `unmet` / 6 `unverified`**. **Мутация не установлена**: пробел доказательственный, а не установленный дефект продукта, и он не является основанием менять критерии.
- **Коллекция:** append-only закрытие **не выполнено**. `.pdca/status/collection-open-issues-r5.md` остаётся байт-идентичной и сохраняет свою историю `incomplete (STOP)`; записанный переносимый снимок `323295` B / sha256 `c2f02e49…` / blob `0e70f7b…` (измерение `N=4`, `2026-11-13T06:36:54Z`) приведён именно как переносимый указатель, а не как новое измерение. Проект блока закрытия (§DO result 5) остаётся `draft, not applied`.
- **Сохранено:** все четыре item-принятия, коммиты и закрытия (`59fb1fb`, `a38bb7f`, `3130f55`, `d042c26`); записи `N=3`/`N=4`, включая `F-R4-EXECUTION` и `incomplete (STOP)` цикла `N=4`; никаких правок исходников, коммитов, `push` или обращений к GitHub; ни один issue не переоткрыт; история не переписана.
- **Handoff / следующее авторизованное действие:** требуется **новое явное решение пользователя** — либо разрешить одну read-only команду-снимок в момент закрытия (`git rev-parse HEAD`, `git status --porcelain`, `git write-tree`, хэширование файлов замороженных записей), чтобы строки непрерывности можно было установить, либо поручить планеру решить, достаточны ли переносимые снимки как свидетельство для этих критериев непрерывности. Здесь ни то, ни другое не выполняется; незакрытая работа остаётся незакрытой (`FU-*` — `unverified`, коллекция — не закрыта).

## Lessons learned

Служебная пометка записи: ниже — отчёт `pdca-check` в режиме `retrospective`, записанный **дословно** (полный оригинал, язык автора). Сохранение выполнено в этом ACT-проходе: единственный писатель `pdca-executor` записал отчёт в эту секцию, после чего ACT переведён в `completed` — то есть **persistence теперь выполнен**, а сигнал самого отчёта («сохранение и завершение ACT этим не подтверждаются») относится к моменту его собственного возврата, не к этой записи.

### Ретроспектива N=5 · r1 · n=1/3

1. **Категория: планирование доказательств.** Исторический снимок не подтверждает непрерывность состояния до нового цикла. План требовал актуальных инвариантов, хотя авторизация разрешала только линт. В дальнейшем на PLAN следует явно сопоставлять каждому требованию доступное свидетельство и отмечать непокрытый интервал, не предполагая неизменность.
   **Основание:** статус N=5, строки 65–71, 135–143, 168; переданное резюме CHECK — `F-R5-CONTINUITY-EVIDENCE`.

2. **Категория: границы подтверждения.** Полезно разделены разрешение пользователя на build-артефакты, исторические тестовые результаты и доказательство паритета. Это разрешение не подтверждает остальные инварианты; успешный быстрый Demucs-тест не заменяет измерение паритета. В дальнейшем сохранять источник и область действия каждого подтверждения.
   **Основание:** §Original task, строки 19–29; §DO result, строки 130–134.

3. **Категория: безопасная финализация.** Проект закрытия оставлен неприменённым до независимого CHECK — полезное разделение подготовки и окончательной записи. При этом `status_lint.py: exit 0` свидетельствует только о проверке охваченной версии статус-файла, а не об отсутствии иных действий или изменений.
   **Основание:** строки 145–168, 230–231; переданное резюме CHECK — `F-R5-EXECUTION-EVIDENCE`.

**Ограничения и неопределённость.** Прочитана существующая запись N=5; новых проверок и команд не выполнялось. В ней CHECK ещё обозначен `not run`, ACT — `pending`. Полный отчёт CHECK и терминальный payload не предоставлены: их содержание доступно только в резюме диспетчера. Пробел непрерывности не является доказательством мутации. Неизменность коллекционной записи в N=5 независимо здесь не устанавливалась.

**Сигнал:** ретроспективный анализ завершён; сохранение отчёта и завершение ACT этим не подтверждаются.

## Revision history

`none`

## Defect history

`none at start` — на входе цикла `N=5` новых дефектов не зафиксировано; ниже по одной строке на каждый из десяти критериев с сырыми указателями и **без вердикта**. Исторические ключи `F-R4-EXECUTION`, `F-R4-NOTICE-BASIS`, `F-R3-CANDIDATE-WRITE`, `F-R3-NOTICE-MISMATCH` переносятся только по ссылке и не переоткрываются.

- `INT-BUILD-TESTS` | observed: `N=5` `r1`, attempt `n=1/3`, DO-этап | fixes applied: `0` | evidence: `.pdca/cycle-open-issues-r5-int-followup-r3/logs/cargo-test-d042c26.log` (25 × `test result: ok.`, `144/0/4`, `exit_code=0`), `manifests/log-summary.json`, `manifests/run-state-result.json` | last recurrence/escalation: вердикта в `N=5` нет; в `N=5` тестов/сборок `0`
- `INT-PARITY-AND-GATES` | observed: `N=5` `r1`, DO-этап | fixes applied: `0` | evidence: `.pdca/open-issues-r5/rv1/item-2-mdx-onnx/logs/comparison.json`; `item-8-ensembles/metrics/parity.json`; `item-3-vr-uvr/logs/41-check-parity-after-reference-rerun.log`; `item-4-demucs/logs/62-parity-after-ctr4.log:197,199,201`; `item-4-demucs/metrics/parity.json`, `metrics/parity.csv`; `item-4-demucs/logs/66-gate-test-full.log` | last recurrence/escalation: вердикта нет; оговорка `0.00 s ≠ повторный паритет` (`cargo-test-d042c26.log:625` против `66-gate-test-full.log:13` `250.64s`) зафиксирована в §Original task, §PLAN record и §DO result
- `INT-HYGIENE` | observed: `N=5` `r1`, DO-этап | fixes applied: `0` | evidence: `.pdca/cycle-open-issues-r5-int-followup-r3/manifests/pre/pre-worktree-state.txt`, `manifests/post/post-worktree-state.txt`, `manifests/comparison.json`, `manifests/frozen-compare.json`; факты основного дерева — `N=4/logs/frozen-record-hashes.txt:25` и §DO result §2 (c) | last recurrence/escalation: вердикта нет; Notice (c) фиксирует, что `0/0/0/0` — про candidate worktree
- `INT-TRACEABILITY` | observed: `N=5` `r1`, DO-этап | fixes applied: `0` | evidence: `.pdca/status/collection-open-issues-r5.md` §Item table / §Current state (коммиты `59fb1fb`, `a38bb7f`, `3130f55`, `d042c26`); `.pdca/status/issue-{2,8,3,4}-r5.md`; `.pdca/open-issues-r5/rv1/integration/evidence/int-f03-no-push-reflog.txt:23–38` | last recurrence/escalation: вердикта нет; amend-исключение остаётся `b15beb2 → 3130f55`
- `INT-LIMITS` | observed: `N=5` `r1`, DO-этап | fixes applied: `0` | evidence: решение пользователя (§DO result 1); `.pdca/cycle-open-issues-r5-int-followup-r3/manifests/command-and-env.txt:22–24`; `manifests/frozen-compare.json`; `manifests/worktree-remove.txt`; `N=4/logs/frozen-record-hashes.txt` | last recurrence/escalation: `F-R3-CANDIDATE-WRITE` — исторический, снят решением пользователя, не переоткрывается
- `FU-CANDIDATE-PIN` | observed: `N=5` `r1`, DO-этап | fixes applied: `0` | evidence: `.pdca/cycle-open-issues-r5-int-followup-r3/manifests/comparison.json`; `N=4` §DO result 2; `N=4/logs/frozen-record-hashes.txt:19–22` | last recurrence/escalation: вердикта нет; повторная git-инспекция не выполнялась (вне разрешённых команд)
- `FU-ISOLATION` | observed: `N=5` `r1`, DO-этап | fixes applied: `0` | evidence: `manifests/pre|post/per-path-hashes.csv`, `manifests/pre|post/union-paths.txt`, `manifests/comparison.json` (43/43, drift 0); `N=4` §CHECK record (`FU-NO-MUTATION-NO-PUBLISH` `met`) | last recurrence/escalation: вердикта нет; правок продукта в `N=5` — `0`
- `FU-R3-IMMUTABILITY` | observed: `N=5` `r1`, DO-этап | fixes applied: `0` | evidence: `.pdca/cycle-open-issues-r5-int-followup-r3/logs/frozen-identities-after-act.txt`; `N=4/logs/frozen-record-hashes.txt:7–16` (7/7 `identical`, `-r1` `identical`) | last recurrence/escalation: вердикта нет; в `N=5` хэширование не выполнялось — только замороженный снимок `2026-11-13T06:36:54Z`
- `FU-HISTORY-PRESERVATION` | observed: `N=5` `r1`, DO-этап | fixes applied: `0` | evidence: `.pdca/status/cycle-open-issues-r5-int-followup-r4.md` §CHECK record, §ACT record (`incomplete (STOP — F-R4-EXECUTION)`), §Defect history (`F-R4-EXECUTION`, `F-R4-NOTICE-BASIS`) | last recurrence/escalation: вердикта нет; история `N=4` не изменяется этой записью
- `FU-R5-EXECUTION-SCOPE` | observed: `N=5` `r1`, attempt `n=1/3` | fixes applied: `0` | evidence: `commands_run = 1` — `python .pdca/scripts/status_lint.py .pdca/status/cycle-open-issues-r5-int-followup-r5.md` (stdout/exit — §Progress log, строка `lint`); `0` тестов/сборок/листингов/правок/коммитов/`push`/GitHub; коллекционная запись не изменена (§DO result 4) | last recurrence/escalation: вердикта нет; ключ закрывает процедурный конфликт `N=4` (`F-R4-EXECUTION`) явным разрешением линта
- carried (frozen, только по ссылке): `F-R4-EXECUTION`, `F-R4-NOTICE-BASIS` → `.pdca/status/cycle-open-issues-r5-int-followup-r4.md` §Defect history; `F-R3-CANDIDATE-WRITE`, `F-R3-NOTICE-MISMATCH` → `.pdca/status/cycle-open-issues-r5-int-followup-r3.md` §Defect history
- `F-R5-CONTINUITY-EVIDENCE` | observed: `N=5` `r1`, attempt `n=1/3`, `CHECK` | fixes applied: `0` (новых мер не применялось; команды-снимки вне авторизации) | evidence: §CHECK record (findings, матрица: `INT-LIMITS`, `FU-CANDIDATE-PIN`, `FU-ISOLATION`, `FU-R3-IMMUTABILITY`, `FU-HISTORY-PRESERVATION`); §DO result `Documentary evidence round (CHECK-routed)` — независимого журнала `N=5` и снимка непрерывности после `N=4` не существует | last recurrence/escalation: `blocking`/`confirmed` **как пробел доказательств**; фактическая мутация не установлена; исход — `incomplete (STOP)`, закрытие коллекции не выполнено; снятие — только новым явным решением пользователя (разрешение read-only снимка) или решением планера о достаточности переносимых снимков
- `F-R5-EXECUTION-EVIDENCE` | observed: `N=5` `r1`, attempt `n=1/3`, `CHECK` | fixes applied: `0` | evidence: §CHECK record (`FU-R5-EXECUTION-SCOPE` `unverified`); §DO result `Documentary evidence round (CHECK-routed)` — упоминания `commands_run` и `status_lint` существуют только внутри этой записи | last recurrence/escalation: `blocking`/`unverified`; самоотчёт §Progress log `:231` не заменён независимой записью действий; новых попыток не добавлено

## Changed files

- `.pdca/status/cycle-open-issues-r5-int-followup-r5.md` → создан в `DO` и изменён в `ACT` (единственный созданный/изменённый файл цикла): `ACT` дописал §DO result `Documentary evidence round (CHECK-routed)`, заменил §CHECK record, §ACT record и §Lessons learned, обновил §Current state, §Defect history (append), §Changed files, §Git state и §Progress log (append)
- продуктовые (отслеживаемые) файлы: `0` изменений; замороженные записи (`collection-open-issues-r5.md`, `-r1`, `-r2`, `-r3`, `-r4`, `issue-{2,8,3,4}-r5.md`): `0` изменений — этот `ACT` их не открывал на запись; коллекционная запись **не тронута**, её `323295` B / sha256 `c2f02e49…` / blob `0e70f7b…` названы как переносимый снимок `N=4` (`2026-11-13T06:36:54Z`), а не как новое измерение

## Git state

- status path: `.pdca/status/cycle-open-issues-r5-int-followup-r5.md` — created and modified this cycle (`DO` + `ACT`) · untracked (durable on disk, not committed; `untracked` перенесено из `DO`-записи — git-инспекция в цикле не выполнялась, `git` вне разрешённого набора команд)
- baseline, supplied once by the cycle: worktree `F:\sources\candle-roformer`, branch `main`, `HEAD = d042c2628edb52deab42118e18ac69974e5ad75d` (`d042c26`), `HEAD^{tree} = 6cb258a028e0c3db51547a4a0b987fdb9e8e3e51`, `origin/main = 8b03cfda15a485b115f8b694d07c03e98fcdf685` (`8b03cfd`); amend-исключение `b15beb2 → 3130f55`; индекс пуст (`0` staged) — значения перенесены из замороженных снимков (`N=4/logs/frozen-record-hashes.txt:19–22`), повторная git-инспекция в этом цикле не выполнялась (`git` вне разрешённого набора команд)
- pre-existing worktree state at cycle start (не вызвано этим циклом): ` M .pdca/status/collection-open-issues-r5.md` (HEAD blob `b26ab9e…` vs worktree blob `0e70f7b…`, 846+/7−; не менялся с ACT `N=3`), untracked `.pdca/status/*` (`-r1`, `-r2`, `-r3`, `-r4`, `collection-open-issues-r3.md`, `issue-4-r5.md`) и `.openidea/` — источник: `N=3/logs/frozen-identities-after-act.txt`, `N=4/logs/frozen-record-hashes.txt:25`
- same-cycle touched-file ledger: только `.pdca/status/cycle-open-issues-r5-int-followup-r5.md` (untracked) — путь, который цикл знает за собой; коллекционная запись в ledger не входит (её ` M` — прежнее состояние, не дельта цикла, и она оставлена байт-идентичной); task changes pending commit: `0`; ничего не добавлялось в индекс, не стагировалось и не коммитилось; коммитов/`push`/GitHub — `0`
- отслеживаемого изменения продуктового кода в основном дереве нет; чужие незакоммиченные изменения не присваиваются и не трогаются; сведений о мутациях после `N=4` нет, но их отсутствие этим циклом не измерено (§ACT record, `unverified`)

## Progress log

- `unknown` | `PLAN` | revision r1 | attempt n=1/3 | plan ready: планер вернул полный план цикла `N=5` (form `cycle`, overlay `none`), записан дословно в §PLAN record `r1`; read-back: блок перечитан и сверен с переданным возвратом построчно — `0` расхождений; количество строк записи указано в строке read-back ниже; байты/sha256 не вычислялись — хэширование вне разрешённого набора команд (`status_lint.py` только) | §PLAN record `r1`
- `unknown` | `PLAN` | revision r1 | attempt n=1/3 | `Notice: план объявляет корень свидетельств .pdca/cycle-open-issues-r5-int-followup-r5/, которого нет среди реальных корней (item-* и N=3); реальные корни — .pdca/open-issues-r5/rv1/item-{2-mdx-onnx,8-ensembles,3-vr-uvr,4-demucs}/ и .pdca/cycle-open-issues-r5-int-followup-r3/; follow-up-запись N=1 — сама коллекционная запись; текст плана не изменён` | §DO result, Notice (a)
- `unknown` | `PLAN` | revision r1 | attempt n=1/3 | `Notice: основание брифа о слипах .pdca/cycle-open-issues-r5/item-… и cycle-open-issues-r5-int-followup.md в дословном блоке r1 не подтверждено — помечено unconfirmed (по образцу F-R4-NOTICE-BASIS); Notice не удаляется, план не переписывается` | §DO result, Notice (b)
- `unknown` | `PLAN` | revision r1 | attempt n=1/3 | `Notice: INT-HYGIENE 0/0/0/0 относится к detached candidate worktree N=3 (manifests/comparison.json); основное дерево имеет untracked .pdca/status/* и .openidea/ и M-модификацию коллекционной записи, существовавшую до N=3; отслеживаемого изменения продуктового кода нет` | §DO result, Notice (c)
- `unknown` | `DO` | revision r1 | attempt n=1/3 | `DO started` (документарный беззапусковой проход; только файловые инструменты чтения) | `.pdca/status/cycle-open-issues-r5-int-followup-r5.md`
- `unknown` | `DO` | revision r1 | attempt n=1/3 | время записи строк журнала неизвестно: часы недоступны без запуска команды, разрешён только `status_lint.py`, его stdout времени не содержит; поэтому все строки этого цикла помечены `unknown` | §DO result 6
- `unknown` | `DO` | revision r1 | attempt n=1/3 | документарная сводка по десяти критериям записана только указателями (без вердиктов); проект append-only блока закрытия подготовлен и **не применён** (`draft, not applied`); коллекционная запись не изменена (`323295` B, sha256 `c2f02e49…` — замороженный указатель `N=4`) | §DO result 3–5
- `unknown` | `DO` | revision r1 | attempt n=1/3 | read-back записи: `229` строк до этих двух финальных строк журнала (проверено чтением файла); блок §PLAN record `r1` перечитан построчно и совпадает с переданным возвратом планера — `0` расхождений, текст плана не изменён; байты/sha256 не вычислялись — хэширование вне разрешённого набора команд (`status_lint.py` только) | `.pdca/status/cycle-open-issues-r5-int-followup-r5.md`
- `unknown` | `DO` | revision r1 | attempt n=1/3 | линт статус-записи: `python .pdca/scripts/status_lint.py .pdca/status/cycle-open-issues-r5-int-followup-r5.md` → stdout `status-lint: OK (.pdca\status\cycle-open-issues-r5-int-followup-r5.md: 0 tables, 0 rows)`; `exit 0`; ровно один запуск, один файл, без повтора; линт покрыл байты записи без этих двух финальных plain-bullet строк | `.pdca/scripts/status_lint.py` (исходник пакета `pdca-collection/scripts/status_lint.py`: совпадают число строк `578` и хвост; побайтовая сверка не выполнялась — хэширование вне разрешённых команд)
- `unknown` | `ACT` | revision r1 | attempt n=1/3 | документарный раунд по маршрутизации `CHECK` (только существующие свидетельства, файловые инструменты чтения, ни одной команды): каталога `.pdca/cycle-open-issues-r5-int-followup-r5/` нет; упоминания `int-followup-r5`/`commands_run`/`status_lint` для `N=5` — только внутри этой записи; последние снимки идентичности — `N=3` `06:21:08Z` и `N=4` `06:36:54Z`; вывод — независимого журнала операций `N=5` и снимка непрерывности после `N=4` **не существует** | §DO result `Documentary evidence round (CHECK-routed)`
- `unknown` | `ACT` | revision r1 | attempt n=1/3 | `CHECK` записан дословно: вердикт `not accepted`, матрица `4 met` / `0 unmet` / `6 unverified`, findings `F-R5-CONTINUITY-EVIDENCE` (blocking, confirmed как пробел доказательств), `F-R5-EXECUTION-EVIDENCE` (blocking, unverified), `F-R4-EXECUTION` сохранён, утверждение о повторном Demucs-паритете `refuted` | §CHECK record
- `unknown` | `ACT` | revision r1 | attempt n=1/3 | ретроспектива `pdca-check` (`retrospective` mode) записана дословно; сохранение выполнено в этом проходе (`persistence` выполнен), `record` переведён в `completed` | §Lessons learned
- `unknown` | `ACT` | revision r1 | attempt n=1/3 | `ACT closed`: `outcome: incomplete (STOP — F-R5-CONTINUITY-EVIDENCE / F-R5-EXECUTION-EVIDENCE)`, `commands_run = 2`; append-only закрытие коллекции **не выполнено** — `.pdca/status/collection-open-issues-r5.md` оставлена байт-идентичной (`323295` B / sha256 `c2f02e49…` / blob `0e70f7b…` — переносимый снимок `N=4`, не новое измерение); мутация не установлена; следующий авторизованный шаг — новое явное решение пользователя (read-only снимок при закрытии либо решение планера о достаточности переносимых снимков) | §ACT record, §Git state
- `unknown` | `ACT` | revision r1 | attempt n=1/3 | `ACT lint`: `python .pdca/scripts/status_lint.py .pdca/status/cycle-open-issues-r5-int-followup-r5.md` → stdout `status-lint: OK (.pdca\status\cycle-open-issues-r5-int-followup-r5.md: 1 tables, 10 rows)`; `exit 0`; второй и последний разрешённый запуск цикла (первый — в `DO`), без повтора; коллекционная запись не линтовалась (не изменялась); линт покрыл байты записи без этих пяти финальных append-only строк журнала | `.pdca/scripts/status_lint.py`
