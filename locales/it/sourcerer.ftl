# Sourcerer — Italiano.

app-name = Sourcerer
tagline = Una ricerca. Ogni sorgente. Ogni OS.
window-title = Sourcerer
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
parse-error-unknown = Sintassi non riconosciuta in questo punto.

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

wizard-title = Benvenuto in Sourcerer
wizard-step-roots = Scegli cosa indicizzare
wizard-step-hotkey = Scegli una scorciatoia globale
wizard-step-locale = Scegli la tua lingua
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
settings-node-indexes-top = (livello superiore)
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
settings-ui-show-tray = Mostra icona nella barra di sistema
settings-ui-single-click-tray = Singolo clic sull'icona della barra
settings-ui-new-window-from-tray = Apri nuova finestra dall'icona della barra
settings-ui-new-window-on-launch = Apri nuova finestra all'avvio di Sourcerer
settings-ui-search-as-you-type = Cerca durante la digitazione
settings-ui-select-on-mouse-click = Seleziona la ricerca al clic del mouse
settings-ui-focus-on-activate = Metti il fuoco sulla ricerca all'attivazione
settings-ui-full-row-select = Selezione dell'intera riga
settings-ui-single-click-open = Apertura con singolo clic
settings-ui-underline-titles = Sottolinea i titoli delle icone
settings-ui-row-density = Densità dei risultati
settings-ui-row-density-compact = Compatta (32 px)
settings-ui-row-density-comfortable = Comoda (44 px)
settings-ui-show-timing-badges = Mostra indicatori di tempo per lente
settings-ui-anim-crossfade = Dissolvenza animata al cambio tema

# §8.3 General → Home.
settings-home-match-case = Distingui maiuscole e minuscole
settings-home-match-whole-word = Parola intera
settings-home-match-path = Cerca nel percorso
settings-home-match-diacritics = Distingui i segni diacritici
settings-home-match-regex = Usa Regex
settings-home-search = Ricerca (query predefinita personalizzata)
settings-home-filter = Filtro
settings-home-sort = Ordinamento
settings-home-view = Visualizzazione
settings-home-index = Indice
settings-home-default-lens-visibility = Visibilità predefinita delle lenti
settings-home-default-lens-result-limits = Limiti predefiniti dei risultati per lente

# §8.4 General → Search.
settings-search-fast-ascii = Ricerca ASCII veloce
settings-search-mp-sep = Cerca nel percorso quando un termine contiene un separatore
settings-search-mw-fn = Confronta nome file intero quando si usano caratteri jolly
settings-search-lit-ops = Consenti operatori letterali
settings-search-paren = Consenti raggruppamento con parentesi tonde
settings-search-env = Espandi le variabili d'ambiente
settings-search-fwd-slash = Sostituisci le barre con barre rovesciate
settings-search-precedence = Precedenza degli operatori
settings-search-strict-everything = Modalità sintassi Everything stretta
settings-search-auto-regex = Rilevamento automatico Regex
settings-search-mod-comp = Completamento dei modificatori
settings-search-parse-tree = Mostra l'albero di parsing al passaggio del mouse

# §8.5 General → Results.
settings-results-hide-empty = Nascondi i risultati quando la ricerca è vuota
settings-results-clear-on-search = Cancella la selezione alla ricerca
settings-results-close-on-execute = Chiudi la finestra all'esecuzione
settings-results-dbl-path = Apri il percorso con doppio clic nella colonna del percorso
settings-results-auto-scroll = Scorri automaticamente la vista
settings-results-dquote-copy = Copia tra virgolette doppie come percorso
settings-results-no-ext-rename = Non selezionare l'estensione durante la rinomina
settings-results-sort-date-desc = Ordina prima per data decrescente
settings-results-sort-size-desc = Ordina prima per dimensione decrescente
settings-results-list-focus = Fuoco sull'elenco dei risultati
settings-results-icon-prio = Priorità di caricamento icone
settings-results-thumb-prio = Priorità di caricamento miniature
settings-results-ext-prio = Priorità di caricamento informazioni estese
settings-results-group-by-lens = Raggruppa risultati per lente
settings-results-snippet-inline = Mostra anteprima dello snippet in linea

# §8.6 General → View.
settings-view-double-buffer = Doppio buffer
settings-view-alt-rows = Colore alternato delle righe
settings-view-row-mouseover = Evidenzia la riga al passaggio del mouse
settings-view-highlight-terms = Evidenzia i termini di ricerca
settings-view-status-show-selected = Mostra l'elemento selezionato nella barra di stato
settings-view-rc-with-sel = Mostra il conteggio dei risultati con quello della selezione
settings-view-status-show-size = Mostra la dimensione nella barra di stato
settings-view-tooltips = Mostra suggerimenti
settings-view-update-on-scroll = Aggiorna la visualizzazione subito dopo lo scorrimento
settings-view-size-format = Formato dimensione
settings-view-selection-rect = Rettangolo di selezione
settings-view-audio-badges = Mostra indicatori LUFS / codec / durata sulle righe audio
settings-view-similarity-score = Mostra punteggio di somiglianza MinHash sulle righe di somiglianza
settings-view-preview-pane = Riquadro di anteprima

# §8.7 General → Context Menu.
settings-context-menu-visibility = Visibilità
settings-context-menu-show = Mostra
settings-context-menu-shift = Mostra solo con Shift premuto
settings-context-menu-hide = Nascondi
settings-context-menu-command = Macro di comando
settings-context-menu-open-folders = Apri (cartelle)
settings-context-menu-open-files = Apri (file)
settings-context-menu-open-path = Apri percorso
settings-context-menu-explore = Esplora
settings-context-menu-explore-path = Esplora percorso
settings-context-menu-copy-name = Copia nome negli appunti
settings-context-menu-copy-path = Copia percorso negli appunti
settings-context-menu-copy-full-name = Copia nome completo negli appunti
settings-context-menu-reveal = Mostra in Sourcerer
settings-context-menu-send-to = Invia a Sourcerer (percorso)

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
settings-fc-per-lens-accent = Accento per lente
settings-fc-theme-inherit = Adatta automaticamente i colori personalizzati al cambio tema

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Scorciatoia globale
settings-keyboard-new-window = Scorciatoia nuova finestra
settings-keyboard-show-window = Scorciatoia mostra finestra
settings-keyboard-toggle-window = Scorciatoia mostra/nascondi finestra
settings-keyboard-show-commands = Mostra comandi contenenti
settings-keyboard-add-chord = + Aggiungi combinazione
settings-keyboard-remove-chord = Rimuovi

# §8.10 History.
settings-history-search-enable = Abilita cronologia delle ricerche
settings-history-search-keep = Conserva la cronologia delle ricerche per { $days } giorni
settings-history-run-enable = Abilita cronologia delle esecuzioni
settings-history-run-keep = Conserva la cronologia delle esecuzioni per { $days } giorni
settings-history-clear-now = Cancella ora
settings-history-privacy-mode = Modalità privacy
settings-history-per-lens = Cronologia per lente

# §8.11 Indexes (top-level).
settings-ix-database-location = Posizione del database
settings-ix-multiuser = Nome del database multi-utente
settings-ix-compress = Comprimi database
settings-ix-recent-changes = Indicizza le modifiche recenti
settings-ix-file-size = Indicizza dimensione file
settings-ix-fast-size-sort = Ordinamento rapido per dimensione
settings-ix-folder-size = Indicizza dimensione cartelle
settings-ix-fast-folder-size-sort = Ordinamento rapido per dimensione cartelle
settings-ix-date-created = Indicizza data di creazione
settings-ix-fast-date-created = Ordinamento rapido per data di creazione
settings-ix-date-modified = Indicizza data di modifica
settings-ix-fast-date-modified = Ordinamento rapido per data di modifica
settings-ix-date-accessed = Indicizza data di accesso
settings-ix-fast-date-accessed = Ordinamento rapido per data di accesso
settings-ix-attributes = Indicizza attributi
settings-ix-fast-attributes = Ordinamento rapido per attributi
settings-ix-fast-path-sort = Ordinamento rapido per percorso
settings-ix-fast-extension-sort = Ordinamento rapido per estensione
settings-ix-force-rebuild = Forza ricostruzione
settings-ix-compact = Compatta indice
settings-ix-verify = Verifica indice
settings-ix-integrity-policy = Criterio di integrità dell'indice
settings-ix-memory-budget = Budget di memoria per l'indicizzatore
settings-ix-throttle = Limitazione dell'indicizzazione in background

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Includi automaticamente i nuovi volumi fissi
settings-vol-auto-removable = Includi automaticamente i nuovi volumi rimovibili
settings-vol-auto-remove-offline = Rimuovi automaticamente i volumi offline
settings-vol-detected = Volumi rilevati
settings-vol-include = Includi nell'indice
settings-vol-include-only = Includi solo (glob/regex)
settings-vol-enable-usn = Abilita USN Journal
settings-vol-enable-fsevents = Abilita stream FSEvents
settings-vol-enable-inotify = Abilita inotify (o fanotify se elevato)
settings-vol-buffer = Dimensione buffer del journal (KB)
settings-vol-allocation-delta = Delta di allocazione (KB)
settings-vol-load-recent = Carica le modifiche recenti dal journal all'avvio
settings-vol-monitor = Monitora le modifiche
settings-vol-recreate-journal = Ricrea journal
settings-vol-reset-stream = Reimposta stream FSEvents
settings-vol-upgrade-fanotify = Esegui l'upgrade a fanotify (polkit)
settings-vol-remove = Rimuovi

# §8.13 Indexes → Folders.
settings-folders-watched = Cartelle monitorate
settings-folders-add = Aggiungi…
settings-folders-rescan-now = Riscansiona ora
settings-folders-rescan-all = Riscansiona tutto ora
settings-folders-monitor = Tenta di monitorare le modifiche
settings-folders-buffer = Dimensione buffer
settings-folders-rescan-on-full = Riscansiona quando il buffer è pieno

# §8.14 Indexes → File Lists.
settings-flists-add = Aggiungi…
settings-flists-monitor = Monitora le modifiche
settings-flists-editor = Editor elenchi di file…
settings-flists-format = Formato elenco file
settings-flists-format-text = Testo (un percorso per riga)
settings-flists-format-json = JSON (con metadati)
settings-flists-format-srcb = Sourcerer Bundle (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Escludi file e cartelle nascosti
settings-exclude-system = Escludi file e cartelle di sistema
settings-exclude-list-en = Abilita elenco di esclusione
settings-exclude-folders = Escludi cartelle
settings-exclude-include-only-files = Includi solo file (glob)
settings-exclude-files = Escludi file (glob)
settings-exclude-os-recommended = Applica esclusioni consigliate dal sistema operativo
settings-exclude-by-class = Escludi per classe di estensione

# §8.16 Lenses → Filename.
settings-lf-trigram = Aggressività del prefiltro trigram
settings-lf-suffix-mem = Budget di memoria per suffix-array
settings-lf-wildcard-limit = Limite di espansione dei caratteri jolly
settings-lf-regex-timeout = Timeout Regex

# §8.17 Lenses → Content.
settings-lc-enable = Abilita lente contenuto
settings-lc-time-budget = Budget di tempo per documento
settings-lc-mem-ceiling = Limite di memoria per documento
settings-lc-snippet-len = Lunghezza dello snippet
settings-lc-stop-words = Lingua delle stop-word
settings-lc-re-extract = Riestrai al cambio delle impostazioni
settings-lc-verify-blobs = Verifica i checksum dei blob di testo estratto in lettura

# §8.18 Lenses → Audio.
settings-la-enable = Abilita lente audio
settings-la-lufs-ref = Standard di riferimento LUFS
settings-la-peak-compute = Calcola il picco tramite
settings-la-silence-thresh = Soglia di silenzio
settings-la-re-extract-modify = Riestrai all'evento di modifica

# §8.19 Lenses → Similarity.
settings-ls-enable = Abilita lente di somiglianza
settings-ls-sig-size = Dimensione della firma MinHash (k)
settings-ls-bands = Bande LSH
settings-ls-recall = Soglia di recall
settings-ls-result-cap = Limite massimo di risultati

# §8.20 Lenses → Custom.
settings-custom-registry = Registro
settings-custom-trust = Affidabilità
settings-custom-refresh-hashes = Aggiorna hash

# §8.21-§8.22 Network.
settings-net-https-enable = Abilita server HTTPS
settings-net-bind = Associa alle interfacce
settings-net-port = Ascolta sulla porta
settings-net-force-https = Forza HTTPS
settings-net-legacy-auth = Autenticazione HTTP-basic legacy
settings-net-token-regen = Rigenera token
settings-net-api-enable = Abilita server API
settings-net-legacy-ftp = Supporto FTP/ETP non cifrato legacy

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Aggiornamento automatico
settings-privacy-prerelease = Canale pre-release
settings-privacy-network-policy = Criterio per le chiamate di rete

# §8.24 Logs & Debug.
settings-logs-level = Livello di log
settings-logs-location = Posizione file di log
settings-logs-retention = Conservazione dei log
settings-logs-debug-overlay = Mostra overlay di debug
settings-logs-open-folder = Apri cartella dei log
settings-logs-export-bundle = Esporta bundle di diagnostica

# §8.25 Backup, Export, Reset.
settings-backup-export = Esporta impostazioni
settings-backup-import = Importa impostazioni
settings-backup-export-bookmarks = Esporta bundle dei segnalibri
settings-backup-import-bookmarks = Importa bundle dei segnalibri
settings-backup-reset-all = Ripristina tutte le impostazioni ai valori predefiniti

# §8.26 Locale.
settings-locale-current = Lingua corrente
settings-locale-rtl-preview = Anteprima RTL
settings-locale-date-format = Formato data
settings-locale-number-format = Formato numerico

# §8.27 About.
settings-about-version = Sourcerer { $version }
settings-about-license = Licenza
settings-about-credits = Riconoscimenti
settings-about-notices = Avvisi open-source
