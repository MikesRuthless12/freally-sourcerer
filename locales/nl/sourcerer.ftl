# Sourcerer — Nederlands.

app-name = Sourcerer
tagline = Eén zoekopdracht. Elke bron. Elk besturingssysteem.
window-title = Sourcerer
search-placeholder = Zoeken…
about-version = Versie { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = Gereed
status-indexed = Geïndexeerd ({ $count } bestanden)
status-indexing = Indexeren… { $done }/{ $total }
status-paused = Gepauzeerd
status-error = Fout
status-result-count-one = { $count } resultaat
status-result-count-many = { $count } resultaten
status-selection = · { $count } geselecteerd
status-selection-size = Geselecteerd: { $size }
status-query-timing = Zoekopdracht: { $ms } ms
status-endpoint-local = Lokale database
status-endpoint-remote = API: { $name }

menu-file = Bestand
menu-edit = Bewerken
menu-view = Beeld
menu-search = Zoeken
menu-bookmarks = Bladwijzers
menu-tools = Extra
menu-help = Help

theme-system = Systeem
theme-light = Licht
theme-dark = Donker

lens-filename = Bestandsnaam
lens-content = Inhoud
lens-audio = Audio
lens-similarity = Gelijkenis

parse-error-empty = Typ een zoekopdracht om te beginnen.
parse-error-unknown = Onherkende syntaxis op deze plek.

action-open = Openen
action-reveal = Tonen in map
action-copy-path = Pad kopiëren
action-copy-name = Naam kopiëren
action-delete = Verwijderen

quick-filter-audio = Audio
quick-filter-video = Video
quick-filter-image = Afbeelding
quick-filter-document = Document
quick-filter-executable = Uitvoerbaar bestand
quick-filter-archive = Archief

wizard-title = Welkom bij Sourcerer
wizard-step-roots = Kies wat u wilt indexeren
wizard-step-hotkey = Kies een globale sneltoets
wizard-step-locale = Kies uw taal
wizard-step-theme = Kies een thema
wizard-finish = Voltooien

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = Opties
settings-search-placeholder = Opties zoeken…
settings-restore-defaults = Standaardwaarden herstellen
settings-ok = OK
settings-cancel = Annuleren
settings-apply = Toepassen

# Tree nav groups (PRD §8.1.1).
settings-group-general = Algemeen
settings-group-indexes = Indexen
settings-group-lenses = Lenzen
settings-group-network = Netwerk

# Tree nav leaves.
settings-node-ui = UI
settings-node-home = Start
settings-node-search = Zoeken
settings-node-results = Resultaten
settings-node-view = Beeld
settings-node-context-menu = Contextmenu
settings-node-fonts-colors = Lettertypen en kleuren
settings-node-keyboard = Toetsenbord
settings-node-history = Geschiedenis
settings-node-indexes-top = (hoofdniveau)
settings-node-volumes = Volumes
settings-node-folders = Mappen
settings-node-file-lists = Bestandslijsten
settings-node-exclude = Uitsluiten
settings-node-https-server = HTTP / HTTPS-server
settings-node-etp-api = ETP / FTP-API
settings-node-privacy = Privacy en updates
settings-node-logs = Logs en debug
settings-node-backup = Back-up, export, reset
settings-node-locale = Taal en regio
settings-node-about = Over

# §8.2 General → UI.
settings-ui-theme = Thema
settings-ui-run-bg = Op de achtergrond uitvoeren
settings-ui-show-tray = Pictogram in systeemvak / menubalk tonen
settings-ui-single-click-tray = Enkele klik op systeemvak / menubalk
settings-ui-new-window-from-tray = Nieuw venster openen vanuit systeemvakpictogram
settings-ui-new-window-on-launch = Nieuw venster openen bij het starten van Sourcerer
settings-ui-search-as-you-type = Zoeken tijdens typen
settings-ui-select-on-mouse-click = Zoekopdracht selecteren bij muisklik
settings-ui-focus-on-activate = Zoekveld focussen bij activeren
settings-ui-full-row-select = Hele rij selecteren
settings-ui-single-click-open = Openen met enkele klik
settings-ui-underline-titles = Pictogramtitels onderstrepen
settings-ui-row-density = Resultaatdichtheid
settings-ui-row-density-compact = Compact (32 px)
settings-ui-row-density-comfortable = Comfortabel (44 px)
settings-ui-show-timing-badges = Timingbadges per lens tonen
settings-ui-anim-crossfade = Geanimeerde thema-overgang

# §8.3 General → Home.
settings-home-match-case = Hoofdlettergevoelig
settings-home-match-whole-word = Heel woord zoeken
settings-home-match-path = Pad doorzoeken
settings-home-match-diacritics = Diakritische tekens meetellen
settings-home-match-regex = Regex gebruiken
settings-home-search = Zoeken (aangepaste standaardquery)
settings-home-filter = Filter
settings-home-sort = Sortering
settings-home-view = Beeld
settings-home-index = Index
settings-home-default-lens-visibility = Standaard zichtbaarheid van lenzen
settings-home-default-lens-result-limits = Standaard resultaatlimieten per lens

# §8.4 General → Search.
settings-search-fast-ascii = Snelle ASCII-zoekopdracht
settings-search-mp-sep = Pad doorzoeken wanneer een zoekterm een padscheidingsteken bevat
settings-search-mw-fn = Hele bestandsnaam matchen bij gebruik van jokertekens
settings-search-lit-ops = Letterlijke operatoren toestaan
settings-search-paren = Groepering met haakjes toestaan
settings-search-env = Omgevingsvariabelen uitvouwen
settings-search-fwd-slash = Slashes vervangen door backslashes
settings-search-precedence = Operatorvolgorde
settings-search-strict-everything = Strikte Everything-syntaxismodus
settings-search-auto-regex = Regex automatisch detecteren
settings-search-mod-comp = Aanvullingen voor modificatoren
settings-search-parse-tree = Parse-tree tonen bij hover

# §8.5 General → Results.
settings-results-hide-empty = Resultaten verbergen bij lege zoekopdracht
settings-results-clear-on-search = Selectie wissen bij zoeken
settings-results-close-on-execute = Venster sluiten bij uitvoeren
settings-results-dbl-path = Pad openen met dubbelklik in padkolom
settings-results-auto-scroll = Automatisch scrollen
settings-results-dquote-copy = Kopiëren met dubbele aanhalingstekens als pad
settings-results-no-ext-rename = Extensie niet selecteren bij hernoemen
settings-results-sort-date-desc = Eerst sorteren op datum aflopend
settings-results-sort-size-desc = Eerst sorteren op grootte aflopend
settings-results-list-focus = Focus op resultaatlijst
settings-results-icon-prio = Prioriteit voor laden van pictogrammen
settings-results-thumb-prio = Prioriteit voor laden van miniaturen
settings-results-ext-prio = Prioriteit voor laden van uitgebreide informatie
settings-results-group-by-lens = Resultaten groeperen per lens
settings-results-snippet-inline = Inline fragmentvoorbeeld tonen

# §8.6 General → View.
settings-view-double-buffer = Dubbele buffer
settings-view-alt-rows = Afwisselende rijkleur
settings-view-row-mouseover = Rijhover tonen
settings-view-highlight-terms = Gemarkeerde zoektermen tonen
settings-view-status-show-selected = Geselecteerd item tonen in statusbalk
settings-view-rc-with-sel = Resultaataantal naast selectieaantal tonen
settings-view-status-show-size = Grootte tonen in statusbalk
settings-view-tooltips = Knopinfo tonen
settings-view-update-on-scroll = Beeld direct bijwerken tijdens scrollen
settings-view-size-format = Notatie voor bestandsgrootte
settings-view-selection-rect = Selectierechthoek
settings-view-audio-badges = LUFS-, codec- en lengtebadges op audiorijen tonen
settings-view-similarity-score = MinHash-gelijkenisscore op gelijkenisrijen tonen
settings-view-preview-pane = Voorbeeldvenster

# §8.7 General → Context Menu.
settings-context-menu-visibility = Zichtbaarheid
settings-context-menu-show = Tonen
settings-context-menu-shift = Alleen tonen bij ingedrukte Shift
settings-context-menu-hide = Verbergen
settings-context-menu-command = Opdrachtmacro
settings-context-menu-open-folders = Openen (mappen)
settings-context-menu-open-files = Openen (bestanden)
settings-context-menu-open-path = Pad openen
settings-context-menu-explore = Verkennen
settings-context-menu-explore-path = Pad verkennen
settings-context-menu-copy-name = Naam naar klembord kopiëren
settings-context-menu-copy-path = Pad naar klembord kopiëren
settings-context-menu-copy-full-name = Volledige naam naar klembord kopiëren
settings-context-menu-reveal = Tonen in Sourcerer
settings-context-menu-send-to = Naar Sourcerer sturen (pad)

# §8.8 General → Fonts & Colors.
settings-fc-font = Lettertype
settings-fc-size = Grootte
settings-fc-state-normal = Normaal
settings-fc-state-highlighted = Gemarkeerd
settings-fc-state-current-sort = Huidige sortering
settings-fc-state-current-sort-h = Huidige sortering (gemarkeerd)
settings-fc-state-selected = Geselecteerd
settings-fc-state-selected-h = Geselecteerd (gemarkeerd)
settings-fc-state-inactive-selected = Inactief geselecteerd
settings-fc-state-inactive-selected-h = Inactief geselecteerd (gemarkeerd)
settings-fc-foreground = Voorgrond
settings-fc-background = Achtergrond
settings-fc-bold = Vet
settings-fc-italic = Cursief
settings-fc-default = Standaard
settings-fc-per-lens-accent = Accent per lens
settings-fc-theme-inherit = Aangepaste kleuren automatisch omkeren bij themawissel

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Globale sneltoets
settings-keyboard-new-window = Sneltoets voor nieuw venster
settings-keyboard-show-window = Sneltoets om venster te tonen
settings-keyboard-toggle-window = Sneltoets om venster te schakelen
settings-keyboard-show-commands = Opdrachten tonen die bevatten
settings-keyboard-add-chord = + Combinatie toevoegen
settings-keyboard-remove-chord = Verwijderen

# §8.10 History.
settings-history-search-enable = Zoekgeschiedenis inschakelen
settings-history-search-keep = Zoekgeschiedenis { $days } dagen bewaren
settings-history-run-enable = Uitvoergeschiedenis inschakelen
settings-history-run-keep = Uitvoergeschiedenis { $days } dagen bewaren
settings-history-clear-now = Nu wissen
settings-history-privacy-mode = Privémodus
settings-history-per-lens = Geschiedenis per lens

# §8.11 Indexes (top-level).
settings-ix-database-location = Locatie van database
settings-ix-multiuser = Bestandsnaam voor database met meerdere gebruikers
settings-ix-compress = Database comprimeren
settings-ix-recent-changes = Recente wijzigingen indexeren
settings-ix-file-size = Bestandsgrootte indexeren
settings-ix-fast-size-sort = Snel sorteren op grootte
settings-ix-folder-size = Mapgrootte indexeren
settings-ix-fast-folder-size-sort = Snel sorteren op mapgrootte
settings-ix-date-created = Aanmaakdatum indexeren
settings-ix-fast-date-created = Snel sorteren op aanmaakdatum
settings-ix-date-modified = Wijzigingsdatum indexeren
settings-ix-fast-date-modified = Snel sorteren op wijzigingsdatum
settings-ix-date-accessed = Toegangsdatum indexeren
settings-ix-fast-date-accessed = Snel sorteren op toegangsdatum
settings-ix-attributes = Attributen indexeren
settings-ix-fast-attributes = Snel sorteren op attributen
settings-ix-fast-path-sort = Snel sorteren op pad
settings-ix-fast-extension-sort = Snel sorteren op extensie
settings-ix-force-rebuild = Geforceerd herbouwen
settings-ix-compact = Index compacteren
settings-ix-verify = Index verifiëren
settings-ix-integrity-policy = Integriteitsbeleid voor index
settings-ix-memory-budget = Geheugenbudget voor indexer
settings-ix-throttle = Beperking van indexeren op de achtergrond

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Nieuwe vaste volumes automatisch opnemen
settings-vol-auto-removable = Nieuwe verwisselbare volumes automatisch opnemen
settings-vol-auto-remove-offline = Offline volumes automatisch verwijderen
settings-vol-detected = Gedetecteerde volumes
settings-vol-include = Opnemen in index
settings-vol-include-only = Alleen opnemen (glob/regex)
settings-vol-enable-usn = USN-journal inschakelen
settings-vol-enable-fsevents = FSEvents-stream inschakelen
settings-vol-enable-inotify = inotify inschakelen (of fanotify met verhoogde rechten)
settings-vol-buffer = Buffergrootte van journal (KB)
settings-vol-allocation-delta = Allocation delta (KB)
settings-vol-load-recent = Recente wijzigingen uit journal laden bij opstarten
settings-vol-monitor = Wijzigingen volgen
settings-vol-recreate-journal = Journal opnieuw aanmaken
settings-vol-reset-stream = FSEvents-stream resetten
settings-vol-upgrade-fanotify = Upgraden naar fanotify (polkit)
settings-vol-remove = Verwijderen

# §8.13 Indexes → Folders.
settings-folders-watched = Gevolgde mappen
settings-folders-add = Toevoegen…
settings-folders-rescan-now = Nu opnieuw scannen
settings-folders-rescan-all = Alles nu opnieuw scannen
settings-folders-monitor = Wijzigingen proberen te volgen
settings-folders-buffer = Buffergrootte
settings-folders-rescan-on-full = Opnieuw scannen bij volle buffer

# §8.14 Indexes → File Lists.
settings-flists-add = Toevoegen…
settings-flists-monitor = Wijzigingen volgen
settings-flists-editor = Bestandslijst-editor…
settings-flists-format = Bestandslijstindeling
settings-flists-format-text = Tekst (één pad per regel)
settings-flists-format-json = JSON (met metadata)
settings-flists-format-srcb = Sourcerer-bundel (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Verborgen bestanden en mappen uitsluiten
settings-exclude-system = Systeembestanden en -mappen uitsluiten
settings-exclude-list-en = Uitsluitlijst inschakelen
settings-exclude-folders = Mappen uitsluiten
settings-exclude-include-only-files = Alleen bestanden opnemen (glob)
settings-exclude-files = Bestanden uitsluiten (glob)
settings-exclude-os-recommended = Door OS aanbevolen uitsluitingen toepassen
settings-exclude-by-class = Uitsluiten op extensieklasse

# §8.16 Lenses → Filename.
settings-lf-trigram = Agressiviteit van trigram-voorfilter
settings-lf-suffix-mem = Geheugenbudget voor suffix-array
settings-lf-wildcard-limit = Limiet voor uitvouwen van jokertekens
settings-lf-regex-timeout = Time-out voor Regex

# §8.17 Lenses → Content.
settings-lc-enable = Inhoudslens inschakelen
settings-lc-time-budget = Tijdbudget per document
settings-lc-mem-ceiling = Geheugenplafond per document
settings-lc-snippet-len = Lengte van fragment
settings-lc-stop-words = Taal voor stopwoorden
settings-lc-re-extract = Opnieuw extraheren bij wijziging van instellingen
settings-lc-verify-blobs = Checksums van geëxtraheerde-tekst-blobs verifiëren bij lezen

# §8.18 Lenses → Audio.
settings-la-enable = Audiolens inschakelen
settings-la-lufs-ref = LUFS-referentienorm
settings-la-peak-compute = Piek berekenen via
settings-la-silence-thresh = Stiltedrempel
settings-la-re-extract-modify = Opnieuw extraheren bij Modify-event

# §8.19 Lenses → Similarity.
settings-ls-enable = Gelijkenislens inschakelen
settings-ls-sig-size = Grootte van MinHash-signatuur (k)
settings-ls-bands = LSH-banden
settings-ls-recall = Recall-drempel
settings-ls-result-cap = Resultaatlimiet

# §8.20 Lenses → Custom.
settings-custom-registry = Register
settings-custom-trust = Vertrouwen
settings-custom-refresh-hashes = Hashes vernieuwen

# §8.21-§8.22 Network.
settings-net-https-enable = HTTPS-server inschakelen
settings-net-bind = Binden aan interfaces
settings-net-port = Luisteren op poort
settings-net-force-https = HTTPS afdwingen
settings-net-legacy-auth = Verouderde HTTP-basic-authenticatie
settings-net-token-regen = Token opnieuw genereren
settings-net-api-enable = API-server inschakelen
settings-net-legacy-ftp = Ondersteuning voor verouderde plain FTP/ETP

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Automatisch bijwerken
settings-privacy-prerelease = Pre-releasekanaal
settings-privacy-network-policy = Beleid voor netwerkverzoeken

# §8.24 Logs & Debug.
settings-logs-level = Logniveau
settings-logs-location = Locatie van logbestand
settings-logs-retention = Logbewaartermijn
settings-logs-debug-overlay = Debug-overlay tonen
settings-logs-open-folder = Logmap openen
settings-logs-export-bundle = Diagnosebundel exporteren

# §8.25 Backup, Export, Reset.
settings-backup-export = Instellingen exporteren
settings-backup-import = Instellingen importeren
settings-backup-export-bookmarks = Bladwijzerbundel exporteren
settings-backup-import-bookmarks = Bladwijzerbundel importeren
settings-backup-reset-all = Alle instellingen herstellen naar standaardwaarden

# §8.26 Locale.
settings-locale-current = Huidige taal en regio
settings-locale-rtl-preview = RTL-voorbeeld
settings-locale-date-format = Datumnotatie
settings-locale-number-format = Getalsnotatie

# §8.27 About.
settings-about-version = Sourcerer { $version }
settings-about-license = Licentie
settings-about-credits = Met dank aan
settings-about-notices = Open-sourcevermeldingen
