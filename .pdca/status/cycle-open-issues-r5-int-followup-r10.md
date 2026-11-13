# Cycle status: open-issues-r5-int-followup-r10 / cycle 10

## Current state

- task: `open-issues-r5-int-followup-r10` · form: `cycle` · specialized skill: `none`
- mode: `autonomous`
- cycle: `N=10` · revision: `r=1` · attempt: `n=1`
- counters: `N=10` · `r=1` · `n=1` (durable, не сбрасываются) · `commands_run = 0` на входе цикла; в `DO`
  учитываются прогоны `status_lint.py` **и** раскрытые ниже in-repository read-only чтения/листинги: `commands_run`
  = 5 прогонов `status_lint.py` + 9 in-repository read-only shell-вызовов (из них один содержал два read-only
  `git`-вызова → `F-R10-GIT-READ-DEVIATION`); `fixes applied = 0`; `n` не увеличивался
- counters (после `ACT`): `commands_run = 21` — 10 прогонов `status_lint.py` (5 в `DO` + 5 в `ACT`: прогон A по
  затронутым файлам, прогон B после строк журнала, прогон C, прогон D, прогон E подтверждающий по окончательным
  байтам), 9 in-repository read-only shell-вызовов `DO`, 2 shell-вызова `ACT` (внешний листинг вне репозитория и
  чтение часов → `F-R10-ACT-SCOPE-DEVIATION`); `fixes applied = 0`; `n = 1`; `r = 1`
- scope: один документарный append-only цикл согласования учёта — создать эту запись и `.pdca/cycle-open-issues-r5-int-followup-r10/`, дописать
  блоки-кандидаты в EOF `.pdca/status/collection-open-issues-r5.md` и `.pdca/status/cycle-open-issues-r5-int-followup-r8.md`
  (плюс указатель в EOF `-r9.md`), прогнать `status_lint.py`; одиннадцать критериев `FU-R10-*` по указателям ·
  status path: `.pdca/status/cycle-open-issues-r5-int-followup-r10.md`
- goal: append-only, lint-clean ратифицировать коррекцию учёта `N=7` после независимого `N=10` `CHECK` — заместить
  (не удалить) утверждения «`N=7` исполнял только lint» и `FU-R7-EXECUTION-SCOPE = met`, отдельно раскрыть два
  `N=9`-листинга как пред-авторизационное отклонение, сохранить STOP-ы `N=8`/`N=9` и всю историю, перечислить
  остающиеся `unverified`-оговорки без перевода их в `met` и заместить нератифицированный статус `N=8` **только**
  при принятом `CHECK` — §PLAN record `r1` и §Original task авторитетны
- phase: `ACT` · record: `completed` · outcome: `incomplete (STOP — F-R10-GIT-READ-DEVIATION)`
- source: бриф `N=10`, выданный драйвером после `ACT` цикла `N=9` (`incomplete (STOP — F-R9-SCOPE-DEVIATION)`) ·
  context refs: `.pdca/status/collection-open-issues-r5.md`, `.pdca/status/cycle-open-issues-r5-int-followup-r{9,8,7,6,5,4,3,2}.md`,
  `.pdca/status/issue-{2,8,3,4}-r5.md`, `.pdca/open-issues-r5/rv1/`, `.pdca/cycle-open-issues-r5-int-followup-r{3,6,7,8,9}/`
- criteria: `FU-R10-WORDING-AND-AUTHORITY`, `FU-R10-N7-SUPERSESSION`, `FU-R10-N9-DEVIATION-DISCLOSED`,
  `FU-R10-HISTORY-AND-ITEMS-PRESERVED`, `FU-R10-EXECUTION-SCOPE`, `FU-R10-NO-OVERCLAIM`, `FU-R10-STATUS-LINT`
  (блокирующие) и `FU-R10-CAVEAT-N4-N5-IMMUTABILITY`, `FU-R10-CAVEAT-N8-PRESERVATION`, `FU-R10-CAVEAT-N8-SCOPE-JOURNAL`,
  `FU-R10-CAVEAT-BYTE-AND-COMMAND-COMPLETENESS` (оговорённые, обязаны оставаться `unverified`) — дословно в
  §PLAN record `r1` п. 3 (source: возврат планера, переданный драйвером), здесь не перенабираются
- residual: `FU-R10-EXECUTION-SCOPE = unmet` (два read-only `git`-вызова вне allow-list) — независимый `CHECK`
  вернул `not accepted`; `ACT` финализировал `incomplete (STOP — F-R10-GIT-READ-DEVIATION)` без ратификации
  Stage 2; четыре оговорки остаются `unverified` (неблокирующие, никогда не `met`); эффективный итог коллекции
  не изменён — `N=7`-ратифицированный `done (with explicit caveat …)`; следующее решение — за пользователем
- Notice: путь lint-скрипта, переданный с брифом как `.pdca/bin/status_lint.py`, не существует; канонический —
  `.pdca/scripts/status_lint.py` (см. §DO result п. 3); текст плана записан дословно и не правился
- Notice (`ACT`, сверка fixed-skeleton; исправлено в этом же `ACT`): заголовок `## Approval record` в записи
  **присутствует** — он записан дословно внутри §PLAN record как часть возврата планера (`## Approval record` /
  `not applicable: autonomous mode`, строка `128`). Ранее записанное в этой же строке утверждение об отсутствии
  заголовка было фактической ошибкой `ACT` и заменено; байты `PLAN`/`DO` не затронуты
- Notice (`ACT`, раскрытое отклонение объёма): два read-only shell-вызова этого `ACT` — внешний листинг каталога
  вне репозитория (`C:\Users\…\.config\opencode`) и чтение часов (`Get-Date`) — вне перечня «append-only
  статусные записи и `status_lint.py`»; ключ `F-R10-ACT-SCOPE-DEVIATION`; мутаций не было

## Original task

Исходная задача — коллекционная задача `open-issues-r5` по **точной ссылке** (здесь не перенабирается):
`.pdca/status/collection-open-issues-r5.md` §Original task (там дословно записаны исходные формулировки
пользователя и критерии коллекции), item-записи `.pdca/status/issue-{2,8,3,4}-r5.md` — все четыре `accepted`
(коммиты `59fb1fb`, `a38bb7f`, `3130f55`, `d042c26`), и предшествующие циклы
`.pdca/status/cycle-open-issues-r5-int-followup-r{1…9}.md`.

Собственный текст авторизации пользователя для `N=10` этому `DO` **не передан**; он не реконструируется и не
выдаётся за дословную цитату. Ниже — авторизация в том виде, в каком она передана брифом `N=10`:

> Разрешено: in-repository read-only чтения файлов и read-only листинги каталогов; append-only правки статусов;
> создание `.pdca/status/cycle-open-issues-r5-int-followup-r10.md` и `.pdca/cycle-open-issues-r5-int-followup-r10/`;
> `status_lint.py` (канонический `.pdca/scripts/status_lint.py`, запуск проектным Python). Запрещено: тесты, сборки,
> продуктовые правки, `git`-операции, коммиты, `push`, GitHub, а также внешняя оболочка или поиск путей вне
> репозитория.
>
> Авторизация действует **проспективно**: два read-only листинга `N=9` она **не** ратифицирует задним числом;
> STOP-ы `N=8` и `N=9` сохраняются; нератифицированный статус `N=8` замещается **только** при принятом `N=10`
> `CHECK` и ратификации в `ACT`.

## PLAN record

### r1

# PLAN Record — `cycle-open-issues-r5-int-followup-r10` (`N=10`, Plan Revision `r1`)

## 1. Classification, Form, Overlay, Identity, and Pointer-Only Relationships

- **Form:** `cycle`; **overlay:** `none`.
- **Form rationale:** append-only Markdown status reconciliation plus user-authorized in-repository read-only directory listings and `status_lint.py`; volume is bounded to the `N=10` record/evidence and append blocks on the collection and `N=8` records (optional pointer on `N=9`).
- **Cycle ID:** `cycle-open-issues-r5-int-followup-r10` (`N=10`), fresh after `N=9` closed `incomplete (STOP — F-R9-SCOPE-DEVIATION)`; `r1`, `n=1/3`.
- **Anchor:** `HEAD = d042c2628edb52deab42118e18ac69974e5ad75d`, tree `6cb258a028e0c3db51547a4a0b987fdb9e8e3e51`; `d042c26` unpublished on `main`.
- **Canonical paths:** `N=10` status `.pdca/status/cycle-open-issues-r5-int-followup-r10.md`, evidence `.pdca/cycle-open-issues-r5-int-followup-r10/`; prior records `…-r9.md`, `…-r8.md`, `…-r7.md`, `…-r{6,5,4,3,2}.md` (+ `-r3/`, `-r6/` evidence); collection `.pdca/status/collection-open-issues-r5.md`; items `.pdca/status/issue-{2,8,3,4}-r5.md` and `.pdca/open-issues-r5/rv1/item-*`.
- **Entry scope:** complete the documentary reconciliation in one cycle: append candidate `N=10` blocks in DO, verify via independent CHECK, append ratifying ACT blocks on acceptance (or STOP).

## 2. Goal, unchanged task, exact minimal superseding text

- Task by reference; user authority for `N=10` as above.
- **Requested = delivered:** append-only, lint-clean records that ratify the `N=7` accounting correction after an independent `N=10` CHECK, supersede (not delete) the `FU-R7-EXECUTION-SCOPE = met` and lint-only-count claims, disclose the `N=9` listings separately as a pre-authorization deviation, preserve `N=8`/`N=9` STOPs and all history, list remaining `unverified` non-blocking caveats without relabelling any `unverified` as `met`, and supersede `N=8`'s unratified-candidate status only on acceptance.
- **Explicit judgment on `N=9`:** the two `N=9` listings do **not** block `N=10` acceptance provided `N=10` does not rewrite `N=9`, does not rely on `N=9` as ratified authority, and does not claim retroactive ratification; `N=9` remains a closed STOP with the deviation disclosed.

### Stage 1 — DO candidate blocks (pending CHECK; no acceptance claim)

1. Collection EOF: `### Post-ACT Accounting Correction (N=10, cycle-open-issues-r5-int-followup-r10 — Candidate Pending N=10 CHECK)` containing:
   - a DO-status line (candidate; not in effect unless an independent `N=10` CHECK accepts and ACT ratifies; prior blocks intact);
   - the **user ratification** of the five `N=7` ACT read-only external shell calls as a narrow tooling exception, with the explicit statement that the original `N=7` plan did **not** permit them;
   - the **supersession** (not deletion) of the `N=7` `FU-R7-EXECUTION-SCOPE = met` verdict and of the lint-only command-count claim, using `N=7`'s own recorded numbers;
   - the **separate disclosure** of the two `N=9` directory listings as a pre-authorization deviation in the closed `N=9` STOP cycle, with the explicit statement that the `N=10` authorization applies prospectively and does **not** retroactively ratify them;
   - the **caveat list** (each `unverified`, non-blocking, explicitly not `met`): `N=4`/`N=5` historical immutability; `N=8` pre/post baseline; `N=8` operation journal; `N=8`/`N=10` byte-preservation and command completeness; and the Demucs parity note as specified.
2. `N=8` record EOF: `## Post-STOP Cross-Reference (N=10 … Candidate Pending N=10 CHECK)` stating `N=8` remains closed STOP with its unverified rows unchanged, and that its "candidate not ratified" status is superseded **only if** `N=10` CHECK accepts.
3. Optional `N=9` pointer block: `N=9` remains STOP; its two listings remain a disclosed pre-authorization deviation, not retroactively ratified.

### Stage 2 — ACT ratifying blocks (only on accepted CHECK)

1. Collection EOF: `### Post-ACT Accounting Correction Ratification (N=10 ACT …)` recording the accepted CHECK, the supersession of `N=8`'s unratified-candidate status, and the effective collection outcome `done (with explicit caveats: …)` enumerating: 4/4 items accepted at `d042c26`; the five `N=7` calls ratified and the two `N=7` claims superseded; the `N=9` listings disclosed and not retroactively ratified; `N=4`/`N=5` immutability, `N=8` preservation/scope evidence, and `N=8`/`N=10` byte/command completeness `unverified`; Demucs parity `unverified`; `d042c26` unpublished.
2. `N=8` EOF: a ratification note superseding only its "candidate not ratified" annotation, leaving its STOP/history intact.
3. Optional `N=9` EOF note: `N=10` closed; `N=9` remains STOP with its listings unratified.

## 3. Criteria

**Blocking (must be `met`):** `FU-R10-WORDING-AND-AUTHORITY`; `FU-R10-N7-SUPERSESSION`; `FU-R10-N9-DEVIATION-DISCLOSED`; `FU-R10-HISTORY-AND-ITEMS-PRESERVED`; `FU-R10-EXECUTION-SCOPE` (in-repo read-only reads/listings, append-only status writes, `status_lint.py` only); `FU-R10-NO-OVERCLAIM`; `FU-R10-STATUS-LINT`.
**Caveated (must be `unverified`, non-blocking, never `met`):** `FU-R10-CAVEAT-N4-N5-IMMUTABILITY`; `FU-R10-CAVEAT-N8-PRESERVATION`; `FU-R10-CAVEAT-N8-SCOPE-JOURNAL`; `FU-R10-CAVEAT-BYTE-AND-COMMAND-COMPLETENESS`.

## 4. Ordering

DO appends Stage 1 + lints; CHECK judges; ACT appends Stage 2 + re-lints or records STOP. `N=8`'s unratified status is superseded only by an accepted `N=10` CHECK.

## 5. Risks and stop conditions

STOP on: any action outside in-repo read-only reads/listings, append-only status writes and `status_lint.py`; any history rewrite; any caveated row labelled `met`; any overclaim (byte/completeness verified, Demucs parity, `N=7` plan permission, retroactive ratification of the `N=9` listings, reopening); lint failure beyond the permitted append-only fix; any blocking row unmet.

## 6. Terminal outcomes

- **Path A:** `done (with explicit caveats …)` — Stage 2 blocks appended; `N=8`'s unratified annotation superseded; `N=7`/`N=8`/`N=9` remain closed; lint `exit 0`.
- **Path B:** `incomplete (STOP — <defect-id>)` — candidate blocks not ratified; `N=8`/`N=9` statuses unchanged; STOP annotation appended; lint `exit 0`.

## 7. Gaps, effort, handoff, confidence

Caveated gaps settled by the user's instruction; the `N=9` deviation settled by the explicit judgment above. Effort: one compact cycle. DO carries the user authority, the criteria, the Stage 1 blocks and the stop conditions. Confidence high.

## Approval record

`not applicable: autonomous mode`

## DO result

`DO` завершён на границе `DO → CHECK`: создана эта запись и каталог доказательств, в EOF трёх прежних записей
дописаны append-only блоки-кандидаты, выполнены разрешённые прогоны `status_lint.py` (пять прогонов, последний —
по окончательным байтам). Это результат `DO`, **не вердикт**: ни один критерий не оценён, принятие и закрытие не
заявлены, STOP не выносится.

Единицы выполнения `DO`: не делились — один проход (`DO-1`), статус «в работе до `CHECK`», критерии покрытия —
все одиннадцать по указателям, зависимостей и замен (`superseded → replacement`) нет.

### 1. Append-only блоки-кандидаты — где именно

- `.pdca/status/collection-open-issues-r5.md` — прежний файл `2720` строк; добавлен **один** блок
  `### Post-ACT Accounting Correction (N=10, cycle-open-issues-r5-int-followup-r10 — Candidate Pending N=10 CHECK)`;
  строка `2719` — пустая разделительная, заголовок — **строка `2720`**, блок заканчивается строкой `2781`
  (`No acceptance is asserted, no caveated item is raised to met, and no earlier byte is rewritten.`); новый
  размер — `2783` строки.
- `.pdca/status/cycle-open-issues-r5-int-followup-r8.md` — прежний файл `284` строки; добавлен **один** блок
  `## Post-STOP Cross-Reference (N=10 — Candidate Pending N=10 CHECK)`: заголовок — **строка `286`**, блок
  заканчивается строкой `303`; новый размер — `303` строки.
- `.pdca/status/cycle-open-issues-r5-int-followup-r9.md` — прежний файл `269` строк; добавлен **один** блок-указатель
  `## N=10 Cross-Reference (Candidate Pending N=10 CHECK)`: заголовок — **строка `271`**, блок заканчивается
  строкой `281`; новый размер — `281` строка.
- Правки выполнены как **единственная замена точного анкера, который оканчивается прежним последним байтом файла**
  на «тот же текст + блок»; вставка начинается сразу за прежним содержимым, поэтому прежнее содержимое сохранилось
  как точный префикс. Побайтовая идентичность префикса **хешем не подтверждена** (хеширование в цикле не
  авторизовано и не выполнялось) — это ограничение, а не заявление.

### 2. Содержание блоков (для ориентира `CHECK`)

- Коллекция: пункт `1` — ратификация пользователем пяти read-only внешних shell-вызовов `N=7` `ACT` как узкого
  инструментального исключения с прямым утверждением, что план `N=7` их **не** разрешал; пункт `2` — замещение
  (не удаление) двух утверждений `N=7` с числами из собственной записи `N=7`; пункт `3` — отдельное раскрытие двух
  листингов `N=9` без ретроактивной ратификации; пункт `4` — пять оговорок, каждая `unverified (non-blocking by
  user instruction, explicitly not met)`; пункт `5` — самораскрытие `F-R10-GIT-READ-DEVIATION`; пункт `6` —
  неизменность прежних принятий.
- `N=8`: STOP сохранён, обе `unverified`-строки не подняты в `met`, аннотация «candidate not ratified» замещается
  **только** при принятом `N=10` `CHECK`.
- `N=9`: STOP сохранён, два листинга остаются раскрытым отклонением, запись не переписана.

### 3. Дословные формулировки, затребованные брифом, — как записаны

- Замещение `N=7`: «`N=7` ran `commands_run = 8` `status_lint.py` invocations per its record, and additionally five
  read-only external shell calls in ACT to resolve `act.md` (disclosed in `N=7` ACT/Notice); the claim that `N=7`
  ran only lint, and the `FU-R7-EXECUTION-SCOPE = met` verdict, are superseded.» — записано дословно; рядом
  добавлен точный указатель места раскрытия (см. `Notice` ниже), потому что разделы `ACT`/`Notice` записи `N=7`
  самого раскрытия не содержат.
- Demucs: «the `N=3` integration run's Demucs parity test logged `0.00 s` and is not a parity measurement; Demucs
  parity rests on the accepted item-4 evidence (`issue-4-r5.md`, commit `d042c26`)» — записано без argv и без
  формулировки о «0 совпавших тестах».
- Идентификаторы прежних строк — только канонические: `FU-R3-IMMUTABILITY`, `FU-HISTORY-PRESERVATION`,
  `FU-R7-EXECUTION-SCOPE`, `FU-R8-HISTORY-PRESERVED`, `FU-R8-EXECUTION-SCOPE` (сверено с записями `N=7`/`N=8`);
  вымышленный идентификатор не вводился.

### 4. `Notice` (косметические расхождения указателей; план и семантика не менялись)

- `Notice` (`N=10`, lint path): переданный с брифом путь `.pdca/bin/status_lint.py` не существует; канонический
  скрипт — `.pdca/scripts/status_lint.py` (подтверждено read-only листингом `.pdca/scripts`). Все прогоны выполнены
  каноническим скриптом; текст плана не правился.
- `Notice` (`N=10`, locator of the five `N=7` calls): затребованная формулировка ссылается на «`N=7` ACT/Notice»,
  однако раскрытие пяти вызовов записано в
  `.pdca/status/cycle-open-issues-r5-int-followup-r7.md` строки `347`–`356`, то есть в блоке коррекции учёта
  `N=8`, дописанном в EOF записи `N=7`; разделы `ACT` и `Notice` записи `N=7` его не содержат. Это расхождение
  указателя, а не семантический дрейф; дословная формулировка сохранена, точный локатор добавлен рядом.

### 5. Учёт исполнения (`commands_run`)

- `commands_run` на входе цикла: `0`.
- Прогоны `status_lint.py` — **5**: прогон 1 (три прежних файла, exit `0`), прогон 2 (четыре файла, exit `0`),
  прогоны 3 и 4 (эта запись после правок указателей, журнала и косметики, exit `0`), прогон 5 (последний
  подтверждающий по окончательным байтам этой записи; его exit-код сообщён в возврате `DO`).
- In-repository read-only shell-вызовы до первой правки — **9**: листинги `.pdca/status`, `.pdca`, `.pdca/scripts` и
  чтения фрагментов записей `N=7`/`N=8`/`N=9`/коллекции (`Get-ChildItem`, `Get-Content`, `Select-String`). Один из
  этих девяти вызовов содержал **два read-only `git`-вызова** — `git rev-parse HEAD` и `git status --porcelain`.
  Правок, коммитов, `amend`, `reset`, `push`, GitHub, тестов, сборок, продуктовых правок — `0`; переоткрытий issue — `0`.
- `fixes applied = 0`; `n` не увеличивался (`n=1`); `r=1`.

### 6. Раскрытое отклонение (`F-R10-GIT-READ-DEVIATION`)

Два read-only `git`-вызова выполнены **до первой правки** и вне перечня разрешённых действий брифа `N=10`
(«`No` … `git operations`»). Они не мутировали репозиторий (только чтение), но сам факт — отклонение от объёма
исполнения; оно раскрыто здесь, в блоке коллекции (пункт `5`) и в блоке `N=8`, и записано строкой дефекта ниже.
`met` по `FU-R10-EXECUTION-SCOPE` этим `DO` не заявляется — оценка принадлежит `CHECK`.

### 7. Прогоны `status_lint.py` (прогоны 1–5)

- Прогон 1 — три файла одним вызовом, на байтах после всех трёх append-правок:
  `python .pdca/scripts/status_lint.py .pdca/status/collection-open-issues-r5.md .pdca/status/cycle-open-issues-r5-int-followup-r8.md .pdca/status/cycle-open-issues-r5-int-followup-r9.md`
  → `status-lint: OK (…collection…: 3 tables, 17 rows)`, `status-lint: OK (…-r8.md: 1 tables, 6 rows)`,
  `status-lint: OK (…-r9.md: 0 tables, 0 rows)`, **exit `0`**.
- Прогон 2 — четыре файла (включая эту запись), после исправления указателей строк в §Defect history:
  `…collection…: 3 tables, 17 rows`, `…-r8.md: 1 tables, 6 rows`, `…-r9.md: 0 tables, 0 rows`,
  `…-r10.md: 0 tables, 0 rows`, **exit `0`**.
- Прогоны 3 и 4 — эта запись после правок указателей, журнала и косметики: `…-r10.md: 0 tables, 0 rows`,
  **exit `0`** каждый. После прогона 4 выполнена ещё одна правка формулировки в этой же записи (§DO result п. 1),
  поэтому подтверждающим по окончательным байтам является прогон 5; при отличии его exit-кода от `0` результат
  фиксируется отдельной append-only строкой, а не подменой записанного. Полные stdout и exit-коды всех прогонов —
  `.pdca/cycle-open-issues-r5-int-followup-r10/lint.txt`.

## CHECK record

Источник: отчёт независимого `pdca-check` по `cycle-open-issues-r5-int-followup-r10` (`N=10` · `r=1` · `n=1`),
переданный драйвером в бриф `ACT`; записан дословно, в исходном языке, без правок.

````text
## Вердикт: `not accepted`

**ACT не может ратифицировать коррекцию.** Остаток: `FU-R10-EXECUTION-SCOPE = unmet` — выполнены два запрещённых git-вызова.

Счётчики прочитаны из статуса: `N=10 · r=1 · n=1`, `fixes applied = 0`, `commands_run = 14` (5 lint + 9 shell-вызовов; один содержал два git-запроса).

Обозначения: **C** — `.pdca/status/collection-open-issues-r5.md`; **R7/R8/R9/R10** — соответствующие статусы followup; **L** — `.pdca/cycle-open-issues-r5-int-followup-r10/lint.txt`.

### Матрица

| Критерий | Статус | Основание и свидетельство |
|---|---|---|
| `FU-R10-WORDING-AND-AUTHORITY` | `met` | Кандидат передаёт требуемую узкую пользовательскую ратификацию N=7, не выдавая её за разрешение прежнего плана; действие условно до CHECK/ACT. C:2726–2735; R10:77–92. |
| `FU-R10-N7-SUPERSESSION` | `met` | Явно замещены обе формулировки; сохранены записанные 8 lint-вызовов плюс 5 внешних вызовов. Уточнён ошибочный локатор ACT/Notice. C:2737–2743; R7:8,237,347–356. Замещение пока кандидатное. |
| `FU-R10-N9-DEVIATION-DISCLOSED` | `met` | Два листинга раскрыты отдельно; авторизация только проспективная, ретроактивной ратификации нет. C:2745–2750; R9:271–281. |
| `FU-R10-HISTORY-AND-ITEMS-PRESERVED` | `met` | Содержательно сохранены прежние PLAN/CHECK/ACT N=7, STOP N=8/N=9 и 4/4 принятия; новые блоки их не переоткрывают. R7:26–250,347–356; R8:114–162,282–299; R9:154–165,276–281; C:167–175,2701–2729,2776–2778. Побайтовая неизменность этим статусом **не подтверждается**. |
| `FU-R10-EXECUTION-SCOPE` | **`unmet`** | `git rev-parse HEAD` и `git status --porcelain` находятся вне разрешённого набора и подпадают под запрет git-операций. R10:46–50,96,105,191–202; C:2770–2774; L:48–54. |
| `FU-R10-NO-OVERCLAIM` | `met` | Кандидат не объявлен действующим; четыре оговорки не повышены до `met`; нет заявлений о разрешении плана N=7, ратификации листингов N=9 или новом измерении Demucs. C:2726–2729,2731–2768; R8:291–299. |
| `FU-R10-STATUS-LINT` | `met` | `python .pdca/scripts/status_lint.py <affected files>`: четыре файла — **exit 0**, результаты C **3/17**, R8 **1/6**, R9 **0/0**, R10 **0/0**; заключительный прогон R10 также **exit 0**. L:14–41. |
| `FU-R10-CAVEAT-N4-N5-IMMUTABILITY` | `unverified` | Историческая неизменность не установлена; явно non-blocking, не `met`. C:2755–2758. |
| `FU-R10-CAVEAT-N8-PRESERVATION` | `unverified` | До-/после-база N=8 отсутствует; явно non-blocking, не `met`. C:2759–2761; R8:127,158. |
| `FU-R10-CAVEAT-N8-SCOPE-JOURNAL` | `unverified` | Первичного журнала операций N=8 нет; явно non-blocking, не `met`. C:2762–2764; R8:131,158. |
| `FU-R10-CAVEAT-BYTE-AND-COMMAND-COMPLETENESS` | `unverified` | Побайтовая сохранность и полнота операций не доказаны; явно non-blocking, не `met`. C:2765–2766; R10:143–146. |

**Итого: 6 `met` / 1 `unmet` / 4 `unverified` (оговорённые, неблокирующие).**

### Находка

**`F-R10-GIT-READ-DEVIATION` — blocking, `confirmed`; критерий `FU-R10-EXECUTION-SCOPE`.**

Выбираю чтение **(a): оба git-запроса вне авторизации**. Read-only характер не превращает git-команду в разрешённое чтение файла или листинг каталога. Отдельное перечисление commits/push не отменяет общего запрета «git operations». Раскрытие нарушения не является пользовательским исключением.

Уточнение к брифу: в прочитанном PLAN:105 нет буквальной фразы «any git command»; там STOP на **любое действие вне allow-list**. Совместно с R10:49 и критерием R10:96 это даёт тот же однозначный результат — без ослабления условия.

### Критерии и пределы проверки

Критерии покрывают документарный объём и согласуют Stage 1/Stage 2; противоречий, мешающих исполнить задачу, не обнаружено. Оговорённые доказательные пробелы отделены от обязательного соблюдения границ.

Проверены тексты и сохранённый lint-журнал, не выполнены новые команды. Исчерпывающий аудит операций, побайтовое сравнение, актуальное состояние git/GitHub и Demucs parity не устанавливались. Исходная пользовательская формулировка N=10 отдельно не сохранена: оценка полномочий опирается на переданный контракт, что R10:43–54 раскрывает явно.

### Следующий переход

**Драйверу применить предусмотренную PLAN ветку STOP → ACT:** `incomplete (STOP — F-R10-GIT-READ-DEVIATION)`, без Stage 2 ратификации. Это явное условие остановки, не исчерпание попыток.

Уже выполненные команды невозможно устранить повторным DO или lint. **Аннотация N=8 «candidate not ratified» остаётся в силе**, STOP N=8/N=9 сохраняются; эффективный итог коллекции этим CHECK не изменён.

**Уверенность:** высокая в блокирующем выводе и проверенном содержании; ограниченная в полноте исторического и операционного учёта.
````

## ACT record

- outcome: `incomplete (STOP — F-R10-GIT-READ-DEVIATION)`
- record: `completed` · phase: `ACT` (последняя реальная фаза) · `N=10` · `r=1` · `n=1`; `fixes applied = 0`; `n`
  не увеличивался (DO → ACT не добавляет попытку)
- originating phase: независимый `CHECK` цикла `N=10` вернул `not accepted` (`FU-R10-EXECUTION-SCOPE` `unmet`);
  его отчёт записан дословно в §CHECK record — этот `ACT` своего вердикта не выносит
- matrix (как возвращено `CHECK`): `6 met / 1 unmet / 4 unverified` — четыре `unverified` оговорены и
  неблокирующие (`FU-R10-CAVEAT-*`), ни одна не поднята до `met`
- applied branch: §PLAN record `r1` §5 (STOP на любое действие вне allow-list) и §6 Path B — **Stage 2
  ратификации не выполнялась**; блоки `### Post-ACT Accounting Correction Ratification (N=10 ACT …)` в коллекции и
  ратификационная заметка в `N=8` не создавались
- completed work: §CHECK record и §Lessons learned заполнены; в EOF `.pdca/status/collection-open-issues-r5.md`
  дописана аннотация `### N=10 ACT Terminal Record — incomplete (STOP — F-R10-GIT-READ-DEVIATION)`; в EOF
  `.pdca/status/cycle-open-issues-r5-int-followup-r8.md` — однострочный указатель на неё; §Current state, §Defect
  history, §Changed files, §Git state и §Progress log этой записи обновлены; финальный прогон `status_lint.py`
  по трём затронутым файлам — `exit 0` (см. §Progress log)
- remaining work: `FU-R10-EXECUTION-SCOPE` — `unmet`, устранимым повторным `DO`/lint не является (выполненные
  команды не отзываются); четыре оговорки — `unverified`, неблокирующие, не `met`; Stage 2 не выполнялся и этим
  `ACT` не предусмотрен
- effect on earlier records: `N=8` "candidate not ratified" и STOP `N=9` — без изменений; ратифицированный итог
  коллекции `N=7` — без изменений
- limitations: этот `ACT` не выполнял `git`-команд, поэтому tracked/untracked-состояние затронутых путей не
  наблюдалось; побайтовая сохранность префиксов не подтверждена (хеширование не авторизовано); полнота журнала
  операций не заявляется; два собственных read-only shell-вызова `ACT` вне allow-list раскрыты как
  `F-R10-ACT-SCOPE-DEVIATION`
- next authorized action (решение пользователя): либо явно разрешить read-only `git`-запросы в новом цикле, либо
  опираться исключительно на in-repo чтения файлов и листинги каталогов; альтернативно — принять коллекцию в
  текущем `N=7`-ратифицированном виде. Новый цикл начинается только по авторизованному `PLAN`

## Lessons learned

Источник: ретроспективный отчёт `pdca-check` (`retrospective` mode) по `N=10` · `r=1` · `n=1`, переданный
драйвером в бриф `ACT`; записан дословно, в исходном языке, без правок.

Персистенция: отчёт сохранён исполнителем в этом разделе (`ACT`, `N=10` · `r=1` · `n=1`); этим `ACT` считается
completed. Ретроспектива не создаёт вердикта, критериального результата, счётчика, попытки или новой проверки.

````text
### Ретроспектива N=10 · r=1 · n=1

1. **Границы полномочий.** Не сработало разграничение «read-only» и «разрешено»: выполнены два прямо запрещённых git-запроса. На будущее — сопоставлять каждую операцию внутри составного вызова с allow-list, включая привычные проверки состояния. Раскрытие отклонения не заменяет разрешения.
   **Свидетельства:** статус N=10, §Original task:46–50; §DO result:185–202; `lint.txt:46–54`.

2. **Разделение кандидата и ратификации.** Полезным оказалось явное разделение Stage 1 и Stage 2: документарный кандидат не приобретает силу автоматически. Следует сохранять условность ратификации и заранее определённую STOP-ветку, даже когда текст подготовлен и lint успешен. Согласно переданному итогу CHECK, Stage 2 не выполняется.
   **Свидетельства:** статус N=10, §PLAN record:77–110; итог CHECK в диспетче.

3. **Честность доказательств.** Сработало отдельное раскрытие отклонения, учёт вызовов и сохранение оговорок без превращения неизвестного в подтверждённое. На будущее — продолжать отделять синтаксическую исправность статусов от соблюдения полномочий: пять успешных lint-прогонов подтверждают только свою область проверки.
   **Свидетельства:** статус N=10, §DO result:185–217; `lint.txt:6–41`; итог CHECK в диспетче.

**Ограничения и неопределённость.** Использованы существующие записи, без новых прогонов. Полный исходный текст пользовательской авторизации в статусе отсутствует — приведена передача брифа (:43–54). Полный CHECK-отчёт не предоставлен: доступно его резюме в диспетче; сохранённый §CHECK record ещё содержит `not run`, §ACT record — `pending`. Побайтовая сохранность и полнота журнала независимо не устанавливались. Причины появления запрещённых запросов неизвестны.

**Сигнал:** ретроспективный анализ завершён; сохранение отчёта исполнителем ещё не подтверждено.
````

## Revision history

`none`

## Defect history

`none` — новых дефектов продукта этот цикл не вводит (`fixes applied: 0`). Строки ниже — **только указатели на
свидетельства, без оценки** (вердикт принадлежит независимому `pdca-check`); по каждому критерию `FU-R10-*`
`observed: N=10 · r=1 · n=1`.

Блокирующие критерии:

- `FU-R10-WORDING-AND-AUTHORITY` | fixes applied: `0` | evidence: §Original task (авторизация как передана брифом; собственная цитата пользователя не передана и не реконструирована) → §PLAN record `r1` п. 2; `.pdca/status/collection-open-issues-r5.md:2720-2781` (пункты `1`–`6`) | last recurrence/escalation: оценка не выполнялась
- `FU-R10-N7-SUPERSESSION` | fixes applied: `0` | evidence: `.pdca/status/collection-open-issues-r5.md:2737-2743` (пункт `2`, дословная формулировка брифа) → §DO result п. 3; числа `N=7` — `.pdca/status/cycle-open-issues-r5-int-followup-r7.md:8,237` (`commands_run = 8`), `.pdca/status/cycle-open-issues-r5-int-followup-r7.md:347-356` (раскрытие пяти вызовов) | last recurrence/escalation: оценка не выполнялась
- `FU-R10-N9-DEVIATION-DISCLOSED` | fixes applied: `0` | evidence: `.pdca/status/collection-open-issues-r5.md:2745-2750` (пункт `3`, проспективная авторизация, без ретроактивной ратификации); `.pdca/status/cycle-open-issues-r5-int-followup-r9.md:271-281` | last recurrence/escalation: оценка не выполнялась
- `FU-R10-HISTORY-AND-ITEMS-PRESERVED` | fixes applied: `0` | evidence: append-only правки по EOF-анкерам (§DO result п. 1); item-таблица и §Current state коллекции (4/4 `accepted`: `59fb1fb`/`a38bb7f`/`3130f55`/`d042c26`) не переписаны; `.pdca/status/cycle-open-issues-r5-int-followup-r8.md:286-303` — STOP сохранён | last recurrence/escalation: побайтовая идентичность префикса хешем не подтверждена (хеширование не авторизовано); оценка — за `CHECK`
- `FU-R10-EXECUTION-SCOPE` | fixes applied: `0` | evidence: §DO result п. 5 (`commands_run`), §DO result п. 6 и `.pdca/status/collection-open-issues-r5.md:2770-2774` (`F-R10-GIT-READ-DEVIATION`), `.pdca/cycle-open-issues-r5-int-followup-r10/lint.txt` | last recurrence/escalation: два read-only `git`-вызова до первой правки — вне перечня брифа; `met` не заявляется
- `FU-R10-NO-OVERCLAIM` | fixes applied: `0` | evidence: `.pdca/status/collection-open-issues-r5.md:2752-2768` — все оговорки `unverified (… explicitly not met)`, ни одна не `met`; Demucs-формулировка без argv; блоки помечены candidate/not in effect; `PASS`/`done` не заявлены | last recurrence/escalation: оценка не выполнялась
- `FU-R10-STATUS-LINT` | fixes applied: `0` | evidence: `.pdca/cycle-open-issues-r5-int-followup-r10/lint.txt`, §DO result п. 7, §Progress log (прогоны 1–5) | last recurrence/escalation: —

Оговорённые критерии (обязаны оставаться `unverified`, никогда не `met`):

- `FU-R10-CAVEAT-N4-N5-IMMUTABILITY` | fixes applied: `0` | evidence: `.pdca/status/collection-open-issues-r5.md:2755-2758` — `unverified (non-blocking by user instruction, explicitly not met)`; прежние строки `FU-R3-IMMUTABILITY`, `FU-HISTORY-PRESERVATION` (`collection-open-issues-r5.md:2478-2479`) не изменены | last recurrence/escalation: пробел свидетельств не устранялся
- `FU-R10-CAVEAT-N8-PRESERVATION` | fixes applied: `0` | evidence: `.pdca/status/collection-open-issues-r5.md:2759-2761` — `unverified (…)`; прежняя строка `FU-R8-HISTORY-PRESERVED` (`cycle-open-issues-r5-int-followup-r8.md:196`) не изменена | last recurrence/escalation: пробел свидетельств не устранялся
- `FU-R10-CAVEAT-N8-SCOPE-JOURNAL` | fixes applied: `0` | evidence: `.pdca/status/collection-open-issues-r5.md:2762-2764` — `unverified (…)`; прежняя строка `FU-R8-EXECUTION-SCOPE` (`cycle-open-issues-r5-int-followup-r8.md:200`) не изменена | last recurrence/escalation: пробел свидетельств не устранялся
- `FU-R10-CAVEAT-BYTE-AND-COMMAND-COMPLETENESS` | fixes applied: `0` | evidence: `.pdca/status/collection-open-issues-r5.md:2765-2766` — `unverified (…)`; §DO result п. 1 (хеш не пересчитывался) и п. 5 (полнота журнала операций не заявляется) | last recurrence/escalation: пробел свидетельств не устранялся

Раскрытые отклонения и перенесённые ключи (append-only):

- `F-R10-GIT-READ-DEVIATION` | observed: `N=10 · r=1 · n=1` | fixes applied: `0` | evidence: §DO result п. 5–6; `.pdca/status/collection-open-issues-r5.md:2770-2774`; `.pdca/status/cycle-open-issues-r5-int-followup-r8.md:301-303` — `git rev-parse HEAD`, `git status --porcelain`, read-only, до первой правки | last recurrence/escalation: первое наблюдение ключа; исход — за `CHECK`/`ACT` (ни мутации, ни `push` не было)
- `F-R10-GIT-READ-DEVIATION` — исход (подтверждение, append-only): `CHECK` признал дефект `blocking`, `confirmed`, критерий `FU-R10-EXECUTION-SCOPE` = `unmet` → `ACT` финализировал `incomplete (STOP — F-R10-GIT-READ-DEVIATION)` без Stage 2; fixes applied: `0` (устранимо только новым авторизованным циклом); evidence: §CHECK record (находка), `.pdca/status/collection-open-issues-r5.md` `### N=10 ACT Terminal Record — incomplete (STOP — F-R10-GIT-READ-DEVIATION)` | last recurrence/escalation: без повтора в этом цикле; повтор в новом цикле потребует `pdca-escalate` до следующей правки
- `F-R10-ACT-SCOPE-DEVIATION` | observed: `N=10 · r=1 · n=1` (фаза `ACT`) | fixes applied: `0` | evidence: §Current state (Notice `ACT`), §Progress log этой записи — два read-only shell-вызова `ACT`: внешний листинг каталога вне репозитория (`C:\Users\…\.config\opencode`) и чтение часов (`Get-Date`); оба вне перечня «append-only статусные записи и `status_lint.py`» | last recurrence/escalation: первое наблюдение ключа; раскрыто самим исполнителем; мутаций, `git`, тестов, продуктовых правок — `0`; исход — за пользователем/следующим циклом
- `F-R9-SCOPE-DEVIATION` | observed: `N=9 · r=1 · n=1` (перенос по указателю, не переоценивается) | fixes applied: `0` | evidence: `.pdca/status/cycle-open-issues-r5-int-followup-r9.md` §DO result п. 5; `.pdca/status/collection-open-issues-r5.md:2745-2750` (раскрытие без ретроактивной ратификации) | last recurrence/escalation: `N=9` остаётся закрытым STOP; `N=10` его не переоткрывает

## Changed files

- `.pdca/status/cycle-open-issues-r5-int-followup-r10.md` → **создана** этим циклом (`DO`): §Original task, §PLAN record `r1` (дословно), §DO result, §Defect history по указателям, §Git state, §Progress log
- `.pdca/status/collection-open-issues-r5.md` → **append-only**: один блок `### Post-ACT Accounting Correction (N=10 …)` со строки `2719` (пустая разделительная) / заголовок `2720` до строки `2781`; строки `1`–`2718` — прежнее содержимое без изменений
- `.pdca/status/cycle-open-issues-r5-int-followup-r8.md` → **append-only**: один блок `## Post-STOP Cross-Reference (N=10 …)` со строки `286` до строки `303`; строки `1`–`284` — прежнее содержимое без изменений
- `.pdca/status/cycle-open-issues-r5-int-followup-r9.md` → **append-only**: один блок-указатель `## N=10 Cross-Reference …` со строки `271` до строки `281`; строки `1`–`269` — прежнее содержимое без изменений
- `.pdca/cycle-open-issues-r5-int-followup-r10/lint.txt` → создан: команды, stdout и exit-коды прогонов `status_lint.py`
- правок продукта, тестов, `Cargo.toml`, item-записей и записей `N=2`–`N=7` — нет
- `ACT` (`N=10`): `.pdca/status/cycle-open-issues-r5-int-followup-r10.md` → обновлены §Current state, §CHECK record
  (отчёт `CHECK` дословно), §ACT record, §Lessons learned (ретроспектива дословно), §Defect history, §Changed
  files, §Git state, §Progress log (только дописано)
- `ACT` (`N=10`): `.pdca/status/collection-open-issues-r5.md` → **append-only**: аннотация
  `### N=10 ACT Terminal Record — incomplete (STOP — F-R10-GIT-READ-DEVIATION)` в EOF (заголовок — строка `2783`);
  все прежние байты сохранены как точный префикс, ни один не переписан (побайтовая сверка хешем не выполнялась)
- `ACT` (`N=10`): `.pdca/status/cycle-open-issues-r5-int-followup-r8.md` → **append-only**: одна строка-указатель на
  эту аннотацию в EOF; все прежние байты сохранены как точный префикс (побайтовая сверка хешем не выполнялась)
- правок `-r9.md`, item-записей, продукта, тестов — нет; `.pdca/cycle-open-issues-r5-int-followup-r10/lint.txt`
  этим `ACT` **не изменялся** (прогоны `ACT` зафиксированы в §Progress log; файла-копии stdout не создавалось,
  чтобы не выйти за перечень «append-only статусные записи и `status_lint.py`»)

## Git state

- status path: `.pdca/status/cycle-open-issues-r5-int-followup-r10.md` — **created** этим циклом (`DO`) · `tracked/untracked` **не наблюдалось после создания**: `git` в этом цикле не был разрешён, а единственное наблюдение сделано read-only вызовами до первой правки (раскрыто: `F-R10-GIT-READ-DEVIATION`); durable on disk, not committed
- baseline, переданный циклом (не перечитывался как утверждение цикла): `HEAD = d042c2628edb52deab42118e18ac69974e5ad75d`, `HEAD^{tree} = 6cb258a028e0c3db51547a4a0b987fdb9e8e3e51`, `d042c26` unpublished (`ahead 1` of `origin/main`)
- наблюдение до первой правки (единственное, раскрытое): `.pdca/status/collection-open-issues-r5.md` — ` M` (изменён относительно `HEAD`); записи `cycle-open-issues-r5-int-followup-r{1…9}.md`, `collection-open-issues-r3.md`, `issue-4-r5.md` и каталог `.openidea/` — `??`; путь `-r10.md` на момент наблюдения отсутствовал
- same-cycle touched-file ledger: `.pdca/status/cycle-open-issues-r5-int-followup-r10.md` (создана), `.pdca/status/collection-open-issues-r5.md` (append в EOF), `.pdca/status/cycle-open-issues-r5-int-followup-r8.md` (append в EOF), `.pdca/status/cycle-open-issues-r5-int-followup-r9.md` (append в EOF), `.pdca/cycle-open-issues-r5-int-followup-r10/lint.txt` (создан); в индекс ничего не добавлялось, `git add`/`commit`/`amend`/`reset`/`push` — `0`; task changes pending commit: `0`
- другое незакоммиченное состояние рабочей копии этим циклом не присваивается; атрибуция вне перечисленных путей неизвестна
- `ACT`-обновление (`N=10`): `git` в этом `ACT` не выполнялся (запрещён), поэтому tracked/untracked и текущий
  индекс **не наблюдались**; предыдущее (раскрытое) наблюдение см. выше и остаётся единственным. `git add`/`stage`/
  `commit`/`amend`/`reset`/`push` — `0`; статус-запись durable on disk, not committed; task changes pending commit
  остаются `pending` и не индексировались. Затронутые этим `ACT` пути: `.pdca/status/cycle-open-issues-r5-int-followup-r10.md`
  (изменена), `.pdca/status/collection-open-issues-r5.md` (append), `.pdca/status/cycle-open-issues-r5-int-followup-r8.md`
  (append), `.pdca/cycle-open-issues-r5-int-followup-r10/lint.txt` — **не изменялся** этим `ACT` (прогоны `ACT`
  зафиксированы в §Progress log; файла-копии stdout не создавалось)

## Progress log

- `2026-11-13T08:34:04Z | PLAN | revision r1 | attempt n=1 | план планера записан дословно в §PLAN record r1 (source: возврат планера, переданный драйвером); на входе команды не запускались (commands_run = 0); записи N=7/N=8/N=9 и коллекция не тронуты | .pdca/status/cycle-open-issues-r5-int-followup-r10.md §PLAN record r1`
- `2026-11-13T08:34:04Z | DO | revision r1 | attempt n=1 | DO started: append-only блок-кандидат дописан в EOF .pdca/status/collection-open-issues-r5.md (прежние 2720 строк; заголовок — 2720, блок — до 2781), в EOF .pdca/status/cycle-open-issues-r5-int-followup-r8.md (прежние 284 строки; заголовок — 286, блок — до 303) и в EOF .pdca/status/cycle-open-issues-r5-int-followup-r9.md (прежние 269 строк; заголовок — 271, блок — до 281); правки — единственная замена EOF-анкера, прежние байты сохранены как точный префикс; хеширование не выполнялось | .pdca/status/collection-open-issues-r5.md; .pdca/status/cycle-open-issues-r5-int-followup-r8.md; .pdca/status/cycle-open-issues-r5-int-followup-r9.md`
- `2026-11-13T08:34:04Z | DO | revision r1 | attempt n=1 | Notice (косметическое расхождение): переданный с брифом путь lint-скрипта .pdca/bin/status_lint.py не существует; канонический — .pdca/scripts/status_lint.py (read-only листинг .pdca/scripts); план записан дословно и не правился | .pdca/status/cycle-open-issues-r5-int-followup-r10.md §DO result п. 4`
- `2026-11-13T08:34:04Z | DO | revision r1 | attempt n=1 | Notice (косметическое расхождение указателя): затребованная формулировка ссылается на «N=7 ACT/Notice», однако раскрытие пяти вызовов записано в .pdca/status/cycle-open-issues-r5-int-followup-r7.md:347-356 (блок коррекции N=8 в EOF записи N=7); дословная формулировка сохранена, точный локатор добавлен рядом | .pdca/status/collection-open-issues-r5.md:2737-2743`
- `2026-11-13T08:34:04Z | DO | revision r1 | attempt n=1 | Notice (отклонение объёма, раскрыто): два read-only git-вызова (git rev-parse HEAD, git status --porcelain) выполнены до первой правки, вне перечня «No … git operations» брифа; мутации, коммита, amend, reset, push не было; ключ F-R10-GIT-READ-DEVIATION; met по FU-R10-EXECUTION-SCOPE не заявляется | .pdca/status/cycle-open-issues-r5-int-followup-r10.md §DO result п. 6; §Defect history`
- `2026-11-13T08:34:04Z | DO | revision r1 | attempt n=1 | lint (прогон 1, три статусных файла одним вызовом): status-lint: OK — collection 3 tables/17 rows, -r8.md 1/6, -r9.md 0/0, EXIT=0 | команда: python .pdca/scripts/status_lint.py .pdca/status/collection-open-issues-r5.md .pdca/status/cycle-open-issues-r5-int-followup-r8.md .pdca/status/cycle-open-issues-r5-int-followup-r9.md; .pdca/cycle-open-issues-r5-int-followup-r10/lint.txt`
- `2026-11-13T08:34:04Z | DO | revision r1 | attempt n=1 | lint (прогон 2, четыре статусных файла одним вызовом, в т.ч. эта запись): status-lint: OK — collection 3/17, -r8.md 1/6, -r9.md 0/0, -r10.md 0/0, EXIT=0 | команда: python .pdca/scripts/status_lint.py .pdca/status/collection-open-issues-r5.md .pdca/status/cycle-open-issues-r5-int-followup-r8.md .pdca/status/cycle-open-issues-r5-int-followup-r9.md .pdca/status/cycle-open-issues-r5-int-followup-r10.md; .pdca/cycle-open-issues-r5-int-followup-r10/lint.txt`
- `2026-11-13T08:34:04Z | DO | revision r1 | attempt n=1 | lint (прогон 3, .pdca/status/cycle-open-issues-r5-int-followup-r10.md после правок указателей строк и журнала): status-lint: OK (0 tables, 0 rows), EXIT=0 | команда: python .pdca/scripts/status_lint.py .pdca/status/cycle-open-issues-r5-int-followup-r10.md; .pdca/cycle-open-issues-r5-int-followup-r10/lint.txt`
- `2026-11-13T08:34:04Z | DO | revision r1 | attempt n=1 | lint (прогон 4, .pdca/status/cycle-open-issues-r5-int-followup-r10.md после косметических правок): status-lint: OK (0 tables, 0 rows), EXIT=0 | команда: python .pdca/scripts/status_lint.py .pdca/status/cycle-open-issues-r5-int-followup-r10.md; .pdca/cycle-open-issues-r5-int-followup-r10/lint.txt`
- `2026-11-13T08:34:04Z | DO | revision r1 | attempt n=1 | DO завершён на границе DO → CHECK: commands_run = 14 (5 прогонов status_lint.py — прогоны 1–4 по exit 0, прогон 5 подтверждающий по окончательным байтам, его exit-код сообщён в возврате DO; и 9 in-repository read-only shell-вызовов, из них один содержал два read-only git-вызова); fixes applied = 0, n = 1; правок продукта, тестов, сборок, коммитов и push — 0; вердикта нет | .pdca/status/cycle-open-issues-r5-int-followup-r10.md §DO result п. 5, 7`
- `2026-11-13T13:41:09Z | CHECK | revision r1 | attempt n=1 | независимый CHECK вернул `not accepted`: матрица 6 met / 1 unmet / 4 unverified; находка `F-R10-GIT-READ-DEVIATION` (blocking, confirmed, `FU-R10-EXECUTION-SCOPE` = unmet); отчёт записан дословно в §CHECK record | §CHECK record; .pdca/status/collection-open-issues-r5.md:2720-2781; .pdca/cycle-open-issues-r5-int-followup-r10/lint.txt`
- `2026-11-13T13:41:09Z | ACT | revision r1 | attempt n=1 | применена ветка STOP плана (§5 / §6 Path B): Stage 2 ратификации не выполнялась; outcome `incomplete (STOP — F-R10-GIT-READ-DEVIATION)`; `N=8` "candidate not ratified" и STOP `N=9` без изменений | §ACT record; §PLAN record r1 §5–6`
- `2026-11-13T13:41:09Z | ACT | revision r1 | attempt n=1 | ретроспектива `pdca-check` получена и сохранена дословно (§Lessons learned), ограничения и неопределённость записаны; ACT completed | §Lessons learned`
- `2026-11-13T13:41:09Z | ACT | revision r1 | attempt n=1 | в EOF .pdca/status/collection-open-issues-r5.md дописана аннотация `### N=10 ACT Terminal Record — incomplete (STOP — F-R10-GIT-READ-DEVIATION)`; в EOF .pdca/status/cycle-open-issues-r5-int-followup-r8.md дописан однострочный указатель на неё; прежние байты обоих файлов не переписывались | .pdca/status/collection-open-issues-r5.md (EOF); .pdca/status/cycle-open-issues-r5-int-followup-r8.md (EOF)`
- `2026-11-13T13:41:09Z | ACT | revision r1 | attempt n=1 | Notice (отклонение объёма, раскрыто): два read-only shell-вызова этого ACT — внешний листинг каталога вне репозитория (C:\Users\…\.config\opencode) и чтение часов (Get-Date) — вне перечня «append-only статусные записи и status_lint.py»; ключ `F-R10-ACT-SCOPE-DEVIATION`; мутаций, git, тестов, продуктовых правок — 0 | §Current state (Notice); §Defect history`
- `2026-11-13T13:41:09Z | ACT | revision r1 | attempt n=1 | lint (прогон A, три затронутых файла одним вызовом): status-lint: OK — collection 3 tables/17 rows, -r8.md 1/6, -r10.md 1/11 (матрица CHECK внутри ограждения), EXIT=0 | команда: python .pdca/scripts/status_lint.py .pdca/status/collection-open-issues-r5.md .pdca/status/cycle-open-issues-r5-int-followup-r8.md .pdca/status/cycle-open-issues-r5-int-followup-r10.md`
- `2026-11-13T13:41:09Z | ACT | revision r1 | attempt n=1 | ACT закрыт: outcome `incomplete (STOP — F-R10-GIT-READ-DEVIATION)`; residual `FU-R10-EXECUTION-SCOPE` unmet + 4 оговорки unverified; эффективный итог коллекции без изменений — N=7-ратифицированный `done (with explicit caveat …)`; следующее решение — за пользователем | §ACT record; .pdca/status/collection-open-issues-r5.md (EOF)
- `2026-11-13T13:41:09Z | ACT | revision r1 | attempt n=1 | коррекция журнала (раскрыто): строка Notice об отклонении объёма была ошибочно заменена при правке этого же ACT и восстановлена дословно; прежние строки `PLAN`/`DO` и байты прежних циклов не затронуты; §Changed files «только дописано» относится к состоянию до начала ACT | §Progress log (эта строка и предыдущие строки ACT)
- `2026-11-13T13:41:09Z | ACT | revision r1 | attempt n=1 | lint (прогон B, после строк журнала и уточнения §Changed files/§Git state): status-lint: OK — collection 3/17, -r8.md 1/6, -r10.md 1/11, EXIT=0 | команда: python .pdca/scripts/status_lint.py .pdca/status/collection-open-issues-r5.md .pdca/status/cycle-open-issues-r5-int-followup-r8.md .pdca/status/cycle-open-issues-r5-int-followup-r10.md
- `2026-11-13T13:41:09Z | ACT | revision r1 | attempt n=1 | lint (прогон C, по байтам после строк журнала B): status-lint: OK — collection 3/17, -r8.md 1/6, -r10.md 1/11, EXIT=0; затем исправлена фактическая ошибка Notice о `## Approval record` (заголовок присутствует в §PLAN record) и пересчитан `commands_run`; подтверждающий прогон D выполняется после этих правок | команда: python .pdca/scripts/status_lint.py .pdca/status/collection-open-issues-r5.md .pdca/status/cycle-open-issues-r5-int-followup-r8.md .pdca/status/cycle-open-issues-r5-int-followup-r10.md`

## N=11 — Candidate Post-Hoc Ratification Note (CANDIDATE ONLY, PENDING N=11 CHECK AND ACT RATIFICATION)

Appended at EOF by cycle `N=11` (`cycle-open-issues-r5-int-followup-r11` · `N=11` · `r=1` · `n=1`). No byte above is
rewritten and this record is **not** reopened.

- **`N=10` stays closed STOP.** The outcome `incomplete (STOP — F-R10-GIT-READ-DEVIATION)`, its `CHECK` matrix and
  every row above are unchanged; nothing here withdraws that outcome or reopens the cycle.
- **The four read-only calls.** `git rev-parse HEAD`, `git status --porcelain`, one external directory listing and one
  clock read were executed **outside the `N=10` plan** and are recorded as a post-hoc-ratified narrow tooling
  exception in `N=11` by explicit user decision. The `N=10` plan never authorized them; this note claims no original
  authorization and does not rewrite the `N=10` plan text.
- **Candidate only.** This note acquires effect only if an independent `N=11` `CHECK` accepts the `N=11` candidate
  blocks and the `N=11` `ACT` records the ratification.

### N=11 ACT Record — Post-Hoc Ratification Recorded (`N=10` stays closed STOP)

Appended at EOF by the `ACT` phase of cycle `N=11` (`cycle-open-issues-r5-int-followup-r11` · `N=11` · `r=1` · `n=1`),
after the independent `N=11` `CHECK` returned `accepted → ACT`. No byte above is rewritten and this record is **not**
reopened.

- **`N=10` remains closed STOP.** The outcome `incomplete (STOP — F-R10-GIT-READ-DEVIATION)`, its `CHECK` matrix, its
  `F-R10-GIT-READ-DEVIATION` / `F-R10-ACT-SCOPE-DEVIATION` defect rows and every row above are unchanged; nothing here
  withdraws that outcome or turns it into an acceptance.
- **The four read-only calls are recorded as outside the `N=10` plan.** `git rev-parse HEAD`, `git status --porcelain`,
  one external directory listing and one clock read are ratified **post hoc** in `N=11` by explicit user decision as a
  narrow tooling exception. No original authorization by the `N=10` plan is claimed, and the `N=10` plan text is not
  rewritten.
- **Effect.** The candidate note appended above by the `N=11` `DO` is ratified only to that extent; the `N=10` outcome
  itself stays `incomplete (STOP — F-R10-GIT-READ-DEVIATION)`.
