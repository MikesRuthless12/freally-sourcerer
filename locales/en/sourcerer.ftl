# Sourcerer — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Sourcerer
tagline = One search. Every source. Every OS.
window-title = Sourcerer
search-placeholder = Search…
about-version = Version { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = Ready
status-indexed = Indexed ({ $count } files)
status-indexing = Indexing… { $done }/{ $total }
status-paused = Paused
status-error = Error
status-result-count-one = { $count } result
status-result-count-many = { $count } results
status-selection = · { $count } selected
status-selection-size = Selected: { $size }
status-query-timing = Query: { $ms } ms
status-endpoint-local = Local DB
status-endpoint-remote = API: { $name }

menu-file = File
menu-edit = Edit
menu-view = View
menu-search = Search
menu-bookmarks = Bookmarks
menu-tools = Tools
menu-help = Help

theme-system = System
theme-light = Light
theme-dark = Dark

lens-filename = Filename
lens-content = Content
lens-audio = Audio
lens-similarity = Similarity

parse-error-empty = Type a query to begin.
parse-error-unknown = Unrecognized syntax near here.

action-open = Open
action-reveal = Reveal in folder
action-copy-path = Copy path
action-copy-name = Copy name
action-delete = Delete

quick-filter-audio = Audio
quick-filter-video = Video
quick-filter-image = Image
quick-filter-document = Document
quick-filter-executable = Executable
quick-filter-archive = Archive

wizard-title = Welcome to Sourcerer
wizard-step-roots = Choose what to index
wizard-step-hotkey = Pick a global hotkey
wizard-step-locale = Pick your language
wizard-step-theme = Pick a theme
wizard-finish = Finish

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = Options
settings-search-placeholder = Search options…
settings-restore-defaults = Restore Defaults
settings-ok = OK
settings-cancel = Cancel
settings-apply = Apply

# Tree nav groups (PRD §8.1.1).
settings-group-general = General
settings-group-indexes = Indexes
settings-group-lenses = Lenses
settings-group-network = Network

# Tree nav leaves.
settings-node-ui = UI
settings-node-home = Home
settings-node-search = Search
settings-node-results = Results
settings-node-view = View
settings-node-context-menu = Context Menu
settings-node-fonts-colors = Fonts & Colors
settings-node-keyboard = Keyboard
settings-node-history = History
settings-node-indexes-top = (top-level)
settings-node-volumes = Volumes
settings-node-folders = Folders
settings-node-file-lists = File Lists
settings-node-exclude = Exclude
settings-node-https-server = HTTP / HTTPS Server
settings-node-etp-api = ETP / FTP API
settings-node-privacy = Privacy & Updates
settings-node-logs = Logs & Debug
settings-node-backup = Backup, Export, Reset
settings-node-locale = Locale
settings-node-about = About

# §8.2 General → UI.
settings-ui-theme = Theme
settings-ui-run-bg = Run in background
settings-ui-show-tray = Show tray / menu-bar icon
settings-ui-single-click-tray = Single click tray / menu bar
settings-ui-new-window-from-tray = Open new window from tray icon
settings-ui-new-window-on-launch = Open new window when launching Sourcerer
settings-ui-search-as-you-type = Search as you type
settings-ui-select-on-mouse-click = Select search on mouse click
settings-ui-focus-on-activate = Focus search on activate
settings-ui-full-row-select = Full row select
settings-ui-single-click-open = Single click open
settings-ui-underline-titles = Underline icon titles
settings-ui-row-density = Result density
settings-ui-row-density-compact = Compact (32 px)
settings-ui-row-density-comfortable = Comfortable (44 px)
settings-ui-show-timing-badges = Show timing badges per lens
settings-ui-anim-crossfade = Animated theme cross-fade

# §8.3 General → Home.
settings-home-match-case = Match case
settings-home-match-whole-word = Match whole word
settings-home-match-path = Match path
settings-home-match-diacritics = Match diacritics
settings-home-match-regex = Match regex
settings-home-search = Search (custom default query)
settings-home-filter = Filter
settings-home-sort = Sort
settings-home-view = View
settings-home-index = Index
settings-home-default-lens-visibility = Default lens visibility
settings-home-default-lens-result-limits = Default lens result limits

# §8.4 General → Search.
settings-search-fast-ascii = Fast ASCII search
settings-search-mp-sep = Match path when a search term contains a path separator
settings-search-mw-fn = Match whole filename when using wildcards
settings-search-lit-ops = Allow literal operators
settings-search-paren = Allow round bracket grouping
settings-search-env = Expand environment variables
settings-search-fwd-slash = Replace forward slashes with backslashes
settings-search-precedence = Operator precedence
settings-search-strict-everything = Strict Everything syntax mode
settings-search-auto-regex = Auto-detect regex
settings-search-mod-comp = Modifier completions
settings-search-parse-tree = Show parse-tree on hover

# §8.5 General → Results.
settings-results-hide-empty = Hide results when the search is empty
settings-results-clear-on-search = Clear selection on search
settings-results-close-on-execute = Close window on execute
settings-results-dbl-path = Open path with double click in path column
settings-results-auto-scroll = Automatically scroll view
settings-results-dquote-copy = Double quote copy as path
settings-results-no-ext-rename = Do not select extension when renaming
settings-results-sort-date-desc = Sort date descending first
settings-results-sort-size-desc = Sort size descending first
settings-results-list-focus = Result list focus
settings-results-icon-prio = Load icon priority
settings-results-thumb-prio = Load thumbnail priority
settings-results-ext-prio = Load extended information priority
settings-results-group-by-lens = Group results by lens
settings-results-snippet-inline = Show snippet preview inline

# §8.6 General → View.
settings-view-double-buffer = Double buffer
settings-view-alt-rows = Alternate row color
settings-view-row-mouseover = Show row mouseover
settings-view-highlight-terms = Show highlighted search terms
settings-view-status-show-selected = Show selected item in status bar
settings-view-rc-with-sel = Show the result count with the selection count
settings-view-status-show-size = Show size in status bar
settings-view-tooltips = Show tooltips
settings-view-update-on-scroll = Update display immediately after scrolling
settings-view-size-format = Size format
settings-view-selection-rect = Selection rectangle
settings-view-audio-badges = Show LUFS / codec / length badges on audio rows
settings-view-similarity-score = Show MinHash similarity score on similarity rows
settings-view-preview-pane = Preview pane

# §8.7 General → Context Menu.
settings-context-menu-visibility = Visibility
settings-context-menu-show = Show
settings-context-menu-shift = Show only when Shift held
settings-context-menu-hide = Hide
settings-context-menu-command = Command macro
settings-context-menu-open-folders = Open (Folders)
settings-context-menu-open-files = Open (Files)
settings-context-menu-open-path = Open Path
settings-context-menu-explore = Explore
settings-context-menu-explore-path = Explore Path
settings-context-menu-copy-name = Copy Name to Clipboard
settings-context-menu-copy-path = Copy Path to Clipboard
settings-context-menu-copy-full-name = Copy Full Name to Clipboard
settings-context-menu-reveal = Reveal in Sourcerer
settings-context-menu-send-to = Send to Sourcerer (path)

# §8.8 General → Fonts & Colors.
settings-fc-font = Font
settings-fc-size = Size
settings-fc-state-normal = Normal
settings-fc-state-highlighted = Highlighted
settings-fc-state-current-sort = Current Sort
settings-fc-state-current-sort-h = Current Sort (Highlighted)
settings-fc-state-selected = Selected
settings-fc-state-selected-h = Selected (Highlighted)
settings-fc-state-inactive-selected = Inactive Selected
settings-fc-state-inactive-selected-h = Inactive Selected (Highlighted)
settings-fc-foreground = Foreground
settings-fc-background = Background
settings-fc-bold = Bold
settings-fc-italic = Italic
settings-fc-default = Default
settings-fc-per-lens-accent = Per-Lens Accent
settings-fc-theme-inherit = Auto-flip custom colors on theme switch

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Global Hotkey
settings-keyboard-new-window = New window Hotkey
settings-keyboard-show-window = Show window Hotkey
settings-keyboard-toggle-window = Toggle window Hotkey
settings-keyboard-show-commands = Show commands containing
settings-keyboard-add-chord = + Add chord
settings-keyboard-remove-chord = Remove

# §8.10 History.
settings-history-search-enable = Enable search history
settings-history-search-keep = Keep search history for { $days } days
settings-history-run-enable = Enable run history
settings-history-run-keep = Keep run history for { $days } days
settings-history-clear-now = Clear Now
settings-history-privacy-mode = Privacy mode
settings-history-per-lens = Per-lens history

# §8.11 Indexes (top-level).
settings-ix-database-location = Database location
settings-ix-multiuser = Multi-user database filename
settings-ix-compress = Compress database
settings-ix-recent-changes = Index recent changes
settings-ix-file-size = Index file size
settings-ix-fast-size-sort = Fast size sort
settings-ix-folder-size = Index folder size
settings-ix-fast-folder-size-sort = Fast folder size sort
settings-ix-date-created = Index date created
settings-ix-fast-date-created = Fast date created sort
settings-ix-date-modified = Index date modified
settings-ix-fast-date-modified = Fast date modified sort
settings-ix-date-accessed = Index date accessed
settings-ix-fast-date-accessed = Fast date accessed sort
settings-ix-attributes = Index attributes
settings-ix-fast-attributes = Fast attributes sort
settings-ix-fast-path-sort = Fast path sort
settings-ix-fast-extension-sort = Fast extension sort
settings-ix-force-rebuild = Force Rebuild
settings-ix-compact = Compact Index
settings-ix-verify = Verify Index
settings-ix-integrity-policy = Index integrity policy
settings-ix-memory-budget = Memory budget for indexer
settings-ix-throttle = Background indexing throttle

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Automatically include new fixed volumes
settings-vol-auto-removable = Automatically include new removable volumes
settings-vol-auto-remove-offline = Automatically remove offline volumes
settings-vol-detected = Detected volumes
settings-vol-include = Include in index
settings-vol-include-only = Include only (glob/regex)
settings-vol-enable-usn = Enable USN Journal
settings-vol-enable-fsevents = Enable FSEvents stream
settings-vol-enable-inotify = Enable inotify (or fanotify if elevated)
settings-vol-buffer = Journal buffer size (KB)
settings-vol-allocation-delta = Allocation delta (KB)
settings-vol-load-recent = Load recent changes from journal on startup
settings-vol-monitor = Monitor changes
settings-vol-recreate-journal = Recreate journal
settings-vol-reset-stream = Reset FSEvents stream
settings-vol-upgrade-fanotify = Upgrade to fanotify (polkit)
settings-vol-remove = Remove

# §8.13 Indexes → Folders.
settings-folders-watched = Watched folders
settings-folders-add = Add…
settings-folders-rescan-now = Rescan Now
settings-folders-rescan-all = Rescan All Now
settings-folders-monitor = Attempt to monitor changes
settings-folders-buffer = Buffer size
settings-folders-rescan-on-full = Rescan on full buffer

# §8.14 Indexes → File Lists.
settings-flists-add = Add…
settings-flists-monitor = Monitor changes
settings-flists-editor = File List Editor…
settings-flists-format = File list format
settings-flists-format-text = Text (one path per line)
settings-flists-format-json = JSON (with metadata)
settings-flists-format-srcb = Sourcerer Bundle (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Exclude hidden files and folders
settings-exclude-system = Exclude system files and folders
settings-exclude-list-en = Enable exclude list
settings-exclude-folders = Exclude folders
settings-exclude-include-only-files = Include only files (glob)
settings-exclude-files = Exclude files (glob)
settings-exclude-os-recommended = Apply OS-recommended excludes
settings-exclude-by-class = Exclude by extension class

# §8.16 Lenses → Filename.
settings-lf-trigram = Trigram pre-filter aggressiveness
settings-lf-suffix-mem = Suffix-array memory budget
settings-lf-wildcard-limit = Wildcard expansion limit
settings-lf-regex-timeout = Regex timeout

# §8.17 Lenses → Content.
settings-lc-enable = Enable content lens
settings-lc-time-budget = Time budget per document
settings-lc-mem-ceiling = Memory ceiling per document
settings-lc-snippet-len = Snippet length
settings-lc-stop-words = Stop-words language
settings-lc-re-extract = Re-extract on settings change
settings-lc-verify-blobs = Verify extracted-text blob checksums on read

# §8.18 Lenses → Audio.
settings-la-enable = Enable audio lens
settings-la-lufs-ref = LUFS reference standard
settings-la-peak-compute = Compute peak via
settings-la-silence-thresh = Silence threshold
settings-la-re-extract-modify = Re-extract on Modify event

# §8.19 Lenses → Similarity.
settings-ls-enable = Enable similarity lens
settings-ls-sig-size = MinHash signature size (k)
settings-ls-bands = LSH bands
settings-ls-recall = Recall threshold
settings-ls-result-cap = Result cap

# §8.20 Lenses → Custom.
settings-custom-registry = Registry
settings-custom-trust = Trust
settings-custom-refresh-hashes = Refresh hashes

# §8.21-§8.22 Network.
settings-net-https-enable = Enable HTTPS server
settings-net-bind = Bind to interfaces
settings-net-port = Listen on port
settings-net-force-https = Force HTTPS
settings-net-legacy-auth = Legacy HTTP-basic auth
settings-net-token-regen = Token regenerate
settings-net-api-enable = Enable API server
settings-net-legacy-ftp = Legacy plain FTP/ETP support

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Auto-update
settings-privacy-prerelease = Pre-release channel
settings-privacy-network-policy = Network calls policy

# §8.24 Logs & Debug.
settings-logs-level = Log level
settings-logs-location = Log file location
settings-logs-retention = Log retention
settings-logs-debug-overlay = Show debug overlay
settings-logs-open-folder = Open log folder
settings-logs-export-bundle = Export diagnostics bundle

# §8.25 Backup, Export, Reset.
settings-backup-export = Export settings
settings-backup-import = Import settings
settings-backup-export-bookmarks = Export bookmarks bundle
settings-backup-import-bookmarks = Import bookmarks bundle
settings-backup-reset-all = Reset all settings to defaults

# §8.26 Locale.
settings-locale-current = Current locale
settings-locale-rtl-preview = RTL preview
settings-locale-date-format = Date format
settings-locale-number-format = Number format

# §8.27 About.
settings-about-version = Sourcerer { $version }
settings-about-license = License
settings-about-credits = Credits
settings-about-notices = Open-source notices
