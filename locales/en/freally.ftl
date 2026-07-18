# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = One search. Every source. Every OS.
window-title = Freally Sourcerer
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

wizard-title = Welcome to Freally
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
settings-ui-new-window-on-launch = Open new window when launching Freally
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
settings-context-menu-reveal = Reveal in Freally
settings-context-menu-send-to = Send to Freally (path)

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
settings-flists-format-srcb = Freally Bundle (.srcb)

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
settings-about-version = Freally { $version }
settings-about-license = License
settings-about-credits = Credits
settings-about-notices = Open-source notices

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = First-run wizard
wizard-step-of-total = Step { $step } of { $total }
wizard-roots-hint = Add the folders or volumes you want Freally to watch. You can change this later from Indexes settings.
wizard-browse = Browse…
wizard-roots-placeholder = …or paste a path
wizard-roots-add = Add
wizard-roots-remove = Remove
wizard-roots-empty = No roots configured yet.
wizard-locale-hint = Freally ships in 18 languages. You can switch later.
wizard-theme-hint = System follows your OS appearance setting.
wizard-back = Back
wizard-next = Next

# Status bar polish.
statusbar-hotkey-hint = Hotkey: { $hotkey }
statusbar-cycle-theme = Cycle theme
statusbar-indexed-suffix = indexed

# Results / lenses.
lens-expand = Expand lens
lens-collapse = Collapse lens
lens-no-matches = No matches in this lens.

# Preview pane.
preview-header = Preview
preview-loading = Loading…
preview-select-file = Select a file to preview.
preview-unavailable = No preview available

# Bookmarks.
bookmarks-label = ★ Bookmarks
bookmarks-empty-hint = No bookmarks yet. Press Ctrl+D to save the current query.
bookmarks-organize-title = Organize Bookmarks
bookmarks-organize-empty = No bookmarks yet.
bookmarks-rename = Rename
bookmarks-close = Close

# Settings tree extras.
settings-group-history = History
settings-group-privacy = Privacy & Updates
settings-group-logs = Logs & Debug
settings-group-backup = Backup, Export, Reset
settings-tree-custom-lens = Custom
settings-unsaved-changes = unsaved changes

# About dialog.
about-dialog-title = Freally
about-copyright = Copyright © 2026 Mike Weaver. All rights reserved.
about-close = Close

# Connect endpoint dialog.
connect-ftp-title = Connect To FTP Server
connect-ftp-host = Host:
connect-ftp-port = Port:
connect-ftp-username = Username:
connect-ftp-password = Password:
connect-ftp-link-type = Link type:

# UI panel.
ui-hint = Theme, tray / menu-bar integration, search-as-you-type, row density. Direct voidtools-Everything parity plus Freally additions marked with (+).
ui-section-theme = Theme
ui-theme-system-default = System (default)
ui-section-tray = Tray / Menu Bar
ui-section-search-behavior = Search Behavior
ui-section-result-rows = Result Rows
ui-single-click-system-default = System settings (default)
ui-single-click-always = Always single click
ui-single-click-always-double = Always double click
ui-underline-always = Always
ui-underline-on-hover = On hover
ui-underline-never = Never

# Home panel.
home-hint = Defaults loaded on app launch — every dropdown can stick to "Use last value" or pin a fixed value. Lens visibility / result limits are Freally additions (+).
home-section-match = Match Defaults
home-section-search-sort = Search & Sort Defaults
home-search-placeholder = Empty by default
home-section-index = Index Source
home-file-list-path = File list path
home-https-endpoint = HTTPS API endpoint URL
home-endpoint-token = Token (fingerprint shown)

# Backup panel.
backup-section-settings = Settings (+)
backup-section-bookmarks = Bookmarks + Custom Extractors (+)
backup-section-reset = Reset
backup-toast-exported = Exported settings to { $path }
backup-toast-export-failed = Export failed: { $error }
backup-toast-imported = Imported settings
backup-toast-import-failed = Import failed: { $error }
backup-toast-bookmarks-exported = Exported bookmarks
backup-toast-bookmarks-export-failed = Bookmark export failed: { $error }
backup-toast-bookmarks-imported = Imported bookmarks
backup-toast-bookmarks-import-failed = Bookmark import failed: { $error }
backup-confirm-reset = Reset all settings to defaults? This cannot be undone (the dialog stays open).
backup-toast-reset = All settings reset

# Keyboard panel.
keyboard-section-global = Global Hotkeys
keyboard-placeholder-example = Super+Space
keyboard-section-commands = Commands
keyboard-placeholder-command = command id (e.g. file.export_results)
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = Search History
history-section-run = Run History
history-section-privacy = Privacy (+)
history-record-filename = Record filename-lens history
history-record-content = Record content-lens history
history-record-audio = Record audio-lens history
history-record-similarity = Record similarity-lens history

# Locale panel.
locale-section-language = Language (+)
locale-section-time-date = Time / Date (+)
locale-date-os = OS default
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = Custom
locale-date-custom-format = Custom format
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = Numbers (+)
locale-number-os = OS default
locale-number-custom = Custom
locale-thousands-sep = Thousands separator
locale-decimal-sep = Decimal separator

# Folders panel.
folders-hint = Additional watched folders beyond the default volumes.
folders-list-title = Watched folders
folders-empty = No folders added yet.
folders-remove = Remove
folders-section-title-dynamic = Settings for { $path }
folders-section-schedule = Rescan schedule
folders-schedule-daily = Every day at HH:MM
folders-schedule-hours = Every N hours
folders-schedule-never = Never
folders-hour = Hour
folders-minute = Minute
folders-hours = Hours
folders-id-label = Folder ID (read-only)
folders-select-prompt = Select a folder to configure it.
folders-section-extras = Freally Extras (+)
folders-extras-note = Rescan on resume from sleep is enabled by default in this build; the toggle joins the folder-level controls in Phase 13's polish pass.

# Volumes panel.
volumes-hint = Cross-platform analogue of voidtools-Everything's NTFS / ReFS panels. Auto-detects NTFS / ReFS / exFAT / FAT32 (Win), APFS / HFS+ (macOS), ext4 / Btrfs / ZFS / XFS / F2FS (Linux).
volumes-section-auto-include = Auto-include
volumes-list-title = Detected volumes
volumes-detecting = Detecting…
volumes-empty = No volumes detected.
volumes-select-prompt = Select a volume to configure it.

# About panel polish.
about-section-version = Version (+)
about-section-license = License (+)
about-license-text = Mike Weaver — All Rights Reserved. This is proprietary software.
about-license-spdx = SPDX: { $spdx }
about-section-credits = Credits (+)
about-credits-inspired = Inspired by Everything by voidtools.
about-credits-voidtools = voidtools.com
about-credits-repo = Project repository

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = Contains commands for working with Freally.
menu-file-new-window = New Search Window
menu-file-open-list = Open File List…
menu-file-close-list = Close File List
menu-file-close = Close
menu-file-export-results = Export Results…
menu-file-export-bundle = Export Index Bundle…
menu-file-exit = Exit

# Edit menu.
menu-edit-hint = Contains commands for editing search results.
menu-edit-cut = Cut
menu-edit-copy = Copy
menu-edit-paste = Paste
menu-edit-copy-to-folder = Copy to Folder…
menu-edit-move-to-folder = Move to Folder…
menu-edit-select-all = Select All
menu-edit-invert-selection = Invert Selection
menu-edit-advanced = Advanced
menu-edit-copy-full-name = Copy Full Name
menu-edit-copy-path = Copy Path
menu-edit-copy-filename = Copy Filename
menu-edit-copy-as-json = Copy as JSON
menu-edit-copy-with-metadata = Copy with metadata
menu-edit-copy-as-bundle-ref = Copy as Freally Bundle reference

# View menu.
menu-view-hint = Contains commands for manipulating the view.
menu-view-filters = Filters
menu-view-preview = Preview
menu-view-status-bar = Status Bar
menu-view-thumbs-xl = Extra Large Thumbnails
menu-view-thumbs-l = Large Thumbnails
menu-view-thumbs-m = Medium Thumbnails
menu-view-details = Details
menu-view-window-size = Window Size
menu-view-window-size-hint = Contains commands for adjusting the size of the window.
menu-view-window-small = Small
menu-view-window-medium = Medium
menu-view-window-large = Large
menu-view-window-auto = Auto Fit
menu-view-zoom = Zoom
menu-view-zoom-hint = Contains commands for adjusting the font and icon size.
menu-view-zoom-in = Zoom In
menu-view-zoom-out = Zoom Out
menu-view-zoom-reset = Reset
menu-view-sort-by = Sort by
menu-view-sort-by-hint = Contains commands for sorting the result list.
menu-view-sort-name = Name
menu-view-sort-path = Path
menu-view-sort-size = Size
menu-view-sort-ext = Extension
menu-view-sort-type = Type
menu-view-sort-modified = Date Modified
menu-view-sort-created = Date Created
menu-view-sort-accessed = Date Accessed
menu-view-sort-attributes = Attributes
menu-view-sort-recently-changed = Date Recently Changed
menu-view-sort-run-count = Run Count
menu-view-sort-run-date = Date Run
menu-view-sort-file-list-filename = File List Filename
menu-view-sort-lufs = LUFS
menu-view-sort-length = Length
menu-view-sort-similarity = Similarity Score
menu-view-sort-asc = Ascending
menu-view-sort-desc = Descending
menu-view-go-to = Go To
menu-view-refresh = Refresh
menu-view-theme = Theme
menu-view-theme-hint = Switch between system, light, or dark themes.
menu-view-lenses = Lenses
menu-view-lenses-hint = Toggle visibility of each lens in the result list.
menu-view-on-top = On Top
menu-view-on-top-hint = Contains commands for keeping this window on top of other windows.
menu-view-on-top-never = Never
menu-view-on-top-always = Always
menu-view-on-top-while-searching = While Searching

# Search menu.
menu-search-hint = Contains search toggles.
menu-search-match-case = Match Case
menu-search-match-whole-word = Match Whole Word
menu-search-match-path = Match Path
menu-search-match-diacritics = Match Diacritics
menu-search-enable-regex = Enable Regex
menu-search-advanced = Advanced Search…
menu-search-add-to-filters = Add to Filters…
menu-search-organize-filters = Organize Filters…
menu-search-filter-everything = Everything
menu-search-filter-archive = Compressed (Archive)
menu-search-filter-folder = Folder
menu-search-filter-custom = Custom Filter…

# Bookmarks menu.
menu-bookmarks-hint = Contains commands for working with bookmarks.
menu-bookmarks-add = Add to Bookmarks
menu-bookmarks-organize = Organize Bookmarks…

# Tools menu.
menu-tools-hint = Contains tools commands.
menu-tools-connect = Connect to FTP Server…
menu-tools-disconnect = Disconnect from FTP Server
menu-tools-file-list-editor = File List Editor…
menu-tools-index-maintenance = Index maintenance
menu-tools-index-maintenance-hint = Index maintenance tools.
menu-tools-verify-index = Verify Index…
menu-tools-compact-index = Compact Index…
menu-tools-rebuild-index = Force Rebuild Index…
menu-tools-custom-extractor = Custom Extractor Manager…
menu-tools-custom-extractor-hint = Manage Wasm-sandboxed custom extractors.
menu-tools-options = Options…

# Help menu.
menu-help-hint = Contains help commands.
menu-help-help = Freally Help
menu-help-search-syntax = Search Syntax
menu-help-regex-syntax = Regex Syntax
menu-help-audio-ref = Audio Modifier Reference
menu-help-similarity-ref = Similarity Modifier Reference
menu-help-cli-options = Command Line Options
menu-help-website = Freally Website
menu-help-check-updates = Check for Updates…
menu-help-sponsor = Sponsor / Donate
menu-help-about = About Freally…

# Result column headers (short forms used in the table header row).
column-name = Name
column-path = Path
column-size = Size
column-modified = Modified
column-type = Type
column-ext = Ext
column-sort-by = Sort by { $name }
column-resize = Resize { $name } column

# Section subtitle bars used inside multiple settings panels.
section-behavior = Behavior
section-rendering = Rendering
section-status-bar = Status Bar
section-display-format = Display Format
section-loading-priority = Loading Priority
section-compatibility = Compatibility
section-storage = Storage
section-index-fields = Index Fields
section-maintenance = Maintenance
section-logging = Logging
section-tools = Tools
section-privacy = Privacy
section-auto-update = Auto-update (+)
section-bind = Bind
section-lens = Lens
section-budgets = Budgets
section-other = Other
section-per-format-mode = Per-format Mode
section-loudness = Loudness
section-tuning = Tuning (+)
section-minhash-lsh = MinHash + LSH Parameters (+)
section-top-level = Top-level
section-file-globs = File globs
section-file-list-settings = Settings for selected file list
section-editor-format = Editor + Format (E + +)
section-api-server = API Server (E adapted)
section-freally-extras = Freally Extras (+)
section-freally-additions = Freally Additions (+)
section-freally-extensions = Freally Extensions (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = Use last value
opt-use-last-value-default = Use last value (default)
opt-low = Low
opt-normal-default = Normal (default)
opt-high = High
opt-disabled = Disabled
opt-off = Off
opt-on-battery = When on battery
opt-always = Always
opt-clamp-default = Clamp (default)
opt-wrap = Wrap
opt-none = None
opt-strict-refuse = Strict (refuse queries on corruption)
opt-lenient-warn = Lenient (warn but query)
opt-system-default = System default
opt-drag-select = Drag-select
opt-auto-binary = Auto (binary)
opt-auto-decimal = Auto (decimal)

# Unit suffixes shown next to number inputs.
unit-days = days
unit-b = B
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-tb = TB

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = Eager
opt-lazy-default = Lazy (default)
opt-on = On
opt-on-default = On (default)
opt-all = All
opt-weekly = Weekly
opt-monthly = Monthly
opt-name-asc = Name asc
opt-name-desc = Name desc
opt-size-asc = Size asc
opt-size-desc = Size desc
opt-modified-asc = Date modified asc
opt-modified-desc = Date modified desc
opt-compact = Compact
opt-comfortable = Comfortable
opt-details = Details
opt-thumbnails = Thumbnails
opt-local-db-default = Local database (default)
opt-file-list = File list
opt-https-endpoint = HTTPS API endpoint
opt-right-default = Right (default)
opt-bottom = Bottom
opt-or-and-default = OR > AND (default)
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128 (default)
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = Broadcast film (-23)
opt-true-peak = True peak (4× oversampling, default)
opt-sample-peak = Sample peak
opt-auto-per-doc = Auto (per-doc)
opt-log-error = Error
opt-log-warn = Warn
opt-log-info-default = Info (default)
opt-log-debug = Debug
opt-log-trace = Trace

# More Freally apps (Central inside panel) — host chrome
menu-help-more-apps = More Freally apps…
moreapps-title = More Freally apps
