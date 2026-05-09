# Sourcerer — Polski.

app-name = Sourcerer
tagline = Jedno wyszukiwanie. Każde źródło. Każdy system.
window-title = Sourcerer
search-placeholder = Szukaj…
about-version = Wersja { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = Gotowy
status-indexed = Zindeksowano ({ $count } plików)
status-indexing = Indeksowanie… { $done }/{ $total }
status-paused = Wstrzymano
status-error = Błąd
status-result-count-one = { $count } wynik
status-result-count-many = { $count } wyników
status-selection = · zaznaczono { $count }
status-selection-size = Zaznaczone: { $size }
status-query-timing = Zapytanie: { $ms } ms
status-endpoint-local = Lokalna baza
status-endpoint-remote = API: { $name }

menu-file = Plik
menu-edit = Edycja
menu-view = Widok
menu-search = Wyszukiwanie
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

wizard-title = Witamy w Sourcerer
wizard-step-roots = Wybierz, co indeksować
wizard-step-hotkey = Wybierz globalny skrót
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
settings-node-home = Strona główna
settings-node-search = Wyszukiwanie
settings-node-results = Wyniki
settings-node-view = Widok
settings-node-context-menu = Menu kontekstowe
settings-node-fonts-colors = Czcionki i kolory
settings-node-keyboard = Klawiatura
settings-node-history = Historia
settings-node-indexes-top = (poziom główny)
settings-node-volumes = Woluminy
settings-node-folders = Foldery
settings-node-file-lists = Listy plików
settings-node-exclude = Wykluczenia
settings-node-https-server = Serwer HTTP / HTTPS
settings-node-etp-api = ETP / FTP API
settings-node-privacy = Prywatność i aktualizacje
settings-node-logs = Dzienniki i debugowanie
settings-node-backup = Kopia zapasowa, eksport, resetowanie
settings-node-locale = Lokalizacja
settings-node-about = Informacje

# §8.2 General → UI.
settings-ui-theme = Motyw
settings-ui-run-bg = Uruchom w tle
settings-ui-show-tray = Pokaż ikonę w zasobniku / na pasku menu
settings-ui-single-click-tray = Pojedyncze kliknięcie zasobnika / paska menu
settings-ui-new-window-from-tray = Otwórz nowe okno z ikony zasobnika
settings-ui-new-window-on-launch = Otwórz nowe okno przy uruchamianiu Sourcerer
settings-ui-search-as-you-type = Szukaj podczas pisania
settings-ui-select-on-mouse-click = Zaznacz wyszukiwanie po kliknięciu myszą
settings-ui-focus-on-activate = Ustaw fokus na wyszukiwaniu po aktywacji
settings-ui-full-row-select = Zaznaczanie całego wiersza
settings-ui-single-click-open = Otwieranie pojedynczym kliknięciem
settings-ui-underline-titles = Podkreślaj tytuły ikon
settings-ui-row-density = Gęstość wyników
settings-ui-row-density-compact = Kompaktowa (32 px)
settings-ui-row-density-comfortable = Wygodna (44 px)
settings-ui-show-timing-badges = Pokaż znaczniki czasu dla każdej soczewki
settings-ui-anim-crossfade = Animowane przenikanie motywu

# §8.3 General → Home.
settings-home-match-case = Uwzględniaj wielkość liter
settings-home-match-whole-word = Dopasuj całe słowo
settings-home-match-path = Dopasuj ścieżkę
settings-home-match-diacritics = Uwzględniaj znaki diakrytyczne
settings-home-match-regex = Dopasuj Regex
settings-home-search = Wyszukiwanie (niestandardowe zapytanie domyślne)
settings-home-filter = Filtr
settings-home-sort = Sortowanie
settings-home-view = Widok
settings-home-index = Indeks
settings-home-default-lens-visibility = Domyślna widoczność soczewek
settings-home-default-lens-result-limits = Domyślne limity wyników soczewek

# §8.4 General → Search.
settings-search-fast-ascii = Szybkie wyszukiwanie ASCII
settings-search-mp-sep = Dopasuj ścieżkę, gdy fraza zawiera separator ścieżki
settings-search-mw-fn = Dopasuj całą nazwę pliku przy użyciu symboli wieloznacznych
settings-search-lit-ops = Zezwalaj na dosłowne operatory
settings-search-paren = Zezwalaj na grupowanie nawiasami okrągłymi
settings-search-env = Rozwijaj zmienne środowiskowe
settings-search-fwd-slash = Zamieniaj ukośniki na ukośniki odwrotne
settings-search-precedence = Priorytet operatorów
settings-search-strict-everything = Tryb ścisłej składni Everything
settings-search-auto-regex = Automatycznie wykrywaj Regex
settings-search-mod-comp = Uzupełnianie modyfikatorów
settings-search-parse-tree = Pokaż drzewo składniowe po najechaniu

# §8.5 General → Results.
settings-results-hide-empty = Ukryj wyniki, gdy zapytanie jest puste
settings-results-clear-on-search = Wyczyść zaznaczenie przy wyszukiwaniu
settings-results-close-on-execute = Zamknij okno po wykonaniu
settings-results-dbl-path = Otwórz ścieżkę dwukrotnym kliknięciem w kolumnie ścieżki
settings-results-auto-scroll = Automatycznie przewijaj widok
settings-results-dquote-copy = Kopiuj w cudzysłowie jako ścieżkę
settings-results-no-ext-rename = Nie zaznaczaj rozszerzenia podczas zmiany nazwy
settings-results-sort-date-desc = Sortuj daty malejąco najpierw
settings-results-sort-size-desc = Sortuj rozmiary malejąco najpierw
settings-results-list-focus = Fokus listy wyników
settings-results-icon-prio = Priorytet ładowania ikon
settings-results-thumb-prio = Priorytet ładowania miniatur
settings-results-ext-prio = Priorytet ładowania rozszerzonych informacji
settings-results-group-by-lens = Grupuj wyniki według soczewki
settings-results-snippet-inline = Pokaż podgląd fragmentu w wierszu

# §8.6 General → View.
settings-view-double-buffer = Podwójne buforowanie
settings-view-alt-rows = Naprzemienny kolor wierszy
settings-view-row-mouseover = Pokaż podświetlenie wiersza pod kursorem
settings-view-highlight-terms = Wyróżniaj wyszukiwane terminy
settings-view-status-show-selected = Pokaż zaznaczony element na pasku stanu
settings-view-rc-with-sel = Pokazuj liczbę wyników wraz z liczbą zaznaczonych
settings-view-status-show-size = Pokaż rozmiar na pasku stanu
settings-view-tooltips = Pokazuj podpowiedzi
settings-view-update-on-scroll = Natychmiast aktualizuj widok po przewijaniu
settings-view-size-format = Format rozmiaru
settings-view-selection-rect = Prostokąt zaznaczenia
settings-view-audio-badges = Pokaż znaczniki LUFS / codec / długości w wierszach audio
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
settings-context-menu-explore = Przeglądaj
settings-context-menu-explore-path = Przeglądaj ścieżkę
settings-context-menu-copy-name = Kopiuj nazwę do schowka
settings-context-menu-copy-path = Kopiuj ścieżkę do schowka
settings-context-menu-copy-full-name = Kopiuj pełną nazwę do schowka
settings-context-menu-reveal = Pokaż w Sourcerer
settings-context-menu-send-to = Wyślij do Sourcerer (ścieżka)

# §8.8 General → Fonts & Colors.
settings-fc-font = Czcionka
settings-fc-size = Rozmiar
settings-fc-state-normal = Normalny
settings-fc-state-highlighted = Podświetlony
settings-fc-state-current-sort = Bieżące sortowanie
settings-fc-state-current-sort-h = Bieżące sortowanie (podświetlone)
settings-fc-state-selected = Zaznaczony
settings-fc-state-selected-h = Zaznaczony (podświetlony)
settings-fc-state-inactive-selected = Zaznaczony nieaktywny
settings-fc-state-inactive-selected-h = Zaznaczony nieaktywny (podświetlony)
settings-fc-foreground = Pierwszy plan
settings-fc-background = Tło
settings-fc-bold = Pogrubienie
settings-fc-italic = Kursywa
settings-fc-default = Domyślne
settings-fc-per-lens-accent = Akcent dla każdej soczewki
settings-fc-theme-inherit = Automatycznie odwracaj kolory niestandardowe przy zmianie motywu

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Globalny skrót klawiszowy
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
settings-history-privacy-mode = Tryb prywatności
settings-history-per-lens = Historia dla każdej soczewki

# §8.11 Indexes (top-level).
settings-ix-database-location = Lokalizacja bazy danych
settings-ix-multiuser = Nazwa pliku bazy danych dla wielu użytkowników
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
settings-ix-verify = Weryfikuj indeks
settings-ix-integrity-policy = Zasady spójności indeksu
settings-ix-memory-budget = Budżet pamięci dla indeksera
settings-ix-throttle = Ograniczenie indeksowania w tle

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Automatycznie dołączaj nowe woluminy stałe
settings-vol-auto-removable = Automatycznie dołączaj nowe woluminy wymienne
settings-vol-auto-remove-offline = Automatycznie usuwaj woluminy w trybie offline
settings-vol-detected = Wykryte woluminy
settings-vol-include = Uwzględnij w indeksie
settings-vol-include-only = Uwzględnij tylko (glob/Regex)
settings-vol-enable-usn = Włącz dziennik USN
settings-vol-enable-fsevents = Włącz strumień FSEvents
settings-vol-enable-inotify = Włącz inotify (lub fanotify, jeśli z podwyższonymi uprawnieniami)
settings-vol-buffer = Rozmiar bufora dziennika (KB)
settings-vol-allocation-delta = Delta alokacji (KB)
settings-vol-load-recent = Ładuj ostatnie zmiany z dziennika przy uruchamianiu
settings-vol-monitor = Monitoruj zmiany
settings-vol-recreate-journal = Odtwórz dziennik
settings-vol-reset-stream = Zresetuj strumień FSEvents
settings-vol-upgrade-fanotify = Przełącz na fanotify (polkit)
settings-vol-remove = Usuń

# §8.13 Indexes → Folders.
settings-folders-watched = Obserwowane foldery
settings-folders-add = Dodaj…
settings-folders-rescan-now = Skanuj ponownie teraz
settings-folders-rescan-all = Skanuj ponownie wszystko teraz
settings-folders-monitor = Próbuj monitorować zmiany
settings-folders-buffer = Rozmiar bufora
settings-folders-rescan-on-full = Skanuj ponownie przy zapełnionym buforze

# §8.14 Indexes → File Lists.
settings-flists-add = Dodaj…
settings-flists-monitor = Monitoruj zmiany
settings-flists-editor = Edytor list plików…
settings-flists-format = Format listy plików
settings-flists-format-text = Tekst (jedna ścieżka w wierszu)
settings-flists-format-json = JSON (z metadanymi)
settings-flists-format-srcb = Pakiet Sourcerer (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Wykluczaj ukryte pliki i foldery
settings-exclude-system = Wykluczaj systemowe pliki i foldery
settings-exclude-list-en = Włącz listę wykluczeń
settings-exclude-folders = Wykluczaj foldery
settings-exclude-include-only-files = Uwzględnij tylko pliki (glob)
settings-exclude-files = Wykluczaj pliki (glob)
settings-exclude-os-recommended = Zastosuj wykluczenia zalecane przez system
settings-exclude-by-class = Wykluczaj według klasy rozszerzeń

# §8.16 Lenses → Filename.
settings-lf-trigram = Agresywność wstępnego filtra trigram
settings-lf-suffix-mem = Budżet pamięci tablicy sufiksów
settings-lf-wildcard-limit = Limit rozwijania symboli wieloznacznych
settings-lf-regex-timeout = Limit czasu Regex

# §8.17 Lenses → Content.
settings-lc-enable = Włącz soczewkę treści
settings-lc-time-budget = Budżet czasu na dokument
settings-lc-mem-ceiling = Limit pamięci na dokument
settings-lc-snippet-len = Długość fragmentu
settings-lc-stop-words = Język słów wykluczonych
settings-lc-re-extract = Wyodrębnij ponownie po zmianie ustawień
settings-lc-verify-blobs = Weryfikuj sumy kontrolne blob wyodrębnionego tekstu przy odczycie

# §8.18 Lenses → Audio.
settings-la-enable = Włącz soczewkę dźwięku
settings-la-lufs-ref = Standard odniesienia LUFS
settings-la-peak-compute = Oblicz wartość szczytową przez
settings-la-silence-thresh = Próg ciszy
settings-la-re-extract-modify = Wyodrębnij ponownie po zdarzeniu Modyfikacji

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
settings-net-bind = Wiąż z interfejsami
settings-net-port = Nasłuchuj na porcie
settings-net-force-https = Wymuszaj HTTPS
settings-net-legacy-auth = Starsze uwierzytelnianie HTTP-basic
settings-net-token-regen = Wygeneruj token ponownie
settings-net-api-enable = Włącz serwer API
settings-net-legacy-ftp = Obsługa starszego, niezaszyfrowanego FTP/ETP

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Automatyczna aktualizacja
settings-privacy-prerelease = Kanał wydań wstępnych
settings-privacy-network-policy = Zasady połączeń sieciowych

# §8.24 Logs & Debug.
settings-logs-level = Poziom dziennika
settings-logs-location = Lokalizacja pliku dziennika
settings-logs-retention = Czas przechowywania dzienników
settings-logs-debug-overlay = Pokaż nakładkę debugowania
settings-logs-open-folder = Otwórz folder dzienników
settings-logs-export-bundle = Eksportuj pakiet diagnostyczny

# §8.25 Backup, Export, Reset.
settings-backup-export = Eksportuj ustawienia
settings-backup-import = Importuj ustawienia
settings-backup-export-bookmarks = Eksportuj pakiet zakładek
settings-backup-import-bookmarks = Importuj pakiet zakładek
settings-backup-reset-all = Przywróć wszystkie ustawienia do domyślnych

# §8.26 Locale.
settings-locale-current = Bieżąca lokalizacja
settings-locale-rtl-preview = Podgląd RTL
settings-locale-date-format = Format daty
settings-locale-number-format = Format liczb

# §8.27 About.
settings-about-version = Sourcerer { $version }
settings-about-license = Licencja
settings-about-credits = Twórcy
settings-about-notices = Informacje o oprogramowaniu open source
