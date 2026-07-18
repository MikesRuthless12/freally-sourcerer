# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = Eine Suche. Jede Quelle. Jedes Betriebssystem.
window-title = Freally Sourcerer
search-placeholder = Suchen…
about-version = Version { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = Bereit
status-indexed = Indiziert ({ $count } Dateien)
status-indexing = Indizierung… { $done }/{ $total }
status-paused = Pausiert
status-error = Fehler
status-result-count-one = { $count } Treffer
status-result-count-many = { $count } Treffer
status-selection = · { $count } ausgewählt
status-selection-size = Ausgewählt: { $size }
status-query-timing = Abfrage: { $ms } ms
status-endpoint-local = Lokale DB
status-endpoint-remote = API: { $name }

menu-file = Datei
menu-edit = Bearbeiten
menu-view = Ansicht
menu-search = Suchen
menu-bookmarks = Lesezeichen
menu-tools = Extras
menu-help = Hilfe

theme-system = System
theme-light = Hell
theme-dark = Dunkel

lens-filename = Dateiname
lens-content = Inhalt
lens-audio = Audio
lens-similarity = Ähnlichkeit

parse-error-empty = Geben Sie eine Suchanfrage ein, um zu beginnen.
parse-error-unknown = Unbekannte Syntax in der Nähe.

action-open = Öffnen
action-reveal = Im Ordner anzeigen
action-copy-path = Pfad kopieren
action-copy-name = Namen kopieren
action-delete = Löschen

quick-filter-audio = Audio
quick-filter-video = Video
quick-filter-image = Bild
quick-filter-document = Dokument
quick-filter-executable = Programm
quick-filter-archive = Archiv

wizard-title = Willkommen bei Freally
wizard-step-roots = Wählen Sie aus, was indiziert werden soll
wizard-step-hotkey = Globales Tastenkürzel festlegen
wizard-step-locale = Sprache auswählen
wizard-step-theme = Design auswählen
wizard-finish = Fertigstellen

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = Optionen
settings-search-placeholder = Optionen durchsuchen…
settings-restore-defaults = Standardwerte wiederherstellen
settings-ok = OK
settings-cancel = Abbrechen
settings-apply = Übernehmen

# Tree nav groups (PRD §8.1.1).
settings-group-general = Allgemein
settings-group-indexes = Indizes
settings-group-lenses = Lupen
settings-group-network = Netzwerk

# Tree nav leaves.
settings-node-ui = Oberfläche
settings-node-home = Start
settings-node-search = Suche
settings-node-results = Ergebnisse
settings-node-view = Ansicht
settings-node-context-menu = Kontextmenü
settings-node-fonts-colors = Schriften & Farben
settings-node-keyboard = Tastatur
settings-node-history = Verlauf
settings-node-indexes-top = (oberste Ebene)
settings-node-volumes = Datenträger
settings-node-folders = Ordner
settings-node-file-lists = Dateilisten
settings-node-exclude = Ausschließen
settings-node-https-server = HTTP-/HTTPS-Server
settings-node-etp-api = ETP-/FTP-API
settings-node-privacy = Datenschutz & Updates
settings-node-logs = Protokolle & Debug
settings-node-backup = Sichern, Exportieren, Zurücksetzen
settings-node-locale = Sprache
settings-node-about = Über

# §8.2 General → UI.
settings-ui-theme = Design
settings-ui-run-bg = Im Hintergrund ausführen
settings-ui-show-tray = Symbol in Infobereich/Menüleiste anzeigen
settings-ui-single-click-tray = Einfacher Klick im Infobereich/Menüleiste
settings-ui-new-window-from-tray = Neues Fenster über Infobereichssymbol öffnen
settings-ui-new-window-on-launch = Neues Fenster beim Start von Freally öffnen
settings-ui-search-as-you-type = Suchen während der Eingabe
settings-ui-select-on-mouse-click = Suche bei Mausklick auswählen
settings-ui-focus-on-activate = Suchfeld beim Aktivieren fokussieren
settings-ui-full-row-select = Gesamte Zeile auswählen
settings-ui-single-click-open = Mit einfachem Klick öffnen
settings-ui-underline-titles = Symboltitel unterstreichen
settings-ui-row-density = Ergebnisdichte
settings-ui-row-density-compact = Kompakt (32 px)
settings-ui-row-density-comfortable = Komfortabel (44 px)
settings-ui-show-timing-badges = Zeit-Badges pro Lupe anzeigen
settings-ui-anim-crossfade = Animierte Überblendung beim Designwechsel

# §8.3 General → Home.
settings-home-match-case = Groß-/Kleinschreibung beachten
settings-home-match-whole-word = Ganzes Wort suchen
settings-home-match-path = Pfad einbeziehen
settings-home-match-diacritics = Diakritische Zeichen beachten
settings-home-match-regex = Regulären Ausdruck verwenden
settings-home-search = Suche (eigene Standardabfrage)
settings-home-filter = Filter
settings-home-sort = Sortierung
settings-home-view = Ansicht
settings-home-index = Index
settings-home-default-lens-visibility = Standard-Lupensichtbarkeit
settings-home-default-lens-result-limits = Standard-Ergebnisgrenzen pro Lupe

# §8.4 General → Search.
settings-search-fast-ascii = Schnelle ASCII-Suche
settings-search-mp-sep = Pfad einbeziehen, wenn ein Suchbegriff ein Pfadtrennzeichen enthält
settings-search-mw-fn = Ganzen Dateinamen bei Platzhaltern suchen
settings-search-lit-ops = Literale Operatoren zulassen
settings-search-paren = Gruppierung mit runden Klammern zulassen
settings-search-env = Umgebungsvariablen erweitern
settings-search-fwd-slash = Schrägstriche durch Backslashes ersetzen
settings-search-precedence = Operatorrangfolge
settings-search-strict-everything = Strikter Everything-Syntaxmodus
settings-search-auto-regex = Regulären Ausdruck automatisch erkennen
settings-search-mod-comp = Modifikator-Vervollständigungen
settings-search-parse-tree = Parse-Baum beim Überfahren anzeigen

# §8.5 General → Results.
settings-results-hide-empty = Ergebnisse bei leerer Suche ausblenden
settings-results-clear-on-search = Auswahl bei neuer Suche aufheben
settings-results-close-on-execute = Fenster beim Ausführen schließen
settings-results-dbl-path = Pfad mit Doppelklick in der Pfadspalte öffnen
settings-results-auto-scroll = Ansicht automatisch scrollen
settings-results-dquote-copy = Mit Anführungszeichen als Pfad kopieren
settings-results-no-ext-rename = Erweiterung beim Umbenennen nicht auswählen
settings-results-sort-date-desc = Datum zuerst absteigend sortieren
settings-results-sort-size-desc = Größe zuerst absteigend sortieren
settings-results-list-focus = Fokus der Ergebnisliste
settings-results-icon-prio = Priorität beim Laden von Symbolen
settings-results-thumb-prio = Priorität beim Laden von Miniaturansichten
settings-results-ext-prio = Priorität beim Laden erweiterter Informationen
settings-results-group-by-lens = Ergebnisse nach Lupe gruppieren
settings-results-snippet-inline = Snippet-Vorschau inline anzeigen

# §8.6 General → View.
settings-view-double-buffer = Doppelpufferung
settings-view-alt-rows = Abwechselnde Zeilenfarbe
settings-view-row-mouseover = Zeilenhervorhebung beim Überfahren anzeigen
settings-view-highlight-terms = Hervorgehobene Suchbegriffe anzeigen
settings-view-status-show-selected = Ausgewähltes Element in der Statusleiste anzeigen
settings-view-rc-with-sel = Ergebnisanzahl zusammen mit der Auswahlanzahl anzeigen
settings-view-status-show-size = Größe in der Statusleiste anzeigen
settings-view-tooltips = Tooltips anzeigen
settings-view-update-on-scroll = Anzeige sofort nach dem Scrollen aktualisieren
settings-view-size-format = Größenformat
settings-view-selection-rect = Auswahlrechteck
settings-view-audio-badges = LUFS-/Codec-/Längen-Badges in Audiozeilen anzeigen
settings-view-similarity-score = MinHash-Ähnlichkeitswert in Ähnlichkeitszeilen anzeigen
settings-view-preview-pane = Vorschaubereich

# §8.7 General → Context Menu.
settings-context-menu-visibility = Sichtbarkeit
settings-context-menu-show = Anzeigen
settings-context-menu-shift = Nur bei gedrückter Umschalttaste anzeigen
settings-context-menu-hide = Ausblenden
settings-context-menu-command = Befehlsmakro
settings-context-menu-open-folders = Öffnen (Ordner)
settings-context-menu-open-files = Öffnen (Dateien)
settings-context-menu-open-path = Pfad öffnen
settings-context-menu-explore = Durchsuchen
settings-context-menu-explore-path = Pfad durchsuchen
settings-context-menu-copy-name = Namen in Zwischenablage kopieren
settings-context-menu-copy-path = Pfad in Zwischenablage kopieren
settings-context-menu-copy-full-name = Vollständigen Namen in Zwischenablage kopieren
settings-context-menu-reveal = In Freally anzeigen
settings-context-menu-send-to = An Freally senden (Pfad)

# §8.8 General → Fonts & Colors.
settings-fc-font = Schriftart
settings-fc-size = Größe
settings-fc-state-normal = Normal
settings-fc-state-highlighted = Hervorgehoben
settings-fc-state-current-sort = Aktuelle Sortierung
settings-fc-state-current-sort-h = Aktuelle Sortierung (hervorgehoben)
settings-fc-state-selected = Ausgewählt
settings-fc-state-selected-h = Ausgewählt (hervorgehoben)
settings-fc-state-inactive-selected = Inaktiv ausgewählt
settings-fc-state-inactive-selected-h = Inaktiv ausgewählt (hervorgehoben)
settings-fc-foreground = Vordergrund
settings-fc-background = Hintergrund
settings-fc-bold = Fett
settings-fc-italic = Kursiv
settings-fc-default = Standard
settings-fc-per-lens-accent = Akzent pro Lupe
settings-fc-theme-inherit = Eigene Farben beim Designwechsel automatisch umkehren

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Globales Tastenkürzel
settings-keyboard-new-window = Tastenkürzel für neues Fenster
settings-keyboard-show-window = Tastenkürzel zum Anzeigen des Fensters
settings-keyboard-toggle-window = Tastenkürzel zum Umschalten des Fensters
settings-keyboard-show-commands = Befehle anzeigen, die enthalten
settings-keyboard-add-chord = + Tastenfolge hinzufügen
settings-keyboard-remove-chord = Entfernen

# §8.10 History.
settings-history-search-enable = Suchverlauf aktivieren
settings-history-search-keep = Suchverlauf { $days } Tage lang aufbewahren
settings-history-run-enable = Ausführungsverlauf aktivieren
settings-history-run-keep = Ausführungsverlauf { $days } Tage lang aufbewahren
settings-history-clear-now = Jetzt löschen
settings-history-privacy-mode = Datenschutzmodus
settings-history-per-lens = Verlauf pro Lupe

# §8.11 Indexes (top-level).
settings-ix-database-location = Speicherort der Datenbank
settings-ix-multiuser = Dateiname der Mehrbenutzer-Datenbank
settings-ix-compress = Datenbank komprimieren
settings-ix-recent-changes = Letzte Änderungen indizieren
settings-ix-file-size = Dateigröße indizieren
settings-ix-fast-size-sort = Schnelle Größensortierung
settings-ix-folder-size = Ordnergröße indizieren
settings-ix-fast-folder-size-sort = Schnelle Ordnergrößensortierung
settings-ix-date-created = Erstellungsdatum indizieren
settings-ix-fast-date-created = Schnelle Sortierung nach Erstellungsdatum
settings-ix-date-modified = Änderungsdatum indizieren
settings-ix-fast-date-modified = Schnelle Sortierung nach Änderungsdatum
settings-ix-date-accessed = Zugriffsdatum indizieren
settings-ix-fast-date-accessed = Schnelle Sortierung nach Zugriffsdatum
settings-ix-attributes = Attribute indizieren
settings-ix-fast-attributes = Schnelle Attributsortierung
settings-ix-fast-path-sort = Schnelle Pfadsortierung
settings-ix-fast-extension-sort = Schnelle Erweiterungssortierung
settings-ix-force-rebuild = Neuaufbau erzwingen
settings-ix-compact = Index komprimieren
settings-ix-verify = Index überprüfen
settings-ix-integrity-policy = Richtlinie zur Indexintegrität
settings-ix-memory-budget = Speicherbudget für Indizierung
settings-ix-throttle = Drosselung der Hintergrundindizierung

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Neue feste Datenträger automatisch einbeziehen
settings-vol-auto-removable = Neue Wechseldatenträger automatisch einbeziehen
settings-vol-auto-remove-offline = Offline-Datenträger automatisch entfernen
settings-vol-detected = Erkannte Datenträger
settings-vol-include = In Index einbeziehen
settings-vol-include-only = Nur einbeziehen (Glob/Regex)
settings-vol-enable-usn = USN-Journal aktivieren
settings-vol-enable-fsevents = FSEvents-Stream aktivieren
settings-vol-enable-inotify = inotify aktivieren (oder fanotify bei erhöhten Rechten)
settings-vol-buffer = Journal-Puffergröße (KB)
settings-vol-allocation-delta = Zuordnungsdelta (KB)
settings-vol-load-recent = Letzte Änderungen beim Start aus dem Journal laden
settings-vol-monitor = Änderungen überwachen
settings-vol-recreate-journal = Journal neu erstellen
settings-vol-reset-stream = FSEvents-Stream zurücksetzen
settings-vol-upgrade-fanotify = Auf fanotify aktualisieren (polkit)
settings-vol-remove = Entfernen

# §8.13 Indexes → Folders.
settings-folders-watched = Überwachte Ordner
settings-folders-add = Hinzufügen…
settings-folders-rescan-now = Jetzt neu scannen
settings-folders-rescan-all = Alle jetzt neu scannen
settings-folders-monitor = Versuchen, Änderungen zu überwachen
settings-folders-buffer = Puffergröße
settings-folders-rescan-on-full = Bei vollem Puffer neu scannen

# §8.14 Indexes → File Lists.
settings-flists-add = Hinzufügen…
settings-flists-monitor = Änderungen überwachen
settings-flists-editor = Dateilisten-Editor…
settings-flists-format = Dateilistenformat
settings-flists-format-text = Text (ein Pfad pro Zeile)
settings-flists-format-json = JSON (mit Metadaten)
settings-flists-format-srcb = Freally-Bundle (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Versteckte Dateien und Ordner ausschließen
settings-exclude-system = Systemdateien und -ordner ausschließen
settings-exclude-list-en = Ausschlussliste aktivieren
settings-exclude-folders = Ordner ausschließen
settings-exclude-include-only-files = Nur Dateien einbeziehen (Glob)
settings-exclude-files = Dateien ausschließen (Glob)
settings-exclude-os-recommended = Vom Betriebssystem empfohlene Ausschlüsse anwenden
settings-exclude-by-class = Nach Erweiterungsklasse ausschließen

# §8.16 Lenses → Filename.
settings-lf-trigram = Aggressivität des Trigramm-Vorfilters
settings-lf-suffix-mem = Speicherbudget für Suffix-Array
settings-lf-wildcard-limit = Grenze für Platzhaltererweiterung
settings-lf-regex-timeout = Regex-Zeitlimit

# §8.17 Lenses → Content.
settings-lc-enable = Inhaltslupe aktivieren
settings-lc-time-budget = Zeitbudget pro Dokument
settings-lc-mem-ceiling = Speicherobergrenze pro Dokument
settings-lc-snippet-len = Snippet-Länge
settings-lc-stop-words = Sprache der Stoppwörter
settings-lc-re-extract = Bei Einstellungsänderung neu extrahieren
settings-lc-verify-blobs = Prüfsummen extrahierter Text-Blobs beim Lesen überprüfen

# §8.18 Lenses → Audio.
settings-la-enable = Audiolupe aktivieren
settings-la-lufs-ref = LUFS-Referenzstandard
settings-la-peak-compute = Spitzenwert berechnen über
settings-la-silence-thresh = Stilleschwelle
settings-la-re-extract-modify = Bei Änderungsereignis neu extrahieren

# §8.19 Lenses → Similarity.
settings-ls-enable = Ähnlichkeitslupe aktivieren
settings-ls-sig-size = MinHash-Signaturgröße (k)
settings-ls-bands = LSH-Bänder
settings-ls-recall = Recall-Schwelle
settings-ls-result-cap = Ergebnisobergrenze

# §8.20 Lenses → Custom.
settings-custom-registry = Registry
settings-custom-trust = Vertrauen
settings-custom-refresh-hashes = Hashes aktualisieren

# §8.21-§8.22 Network.
settings-net-https-enable = HTTPS-Server aktivieren
settings-net-bind = An Schnittstellen binden
settings-net-port = An Port lauschen
settings-net-force-https = HTTPS erzwingen
settings-net-legacy-auth = Veraltete HTTP-Basic-Authentifizierung
settings-net-token-regen = Token neu generieren
settings-net-api-enable = API-Server aktivieren
settings-net-legacy-ftp = Unterstützung für unverschlüsseltes FTP/ETP (veraltet)

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Automatische Updates
settings-privacy-prerelease = Vorabversions-Kanal
settings-privacy-network-policy = Richtlinie für Netzwerkverbindungen

# §8.24 Logs & Debug.
settings-logs-level = Protokollstufe
settings-logs-location = Speicherort der Protokolldatei
settings-logs-retention = Aufbewahrung der Protokolle
settings-logs-debug-overlay = Debug-Overlay anzeigen
settings-logs-open-folder = Protokollordner öffnen
settings-logs-export-bundle = Diagnose-Bundle exportieren

# §8.25 Backup, Export, Reset.
settings-backup-export = Einstellungen exportieren
settings-backup-import = Einstellungen importieren
settings-backup-export-bookmarks = Lesezeichen-Bundle exportieren
settings-backup-import-bookmarks = Lesezeichen-Bundle importieren
settings-backup-reset-all = Alle Einstellungen auf Standardwerte zurücksetzen

# §8.26 Locale.
settings-locale-current = Aktuelle Sprache
settings-locale-rtl-preview = RTL-Vorschau
settings-locale-date-format = Datumsformat
settings-locale-number-format = Zahlenformat

# §8.27 About.
settings-about-version = Freally { $version }
settings-about-license = Lizenz
settings-about-credits = Mitwirkende
settings-about-notices = Open-Source-Hinweise

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = Assistent für den ersten Start
wizard-step-of-total = Schritt { $step } von { $total }
wizard-roots-hint = Fügen Sie die Ordner oder Datenträger hinzu, die Freally überwachen soll. Sie können dies später in den Index-Einstellungen ändern.
wizard-browse = Durchsuchen…
wizard-roots-placeholder = …oder einen Pfad einfügen
wizard-roots-add = Hinzufügen
wizard-roots-remove = Entfernen
wizard-roots-empty = Noch keine Quellen konfiguriert.
wizard-locale-hint = Freally ist in 18 Sprachen verfügbar. Sie können später wechseln.
wizard-theme-hint = System folgt der Darstellungseinstellung Ihres Betriebssystems.
wizard-back = Zurück
wizard-next = Weiter

# Status bar polish.
statusbar-hotkey-hint = Tastenkürzel: { $hotkey }
statusbar-cycle-theme = Design wechseln
statusbar-indexed-suffix = indiziert

# Results / lenses.
lens-expand = Lupe aufklappen
lens-collapse = Lupe einklappen
lens-no-matches = Keine Treffer in dieser Lupe.

# Preview pane.
preview-header = Vorschau
preview-loading = Wird geladen…
preview-select-file = Wählen Sie eine Datei zur Vorschau aus.
preview-unavailable = Keine Vorschau verfügbar

# Bookmarks.
bookmarks-label = ★ Lesezeichen
bookmarks-empty-hint = Noch keine Lesezeichen. Drücken Sie Ctrl+D, um die aktuelle Abfrage zu speichern.
bookmarks-organize-title = Lesezeichen verwalten
bookmarks-organize-empty = Noch keine Lesezeichen.
bookmarks-rename = Umbenennen
bookmarks-close = Schließen

# Settings tree extras.
settings-group-history = Verlauf
settings-group-privacy = Datenschutz & Updates
settings-group-logs = Protokolle & Debug
settings-group-backup = Sichern, Exportieren, Zurücksetzen
settings-tree-custom-lens = Benutzerdefiniert
settings-unsaved-changes = ungespeicherte Änderungen

# About dialog.
about-dialog-title = Freally
about-copyright = Copyright © 2026 Mike Weaver. Alle Rechte vorbehalten.
about-close = Schließen

# Connect endpoint dialog.
connect-ftp-title = Mit FTP-Server verbinden
connect-ftp-host = Host:
connect-ftp-port = Port:
connect-ftp-username = Benutzername:
connect-ftp-password = Passwort:
connect-ftp-link-type = Verbindungstyp:

# UI panel.
ui-hint = Design, Integration in Infobereich/Menüleiste, Suchen während der Eingabe, Zeilendichte. Direkte Übereinstimmung mit voidtools Everything sowie Freally-Ergänzungen, gekennzeichnet mit (+).
ui-section-theme = Design
ui-theme-system-default = System (Standard)
ui-section-tray = Infobereich/Menüleiste
ui-section-search-behavior = Suchverhalten
ui-section-result-rows = Ergebniszeilen
ui-single-click-system-default = Systemeinstellungen (Standard)
ui-single-click-always = Immer einfacher Klick
ui-single-click-always-double = Immer Doppelklick
ui-underline-always = Immer
ui-underline-on-hover = Beim Überfahren
ui-underline-never = Nie

# Home panel.
home-hint = Standardwerte werden beim App-Start geladen — jedes Dropdown kann auf „Letzten Wert verwenden“ bleiben oder einen festen Wert beibehalten. Lupensichtbarkeit/Ergebnisgrenzen sind Freally-Ergänzungen (+).
home-section-match = Übereinstimmungs-Standardwerte
home-section-search-sort = Such- & Sortier-Standardwerte
home-search-placeholder = Standardmäßig leer
home-section-index = Indexquelle
home-file-list-path = Pfad der Dateiliste
home-https-endpoint = URL des HTTPS-API-Endpunkts
home-endpoint-token = Token (Fingerabdruck angezeigt)

# Backup panel.
backup-section-settings = Einstellungen (+)
backup-section-bookmarks = Lesezeichen + benutzerdefinierte Extraktoren (+)
backup-section-reset = Zurücksetzen
backup-toast-exported = Einstellungen nach { $path } exportiert
backup-toast-export-failed = Export fehlgeschlagen: { $error }
backup-toast-imported = Einstellungen importiert
backup-toast-import-failed = Import fehlgeschlagen: { $error }
backup-toast-bookmarks-exported = Lesezeichen exportiert
backup-toast-bookmarks-export-failed = Lesezeichen-Export fehlgeschlagen: { $error }
backup-toast-bookmarks-imported = Lesezeichen importiert
backup-toast-bookmarks-import-failed = Lesezeichen-Import fehlgeschlagen: { $error }
backup-confirm-reset = Alle Einstellungen auf Standardwerte zurücksetzen? Dies kann nicht rückgängig gemacht werden (der Dialog bleibt geöffnet).
backup-toast-reset = Alle Einstellungen zurückgesetzt

# Keyboard panel.
keyboard-section-global = Globale Tastenkürzel
keyboard-placeholder-example = Super+Space
keyboard-section-commands = Befehle
keyboard-placeholder-command = Befehls-ID (z. B. file.export_results)
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = Suchverlauf
history-section-run = Ausführungsverlauf
history-section-privacy = Datenschutz (+)
history-record-filename = Verlauf der Dateinamenlupe aufzeichnen
history-record-content = Verlauf der Inhaltslupe aufzeichnen
history-record-audio = Verlauf der Audiolupe aufzeichnen
history-record-similarity = Verlauf der Ähnlichkeitslupe aufzeichnen

# Locale panel.
locale-section-language = Sprache (+)
locale-section-time-date = Uhrzeit/Datum (+)
locale-date-os = Betriebssystem-Standard
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = Benutzerdefiniert
locale-date-custom-format = Benutzerdefiniertes Format
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = Zahlen (+)
locale-number-os = Betriebssystem-Standard
locale-number-custom = Benutzerdefiniert
locale-thousands-sep = Tausendertrennzeichen
locale-decimal-sep = Dezimaltrennzeichen

# Folders panel.
folders-hint = Zusätzliche überwachte Ordner über die Standard-Datenträger hinaus.
folders-list-title = Überwachte Ordner
folders-empty = Noch keine Ordner hinzugefügt.
folders-remove = Entfernen
folders-section-title-dynamic = Einstellungen für { $path }
folders-section-schedule = Zeitplan für Neuscan
folders-schedule-daily = Täglich um HH:MM
folders-schedule-hours = Alle N Stunden
folders-schedule-never = Nie
folders-hour = Stunde
folders-minute = Minute
folders-hours = Stunden
folders-id-label = Ordner-ID (schreibgeschützt)
folders-select-prompt = Wählen Sie einen Ordner aus, um ihn zu konfigurieren.
folders-section-extras = Freally-Extras (+)
folders-extras-note = Der Neuscan bei der Rückkehr aus dem Ruhezustand ist in diesem Build standardmäßig aktiviert; der Schalter wird im Feinschliff von Phase 13 zu den Steuerelementen auf Ordnerebene hinzugefügt.

# Volumes panel.
volumes-hint = Plattformübergreifendes Gegenstück zu den NTFS-/ReFS-Bereichen von voidtools Everything. Erkennt automatisch NTFS/ReFS/exFAT/FAT32 (Win), APFS/HFS+ (macOS), ext4/Btrfs/ZFS/XFS/F2FS (Linux).
volumes-section-auto-include = Automatisch einbeziehen
volumes-list-title = Erkannte Datenträger
volumes-detecting = Wird erkannt…
volumes-empty = Keine Datenträger erkannt.
volumes-select-prompt = Wählen Sie einen Datenträger aus, um ihn zu konfigurieren.

# About panel polish.
about-section-version = Version (+)
about-section-license = Lizenz (+)
about-license-text = Mike Weaver — Alle Rechte vorbehalten. Dies ist proprietäre Software.
about-license-spdx = SPDX: { $spdx }
about-section-credits = Mitwirkende (+)
about-credits-inspired = Inspiriert von Everything von voidtools.
about-credits-voidtools = voidtools.com
about-credits-repo = Projekt-Repository

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = Enthält Befehle für die Arbeit mit Freally.
menu-file-new-window = Neues Suchfenster
menu-file-open-list = Dateiliste öffnen…
menu-file-close-list = Dateiliste schließen
menu-file-close = Schließen
menu-file-export-results = Ergebnisse exportieren…
menu-file-export-bundle = Index-Bundle exportieren…
menu-file-exit = Beenden

# Edit menu.
menu-edit-hint = Enthält Befehle zum Bearbeiten von Suchergebnissen.
menu-edit-cut = Ausschneiden
menu-edit-copy = Kopieren
menu-edit-paste = Einfügen
menu-edit-copy-to-folder = In Ordner kopieren…
menu-edit-move-to-folder = In Ordner verschieben…
menu-edit-select-all = Alles auswählen
menu-edit-invert-selection = Auswahl umkehren
menu-edit-advanced = Erweitert
menu-edit-copy-full-name = Vollständigen Namen kopieren
menu-edit-copy-path = Pfad kopieren
menu-edit-copy-filename = Dateinamen kopieren
menu-edit-copy-as-json = Als JSON kopieren
menu-edit-copy-with-metadata = Mit Metadaten kopieren
menu-edit-copy-as-bundle-ref = Als Freally-Bundle-Referenz kopieren

# View menu.
menu-view-hint = Enthält Befehle zur Anpassung der Ansicht.
menu-view-filters = Filter
menu-view-preview = Vorschau
menu-view-status-bar = Statusleiste
menu-view-thumbs-xl = Extragroße Miniaturansichten
menu-view-thumbs-l = Große Miniaturansichten
menu-view-thumbs-m = Mittlere Miniaturansichten
menu-view-details = Details
menu-view-window-size = Fenstergröße
menu-view-window-size-hint = Enthält Befehle zur Anpassung der Fenstergröße.
menu-view-window-small = Klein
menu-view-window-medium = Mittel
menu-view-window-large = Groß
menu-view-window-auto = Automatisch anpassen
menu-view-zoom = Zoom
menu-view-zoom-hint = Enthält Befehle zur Anpassung der Schrift- und Symbolgröße.
menu-view-zoom-in = Vergrößern
menu-view-zoom-out = Verkleinern
menu-view-zoom-reset = Zurücksetzen
menu-view-sort-by = Sortieren nach
menu-view-sort-by-hint = Enthält Befehle zum Sortieren der Ergebnisliste.
menu-view-sort-name = Name
menu-view-sort-path = Pfad
menu-view-sort-size = Größe
menu-view-sort-ext = Erweiterung
menu-view-sort-type = Typ
menu-view-sort-modified = Änderungsdatum
menu-view-sort-created = Erstellungsdatum
menu-view-sort-accessed = Zugriffsdatum
menu-view-sort-attributes = Attribute
menu-view-sort-recently-changed = Datum der letzten Änderung
menu-view-sort-run-count = Ausführungsanzahl
menu-view-sort-run-date = Ausführungsdatum
menu-view-sort-file-list-filename = Dateilisten-Dateiname
menu-view-sort-lufs = LUFS
menu-view-sort-length = Länge
menu-view-sort-similarity = Ähnlichkeitswert
menu-view-sort-asc = Aufsteigend
menu-view-sort-desc = Absteigend
menu-view-go-to = Gehe zu
menu-view-refresh = Aktualisieren
menu-view-theme = Design
menu-view-theme-hint = Zwischen System-, hellem oder dunklem Design wechseln.
menu-view-lenses = Lupen
menu-view-lenses-hint = Sichtbarkeit jeder Lupe in der Ergebnisliste umschalten.
menu-view-on-top = Im Vordergrund
menu-view-on-top-hint = Enthält Befehle, um dieses Fenster über anderen Fenstern zu halten.
menu-view-on-top-never = Nie
menu-view-on-top-always = Immer
menu-view-on-top-while-searching = Während der Suche

# Search menu.
menu-search-hint = Enthält Suchschalter.
menu-search-match-case = Groß-/Kleinschreibung beachten
menu-search-match-whole-word = Ganzes Wort suchen
menu-search-match-path = Pfad einbeziehen
menu-search-match-diacritics = Diakritische Zeichen beachten
menu-search-enable-regex = Regulären Ausdruck aktivieren
menu-search-advanced = Erweiterte Suche…
menu-search-add-to-filters = Zu Filtern hinzufügen…
menu-search-organize-filters = Filter verwalten…
menu-search-filter-everything = Everything
menu-search-filter-archive = Komprimiert (Archiv)
menu-search-filter-folder = Ordner
menu-search-filter-custom = Benutzerdefinierter Filter…

# Bookmarks menu.
menu-bookmarks-hint = Enthält Befehle für die Arbeit mit Lesezeichen.
menu-bookmarks-add = Zu Lesezeichen hinzufügen
menu-bookmarks-organize = Lesezeichen verwalten…

# Tools menu.
menu-tools-hint = Enthält Befehle für Extras.
menu-tools-connect = Mit FTP-Server verbinden…
menu-tools-disconnect = Verbindung zum FTP-Server trennen
menu-tools-file-list-editor = Dateilisten-Editor…
menu-tools-index-maintenance = Indexwartung
menu-tools-index-maintenance-hint = Werkzeuge zur Indexwartung.
menu-tools-verify-index = Index überprüfen…
menu-tools-compact-index = Index komprimieren…
menu-tools-rebuild-index = Neuaufbau des Index erzwingen…
menu-tools-custom-extractor = Verwaltung benutzerdefinierter Extraktoren…
menu-tools-custom-extractor-hint = Wasm-sandboxed benutzerdefinierte Extraktoren verwalten.
menu-tools-options = Optionen…

# Help menu.
menu-help-hint = Enthält Hilfebefehle.
menu-help-help = Freally-Hilfe
menu-help-search-syntax = Suchsyntax
menu-help-regex-syntax = Regex-Syntax
menu-help-audio-ref = Referenz für Audio-Modifikatoren
menu-help-similarity-ref = Referenz für Ähnlichkeits-Modifikatoren
menu-help-cli-options = Befehlszeilenoptionen
menu-help-website = Freally-Website
menu-help-check-updates = Nach Updates suchen…
menu-help-sponsor = Sponsern/Spenden
menu-help-about = Über Freally…

# Result column headers (short forms used in the table header row).
column-name = Name
column-path = Pfad
column-size = Größe
column-modified = Geändert
column-type = Typ
column-ext = Erw.
column-sort-by = Nach { $name } sortieren
column-resize = Spalte { $name } anpassen

# Section subtitle bars used inside multiple settings panels.
section-behavior = Verhalten
section-rendering = Darstellung
section-status-bar = Statusleiste
section-display-format = Anzeigeformat
section-loading-priority = Ladepriorität
section-compatibility = Kompatibilität
section-storage = Speicher
section-index-fields = Indexfelder
section-maintenance = Wartung
section-logging = Protokollierung
section-tools = Extras
section-privacy = Datenschutz
section-auto-update = Automatische Updates (+)
section-bind = Binden
section-lens = Lupe
section-budgets = Budgets
section-other = Sonstiges
section-per-format-mode = Modus pro Format
section-loudness = Lautheit
section-tuning = Feinabstimmung (+)
section-minhash-lsh = MinHash- + LSH-Parameter (+)
section-top-level = Oberste Ebene
section-file-globs = Datei-Globs
section-file-list-settings = Einstellungen für ausgewählte Dateiliste
section-editor-format = Editor + Format (E + +)
section-api-server = API-Server (E angepasst)
section-freally-extras = Freally-Extras (+)
section-freally-additions = Freally-Ergänzungen (+)
section-freally-extensions = Freally-Erweiterungen (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = Letzten Wert verwenden
opt-use-last-value-default = Letzten Wert verwenden (Standard)
opt-low = Niedrig
opt-normal-default = Normal (Standard)
opt-high = Hoch
opt-disabled = Deaktiviert
opt-off = Aus
opt-on-battery = Im Akkubetrieb
opt-always = Immer
opt-clamp-default = Begrenzen (Standard)
opt-wrap = Umbrechen
opt-none = Keine
opt-strict-refuse = Strikt (Abfragen bei Beschädigung ablehnen)
opt-lenient-warn = Nachsichtig (warnen, aber abfragen)
opt-system-default = Systemstandard
opt-drag-select = Ziehen zum Auswählen
opt-auto-binary = Automatisch (binär)
opt-auto-decimal = Automatisch (dezimal)

# Unit suffixes shown next to number inputs.
unit-days = Tage
unit-b = B
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-tb = TB

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = Sofort
opt-lazy-default = Verzögert (Standard)
opt-on = Ein
opt-on-default = Ein (Standard)
opt-all = Alle
opt-weekly = Wöchentlich
opt-monthly = Monatlich
opt-name-asc = Name aufst.
opt-name-desc = Name abst.
opt-size-asc = Größe aufst.
opt-size-desc = Größe abst.
opt-modified-asc = Änderungsdatum aufst.
opt-modified-desc = Änderungsdatum abst.
opt-compact = Kompakt
opt-comfortable = Komfortabel
opt-details = Details
opt-thumbnails = Miniaturansichten
opt-local-db-default = Lokale Datenbank (Standard)
opt-file-list = Dateiliste
opt-https-endpoint = HTTPS-API-Endpunkt
opt-right-default = Rechts (Standard)
opt-bottom = Unten
opt-or-and-default = OR > AND (Standard)
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128 (Standard)
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = Broadcast-Film (-23)
opt-true-peak = True Peak (4× Oversampling, Standard)
opt-sample-peak = Sample Peak
opt-auto-per-doc = Automatisch (pro Dokument)
opt-log-error = Error
opt-log-warn = Warn
opt-log-info-default = Info (Standard)
opt-log-debug = Debug
opt-log-trace = Trace

# More Freally apps (Central inside panel) — host chrome
menu-help-more-apps = Weitere Freally-Apps…
moreapps-title = Weitere Freally-Apps
