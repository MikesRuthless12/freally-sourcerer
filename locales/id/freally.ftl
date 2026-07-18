# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = Satu pencarian. Setiap sumber. Setiap OS.
window-title = Freally Sourcerer
search-placeholder = Cari…
about-version = Versi { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = Siap
status-indexed = Terindeks ({ $count } berkas)
status-indexing = Mengindeks… { $done }/{ $total }
status-paused = Dijeda
status-error = Galat
status-result-count-one = { $count } hasil
status-result-count-many = { $count } hasil
status-selection = · { $count } dipilih
status-selection-size = Dipilih: { $size }
status-query-timing = Kueri: { $ms } ms
status-endpoint-local = DB Lokal
status-endpoint-remote = API: { $name }

menu-file = Berkas
menu-edit = Edit
menu-view = Tampilan
menu-search = Cari
menu-bookmarks = Markah
menu-tools = Alat
menu-help = Bantuan

theme-system = Sistem
theme-light = Terang
theme-dark = Gelap

lens-filename = Nama Berkas
lens-content = Konten
lens-audio = Audio
lens-similarity = Kemiripan

parse-error-empty = Ketik kueri untuk memulai.
parse-error-unknown = Sintaks tidak dikenali di sekitar sini.

action-open = Buka
action-reveal = Tampilkan di folder
action-copy-path = Salin path
action-copy-name = Salin nama
action-delete = Hapus

quick-filter-audio = Audio
quick-filter-video = Video
quick-filter-image = Gambar
quick-filter-document = Dokumen
quick-filter-executable = Berkas Eksekusi
quick-filter-archive = Arsip

wizard-title = Selamat datang di Freally
wizard-step-roots = Pilih yang akan diindeks
wizard-step-hotkey = Pilih tombol pintas global
wizard-step-locale = Pilih bahasa Anda
wizard-step-theme = Pilih tema
wizard-finish = Selesai

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = Opsi
settings-search-placeholder = Cari opsi…
settings-restore-defaults = Kembalikan ke Bawaan
settings-ok = OK
settings-cancel = Batal
settings-apply = Terapkan

# Tree nav groups (PRD §8.1.1).
settings-group-general = Umum
settings-group-indexes = Indeks
settings-group-lenses = Lensa
settings-group-network = Jaringan

# Tree nav leaves.
settings-node-ui = UI
settings-node-home = Beranda
settings-node-search = Cari
settings-node-results = Hasil
settings-node-view = Tampilan
settings-node-context-menu = Menu Konteks
settings-node-fonts-colors = Font & Warna
settings-node-keyboard = Papan Ketik
settings-node-history = Riwayat
settings-node-indexes-top = (tingkat atas)
settings-node-volumes = Volume
settings-node-folders = Folder
settings-node-file-lists = Daftar Berkas
settings-node-exclude = Kecualikan
settings-node-https-server = Server HTTP / HTTPS
settings-node-etp-api = API ETP / FTP
settings-node-privacy = Privasi & Pembaruan
settings-node-logs = Log & Debug
settings-node-backup = Cadangkan, Ekspor, Reset
settings-node-locale = Lokal
settings-node-about = Tentang

# §8.2 General → UI.
settings-ui-theme = Tema
settings-ui-run-bg = Jalankan di latar belakang
settings-ui-show-tray = Tampilkan ikon baki / bilah menu
settings-ui-single-click-tray = Klik tunggal baki / bilah menu
settings-ui-new-window-from-tray = Buka jendela baru dari ikon baki
settings-ui-new-window-on-launch = Buka jendela baru saat menjalankan Freally
settings-ui-search-as-you-type = Cari sambil mengetik
settings-ui-select-on-mouse-click = Pilih pencarian saat klik tetikus
settings-ui-focus-on-activate = Fokuskan pencarian saat diaktifkan
settings-ui-full-row-select = Pilih seluruh baris
settings-ui-single-click-open = Buka dengan klik tunggal
settings-ui-underline-titles = Garis bawahi judul ikon
settings-ui-row-density = Kepadatan hasil
settings-ui-row-density-compact = Padat (32 px)
settings-ui-row-density-comfortable = Nyaman (44 px)
settings-ui-show-timing-badges = Tampilkan lencana waktu per lensa
settings-ui-anim-crossfade = Transisi tema beranimasi

# §8.3 General → Home.
settings-home-match-case = Cocokkan huruf besar/kecil
settings-home-match-whole-word = Cocokkan seluruh kata
settings-home-match-path = Cocokkan path
settings-home-match-diacritics = Cocokkan diakritik
settings-home-match-regex = Cocokkan regex
settings-home-search = Cari (kueri bawaan kustom)
settings-home-filter = Filter
settings-home-sort = Urutkan
settings-home-view = Tampilan
settings-home-index = Indeks
settings-home-default-lens-visibility = Visibilitas lensa bawaan
settings-home-default-lens-result-limits = Batas hasil lensa bawaan

# §8.4 General → Search.
settings-search-fast-ascii = Pencarian ASCII cepat
settings-search-mp-sep = Cocokkan path saat istilah pencarian memuat pemisah path
settings-search-mw-fn = Cocokkan seluruh nama berkas saat memakai wildcard
settings-search-lit-ops = Izinkan operator literal
settings-search-paren = Izinkan pengelompokan tanda kurung
settings-search-env = Perluas variabel lingkungan
settings-search-fwd-slash = Ganti garis miring depan dengan garis miring belakang
settings-search-precedence = Prioritas operator
settings-search-strict-everything = Mode sintaks ketat Everything
settings-search-auto-regex = Deteksi regex otomatis
settings-search-mod-comp = Pelengkapan pengubah
settings-search-parse-tree = Tampilkan parse-tree saat diarahkan

# §8.5 General → Results.
settings-results-hide-empty = Sembunyikan hasil saat pencarian kosong
settings-results-clear-on-search = Hapus pilihan saat mencari
settings-results-close-on-execute = Tutup jendela saat mengeksekusi
settings-results-dbl-path = Buka path dengan klik ganda di kolom path
settings-results-auto-scroll = Gulir tampilan secara otomatis
settings-results-dquote-copy = Salin tanda kutip ganda sebagai path
settings-results-no-ext-rename = Jangan pilih ekstensi saat mengganti nama
settings-results-sort-date-desc = Urutkan tanggal menurun dahulu
settings-results-sort-size-desc = Urutkan ukuran menurun dahulu
settings-results-list-focus = Fokus daftar hasil
settings-results-icon-prio = Prioritas pemuatan ikon
settings-results-thumb-prio = Prioritas pemuatan gambar mini
settings-results-ext-prio = Prioritas pemuatan informasi lanjutan
settings-results-group-by-lens = Kelompokkan hasil berdasarkan lensa
settings-results-snippet-inline = Tampilkan pratinjau cuplikan sebaris

# §8.6 General → View.
settings-view-double-buffer = Buffer ganda
settings-view-alt-rows = Warna baris berselang-seling
settings-view-row-mouseover = Tampilkan sorotan baris saat tetikus di atas
settings-view-highlight-terms = Tampilkan istilah pencarian tersorot
settings-view-status-show-selected = Tampilkan item terpilih di bilah status
settings-view-rc-with-sel = Tampilkan jumlah hasil bersama jumlah pilihan
settings-view-status-show-size = Tampilkan ukuran di bilah status
settings-view-tooltips = Tampilkan tooltip
settings-view-update-on-scroll = Perbarui tampilan segera setelah menggulir
settings-view-size-format = Format ukuran
settings-view-selection-rect = Persegi panjang pilihan
settings-view-audio-badges = Tampilkan lencana LUFS / codec / durasi pada baris audio
settings-view-similarity-score = Tampilkan skor kemiripan MinHash pada baris kemiripan
settings-view-preview-pane = Panel pratinjau

# §8.7 General → Context Menu.
settings-context-menu-visibility = Visibilitas
settings-context-menu-show = Tampilkan
settings-context-menu-shift = Tampilkan hanya saat Shift ditahan
settings-context-menu-hide = Sembunyikan
settings-context-menu-command = Makro perintah
settings-context-menu-open-folders = Buka (Folder)
settings-context-menu-open-files = Buka (Berkas)
settings-context-menu-open-path = Buka Path
settings-context-menu-explore = Jelajahi
settings-context-menu-explore-path = Jelajahi Path
settings-context-menu-copy-name = Salin Nama ke Papan Klip
settings-context-menu-copy-path = Salin Path ke Papan Klip
settings-context-menu-copy-full-name = Salin Nama Lengkap ke Papan Klip
settings-context-menu-reveal = Tampilkan di Freally
settings-context-menu-send-to = Kirim ke Freally (path)

# §8.8 General → Fonts & Colors.
settings-fc-font = Font
settings-fc-size = Ukuran
settings-fc-state-normal = Normal
settings-fc-state-highlighted = Tersorot
settings-fc-state-current-sort = Urutan Saat Ini
settings-fc-state-current-sort-h = Urutan Saat Ini (Tersorot)
settings-fc-state-selected = Terpilih
settings-fc-state-selected-h = Terpilih (Tersorot)
settings-fc-state-inactive-selected = Terpilih Tidak Aktif
settings-fc-state-inactive-selected-h = Terpilih Tidak Aktif (Tersorot)
settings-fc-foreground = Latar Depan
settings-fc-background = Latar Belakang
settings-fc-bold = Tebal
settings-fc-italic = Miring
settings-fc-default = Bawaan
settings-fc-per-lens-accent = Aksen Per-Lensa
settings-fc-theme-inherit = Balik otomatis warna kustom saat ganti tema

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Tombol Pintas Global
settings-keyboard-new-window = Tombol Pintas jendela baru
settings-keyboard-show-window = Tombol Pintas tampilkan jendela
settings-keyboard-toggle-window = Tombol Pintas alihkan jendela
settings-keyboard-show-commands = Tampilkan perintah yang memuat
settings-keyboard-add-chord = + Tambah kombinasi
settings-keyboard-remove-chord = Hapus

# §8.10 History.
settings-history-search-enable = Aktifkan riwayat pencarian
settings-history-search-keep = Simpan riwayat pencarian selama { $days } hari
settings-history-run-enable = Aktifkan riwayat jalankan
settings-history-run-keep = Simpan riwayat jalankan selama { $days } hari
settings-history-clear-now = Hapus Sekarang
settings-history-privacy-mode = Mode privasi
settings-history-per-lens = Riwayat per-lensa

# §8.11 Indexes (top-level).
settings-ix-database-location = Lokasi basis data
settings-ix-multiuser = Nama berkas basis data multi-pengguna
settings-ix-compress = Kompresi basis data
settings-ix-recent-changes = Indeks perubahan terbaru
settings-ix-file-size = Indeks ukuran berkas
settings-ix-fast-size-sort = Urutan ukuran cepat
settings-ix-folder-size = Indeks ukuran folder
settings-ix-fast-folder-size-sort = Urutan ukuran folder cepat
settings-ix-date-created = Indeks tanggal dibuat
settings-ix-fast-date-created = Urutan tanggal dibuat cepat
settings-ix-date-modified = Indeks tanggal diubah
settings-ix-fast-date-modified = Urutan tanggal diubah cepat
settings-ix-date-accessed = Indeks tanggal diakses
settings-ix-fast-date-accessed = Urutan tanggal diakses cepat
settings-ix-attributes = Indeks atribut
settings-ix-fast-attributes = Urutan atribut cepat
settings-ix-fast-path-sort = Urutan path cepat
settings-ix-fast-extension-sort = Urutan ekstensi cepat
settings-ix-force-rebuild = Paksa Bangun Ulang
settings-ix-compact = Padatkan Indeks
settings-ix-verify = Verifikasi Indeks
settings-ix-integrity-policy = Kebijakan integritas indeks
settings-ix-memory-budget = Anggaran memori untuk pengindeks
settings-ix-throttle = Pembatasan pengindeksan latar belakang

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Sertakan otomatis volume tetap baru
settings-vol-auto-removable = Sertakan otomatis volume lepasan baru
settings-vol-auto-remove-offline = Hapus otomatis volume luring
settings-vol-detected = Volume terdeteksi
settings-vol-include = Sertakan dalam indeks
settings-vol-include-only = Sertakan saja (glob/regex)
settings-vol-enable-usn = Aktifkan USN Journal
settings-vol-enable-fsevents = Aktifkan aliran FSEvents
settings-vol-enable-inotify = Aktifkan inotify (atau fanotify jika dengan hak akses tinggi)
settings-vol-buffer = Ukuran buffer journal (KB)
settings-vol-allocation-delta = Delta alokasi (KB)
settings-vol-load-recent = Muat perubahan terbaru dari journal saat memulai
settings-vol-monitor = Pantau perubahan
settings-vol-recreate-journal = Buat ulang journal
settings-vol-reset-stream = Reset aliran FSEvents
settings-vol-upgrade-fanotify = Tingkatkan ke fanotify (polkit)
settings-vol-remove = Hapus

# §8.13 Indexes → Folders.
settings-folders-watched = Folder yang dipantau
settings-folders-add = Tambah…
settings-folders-rescan-now = Pindai Ulang Sekarang
settings-folders-rescan-all = Pindai Ulang Semua Sekarang
settings-folders-monitor = Coba pantau perubahan
settings-folders-buffer = Ukuran buffer
settings-folders-rescan-on-full = Pindai ulang saat buffer penuh

# §8.14 Indexes → File Lists.
settings-flists-add = Tambah…
settings-flists-monitor = Pantau perubahan
settings-flists-editor = Editor Daftar Berkas…
settings-flists-format = Format daftar berkas
settings-flists-format-text = Teks (satu path per baris)
settings-flists-format-json = JSON (dengan metadata)
settings-flists-format-srcb = Freally Bundle (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Kecualikan berkas dan folder tersembunyi
settings-exclude-system = Kecualikan berkas dan folder sistem
settings-exclude-list-en = Aktifkan daftar pengecualian
settings-exclude-folders = Kecualikan folder
settings-exclude-include-only-files = Sertakan berkas saja (glob)
settings-exclude-files = Kecualikan berkas (glob)
settings-exclude-os-recommended = Terapkan pengecualian yang direkomendasikan OS
settings-exclude-by-class = Kecualikan berdasarkan kelas ekstensi

# §8.16 Lenses → Filename.
settings-lf-trigram = Agresivitas pra-filter trigram
settings-lf-suffix-mem = Anggaran memori suffix-array
settings-lf-wildcard-limit = Batas perluasan wildcard
settings-lf-regex-timeout = Batas waktu regex

# §8.17 Lenses → Content.
settings-lc-enable = Aktifkan lensa konten
settings-lc-time-budget = Anggaran waktu per dokumen
settings-lc-mem-ceiling = Batas memori per dokumen
settings-lc-snippet-len = Panjang cuplikan
settings-lc-stop-words = Bahasa stop-word
settings-lc-re-extract = Ekstrak ulang saat pengaturan berubah
settings-lc-verify-blobs = Verifikasi checksum blob teks terekstrak saat dibaca

# §8.18 Lenses → Audio.
settings-la-enable = Aktifkan lensa audio
settings-la-lufs-ref = Standar referensi LUFS
settings-la-peak-compute = Hitung puncak via
settings-la-silence-thresh = Ambang keheningan
settings-la-re-extract-modify = Ekstrak ulang saat peristiwa Modify

# §8.19 Lenses → Similarity.
settings-ls-enable = Aktifkan lensa kemiripan
settings-ls-sig-size = Ukuran tanda tangan MinHash (k)
settings-ls-bands = Pita LSH
settings-ls-recall = Ambang recall
settings-ls-result-cap = Batas hasil

# §8.20 Lenses → Custom.
settings-custom-registry = Registri
settings-custom-trust = Tepercaya
settings-custom-refresh-hashes = Segarkan hash

# §8.21-§8.22 Network.
settings-net-https-enable = Aktifkan server HTTPS
settings-net-bind = Ikat ke antarmuka
settings-net-port = Dengarkan di port
settings-net-force-https = Paksa HTTPS
settings-net-legacy-auth = Autentikasi HTTP-basic lawas
settings-net-token-regen = Buat ulang token
settings-net-api-enable = Aktifkan server API
settings-net-legacy-ftp = Dukungan FTP/ETP polos lawas

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Pembaruan otomatis
settings-privacy-prerelease = Saluran pra-rilis
settings-privacy-network-policy = Kebijakan panggilan jaringan

# §8.24 Logs & Debug.
settings-logs-level = Tingkat log
settings-logs-location = Lokasi berkas log
settings-logs-retention = Retensi log
settings-logs-debug-overlay = Tampilkan overlay debug
settings-logs-open-folder = Buka folder log
settings-logs-export-bundle = Ekspor paket diagnostik

# §8.25 Backup, Export, Reset.
settings-backup-export = Ekspor pengaturan
settings-backup-import = Impor pengaturan
settings-backup-export-bookmarks = Ekspor paket markah
settings-backup-import-bookmarks = Impor paket markah
settings-backup-reset-all = Reset semua pengaturan ke bawaan

# §8.26 Locale.
settings-locale-current = Lokal saat ini
settings-locale-rtl-preview = Pratinjau RTL
settings-locale-date-format = Format tanggal
settings-locale-number-format = Format angka

# §8.27 About.
settings-about-version = Freally { $version }
settings-about-license = Lisensi
settings-about-credits = Kredit
settings-about-notices = Pemberitahuan sumber terbuka

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = Wisaya jalankan pertama
wizard-step-of-total = Langkah { $step } dari { $total }
wizard-roots-hint = Tambahkan folder atau volume yang ingin dipantau Freally. Anda dapat mengubahnya nanti dari pengaturan Indeks.
wizard-browse = Telusuri…
wizard-roots-placeholder = …atau tempel sebuah path
wizard-roots-add = Tambah
wizard-roots-remove = Hapus
wizard-roots-empty = Belum ada root yang dikonfigurasi.
wizard-locale-hint = Freally tersedia dalam 18 bahasa. Anda dapat beralih nanti.
wizard-theme-hint = Sistem mengikuti pengaturan tampilan OS Anda.
wizard-back = Kembali
wizard-next = Berikutnya

# Status bar polish.
statusbar-hotkey-hint = Tombol Pintas: { $hotkey }
statusbar-cycle-theme = Putar tema
statusbar-indexed-suffix = terindeks

# Results / lenses.
lens-expand = Bentangkan lensa
lens-collapse = Ciutkan lensa
lens-no-matches = Tidak ada kecocokan di lensa ini.

# Preview pane.
preview-header = Pratinjau
preview-loading = Memuat…
preview-select-file = Pilih berkas untuk dipratinjau.
preview-unavailable = Pratinjau tidak tersedia

# Bookmarks.
bookmarks-label = ★ Markah
bookmarks-empty-hint = Belum ada markah. Tekan Ctrl+D untuk menyimpan kueri saat ini.
bookmarks-organize-title = Atur Markah
bookmarks-organize-empty = Belum ada markah.
bookmarks-rename = Ganti Nama
bookmarks-close = Tutup

# Settings tree extras.
settings-group-history = Riwayat
settings-group-privacy = Privasi & Pembaruan
settings-group-logs = Log & Debug
settings-group-backup = Cadangkan, Ekspor, Reset
settings-tree-custom-lens = Kustom
settings-unsaved-changes = perubahan belum disimpan

# About dialog.
about-dialog-title = Freally
about-copyright = Hak Cipta © 2026 Mike Weaver. Semua hak dilindungi.
about-close = Tutup

# Connect endpoint dialog.
connect-ftp-title = Sambungkan ke Server FTP
connect-ftp-host = Host:
connect-ftp-port = Port:
connect-ftp-username = Nama pengguna:
connect-ftp-password = Kata sandi:
connect-ftp-link-type = Tipe tautan:

# UI panel.
ui-hint = Tema, integrasi baki / bilah menu, cari sambil mengetik, kepadatan baris. Kesetaraan langsung dengan voidtools-Everything plus tambahan Freally yang ditandai dengan (+).
ui-section-theme = Tema
ui-theme-system-default = Sistem (bawaan)
ui-section-tray = Baki / Bilah Menu
ui-section-search-behavior = Perilaku Pencarian
ui-section-result-rows = Baris Hasil
ui-single-click-system-default = Pengaturan sistem (bawaan)
ui-single-click-always = Selalu klik tunggal
ui-single-click-always-double = Selalu klik ganda
ui-underline-always = Selalu
ui-underline-on-hover = Saat diarahkan
ui-underline-never = Tidak pernah

# Home panel.
home-hint = Bawaan dimuat saat aplikasi dijalankan — setiap menu turun dapat tetap di "Gunakan nilai terakhir" atau menyematkan nilai tetap. Visibilitas lensa / batas hasil adalah tambahan Freally (+).
home-section-match = Bawaan Pencocokan
home-section-search-sort = Bawaan Pencarian & Pengurutan
home-search-placeholder = Kosong secara bawaan
home-section-index = Sumber Indeks
home-file-list-path = Path daftar berkas
home-https-endpoint = URL endpoint API HTTPS
home-endpoint-token = Token (sidik jari ditampilkan)

# Backup panel.
backup-section-settings = Pengaturan (+)
backup-section-bookmarks = Markah + Pengekstrak Kustom (+)
backup-section-reset = Reset
backup-toast-exported = Pengaturan diekspor ke { $path }
backup-toast-export-failed = Ekspor gagal: { $error }
backup-toast-imported = Pengaturan diimpor
backup-toast-import-failed = Impor gagal: { $error }
backup-toast-bookmarks-exported = Markah diekspor
backup-toast-bookmarks-export-failed = Ekspor markah gagal: { $error }
backup-toast-bookmarks-imported = Markah diimpor
backup-toast-bookmarks-import-failed = Impor markah gagal: { $error }
backup-confirm-reset = Reset semua pengaturan ke bawaan? Ini tidak dapat dibatalkan (dialog tetap terbuka).
backup-toast-reset = Semua pengaturan direset

# Keyboard panel.
keyboard-section-global = Tombol Pintas Global
keyboard-placeholder-example = Super+Space
keyboard-section-commands = Perintah
keyboard-placeholder-command = id perintah (mis. file.export_results)
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = Riwayat Pencarian
history-section-run = Riwayat Jalankan
history-section-privacy = Privasi (+)
history-record-filename = Rekam riwayat lensa nama berkas
history-record-content = Rekam riwayat lensa konten
history-record-audio = Rekam riwayat lensa audio
history-record-similarity = Rekam riwayat lensa kemiripan

# Locale panel.
locale-section-language = Bahasa (+)
locale-section-time-date = Waktu / Tanggal (+)
locale-date-os = Bawaan OS
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = Kustom
locale-date-custom-format = Format kustom
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = Angka (+)
locale-number-os = Bawaan OS
locale-number-custom = Kustom
locale-thousands-sep = Pemisah ribuan
locale-decimal-sep = Pemisah desimal

# Folders panel.
folders-hint = Folder tambahan yang dipantau di luar volume bawaan.
folders-list-title = Folder yang dipantau
folders-empty = Belum ada folder ditambahkan.
folders-remove = Hapus
folders-section-title-dynamic = Pengaturan untuk { $path }
folders-section-schedule = Jadwal pindai ulang
folders-schedule-daily = Setiap hari pukul HH:MM
folders-schedule-hours = Setiap N jam
folders-schedule-never = Tidak pernah
folders-hour = Jam
folders-minute = Menit
folders-hours = Jam
folders-id-label = ID Folder (hanya-baca)
folders-select-prompt = Pilih folder untuk mengonfigurasinya.
folders-section-extras = Tambahan Freally (+)
folders-extras-note = Pindai ulang saat melanjutkan dari tidur aktif secara bawaan di build ini; tombolnya bergabung dengan kontrol tingkat-folder pada penyempurnaan Fase 13.

# Volumes panel.
volumes-hint = Analog lintas platform dari panel NTFS / ReFS voidtools-Everything. Mendeteksi otomatis NTFS / ReFS / exFAT / FAT32 (Win), APFS / HFS+ (macOS), ext4 / Btrfs / ZFS / XFS / F2FS (Linux).
volumes-section-auto-include = Sertakan otomatis
volumes-list-title = Volume terdeteksi
volumes-detecting = Mendeteksi…
volumes-empty = Tidak ada volume terdeteksi.
volumes-select-prompt = Pilih volume untuk mengonfigurasinya.

# About panel polish.
about-section-version = Versi (+)
about-section-license = Lisensi (+)
about-license-text = Mike Weaver — Semua Hak Dilindungi. Ini adalah perangkat lunak berpemilik.
about-license-spdx = SPDX: { $spdx }
about-section-credits = Kredit (+)
about-credits-inspired = Terinspirasi oleh Everything dari voidtools.
about-credits-voidtools = voidtools.com
about-credits-repo = Repositori proyek

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = Berisi perintah untuk bekerja dengan Freally.
menu-file-new-window = Jendela Pencarian Baru
menu-file-open-list = Buka Daftar Berkas…
menu-file-close-list = Tutup Daftar Berkas
menu-file-close = Tutup
menu-file-export-results = Ekspor Hasil…
menu-file-export-bundle = Ekspor Paket Indeks…
menu-file-exit = Keluar

# Edit menu.
menu-edit-hint = Berisi perintah untuk mengedit hasil pencarian.
menu-edit-cut = Potong
menu-edit-copy = Salin
menu-edit-paste = Tempel
menu-edit-copy-to-folder = Salin ke Folder…
menu-edit-move-to-folder = Pindahkan ke Folder…
menu-edit-select-all = Pilih Semua
menu-edit-invert-selection = Balik Pilihan
menu-edit-advanced = Lanjutan
menu-edit-copy-full-name = Salin Nama Lengkap
menu-edit-copy-path = Salin Path
menu-edit-copy-filename = Salin Nama Berkas
menu-edit-copy-as-json = Salin sebagai JSON
menu-edit-copy-with-metadata = Salin dengan metadata
menu-edit-copy-as-bundle-ref = Salin sebagai referensi Freally Bundle

# View menu.
menu-view-hint = Berisi perintah untuk memanipulasi tampilan.
menu-view-filters = Filter
menu-view-preview = Pratinjau
menu-view-status-bar = Bilah Status
menu-view-thumbs-xl = Gambar Mini Ekstra Besar
menu-view-thumbs-l = Gambar Mini Besar
menu-view-thumbs-m = Gambar Mini Sedang
menu-view-details = Detail
menu-view-window-size = Ukuran Jendela
menu-view-window-size-hint = Berisi perintah untuk menyesuaikan ukuran jendela.
menu-view-window-small = Kecil
menu-view-window-medium = Sedang
menu-view-window-large = Besar
menu-view-window-auto = Pas Otomatis
menu-view-zoom = Zoom
menu-view-zoom-hint = Berisi perintah untuk menyesuaikan ukuran font dan ikon.
menu-view-zoom-in = Perbesar
menu-view-zoom-out = Perkecil
menu-view-zoom-reset = Reset
menu-view-sort-by = Urutkan berdasarkan
menu-view-sort-by-hint = Berisi perintah untuk mengurutkan daftar hasil.
menu-view-sort-name = Nama
menu-view-sort-path = Path
menu-view-sort-size = Ukuran
menu-view-sort-ext = Ekstensi
menu-view-sort-type = Tipe
menu-view-sort-modified = Tanggal Diubah
menu-view-sort-created = Tanggal Dibuat
menu-view-sort-accessed = Tanggal Diakses
menu-view-sort-attributes = Atribut
menu-view-sort-recently-changed = Tanggal Baru Diubah
menu-view-sort-run-count = Jumlah Jalankan
menu-view-sort-run-date = Tanggal Jalankan
menu-view-sort-file-list-filename = Nama Berkas Daftar Berkas
menu-view-sort-lufs = LUFS
menu-view-sort-length = Durasi
menu-view-sort-similarity = Skor Kemiripan
menu-view-sort-asc = Menaik
menu-view-sort-desc = Menurun
menu-view-go-to = Ke
menu-view-refresh = Segarkan
menu-view-theme = Tema
menu-view-theme-hint = Beralih antara tema sistem, terang, atau gelap.
menu-view-lenses = Lensa
menu-view-lenses-hint = Alihkan visibilitas setiap lensa di daftar hasil.
menu-view-on-top = Di Atas
menu-view-on-top-hint = Berisi perintah untuk menjaga jendela ini di atas jendela lain.
menu-view-on-top-never = Tidak pernah
menu-view-on-top-always = Selalu
menu-view-on-top-while-searching = Saat Mencari

# Search menu.
menu-search-hint = Berisi tombol pencarian.
menu-search-match-case = Cocokkan Huruf Besar/Kecil
menu-search-match-whole-word = Cocokkan Seluruh Kata
menu-search-match-path = Cocokkan Path
menu-search-match-diacritics = Cocokkan Diakritik
menu-search-enable-regex = Aktifkan Regex
menu-search-advanced = Pencarian Lanjutan…
menu-search-add-to-filters = Tambahkan ke Filter…
menu-search-organize-filters = Atur Filter…
menu-search-filter-everything = Everything
menu-search-filter-archive = Terkompresi (Arsip)
menu-search-filter-folder = Folder
menu-search-filter-custom = Filter Kustom…

# Bookmarks menu.
menu-bookmarks-hint = Berisi perintah untuk bekerja dengan markah.
menu-bookmarks-add = Tambahkan ke Markah
menu-bookmarks-organize = Atur Markah…

# Tools menu.
menu-tools-hint = Berisi perintah alat.
menu-tools-connect = Sambungkan ke Server FTP…
menu-tools-disconnect = Putuskan dari Server FTP
menu-tools-file-list-editor = Editor Daftar Berkas…
menu-tools-index-maintenance = Pemeliharaan indeks
menu-tools-index-maintenance-hint = Alat pemeliharaan indeks.
menu-tools-verify-index = Verifikasi Indeks…
menu-tools-compact-index = Padatkan Indeks…
menu-tools-rebuild-index = Paksa Bangun Ulang Indeks…
menu-tools-custom-extractor = Pengelola Pengekstrak Kustom…
menu-tools-custom-extractor-hint = Kelola pengekstrak kustom dengan kotak pasir Wasm.
menu-tools-options = Opsi…

# Help menu.
menu-help-hint = Berisi perintah bantuan.
menu-help-help = Bantuan Freally
menu-help-search-syntax = Sintaks Pencarian
menu-help-regex-syntax = Sintaks Regex
menu-help-audio-ref = Referensi Pengubah Audio
menu-help-similarity-ref = Referensi Pengubah Kemiripan
menu-help-cli-options = Opsi Baris Perintah
menu-help-website = Situs Web Freally
menu-help-check-updates = Periksa Pembaruan…
menu-help-sponsor = Sponsori / Donasi
menu-help-about = Tentang Freally…

# Result column headers (short forms used in the table header row).
column-name = Nama
column-path = Path
column-size = Ukuran
column-modified = Diubah
column-type = Tipe
column-ext = Ekst
column-sort-by = Urutkan berdasarkan { $name }
column-resize = Ubah ukuran kolom { $name }

# Section subtitle bars used inside multiple settings panels.
section-behavior = Perilaku
section-rendering = Perenderan
section-status-bar = Bilah Status
section-display-format = Format Tampilan
section-loading-priority = Prioritas Pemuatan
section-compatibility = Kompatibilitas
section-storage = Penyimpanan
section-index-fields = Bidang Indeks
section-maintenance = Pemeliharaan
section-logging = Pencatatan Log
section-tools = Alat
section-privacy = Privasi
section-auto-update = Pembaruan otomatis (+)
section-bind = Ikat
section-lens = Lensa
section-budgets = Anggaran
section-other = Lainnya
section-per-format-mode = Mode Per-format
section-loudness = Kenyaringan
section-tuning = Penyetelan (+)
section-minhash-lsh = Parameter MinHash + LSH (+)
section-top-level = Tingkat atas
section-file-globs = Glob berkas
section-file-list-settings = Pengaturan untuk daftar berkas terpilih
section-editor-format = Editor + Format (E + +)
section-api-server = Server API (E disesuaikan)
section-freally-extras = Tambahan Freally (+)
section-freally-additions = Tambahan Freally (+)
section-freally-extensions = Ekstensi Freally (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = Gunakan nilai terakhir
opt-use-last-value-default = Gunakan nilai terakhir (bawaan)
opt-low = Rendah
opt-normal-default = Normal (bawaan)
opt-high = Tinggi
opt-disabled = Dinonaktifkan
opt-off = Mati
opt-on-battery = Saat memakai baterai
opt-always = Selalu
opt-clamp-default = Jepit (bawaan)
opt-wrap = Bungkus
opt-none = Tidak ada
opt-strict-refuse = Ketat (tolak kueri saat ada kerusakan)
opt-lenient-warn = Longgar (peringatkan tapi tetap kueri)
opt-system-default = Bawaan sistem
opt-drag-select = Seret-pilih
opt-auto-binary = Otomatis (biner)
opt-auto-decimal = Otomatis (desimal)

# Unit suffixes shown next to number inputs.
unit-days = hari
unit-b = B
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-tb = TB

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = Segera
opt-lazy-default = Malas (bawaan)
opt-on = Nyala
opt-on-default = Nyala (bawaan)
opt-all = Semua
opt-weekly = Mingguan
opt-monthly = Bulanan
opt-name-asc = Nama menaik
opt-name-desc = Nama menurun
opt-size-asc = Ukuran menaik
opt-size-desc = Ukuran menurun
opt-modified-asc = Tanggal diubah menaik
opt-modified-desc = Tanggal diubah menurun
opt-compact = Padat
opt-comfortable = Nyaman
opt-details = Detail
opt-thumbnails = Gambar Mini
opt-local-db-default = Basis data lokal (bawaan)
opt-file-list = Daftar berkas
opt-https-endpoint = Endpoint API HTTPS
opt-right-default = Kanan (bawaan)
opt-bottom = Bawah
opt-or-and-default = OR > AND (bawaan)
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128 (bawaan)
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = Broadcast film (-23)
opt-true-peak = Puncak sejati (oversampling 4×, bawaan)
opt-sample-peak = Puncak sampel
opt-auto-per-doc = Otomatis (per-dokumen)
opt-log-error = Error
opt-log-warn = Warn
opt-log-info-default = Info (bawaan)
opt-log-debug = Debug
opt-log-trace = Trace

# More Freally apps (Central inside panel) — host chrome
menu-help-more-apps = Aplikasi Freally lainnya…
moreapps-title = Aplikasi Freally lainnya
