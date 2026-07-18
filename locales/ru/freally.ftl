# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = Один поиск. Все источники. Любая ОС.
window-title = Freally Sourcerer
search-placeholder = Поиск…
about-version = Версия { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = Готово
status-indexed = Проиндексировано ({ $count } файлов)
status-indexing = Индексация… { $done }/{ $total }
status-paused = Приостановлено
status-error = Ошибка
status-result-count-one = { $count } результат
status-result-count-many = { $count } результатов
status-selection = · выбрано: { $count }
status-selection-size = Выбрано: { $size }
status-query-timing = Запрос: { $ms } мс
status-endpoint-local = Локальная БД
status-endpoint-remote = API: { $name }

menu-file = Файл
menu-edit = Правка
menu-view = Вид
menu-search = Поиск
menu-bookmarks = Закладки
menu-tools = Инструменты
menu-help = Справка

theme-system = Системная
theme-light = Светлая
theme-dark = Тёмная

lens-filename = Имя файла
lens-content = Содержимое
lens-audio = Аудио
lens-similarity = Сходство

parse-error-empty = Введите запрос, чтобы начать.
parse-error-unknown = Нераспознанный синтаксис рядом.

action-open = Открыть
action-reveal = Показать в папке
action-copy-path = Копировать путь
action-copy-name = Копировать имя
action-delete = Удалить

quick-filter-audio = Аудио
quick-filter-video = Видео
quick-filter-image = Изображения
quick-filter-document = Документы
quick-filter-executable = Исполняемые
quick-filter-archive = Архивы

wizard-title = Добро пожаловать в Freally
wizard-step-roots = Выберите, что индексировать
wizard-step-hotkey = Назначьте глобальную горячую клавишу
wizard-step-locale = Выберите язык
wizard-step-theme = Выберите тему
wizard-finish = Готово

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = Параметры
settings-search-placeholder = Поиск параметров…
settings-restore-defaults = Сбросить настройки
settings-ok = ОК
settings-cancel = Отмена
settings-apply = Применить

# Tree nav groups (PRD §8.1.1).
settings-group-general = Общие
settings-group-indexes = Индексы
settings-group-lenses = Линзы
settings-group-network = Сеть

# Tree nav leaves.
settings-node-ui = Интерфейс
settings-node-home = Главная
settings-node-search = Поиск
settings-node-results = Результаты
settings-node-view = Вид
settings-node-context-menu = Контекстное меню
settings-node-fonts-colors = Шрифты и цвета
settings-node-keyboard = Клавиатура
settings-node-history = История
settings-node-indexes-top = (верхний уровень)
settings-node-volumes = Тома
settings-node-folders = Папки
settings-node-file-lists = Списки файлов
settings-node-exclude = Исключения
settings-node-https-server = Сервер HTTP / HTTPS
settings-node-etp-api = API ETP / FTP
settings-node-privacy = Конфиденциальность и обновления
settings-node-logs = Журналы и отладка
settings-node-backup = Резервная копия, экспорт, сброс
settings-node-locale = Язык
settings-node-about = О программе

# §8.2 General → UI.
settings-ui-theme = Тема
settings-ui-run-bg = Работать в фоне
settings-ui-show-tray = Показывать значок в трее / строке меню
settings-ui-single-click-tray = Одиночный клик по трею / строке меню
settings-ui-new-window-from-tray = Открывать новое окно из значка в трее
settings-ui-new-window-on-launch = Открывать новое окно при запуске Freally
settings-ui-search-as-you-type = Поиск по мере ввода
settings-ui-select-on-mouse-click = Выделять поиск при клике мышью
settings-ui-focus-on-activate = Фокус на поиске при активации
settings-ui-full-row-select = Выделение всей строки
settings-ui-single-click-open = Открытие одиночным кликом
settings-ui-underline-titles = Подчёркивать подписи значков
settings-ui-row-density = Плотность результатов
settings-ui-row-density-compact = Компактная (32 px)
settings-ui-row-density-comfortable = Свободная (44 px)
settings-ui-show-timing-badges = Показывать метки времени для каждой линзы
settings-ui-anim-crossfade = Анимированный переход между темами

# §8.3 General → Home.
settings-home-match-case = Учитывать регистр
settings-home-match-whole-word = Слово целиком
settings-home-match-path = Искать по пути
settings-home-match-diacritics = Учитывать диакритику
settings-home-match-regex = Регулярное выражение
settings-home-search = Поиск (свой запрос по умолчанию)
settings-home-filter = Фильтр
settings-home-sort = Сортировка
settings-home-view = Вид
settings-home-index = Индекс
settings-home-default-lens-visibility = Видимость линз по умолчанию
settings-home-default-lens-result-limits = Лимиты результатов линз по умолчанию

# §8.4 General → Search.
settings-search-fast-ascii = Быстрый ASCII-поиск
settings-search-mp-sep = Искать по пути, если запрос содержит разделитель пути
settings-search-mw-fn = Сопоставлять имя файла целиком при использовании подстановочных знаков
settings-search-lit-ops = Разрешить буквальные операторы
settings-search-paren = Разрешить группировку круглыми скобками
settings-search-env = Раскрывать переменные среды
settings-search-fwd-slash = Заменять прямые слеши обратными
settings-search-precedence = Приоритет операторов
settings-search-strict-everything = Строгий режим синтаксиса Everything
settings-search-auto-regex = Автоопределение регулярных выражений
settings-search-mod-comp = Автодополнение модификаторов
settings-search-parse-tree = Показывать дерево разбора при наведении

# §8.5 General → Results.
settings-results-hide-empty = Скрывать результаты при пустом запросе
settings-results-clear-on-search = Сбрасывать выделение при поиске
settings-results-close-on-execute = Закрывать окно при выполнении
settings-results-dbl-path = Открывать путь двойным кликом в столбце пути
settings-results-auto-scroll = Автоматически прокручивать вид
settings-results-dquote-copy = Копировать путь в двойных кавычках
settings-results-no-ext-rename = Не выделять расширение при переименовании
settings-results-sort-date-desc = Сначала сортировать дату по убыванию
settings-results-sort-size-desc = Сначала сортировать размер по убыванию
settings-results-list-focus = Фокус на списке результатов
settings-results-icon-prio = Приоритет загрузки значков
settings-results-thumb-prio = Приоритет загрузки миниатюр
settings-results-ext-prio = Приоритет загрузки расширенных сведений
settings-results-group-by-lens = Группировать результаты по линзам
settings-results-snippet-inline = Показывать фрагмент-превью в строке

# §8.6 General → View.
settings-view-double-buffer = Двойная буферизация
settings-view-alt-rows = Чередовать цвет строк
settings-view-row-mouseover = Подсвечивать строку под курсором
settings-view-highlight-terms = Подсвечивать найденные термины
settings-view-status-show-selected = Показывать выбранный элемент в строке состояния
settings-view-rc-with-sel = Показывать число результатов вместе с числом выбранных
settings-view-status-show-size = Показывать размер в строке состояния
settings-view-tooltips = Показывать подсказки
settings-view-update-on-scroll = Обновлять экран сразу после прокрутки
settings-view-size-format = Формат размера
settings-view-selection-rect = Прямоугольник выделения
settings-view-audio-badges = Показывать метки LUFS / кодека / длительности в аудиострoках
settings-view-similarity-score = Показывать оценку сходства MinHash в строках сходства
settings-view-preview-pane = Панель предпросмотра

# §8.7 General → Context Menu.
settings-context-menu-visibility = Видимость
settings-context-menu-show = Показывать
settings-context-menu-shift = Показывать только при зажатом Shift
settings-context-menu-hide = Скрывать
settings-context-menu-command = Командный макрос
settings-context-menu-open-folders = Открыть (папки)
settings-context-menu-open-files = Открыть (файлы)
settings-context-menu-open-path = Открыть путь
settings-context-menu-explore = Обзор
settings-context-menu-explore-path = Обзор пути
settings-context-menu-copy-name = Копировать имя в буфер обмена
settings-context-menu-copy-path = Копировать путь в буфер обмена
settings-context-menu-copy-full-name = Копировать полное имя в буфер обмена
settings-context-menu-reveal = Показать в Freally
settings-context-menu-send-to = Отправить в Freally (путь)

# §8.8 General → Fonts & Colors.
settings-fc-font = Шрифт
settings-fc-size = Размер
settings-fc-state-normal = Обычный
settings-fc-state-highlighted = Подсвеченный
settings-fc-state-current-sort = Текущая сортировка
settings-fc-state-current-sort-h = Текущая сортировка (подсвеченная)
settings-fc-state-selected = Выбранный
settings-fc-state-selected-h = Выбранный (подсвеченный)
settings-fc-state-inactive-selected = Неактивный выбранный
settings-fc-state-inactive-selected-h = Неактивный выбранный (подсвеченный)
settings-fc-foreground = Передний план
settings-fc-background = Фон
settings-fc-bold = Полужирный
settings-fc-italic = Курсив
settings-fc-default = По умолчанию
settings-fc-per-lens-accent = Акцент для каждой линзы
settings-fc-theme-inherit = Автоматически менять свои цвета при смене темы

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Глобальная горячая клавиша
settings-keyboard-new-window = Горячая клавиша нового окна
settings-keyboard-show-window = Горячая клавиша показа окна
settings-keyboard-toggle-window = Горячая клавиша переключения окна
settings-keyboard-show-commands = Показывать команды, содержащие
settings-keyboard-add-chord = + Добавить сочетание
settings-keyboard-remove-chord = Удалить

# §8.10 History.
settings-history-search-enable = Включить историю поиска
settings-history-search-keep = Хранить историю поиска { $days } дней
settings-history-run-enable = Включить историю запусков
settings-history-run-keep = Хранить историю запусков { $days } дней
settings-history-clear-now = Очистить сейчас
settings-history-privacy-mode = Режим конфиденциальности
settings-history-per-lens = История по линзам

# §8.11 Indexes (top-level).
settings-ix-database-location = Расположение базы данных
settings-ix-multiuser = Имя файла многопользовательской базы данных
settings-ix-compress = Сжимать базу данных
settings-ix-recent-changes = Индексировать недавние изменения
settings-ix-file-size = Индексировать размер файла
settings-ix-fast-size-sort = Быстрая сортировка по размеру
settings-ix-folder-size = Индексировать размер папки
settings-ix-fast-folder-size-sort = Быстрая сортировка по размеру папки
settings-ix-date-created = Индексировать дату создания
settings-ix-fast-date-created = Быстрая сортировка по дате создания
settings-ix-date-modified = Индексировать дату изменения
settings-ix-fast-date-modified = Быстрая сортировка по дате изменения
settings-ix-date-accessed = Индексировать дату доступа
settings-ix-fast-date-accessed = Быстрая сортировка по дате доступа
settings-ix-attributes = Индексировать атрибуты
settings-ix-fast-attributes = Быстрая сортировка по атрибутам
settings-ix-fast-path-sort = Быстрая сортировка по пути
settings-ix-fast-extension-sort = Быстрая сортировка по расширению
settings-ix-force-rebuild = Принудительно пересобрать
settings-ix-compact = Сжать индекс
settings-ix-verify = Проверить индекс
settings-ix-integrity-policy = Политика целостности индекса
settings-ix-memory-budget = Лимит памяти для индексатора
settings-ix-throttle = Ограничение фоновой индексации

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Автоматически добавлять новые фиксированные тома
settings-vol-auto-removable = Автоматически добавлять новые съёмные тома
settings-vol-auto-remove-offline = Автоматически удалять отключённые тома
settings-vol-detected = Обнаруженные тома
settings-vol-include = Включить в индекс
settings-vol-include-only = Включать только (glob/regex)
settings-vol-enable-usn = Включить журнал USN
settings-vol-enable-fsevents = Включить поток FSEvents
settings-vol-enable-inotify = Включить inotify (или fanotify с правами)
settings-vol-buffer = Размер буфера журнала (KB)
settings-vol-allocation-delta = Дельта выделения (KB)
settings-vol-load-recent = Загружать недавние изменения из журнала при запуске
settings-vol-monitor = Отслеживать изменения
settings-vol-recreate-journal = Пересоздать журнал
settings-vol-reset-stream = Сбросить поток FSEvents
settings-vol-upgrade-fanotify = Перейти на fanotify (polkit)
settings-vol-remove = Удалить

# §8.13 Indexes → Folders.
settings-folders-watched = Отслеживаемые папки
settings-folders-add = Добавить…
settings-folders-rescan-now = Пересканировать сейчас
settings-folders-rescan-all = Пересканировать всё сейчас
settings-folders-monitor = Пытаться отслеживать изменения
settings-folders-buffer = Размер буфера
settings-folders-rescan-on-full = Пересканировать при заполнении буфера

# §8.14 Indexes → File Lists.
settings-flists-add = Добавить…
settings-flists-monitor = Отслеживать изменения
settings-flists-editor = Редактор списка файлов…
settings-flists-format = Формат списка файлов
settings-flists-format-text = Текст (по одному пути в строке)
settings-flists-format-json = JSON (с метаданными)
settings-flists-format-srcb = Пакет Freally (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Исключать скрытые файлы и папки
settings-exclude-system = Исключать системные файлы и папки
settings-exclude-list-en = Включить список исключений
settings-exclude-folders = Исключаемые папки
settings-exclude-include-only-files = Включать только файлы (glob)
settings-exclude-files = Исключаемые файлы (glob)
settings-exclude-os-recommended = Применять исключения, рекомендованные ОС
settings-exclude-by-class = Исключать по классу расширений

# §8.16 Lenses → Filename.
settings-lf-trigram = Агрессивность триграммного предфильтра
settings-lf-suffix-mem = Лимит памяти суффиксного массива
settings-lf-wildcard-limit = Предел раскрытия подстановочных знаков
settings-lf-regex-timeout = Тайм-аут регулярного выражения

# §8.17 Lenses → Content.
settings-lc-enable = Включить линзу содержимого
settings-lc-time-budget = Лимит времени на документ
settings-lc-mem-ceiling = Предел памяти на документ
settings-lc-snippet-len = Длина фрагмента
settings-lc-stop-words = Язык стоп-слов
settings-lc-re-extract = Извлекать заново при изменении настроек
settings-lc-verify-blobs = Проверять контрольные суммы блоба извлечённого текста при чтении

# §8.18 Lenses → Audio.
settings-la-enable = Включить аудиолинзу
settings-la-lufs-ref = Эталонный стандарт LUFS
settings-la-peak-compute = Вычислять пик через
settings-la-silence-thresh = Порог тишины
settings-la-re-extract-modify = Извлекать заново при событии изменения

# §8.19 Lenses → Similarity.
settings-ls-enable = Включить линзу сходства
settings-ls-sig-size = Размер сигнатуры MinHash (k)
settings-ls-bands = Полосы LSH
settings-ls-recall = Порог полноты
settings-ls-result-cap = Лимит результатов

# §8.20 Lenses → Custom.
settings-custom-registry = Реестр
settings-custom-trust = Доверие
settings-custom-refresh-hashes = Обновить хеши

# §8.21-§8.22 Network.
settings-net-https-enable = Включить сервер HTTPS
settings-net-bind = Привязка к интерфейсам
settings-net-port = Слушать порт
settings-net-force-https = Принудительный HTTPS
settings-net-legacy-auth = Устаревшая базовая аутентификация HTTP
settings-net-token-regen = Перегенерировать токен
settings-net-api-enable = Включить сервер API
settings-net-legacy-ftp = Поддержка устаревших FTP/ETP без шифрования

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Автообновление
settings-privacy-prerelease = Канал предварительных версий
settings-privacy-network-policy = Политика сетевых вызовов

# §8.24 Logs & Debug.
settings-logs-level = Уровень журналирования
settings-logs-location = Расположение файла журнала
settings-logs-retention = Срок хранения журналов
settings-logs-debug-overlay = Показывать отладочный оверлей
settings-logs-open-folder = Открыть папку журналов
settings-logs-export-bundle = Экспортировать пакет диагностики

# §8.25 Backup, Export, Reset.
settings-backup-export = Экспортировать настройки
settings-backup-import = Импортировать настройки
settings-backup-export-bookmarks = Экспортировать пакет закладок
settings-backup-import-bookmarks = Импортировать пакет закладок
settings-backup-reset-all = Сбросить все настройки к значениям по умолчанию

# §8.26 Locale.
settings-locale-current = Текущий язык
settings-locale-rtl-preview = Предпросмотр RTL
settings-locale-date-format = Формат даты
settings-locale-number-format = Формат чисел

# §8.27 About.
settings-about-version = Freally { $version }
settings-about-license = Лицензия
settings-about-credits = Благодарности
settings-about-notices = Уведомления об открытом ПО

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = Мастер первого запуска
wizard-step-of-total = Шаг { $step } из { $total }
wizard-roots-hint = Добавьте папки или тома, которые Freally будет отслеживать. Это можно изменить позже в настройках индексов.
wizard-browse = Обзор…
wizard-roots-placeholder = …или вставьте путь
wizard-roots-add = Добавить
wizard-roots-remove = Удалить
wizard-roots-empty = Корневые папки ещё не настроены.
wizard-locale-hint = Freally доступен на 18 языках. Язык можно сменить позже.
wizard-theme-hint = Системная следует настройке оформления вашей ОС.
wizard-back = Назад
wizard-next = Далее

# Status bar polish.
statusbar-hotkey-hint = Горячая клавиша: { $hotkey }
statusbar-cycle-theme = Переключить тему
statusbar-indexed-suffix = проиндексировано

# Results / lenses.
lens-expand = Развернуть линзу
lens-collapse = Свернуть линзу
lens-no-matches = В этой линзе нет совпадений.

# Preview pane.
preview-header = Предпросмотр
preview-loading = Загрузка…
preview-select-file = Выберите файл для предпросмотра.
preview-unavailable = Предпросмотр недоступен

# Bookmarks.
bookmarks-label = ★ Закладки
bookmarks-empty-hint = Закладок пока нет. Нажмите Ctrl+D, чтобы сохранить текущий запрос.
bookmarks-organize-title = Упорядочить закладки
bookmarks-organize-empty = Закладок пока нет.
bookmarks-rename = Переименовать
bookmarks-close = Закрыть

# Settings tree extras.
settings-group-history = История
settings-group-privacy = Конфиденциальность и обновления
settings-group-logs = Журналы и отладка
settings-group-backup = Резервная копия, экспорт, сброс
settings-tree-custom-lens = Своя
settings-unsaved-changes = несохранённые изменения

# About dialog.
about-dialog-title = Freally
about-copyright = Copyright © 2026 Mike Weaver. All rights reserved.
about-close = Закрыть

# Connect endpoint dialog.
connect-ftp-title = Подключение к серверу FTP
connect-ftp-host = Хост:
connect-ftp-port = Порт:
connect-ftp-username = Имя пользователя:
connect-ftp-password = Пароль:
connect-ftp-link-type = Тип соединения:

# UI panel.
ui-hint = Тема, интеграция с треем / строкой меню, поиск по мере ввода, плотность строк. Прямой паритет с voidtools-Everything плюс дополнения Freally, отмеченные (+).
ui-section-theme = Тема
ui-theme-system-default = Системная (по умолчанию)
ui-section-tray = Трей / строка меню
ui-section-search-behavior = Поведение поиска
ui-section-result-rows = Строки результатов
ui-single-click-system-default = Системные настройки (по умолчанию)
ui-single-click-always = Всегда одиночный клик
ui-single-click-always-double = Всегда двойной клик
ui-underline-always = Всегда
ui-underline-on-hover = При наведении
ui-underline-never = Никогда

# Home panel.
home-hint = Значения по умолчанию загружаются при запуске — каждый список может оставаться на «Использовать последнее значение» или закреплять фиксированное. Видимость линз / лимиты результатов — дополнения Freally (+).
home-section-match = Значения совпадения по умолчанию
home-section-search-sort = Значения поиска и сортировки по умолчанию
home-search-placeholder = По умолчанию пусто
home-section-index = Источник индекса
home-file-list-path = Путь к списку файлов
home-https-endpoint = URL конечной точки HTTPS API
home-endpoint-token = Токен (показан отпечаток)

# Backup panel.
backup-section-settings = Настройки (+)
backup-section-bookmarks = Закладки + свои экстракторы (+)
backup-section-reset = Сброс
backup-toast-exported = Настройки экспортированы в { $path }
backup-toast-export-failed = Сбой экспорта: { $error }
backup-toast-imported = Настройки импортированы
backup-toast-import-failed = Сбой импорта: { $error }
backup-toast-bookmarks-exported = Закладки экспортированы
backup-toast-bookmarks-export-failed = Сбой экспорта закладок: { $error }
backup-toast-bookmarks-imported = Закладки импортированы
backup-toast-bookmarks-import-failed = Сбой импорта закладок: { $error }
backup-confirm-reset = Сбросить все настройки к значениям по умолчанию? Это нельзя отменить (диалог останется открытым).
backup-toast-reset = Все настройки сброшены

# Keyboard panel.
keyboard-section-global = Глобальные горячие клавиши
keyboard-placeholder-example = Super+Space
keyboard-section-commands = Команды
keyboard-placeholder-command = идентификатор команды (например, file.export_results)
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = История поиска
history-section-run = История запусков
history-section-privacy = Конфиденциальность (+)
history-record-filename = Записывать историю линзы имён файлов
history-record-content = Записывать историю линзы содержимого
history-record-audio = Записывать историю аудиолинзы
history-record-similarity = Записывать историю линзы сходства

# Locale panel.
locale-section-language = Язык (+)
locale-section-time-date = Время / дата (+)
locale-date-os = По умолчанию ОС
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = Свой
locale-date-custom-format = Свой формат
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = Числа (+)
locale-number-os = По умолчанию ОС
locale-number-custom = Свой
locale-thousands-sep = Разделитель тысяч
locale-decimal-sep = Десятичный разделитель

# Folders panel.
folders-hint = Дополнительные отслеживаемые папки помимо томов по умолчанию.
folders-list-title = Отслеживаемые папки
folders-empty = Папки ещё не добавлены.
folders-remove = Удалить
folders-section-title-dynamic = Настройки для { $path }
folders-section-schedule = Расписание пересканирования
folders-schedule-daily = Каждый день в ЧЧ:ММ
folders-schedule-hours = Каждые N часов
folders-schedule-never = Никогда
folders-hour = Час
folders-minute = Минута
folders-hours = Часы
folders-id-label = ID папки (только чтение)
folders-select-prompt = Выберите папку для настройки.
folders-section-extras = Дополнения Freally (+)
folders-extras-note = Пересканирование при выходе из сна включено по умолчанию в этой сборке; переключатель присоединится к элементам управления на уровне папок в этапе полировки фазы 13.

# Volumes panel.
volumes-hint = Кроссплатформенный аналог панелей NTFS / ReFS из voidtools-Everything. Автоопределяет NTFS / ReFS / exFAT / FAT32 (Win), APFS / HFS+ (macOS), ext4 / Btrfs / ZFS / XFS / F2FS (Linux).
volumes-section-auto-include = Автодобавление
volumes-list-title = Обнаруженные тома
volumes-detecting = Обнаружение…
volumes-empty = Тома не обнаружены.
volumes-select-prompt = Выберите том для настройки.

# About panel polish.
about-section-version = Версия (+)
about-section-license = Лицензия (+)
about-license-text = Mike Weaver — Все права защищены. Это проприетарное программное обеспечение.
about-license-spdx = SPDX: { $spdx }
about-section-credits = Благодарности (+)
about-credits-inspired = Вдохновлено программой Everything от voidtools.
about-credits-voidtools = voidtools.com
about-credits-repo = Репозиторий проекта

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = Содержит команды для работы с Freally.
menu-file-new-window = Новое окно поиска
menu-file-open-list = Открыть список файлов…
menu-file-close-list = Закрыть список файлов
menu-file-close = Закрыть
menu-file-export-results = Экспортировать результаты…
menu-file-export-bundle = Экспортировать пакет индекса…
menu-file-exit = Выход

# Edit menu.
menu-edit-hint = Содержит команды для редактирования результатов поиска.
menu-edit-cut = Вырезать
menu-edit-copy = Копировать
menu-edit-paste = Вставить
menu-edit-copy-to-folder = Копировать в папку…
menu-edit-move-to-folder = Переместить в папку…
menu-edit-select-all = Выделить всё
menu-edit-invert-selection = Инвертировать выделение
menu-edit-advanced = Дополнительно
menu-edit-copy-full-name = Копировать полное имя
menu-edit-copy-path = Копировать путь
menu-edit-copy-filename = Копировать имя файла
menu-edit-copy-as-json = Копировать как JSON
menu-edit-copy-with-metadata = Копировать с метаданными
menu-edit-copy-as-bundle-ref = Копировать как ссылку на пакет Freally

# View menu.
menu-view-hint = Содержит команды для управления видом.
menu-view-filters = Фильтры
menu-view-preview = Предпросмотр
menu-view-status-bar = Строка состояния
menu-view-thumbs-xl = Очень крупные миниатюры
menu-view-thumbs-l = Крупные миниатюры
menu-view-thumbs-m = Средние миниатюры
menu-view-details = Таблица
menu-view-window-size = Размер окна
menu-view-window-size-hint = Содержит команды для изменения размера окна.
menu-view-window-small = Маленький
menu-view-window-medium = Средний
menu-view-window-large = Большой
menu-view-window-auto = Автоподбор
menu-view-zoom = Масштаб
menu-view-zoom-hint = Содержит команды для изменения размера шрифта и значков.
menu-view-zoom-in = Увеличить
menu-view-zoom-out = Уменьшить
menu-view-zoom-reset = Сбросить
menu-view-sort-by = Сортировать по
menu-view-sort-by-hint = Содержит команды для сортировки списка результатов.
menu-view-sort-name = Имя
menu-view-sort-path = Путь
menu-view-sort-size = Размер
menu-view-sort-ext = Расширение
menu-view-sort-type = Тип
menu-view-sort-modified = Дата изменения
menu-view-sort-created = Дата создания
menu-view-sort-accessed = Дата доступа
menu-view-sort-attributes = Атрибуты
menu-view-sort-recently-changed = Дата недавнего изменения
menu-view-sort-run-count = Число запусков
menu-view-sort-run-date = Дата запуска
menu-view-sort-file-list-filename = Имя файла из списка
menu-view-sort-lufs = LUFS
menu-view-sort-length = Длительность
menu-view-sort-similarity = Оценка сходства
menu-view-sort-asc = По возрастанию
menu-view-sort-desc = По убыванию
menu-view-go-to = Перейти к
menu-view-refresh = Обновить
menu-view-theme = Тема
menu-view-theme-hint = Переключение между системной, светлой и тёмной темами.
menu-view-lenses = Линзы
menu-view-lenses-hint = Переключение видимости каждой линзы в списке результатов.
menu-view-on-top = Поверх всех
menu-view-on-top-hint = Содержит команды для удержания этого окна поверх других окон.
menu-view-on-top-never = Никогда
menu-view-on-top-always = Всегда
menu-view-on-top-while-searching = Во время поиска

# Search menu.
menu-search-hint = Содержит переключатели поиска.
menu-search-match-case = Учитывать регистр
menu-search-match-whole-word = Слово целиком
menu-search-match-path = Искать по пути
menu-search-match-diacritics = Учитывать диакритику
menu-search-enable-regex = Включить регулярные выражения
menu-search-advanced = Расширенный поиск…
menu-search-add-to-filters = Добавить в фильтры…
menu-search-organize-filters = Упорядочить фильтры…
menu-search-filter-everything = Everything
menu-search-filter-archive = Сжатые (архивы)
menu-search-filter-folder = Папка
menu-search-filter-custom = Свой фильтр…

# Bookmarks menu.
menu-bookmarks-hint = Содержит команды для работы с закладками.
menu-bookmarks-add = Добавить в закладки
menu-bookmarks-organize = Упорядочить закладки…

# Tools menu.
menu-tools-hint = Содержит команды инструментов.
menu-tools-connect = Подключиться к серверу FTP…
menu-tools-disconnect = Отключиться от сервера FTP
menu-tools-file-list-editor = Редактор списка файлов…
menu-tools-index-maintenance = Обслуживание индекса
menu-tools-index-maintenance-hint = Инструменты обслуживания индекса.
menu-tools-verify-index = Проверить индекс…
menu-tools-compact-index = Сжать индекс…
menu-tools-rebuild-index = Принудительно пересобрать индекс…
menu-tools-custom-extractor = Менеджер своих экстракторов…
menu-tools-custom-extractor-hint = Управление своими экстракторами в песочнице Wasm.
menu-tools-options = Параметры…

# Help menu.
menu-help-hint = Содержит команды справки.
menu-help-help = Справка Freally
menu-help-search-syntax = Синтаксис поиска
menu-help-regex-syntax = Синтаксис регулярных выражений
menu-help-audio-ref = Справочник модификаторов аудио
menu-help-similarity-ref = Справочник модификаторов сходства
menu-help-cli-options = Параметры командной строки
menu-help-website = Веб-сайт Freally
menu-help-check-updates = Проверить обновления…
menu-help-sponsor = Спонсировать / пожертвовать
menu-help-about = О программе Freally…

# Result column headers (short forms used in the table header row).
column-name = Имя
column-path = Путь
column-size = Размер
column-modified = Изменён
column-type = Тип
column-ext = Расш.
column-sort-by = Сортировать по { $name }
column-resize = Изменить ширину столбца { $name }

# Section subtitle bars used inside multiple settings panels.
section-behavior = Поведение
section-rendering = Отрисовка
section-status-bar = Строка состояния
section-display-format = Формат отображения
section-loading-priority = Приоритет загрузки
section-compatibility = Совместимость
section-storage = Хранилище
section-index-fields = Поля индекса
section-maintenance = Обслуживание
section-logging = Журналирование
section-tools = Инструменты
section-privacy = Конфиденциальность
section-auto-update = Автообновление (+)
section-bind = Привязка
section-lens = Линза
section-budgets = Лимиты
section-other = Прочее
section-per-format-mode = Режим для каждого формата
section-loudness = Громкость
section-tuning = Тонкая настройка (+)
section-minhash-lsh = Параметры MinHash + LSH (+)
section-top-level = Верхний уровень
section-file-globs = Маски файлов
section-file-list-settings = Настройки выбранного списка файлов
section-editor-format = Редактор + формат (E + +)
section-api-server = Сервер API (адаптировано из E)
section-freally-extras = Дополнения Freally (+)
section-freally-additions = Дополнения Freally (+)
section-freally-extensions = Расширения Freally (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = Использовать последнее значение
opt-use-last-value-default = Использовать последнее значение (по умолчанию)
opt-low = Низкий
opt-normal-default = Обычный (по умолчанию)
opt-high = Высокий
opt-disabled = Отключено
opt-off = Выкл.
opt-on-battery = При работе от батареи
opt-always = Всегда
opt-clamp-default = Ограничивать (по умолчанию)
opt-wrap = Переносить
opt-none = Нет
opt-strict-refuse = Строгий (отклонять запросы при повреждении)
opt-lenient-warn = Мягкий (предупреждать, но выполнять)
opt-system-default = По умолчанию системы
opt-drag-select = Выделение перетаскиванием
opt-auto-binary = Авто (двоичный)
opt-auto-decimal = Авто (десятичный)

# Unit suffixes shown next to number inputs.
unit-days = дн.
unit-b = B
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-tb = TB

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = Немедленно
opt-lazy-default = Отложенно (по умолчанию)
opt-on = Вкл.
opt-on-default = Вкл. (по умолчанию)
opt-all = Все
opt-weekly = Еженедельно
opt-monthly = Ежемесячно
opt-name-asc = Имя по возр.
opt-name-desc = Имя по убыв.
opt-size-asc = Размер по возр.
opt-size-desc = Размер по убыв.
opt-modified-asc = Дата изменения по возр.
opt-modified-desc = Дата изменения по убыв.
opt-compact = Компактный
opt-comfortable = Свободный
opt-details = Таблица
opt-thumbnails = Миниатюры
opt-local-db-default = Локальная база данных (по умолчанию)
opt-file-list = Список файлов
opt-https-endpoint = Конечная точка HTTPS API
opt-right-default = Справа (по умолчанию)
opt-bottom = Снизу
opt-or-and-default = OR > AND (по умолчанию)
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128 (по умолчанию)
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = Broadcast film (-23)
opt-true-peak = Истинный пик (4× передискретизация, по умолчанию)
opt-sample-peak = Пиковый отсчёт
opt-auto-per-doc = Авто (на документ)
opt-log-error = Error
opt-log-warn = Warn
opt-log-info-default = Info (по умолчанию)
opt-log-debug = Debug
opt-log-trace = Trace

# More Freally apps (Central inside panel) — host chrome
menu-help-more-apps = Другие приложения Freally…
moreapps-title = Другие приложения Freally
