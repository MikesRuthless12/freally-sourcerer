# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = Una ricerca. Ogni sorgente. Ogni sistema operativo.
window-title = Freally Sourcerer
search-placeholder = Cerca…
about-version = Versione { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = Pronto
status-indexed = Indicizzati ({ $count } file)
status-indexing = Indicizzazione… { $done }/{ $total }
status-paused = In pausa
status-error = Errore
status-result-count-one = { $count } risultato
status-result-count-many = { $count } risultati
status-selection = · { $count } selezionati
status-selection-size = Selezionati: { $size }
status-query-timing = Query: { $ms } ms
status-endpoint-local = DB locale
status-endpoint-remote = API: { $name }

menu-file = File
menu-edit = Modifica
menu-view = Visualizza
menu-search = Cerca
menu-bookmarks = Segnalibri
menu-tools = Strumenti
menu-help = Aiuto

theme-system = Sistema
theme-light = Chiaro
theme-dark = Scuro

lens-filename = Nome file
lens-content = Contenuto
lens-audio = Audio
lens-similarity = Somiglianza

parse-error-empty = Digita una query per iniziare.
parse-error-unknown = Sintassi non riconosciuta qui.

action-open = Apri
action-reveal = Mostra nella cartella
action-copy-path = Copia percorso
action-copy-name = Copia nome
action-delete = Elimina

quick-filter-audio = Audio
quick-filter-video = Video
quick-filter-image = Immagine
quick-filter-document = Documento
quick-filter-executable = Eseguibile
quick-filter-archive = Archivio

wizard-title = Benvenuto in Freally
wizard-step-roots = Scegli cosa indicizzare
wizard-step-hotkey = Scegli una scorciatoia globale
wizard-step-locale = Scegli la lingua
wizard-step-theme = Scegli un tema
wizard-finish = Fine

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = Opzioni
settings-search-placeholder = Cerca opzioni…
settings-restore-defaults = Ripristina predefiniti
settings-ok = OK
settings-cancel = Annulla
settings-apply = Applica

# Tree nav groups (PRD §8.1.1).
settings-group-general = Generale
settings-group-indexes = Indici
settings-group-lenses = Lenti
settings-group-network = Rete

# Tree nav leaves.
settings-node-ui = Interfaccia
settings-node-home = Home
settings-node-search = Ricerca
settings-node-results = Risultati
settings-node-view = Visualizzazione
settings-node-context-menu = Menu contestuale
settings-node-fonts-colors = Caratteri e colori
settings-node-keyboard = Tastiera
settings-node-history = Cronologia
settings-node-indexes-top = (primo livello)
settings-node-volumes = Volumi
settings-node-folders = Cartelle
settings-node-file-lists = Elenchi di file
settings-node-exclude = Esclusioni
settings-node-https-server = Server HTTP / HTTPS
settings-node-etp-api = API ETP / FTP
settings-node-privacy = Privacy e aggiornamenti
settings-node-logs = Log e debug
settings-node-backup = Backup, esportazione, ripristino
settings-node-locale = Lingua
settings-node-about = Informazioni

# §8.2 General → UI.
settings-ui-theme = Tema
settings-ui-run-bg = Esegui in background
settings-ui-show-tray = Mostra icona nell'area di notifica / barra dei menu
settings-ui-single-click-tray = Clic singolo sull'area di notifica / barra dei menu
settings-ui-new-window-from-tray = Apri una nuova finestra dall'icona dell'area di notifica
settings-ui-new-window-on-launch = Apri una nuova finestra all'avvio di Freally
settings-ui-search-as-you-type = Cerca mentre digiti
settings-ui-select-on-mouse-click = Seleziona la ricerca al clic del mouse
settings-ui-focus-on-activate = Metti a fuoco la ricerca all'attivazione
settings-ui-full-row-select = Selezione dell'intera riga
settings-ui-single-click-open = Apri con clic singolo
settings-ui-underline-titles = Sottolinea i titoli delle icone
settings-ui-row-density = Densità dei risultati
settings-ui-row-density-compact = Compatta (32 px)
settings-ui-row-density-comfortable = Comoda (44 px)
settings-ui-show-timing-badges = Mostra i badge dei tempi per ogni lente
settings-ui-anim-crossfade = Dissolvenza animata al cambio di tema

# §8.3 General → Home.
settings-home-match-case = Maiuscole/minuscole
settings-home-match-whole-word = Parola intera
settings-home-match-path = Confronta il percorso
settings-home-match-diacritics = Considera i segni diacritici
settings-home-match-regex = Espressione regolare
settings-home-search = Ricerca (query predefinita personalizzata)
settings-home-filter = Filtro
settings-home-sort = Ordinamento
settings-home-view = Visualizzazione
settings-home-index = Indice
settings-home-default-lens-visibility = Visibilità predefinita delle lenti
settings-home-default-lens-result-limits = Limiti predefiniti dei risultati per lente

# §8.4 General → Search.
settings-search-fast-ascii = Ricerca ASCII rapida
settings-search-mp-sep = Confronta il percorso quando un termine contiene un separatore di percorso
settings-search-mw-fn = Confronta l'intero nome file quando si usano caratteri jolly
settings-search-lit-ops = Consenti operatori letterali
settings-search-paren = Consenti il raggruppamento con parentesi tonde
settings-search-env = Espandi le variabili d'ambiente
settings-search-fwd-slash = Sostituisci le barre con barre rovesciate
settings-search-precedence = Precedenza degli operatori
settings-search-strict-everything = Modalità sintassi Everything rigorosa
settings-search-auto-regex = Rileva automaticamente le espressioni regolari
settings-search-mod-comp = Completamento dei modificatori
settings-search-parse-tree = Mostra l'albero di parsing al passaggio del mouse

# §8.5 General → Results.
settings-results-hide-empty = Nascondi i risultati quando la ricerca è vuota
settings-results-clear-on-search = Azzera la selezione a ogni ricerca
settings-results-close-on-execute = Chiudi la finestra all'esecuzione
settings-results-dbl-path = Apri il percorso con doppio clic nella colonna del percorso
settings-results-auto-scroll = Scorri automaticamente la vista
settings-results-dquote-copy = Copia tra virgolette doppie come percorso
settings-results-no-ext-rename = Non selezionare l'estensione durante la rinomina
settings-results-sort-date-desc = Ordina prima per data decrescente
settings-results-sort-size-desc = Ordina prima per dimensione decrescente
settings-results-list-focus = Focus sull'elenco dei risultati
settings-results-icon-prio = Priorità di caricamento delle icone
settings-results-thumb-prio = Priorità di caricamento delle miniature
settings-results-ext-prio = Priorità di caricamento delle informazioni estese
settings-results-group-by-lens = Raggruppa i risultati per lente
settings-results-snippet-inline = Mostra l'anteprima dello snippet in linea

# §8.6 General → View.
settings-view-double-buffer = Doppio buffer
settings-view-alt-rows = Colore alternato delle righe
settings-view-row-mouseover = Mostra l'evidenziazione al passaggio del mouse
settings-view-highlight-terms = Evidenzia i termini di ricerca
settings-view-status-show-selected = Mostra l'elemento selezionato nella barra di stato
settings-view-rc-with-sel = Mostra il conteggio dei risultati insieme a quello della selezione
settings-view-status-show-size = Mostra la dimensione nella barra di stato
settings-view-tooltips = Mostra i suggerimenti
settings-view-update-on-scroll = Aggiorna la visualizzazione subito dopo lo scorrimento
settings-view-size-format = Formato della dimensione
settings-view-selection-rect = Rettangolo di selezione
settings-view-audio-badges = Mostra i badge LUFS / codec / durata sulle righe audio
settings-view-similarity-score = Mostra il punteggio di somiglianza MinHash sulle righe di somiglianza
settings-view-preview-pane = Riquadro di anteprima

# §8.7 General → Context Menu.
settings-context-menu-visibility = Visibilità
settings-context-menu-show = Mostra
settings-context-menu-shift = Mostra solo tenendo premuto Maiusc
settings-context-menu-hide = Nascondi
settings-context-menu-command = Macro di comando
settings-context-menu-open-folders = Apri (cartelle)
settings-context-menu-open-files = Apri (file)
settings-context-menu-open-path = Apri percorso
settings-context-menu-explore = Esplora
settings-context-menu-explore-path = Esplora percorso
settings-context-menu-copy-name = Copia il nome negli appunti
settings-context-menu-copy-path = Copia il percorso negli appunti
settings-context-menu-copy-full-name = Copia il nome completo negli appunti
settings-context-menu-reveal = Mostra in Freally
settings-context-menu-send-to = Invia a Freally (percorso)

# §8.8 General → Fonts & Colors.
settings-fc-font = Carattere
settings-fc-size = Dimensione
settings-fc-state-normal = Normale
settings-fc-state-highlighted = Evidenziato
settings-fc-state-current-sort = Ordinamento corrente
settings-fc-state-current-sort-h = Ordinamento corrente (evidenziato)
settings-fc-state-selected = Selezionato
settings-fc-state-selected-h = Selezionato (evidenziato)
settings-fc-state-inactive-selected = Selezionato inattivo
settings-fc-state-inactive-selected-h = Selezionato inattivo (evidenziato)
settings-fc-foreground = Primo piano
settings-fc-background = Sfondo
settings-fc-bold = Grassetto
settings-fc-italic = Corsivo
settings-fc-default = Predefinito
settings-fc-per-lens-accent = Colore d'accento per lente
settings-fc-theme-inherit = Inverti automaticamente i colori personalizzati al cambio di tema

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Scorciatoia globale
settings-keyboard-new-window = Scorciatoia nuova finestra
settings-keyboard-show-window = Scorciatoia mostra finestra
settings-keyboard-toggle-window = Scorciatoia mostra/nascondi finestra
settings-keyboard-show-commands = Mostra i comandi che contengono
settings-keyboard-add-chord = + Aggiungi combinazione
settings-keyboard-remove-chord = Rimuovi

# §8.10 History.
settings-history-search-enable = Abilita la cronologia delle ricerche
settings-history-search-keep = Conserva la cronologia delle ricerche per { $days } giorni
settings-history-run-enable = Abilita la cronologia delle esecuzioni
settings-history-run-keep = Conserva la cronologia delle esecuzioni per { $days } giorni
settings-history-clear-now = Cancella ora
settings-history-privacy-mode = Modalità privacy
settings-history-per-lens = Cronologia per lente

# §8.11 Indexes (top-level).
settings-ix-database-location = Posizione del database
settings-ix-multiuser = Nome file del database multiutente
settings-ix-compress = Comprimi il database
settings-ix-recent-changes = Indicizza le modifiche recenti
settings-ix-file-size = Indicizza la dimensione dei file
settings-ix-fast-size-sort = Ordinamento rapido per dimensione
settings-ix-folder-size = Indicizza la dimensione delle cartelle
settings-ix-fast-folder-size-sort = Ordinamento rapido per dimensione delle cartelle
settings-ix-date-created = Indicizza la data di creazione
settings-ix-fast-date-created = Ordinamento rapido per data di creazione
settings-ix-date-modified = Indicizza la data di modifica
settings-ix-fast-date-modified = Ordinamento rapido per data di modifica
settings-ix-date-accessed = Indicizza la data di accesso
settings-ix-fast-date-accessed = Ordinamento rapido per data di accesso
settings-ix-attributes = Indicizza gli attributi
settings-ix-fast-attributes = Ordinamento rapido per attributi
settings-ix-fast-path-sort = Ordinamento rapido per percorso
settings-ix-fast-extension-sort = Ordinamento rapido per estensione
settings-ix-force-rebuild = Forza la ricostruzione
settings-ix-compact = Compatta l'indice
settings-ix-verify = Verifica l'indice
settings-ix-integrity-policy = Criteri di integrità dell'indice
settings-ix-memory-budget = Limite di memoria per l'indicizzatore
settings-ix-throttle = Limitazione dell'indicizzazione in background

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Includi automaticamente i nuovi volumi fissi
settings-vol-auto-removable = Includi automaticamente i nuovi volumi rimovibili
settings-vol-auto-remove-offline = Rimuovi automaticamente i volumi offline
settings-vol-detected = Volumi rilevati
settings-vol-include = Includi nell'indice
settings-vol-include-only = Includi solo (glob/regex)
settings-vol-enable-usn = Abilita il journal USN
settings-vol-enable-fsevents = Abilita il flusso FSEvents
settings-vol-enable-inotify = Abilita inotify (o fanotify con privilegi elevati)
settings-vol-buffer = Dimensione del buffer del journal (KB)
settings-vol-allocation-delta = Delta di allocazione (KB)
settings-vol-load-recent = Carica le modifiche recenti dal journal all'avvio
settings-vol-monitor = Monitora le modifiche
settings-vol-recreate-journal = Ricrea il journal
settings-vol-reset-stream = Reimposta il flusso FSEvents
settings-vol-upgrade-fanotify = Passa a fanotify (polkit)
settings-vol-remove = Rimuovi

# §8.13 Indexes → Folders.
settings-folders-watched = Cartelle monitorate
settings-folders-add = Aggiungi…
settings-folders-rescan-now = Riesegui la scansione ora
settings-folders-rescan-all = Riesegui la scansione di tutto ora
settings-folders-monitor = Tenta di monitorare le modifiche
settings-folders-buffer = Dimensione del buffer
settings-folders-rescan-on-full = Riesegui la scansione quando il buffer è pieno

# §8.14 Indexes → File Lists.
settings-flists-add = Aggiungi…
settings-flists-monitor = Monitora le modifiche
settings-flists-editor = Editor degli elenchi di file…
settings-flists-format = Formato dell'elenco di file
settings-flists-format-text = Testo (un percorso per riga)
settings-flists-format-json = JSON (con metadati)
settings-flists-format-srcb = Bundle Freally (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Escludi file e cartelle nascosti
settings-exclude-system = Escludi file e cartelle di sistema
settings-exclude-list-en = Abilita l'elenco delle esclusioni
settings-exclude-folders = Escludi cartelle
settings-exclude-include-only-files = Includi solo i file (glob)
settings-exclude-files = Escludi file (glob)
settings-exclude-os-recommended = Applica le esclusioni consigliate dal sistema operativo
settings-exclude-by-class = Escludi per classe di estensione

# §8.16 Lenses → Filename.
settings-lf-trigram = Aggressività del pre-filtro a trigrammi
settings-lf-suffix-mem = Limite di memoria dell'array di suffissi
settings-lf-wildcard-limit = Limite di espansione dei caratteri jolly
settings-lf-regex-timeout = Timeout delle espressioni regolari

# §8.17 Lenses → Content.
settings-lc-enable = Abilita la lente del contenuto
settings-lc-time-budget = Tempo massimo per documento
settings-lc-mem-ceiling = Limite di memoria per documento
settings-lc-snippet-len = Lunghezza dello snippet
settings-lc-stop-words = Lingua delle stop-word
settings-lc-re-extract = Riestrai al cambio delle impostazioni
settings-lc-verify-blobs = Verifica i checksum dei blob di testo estratto in lettura

# §8.18 Lenses → Audio.
settings-la-enable = Abilita la lente audio
settings-la-lufs-ref = Standard di riferimento LUFS
settings-la-peak-compute = Calcola il picco tramite
settings-la-silence-thresh = Soglia del silenzio
settings-la-re-extract-modify = Riestrai all'evento di modifica

# §8.19 Lenses → Similarity.
settings-ls-enable = Abilita la lente di somiglianza
settings-ls-sig-size = Dimensione della firma MinHash (k)
settings-ls-bands = Bande LSH
settings-ls-recall = Soglia di richiamo
settings-ls-result-cap = Limite dei risultati

# §8.20 Lenses → Custom.
settings-custom-registry = Registro
settings-custom-trust = Attendibilità
settings-custom-refresh-hashes = Aggiorna gli hash

# §8.21-§8.22 Network.
settings-net-https-enable = Abilita il server HTTPS
settings-net-bind = Associa alle interfacce
settings-net-port = Ascolta sulla porta
settings-net-force-https = Forza HTTPS
settings-net-legacy-auth = Autenticazione HTTP-basic legacy
settings-net-token-regen = Rigenera il token
settings-net-api-enable = Abilita il server API
settings-net-legacy-ftp = Supporto FTP/ETP in chiaro legacy

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Aggiornamento automatico
settings-privacy-prerelease = Canale pre-release
settings-privacy-network-policy = Criteri per le chiamate di rete

# §8.24 Logs & Debug.
settings-logs-level = Livello di log
settings-logs-location = Posizione del file di log
settings-logs-retention = Conservazione dei log
settings-logs-debug-overlay = Mostra l'overlay di debug
settings-logs-open-folder = Apri la cartella dei log
settings-logs-export-bundle = Esporta il pacchetto di diagnostica

# §8.25 Backup, Export, Reset.
settings-backup-export = Esporta le impostazioni
settings-backup-import = Importa le impostazioni
settings-backup-export-bookmarks = Esporta il pacchetto dei segnalibri
settings-backup-import-bookmarks = Importa il pacchetto dei segnalibri
settings-backup-reset-all = Ripristina tutte le impostazioni ai valori predefiniti

# §8.26 Locale.
settings-locale-current = Lingua corrente
settings-locale-rtl-preview = Anteprima RTL
settings-locale-date-format = Formato della data
settings-locale-number-format = Formato dei numeri

# §8.27 About.
settings-about-version = Freally { $version }
settings-about-license = Licenza
settings-about-credits = Riconoscimenti
settings-about-notices = Note open source

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = Procedura guidata al primo avvio
wizard-step-of-total = Passaggio { $step } di { $total }
wizard-roots-hint = Aggiungi le cartelle o i volumi che vuoi far monitorare a Freally. Potrai modificarli in seguito dalle impostazioni Indici.
wizard-browse = Sfoglia…
wizard-roots-placeholder = …oppure incolla un percorso
wizard-roots-add = Aggiungi
wizard-roots-remove = Rimuovi
wizard-roots-empty = Nessuna radice ancora configurata.
wizard-locale-hint = Freally è disponibile in 18 lingue. Potrai cambiarla in seguito.
wizard-theme-hint = «Sistema» segue le impostazioni di aspetto del sistema operativo.
wizard-back = Indietro
wizard-next = Avanti

# Status bar polish.
statusbar-hotkey-hint = Scorciatoia: { $hotkey }
statusbar-cycle-theme = Cambia tema
statusbar-indexed-suffix = indicizzati

# Results / lenses.
lens-expand = Espandi la lente
lens-collapse = Comprimi la lente
lens-no-matches = Nessuna corrispondenza in questa lente.

# Preview pane.
preview-header = Anteprima
preview-loading = Caricamento…
preview-select-file = Seleziona un file per visualizzarne l'anteprima.
preview-unavailable = Nessuna anteprima disponibile

# Bookmarks.
bookmarks-label = ★ Segnalibri
bookmarks-empty-hint = Ancora nessun segnalibro. Premi Ctrl+D per salvare la query corrente.
bookmarks-organize-title = Organizza i segnalibri
bookmarks-organize-empty = Ancora nessun segnalibro.
bookmarks-rename = Rinomina
bookmarks-close = Chiudi

# Settings tree extras.
settings-group-history = Cronologia
settings-group-privacy = Privacy e aggiornamenti
settings-group-logs = Log e debug
settings-group-backup = Backup, esportazione, ripristino
settings-tree-custom-lens = Personalizzata
settings-unsaved-changes = modifiche non salvate

# About dialog.
about-dialog-title = Freally
about-copyright = Copyright © 2026 Mike Weaver. Tutti i diritti riservati.
about-close = Chiudi

# Connect endpoint dialog.
connect-ftp-title = Connetti al server FTP
connect-ftp-host = Host:
connect-ftp-port = Porta:
connect-ftp-username = Nome utente:
connect-ftp-password = Password:
connect-ftp-link-type = Tipo di connessione:

# UI panel.
ui-hint = Tema, integrazione con area di notifica / barra dei menu, ricerca mentre digiti, densità delle righe. Parità diretta con voidtools-Everything più le aggiunte di Freally contrassegnate con (+).
ui-section-theme = Tema
ui-theme-system-default = Sistema (predefinito)
ui-section-tray = Area di notifica / barra dei menu
ui-section-search-behavior = Comportamento della ricerca
ui-section-result-rows = Righe dei risultati
ui-single-click-system-default = Impostazioni di sistema (predefinito)
ui-single-click-always = Sempre clic singolo
ui-single-click-always-double = Sempre doppio clic
ui-underline-always = Sempre
ui-underline-on-hover = Al passaggio del mouse
ui-underline-never = Mai

# Home panel.
home-hint = Valori predefiniti caricati all'avvio dell'app: ogni menu a discesa può mantenere «Usa ultimo valore» o fissare un valore. La visibilità delle lenti e i limiti dei risultati sono aggiunte di Freally (+).
home-section-match = Impostazioni di confronto predefinite
home-section-search-sort = Impostazioni di ricerca e ordinamento predefinite
home-search-placeholder = Vuoto per impostazione predefinita
home-section-index = Sorgente dell'indice
home-file-list-path = Percorso dell'elenco di file
home-https-endpoint = URL dell'endpoint API HTTPS
home-endpoint-token = Token (impronta mostrata)

# Backup panel.
backup-section-settings = Impostazioni (+)
backup-section-bookmarks = Segnalibri + estrattori personalizzati (+)
backup-section-reset = Ripristino
backup-toast-exported = Impostazioni esportate in { $path }
backup-toast-export-failed = Esportazione non riuscita: { $error }
backup-toast-imported = Impostazioni importate
backup-toast-import-failed = Importazione non riuscita: { $error }
backup-toast-bookmarks-exported = Segnalibri esportati
backup-toast-bookmarks-export-failed = Esportazione dei segnalibri non riuscita: { $error }
backup-toast-bookmarks-imported = Segnalibri importati
backup-toast-bookmarks-import-failed = Importazione dei segnalibri non riuscita: { $error }
backup-confirm-reset = Ripristinare tutte le impostazioni ai valori predefiniti? L'operazione non può essere annullata (la finestra di dialogo resta aperta).
backup-toast-reset = Tutte le impostazioni ripristinate

# Keyboard panel.
keyboard-section-global = Scorciatoie globali
keyboard-placeholder-example = Super+Space
keyboard-section-commands = Comandi
keyboard-placeholder-command = ID comando (es. file.export_results)
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = Cronologia delle ricerche
history-section-run = Cronologia delle esecuzioni
history-section-privacy = Privacy (+)
history-record-filename = Registra la cronologia della lente nome file
history-record-content = Registra la cronologia della lente contenuto
history-record-audio = Registra la cronologia della lente audio
history-record-similarity = Registra la cronologia della lente somiglianza

# Locale panel.
locale-section-language = Lingua (+)
locale-section-time-date = Ora / data (+)
locale-date-os = Predefinito del sistema operativo
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = Personalizzato
locale-date-custom-format = Formato personalizzato
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = Numeri (+)
locale-number-os = Predefinito del sistema operativo
locale-number-custom = Personalizzato
locale-thousands-sep = Separatore delle migliaia
locale-decimal-sep = Separatore decimale

# Folders panel.
folders-hint = Cartelle monitorate aggiuntive oltre ai volumi predefiniti.
folders-list-title = Cartelle monitorate
folders-empty = Nessuna cartella ancora aggiunta.
folders-remove = Rimuovi
folders-section-title-dynamic = Impostazioni per { $path }
folders-section-schedule = Pianificazione della scansione
folders-schedule-daily = Ogni giorno alle HH:MM
folders-schedule-hours = Ogni N ore
folders-schedule-never = Mai
folders-hour = Ora
folders-minute = Minuto
folders-hours = Ore
folders-id-label = ID cartella (sola lettura)
folders-select-prompt = Seleziona una cartella per configurarla.
folders-section-extras = Extra di Freally (+)
folders-extras-note = In questa build la nuova scansione alla ripresa dalla sospensione è abilitata per impostazione predefinita; l'interruttore si unirà ai controlli a livello di cartella nella rifinitura della Fase 13.

# Volumes panel.
volumes-hint = Equivalente multipiattaforma dei pannelli NTFS / ReFS di voidtools-Everything. Rileva automaticamente NTFS / ReFS / exFAT / FAT32 (Win), APFS / HFS+ (macOS), ext4 / Btrfs / ZFS / XFS / F2FS (Linux).
volumes-section-auto-include = Inclusione automatica
volumes-list-title = Volumi rilevati
volumes-detecting = Rilevamento in corso…
volumes-empty = Nessun volume rilevato.
volumes-select-prompt = Seleziona un volume per configurarlo.

# About panel polish.
about-section-version = Versione (+)
about-section-license = Licenza (+)
about-license-text = Mike Weaver — Tutti i diritti riservati. Questo è software proprietario.
about-license-spdx = SPDX: { $spdx }
about-section-credits = Riconoscimenti (+)
about-credits-inspired = Ispirato a Everything di voidtools.
about-credits-voidtools = voidtools.com
about-credits-repo = Repository del progetto

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = Contiene i comandi per lavorare con Freally.
menu-file-new-window = Nuova finestra di ricerca
menu-file-open-list = Apri elenco di file…
menu-file-close-list = Chiudi elenco di file
menu-file-close = Chiudi
menu-file-export-results = Esporta risultati…
menu-file-export-bundle = Esporta pacchetto dell'indice…
menu-file-exit = Esci

# Edit menu.
menu-edit-hint = Contiene i comandi per modificare i risultati della ricerca.
menu-edit-cut = Taglia
menu-edit-copy = Copia
menu-edit-paste = Incolla
menu-edit-copy-to-folder = Copia nella cartella…
menu-edit-move-to-folder = Sposta nella cartella…
menu-edit-select-all = Seleziona tutto
menu-edit-invert-selection = Inverti la selezione
menu-edit-advanced = Avanzate
menu-edit-copy-full-name = Copia il nome completo
menu-edit-copy-path = Copia il percorso
menu-edit-copy-filename = Copia il nome file
menu-edit-copy-as-json = Copia come JSON
menu-edit-copy-with-metadata = Copia con i metadati
menu-edit-copy-as-bundle-ref = Copia come riferimento a un bundle Freally

# View menu.
menu-view-hint = Contiene i comandi per gestire la visualizzazione.
menu-view-filters = Filtri
menu-view-preview = Anteprima
menu-view-status-bar = Barra di stato
menu-view-thumbs-xl = Miniature extra grandi
menu-view-thumbs-l = Miniature grandi
menu-view-thumbs-m = Miniature medie
menu-view-details = Dettagli
menu-view-window-size = Dimensione della finestra
menu-view-window-size-hint = Contiene i comandi per regolare la dimensione della finestra.
menu-view-window-small = Piccola
menu-view-window-medium = Media
menu-view-window-large = Grande
menu-view-window-auto = Adattamento automatico
menu-view-zoom = Zoom
menu-view-zoom-hint = Contiene i comandi per regolare la dimensione di caratteri e icone.
menu-view-zoom-in = Aumenta lo zoom
menu-view-zoom-out = Riduci lo zoom
menu-view-zoom-reset = Reimposta
menu-view-sort-by = Ordina per
menu-view-sort-by-hint = Contiene i comandi per ordinare l'elenco dei risultati.
menu-view-sort-name = Nome
menu-view-sort-path = Percorso
menu-view-sort-size = Dimensione
menu-view-sort-ext = Estensione
menu-view-sort-type = Tipo
menu-view-sort-modified = Data di modifica
menu-view-sort-created = Data di creazione
menu-view-sort-accessed = Data di accesso
menu-view-sort-attributes = Attributi
menu-view-sort-recently-changed = Data di modifica recente
menu-view-sort-run-count = Numero di esecuzioni
menu-view-sort-run-date = Data di esecuzione
menu-view-sort-file-list-filename = Nome file dell'elenco
menu-view-sort-lufs = LUFS
menu-view-sort-length = Durata
menu-view-sort-similarity = Punteggio di somiglianza
menu-view-sort-asc = Crescente
menu-view-sort-desc = Decrescente
menu-view-go-to = Vai a
menu-view-refresh = Aggiorna
menu-view-theme = Tema
menu-view-theme-hint = Passa tra i temi sistema, chiaro o scuro.
menu-view-lenses = Lenti
menu-view-lenses-hint = Attiva o disattiva la visibilità di ogni lente nell'elenco dei risultati.
menu-view-on-top = In primo piano
menu-view-on-top-hint = Contiene i comandi per mantenere questa finestra sopra le altre.
menu-view-on-top-never = Mai
menu-view-on-top-always = Sempre
menu-view-on-top-while-searching = Durante la ricerca

# Search menu.
menu-search-hint = Contiene le opzioni di ricerca.
menu-search-match-case = Maiuscole/minuscole
menu-search-match-whole-word = Parola intera
menu-search-match-path = Confronta il percorso
menu-search-match-diacritics = Considera i segni diacritici
menu-search-enable-regex = Abilita le espressioni regolari
menu-search-advanced = Ricerca avanzata…
menu-search-add-to-filters = Aggiungi ai filtri…
menu-search-organize-filters = Organizza i filtri…
menu-search-filter-everything = Tutto
menu-search-filter-archive = Compresso (archivio)
menu-search-filter-folder = Cartella
menu-search-filter-custom = Filtro personalizzato…

# Bookmarks menu.
menu-bookmarks-hint = Contiene i comandi per lavorare con i segnalibri.
menu-bookmarks-add = Aggiungi ai segnalibri
menu-bookmarks-organize = Organizza i segnalibri…

# Tools menu.
menu-tools-hint = Contiene i comandi degli strumenti.
menu-tools-connect = Connetti al server FTP…
menu-tools-disconnect = Disconnetti dal server FTP
menu-tools-file-list-editor = Editor degli elenchi di file…
menu-tools-index-maintenance = Manutenzione dell'indice
menu-tools-index-maintenance-hint = Strumenti di manutenzione dell'indice.
menu-tools-verify-index = Verifica indice…
menu-tools-compact-index = Compatta indice…
menu-tools-rebuild-index = Forza la ricostruzione dell'indice…
menu-tools-custom-extractor = Gestione estrattori personalizzati…
menu-tools-custom-extractor-hint = Gestisci gli estrattori personalizzati in sandbox Wasm.
menu-tools-options = Opzioni…

# Help menu.
menu-help-hint = Contiene i comandi di aiuto.
menu-help-help = Guida di Freally
menu-help-search-syntax = Sintassi di ricerca
menu-help-regex-syntax = Sintassi delle espressioni regolari
menu-help-audio-ref = Riferimento dei modificatori audio
menu-help-similarity-ref = Riferimento dei modificatori di somiglianza
menu-help-cli-options = Opzioni della riga di comando
menu-help-website = Sito web di Freally
menu-help-check-updates = Verifica aggiornamenti…
menu-help-sponsor = Sostieni / Dona
menu-help-about = Informazioni su Freally…

# Result column headers (short forms used in the table header row).
column-name = Nome
column-path = Percorso
column-size = Dimensione
column-modified = Modificato
column-type = Tipo
column-ext = Est.
column-sort-by = Ordina per { $name }
column-resize = Ridimensiona la colonna { $name }

# Section subtitle bars used inside multiple settings panels.
section-behavior = Comportamento
section-rendering = Rendering
section-status-bar = Barra di stato
section-display-format = Formato di visualizzazione
section-loading-priority = Priorità di caricamento
section-compatibility = Compatibilità
section-storage = Archiviazione
section-index-fields = Campi dell'indice
section-maintenance = Manutenzione
section-logging = Registrazione log
section-tools = Strumenti
section-privacy = Privacy
section-auto-update = Aggiornamento automatico (+)
section-bind = Associazione
section-lens = Lente
section-budgets = Limiti
section-other = Altro
section-per-format-mode = Modalità per formato
section-loudness = Volume sonoro
section-tuning = Ottimizzazione (+)
section-minhash-lsh = Parametri MinHash + LSH (+)
section-top-level = Primo livello
section-file-globs = Glob dei file
section-file-list-settings = Impostazioni per l'elenco di file selezionato
section-editor-format = Editor + formato (E + +)
section-api-server = Server API (E adattato)
section-freally-extras = Extra di Freally (+)
section-freally-additions = Aggiunte di Freally (+)
section-freally-extensions = Estensioni di Freally (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = Usa ultimo valore
opt-use-last-value-default = Usa ultimo valore (predefinito)
opt-low = Bassa
opt-normal-default = Normale (predefinito)
opt-high = Alta
opt-disabled = Disabilitato
opt-off = Disattivato
opt-on-battery = Quando si è a batteria
opt-always = Sempre
opt-clamp-default = Limita (predefinito)
opt-wrap = Avvolgi
opt-none = Nessuno
opt-strict-refuse = Rigoroso (rifiuta le query in caso di corruzione)
opt-lenient-warn = Tollerante (avvisa ma esegue la query)
opt-system-default = Predefinito del sistema
opt-drag-select = Selezione con trascinamento
opt-auto-binary = Auto (binario)
opt-auto-decimal = Auto (decimale)

# Unit suffixes shown next to number inputs.
unit-days = giorni
unit-b = B
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-tb = TB

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = Immediato
opt-lazy-default = Differito (predefinito)
opt-on = Attivato
opt-on-default = Attivato (predefinito)
opt-all = Tutti
opt-weekly = Settimanale
opt-monthly = Mensile
opt-name-asc = Nome cresc.
opt-name-desc = Nome decr.
opt-size-asc = Dimensione cresc.
opt-size-desc = Dimensione decr.
opt-modified-asc = Data di modifica cresc.
opt-modified-desc = Data di modifica decr.
opt-compact = Compatta
opt-comfortable = Comoda
opt-details = Dettagli
opt-thumbnails = Miniature
opt-local-db-default = Database locale (predefinito)
opt-file-list = Elenco di file
opt-https-endpoint = Endpoint API HTTPS
opt-right-default = Destra (predefinito)
opt-bottom = In basso
opt-or-and-default = OR > AND (predefinito)
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128 (predefinito)
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = Broadcast film (-23)
opt-true-peak = True peak (sovracampionamento 4×, predefinito)
opt-sample-peak = Sample peak
opt-auto-per-doc = Auto (per documento)
opt-log-error = Error
opt-log-warn = Warn
opt-log-info-default = Info (predefinito)
opt-log-debug = Debug
opt-log-trace = Trace
