# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = Eén zoekopdracht. Elke bron. Elk besturingssysteem.
window-title = Freally Sourcerer
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
parse-error-unknown = Onbekende syntaxis hier in de buurt.

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

wizard-title = Welkom bij Freally
wizard-step-roots = Kies wat je wilt indexeren
wizard-step-hotkey = Kies een globale sneltoets
wizard-step-locale = Kies je taal
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
settings-node-https-server = HTTP-/HTTPS-server
settings-node-etp-api = ETP-/FTP-API
settings-node-privacy = Privacy en updates
settings-node-logs = Logboeken en foutopsporing
settings-node-backup = Back-up, export, reset
settings-node-locale = Landinstelling
settings-node-about = Over

# §8.2 General → UI.
settings-ui-theme = Thema
settings-ui-run-bg = Op de achtergrond uitvoeren
settings-ui-show-tray = Pictogram in systeemvak/menubalk tonen
settings-ui-single-click-tray = Enkele klik op systeemvak/menubalk
settings-ui-new-window-from-tray = Nieuw venster openen via systeemvakpictogram
settings-ui-new-window-on-launch = Nieuw venster openen bij het starten van Freally
settings-ui-search-as-you-type = Zoeken tijdens het typen
settings-ui-select-on-mouse-click = Zoekopdracht selecteren bij muisklik
settings-ui-focus-on-activate = Zoekbalk focussen bij activeren
settings-ui-full-row-select = Volledige rij selecteren
settings-ui-single-click-open = Openen met enkele klik
settings-ui-underline-titles = Pictogramtitels onderstrepen
settings-ui-row-density = Resultaatdichtheid
settings-ui-row-density-compact = Compact (32 px)
settings-ui-row-density-comfortable = Comfortabel (44 px)
settings-ui-show-timing-badges = Tijdsindicatoren per lens tonen
settings-ui-anim-crossfade = Geanimeerde overgang tussen thema's

# §8.3 General → Home.
settings-home-match-case = Hoofdlettergevoelig
settings-home-match-whole-word = Heel woord zoeken
settings-home-match-path = Pad doorzoeken
settings-home-match-diacritics = Diakritische tekens meenemen
settings-home-match-regex = Reguliere expressie gebruiken
settings-home-search = Zoeken (aangepaste standaardzoekopdracht)
settings-home-filter = Filter
settings-home-sort = Sorteren
settings-home-view = Beeld
settings-home-index = Index
settings-home-default-lens-visibility = Standaard zichtbaarheid van lenzen
settings-home-default-lens-result-limits = Standaard resultaatlimieten per lens

# §8.4 General → Search.
settings-search-fast-ascii = Snel ASCII-zoeken
settings-search-mp-sep = Pad doorzoeken wanneer een zoekterm een padscheidingsteken bevat
settings-search-mw-fn = Volledige bestandsnaam zoeken bij gebruik van jokertekens
settings-search-lit-ops = Letterlijke operatoren toestaan
settings-search-paren = Groeperen met ronde haakjes toestaan
settings-search-env = Omgevingsvariabelen uitvouwen
settings-search-fwd-slash = Voorwaartse schuine strepen vervangen door backslashes
settings-search-precedence = Operatorvoorrang
settings-search-strict-everything = Strikte Everything-syntaxismodus
settings-search-auto-regex = Reguliere expressie automatisch detecteren
settings-search-mod-comp = Aanvullingen voor modifiers
settings-search-parse-tree = Ontledingsboom tonen bij zweven

# §8.5 General → Results.
settings-results-hide-empty = Resultaten verbergen wanneer de zoekopdracht leeg is
settings-results-clear-on-search = Selectie wissen bij zoeken
settings-results-close-on-execute = Venster sluiten bij uitvoeren
settings-results-dbl-path = Pad openen met dubbelklik in padkolom
settings-results-auto-scroll = Weergave automatisch scrollen
settings-results-dquote-copy = Met dubbele aanhalingstekens kopiëren als pad
settings-results-no-ext-rename = Extensie niet selecteren bij hernoemen
settings-results-sort-date-desc = Datum eerst aflopend sorteren
settings-results-sort-size-desc = Grootte eerst aflopend sorteren
settings-results-list-focus = Focus op resultaatlijst
settings-results-icon-prio = Prioriteit voor laden van pictogrammen
settings-results-thumb-prio = Prioriteit voor laden van miniaturen
settings-results-ext-prio = Prioriteit voor laden van uitgebreide informatie
settings-results-group-by-lens = Resultaten groeperen per lens
settings-results-snippet-inline = Fragmentvoorbeeld inline tonen

# §8.6 General → View.
settings-view-double-buffer = Dubbele buffering
settings-view-alt-rows = Afwisselende rijkleur
settings-view-row-mouseover = Rij markeren bij muisover
settings-view-highlight-terms = Gemarkeerde zoektermen tonen
settings-view-status-show-selected = Geselecteerd item in statusbalk tonen
settings-view-rc-with-sel = Resultaataantal samen met selectieaantal tonen
settings-view-status-show-size = Grootte in statusbalk tonen
settings-view-tooltips = Knopinfo tonen
settings-view-update-on-scroll = Weergave direct bijwerken na scrollen
settings-view-size-format = Grootteformaat
settings-view-selection-rect = Selectierechthoek
settings-view-audio-badges = LUFS-/codec-/lengte-indicatoren op audiorijen tonen
settings-view-similarity-score = MinHash-gelijkenisscore op gelijkenisrijen tonen
settings-view-preview-pane = Voorbeeldvenster

# §8.7 General → Context Menu.
settings-context-menu-visibility = Zichtbaarheid
settings-context-menu-show = Tonen
settings-context-menu-shift = Alleen tonen wanneer Shift ingedrukt is
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
settings-context-menu-reveal = Tonen in Freally
settings-context-menu-send-to = Naar Freally sturen (pad)

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
settings-fc-theme-inherit = Aangepaste kleuren automatisch omkeren bij thema-wisseling

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Globale sneltoets
settings-keyboard-new-window = Sneltoets nieuw venster
settings-keyboard-show-window = Sneltoets venster tonen
settings-keyboard-toggle-window = Sneltoets venster wisselen
settings-keyboard-show-commands = Opdrachten tonen die bevatten
settings-keyboard-add-chord = + Toetsencombinatie toevoegen
settings-keyboard-remove-chord = Verwijderen

# §8.10 History.
settings-history-search-enable = Zoekgeschiedenis inschakelen
settings-history-search-keep = Zoekgeschiedenis { $days } dagen bewaren
settings-history-run-enable = Uitvoeringsgeschiedenis inschakelen
settings-history-run-keep = Uitvoeringsgeschiedenis { $days } dagen bewaren
settings-history-clear-now = Nu wissen
settings-history-privacy-mode = Privacymodus
settings-history-per-lens = Geschiedenis per lens

# §8.11 Indexes (top-level).
settings-ix-database-location = Databaselocatie
settings-ix-multiuser = Bestandsnaam multi-userdatabase
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
settings-ix-attributes = Kenmerken indexeren
settings-ix-fast-attributes = Snel sorteren op kenmerken
settings-ix-fast-path-sort = Snel sorteren op pad
settings-ix-fast-extension-sort = Snel sorteren op extensie
settings-ix-force-rebuild = Opnieuw opbouwen forceren
settings-ix-compact = Index comprimeren
settings-ix-verify = Index verifiëren
settings-ix-integrity-policy = Beleid voor indexintegriteit
settings-ix-memory-budget = Geheugenbudget voor indexeerprogramma
settings-ix-throttle = Beperking van achtergrondindexering

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Nieuwe vaste volumes automatisch opnemen
settings-vol-auto-removable = Nieuwe verwisselbare volumes automatisch opnemen
settings-vol-auto-remove-offline = Offline volumes automatisch verwijderen
settings-vol-detected = Gedetecteerde volumes
settings-vol-include = Opnemen in index
settings-vol-include-only = Alleen opnemen (glob/regex)
settings-vol-enable-usn = USN Journal inschakelen
settings-vol-enable-fsevents = FSEvents-stream inschakelen
settings-vol-enable-inotify = inotify inschakelen (of fanotify indien verhoogd)
settings-vol-buffer = Journalbuffergrootte (KB)
settings-vol-allocation-delta = Toewijzingsdelta (KB)
settings-vol-load-recent = Recente wijzigingen uit journal laden bij opstarten
settings-vol-monitor = Wijzigingen bewaken
settings-vol-recreate-journal = Journal opnieuw maken
settings-vol-reset-stream = FSEvents-stream resetten
settings-vol-upgrade-fanotify = Upgraden naar fanotify (polkit)
settings-vol-remove = Verwijderen

# §8.13 Indexes → Folders.
settings-folders-watched = Bewaakte mappen
settings-folders-add = Toevoegen…
settings-folders-rescan-now = Nu opnieuw scannen
settings-folders-rescan-all = Alles nu opnieuw scannen
settings-folders-monitor = Wijzigingen proberen te bewaken
settings-folders-buffer = Buffergrootte
settings-folders-rescan-on-full = Opnieuw scannen bij volle buffer

# §8.14 Indexes → File Lists.
settings-flists-add = Toevoegen…
settings-flists-monitor = Wijzigingen bewaken
settings-flists-editor = Editor voor bestandslijsten…
settings-flists-format = Indeling bestandslijst
settings-flists-format-text = Tekst (één pad per regel)
settings-flists-format-json = JSON (met metadata)
settings-flists-format-srcb = Freally Bundle (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Verborgen bestanden en mappen uitsluiten
settings-exclude-system = Systeembestanden en -mappen uitsluiten
settings-exclude-list-en = Uitsluitlijst inschakelen
settings-exclude-folders = Mappen uitsluiten
settings-exclude-include-only-files = Alleen bestanden opnemen (glob)
settings-exclude-files = Bestanden uitsluiten (glob)
settings-exclude-os-recommended = Door besturingssysteem aanbevolen uitsluitingen toepassen
settings-exclude-by-class = Uitsluiten op extensieklasse

# §8.16 Lenses → Filename.
settings-lf-trigram = Agressiviteit van trigram-voorfilter
settings-lf-suffix-mem = Geheugenbudget voor suffixarray
settings-lf-wildcard-limit = Limiet voor jokertekenuitbreiding
settings-lf-regex-timeout = Time-out voor reguliere expressie

# §8.17 Lenses → Content.
settings-lc-enable = Inhoudslens inschakelen
settings-lc-time-budget = Tijdsbudget per document
settings-lc-mem-ceiling = Geheugenplafond per document
settings-lc-snippet-len = Fragmentlengte
settings-lc-stop-words = Taal voor stopwoorden
settings-lc-re-extract = Opnieuw extraheren bij instellingswijziging
settings-lc-verify-blobs = Controlesommen van geëxtraheerde-tekstblobs verifiëren bij lezen

# §8.18 Lenses → Audio.
settings-la-enable = Audiolens inschakelen
settings-la-lufs-ref = LUFS-referentiestandaard
settings-la-peak-compute = Piek berekenen via
settings-la-silence-thresh = Stiltedrempel
settings-la-re-extract-modify = Opnieuw extraheren bij wijzigingsgebeurtenis

# §8.19 Lenses → Similarity.
settings-ls-enable = Gelijkenislens inschakelen
settings-ls-sig-size = MinHash-signatuurgrootte (k)
settings-ls-bands = LSH-banden
settings-ls-recall = Recall-drempel
settings-ls-result-cap = Resultaatlimiet

# §8.20 Lenses → Custom.
settings-custom-registry = Register
settings-custom-trust = Vertrouwen
settings-custom-refresh-hashes = Hashes vernieuwen

# §8.21-§8.22 Network.
settings-net-https-enable = HTTPS-server inschakelen
settings-net-bind = Aan interfaces binden
settings-net-port = Luisteren op poort
settings-net-force-https = HTTPS afdwingen
settings-net-legacy-auth = Verouderde HTTP-basisauthenticatie
settings-net-token-regen = Token opnieuw genereren
settings-net-api-enable = API-server inschakelen
settings-net-legacy-ftp = Ondersteuning voor verouderde gewone FTP/ETP

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Automatisch bijwerken
settings-privacy-prerelease = Pre-releasekanaal
settings-privacy-network-policy = Beleid voor netwerkaanvragen

# §8.24 Logs & Debug.
settings-logs-level = Logniveau
settings-logs-location = Locatie van logbestand
settings-logs-retention = Bewaartermijn logboeken
settings-logs-debug-overlay = Foutopsporingsoverlay tonen
settings-logs-open-folder = Logmap openen
settings-logs-export-bundle = Diagnostiekbundel exporteren

# §8.25 Backup, Export, Reset.
settings-backup-export = Instellingen exporteren
settings-backup-import = Instellingen importeren
settings-backup-export-bookmarks = Bladwijzerbundel exporteren
settings-backup-import-bookmarks = Bladwijzerbundel importeren
settings-backup-reset-all = Alle instellingen terugzetten naar standaardwaarden

# §8.26 Locale.
settings-locale-current = Huidige landinstelling
settings-locale-rtl-preview = RTL-voorbeeld
settings-locale-date-format = Datumnotatie
settings-locale-number-format = Getalnotatie

# §8.27 About.
settings-about-version = Freally { $version }
settings-about-license = Licentie
settings-about-credits = Met dank aan
settings-about-notices = Open-sourcevermeldingen

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = Wizard voor eerste keer opstarten
wizard-step-of-total = Stap { $step } van { $total }
wizard-roots-hint = Voeg de mappen of volumes toe die je door Freally wilt laten bewaken. Je kunt dit later wijzigen via de indexinstellingen.
wizard-browse = Bladeren…
wizard-roots-placeholder = …of plak een pad
wizard-roots-add = Toevoegen
wizard-roots-remove = Verwijderen
wizard-roots-empty = Nog geen bronnen geconfigureerd.
wizard-locale-hint = Freally is beschikbaar in 18 talen. Je kunt later wisselen.
wizard-theme-hint = Systeem volgt de weergave-instelling van je besturingssysteem.
wizard-back = Terug
wizard-next = Volgende

# Status bar polish.
statusbar-hotkey-hint = Sneltoets: { $hotkey }
statusbar-cycle-theme = Door thema's bladeren
statusbar-indexed-suffix = geïndexeerd

# Results / lenses.
lens-expand = Lens uitvouwen
lens-collapse = Lens samenvouwen
lens-no-matches = Geen overeenkomsten in deze lens.

# Preview pane.
preview-header = Voorbeeld
preview-loading = Laden…
preview-select-file = Selecteer een bestand voor een voorbeeld.
preview-unavailable = Geen voorbeeld beschikbaar

# Bookmarks.
bookmarks-label = ★ Bladwijzers
bookmarks-empty-hint = Nog geen bladwijzers. Druk op Ctrl+D om de huidige zoekopdracht op te slaan.
bookmarks-organize-title = Bladwijzers ordenen
bookmarks-organize-empty = Nog geen bladwijzers.
bookmarks-rename = Hernoemen
bookmarks-close = Sluiten

# Settings tree extras.
settings-group-history = Geschiedenis
settings-group-privacy = Privacy en updates
settings-group-logs = Logboeken en foutopsporing
settings-group-backup = Back-up, export, reset
settings-tree-custom-lens = Aangepast
settings-unsaved-changes = niet-opgeslagen wijzigingen

# About dialog.
about-dialog-title = Freally
about-copyright = Copyright © 2026 Mike Weaver. Alle rechten voorbehouden.
about-close = Sluiten

# Connect endpoint dialog.
connect-ftp-title = Verbinden met FTP-server
connect-ftp-host = Host:
connect-ftp-port = Poort:
connect-ftp-username = Gebruikersnaam:
connect-ftp-password = Wachtwoord:
connect-ftp-link-type = Verbindingstype:

# UI panel.
ui-hint = Thema, integratie met systeemvak/menubalk, zoeken tijdens het typen, rijdichtheid. Directe pariteit met voidtools Everything plus toevoegingen van Freally, gemarkeerd met (+).
ui-section-theme = Thema
ui-theme-system-default = Systeem (standaard)
ui-section-tray = Systeemvak/menubalk
ui-section-search-behavior = Zoekgedrag
ui-section-result-rows = Resultaatrijen
ui-single-click-system-default = Systeeminstellingen (standaard)
ui-single-click-always = Altijd enkele klik
ui-single-click-always-double = Altijd dubbelklik
ui-underline-always = Altijd
ui-underline-on-hover = Bij zweven
ui-underline-never = Nooit

# Home panel.
home-hint = Standaardwaarden geladen bij het starten van de app — elk vervolgkeuzemenu kan op "Laatste waarde gebruiken" blijven staan of een vaste waarde vastzetten. Lenszichtbaarheid/resultaatlimieten zijn toevoegingen van Freally (+).
home-section-match = Standaardwaarden voor overeenkomst
home-section-search-sort = Standaardwaarden voor zoeken en sorteren
home-search-placeholder = Standaard leeg
home-section-index = Indexbron
home-file-list-path = Pad naar bestandslijst
home-https-endpoint = URL van HTTPS-API-eindpunt
home-endpoint-token = Token (vingerafdruk getoond)

# Backup panel.
backup-section-settings = Instellingen (+)
backup-section-bookmarks = Bladwijzers + aangepaste extractors (+)
backup-section-reset = Reset
backup-toast-exported = Instellingen geëxporteerd naar { $path }
backup-toast-export-failed = Export mislukt: { $error }
backup-toast-imported = Instellingen geïmporteerd
backup-toast-import-failed = Import mislukt: { $error }
backup-toast-bookmarks-exported = Bladwijzers geëxporteerd
backup-toast-bookmarks-export-failed = Export van bladwijzers mislukt: { $error }
backup-toast-bookmarks-imported = Bladwijzers geïmporteerd
backup-toast-bookmarks-import-failed = Import van bladwijzers mislukt: { $error }
backup-confirm-reset = Alle instellingen terugzetten naar standaardwaarden? Dit kan niet ongedaan worden gemaakt (het dialoogvenster blijft open).
backup-toast-reset = Alle instellingen gereset

# Keyboard panel.
keyboard-section-global = Globale sneltoetsen
keyboard-placeholder-example = Super+Space
keyboard-section-commands = Opdrachten
keyboard-placeholder-command = opdracht-id (bijv. file.export_results)
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = Zoekgeschiedenis
history-section-run = Uitvoeringsgeschiedenis
history-section-privacy = Privacy (+)
history-record-filename = Geschiedenis van bestandsnaamlens vastleggen
history-record-content = Geschiedenis van inhoudslens vastleggen
history-record-audio = Geschiedenis van audiolens vastleggen
history-record-similarity = Geschiedenis van gelijkenislens vastleggen

# Locale panel.
locale-section-language = Taal (+)
locale-section-time-date = Tijd/datum (+)
locale-date-os = Standaard besturingssysteem
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = Aangepast
locale-date-custom-format = Aangepaste notatie
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = Getallen (+)
locale-number-os = Standaard besturingssysteem
locale-number-custom = Aangepast
locale-thousands-sep = Scheidingsteken voor duizendtallen
locale-decimal-sep = Decimaalscheidingsteken

# Folders panel.
folders-hint = Aanvullende bewaakte mappen naast de standaardvolumes.
folders-list-title = Bewaakte mappen
folders-empty = Nog geen mappen toegevoegd.
folders-remove = Verwijderen
folders-section-title-dynamic = Instellingen voor { $path }
folders-section-schedule = Schema voor opnieuw scannen
folders-schedule-daily = Elke dag om HH:MM
folders-schedule-hours = Elke N uur
folders-schedule-never = Nooit
folders-hour = Uur
folders-minute = Minuut
folders-hours = Uren
folders-id-label = Map-ID (alleen-lezen)
folders-select-prompt = Selecteer een map om deze te configureren.
folders-section-extras = Freally-extra's (+)
folders-extras-note = Opnieuw scannen bij hervatten uit slaapstand is in deze build standaard ingeschakeld; de schakelaar wordt in de afwerkingsronde van fase 13 toegevoegd aan de bedieningselementen op mapniveau.

# Volumes panel.
volumes-hint = Platformonafhankelijke tegenhanger van de NTFS-/ReFS-panelen van voidtools Everything. Detecteert automatisch NTFS/ReFS/exFAT/FAT32 (Win), APFS/HFS+ (macOS), ext4/Btrfs/ZFS/XFS/F2FS (Linux).
volumes-section-auto-include = Automatisch opnemen
volumes-list-title = Gedetecteerde volumes
volumes-detecting = Detecteren…
volumes-empty = Geen volumes gedetecteerd.
volumes-select-prompt = Selecteer een volume om dit te configureren.

# About panel polish.
about-section-version = Versie (+)
about-section-license = Licentie (+)
about-license-text = Mike Weaver — Alle rechten voorbehouden. Dit is propriëtaire software.
about-license-spdx = SPDX: { $spdx }
about-section-credits = Met dank aan (+)
about-credits-inspired = Geïnspireerd door Everything van voidtools.
about-credits-voidtools = voidtools.com
about-credits-repo = Projectrepository

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = Bevat opdrachten om met Freally te werken.
menu-file-new-window = Nieuw zoekvenster
menu-file-open-list = Bestandslijst openen…
menu-file-close-list = Bestandslijst sluiten
menu-file-close = Sluiten
menu-file-export-results = Resultaten exporteren…
menu-file-export-bundle = Indexbundel exporteren…
menu-file-exit = Afsluiten

# Edit menu.
menu-edit-hint = Bevat opdrachten om zoekresultaten te bewerken.
menu-edit-cut = Knippen
menu-edit-copy = Kopiëren
menu-edit-paste = Plakken
menu-edit-copy-to-folder = Naar map kopiëren…
menu-edit-move-to-folder = Naar map verplaatsen…
menu-edit-select-all = Alles selecteren
menu-edit-invert-selection = Selectie omkeren
menu-edit-advanced = Geavanceerd
menu-edit-copy-full-name = Volledige naam kopiëren
menu-edit-copy-path = Pad kopiëren
menu-edit-copy-filename = Bestandsnaam kopiëren
menu-edit-copy-as-json = Kopiëren als JSON
menu-edit-copy-with-metadata = Kopiëren met metadata
menu-edit-copy-as-bundle-ref = Kopiëren als Freally Bundle-verwijzing

# View menu.
menu-view-hint = Bevat opdrachten om de weergave aan te passen.
menu-view-filters = Filters
menu-view-preview = Voorbeeld
menu-view-status-bar = Statusbalk
menu-view-thumbs-xl = Extra grote miniaturen
menu-view-thumbs-l = Grote miniaturen
menu-view-thumbs-m = Middelgrote miniaturen
menu-view-details = Details
menu-view-window-size = Venstergrootte
menu-view-window-size-hint = Bevat opdrachten om de grootte van het venster aan te passen.
menu-view-window-small = Klein
menu-view-window-medium = Middel
menu-view-window-large = Groot
menu-view-window-auto = Automatisch aanpassen
menu-view-zoom = Zoomen
menu-view-zoom-hint = Bevat opdrachten om de letter- en pictogramgrootte aan te passen.
menu-view-zoom-in = Inzoomen
menu-view-zoom-out = Uitzoomen
menu-view-zoom-reset = Resetten
menu-view-sort-by = Sorteren op
menu-view-sort-by-hint = Bevat opdrachten om de resultaatlijst te sorteren.
menu-view-sort-name = Naam
menu-view-sort-path = Pad
menu-view-sort-size = Grootte
menu-view-sort-ext = Extensie
menu-view-sort-type = Type
menu-view-sort-modified = Wijzigingsdatum
menu-view-sort-created = Aanmaakdatum
menu-view-sort-accessed = Toegangsdatum
menu-view-sort-attributes = Kenmerken
menu-view-sort-recently-changed = Datum recent gewijzigd
menu-view-sort-run-count = Aantal uitvoeringen
menu-view-sort-run-date = Datum uitgevoerd
menu-view-sort-file-list-filename = Bestandsnaam van bestandslijst
menu-view-sort-lufs = LUFS
menu-view-sort-length = Lengte
menu-view-sort-similarity = Gelijkenisscore
menu-view-sort-asc = Oplopend
menu-view-sort-desc = Aflopend
menu-view-go-to = Ga naar
menu-view-refresh = Vernieuwen
menu-view-theme = Thema
menu-view-theme-hint = Wisselen tussen systeem-, lichte of donkere thema's.
menu-view-lenses = Lenzen
menu-view-lenses-hint = Zichtbaarheid van elke lens in de resultaatlijst in-/uitschakelen.
menu-view-on-top = Op voorgrond
menu-view-on-top-hint = Bevat opdrachten om dit venster boven andere vensters te houden.
menu-view-on-top-never = Nooit
menu-view-on-top-always = Altijd
menu-view-on-top-while-searching = Tijdens zoeken

# Search menu.
menu-search-hint = Bevat zoekschakelaars.
menu-search-match-case = Hoofdlettergevoelig
menu-search-match-whole-word = Heel woord zoeken
menu-search-match-path = Pad doorzoeken
menu-search-match-diacritics = Diakritische tekens meenemen
menu-search-enable-regex = Reguliere expressie inschakelen
menu-search-advanced = Geavanceerd zoeken…
menu-search-add-to-filters = Aan filters toevoegen…
menu-search-organize-filters = Filters ordenen…
menu-search-filter-everything = Everything
menu-search-filter-archive = Gecomprimeerd (archief)
menu-search-filter-folder = Map
menu-search-filter-custom = Aangepast filter…

# Bookmarks menu.
menu-bookmarks-hint = Bevat opdrachten om met bladwijzers te werken.
menu-bookmarks-add = Aan bladwijzers toevoegen
menu-bookmarks-organize = Bladwijzers ordenen…

# Tools menu.
menu-tools-hint = Bevat opdrachten voor extra functies.
menu-tools-connect = Verbinden met FTP-server…
menu-tools-disconnect = Verbinding met FTP-server verbreken
menu-tools-file-list-editor = Editor voor bestandslijsten…
menu-tools-index-maintenance = Indexonderhoud
menu-tools-index-maintenance-hint = Hulpmiddelen voor indexonderhoud.
menu-tools-verify-index = Index verifiëren…
menu-tools-compact-index = Index comprimeren…
menu-tools-rebuild-index = Opnieuw opbouwen van index forceren…
menu-tools-custom-extractor = Beheer aangepaste extractors…
menu-tools-custom-extractor-hint = Wasm-sandboxed aangepaste extractors beheren.
menu-tools-options = Opties…

# Help menu.
menu-help-hint = Bevat helpopdrachten.
menu-help-help = Freally Help
menu-help-search-syntax = Zoeksyntaxis
menu-help-regex-syntax = Syntaxis reguliere expressies
menu-help-audio-ref = Naslagwerk audiomodifiers
menu-help-similarity-ref = Naslagwerk gelijkenismodifiers
menu-help-cli-options = Opdrachtregelopties
menu-help-website = Freally-website
menu-help-check-updates = Controleren op updates…
menu-help-sponsor = Sponsoren/doneren
menu-help-about = Over Freally…

# Result column headers (short forms used in the table header row).
column-name = Naam
column-path = Pad
column-size = Grootte
column-modified = Gewijzigd
column-type = Type
column-ext = Ext
column-sort-by = Sorteren op { $name }
column-resize = Kolom { $name } vergroten/verkleinen

# Section subtitle bars used inside multiple settings panels.
section-behavior = Gedrag
section-rendering = Weergave
section-status-bar = Statusbalk
section-display-format = Weergavenotatie
section-loading-priority = Laadprioriteit
section-compatibility = Compatibiliteit
section-storage = Opslag
section-index-fields = Indexvelden
section-maintenance = Onderhoud
section-logging = Logboekregistratie
section-tools = Extra
section-privacy = Privacy
section-auto-update = Automatisch bijwerken (+)
section-bind = Binden
section-lens = Lens
section-budgets = Budgetten
section-other = Overig
section-per-format-mode = Modus per formaat
section-loudness = Luidheid
section-tuning = Afstemming (+)
section-minhash-lsh = MinHash + LSH-parameters (+)
section-top-level = Hoofdniveau
section-file-globs = Bestandsglobs
section-file-list-settings = Instellingen voor geselecteerde bestandslijst
section-editor-format = Editor + formaat (E + +)
section-api-server = API-server (E aangepast)
section-freally-extras = Freally-extra's (+)
section-freally-additions = Freally-toevoegingen (+)
section-freally-extensions = Freally-uitbreidingen (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = Laatste waarde gebruiken
opt-use-last-value-default = Laatste waarde gebruiken (standaard)
opt-low = Laag
opt-normal-default = Normaal (standaard)
opt-high = Hoog
opt-disabled = Uitgeschakeld
opt-off = Uit
opt-on-battery = Bij accugebruik
opt-always = Altijd
opt-clamp-default = Begrenzen (standaard)
opt-wrap = Omslaan
opt-none = Geen
opt-strict-refuse = Strikt (zoekopdrachten weigeren bij beschadiging)
opt-lenient-warn = Soepel (waarschuwen maar zoeken)
opt-system-default = Standaard besturingssysteem
opt-drag-select = Slepen om te selecteren
opt-auto-binary = Automatisch (binair)
opt-auto-decimal = Automatisch (decimaal)

# Unit suffixes shown next to number inputs.
unit-days = dagen
unit-b = B
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-tb = TB

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = Direct
opt-lazy-default = Lui (standaard)
opt-on = Aan
opt-on-default = Aan (standaard)
opt-all = Alles
opt-weekly = Wekelijks
opt-monthly = Maandelijks
opt-name-asc = Naam oplopend
opt-name-desc = Naam aflopend
opt-size-asc = Grootte oplopend
opt-size-desc = Grootte aflopend
opt-modified-asc = Wijzigingsdatum oplopend
opt-modified-desc = Wijzigingsdatum aflopend
opt-compact = Compact
opt-comfortable = Comfortabel
opt-details = Details
opt-thumbnails = Miniaturen
opt-local-db-default = Lokale database (standaard)
opt-file-list = Bestandslijst
opt-https-endpoint = HTTPS-API-eindpunt
opt-right-default = Rechts (standaard)
opt-bottom = Onder
opt-or-and-default = OR > AND (standaard)
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128 (standaard)
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = Broadcast film (-23)
opt-true-peak = True peak (4× oversampling, standaard)
opt-sample-peak = Sample peak
opt-auto-per-doc = Automatisch (per document)
opt-log-error = Error
opt-log-warn = Warn
opt-log-info-default = Info (standaard)
opt-log-debug = Debug
opt-log-trace = Trace
