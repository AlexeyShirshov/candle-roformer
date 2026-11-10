# Cycle status: issue-13 / cycle 2

## Current state

- task: `issue-13` · form: `cycle` · specialized skill: `pdca-coder`
- mode: `autonomous`
- cycle: `N=2` · revision: `r=1` · attempt: `n=2/3`
- scope: `new authorized cycle of issue #13 (separate from closed collection cycle N=1); unifies both branches to N(p − y), parity test for normalized path, bounded-memory measurement` · status path: `.pdca/status/issue-13-2.md`
- goal: `feat: library-level normalization, amplification and spectral inversion (#13)` — the §PLAN record and §Original task are authoritative
- phase: `ACT` · record: `completed` · outcome: `accepted`
- source: `user authorization (reply "1") + issue #13; new cycle separate from closed collection cycle N=1` · context refs: `.pdca/status/issue-13-1.md; issue #13; .pdca/issues-milestone-1.0-r2/rv1/item-13/incomplete-13.patch`
- criteria: `AC-1, AC-2, AC-3 — exact text in §PLAN record r1` (source: planner PLAN)

## Original task

«Новый цикл по #13: свести обе ветки к одной формуле `N(p − y)`, добавить тест на паритет нормализованного пути + замер bounded-memory — работа узкая, кандидат уже готов на 90%»

Confirmed by the user's reply «1» to that proposal. Criteria verbatim from issue #13 (full issue text and the two verbatim criteria (a) and (b)): `.pdca/status/issue-13-1.md:16–39`.

## PLAN record

### r1

# План PDCA — Цикл N=2 (`issue-13`, `r=1`, `n=1/3`, автономный режим)

## 1. Форма, специализированный скилл, точка привязки и входной объём (Entry scope)

- **Форма:** `cycle` (Bounded Execution — задана пользователем как ограничение и подтверждена фактами).
- **Специализированный скилл (overlay):** `pdca-coder` (Rust-библиотека, DSP/streaming-код, тесты и документация).
- **Обоснование формы (в одну строку):** Метод полностью определён (применение готового на 90% патча-кандидата `.pdca/issues-milestone-1.0-r2/rv1/item-13/incomplete-13.patch` с выравниванием формулы вторичного стема в in-memory ветке под `N(p − y)` из streaming-ветки, добавлением теста паритета нормализованного режима и замером пикового потребления памяти на коротких фикстурах), а объём жёстко ограничен файлами задачи `#13` (`src/config.rs`, `src/dsp.rs`, `src/separator.rs`, `tests/library_api.rs`, `tests/streaming_chunked.rs`, `README.md`) и артефактами цикла `.pdca/`.
- **Точка привязки (Re-anchor) и локация доказательств (Evidence location):** Репозиторий `/home/alex/sources/candle-roformer` (единственный рабочий checkout, он же корень привязки доказательств), ветка `main`, `HEAD = 1603333` (`feat: named multi-stem output API with residual semantics (#12)`), рабочее дерево чистое.
- **Входной объём (Entry scope):** Целиком задача issue `#13` в рамках авторизованного цикла N=2 — библиотечный API управления уровнем и спектром (`normalization`, `amplification`, `invert_spect`) с эталонными значениями по умолчанию, единой формулой вторичного стема `N(p − y)` между in-memory и streaming-путями при сохранении bounded-memory стриминга, тестами на оба режима (`exact residual` и `normalized`, включая межканальный паритет in-memory vs streaming в нормализованном режиме) и точной формулировкой инварианта и ограничений памяти в `README.md`.

## 2. Цель, запрашиваемый/поставляемый результат и критерии приёмки

### Исходная постановка задачи (дословно, неизменяемо)

Авторизация цикла N=2 пользователем (подтверждена ответом «1»):
> «Новый цикл по #13: свести обе ветки к одной формуле `N(p − y)`, добавить тест на паритет нормализованного пути + замер bounded-memory — работа узкая, кандидат уже готов на 90%»

Полный неизменяемый текст issue `#13` зафиксирован в `.pdca/status/issue-13-1.md:16–39` (`## Original task`). Пользовательские критерии из issue `#13` (дословно):
> - (a) Инвариант переформулирован точно в доках, есть тест на оба режима: «exact residual» и «normalized».
> - (b) Library-level API реализует нормализацию, amplification и spectral inversion с reference-defaults: `normalization = 0.9`, `amplification = 0.0` (off), `invert_spect`.

- **Запрашиваемый результат (Requested result):** Зафиксированное в одном атомарном коммите на `main` состояние библиотеки `candle-roformer` по задаче `#13`, в котором реализованы `normalization = 0.9`, `amplification = 0.0`, `invert_spect = false` (по умолчанию), оба пути разделения (in-memory и bounded-memory streaming) вычисляют вторичный стем по единой формуле `N(p − y)` без деградации константного по длине входа профиля памяти стриминга, документация точно описывает инвариант обоих режимов и оговорку масштаба памяти, а тесты подтверждают оба режима (`exact residual` и `normalized`, включая паритет in-memory и streaming при включённой нормализации).
- **Поставляемый результат (Delivered result):** Зафиксированное в одном атомарном коммите на `main` состояние библиотеки `candle-roformer` по задаче `#13`, в котором реализованы `normalization = 0.9`, `amplification = 0.0`, `invert_spect = false` (по умолчанию), оба пути разделения (in-memory и bounded-memory streaming) вычисляют вторичный стем по единой формуле `N(p − y)` без деградации константного по длине входа профиля памяти стриминга, документация точно описывает инвариант обоих режимов и оговорку масштаба памяти, а тесты подтверждают оба режима (`exact residual` и `normalized`, включая паритет in-memory и streaming при включённой нормализации). *(Тот же самый объект).*

### Критерии приёмки (дословно из постановки пользователя, без добавления новых критериев)

- **AC-1 (критерий (a) из issue #13, дословно):**
  «Инвариант переформулирован точно в доках, есть тест на оба режима: «exact residual» и «normalized».»
- **AC-2 (критерий (b) из issue #13, дословно):**
  «Library-level API реализует нормализацию, amplification и spectral inversion с reference-defaults: `normalization = 0.9`, `amplification = 0.0` (off), `invert_spect`.»
- **AC-3 (условие авторизации цикла N=2 пользователем, дословно):**
  «Новый цикл по #13: свести обе ветки к одной формуле `N(p − y)`, добавить тест на паритет нормализованного пути + замер bounded-memory — работа узкая, кандидат уже готов на 90%»

### Как оценивается результат цикла относительно исходной задачи и истории дефектов цикла N=1

1. **Устранение дефекта `13-D6` (расхождение формул вторичного стема между in-memory и streaming):**
   - В цикле N=1 (`r=1, n=3/3`) in-memory путь вычислял вторичный стем как `N(p − N(y))` (`src/separator.rs` ~356–366, ~440–446 в патче-кандидате), тогда как streaming-путь вычислял `N(p − y)` (~711–716, ~792, ~861, ~905–938), где `p` — входной сигнал после предварительного масштабирования (`amplify_quiet` / входной `normalize_peak`), `y` — сырой выход первичного стема в том же масштабе `p` до индивидуальной пиковой нормализации стема, а `N(·)` — `normalize_peak(·, normalization)` (тождественное отображение при `normalization == 0.0`).
   - В цикле N=2 обе ветки (in-memory и streaming, как для фазового вычитания при `invert_spect = false`, так и для спектральной инверсии при `invert_spect = true`) обязаны вычислять вторичный стем по единой формуле `N(p − y)`, не нормализуя первичный стем `y` до вычитания из `p`.
   - В режиме `exact residual` (`normalization = 0.0`, `amplification = 0.0`, `invert_spect = false`) `N` и предварительное масштабирование вырождаются в тождество, сохраняя тождество `vocals + instrumental == input`; в режиме `normalized` (`normalization > 0.0`) каждый стем на выходе имеет пик `<= normalization`, а выходы in-memory и streaming путей при совпадающей частоте дискретизации совпадают в пределах численной погрешности OLA-стриминга (проверяется тестом паритета нормализованного пути).
   - В `README.md` формулировка инварианта должна точно отражать как `exact residual`, так и `normalized` режим (единую формулу `N(p − y)` и независимую пиковую нормализацию стемов), а раздел про bounded-memory / оговорку масштаба (scale caveat, ранее на строках `README.md:170–215`, удалённый в патче-кандидате `:107–120`) должен быть сохранён и согласован с актуальными замерами.
2. **Верификация дефекта `13-D4` (сохранение bounded-memory стриминга):**
   - В цикле N=1 (`n=3`) OLA-стриминг с `chunk_duration` был восстановлен, но остался в статусе `unverified` из-за отсутствия замера памяти.
   - В цикле N=2 bounded-memory поведение `Separator::separate_file` при уровневой обработке подтверждается измерением по методу из §3 на коротких фикстурах разной длительности при фиксированном `chunk_duration`.

## 3. Метод измерения (Measurement method)

В задаче присутствуют две измеряемые величины: **(M1)** ограниченность памяти (bounded-memory) потокового разделения `separate_file` и **(M2)** численный паритет нормализованного пути (`in-memory` против `streaming`) и точность `exact residual`.

### M1. Пиковое потребление памяти потокового разделения (Bounded-memory RSS)

- **Метрика и единица измерения:** Пиковый резидентный размер памяти процесса (`Peak RSS` / `VmHWM`), выраженный в кибибайтах (`KiB`) или мебибайтах (`MiB`, `1 MiB = 1024 KiB`), а также относительный прирост пикового RSS между самой короткой и самой длинной входной фикстурой серии:
  $$\Delta_{\text{RSS}} = \text{PeakRSS}(T_{\max}) - \text{PeakRSS}(T_{\min}), \quad \text{Ratio}_{\text{RSS}} = \frac{\text{PeakRSS}(T_{\max})}{\text{PeakRSS}(T_{\min})}$$
- **Знаменатель (база нормировки):** $\text{PeakRSS}(T_{\min})$ при минимальной длительности входного сигнала из тестовой серии $T_{\min} \ge 10\text{ s}$ (при отношении длительностей входа $T_{\max} / T_{\min} \ge 4$, например серия из длительностей в диапазоне `10 / 30 / 60 / 120 s`).
- **Фильтры включения и исключения:**
  - *Включаются:* отдельные запуски процесса (один процесс на одну длительность входного WAV-файла $T$) для `Separator::separate_file` с включённой по умолчанию уровневой обработкой (`normalization = 0.9`) и фиксированным `chunk_duration` (при $T > \text{chunk\_duration}$ и совпадении частоты дискретизации входа с `model_sample_rate`, то есть в основном потоковом пути без ресемплирующего fallback).
  - *Исключаются:* объединённые прогоны нескольких длительностей в одном процессе (поскольку `VmHWM` монотонен на время жизни процесса), прогоны с несовпадающей частотой дискретизации (задокументированный non-bounded fallback `src/separator.rs:450–470`), а также сверхдлинные прогоны ($> 120\text{ s}$), запрещённые ограничениями вызова.
- **Условия сопоставимости:** Одинаковый бинарный артефакт и профиль сборки, одинаковая модель (синтетическая tiny-модель фиксированной конфигурации или локальный чекпойнт `/home/alex/models/MelBandRoformer_fp32.safetensors`), одинаковое устройство (`CPU`), одинаковое число каналов, частота дискретизации и `chunk_duration`, запуски в отдельных процессах на одной машине.
- **Порог и его происхождение (Provenance):**
  - При росте длительности входного аудио минимум в $4\times$ ($T_{\max} / T_{\min} \ge 4$, например от $10\text{ s}$ до $60\text{–}120\text{ s}$) пиковый RSS процесса не должен расти пропорционально длине входа: $\text{Ratio}_{\text{RSS}} \le 1.15$ (не более $15\%$ прироста пикового RSS при $\ge 400\%$ росте длины входа; в эталонных замерах задачи `#10` `.pdca/issues-milestone-1.0/rv1/item10/` разброс на коротких фикстурах составлял $< 5\%$). Порог $\text{Ratio}_{\text{RSS}} \le 1.15$ является **provisional** (основан на базовой линии задачи `#10` для OLA-стриминга и фиксирует отсутствие линейного $O(T)$ накопления буферов полного файла при двухпроходном/потоковом масштабировании).
- **Известные ограничения метода:** `VmHWM` / `ru_maxrss` включает фиксированный базовый оверхед рантайма, весов модели и аллокатора страницы ОС; на синтетической tiny-модели абсолютный RSS мал, поэтому границу $O(1)$ по памяти доказывает именно инвариантность $\text{PeakRSS}(T)$ к росту $T$ при $T \gg \text{chunk\_duration}$, а для сверхдлинных входов (масштаб часов, например $7200\text{ s}$) действует задокументированная в `README.md` оговорка масштаба (scale caveat).

### M2. Численный паритет нормализованного пути (`in-memory` vs `streaming`) и точность режимов

- **Метрика и единица измерения:** Максимальное абсолютное поточечное расхождение по всем сэмплам и каналам каждого стема $s \in \{\text{primary}, \text{secondary}\}$:
  $$E_{\infty}^{(s)} = \max_{i} \left| u_{s,\text{mem}}[i] - u_{s,\text{stream}}[i] \right| \quad (\text{в единицах амплитуды } \text{f32})$$
  а также пик модуля сигнала $\text{peak}(u_s) = \max_i |u_s[i]|$ и ошибка реконструкции в режиме `exact residual`:
  $$E_{\text{res}} = \max_{i} \left| (u_{\text{primary}}[i] + u_{\text{secondary}}[i]) - x_{\text{input}}[i] \right|.$$
- **Знаменатель и фильтры:** Все отсчёты $i \in [0, N_{\text{samples}} \times C)$ выходных стемов для идентичного ненулевого многоблочного входного сигнала ($T > \text{chunk\_duration}$) при одинаковой модели и одинаковом `InferenceConfig`.
- **Условия сопоставимости:** Идентичные веса модели, идентичный входной сигнал (частота дискретизации равна `model_sample_rate`), идентичный `InferenceConfig` с включённой нормализацией (`normalization = 0.9`, как для `invert_spect = false`, так и при проверке конфигураций уровневой обработки).
- **Порог и его происхождение (Provenance):**
  - В режиме `normalized` (`normalization = 0.9`):
    1. Пик каждого ненулевого выходного стема удовлетворяет $\left|\text{peak}(u_s) - 0.9\right| \le 10^{-4}$ (и $\text{peak}(u_s) \le 0.9 + 10^{-5}$).
    2. Паритет между `in-memory` и `streaming` для каждого стема (включая вторичный стем по единой формуле `N(p − y)`): $E_{\infty}^{(s)} \le 10^{-4}$ (происхождение порога: существующий допуск межканального OLA-паритета `chunked_boundary_parity` в `tests/streaming_chunked.rs` для `f32` STFT/ISTFT; при старой расходящейся формуле `N(p − N(y))` vs `N(p − y)` ошибка на вторичном стеме имеет порядок $\mathcal{O}(|1 - 1/\text{peak}(y)|) \gg 10^{-2}$).
  - В режиме `exact residual` (`normalization = 0.0`, `amplification = 0.0`, `invert_spect = false`): $E_{\text{res}} \le 10^{-5}$ (происхождение порога: инвариант точного остатка задачи `#12`/`#13` в существующих тестах `tests/library_api.rs` и `tests/streaming_chunked.rs`).

## 4. Ограничения, допущения и ресурсные границы

### Ограничения и запреты (Constraints & Prohibitions)

- **Автономный режим:** Никаких вопросов пользователю и промежуточных подтверждений.
- **Один атомарный коммит после принятого CHECK:** На ветке `main`, затрагивающий только файлы задачи `#13`, с заголовком `feat: library-level normalization, amplification and spectral inversion (#13)` и телом, перечисляющим значения по умолчанию, единую формулу вторичного стема `N(p − y)`, тесты, замеры RSS и оговорку масштаба (scale caveat).
- **Запрещённые git/GitHub-операции:** Запрещены `git push`, переключение веток, `rebase`, `reset --hard`, `commit --amend`, `--force`. Запрещены любые мутирующие вызовы `gh` (`close`, `comment`, `create`, `edit`); разрешён только read-only `gh issue view`. Закрытие issue принадлежит вызывающей стороне.
- **Запрещённые пути и внешние ресурсы:** Не скачивать, не бандлить и не кэшировать веса из сети; допустимо использовать только локальный `/home/alex/models/MelBandRoformer_fp32.safetensors` и синтетические tiny-модели в тестах. Запрещено изменять `AGENTS.md`, `LICENSE`, `docs/superpowers/**`, другие репозитории (включая `/home/alex/sources/karaoke-cli`) и прошлые записи `.pdca` (`.pdca/status/issue-13-1.md`, `.pdca/issues-milestone-1.0*`).
- **Жёсткие лимиты времени и процессов:** Любой одиночный shell-вызов $\le 300\text{ s}$; `cargo build|test|check` $\le 600\text{ s}$; запрещены `sleep > 60 s`, поллинг-циклы, `timeout > 600\text{ s}` и фоновые процессы, остающиеся после вызова. Замеры RSS — только на коротких фикстурах (например, `10 / 30 / 60 / 120 s`), без многотысячесекундных прогонов.
- **Дисциплина дерева и инструментов:** Никаких незафиксированных правок рабочего дерева — каждое изменение должно быть отражено в доказательствах (`diff.patch` + `sha256`) и в статус-файле `.pdca/status/issue-13-2.md` до входа в CHECK. При двух последовательных отказах одного и того же инструмента — зафиксировать причину и остановиться/эскалировать, не повторять вслепую.
- **Сохранение обратной совместимости и зелёного состояния:** Двухстемовый API, режим `exact residual`, потоковый `Separator::separate_file` и именованный мульти-стем API (`#12`) обязаны оставаться полностью работоспособными.

### Известные ресурсные ограничения и доступные средства

- Локальный тулчейн Rust (`cargo`), `/proc/self/status` (`VmHWM`) и `/usr/bin/time -v` (`Maximum resident set size`) доступны локально в Linux-окружении без сетевого доступа.
- Локальный чекпойнт `/home/alex/models/MelBandRoformer_fp32.safetensors` и генераторы `write_tiny_model` / `write_wav_f32` доступны локально.
- Все необходимые средства для реализации и вычисления метрик **M1** и **M2** в наличии; ресурсных блокеров нет.

### Допущения (Assumptions)

- **A1:** В формуле `N(p − y)` для in-memory пути `p` обозначает входной буфер после этапа предварительной уровневой обработки входа (`amplify_quiet` и входной `normalize_peak` при `normalization > 0.0`), а `y` — выход модели для первичного стема, полученный на этом входе `p` **до** применения финальной пиковой нормализации первичного стема `N(y)`. После вычисления остатка `r = p − y` (или `spectral_invert(p, y)` при `invert_spect = true`) к первичному и вторичному стемам независимо применяется финальная пиковая нормализация `N(y)` и `N(r)`. Это в точности совпадает с математикой streaming-пути, где вход масштабируется коэффициентом `total_scale = amp_scale * norm_scale` один раз, модель вызывается по чанкам на масштабированном входе, сырой остаток вычисляется до финальной нормализации стемов, и затем каждый стем независимо масштабируется до порога `normalization`.

## 5. Закрытие пробелов (Gaps)

| Пробел в постановке | Способ закрытия | Основание / Доказательство |
| --- | --- | --- |
| В каком именно месте in-memory и streaming расходятся по формуле вторичного стема (`13-D6`)? | **Evidence** | В патче-кандидате `.pdca/issues-milestone-1.0-r2/rv1/item-13/incomplete-13.patch` in-memory путь (`src/separator.rs` ~356–358) вызывает `normalize_peak` на первичном стеме `vocals` **до** вычитания `instr = input_ref - vocals` (~440–446), получая `N(p − N(y))`, тогда как streaming-путь вычитает ненормализованный `vocals_orig` из предварительно масштабированного входа и нормализует оба стема в конце (`N(p − y)`). |
| Почему `README.md:90–93` в прошлом отчёте не указывал на инвариант и что пропало из `README.md`? | **Evidence** | В патче-кандидате формулировка уровневой обработки и режимов находится на строках `:41–44`, `:52–62`, `:66–69` нового `README.md`, при этом секция bounded-memory RSS с оговоркой масштаба (`README.md:170–215` в `HEAD`) была случайно удалена ханком `:107–120` патча-кандидата. |
| Как верифицировать `13-D4` в рамках лимита времени $\le 300\text{ s}$ без многочасовых прогонов? | **Evidence + Assumption** | Схема замеров на коротких фикстурах (10–120 с) уже отработана в `.pdca/issues-milestone-1.0/rv1/item10/`; `chunk_duration` выбирается малым (например, `1.0–2.0 s`), чтобы уже на `10–120 s` проходить от единиц до десятков перекрывающихся чанков и проверять отсутствие линейного роста `Peak RSS` по длине входа. |

## 6. Минимальное решение и альтернативы

### Выбранное минимальное решение

1. Взять за основу проверенный патч-кандидат `.pdca/issues-milestone-1.0-r2/rv1/item-13/incomplete-13.patch` (`sha256: 6f6d27c706db1d1dfee65124b76974237acfdda1634eedbedb638e132c340df8`).
2. В `src/separator.rs` привести in-memory ветку к формуле `N(p − y)`: вычислять вторичный стем (как при обычном вычитании во временной области, так и при `invert_spect = true`) из предварительно обработанного входа `p` и **ненормализованного** выхода первичного стема `y`, и только после получения вторичного стема применять финальную пиковую нормализацию `N(·)` к первичному и вторичному стемам независимо (когда `normalization > 0.0`).
3. Добавить интеграционный тест межканального паритета нормализованного пути (`in-memory` против `streaming` при `normalization = 0.9` по умолчанию), проверяющий оба стема (первичный и вторичный) по метрике **M2**, в дополнение к тестам обоих режимов (`exact residual` и `normalized`), `amplification` и `invert_spect`.
4. Восстановить в `README.md` секцию bounded-memory RSS и оговорку масштаба (scale caveat), согласовав описание режимов `exact residual` и `normalized` с единой формулой `N(p − y)` и актуальными цифрами замеров **M1** на коротких фикстурах.

### Рассмотренные альтернативы

- *Альтернатива А (отклонена):* Привести streaming-ветку к `N(p − N(y))` вместо приведения in-memory к `N(p − y)`.
  - *Причина отклонения:* Противоречит прямому указанию пользователя («свести обе ветки к одной формуле `N(p − y)`»), физическому смыслу остатка в масштабе входа и потребовало бы дополнительного прохода по полному потоку в streaming-режиме для знания глобального пика `y` до вычисления остатка.
- *Альтернатива Б (выбрана):* Свести in-memory ветку к `N(p − y)`, сохранив двухпроходную/потоковую архитектуру `separate_file` из патча-кандидата.

## 7. Оценка трудоёмкости (Effort estimate — наблюдение для автономного режима)

- Механизм полностью доказан (патч-кандидат готов на 90%, OLA-стриминг зафиксирован в `HEAD`). Предварительный исследовательский пробник (choice probe) перед DO не требуется. Ожидаемо укладывается в одну попытку `n=1/3`.

## 8. Передача в DO (Plan-to-DO handoff)

Каждый диспатч в DO в рамках этого цикла несёт по ссылке следующий неизменяемый пакет:

- **Исходная задача и авторизация:** `.pdca/status/issue-13-1.md:16–39` + авторизация цикла N=2 («Новый цикл по #13: свести обе ветки к одной формуле `N(p − y)`, добавить тест на паритет нормализованного пути + замер bounded-memory — работа узкая, кандидат уже готов на 90%»).
- **Критерии приёмки:** `AC-1`, `AC-2`, `AC-3` (§2) и метод измерения `M1`, `M2` (§3).
- **История дефектов из цикла N=1, обязательная к закрытию:**
  - `13-D6` (`open` в N=1): расхождение формул вторичного стема (`N(p − N(y))` в памяти против `N(p − y)` в стриминге) — устранить сведением обеих веток к `N(p − y)` и покрыть тестом паритета нормализованного пути.
  - `13-D4` (`unverified` в N=1): bounded-memory стриминг после добавления уровневой обработки — подтвердить замером по методу `M1` на коротких фикстурах и сохранить документацию/scale caveat в `README.md`.
- **Базовый патч-кандидат:** `.pdca/issues-milestone-1.0-r2/rv1/item-13/incomplete-13.patch` (идентичен `.pdca/issue-13-1` / `attempts/n3/diff.patch`, `sha256: 6f6d27c706db1d1dfee65124b76974237acfdda1634eedbedb638e132c340df8`).
- **Целевые пути записи для артефактов цикла N=2:** статус-файл `.pdca/status/issue-13-2.md`, директория доказательств `.pdca/issue-13/` (или привязанный каталог попытки согласно статус-файлу цикла N=2), без перезаписи `.pdca/status/issue-13-1.md` и `.pdca/issues-milestone-1.0*`.
- **Ограничения и запреты:** Все правила из §4 (автономность, лимиты времени $\le 300\text{ s}$ / $\le 600\text{ s}$, запрет сетевых загрузок и мутаций `gh`/`git push`, сохранность посторонних файлов и существующих API `#9`, `#10`, `#12`).

## 9. Риски и условия остановки (Risks and stop conditions)

### Риски

1. **R1 (Побочное влияние `invert_spect` на порядок нормализации в памяти):** В in-memory пути есть две ветки получения вторичного стема (`invert_spect = false` через разность во временной области и `invert_spect = true` через `dsp::spectral_invert`). Если перенести `normalize_peak` первичного стема только для одной из них, расхождение `N(p − N(y))` останется во второй. *Митигация:* применять финальную нормализацию обоих стемов единообразно после блока вычисления вторичного стема (и для обычного вычитания, и для `spectral_invert`).
2. **R2 (Случайная потеря секции bounded-memory в `README.md` при накате патча-кандидата):** Патч-кандидат удаляет строки `README.md:170–215`. *Митигация:* при подготовке `README.md` явно сохранить/обновить раздел bounded-memory streaming и scale caveat вместе с описанием режимов `exact residual` и `normalized`.
3. **R3 (Превышение таймаутов при замере RSS на реальных весах):** Прогон большого числа длительностей на полноразмерной fp32-модели на CPU может приблизиться к лимиту $300\text{ s}$. *Митигация:* использовать короткие фикстуры (`10 / 30 / 60 / 120 s`) и контролировать время одного запуска задолго до лимита $300\text{ s}$.

### Условия остановки (Stop conditions)

- **Успешное завершение (`done`):** Независимый CHECK подтверждает выполнение `AC-1`, `AC-2`, `AC-3` (включая закрытие `13-D6` и `13-D4` по метрикам `M1` и `M2`), после чего создаётся ровно один атомарный коммит на `main`.
- **Бюджет попыток:** Исчерпание бюджета `n = 3/3` без достижения критериев приёмки переводит цикл в `incomplete / STOP` с полной фиксацией остатка в `.pdca/status/issue-13-2.md`.
- **Инструментальный стоп:** Два последовательных отказа одного и того же инструмента или превышение жёсткого лимита времени ($300\text{ s}$ на команду / $600\text{ s}$ на сборку или тесты) — немедленная фиксация причины и остановка/эскалация без слепых повторов.

## 10. Уверенность и остаточные допущения (Confidence and gaps)

- **Подтверждено фактами (Verified):**
  - Чистое состояние дерева на `HEAD = 1603333` (`main`).
  - Наличие и целостность патча-кандидата `.pdca/issues-milestone-1.0-r2/rv1/item-13/incomplete-13.patch` (`sha256: 6f6d27c706db1d1dfee65124b76974237acfdda1634eedbedb638e132c340df8`).
  - Точная локализация дефекта `13-D6` в `src/separator.rs` (порядок вызова `normalize_peak` для первичного стема относительно вычитания вторичного стема в in-memory ветке) и причина удаления секции bounded-memory в `README.md` патча-кандидата.
- **Остаётся допущением до выполнения DO/CHECK:**
  - Конкретные численные значения $E_{\infty}^{(s)}$ в тесте паритета нормализованного пути и $\text{Ratio}_{\text{RSS}}$ на серии коротких фикстур после выравнивания формулы `N(p − y)` (проверяются и фиксируются в процессе DO и независимого CHECK).

## Approval record

`not applicable: autonomous mode`

## DO result

completed (n=1/3): unified both paths to N(p − y) in src/separator.rs (plain and invert_spect branches now use unnormalized primary output). Added normalized-mode parity test `chunked_normalized_parity` (in-memory vs streaming, both stems, threshold 1e-4). Restored README bounded-memory section with measured table. Red-run max-abs (old formula) = 0.29365373; green-run max-abs = vocals 1.192e-7, instrumental 0. RSS attribution: growth was harness (fixture generation in-process), not product; separate_file bounded. Reproduction-grade re-measurement (pre-generated fixtures, separate process per fixture, release binary sha256 1a5d5660…): 10 s 17.07, 30 s 17.01, 60 s 18.52, 120 s 18.38 MiB (VmHWM = /usr/bin/time max RSS); ratio 10->120 s = 18820/17476 = 1.077 <= 1.15. M1 holds. Build rc=0, tests rc=0 (79 tests). Evidence: .pdca/issue-13/rv1/ (diff.patch sha256 ce1ca326… (post-correction, README only), red_run.log, green_run.log, build.log, test.log, rss/run_10s.log..run_120s.log, rss/measurements.tsv, rss/ATTRIBUTION.md, rss/IDENTITY.txt, rss/FIXTURES.txt).

## CHECK record

`not run: PLAN just completed; DO not started`

## ACT record

accepted. Outcome: AC-1 met, AC-2 met, AC-3 met. Carried defect 13-D6 refuted (unified formula, red 0.29365373 -> green vocals 1.192e-7 / instrumental 0). 13-D4 refuted in measured range (separate-process RSS 17476/17420/18964/18820 KiB for 10/30/60/120 s, ratio 1.077 <= 1.15). 13-D7 corrected (README table aligned to reproduction series; binary/patch identity recorded). Historical-provenance exception: release-build command/time unrecoverable (recorded in rss/IDENTITY-ADDENDUM.md). Commit cb53e22 on main. No push, no branch change.

## Revision history

`none`

## Defect history

Carried from `.pdca/status/issue-13-1.md:214–229` (prior cycle N=1, r=1, n=3/3, terminal incomplete/STOP). Keys carried into cycle N=2 (statuses as recorded in the prior record, not reinterpreted):

- `13-D4` — `unverified` (bounded-memory behaviour has no current measurement). `.pdca/status/issue-13-1.md:227`
- `13-D6` — confirmed, fix not applied (normalized secondary stem computed as N(p−N(y)) in memory vs N(p−y) in streaming; budget exhausted, n=3/3). `.pdca/status/issue-13-1.md:225`
- `13-D2`, `13-D3`, `13-D5` — refuted within their checked scope. `.pdca/status/issue-13-1.md:216,217,223,229`
- Refuted (not fixed, not regressed): `13-E1`, `13-E2`, `13-E3`, `13-D1`. `.pdca/status/issue-13-1.md:218`
- `13-R1` (recovery drift, not a product defect, not a counted attempt). `.pdca/status/issue-13-1.md:220`

No new defect keys recorded yet in cycle N=2 (n=1/3, DO not started).

Cycle N=2 DO closure:
- `13-D6` (major, AC-2): CLOSED. In-memory formula unified to N(p − y) for both plain and invert_spect branches. Red-run max-abs (old formula) = 0.29365373; green-run max-abs = vocals 1.192e-7, instrumental 0. Evidence: .pdca/issue-13/rv1/red_run.log, green_run.log.
- `13-D4` (major, AC-2): VERIFIED. separate_file bounded: reproduction series 10 s 17.07, 30 s 17.01, 60 s 18.52, 120 s 18.38 MiB; ratio 10->120 s = 1.077 <= 1.15. Growth in prior series attributed to harness (fixture generation in-process), not product. Evidence: .pdca/issue-13/rv1/rss/ATTRIBUTION.md, run_*.log.
- `13-D7` (minor, AC-3/documentation): confirmed and corrected (n=2/3). Frozen deliverable docs/evidence identity did not agree with reproduction RSS series; binary->patch binding unrecorded. Corrected: README table updated to reproduction series (17.07/17.01/18.52/18.38 MiB, ratio 1.077); ATTRIBUTION.md erratum appended; measurements.clean.tsv written; IDENTITY-ADDENDUM.md records recovered identity. No product code change. Evidence: .pdca/issue-13/rv1/rss/.

## Changed files

- `src/separator.rs` — unified secondary-stem formula to N(p − y) (plain + invert_spect); raw vocals computed via no-norm path
- `tests/streaming_chunked.rs` — added `chunked_normalized_parity` test
- `README.md` — restored bounded-memory RSS section with measured table and scale caveat
- `.pdca/status/issue-13-2.md` — DO result, changed files, progress log, git state
- `.pdca/issue-13/rv1/` — evidence (diff.patch, red/green logs, build/test logs, rss attribution, harness copy)

## Git state

`status path: .pdca/status/issue-13-2.md — durable on disk, gitignored, not committed` · commit `cb53e22` on `main`; working tree clean; .pdca/ is durable state, not Git-tracked.

## Progress log

append-only, one row per event, never edited retroactively:

- `2026-11-10T08:38:26Z | PLAN | revision r1 | attempt n=1/3 | plan ready (cycle N=2, autonomous, form cycle, overlay pdca-coder) | .pdca/status/issue-13-2.md`
- `2026-11-10T14:00:00Z | DO | revision r1 | attempt n=1/3 | DO started (apply candidate patch + unify formula) | .pdca/issue-13/rv1/`
- `2026-11-10T14:05:00Z | DO | revision r1 | attempt n=1/3 | candidate patch applied (sha256 6f6d27c7… matches) | .pdca/issue-13/rv1/`
- `2026-11-10T14:10:00Z | DO | revision r1 | attempt n=1/3 | formula unified to N(p − y) in memory (plain + invert_spect) | src/separator.rs`
- `2026-11-10T14:15:00Z | DO | revision r1 | attempt n=1/3 | normalized parity test added | tests/streaming_chunked.rs`
- `2026-11-10T14:20:00Z | DO | revision r1 | attempt n=1/3 | red run recorded: max-abs instrumental 0.29365373 (old formula) | .pdca/issue-13/rv1/red_run.log`
- `2026-11-10T14:25:00Z | DO | revision r1 | attempt n=1/3 | green run recorded: vocals 1.192e-7, instrumental 0 | .pdca/issue-13/rv1/green_run.log`
- `2026-11-10T14:40:00Z | DO | revision r1 | attempt n=1/3 | RSS attribution: growth = harness (fixture gen in-process); product bounded, ratio 1.01 | .pdca/issue-13/rv1/rss/ATTRIBUTION.md`
- `2026-11-10T14:50:00Z | DO | revision r1 | attempt n=1/3 | README bounded-memory section restored with measured table + scale caveat | README.md`
- `2026-11-10T14:55:00Z | DO | revision r1 | attempt n=1/3 | build rc=0, tests rc=0 (79 tests) | .pdca/issue-13/rv1/build.log, test.log`
- `2026-11-10T15:00:00Z | DO | revision r1 | attempt n=1/3 | diff.patch persisted (sha256 d2e147b6…) | .pdca/issue-13/rv1/diff.patch`
- `2026-11-10T15:00:00Z | DO | revision r1 | attempt n=1/3 | DO completed; 13-D6 closed, 13-D4 verified | .pdca/status/issue-13-2.md`
- `2026-11-10T15:30:00Z | DO | revision r1 | attempt n=1/3 | CHECK→DO re-gather: reproduction-grade RSS (pre-generated fixtures, separate processes); ratio 1.077; M1 holds | .pdca/issue-13/rv1/rss/`
- `2026-11-10T16:00:00Z | DO | revision r1 | attempt n=2/3 | CHECK→DO correction (13-D7): README RSS table updated to reproduction series; diff re-frozen (sha256 ce1ca326…); tests/rss_measure.rs removed | .pdca/issue-13/rv1/rss/`
- `2026-11-10T16:30:00Z | ACT | revision r1 | attempt n=2/3 | accepted — commit cb53e22 on main (7 files); tree clean; historical-provenance exception recorded | .pdca/status/issue-13-2.md`
