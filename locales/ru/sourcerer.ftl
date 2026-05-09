# Sourcerer — Русский.

app-name = Sourcerer
tagline = Один поиск. Любой источник. Любая ОС.
window-title = Sourcerer
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
parse-error-unknown = Нераспознанный синтаксис в этом месте.

action-open = Открыть
action-reveal = Показать в папке
action-copy-path = Копировать путь
action-copy-name = Копировать имя
action-delete = Удалить

quick-filter-audio = Аудио
quick-filter-video = Видео
quick-filter-image = Изображение
quick-filter-document = Документ
quick-filter-executable = Исполняемый файл
quick-filter-archive = Архив

wizard-title = Добро пожаловать в Sourcerer
wizard-step-roots = Выберите, что индексировать
wizard-step-hotkey = Назначьте глобальную горячую клавишу
wizard-step-locale = Выберите язык
wizard-step-theme = Выберите тему
wizard-finish = Готово

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = Параметры
settings-search-placeholder = Поиск по параметрам…
settings-restore-defaults = Сбросить к умолчаниям
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
settings-node-home = Стартовая
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
settings-node-backup = Резервное копирование, экспорт, сброс
settings-node-locale = Язык и регион
settings-node-about = О программе

# §8.2 General → UI.
settings-ui-theme = Тема
settings-ui-run-bg = Работать в фоне
settings-ui-show-tray = Показывать значок в трее / строке меню
settings-ui-single-click-tray = Открывать трей / строку меню одним щелчком
settings-ui-new-window-from-tray = Открывать новое окно из значка в трее
settings-ui-new-window-on-launch = Открывать новое окно при запуске Sourcerer
settings-ui-search-as-you-type = Искать по мере ввода
settings-ui-select-on-mouse-click = Выделять запрос при щелчке мыши
settings-ui-focus-on-activate = Фокус на поиске при активации
settings-ui-full-row-select = Выделение всей строки
settings-ui-single-click-open = Открытие одним щелчком
settings-ui-underline-titles = Подчёркивать подписи значков
settings-ui-row-density = Плотность результатов
settings-ui-row-density-compact = Компактная (32 px)
settings-ui-row-density-comfortable = Комфортная (44 px)
settings-ui-show-timing-badges = Показывать значки времени выполнения для каждой линзы
settings-ui-anim-crossfade = Плавная смена темы

# §8.3 General → Home.
settings-home-match-case = Учитывать регистр
settings-home-match-whole-word = Только целые слова
settings-home-match-path = Искать по пути
settings-home-match-diacritics = Учитывать диакритику
settings-home-match-regex = Использовать Regex
settings-home-search = Поиск (свой запрос по умолчанию)
settings-home-filter = Фильтр
settings-home-sort = Сортировка
settings-home-view = Вид
settings-home-index = Индекс
settings-home-default-lens-visibility = Видимость линз по умолчанию
settings-home-default-lens-result-limits = Лимиты результатов линз по умолчанию

# §8.4 General → Search.
settings-search-fast-ascii = Быстрый поиск ASCII
settings-search-mp-sep = Искать по пути, если в запросе есть разделитель пути
settings-search-mw-fn = Сравнивать всё имя файла при использовании подстановочных знаков
settings-search-lit-ops = Разрешить буквальные операторы
settings-search-paren = Разрешить группировку круглыми скобками
settings-search-env = Раскрывать переменные окружения
settings-search-fwd-slash = Заменять прямые слэши на обратные
settings-search-precedence = Приоритет операторов
settings-search-strict-everything = Строгий синтаксис Everything
settings-search-auto-regex = Автоопределение Regex
settings-search-mod-comp = Автодополнение модификаторов
settings-search-parse-tree = Показывать дерево разбора при наведении

# §8.5 General → Results.
settings-results-hide-empty = Скрывать результаты при пустом запросе
settings-results-clear-on-search = Снимать выделение при поиске
settings-results-close-on-execute = Закрывать окно при выполнении
settings-results-dbl-path = Открывать путь двойным щелчком в столбце пути
settings-results-auto-scroll = Автопрокрутка списка
settings-results-dquote-copy = Копировать в кавычках как путь
settings-results-no-ext-rename = Не выделять расширение при переименовании
settings-results-sort-date-desc = Сначала сортировка по дате по убыванию
settings-results-sort-size-desc = Сначала сортировка по размеру по убыванию
settings-results-list-focus = Фокус списка результатов
settings-results-icon-prio = Приоритет загрузки значков
settings-results-thumb-prio = Приоритет загрузки миниатюр
settings-results-ext-prio = Приоритет загрузки расширенных сведений
settings-results-group-by-lens = Группировать результаты по линзам
settings-results-snippet-inline = Показывать предпросмотр фрагмента в строке

# §8.6 General → View.
settings-view-double-buffer = Двойная буферизация
settings-view-alt-rows = Чередующийся цвет строк
settings-view-row-mouseover = Подсвечивать строку под курсором
settings-view-highlight-terms = Подсвечивать найденные слова
settings-view-status-show-selected = Показывать выбранный элемент в строке состояния
settings-view-rc-with-sel = Показывать число результатов вместе с числом выбранных
settings-view-status-show-size = Показывать размер в строке состояния
settings-view-tooltips = Показывать всплывающие подсказки
settings-view-update-on-scroll = Обновлять отображение сразу после прокрутки
settings-view-size-format = Формат размера
settings-view-selection-rect = Прямоугольник выделения
settings-view-audio-badges = Показывать значки LUFS / codec / длительности на аудио-строках
settings-view-similarity-score = Показывать оценку сходства MinHash на строках сходства
settings-view-preview-pane = Панель предпросмотра

# §8.7 General → Context Menu.
settings-context-menu-visibility = Видимость
settings-context-menu-show = Показывать
settings-context-menu-shift = Показывать только при удержании Shift
settings-context-menu-hide = Скрывать
settings-context-menu-command = Макрос команды
settings-context-menu-open-folders = Открыть (папки)
settings-context-menu-open-files = Открыть (файлы)
settings-context-menu-open-path = Открыть путь
settings-context-menu-explore = Обзор
settings-context-menu-explore-path = Обзор пути
settings-context-menu-copy-name = Копировать имя в буфер обмена
settings-context-menu-copy-path = Копировать путь в буфер обмена
settings-context-menu-copy-full-name = Копировать полное имя в буфер обмена
settings-context-menu-reveal = Показать в Sourcerer
settings-context-menu-send-to = Отправить в Sourcerer (путь)

# §8.8 General → Fonts & Colors.
settings-fc-font = Шрифт
settings-fc-size = Размер
settings-fc-state-normal = Обычный
settings-fc-state-highlighted = Подсвеченный
settings-fc-state-current-sort = Текущая сортировка
settings-fc-state-current-sort-h = Текущая сортировка (подсвечено)
settings-fc-state-selected = Выделенный
settings-fc-state-selected-h = Выделенный (подсвечено)
settings-fc-state-inactive-selected = Неактивный выделенный
settings-fc-state-inactive-selected-h = Неактивный выделенный (подсвечено)
settings-fc-foreground = Передний план
settings-fc-background = Фон
settings-fc-bold = Жирный
settings-fc-italic = Курсив
settings-fc-default = По умолчанию
settings-fc-per-lens-accent = Акцент по линзе
settings-fc-theme-inherit = Автоматически инвертировать пользовательские цвета при смене темы

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Глобальная горячая клавиша
settings-keyboard-new-window = Горячая клавиша «Новое окно»
settings-keyboard-show-window = Горячая клавиша «Показать окно»
settings-keyboard-toggle-window = Горячая клавиша «Переключить окно»
settings-keyboard-show-commands = Показывать команды, содержащие
settings-keyboard-add-chord = + Добавить аккорд
settings-keyboard-remove-chord = Удалить

# §8.10 History.
settings-history-search-enable = Вести историю поиска
settings-history-search-keep = Хранить историю поиска { $days } дн.
settings-history-run-enable = Вести историю запусков
settings-history-run-keep = Хранить историю запусков { $days } дн.
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
settings-ix-force-rebuild = Принудительная перестройка
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
settings-vol-enable-inotify = Включить inotify (или fanotify при повышенных привилегиях)
settings-vol-buffer = Размер буфера журнала (КБ)
settings-vol-allocation-delta = Дельта выделения (КБ)
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
settings-folders-rescan-all = Пересканировать всё
settings-folders-monitor = Пытаться отслеживать изменения
settings-folders-buffer = Размер буфера
settings-folders-rescan-on-full = Пересканировать при заполнении буфера

# §8.14 Indexes → File Lists.
settings-flists-add = Добавить…
settings-flists-monitor = Отслеживать изменения
settings-flists-editor = Редактор списков файлов…
settings-flists-format = Формат списка файлов
settings-flists-format-text = Текст (один путь в строке)
settings-flists-format-json = JSON (с метаданными)
settings-flists-format-srcb = Пакет Sourcerer (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Исключать скрытые файлы и папки
settings-exclude-system = Исключать системные файлы и папки
settings-exclude-list-en = Включить список исключений
settings-exclude-folders = Исключать папки
settings-exclude-include-only-files = Включать только файлы (glob)
settings-exclude-files = Исключать файлы (glob)
settings-exclude-os-recommended = Применять рекомендуемые ОС исключения
settings-exclude-by-class = Исключать по классу расширений

# §8.16 Lenses → Filename.
settings-lf-trigram = Агрессивность префильтра по trigram
settings-lf-suffix-mem = Лимит памяти для суффиксного массива
settings-lf-wildcard-limit = Лимит раскрытия подстановочных знаков
settings-lf-regex-timeout = Тайм-аут Regex

# §8.17 Lenses → Content.
settings-lc-enable = Включить линзу содержимого
settings-lc-time-budget = Бюджет времени на документ
settings-lc-mem-ceiling = Лимит памяти на документ
settings-lc-snippet-len = Длина фрагмента
settings-lc-stop-words = Язык стоп-слов
settings-lc-re-extract = Заново извлекать при изменении настроек
settings-lc-verify-blobs = Проверять контрольные суммы blob с извлечённым текстом при чтении

# §8.18 Lenses → Audio.
settings-la-enable = Включить аудио-линзу
settings-la-lufs-ref = Стандарт-эталон LUFS
settings-la-peak-compute = Способ вычисления пика
settings-la-silence-thresh = Порог тишины
settings-la-re-extract-modify = Заново извлекать при событии Modify

# §8.19 Lenses → Similarity.
settings-ls-enable = Включить линзу сходства
settings-ls-sig-size = Размер подписи MinHash (k)
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
settings-net-port = Слушать на порту
settings-net-force-https = Принудительный HTTPS
settings-net-legacy-auth = Устаревшая аутентификация HTTP-basic
settings-net-token-regen = Пересоздать токен
settings-net-api-enable = Включить API-сервер
settings-net-legacy-ftp = Поддержка устаревших FTP/ETP без шифрования

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Автообновление
settings-privacy-prerelease = Канал предварительных версий
settings-privacy-network-policy = Политика сетевых вызовов

# §8.24 Logs & Debug.
settings-logs-level = Уровень журналирования
settings-logs-location = Расположение файла журнала
settings-logs-retention = Срок хранения журналов
settings-logs-debug-overlay = Показывать оверлей отладки
settings-logs-open-folder = Открыть папку журналов
settings-logs-export-bundle = Экспорт диагностического пакета

# §8.25 Backup, Export, Reset.
settings-backup-export = Экспортировать настройки
settings-backup-import = Импортировать настройки
settings-backup-export-bookmarks = Экспортировать пакет закладок
settings-backup-import-bookmarks = Импортировать пакет закладок
settings-backup-reset-all = Сбросить все настройки к умолчаниям

# §8.26 Locale.
settings-locale-current = Текущий язык
settings-locale-rtl-preview = Предпросмотр RTL
settings-locale-date-format = Формат даты
settings-locale-number-format = Формат чисел

# §8.27 About.
settings-about-version = Sourcerer { $version }
settings-about-license = Лицензия
settings-about-credits = Благодарности
settings-about-notices = Уведомления об открытом ПО
