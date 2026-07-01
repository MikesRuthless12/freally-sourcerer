# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = Jedno wyszukiwanie. Każde źródło. Każdy system.
window-title = Freally Sourcerer
search-placeholder = Szukaj…
about-version = Wersja { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = Gotowe
status-indexed = Zindeksowano ({ $count } plików)
status-indexing = Indeksowanie… { $done }/{ $total }
status-paused = Wstrzymano
status-error = Błąd
status-result-count-one = { $count } wynik
status-result-count-many = { $count } wyników
status-selection = · zaznaczono { $count }
status-selection-size = Zaznaczono: { $size }
status-query-timing = Zapytanie: { $ms } ms
status-endpoint-local = Lokalna baza
status-endpoint-remote = API: { $name }

menu-file = Plik
menu-edit = Edycja
menu-view = Widok
menu-search = Szukaj
menu-bookmarks = Zakładki
menu-tools = Narzędzia
menu-help = Pomoc

theme-system = Systemowy
theme-light = Jasny
theme-dark = Ciemny

lens-filename = Nazwa pliku
lens-content = Treść
lens-audio = Dźwięk
lens-similarity = Podobieństwo

parse-error-empty = Wpisz zapytanie, aby rozpocząć.
parse-error-unknown = Nierozpoznana składnia w tym miejscu.

action-open = Otwórz
action-reveal = Pokaż w folderze
action-copy-path = Kopiuj ścieżkę
action-copy-name = Kopiuj nazwę
action-delete = Usuń

quick-filter-audio = Dźwięk
quick-filter-video = Wideo
quick-filter-image = Obraz
quick-filter-document = Dokument
quick-filter-executable = Plik wykonywalny
quick-filter-archive = Archiwum

wizard-title = Witamy w Freally
wizard-step-roots = Wybierz, co indeksować
wizard-step-hotkey = Wybierz skrót globalny
wizard-step-locale = Wybierz język
wizard-step-theme = Wybierz motyw
wizard-finish = Zakończ

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = Opcje
settings-search-placeholder = Szukaj opcji…
settings-restore-defaults = Przywróć domyślne
settings-ok = OK
settings-cancel = Anuluj
settings-apply = Zastosuj

# Tree nav groups (PRD §8.1.1).
settings-group-general = Ogólne
settings-group-indexes = Indeksy
settings-group-lenses = Soczewki
settings-group-network = Sieć

# Tree nav leaves.
settings-node-ui = Interfejs
settings-node-home = Start
settings-node-search = Szukaj
settings-node-results = Wyniki
settings-node-view = Widok
settings-node-context-menu = Menu kontekstowe
settings-node-fonts-colors = Czcionki i kolory
settings-node-keyboard = Klawiatura
settings-node-history = Historia
settings-node-indexes-top = (najwyższy poziom)
settings-node-volumes = Woluminy
settings-node-folders = Foldery
settings-node-file-lists = Listy plików
settings-node-exclude = Wykluczenia
settings-node-https-server = Serwer HTTP / HTTPS
settings-node-etp-api = API ETP / FTP
settings-node-privacy = Prywatność i aktualizacje
settings-node-logs = Dzienniki i debugowanie
settings-node-backup = Kopia zapasowa, eksport, reset
settings-node-locale = Język
settings-node-about = O programie

# §8.2 General → UI.
settings-ui-theme = Motyw
settings-ui-run-bg = Działaj w tle
settings-ui-show-tray = Pokaż ikonę w zasobniku / pasku menu
settings-ui-single-click-tray = Pojedyncze kliknięcie zasobnika / paska menu
settings-ui-new-window-from-tray = Otwórz nowe okno z ikony zasobnika
settings-ui-new-window-on-launch = Otwieraj nowe okno przy uruchamianiu Freally
settings-ui-search-as-you-type = Szukaj podczas pisania
settings-ui-select-on-mouse-click = Zaznacz wyszukiwanie po kliknięciu myszą
settings-ui-focus-on-activate = Aktywuj pole wyszukiwania po włączeniu
settings-ui-full-row-select = Zaznaczanie całego wiersza
settings-ui-single-click-open = Otwieranie pojedynczym kliknięciem
settings-ui-underline-titles = Podkreślaj tytuły ikon
settings-ui-row-density = Gęstość wyników
settings-ui-row-density-compact = Kompaktowa (32 px)
settings-ui-row-density-comfortable = Komfortowa (44 px)
settings-ui-show-timing-badges = Pokaż plakietki czasu dla każdej soczewki
settings-ui-anim-crossfade = Animowane przejście motywu

# §8.3 General → Home.
settings-home-match-case = Uwzględniaj wielkość liter
settings-home-match-whole-word = Dopasuj całe słowo
settings-home-match-path = Dopasuj ścieżkę
settings-home-match-diacritics = Uwzględniaj znaki diakrytyczne
settings-home-match-regex = Dopasuj wyrażenie regularne
settings-home-search = Szukaj (własne domyślne zapytanie)
settings-home-filter = Filtr
settings-home-sort = Sortowanie
settings-home-view = Widok
settings-home-index = Indeks
settings-home-default-lens-visibility = Domyślna widoczność soczewek
settings-home-default-lens-result-limits = Domyślne limity wyników soczewek

# §8.4 General → Search.
settings-search-fast-ascii = Szybkie wyszukiwanie ASCII
settings-search-mp-sep = Dopasuj ścieżkę, gdy termin zawiera separator ścieżki
settings-search-mw-fn = Dopasuj całą nazwę pliku przy użyciu symboli wieloznacznych
settings-search-lit-ops = Zezwalaj na operatory literalne
settings-search-paren = Zezwalaj na grupowanie nawiasami okrągłymi
settings-search-env = Rozwijaj zmienne środowiskowe
settings-search-fwd-slash = Zamieniaj ukośniki na ukośniki wsteczne
settings-search-precedence = Pierwszeństwo operatorów
settings-search-strict-everything = Tryb ścisłej składni Everything
settings-search-auto-regex = Automatycznie wykrywaj wyrażenia regularne
settings-search-mod-comp = Uzupełnianie modyfikatorów
settings-search-parse-tree = Pokaż drzewo składni po najechaniu

# §8.5 General → Results.
settings-results-hide-empty = Ukryj wyniki, gdy wyszukiwanie jest puste
settings-results-clear-on-search = Czyść zaznaczenie przy wyszukiwaniu
settings-results-close-on-execute = Zamknij okno po wykonaniu
settings-results-dbl-path = Otwórz ścieżkę dwukrotnym kliknięciem w kolumnie ścieżki
settings-results-auto-scroll = Automatycznie przewijaj widok
settings-results-dquote-copy = Kopiuj w cudzysłowie jako ścieżkę
settings-results-no-ext-rename = Nie zaznaczaj rozszerzenia podczas zmiany nazwy
settings-results-sort-date-desc = Najpierw sortuj daty malejąco
settings-results-sort-size-desc = Najpierw sortuj rozmiary malejąco
settings-results-list-focus = Fokus listy wyników
settings-results-icon-prio = Priorytet wczytywania ikon
settings-results-thumb-prio = Priorytet wczytywania miniatur
settings-results-ext-prio = Priorytet wczytywania informacji rozszerzonych
settings-results-group-by-lens = Grupuj wyniki według soczewki
settings-results-snippet-inline = Pokaż podgląd fragmentu w wierszu

# §8.6 General → View.
settings-view-double-buffer = Podwójne buforowanie
settings-view-alt-rows = Naprzemienny kolor wierszy
settings-view-row-mouseover = Pokaż podświetlenie wiersza po najechaniu
settings-view-highlight-terms = Pokaż podświetlone wyszukiwane terminy
settings-view-status-show-selected = Pokaż zaznaczony element na pasku stanu
settings-view-rc-with-sel = Pokaż liczbę wyników wraz z liczbą zaznaczonych
settings-view-status-show-size = Pokaż rozmiar na pasku stanu
settings-view-tooltips = Pokaż podpowiedzi
settings-view-update-on-scroll = Aktualizuj widok natychmiast po przewinięciu
settings-view-size-format = Format rozmiaru
settings-view-selection-rect = Prostokąt zaznaczenia
settings-view-audio-badges = Pokaż plakietki LUFS / kodeka / długości w wierszach dźwięku
settings-view-similarity-score = Pokaż wynik podobieństwa MinHash w wierszach podobieństwa
settings-view-preview-pane = Panel podglądu

# §8.7 General → Context Menu.
settings-context-menu-visibility = Widoczność
settings-context-menu-show = Pokaż
settings-context-menu-shift = Pokaż tylko z wciśniętym Shift
settings-context-menu-hide = Ukryj
settings-context-menu-command = Makro polecenia
settings-context-menu-open-folders = Otwórz (foldery)
settings-context-menu-open-files = Otwórz (pliki)
settings-context-menu-open-path = Otwórz ścieżkę
settings-context-menu-explore = Eksploruj
settings-context-menu-explore-path = Eksploruj ścieżkę
settings-context-menu-copy-name = Kopiuj nazwę do schowka
settings-context-menu-copy-path = Kopiuj ścieżkę do schowka
settings-context-menu-copy-full-name = Kopiuj pełną nazwę do schowka
settings-context-menu-reveal = Pokaż w Freally
settings-context-menu-send-to = Wyślij do Freally (ścieżka)

# §8.8 General → Fonts & Colors.
settings-fc-font = Czcionka
settings-fc-size = Rozmiar
settings-fc-state-normal = Normalny
settings-fc-state-highlighted = Podświetlony
settings-fc-state-current-sort = Bieżące sortowanie
settings-fc-state-current-sort-h = Bieżące sortowanie (podświetlone)
settings-fc-state-selected = Zaznaczony
settings-fc-state-selected-h = Zaznaczony (podświetlony)
settings-fc-state-inactive-selected = Nieaktywny zaznaczony
settings-fc-state-inactive-selected-h = Nieaktywny zaznaczony (podświetlony)
settings-fc-foreground = Pierwszy plan
settings-fc-background = Tło
settings-fc-bold = Pogrubienie
settings-fc-italic = Kursywa
settings-fc-default = Domyślny
settings-fc-per-lens-accent = Akcent dla soczewki
settings-fc-theme-inherit = Automatycznie odwracaj własne kolory przy zmianie motywu

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Skrót globalny
settings-keyboard-new-window = Skrót nowego okna
settings-keyboard-show-window = Skrót pokazania okna
settings-keyboard-toggle-window = Skrót przełączania okna
settings-keyboard-show-commands = Pokaż polecenia zawierające
settings-keyboard-add-chord = + Dodaj akord
settings-keyboard-remove-chord = Usuń

# §8.10 History.
settings-history-search-enable = Włącz historię wyszukiwania
settings-history-search-keep = Przechowuj historię wyszukiwania przez { $days } dni
settings-history-run-enable = Włącz historię uruchomień
settings-history-run-keep = Przechowuj historię uruchomień przez { $days } dni
settings-history-clear-now = Wyczyść teraz
settings-history-privacy-mode = Tryb prywatny
settings-history-per-lens = Historia dla każdej soczewki

# §8.11 Indexes (top-level).
settings-ix-database-location = Lokalizacja bazy danych
settings-ix-multiuser = Nazwa pliku bazy wielu użytkowników
settings-ix-compress = Kompresuj bazę danych
settings-ix-recent-changes = Indeksuj ostatnie zmiany
settings-ix-file-size = Indeksuj rozmiar pliku
settings-ix-fast-size-sort = Szybkie sortowanie według rozmiaru
settings-ix-folder-size = Indeksuj rozmiar folderu
settings-ix-fast-folder-size-sort = Szybkie sortowanie według rozmiaru folderu
settings-ix-date-created = Indeksuj datę utworzenia
settings-ix-fast-date-created = Szybkie sortowanie według daty utworzenia
settings-ix-date-modified = Indeksuj datę modyfikacji
settings-ix-fast-date-modified = Szybkie sortowanie według daty modyfikacji
settings-ix-date-accessed = Indeksuj datę dostępu
settings-ix-fast-date-accessed = Szybkie sortowanie według daty dostępu
settings-ix-attributes = Indeksuj atrybuty
settings-ix-fast-attributes = Szybkie sortowanie według atrybutów
settings-ix-fast-path-sort = Szybkie sortowanie według ścieżki
settings-ix-fast-extension-sort = Szybkie sortowanie według rozszerzenia
settings-ix-force-rebuild = Wymuś przebudowę
settings-ix-compact = Kompaktuj indeks
settings-ix-verify = Zweryfikuj indeks
settings-ix-integrity-policy = Zasady integralności indeksu
settings-ix-memory-budget = Budżet pamięci indeksera
settings-ix-throttle = Ograniczanie indeksowania w tle

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Automatycznie dołączaj nowe woluminy stałe
settings-vol-auto-removable = Automatycznie dołączaj nowe woluminy wymienne
settings-vol-auto-remove-offline = Automatycznie usuwaj woluminy offline
settings-vol-detected = Wykryte woluminy
settings-vol-include = Dołącz do indeksu
settings-vol-include-only = Dołącz tylko (glob/regex)
settings-vol-enable-usn = Włącz dziennik USN
settings-vol-enable-fsevents = Włącz strumień FSEvents
settings-vol-enable-inotify = Włącz inotify (lub fanotify z podwyższonymi uprawnieniami)
settings-vol-buffer = Rozmiar bufora dziennika (KB)
settings-vol-allocation-delta = Delta alokacji (KB)
settings-vol-load-recent = Wczytaj ostatnie zmiany z dziennika przy starcie
settings-vol-monitor = Monitoruj zmiany
settings-vol-recreate-journal = Odtwórz dziennik
settings-vol-reset-stream = Zresetuj strumień FSEvents
settings-vol-upgrade-fanotify = Przejdź na fanotify (polkit)
settings-vol-remove = Usuń

# §8.13 Indexes → Folders.
settings-folders-watched = Obserwowane foldery
settings-folders-add = Dodaj…
settings-folders-rescan-now = Skanuj ponownie teraz
settings-folders-rescan-all = Skanuj ponownie wszystkie teraz
settings-folders-monitor = Próbuj monitorować zmiany
settings-folders-buffer = Rozmiar bufora
settings-folders-rescan-on-full = Skanuj ponownie przy pełnym buforze

# §8.14 Indexes → File Lists.
settings-flists-add = Dodaj…
settings-flists-monitor = Monitoruj zmiany
settings-flists-editor = Edytor list plików…
settings-flists-format = Format listy plików
settings-flists-format-text = Tekst (jedna ścieżka w wierszu)
settings-flists-format-json = JSON (z metadanymi)
settings-flists-format-srcb = Pakiet Freally (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Wyklucz ukryte pliki i foldery
settings-exclude-system = Wyklucz pliki i foldery systemowe
settings-exclude-list-en = Włącz listę wykluczeń
settings-exclude-folders = Wyklucz foldery
settings-exclude-include-only-files = Dołącz tylko pliki (glob)
settings-exclude-files = Wyklucz pliki (glob)
settings-exclude-os-recommended = Zastosuj wykluczenia zalecane przez system
settings-exclude-by-class = Wyklucz według klasy rozszerzenia

# §8.16 Lenses → Filename.
settings-lf-trigram = Agresywność wstępnego filtru trygramów
settings-lf-suffix-mem = Budżet pamięci tablicy sufiksów
settings-lf-wildcard-limit = Limit rozwijania symboli wieloznacznych
settings-lf-regex-timeout = Limit czasu wyrażeń regularnych

# §8.17 Lenses → Content.
settings-lc-enable = Włącz soczewkę treści
settings-lc-time-budget = Budżet czasu na dokument
settings-lc-mem-ceiling = Limit pamięci na dokument
settings-lc-snippet-len = Długość fragmentu
settings-lc-stop-words = Język słów pomijanych
settings-lc-re-extract = Wyodrębnij ponownie przy zmianie ustawień
settings-lc-verify-blobs = Weryfikuj sumy kontrolne wyodrębnionego tekstu przy odczycie

# §8.18 Lenses → Audio.
settings-la-enable = Włącz soczewkę dźwięku
settings-la-lufs-ref = Standard odniesienia LUFS
settings-la-peak-compute = Oblicz szczyt przez
settings-la-silence-thresh = Próg ciszy
settings-la-re-extract-modify = Wyodrębnij ponownie przy zdarzeniu modyfikacji

# §8.19 Lenses → Similarity.
settings-ls-enable = Włącz soczewkę podobieństwa
settings-ls-sig-size = Rozmiar sygnatury MinHash (k)
settings-ls-bands = Pasma LSH
settings-ls-recall = Próg czułości
settings-ls-result-cap = Limit wyników

# §8.20 Lenses → Custom.
settings-custom-registry = Rejestr
settings-custom-trust = Zaufanie
settings-custom-refresh-hashes = Odśwież skróty

# §8.21-§8.22 Network.
settings-net-https-enable = Włącz serwer HTTPS
settings-net-bind = Powiąż z interfejsami
settings-net-port = Nasłuchuj na porcie
settings-net-force-https = Wymuś HTTPS
settings-net-legacy-auth = Starsze uwierzytelnianie HTTP-basic
settings-net-token-regen = Wygeneruj token ponownie
settings-net-api-enable = Włącz serwer API
settings-net-legacy-ftp = Obsługa starszego nieszyfrowanego FTP/ETP

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Automatyczna aktualizacja
settings-privacy-prerelease = Kanał przedpremierowy
settings-privacy-network-policy = Zasady połączeń sieciowych

# §8.24 Logs & Debug.
settings-logs-level = Poziom dziennika
settings-logs-location = Lokalizacja pliku dziennika
settings-logs-retention = Przechowywanie dzienników
settings-logs-debug-overlay = Pokaż nakładkę debugowania
settings-logs-open-folder = Otwórz folder dzienników
settings-logs-export-bundle = Eksportuj pakiet diagnostyczny

# §8.25 Backup, Export, Reset.
settings-backup-export = Eksportuj ustawienia
settings-backup-import = Importuj ustawienia
settings-backup-export-bookmarks = Eksportuj pakiet zakładek
settings-backup-import-bookmarks = Importuj pakiet zakładek
settings-backup-reset-all = Przywróć wszystkie ustawienia domyślne

# §8.26 Locale.
settings-locale-current = Bieżący język
settings-locale-rtl-preview = Podgląd RTL
settings-locale-date-format = Format daty
settings-locale-number-format = Format liczb

# §8.27 About.
settings-about-version = Freally { $version }
settings-about-license = Licencja
settings-about-credits = Podziękowania
settings-about-notices = Informacje o oprogramowaniu open-source

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = Kreator pierwszego uruchomienia
wizard-step-of-total = Krok { $step } z { $total }
wizard-roots-hint = Dodaj foldery lub woluminy, które ma obserwować Freally. Możesz to później zmienić w ustawieniach Indeksów.
wizard-browse = Przeglądaj…
wizard-roots-placeholder = …lub wklej ścieżkę
wizard-roots-add = Dodaj
wizard-roots-remove = Usuń
wizard-roots-empty = Nie skonfigurowano jeszcze żadnych źródeł.
wizard-locale-hint = Freally jest dostępny w 18 językach. Możesz przełączyć później.
wizard-theme-hint = Systemowy podąża za ustawieniem wyglądu systemu operacyjnego.
wizard-back = Wstecz
wizard-next = Dalej

# Status bar polish.
statusbar-hotkey-hint = Skrót: { $hotkey }
statusbar-cycle-theme = Przełącz motyw
statusbar-indexed-suffix = zindeksowano

# Results / lenses.
lens-expand = Rozwiń soczewkę
lens-collapse = Zwiń soczewkę
lens-no-matches = Brak dopasowań w tej soczewce.

# Preview pane.
preview-header = Podgląd
preview-loading = Wczytywanie…
preview-select-file = Wybierz plik do podglądu.
preview-unavailable = Podgląd niedostępny

# Bookmarks.
bookmarks-label = ★ Zakładki
bookmarks-empty-hint = Brak zakładek. Naciśnij Ctrl+D, aby zapisać bieżące zapytanie.
bookmarks-organize-title = Organizuj zakładki
bookmarks-organize-empty = Brak zakładek.
bookmarks-rename = Zmień nazwę
bookmarks-close = Zamknij

# Settings tree extras.
settings-group-history = Historia
settings-group-privacy = Prywatność i aktualizacje
settings-group-logs = Dzienniki i debugowanie
settings-group-backup = Kopia zapasowa, eksport, reset
settings-tree-custom-lens = Niestandardowa
settings-unsaved-changes = niezapisane zmiany

# About dialog.
about-dialog-title = Freally
about-copyright = Copyright © 2026 Mike Weaver. Wszelkie prawa zastrzeżone.
about-close = Zamknij

# Connect endpoint dialog.
connect-ftp-title = Połącz z serwerem FTP
connect-ftp-host = Host:
connect-ftp-port = Port:
connect-ftp-username = Nazwa użytkownika:
connect-ftp-password = Hasło:
connect-ftp-link-type = Typ połączenia:

# UI panel.
ui-hint = Motyw, integracja z zasobnikiem / paskiem menu, wyszukiwanie podczas pisania, gęstość wierszy. Bezpośrednia zgodność z voidtools-Everything oraz dodatki Freally oznaczone (+).
ui-section-theme = Motyw
ui-theme-system-default = Systemowy (domyślnie)
ui-section-tray = Zasobnik / pasek menu
ui-section-search-behavior = Zachowanie wyszukiwania
ui-section-result-rows = Wiersze wyników
ui-single-click-system-default = Ustawienia systemowe (domyślnie)
ui-single-click-always = Zawsze pojedyncze kliknięcie
ui-single-click-always-double = Zawsze podwójne kliknięcie
ui-underline-always = Zawsze
ui-underline-on-hover = Po najechaniu
ui-underline-never = Nigdy

# Home panel.
home-hint = Domyślne wartości wczytywane przy uruchomieniu aplikacji — każda lista rozwijana może pozostać przy „Użyj ostatniej wartości” lub przypiąć stałą wartość. Widoczność soczewek / limity wyników to dodatki Freally (+).
home-section-match = Domyślne dopasowanie
home-section-search-sort = Domyślne wyszukiwanie i sortowanie
home-search-placeholder = Domyślnie puste
home-section-index = Źródło indeksu
home-file-list-path = Ścieżka listy plików
home-https-endpoint = Adres URL punktu końcowego API HTTPS
home-endpoint-token = Token (pokazany odcisk palca)

# Backup panel.
backup-section-settings = Ustawienia (+)
backup-section-bookmarks = Zakładki + niestandardowe ekstraktory (+)
backup-section-reset = Reset
backup-toast-exported = Wyeksportowano ustawienia do { $path }
backup-toast-export-failed = Eksport nie powiódł się: { $error }
backup-toast-imported = Zaimportowano ustawienia
backup-toast-import-failed = Import nie powiódł się: { $error }
backup-toast-bookmarks-exported = Wyeksportowano zakładki
backup-toast-bookmarks-export-failed = Eksport zakładek nie powiódł się: { $error }
backup-toast-bookmarks-imported = Zaimportowano zakładki
backup-toast-bookmarks-import-failed = Import zakładek nie powiódł się: { $error }
backup-confirm-reset = Przywrócić wszystkie ustawienia domyślne? Tej operacji nie można cofnąć (okno pozostanie otwarte).
backup-toast-reset = Zresetowano wszystkie ustawienia

# Keyboard panel.
keyboard-section-global = Skróty globalne
keyboard-placeholder-example = Super+Space
keyboard-section-commands = Polecenia
keyboard-placeholder-command = identyfikator polecenia (np. file.export_results)
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = Historia wyszukiwania
history-section-run = Historia uruchomień
history-section-privacy = Prywatność (+)
history-record-filename = Zapisuj historię soczewki nazw plików
history-record-content = Zapisuj historię soczewki treści
history-record-audio = Zapisuj historię soczewki dźwięku
history-record-similarity = Zapisuj historię soczewki podobieństwa

# Locale panel.
locale-section-language = Język (+)
locale-section-time-date = Godzina / data (+)
locale-date-os = Domyślny systemu
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = Niestandardowy
locale-date-custom-format = Niestandardowy format
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = Liczby (+)
locale-number-os = Domyślny systemu
locale-number-custom = Niestandardowy
locale-thousands-sep = Separator tysięcy
locale-decimal-sep = Separator dziesiętny

# Folders panel.
folders-hint = Dodatkowe obserwowane foldery poza domyślnymi woluminami.
folders-list-title = Obserwowane foldery
folders-empty = Nie dodano jeszcze żadnych folderów.
folders-remove = Usuń
folders-section-title-dynamic = Ustawienia dla { $path }
folders-section-schedule = Harmonogram ponownego skanowania
folders-schedule-daily = Codziennie o HH:MM
folders-schedule-hours = Co N godzin
folders-schedule-never = Nigdy
folders-hour = Godzina
folders-minute = Minuta
folders-hours = Godziny
folders-id-label = Identyfikator folderu (tylko do odczytu)
folders-select-prompt = Wybierz folder, aby go skonfigurować.
folders-section-extras = Dodatki Freally (+)
folders-extras-note = Ponowne skanowanie po wznowieniu z uśpienia jest domyślnie włączone w tej kompilacji; przełącznik dołączy do kontrolek na poziomie folderu w pakiecie dopracowania w fazie 13.

# Volumes panel.
volumes-hint = Wieloplatformowy odpowiednik paneli NTFS / ReFS z voidtools-Everything. Automatycznie wykrywa NTFS / ReFS / exFAT / FAT32 (Win), APFS / HFS+ (macOS), ext4 / Btrfs / ZFS / XFS / F2FS (Linux).
volumes-section-auto-include = Automatyczne dołączanie
volumes-list-title = Wykryte woluminy
volumes-detecting = Wykrywanie…
volumes-empty = Nie wykryto żadnych woluminów.
volumes-select-prompt = Wybierz wolumin, aby go skonfigurować.

# About panel polish.
about-section-version = Wersja (+)
about-section-license = Licencja (+)
about-license-text = Mike Weaver — Wszelkie prawa zastrzeżone. To jest oprogramowanie własnościowe.
about-license-spdx = SPDX: { $spdx }
about-section-credits = Podziękowania (+)
about-credits-inspired = Zainspirowane przez Everything firmy voidtools.
about-credits-voidtools = voidtools.com
about-credits-repo = Repozytorium projektu

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = Zawiera polecenia do pracy z Freally.
menu-file-new-window = Nowe okno wyszukiwania
menu-file-open-list = Otwórz listę plików…
menu-file-close-list = Zamknij listę plików
menu-file-close = Zamknij
menu-file-export-results = Eksportuj wyniki…
menu-file-export-bundle = Eksportuj pakiet indeksu…
menu-file-exit = Zakończ

# Edit menu.
menu-edit-hint = Zawiera polecenia do edycji wyników wyszukiwania.
menu-edit-cut = Wytnij
menu-edit-copy = Kopiuj
menu-edit-paste = Wklej
menu-edit-copy-to-folder = Kopiuj do folderu…
menu-edit-move-to-folder = Przenieś do folderu…
menu-edit-select-all = Zaznacz wszystko
menu-edit-invert-selection = Odwróć zaznaczenie
menu-edit-advanced = Zaawansowane
menu-edit-copy-full-name = Kopiuj pełną nazwę
menu-edit-copy-path = Kopiuj ścieżkę
menu-edit-copy-filename = Kopiuj nazwę pliku
menu-edit-copy-as-json = Kopiuj jako JSON
menu-edit-copy-with-metadata = Kopiuj z metadanymi
menu-edit-copy-as-bundle-ref = Kopiuj jako odwołanie do pakietu Freally

# View menu.
menu-view-hint = Zawiera polecenia do manipulowania widokiem.
menu-view-filters = Filtry
menu-view-preview = Podgląd
menu-view-status-bar = Pasek stanu
menu-view-thumbs-xl = Bardzo duże miniatury
menu-view-thumbs-l = Duże miniatury
menu-view-thumbs-m = Średnie miniatury
menu-view-details = Szczegóły
menu-view-window-size = Rozmiar okna
menu-view-window-size-hint = Zawiera polecenia do dostosowywania rozmiaru okna.
menu-view-window-small = Małe
menu-view-window-medium = Średnie
menu-view-window-large = Duże
menu-view-window-auto = Dopasuj automatycznie
menu-view-zoom = Powiększenie
menu-view-zoom-hint = Zawiera polecenia do dostosowywania rozmiaru czcionki i ikon.
menu-view-zoom-in = Powiększ
menu-view-zoom-out = Pomniejsz
menu-view-zoom-reset = Resetuj
menu-view-sort-by = Sortuj według
menu-view-sort-by-hint = Zawiera polecenia do sortowania listy wyników.
menu-view-sort-name = Nazwa
menu-view-sort-path = Ścieżka
menu-view-sort-size = Rozmiar
menu-view-sort-ext = Rozszerzenie
menu-view-sort-type = Typ
menu-view-sort-modified = Data modyfikacji
menu-view-sort-created = Data utworzenia
menu-view-sort-accessed = Data dostępu
menu-view-sort-attributes = Atrybuty
menu-view-sort-recently-changed = Data ostatniej zmiany
menu-view-sort-run-count = Liczba uruchomień
menu-view-sort-run-date = Data uruchomienia
menu-view-sort-file-list-filename = Nazwa pliku listy plików
menu-view-sort-lufs = LUFS
menu-view-sort-length = Długość
menu-view-sort-similarity = Wynik podobieństwa
menu-view-sort-asc = Rosnąco
menu-view-sort-desc = Malejąco
menu-view-go-to = Przejdź do
menu-view-refresh = Odśwież
menu-view-theme = Motyw
menu-view-theme-hint = Przełączaj między motywem systemowym, jasnym lub ciemnym.
menu-view-lenses = Soczewki
menu-view-lenses-hint = Przełączaj widoczność każdej soczewki na liście wyników.
menu-view-on-top = Na wierzchu
menu-view-on-top-hint = Zawiera polecenia do utrzymywania tego okna na wierzchu innych okien.
menu-view-on-top-never = Nigdy
menu-view-on-top-always = Zawsze
menu-view-on-top-while-searching = Podczas wyszukiwania

# Search menu.
menu-search-hint = Zawiera przełączniki wyszukiwania.
menu-search-match-case = Uwzględniaj wielkość liter
menu-search-match-whole-word = Dopasuj całe słowo
menu-search-match-path = Dopasuj ścieżkę
menu-search-match-diacritics = Uwzględniaj znaki diakrytyczne
menu-search-enable-regex = Włącz wyrażenia regularne
menu-search-advanced = Wyszukiwanie zaawansowane…
menu-search-add-to-filters = Dodaj do filtrów…
menu-search-organize-filters = Organizuj filtry…
menu-search-filter-everything = Everything
menu-search-filter-archive = Skompresowane (archiwum)
menu-search-filter-folder = Folder
menu-search-filter-custom = Filtr niestandardowy…

# Bookmarks menu.
menu-bookmarks-hint = Zawiera polecenia do pracy z zakładkami.
menu-bookmarks-add = Dodaj do zakładek
menu-bookmarks-organize = Organizuj zakładki…

# Tools menu.
menu-tools-hint = Zawiera polecenia narzędzi.
menu-tools-connect = Połącz z serwerem FTP…
menu-tools-disconnect = Rozłącz z serwerem FTP
menu-tools-file-list-editor = Edytor list plików…
menu-tools-index-maintenance = Konserwacja indeksu
menu-tools-index-maintenance-hint = Narzędzia konserwacji indeksu.
menu-tools-verify-index = Zweryfikuj indeks…
menu-tools-compact-index = Kompaktuj indeks…
menu-tools-rebuild-index = Wymuś przebudowę indeksu…
menu-tools-custom-extractor = Menedżer niestandardowych ekstraktorów…
menu-tools-custom-extractor-hint = Zarządzaj niestandardowymi ekstraktorami w piaskownicy Wasm.
menu-tools-options = Opcje…

# Help menu.
menu-help-hint = Zawiera polecenia pomocy.
menu-help-help = Pomoc Freally
menu-help-search-syntax = Składnia wyszukiwania
menu-help-regex-syntax = Składnia wyrażeń regularnych
menu-help-audio-ref = Dokumentacja modyfikatorów dźwięku
menu-help-similarity-ref = Dokumentacja modyfikatorów podobieństwa
menu-help-cli-options = Opcje wiersza poleceń
menu-help-website = Witryna Freally
menu-help-check-updates = Sprawdź aktualizacje…
menu-help-sponsor = Sponsoruj / wesprzyj
menu-help-about = O programie Freally…

# Result column headers (short forms used in the table header row).
column-name = Nazwa
column-path = Ścieżka
column-size = Rozmiar
column-modified = Zmodyfikowano
column-type = Typ
column-ext = Rozsz.
column-sort-by = Sortuj według { $name }
column-resize = Zmień szerokość kolumny { $name }

# Section subtitle bars used inside multiple settings panels.
section-behavior = Zachowanie
section-rendering = Renderowanie
section-status-bar = Pasek stanu
section-display-format = Format wyświetlania
section-loading-priority = Priorytet wczytywania
section-compatibility = Zgodność
section-storage = Przechowywanie
section-index-fields = Pola indeksu
section-maintenance = Konserwacja
section-logging = Rejestrowanie
section-tools = Narzędzia
section-privacy = Prywatność
section-auto-update = Automatyczna aktualizacja (+)
section-bind = Powiązanie
section-lens = Soczewka
section-budgets = Budżety
section-other = Inne
section-per-format-mode = Tryb na format
section-loudness = Głośność
section-tuning = Strojenie (+)
section-minhash-lsh = Parametry MinHash + LSH (+)
section-top-level = Najwyższy poziom
section-file-globs = Wzorce glob plików
section-file-list-settings = Ustawienia wybranej listy plików
section-editor-format = Edytor + format (E + +)
section-api-server = Serwer API (E zaadaptowane)
section-freally-extras = Dodatki Freally (+)
section-freally-additions = Rozszerzenia Freally (+)
section-freally-extensions = Rozszerzenia Freally (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = Użyj ostatniej wartości
opt-use-last-value-default = Użyj ostatniej wartości (domyślnie)
opt-low = Niski
opt-normal-default = Normalny (domyślnie)
opt-high = Wysoki
opt-disabled = Wyłączone
opt-off = Wył.
opt-on-battery = Przy zasilaniu z baterii
opt-always = Zawsze
opt-clamp-default = Przytnij (domyślnie)
opt-wrap = Zawijaj
opt-none = Brak
opt-strict-refuse = Ścisły (odrzucaj zapytania przy uszkodzeniu)
opt-lenient-warn = Łagodny (ostrzegaj, ale wykonuj zapytanie)
opt-system-default = Domyślny systemu
opt-drag-select = Zaznaczanie przeciągnięciem
opt-auto-binary = Auto (binarnie)
opt-auto-decimal = Auto (dziesiętnie)

# Unit suffixes shown next to number inputs.
unit-days = dni
unit-b = B
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-tb = TB

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = Zachłannie
opt-lazy-default = Leniwie (domyślnie)
opt-on = Wł.
opt-on-default = Wł. (domyślnie)
opt-all = Wszystkie
opt-weekly = Co tydzień
opt-monthly = Co miesiąc
opt-name-asc = Nazwa rosnąco
opt-name-desc = Nazwa malejąco
opt-size-asc = Rozmiar rosnąco
opt-size-desc = Rozmiar malejąco
opt-modified-asc = Data modyfikacji rosnąco
opt-modified-desc = Data modyfikacji malejąco
opt-compact = Kompaktowy
opt-comfortable = Komfortowy
opt-details = Szczegóły
opt-thumbnails = Miniatury
opt-local-db-default = Lokalna baza danych (domyślnie)
opt-file-list = Lista plików
opt-https-endpoint = Punkt końcowy API HTTPS
opt-right-default = Prawa (domyślnie)
opt-bottom = Dół
opt-or-and-default = OR > AND (domyślnie)
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128 (domyślnie)
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = Film telewizyjny (-23)
opt-true-peak = Szczyt rzeczywisty (4× nadpróbkowanie, domyślnie)
opt-sample-peak = Szczyt próbki
opt-auto-per-doc = Auto (na dokument)
opt-log-error = Błąd
opt-log-warn = Ostrzeżenie
opt-log-info-default = Informacja (domyślnie)
opt-log-debug = Debugowanie
opt-log-trace = Śledzenie
