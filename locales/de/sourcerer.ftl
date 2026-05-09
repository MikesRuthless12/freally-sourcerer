# Sourcerer — Deutsch.

app-name = Sourcerer
tagline = Eine Suche. Jede Quelle. Jedes Betriebssystem.
window-title = Sourcerer
search-placeholder = Suchen…
about-version = Version { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = Bereit
status-indexed = Indiziert ({ $count } Dateien)
status-indexing = Indizierung läuft… { $done }/{ $total }
status-paused = Pausiert
status-error = Fehler
status-result-count-one = { $count } Ergebnis
status-result-count-many = { $count } Ergebnisse
status-selection = · { $count } ausgewählt
status-selection-size = Auswahl: { $size }
status-query-timing = Abfrage: { $ms } ms
status-endpoint-local = Lokale DB
status-endpoint-remote = API: { $name }

menu-file = Datei
menu-edit = Bearbeiten
menu-view = Ansicht
menu-search = Suche
menu-bookmarks = Lesezeichen
menu-tools = Werkzeuge
menu-help = Hilfe

theme-system = System
theme-light = Hell
theme-dark = Dunkel

lens-filename = Dateiname
lens-content = Inhalt
lens-audio = Audio
lens-similarity = Ähnlichkeit

parse-error-empty = Geben Sie eine Abfrage ein, um zu beginnen.
parse-error-unknown = Unbekannte Syntax an dieser Stelle.

action-open = Öffnen
action-reveal = Im Ordner anzeigen
action-copy-path = Pfad kopieren
action-copy-name = Name kopieren
action-delete = Löschen

quick-filter-audio = Audio
quick-filter-video = Video
quick-filter-image = Bild
quick-filter-document = Dokument
quick-filter-executable = Ausführbar
quick-filter-archive = Archiv

wizard-title = Willkommen bei Sourcerer
wizard-step-roots = Indizierungsumfang wählen
wizard-step-hotkey = Globalen Hotkey festlegen
wizard-step-locale = Sprache wählen
wizard-step-theme = Design wählen
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
settings-group-lenses = Lenses
settings-group-network = Netzwerk

# Tree nav leaves.
settings-node-ui = Benutzeroberfläche
settings-node-home = Start
settings-node-search = Suche
settings-node-results = Ergebnisse
settings-node-view = Ansicht
settings-node-context-menu = Kontextmenü
settings-node-fonts-colors = Schriften & Farben
settings-node-keyboard = Tastatur
settings-node-history = Verlauf
settings-node-indexes-top = (Oberste Ebene)
settings-node-volumes = Datenträger
settings-node-folders = Ordner
settings-node-file-lists = Dateilisten
settings-node-exclude = Ausschließen
settings-node-https-server = HTTP- / HTTPS-Server
settings-node-etp-api = ETP- / FTP-API
settings-node-privacy = Datenschutz & Updates
settings-node-logs = Protokolle & Debug
settings-node-backup = Sicherung, Export, Zurücksetzen
settings-node-locale = Sprache & Region
settings-node-about = Über

# §8.2 General → UI.
settings-ui-theme = Design
settings-ui-run-bg = Im Hintergrund ausführen
settings-ui-show-tray = Tray- / Menüleisten-Symbol anzeigen
settings-ui-single-click-tray = Einfacher Klick auf Tray / Menüleiste
settings-ui-new-window-from-tray = Neues Fenster über Tray-Symbol öffnen
settings-ui-new-window-on-launch = Beim Start von Sourcerer neues Fenster öffnen
settings-ui-search-as-you-type = Sofortsuche während der Eingabe
settings-ui-select-on-mouse-click = Suche bei Mausklick auswählen
settings-ui-focus-on-activate = Suchfeld bei Aktivierung fokussieren
settings-ui-full-row-select = Ganze Zeile auswählen
settings-ui-single-click-open = Mit einfachem Klick öffnen
settings-ui-underline-titles = Symboltitel unterstreichen
settings-ui-row-density = Ergebnisdichte
settings-ui-row-density-compact = Kompakt (32 px)
settings-ui-row-density-comfortable = Komfortabel (44 px)
settings-ui-show-timing-badges = Timing-Abzeichen pro Lens anzeigen
settings-ui-anim-crossfade = Animierter Designwechsel mit Überblendung

# §8.3 General → Home.
settings-home-match-case = Groß-/Kleinschreibung beachten
settings-home-match-whole-word = Ganzes Wort suchen
settings-home-match-path = Pfad einbeziehen
settings-home-match-diacritics = Diakritische Zeichen beachten
settings-home-match-regex = Regex anwenden
settings-home-search = Suche (benutzerdefinierte Standardabfrage)
settings-home-filter = Filter
settings-home-sort = Sortierung
settings-home-view = Ansicht
settings-home-index = Index
settings-home-default-lens-visibility = Standard-Sichtbarkeit der Lenses
settings-home-default-lens-result-limits = Standard-Ergebnislimits der Lenses

# §8.4 General → Search.
settings-search-fast-ascii = Schnelle ASCII-Suche
settings-search-mp-sep = Pfad einbeziehen, wenn Suchbegriff einen Pfadtrenner enthält
settings-search-mw-fn = Vollständigen Dateinamen bei Wildcards prüfen
settings-search-lit-ops = Literale Operatoren erlauben
settings-search-paren = Gruppierung mit runden Klammern erlauben
settings-search-env = Umgebungsvariablen auflösen
settings-search-fwd-slash = Schrägstriche durch Rückstriche ersetzen
settings-search-precedence = Operator-Vorrang
settings-search-strict-everything = Strikter Everything-Syntaxmodus
settings-search-auto-regex = Regex automatisch erkennen
settings-search-mod-comp = Modifier-Vervollständigung
settings-search-parse-tree = Parse-Baum bei Mauszeiger anzeigen

# §8.5 General → Results.
settings-results-hide-empty = Ergebnisse bei leerer Suche ausblenden
settings-results-clear-on-search = Auswahl bei neuer Suche aufheben
settings-results-close-on-execute = Fenster nach Ausführen schließen
settings-results-dbl-path = Pfad mit Doppelklick in Pfadspalte öffnen
settings-results-auto-scroll = Ansicht automatisch scrollen
settings-results-dquote-copy = Beim Kopieren als Pfad in Anführungszeichen setzen
settings-results-no-ext-rename = Erweiterung beim Umbenennen nicht markieren
settings-results-sort-date-desc = Datum zuerst absteigend sortieren
settings-results-sort-size-desc = Größe zuerst absteigend sortieren
settings-results-list-focus = Ergebnisliste-Fokus
settings-results-icon-prio = Priorität für Symbol-Ladevorgang
settings-results-thumb-prio = Priorität für Vorschaubild-Ladevorgang
settings-results-ext-prio = Priorität für erweiterte Informationen
settings-results-group-by-lens = Ergebnisse nach Lens gruppieren
settings-results-snippet-inline = Snippet-Vorschau inline anzeigen

# §8.6 General → View.
settings-view-double-buffer = Doppelpufferung
settings-view-alt-rows = Zeilen abwechselnd einfärben
settings-view-row-mouseover = Zeilen-Hervorhebung bei Mauszeiger
settings-view-highlight-terms = Suchbegriffe hervorheben
settings-view-status-show-selected = Ausgewähltes Element in Statusleiste anzeigen
settings-view-rc-with-sel = Ergebnisanzahl zusammen mit Auswahl anzeigen
settings-view-status-show-size = Größe in Statusleiste anzeigen
settings-view-tooltips = Tooltips anzeigen
settings-view-update-on-scroll = Anzeige sofort beim Scrollen aktualisieren
settings-view-size-format = Größenformat
settings-view-selection-rect = Auswahlrechteck
settings-view-audio-badges = LUFS- / Codec- / Längen-Abzeichen in Audio-Zeilen anzeigen
settings-view-similarity-score = MinHash-Ähnlichkeitswert in Ähnlichkeits-Zeilen anzeigen
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
settings-context-menu-explore = Erkunden
settings-context-menu-explore-path = Pfad erkunden
settings-context-menu-copy-name = Name in Zwischenablage kopieren
settings-context-menu-copy-path = Pfad in Zwischenablage kopieren
settings-context-menu-copy-full-name = Vollständigen Namen in Zwischenablage kopieren
settings-context-menu-reveal = In Sourcerer anzeigen
settings-context-menu-send-to = An Sourcerer senden (Pfad)

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
settings-fc-per-lens-accent = Akzent pro Lens
settings-fc-theme-inherit = Eigene Farben beim Designwechsel automatisch invertieren

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Globaler Hotkey
settings-keyboard-new-window = Hotkey für neues Fenster
settings-keyboard-show-window = Hotkey zum Einblenden des Fensters
settings-keyboard-toggle-window = Hotkey zum Umschalten des Fensters
settings-keyboard-show-commands = Befehle anzeigen, die Folgendes enthalten
settings-keyboard-add-chord = + Kombination hinzufügen
settings-keyboard-remove-chord = Entfernen

# §8.10 History.
settings-history-search-enable = Suchverlauf aktivieren
settings-history-search-keep = Suchverlauf für { $days } Tage aufbewahren
settings-history-run-enable = Ausführungsverlauf aktivieren
settings-history-run-keep = Ausführungsverlauf für { $days } Tage aufbewahren
settings-history-clear-now = Jetzt löschen
settings-history-privacy-mode = Datenschutzmodus
settings-history-per-lens = Verlauf pro Lens

# §8.11 Indexes (top-level).
settings-ix-database-location = Datenbank-Speicherort
settings-ix-multiuser = Dateiname der Mehrbenutzer-Datenbank
settings-ix-compress = Datenbank komprimieren
settings-ix-recent-changes = Letzte Änderungen indizieren
settings-ix-file-size = Dateigröße indizieren
settings-ix-fast-size-sort = Schnelle Sortierung nach Größe
settings-ix-folder-size = Ordnergröße indizieren
settings-ix-fast-folder-size-sort = Schnelle Sortierung nach Ordnergröße
settings-ix-date-created = Erstellungsdatum indizieren
settings-ix-fast-date-created = Schnelle Sortierung nach Erstellungsdatum
settings-ix-date-modified = Änderungsdatum indizieren
settings-ix-fast-date-modified = Schnelle Sortierung nach Änderungsdatum
settings-ix-date-accessed = Zugriffsdatum indizieren
settings-ix-fast-date-accessed = Schnelle Sortierung nach Zugriffsdatum
settings-ix-attributes = Attribute indizieren
settings-ix-fast-attributes = Schnelle Sortierung nach Attributen
settings-ix-fast-path-sort = Schnelle Sortierung nach Pfad
settings-ix-fast-extension-sort = Schnelle Sortierung nach Erweiterung
settings-ix-force-rebuild = Neuaufbau erzwingen
settings-ix-compact = Index verdichten
settings-ix-verify = Index überprüfen
settings-ix-integrity-policy = Richtlinie zur Index-Integrität
settings-ix-memory-budget = Speicherbudget des Indexers
settings-ix-throttle = Drosselung der Hintergrundindizierung

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Neue feste Datenträger automatisch einbeziehen
settings-vol-auto-removable = Neue Wechseldatenträger automatisch einbeziehen
settings-vol-auto-remove-offline = Offline-Datenträger automatisch entfernen
settings-vol-detected = Erkannte Datenträger
settings-vol-include = In Index aufnehmen
settings-vol-include-only = Nur einbeziehen (glob/Regex)
settings-vol-enable-usn = USN-Journal aktivieren
settings-vol-enable-fsevents = FSEvents-Stream aktivieren
settings-vol-enable-inotify = inotify aktivieren (oder fanotify mit erhöhten Rechten)
settings-vol-buffer = Journal-Puffergröße (KB)
settings-vol-allocation-delta = Allokations-Delta (KB)
settings-vol-load-recent = Letzte Änderungen beim Start aus dem Journal laden
settings-vol-monitor = Änderungen überwachen
settings-vol-recreate-journal = Journal neu erstellen
settings-vol-reset-stream = FSEvents-Stream zurücksetzen
settings-vol-upgrade-fanotify = Auf fanotify aufrüsten (polkit)
settings-vol-remove = Entfernen

# §8.13 Indexes → Folders.
settings-folders-watched = Überwachte Ordner
settings-folders-add = Hinzufügen…
settings-folders-rescan-now = Jetzt erneut scannen
settings-folders-rescan-all = Alle jetzt erneut scannen
settings-folders-monitor = Änderungen nach Möglichkeit überwachen
settings-folders-buffer = Puffergröße
settings-folders-rescan-on-full = Bei vollem Puffer erneut scannen

# §8.14 Indexes → File Lists.
settings-flists-add = Hinzufügen…
settings-flists-monitor = Änderungen überwachen
settings-flists-editor = Dateilisten-Editor…
settings-flists-format = Dateilistenformat
settings-flists-format-text = Text (ein Pfad pro Zeile)
settings-flists-format-json = JSON (mit Metadaten)
settings-flists-format-srcb = Sourcerer Bundle (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Versteckte Dateien und Ordner ausschließen
settings-exclude-system = Systemdateien und -ordner ausschließen
settings-exclude-list-en = Ausschlussliste aktivieren
settings-exclude-folders = Ordner ausschließen
settings-exclude-include-only-files = Nur folgende Dateien einbeziehen (glob)
settings-exclude-files = Dateien ausschließen (glob)
settings-exclude-os-recommended = Vom Betriebssystem empfohlene Ausschlüsse anwenden
settings-exclude-by-class = Nach Erweiterungsklasse ausschließen

# §8.16 Lenses → Filename.
settings-lf-trigram = Aggressivität des Trigram-Vorfilters
settings-lf-suffix-mem = Speicherbudget für Suffix-Array
settings-lf-wildcard-limit = Limit für Wildcard-Erweiterung
settings-lf-regex-timeout = Regex-Timeout

# §8.17 Lenses → Content.
settings-lc-enable = Inhalts-Lens aktivieren
settings-lc-time-budget = Zeitbudget pro Dokument
settings-lc-mem-ceiling = Speicherobergrenze pro Dokument
settings-lc-snippet-len = Snippet-Länge
settings-lc-stop-words = Sprache der Stoppwörter
settings-lc-re-extract = Bei Einstellungsänderung neu extrahieren
settings-lc-verify-blobs = Prüfsummen extrahierter Text-Blobs beim Lesen verifizieren

# §8.18 Lenses → Audio.
settings-la-enable = Audio-Lens aktivieren
settings-la-lufs-ref = LUFS-Referenzstandard
settings-la-peak-compute = Spitzenwert berechnen über
settings-la-silence-thresh = Stille-Schwellenwert
settings-la-re-extract-modify = Bei Modify-Ereignis neu extrahieren

# §8.19 Lenses → Similarity.
settings-ls-enable = Ähnlichkeits-Lens aktivieren
settings-ls-sig-size = MinHash-Signaturgröße (k)
settings-ls-bands = LSH-Bänder
settings-ls-recall = Recall-Schwellenwert
settings-ls-result-cap = Ergebnisobergrenze

# §8.20 Lenses → Custom.
settings-custom-registry = Registry
settings-custom-trust = Vertrauen
settings-custom-refresh-hashes = Hashes aktualisieren

# §8.21-§8.22 Network.
settings-net-https-enable = HTTPS-Server aktivieren
settings-net-bind = An Schnittstellen binden
settings-net-port = Port abhören
settings-net-force-https = HTTPS erzwingen
settings-net-legacy-auth = Legacy-Authentifizierung über HTTP-Basic
settings-net-token-regen = Token neu generieren
settings-net-api-enable = API-Server aktivieren
settings-net-legacy-ftp = Unterstützung für klassisches FTP/ETP

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Automatische Updates
settings-privacy-prerelease = Vorab-Versionen
settings-privacy-network-policy = Richtlinie für Netzwerkaufrufe

# §8.24 Logs & Debug.
settings-logs-level = Protokollstufe
settings-logs-location = Speicherort der Protokolldatei
settings-logs-retention = Aufbewahrung der Protokolle
settings-logs-debug-overlay = Debug-Overlay anzeigen
settings-logs-open-folder = Protokollordner öffnen
settings-logs-export-bundle = Diagnose-Paket exportieren

# §8.25 Backup, Export, Reset.
settings-backup-export = Einstellungen exportieren
settings-backup-import = Einstellungen importieren
settings-backup-export-bookmarks = Lesezeichen-Paket exportieren
settings-backup-import-bookmarks = Lesezeichen-Paket importieren
settings-backup-reset-all = Alle Einstellungen auf Standardwerte zurücksetzen

# §8.26 Locale.
settings-locale-current = Aktuelle Sprache
settings-locale-rtl-preview = RTL-Vorschau
settings-locale-date-format = Datumsformat
settings-locale-number-format = Zahlenformat

# §8.27 About.
settings-about-version = Sourcerer { $version }
settings-about-license = Lizenz
settings-about-credits = Mitwirkende
settings-about-notices = Open-Source-Hinweise
