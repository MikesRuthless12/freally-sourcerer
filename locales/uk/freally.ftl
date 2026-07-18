# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = Один пошук. Усі джерела. Усі ОС.
window-title = Freally Sourcerer
search-placeholder = Пошук…
about-version = Версія { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = Готово
status-indexed = Проіндексовано ({ $count } файлів)
status-indexing = Індексування… { $done }/{ $total }
status-paused = Призупинено
status-error = Помилка
status-result-count-one = { $count } результат
status-result-count-many = { $count } результатів
status-selection = · вибрано { $count }
status-selection-size = Вибрано: { $size }
status-query-timing = Запит: { $ms } мс
status-endpoint-local = Локальна БД
status-endpoint-remote = API: { $name }

menu-file = Файл
menu-edit = Редагування
menu-view = Вигляд
menu-search = Пошук
menu-bookmarks = Закладки
menu-tools = Інструменти
menu-help = Довідка

theme-system = Системна
theme-light = Світла
theme-dark = Темна

lens-filename = Ім’я файлу
lens-content = Вміст
lens-audio = Аудіо
lens-similarity = Подібність

parse-error-empty = Введіть запит, щоб почати.
parse-error-unknown = Нерозпізнаний синтаксис поблизу.

action-open = Відкрити
action-reveal = Показати в теці
action-copy-path = Копіювати шлях
action-copy-name = Копіювати ім’я
action-delete = Видалити

quick-filter-audio = Аудіо
quick-filter-video = Відео
quick-filter-image = Зображення
quick-filter-document = Документ
quick-filter-executable = Виконуваний
quick-filter-archive = Архів

wizard-title = Ласкаво просимо до Freally
wizard-step-roots = Виберіть, що індексувати
wizard-step-hotkey = Виберіть глобальну гарячу клавішу
wizard-step-locale = Виберіть мову
wizard-step-theme = Виберіть тему
wizard-finish = Завершити

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = Параметри
settings-search-placeholder = Пошук параметрів…
settings-restore-defaults = Відновити типові
settings-ok = OK
settings-cancel = Скасувати
settings-apply = Застосувати

# Tree nav groups (PRD §8.1.1).
settings-group-general = Загальні
settings-group-indexes = Індекси
settings-group-lenses = Лінзи
settings-group-network = Мережа

# Tree nav leaves.
settings-node-ui = Інтерфейс
settings-node-home = Початок
settings-node-search = Пошук
settings-node-results = Результати
settings-node-view = Вигляд
settings-node-context-menu = Контекстне меню
settings-node-fonts-colors = Шрифти й кольори
settings-node-keyboard = Клавіатура
settings-node-history = Історія
settings-node-indexes-top = (верхній рівень)
settings-node-volumes = Томи
settings-node-folders = Теки
settings-node-file-lists = Списки файлів
settings-node-exclude = Виключення
settings-node-https-server = Сервер HTTP / HTTPS
settings-node-etp-api = API ETP / FTP
settings-node-privacy = Конфіденційність і оновлення
settings-node-logs = Журнали й налагодження
settings-node-backup = Резервне копіювання, експорт, скидання
settings-node-locale = Локаль
settings-node-about = Про програму

# §8.2 General → UI.
settings-ui-theme = Тема
settings-ui-run-bg = Працювати у фоні
settings-ui-show-tray = Показувати значок у лотку / рядку меню
settings-ui-single-click-tray = Одинарний клік у лотку / рядку меню
settings-ui-new-window-from-tray = Відкривати нове вікно зі значка в лотку
settings-ui-new-window-on-launch = Відкривати нове вікно під час запуску Freally
settings-ui-search-as-you-type = Пошук під час набору
settings-ui-select-on-mouse-click = Вибирати пошук кліком миші
settings-ui-focus-on-activate = Фокус на пошуку при активації
settings-ui-full-row-select = Вибір цілого рядка
settings-ui-single-click-open = Відкриття одинарним кліком
settings-ui-underline-titles = Підкреслювати підписи значків
settings-ui-row-density = Щільність результатів
settings-ui-row-density-compact = Компактна (32 пікс)
settings-ui-row-density-comfortable = Комфортна (44 пікс)
settings-ui-show-timing-badges = Показувати значки часу для кожної лінзи
settings-ui-anim-crossfade = Анімований перехід між темами

# §8.3 General → Home.
settings-home-match-case = Враховувати регістр
settings-home-match-whole-word = Слово цілком
settings-home-match-path = Збіг за шляхом
settings-home-match-diacritics = Враховувати діакритику
settings-home-match-regex = Збіг за регулярним виразом
settings-home-search = Пошук (власний типовий запит)
settings-home-filter = Фільтр
settings-home-sort = Сортування
settings-home-view = Вигляд
settings-home-index = Індекс
settings-home-default-lens-visibility = Типова видимість лінз
settings-home-default-lens-result-limits = Типові обмеження результатів лінз

# §8.4 General → Search.
settings-search-fast-ascii = Швидкий пошук ASCII
settings-search-mp-sep = Збіг за шляхом, коли запит містить роздільник шляху
settings-search-mw-fn = Збіг за повним ім’ям файлу під час використання шаблонів
settings-search-lit-ops = Дозволити літеральні оператори
settings-search-paren = Дозволити групування круглими дужками
settings-search-env = Розгортати змінні середовища
settings-search-fwd-slash = Замінювати прямі скісні риски зворотними
settings-search-precedence = Пріоритет операторів
settings-search-strict-everything = Строгий режим синтаксису Everything
settings-search-auto-regex = Автовиявлення регулярних виразів
settings-search-mod-comp = Доповнення модифікаторів
settings-search-parse-tree = Показувати дерево розбору при наведенні

# §8.5 General → Results.
settings-results-hide-empty = Приховувати результати за порожнього пошуку
settings-results-clear-on-search = Скидати вибір під час пошуку
settings-results-close-on-execute = Закривати вікно після виконання
settings-results-dbl-path = Відкривати шлях подвійним кліком у стовпці шляху
settings-results-auto-scroll = Автоматично прокручувати вигляд
settings-results-dquote-copy = Копіювати в подвійних лапках як шлях
settings-results-no-ext-rename = Не виділяти розширення під час перейменування
settings-results-sort-date-desc = Спершу сортувати за датою за спаданням
settings-results-sort-size-desc = Спершу сортувати за розміром за спаданням
settings-results-list-focus = Фокус списку результатів
settings-results-icon-prio = Пріоритет завантаження значків
settings-results-thumb-prio = Пріоритет завантаження мініатюр
settings-results-ext-prio = Пріоритет завантаження розширеної інформації
settings-results-group-by-lens = Групувати результати за лінзами
settings-results-snippet-inline = Показувати фрагмент попереднього перегляду в рядку

# §8.6 General → View.
settings-view-double-buffer = Подвійна буферизація
settings-view-alt-rows = Чергувати колір рядків
settings-view-row-mouseover = Підсвічувати рядок під курсором
settings-view-highlight-terms = Підсвічувати знайдені терміни
settings-view-status-show-selected = Показувати вибраний елемент у рядку стану
settings-view-rc-with-sel = Показувати кількість результатів разом із кількістю вибраних
settings-view-status-show-size = Показувати розмір у рядку стану
settings-view-tooltips = Показувати підказки
settings-view-update-on-scroll = Оновлювати показ одразу після прокручування
settings-view-size-format = Формат розміру
settings-view-selection-rect = Прямокутник виділення
settings-view-audio-badges = Показувати значки LUFS / кодека / тривалості в аудіорядках
settings-view-similarity-score = Показувати оцінку подібності MinHash у рядках подібності
settings-view-preview-pane = Панель попереднього перегляду

# §8.7 General → Context Menu.
settings-context-menu-visibility = Видимість
settings-context-menu-show = Показувати
settings-context-menu-shift = Показувати лише при утриманні Shift
settings-context-menu-hide = Приховувати
settings-context-menu-command = Макрос команди
settings-context-menu-open-folders = Відкрити (теки)
settings-context-menu-open-files = Відкрити (файли)
settings-context-menu-open-path = Відкрити шлях
settings-context-menu-explore = Огляд
settings-context-menu-explore-path = Оглянути шлях
settings-context-menu-copy-name = Копіювати ім’я в буфер обміну
settings-context-menu-copy-path = Копіювати шлях у буфер обміну
settings-context-menu-copy-full-name = Копіювати повне ім’я в буфер обміну
settings-context-menu-reveal = Показати в Freally
settings-context-menu-send-to = Надіслати до Freally (шлях)

# §8.8 General → Fonts & Colors.
settings-fc-font = Шрифт
settings-fc-size = Розмір
settings-fc-state-normal = Звичайний
settings-fc-state-highlighted = Підсвічений
settings-fc-state-current-sort = Поточне сортування
settings-fc-state-current-sort-h = Поточне сортування (підсвічене)
settings-fc-state-selected = Вибраний
settings-fc-state-selected-h = Вибраний (підсвічений)
settings-fc-state-inactive-selected = Неактивний вибраний
settings-fc-state-inactive-selected-h = Неактивний вибраний (підсвічений)
settings-fc-foreground = Передній план
settings-fc-background = Тло
settings-fc-bold = Жирний
settings-fc-italic = Курсив
settings-fc-default = Типовий
settings-fc-per-lens-accent = Акцент для кожної лінзи
settings-fc-theme-inherit = Автоматично інвертувати власні кольори при зміні теми

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Глобальна гаряча клавіша
settings-keyboard-new-window = Гаряча клавіша нового вікна
settings-keyboard-show-window = Гаряча клавіша показу вікна
settings-keyboard-toggle-window = Гаряча клавіша перемикання вікна
settings-keyboard-show-commands = Показувати команди, що містять
settings-keyboard-add-chord = + Додати акорд
settings-keyboard-remove-chord = Видалити

# §8.10 History.
settings-history-search-enable = Увімкнути історію пошуку
settings-history-search-keep = Зберігати історію пошуку { $days } днів
settings-history-run-enable = Увімкнути історію запусків
settings-history-run-keep = Зберігати історію запусків { $days } днів
settings-history-clear-now = Очистити зараз
settings-history-privacy-mode = Режим конфіденційності
settings-history-per-lens = Історія для кожної лінзи

# §8.11 Indexes (top-level).
settings-ix-database-location = Розташування бази даних
settings-ix-multiuser = Ім’я файлу багатокористувацької бази даних
settings-ix-compress = Стискати базу даних
settings-ix-recent-changes = Індексувати останні зміни
settings-ix-file-size = Індексувати розмір файлу
settings-ix-fast-size-sort = Швидке сортування за розміром
settings-ix-folder-size = Індексувати розмір теки
settings-ix-fast-folder-size-sort = Швидке сортування за розміром теки
settings-ix-date-created = Індексувати дату створення
settings-ix-fast-date-created = Швидке сортування за датою створення
settings-ix-date-modified = Індексувати дату змінення
settings-ix-fast-date-modified = Швидке сортування за датою змінення
settings-ix-date-accessed = Індексувати дату доступу
settings-ix-fast-date-accessed = Швидке сортування за датою доступу
settings-ix-attributes = Індексувати атрибути
settings-ix-fast-attributes = Швидке сортування за атрибутами
settings-ix-fast-path-sort = Швидке сортування за шляхом
settings-ix-fast-extension-sort = Швидке сортування за розширенням
settings-ix-force-rebuild = Примусова перебудова
settings-ix-compact = Ущільнити індекс
settings-ix-verify = Перевірити індекс
settings-ix-integrity-policy = Політика цілісності індексу
settings-ix-memory-budget = Бюджет пам’яті для індексатора
settings-ix-throttle = Обмеження фонового індексування

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Автоматично додавати нові фіксовані томи
settings-vol-auto-removable = Автоматично додавати нові знімні томи
settings-vol-auto-remove-offline = Автоматично вилучати недоступні томи
settings-vol-detected = Виявлені томи
settings-vol-include = Додати до індексу
settings-vol-include-only = Лише ці (glob/regex)
settings-vol-enable-usn = Увімкнути журнал USN
settings-vol-enable-fsevents = Увімкнути потік FSEvents
settings-vol-enable-inotify = Увімкнути inotify (або fanotify за підвищених прав)
settings-vol-buffer = Розмір буфера журналу (KB)
settings-vol-allocation-delta = Дельта розподілу (KB)
settings-vol-load-recent = Завантажувати останні зміни з журналу під час запуску
settings-vol-monitor = Відстежувати зміни
settings-vol-recreate-journal = Перестворити журнал
settings-vol-reset-stream = Скинути потік FSEvents
settings-vol-upgrade-fanotify = Оновити до fanotify (polkit)
settings-vol-remove = Видалити

# §8.13 Indexes → Folders.
settings-folders-watched = Відстежувані теки
settings-folders-add = Додати…
settings-folders-rescan-now = Пересканувати зараз
settings-folders-rescan-all = Пересканувати все зараз
settings-folders-monitor = Намагатися відстежувати зміни
settings-folders-buffer = Розмір буфера
settings-folders-rescan-on-full = Пересканувати за повного буфера

# §8.14 Indexes → File Lists.
settings-flists-add = Додати…
settings-flists-monitor = Відстежувати зміни
settings-flists-editor = Редактор списку файлів…
settings-flists-format = Формат списку файлів
settings-flists-format-text = Текст (один шлях на рядок)
settings-flists-format-json = JSON (з метаданими)
settings-flists-format-srcb = Пакунок Freally (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Виключати приховані файли й теки
settings-exclude-system = Виключати системні файли й теки
settings-exclude-list-en = Увімкнути список виключень
settings-exclude-folders = Виключати теки
settings-exclude-include-only-files = Лише ці файли (glob)
settings-exclude-files = Виключати файли (glob)
settings-exclude-os-recommended = Застосувати рекомендовані ОС виключення
settings-exclude-by-class = Виключати за класом розширення

# §8.16 Lenses → Filename.
settings-lf-trigram = Агресивність триграмного попереднього фільтра
settings-lf-suffix-mem = Бюджет пам’яті суфіксного масиву
settings-lf-wildcard-limit = Ліміт розгортання шаблонів
settings-lf-regex-timeout = Тайм-аут регулярного виразу

# §8.17 Lenses → Content.
settings-lc-enable = Увімкнути лінзу вмісту
settings-lc-time-budget = Бюджет часу на документ
settings-lc-mem-ceiling = Ліміт пам’яті на документ
settings-lc-snippet-len = Довжина фрагмента
settings-lc-stop-words = Мова стоп-слів
settings-lc-re-extract = Повторне видобування при зміні параметрів
settings-lc-verify-blobs = Перевіряти контрольні суми блобів видобутого тексту під час читання

# §8.18 Lenses → Audio.
settings-la-enable = Увімкнути аудіолінзу
settings-la-lufs-ref = Еталонний стандарт LUFS
settings-la-peak-compute = Обчислювати пік через
settings-la-silence-thresh = Поріг тиші
settings-la-re-extract-modify = Повторне видобування при події змінення

# §8.19 Lenses → Similarity.
settings-ls-enable = Увімкнути лінзу подібності
settings-ls-sig-size = Розмір сигнатури MinHash (k)
settings-ls-bands = Смуги LSH
settings-ls-recall = Поріг повноти
settings-ls-result-cap = Обмеження результатів

# §8.20 Lenses → Custom.
settings-custom-registry = Реєстр
settings-custom-trust = Довіра
settings-custom-refresh-hashes = Оновити хеші

# §8.21-§8.22 Network.
settings-net-https-enable = Увімкнути сервер HTTPS
settings-net-bind = Прив’язати до інтерфейсів
settings-net-port = Слухати порт
settings-net-force-https = Примусовий HTTPS
settings-net-legacy-auth = Застаріла базова автентифікація HTTP
settings-net-token-regen = Перегенерувати токен
settings-net-api-enable = Увімкнути сервер API
settings-net-legacy-ftp = Підтримка застарілого простого FTP/ETP

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Автооновлення
settings-privacy-prerelease = Канал попередніх версій
settings-privacy-network-policy = Політика мережевих запитів

# §8.24 Logs & Debug.
settings-logs-level = Рівень журналювання
settings-logs-location = Розташування файлу журналу
settings-logs-retention = Зберігання журналів
settings-logs-debug-overlay = Показувати накладку налагодження
settings-logs-open-folder = Відкрити теку журналів
settings-logs-export-bundle = Експортувати пакунок діагностики

# §8.25 Backup, Export, Reset.
settings-backup-export = Експортувати параметри
settings-backup-import = Імпортувати параметри
settings-backup-export-bookmarks = Експортувати пакунок закладок
settings-backup-import-bookmarks = Імпортувати пакунок закладок
settings-backup-reset-all = Скинути всі параметри до типових

# §8.26 Locale.
settings-locale-current = Поточна локаль
settings-locale-rtl-preview = Перегляд RTL
settings-locale-date-format = Формат дати
settings-locale-number-format = Формат чисел

# §8.27 About.
settings-about-version = Freally { $version }
settings-about-license = Ліцензія
settings-about-credits = Подяки
settings-about-notices = Повідомлення про відкритий код

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = Майстер першого запуску
wizard-step-of-total = Крок { $step } з { $total }
wizard-roots-hint = Додайте теки або томи, які Freally має відстежувати. Це можна змінити пізніше в параметрах індексів.
wizard-browse = Огляд…
wizard-roots-placeholder = …або вставте шлях
wizard-roots-add = Додати
wizard-roots-remove = Видалити
wizard-roots-empty = Кореневих тек ще не налаштовано.
wizard-locale-hint = Freally постачається 18 мовами. Перемкнути можна пізніше.
wizard-theme-hint = «Системна» враховує налаштування вигляду вашої ОС.
wizard-back = Назад
wizard-next = Далі

# Status bar polish.
statusbar-hotkey-hint = Гаряча клавіша: { $hotkey }
statusbar-cycle-theme = Перемкнути тему
statusbar-indexed-suffix = проіндексовано

# Results / lenses.
lens-expand = Розгорнути лінзу
lens-collapse = Згорнути лінзу
lens-no-matches = У цій лінзі немає збігів.

# Preview pane.
preview-header = Попередній перегляд
preview-loading = Завантаження…
preview-select-file = Виберіть файл для перегляду.
preview-unavailable = Перегляд недоступний

# Bookmarks.
bookmarks-label = ★ Закладки
bookmarks-empty-hint = Закладок ще немає. Натисніть Ctrl+D, щоб зберегти поточний запит.
bookmarks-organize-title = Упорядкувати закладки
bookmarks-organize-empty = Закладок ще немає.
bookmarks-rename = Перейменувати
bookmarks-close = Закрити

# Settings tree extras.
settings-group-history = Історія
settings-group-privacy = Конфіденційність і оновлення
settings-group-logs = Журнали й налагодження
settings-group-backup = Резервне копіювання, експорт, скидання
settings-tree-custom-lens = Власна
settings-unsaved-changes = незбережені зміни

# About dialog.
about-dialog-title = Freally
about-copyright = Авторське право © 2026 Mike Weaver. Усі права захищено.
about-close = Закрити

# Connect endpoint dialog.
connect-ftp-title = Підключення до сервера FTP
connect-ftp-host = Хост:
connect-ftp-port = Порт:
connect-ftp-username = Ім’я користувача:
connect-ftp-password = Пароль:
connect-ftp-link-type = Тип з’єднання:

# UI panel.
ui-hint = Тема, інтеграція з лотком / рядком меню, пошук під час набору, щільність рядків. Повна відповідність voidtools-Everything плюс доповнення Freally, позначені (+).
ui-section-theme = Тема
ui-theme-system-default = Системна (типово)
ui-section-tray = Лоток / рядок меню
ui-section-search-behavior = Поведінка пошуку
ui-section-result-rows = Рядки результатів
ui-single-click-system-default = Системні налаштування (типово)
ui-single-click-always = Завжди одинарний клік
ui-single-click-always-double = Завжди подвійний клік
ui-underline-always = Завжди
ui-underline-on-hover = При наведенні
ui-underline-never = Ніколи

# Home panel.
home-hint = Типові значення завантажуються під час запуску застосунку — кожен список може зберігати «Останнє значення» або фіксувати задане. Видимість лінз / обмеження результатів — доповнення Freally (+).
home-section-match = Типові збіги
home-section-search-sort = Типові пошук і сортування
home-search-placeholder = Типово порожньо
home-section-index = Джерело індексу
home-file-list-path = Шлях до списку файлів
home-https-endpoint = URL кінцевої точки API HTTPS
home-endpoint-token = Токен (показано відбиток)

# Backup panel.
backup-section-settings = Параметри (+)
backup-section-bookmarks = Закладки + власні екстрактори (+)
backup-section-reset = Скидання
backup-toast-exported = Параметри експортовано до { $path }
backup-toast-export-failed = Не вдалося експортувати: { $error }
backup-toast-imported = Параметри імпортовано
backup-toast-import-failed = Не вдалося імпортувати: { $error }
backup-toast-bookmarks-exported = Закладки експортовано
backup-toast-bookmarks-export-failed = Не вдалося експортувати закладки: { $error }
backup-toast-bookmarks-imported = Закладки імпортовано
backup-toast-bookmarks-import-failed = Не вдалося імпортувати закладки: { $error }
backup-confirm-reset = Скинути всі параметри до типових? Це не можна скасувати (діалог залишиться відкритим).
backup-toast-reset = Усі параметри скинуто

# Keyboard panel.
keyboard-section-global = Глобальні гарячі клавіші
keyboard-placeholder-example = Super+Space
keyboard-section-commands = Команди
keyboard-placeholder-command = ідентифікатор команди (напр. file.export_results)
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = Історія пошуку
history-section-run = Історія запусків
history-section-privacy = Конфіденційність (+)
history-record-filename = Записувати історію лінзи імен файлів
history-record-content = Записувати історію лінзи вмісту
history-record-audio = Записувати історію аудіолінзи
history-record-similarity = Записувати історію лінзи подібності

# Locale panel.
locale-section-language = Мова (+)
locale-section-time-date = Час / дата (+)
locale-date-os = Типово для ОС
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = Власний
locale-date-custom-format = Власний формат
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = Числа (+)
locale-number-os = Типово для ОС
locale-number-custom = Власний
locale-thousands-sep = Роздільник тисяч
locale-decimal-sep = Десятковий роздільник

# Folders panel.
folders-hint = Додаткові відстежувані теки понад типові томи.
folders-list-title = Відстежувані теки
folders-empty = Тек ще не додано.
folders-remove = Видалити
folders-section-title-dynamic = Параметри для { $path }
folders-section-schedule = Розклад пересканування
folders-schedule-daily = Щодня о HH:MM
folders-schedule-hours = Кожні N годин
folders-schedule-never = Ніколи
folders-hour = Година
folders-minute = Хвилина
folders-hours = Години
folders-id-label = Ідентифікатор теки (лише для читання)
folders-select-prompt = Виберіть теку, щоб налаштувати її.
folders-section-extras = Додатки Freally (+)
folders-extras-note = Пересканування після виходу зі сну ввімкнено типово в цій збірці; перемикач приєднається до елементів керування рівня теки під час шліфування у фазі 13.

# Volumes panel.
volumes-hint = Кросплатформний аналог панелей NTFS / ReFS у voidtools-Everything. Автоматично виявляє NTFS / ReFS / exFAT / FAT32 (Win), APFS / HFS+ (macOS), ext4 / Btrfs / ZFS / XFS / F2FS (Linux).
volumes-section-auto-include = Автододавання
volumes-list-title = Виявлені томи
volumes-detecting = Виявлення…
volumes-empty = Томів не виявлено.
volumes-select-prompt = Виберіть том, щоб налаштувати його.

# About panel polish.
about-section-version = Версія (+)
about-section-license = Ліцензія (+)
about-license-text = Mike Weaver — усі права захищено. Це пропрієтарне програмне забезпечення.
about-license-spdx = SPDX: { $spdx }
about-section-credits = Подяки (+)
about-credits-inspired = Натхненно Everything від voidtools.
about-credits-voidtools = voidtools.com
about-credits-repo = Репозиторій проєкту

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = Містить команди для роботи з Freally.
menu-file-new-window = Нове вікно пошуку
menu-file-open-list = Відкрити список файлів…
menu-file-close-list = Закрити список файлів
menu-file-close = Закрити
menu-file-export-results = Експортувати результати…
menu-file-export-bundle = Експортувати пакунок індексу…
menu-file-exit = Вийти

# Edit menu.
menu-edit-hint = Містить команди для редагування результатів пошуку.
menu-edit-cut = Вирізати
menu-edit-copy = Копіювати
menu-edit-paste = Вставити
menu-edit-copy-to-folder = Копіювати до теки…
menu-edit-move-to-folder = Перемістити до теки…
menu-edit-select-all = Виділити все
menu-edit-invert-selection = Інвертувати виділення
menu-edit-advanced = Додатково
menu-edit-copy-full-name = Копіювати повне ім’я
menu-edit-copy-path = Копіювати шлях
menu-edit-copy-filename = Копіювати ім’я файлу
menu-edit-copy-as-json = Копіювати як JSON
menu-edit-copy-with-metadata = Копіювати з метаданими
menu-edit-copy-as-bundle-ref = Копіювати як посилання на пакунок Freally

# View menu.
menu-view-hint = Містить команди для керування виглядом.
menu-view-filters = Фільтри
menu-view-preview = Попередній перегляд
menu-view-status-bar = Рядок стану
menu-view-thumbs-xl = Дуже великі мініатюри
menu-view-thumbs-l = Великі мініатюри
menu-view-thumbs-m = Середні мініатюри
menu-view-details = Подробиці
menu-view-window-size = Розмір вікна
menu-view-window-size-hint = Містить команди для регулювання розміру вікна.
menu-view-window-small = Малий
menu-view-window-medium = Середній
menu-view-window-large = Великий
menu-view-window-auto = Автопідбір
menu-view-zoom = Масштаб
menu-view-zoom-hint = Містить команди для регулювання розміру шрифту та значків.
menu-view-zoom-in = Збільшити
menu-view-zoom-out = Зменшити
menu-view-zoom-reset = Скинути
menu-view-sort-by = Сортувати за
menu-view-sort-by-hint = Містить команди для сортування списку результатів.
menu-view-sort-name = Ім’я
menu-view-sort-path = Шлях
menu-view-sort-size = Розмір
menu-view-sort-ext = Розширення
menu-view-sort-type = Тип
menu-view-sort-modified = Дата змінення
menu-view-sort-created = Дата створення
menu-view-sort-accessed = Дата доступу
menu-view-sort-attributes = Атрибути
menu-view-sort-recently-changed = Дата останньої зміни
menu-view-sort-run-count = Кількість запусків
menu-view-sort-run-date = Дата запуску
menu-view-sort-file-list-filename = Ім’я файлу списку файлів
menu-view-sort-lufs = LUFS
menu-view-sort-length = Тривалість
menu-view-sort-similarity = Оцінка подібності
menu-view-sort-asc = За зростанням
menu-view-sort-desc = За спаданням
menu-view-go-to = Перейти до
menu-view-refresh = Оновити
menu-view-theme = Тема
menu-view-theme-hint = Перемикання між системною, світлою або темною темами.
menu-view-lenses = Лінзи
menu-view-lenses-hint = Перемикання видимості кожної лінзи в списку результатів.
menu-view-on-top = Поверх інших
menu-view-on-top-hint = Містить команди для утримання цього вікна поверх інших вікон.
menu-view-on-top-never = Ніколи
menu-view-on-top-always = Завжди
menu-view-on-top-while-searching = Під час пошуку

# Search menu.
menu-search-hint = Містить перемикачі пошуку.
menu-search-match-case = Враховувати регістр
menu-search-match-whole-word = Слово цілком
menu-search-match-path = Збіг за шляхом
menu-search-match-diacritics = Враховувати діакритику
menu-search-enable-regex = Увімкнути регулярні вирази
menu-search-advanced = Розширений пошук…
menu-search-add-to-filters = Додати до фільтрів…
menu-search-organize-filters = Упорядкувати фільтри…
menu-search-filter-everything = Усе
menu-search-filter-archive = Стиснені (архів)
menu-search-filter-folder = Тека
menu-search-filter-custom = Власний фільтр…

# Bookmarks menu.
menu-bookmarks-hint = Містить команди для роботи із закладками.
menu-bookmarks-add = Додати до закладок
menu-bookmarks-organize = Упорядкувати закладки…

# Tools menu.
menu-tools-hint = Містить команди інструментів.
menu-tools-connect = Підключитися до сервера FTP…
menu-tools-disconnect = Відключитися від сервера FTP
menu-tools-file-list-editor = Редактор списку файлів…
menu-tools-index-maintenance = Обслуговування індексу
menu-tools-index-maintenance-hint = Інструменти обслуговування індексу.
menu-tools-verify-index = Перевірити індекс…
menu-tools-compact-index = Ущільнити індекс…
menu-tools-rebuild-index = Примусова перебудова індексу…
menu-tools-custom-extractor = Менеджер власних екстракторів…
menu-tools-custom-extractor-hint = Керування власними екстракторами в пісочниці Wasm.
menu-tools-options = Параметри…

# Help menu.
menu-help-hint = Містить команди довідки.
menu-help-help = Довідка Freally
menu-help-search-syntax = Синтаксис пошуку
menu-help-regex-syntax = Синтаксис регулярних виразів
menu-help-audio-ref = Довідник модифікаторів аудіо
menu-help-similarity-ref = Довідник модифікаторів подібності
menu-help-cli-options = Параметри командного рядка
menu-help-website = Вебсайт Freally
menu-help-check-updates = Перевірити оновлення…
menu-help-sponsor = Підтримати / пожертвувати
menu-help-about = Про Freally…

# Result column headers (short forms used in the table header row).
column-name = Ім’я
column-path = Шлях
column-size = Розмір
column-modified = Змінено
column-type = Тип
column-ext = Розш.
column-sort-by = Сортувати за { $name }
column-resize = Змінити розмір стовпця { $name }

# Section subtitle bars used inside multiple settings panels.
section-behavior = Поведінка
section-rendering = Рендеринг
section-status-bar = Рядок стану
section-display-format = Формат показу
section-loading-priority = Пріоритет завантаження
section-compatibility = Сумісність
section-storage = Сховище
section-index-fields = Поля індексу
section-maintenance = Обслуговування
section-logging = Журналювання
section-tools = Інструменти
section-privacy = Конфіденційність
section-auto-update = Автооновлення (+)
section-bind = Прив’язка
section-lens = Лінза
section-budgets = Бюджети
section-other = Інше
section-per-format-mode = Режим за форматом
section-loudness = Гучність
section-tuning = Налаштування (+)
section-minhash-lsh = Параметри MinHash + LSH (+)
section-top-level = Верхній рівень
section-file-globs = Шаблони файлів
section-file-list-settings = Параметри вибраного списку файлів
section-editor-format = Редактор + формат (E + +)
section-api-server = Сервер API (E адаптовано)
section-freally-extras = Додатки Freally (+)
section-freally-additions = Доповнення Freally (+)
section-freally-extensions = Розширення Freally (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = Останнє значення
opt-use-last-value-default = Останнє значення (типово)
opt-low = Низький
opt-normal-default = Звичайний (типово)
opt-high = Високий
opt-disabled = Вимкнено
opt-off = Вимкнено
opt-on-battery = Від акумулятора
opt-always = Завжди
opt-clamp-default = Обмежувати (типово)
opt-wrap = Переносити
opt-none = Немає
opt-strict-refuse = Строго (відхиляти запити за пошкодження)
opt-lenient-warn = Поблажливо (попереджати, але виконувати запит)
opt-system-default = Типово для системи
opt-drag-select = Виділення перетягуванням
opt-auto-binary = Авто (двійковий)
opt-auto-decimal = Авто (десятковий)

# Unit suffixes shown next to number inputs.
unit-days = днів
unit-b = B
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-tb = TB

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = Активний
opt-lazy-default = Лінивий (типово)
opt-on = Увімкнено
opt-on-default = Увімкнено (типово)
opt-all = Усі
opt-weekly = Щотижня
opt-monthly = Щомісяця
opt-name-asc = Ім’я за зростанням
opt-name-desc = Ім’я за спаданням
opt-size-asc = Розмір за зростанням
opt-size-desc = Розмір за спаданням
opt-modified-asc = Дата змінення за зростанням
opt-modified-desc = Дата змінення за спаданням
opt-compact = Компактний
opt-comfortable = Комфортний
opt-details = Подробиці
opt-thumbnails = Мініатюри
opt-local-db-default = Локальна база даних (типово)
opt-file-list = Список файлів
opt-https-endpoint = Кінцева точка API HTTPS
opt-right-default = Праворуч (типово)
opt-bottom = Знизу
opt-or-and-default = OR > AND (типово)
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128 (типово)
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = Broadcast film (-23)
opt-true-peak = True peak (4× передискретизація, типово)
opt-sample-peak = Sample peak
opt-auto-per-doc = Авто (на документ)
opt-log-error = Error
opt-log-warn = Warn
opt-log-info-default = Info (типово)
opt-log-debug = Debug
opt-log-trace = Trace

# More Freally apps (Central inside panel) — host chrome
menu-help-more-apps = Інші застосунки Freally…
moreapps-title = Інші застосунки Freally
