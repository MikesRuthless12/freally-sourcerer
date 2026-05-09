# Sourcerer — Bahasa Indonesia.

app-name = Sourcerer
tagline = Satu pencarian. Setiap sumber. Setiap OS.
window-title = Sourcerer
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
status-query-timing = Kueri: { $ms } md
status-endpoint-local = Basis Data Lokal
status-endpoint-remote = API: { $name }

menu-file = Berkas
menu-edit = Sunting
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

wizard-title = Selamat Datang di Sourcerer
wizard-step-roots = Pilih apa yang akan diindeks
wizard-step-hotkey = Pilih tombol pintas global
wizard-step-locale = Pilih bahasa Anda
wizard-step-theme = Pilih tema
wizard-finish = Selesai

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = Opsi
settings-search-placeholder = Cari opsi…
settings-restore-defaults = Pulihkan Bawaan
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
settings-node-search = Pencarian
settings-node-results = Hasil
settings-node-view = Tampilan
settings-node-context-menu = Menu Konteks
settings-node-fonts-colors = Fon & Warna
settings-node-keyboard = Papan Ketik
settings-node-history = Riwayat
settings-node-indexes-top = (tingkat-atas)
settings-node-volumes = Volume
settings-node-folders = Folder
settings-node-file-lists = Daftar Berkas
settings-node-exclude = Kecualikan
settings-node-https-server = Server HTTP / HTTPS
settings-node-etp-api = API ETP / FTP
settings-node-privacy = Privasi & Pembaruan
settings-node-logs = Log & Debug
settings-node-backup = Cadangan, Ekspor, Atur Ulang
settings-node-locale = Lokal
settings-node-about = Tentang

# §8.2 General → UI.
settings-ui-theme = Tema
settings-ui-run-bg = Jalankan di latar belakang
settings-ui-show-tray = Tampilkan ikon baki / bilah menu
settings-ui-single-click-tray = Klik tunggal pada baki / bilah menu
settings-ui-new-window-from-tray = Buka jendela baru dari ikon baki
settings-ui-new-window-on-launch = Buka jendela baru saat menjalankan Sourcerer
settings-ui-search-as-you-type = Cari saat mengetik
settings-ui-select-on-mouse-click = Pilih kueri saat klik mouse
settings-ui-focus-on-activate = Fokus ke pencarian saat diaktifkan
settings-ui-full-row-select = Pilih satu baris penuh
settings-ui-single-click-open = Buka dengan klik tunggal
settings-ui-underline-titles = Garisbawahi judul ikon
settings-ui-row-density = Kepadatan hasil
settings-ui-row-density-compact = Padat (32 px)
settings-ui-row-density-comfortable = Nyaman (44 px)
settings-ui-show-timing-badges = Tampilkan lencana waktu per lensa
settings-ui-anim-crossfade = Animasi transisi tema

# §8.3 General → Home.
settings-home-match-case = Cocokkan huruf besar/kecil
settings-home-match-whole-word = Cocokkan kata utuh
settings-home-match-path = Cocokkan path
settings-home-match-diacritics = Cocokkan diakritik
settings-home-match-regex = Cocokkan Regex
settings-home-search = Pencarian (kueri bawaan kustom)
settings-home-filter = Filter
settings-home-sort = Urutkan
settings-home-view = Tampilan
settings-home-index = Indeks
settings-home-default-lens-visibility = Visibilitas lensa bawaan
settings-home-default-lens-result-limits = Batas hasil lensa bawaan

# §8.4 General → Search.
settings-search-fast-ascii = Pencarian ASCII cepat
settings-search-mp-sep = Cocokkan path saat istilah pencarian berisi pemisah path
settings-search-mw-fn = Cocokkan nama berkas utuh saat menggunakan wildcard
settings-search-lit-ops = Izinkan operator literal
settings-search-paren = Izinkan pengelompokan dengan tanda kurung
settings-search-env = Perluas variabel lingkungan
settings-search-fwd-slash = Ganti garis miring depan dengan garis miring belakang
settings-search-precedence = Prioritas operator
settings-search-strict-everything = Mode sintaks Everything ketat
settings-search-auto-regex = Deteksi Regex otomatis
settings-search-mod-comp = Penyelesaian otomatis pemodifikasi
settings-search-parse-tree = Tampilkan parse-tree saat hover

# §8.5 General → Results.
settings-results-hide-empty = Sembunyikan hasil saat pencarian kosong
settings-results-clear-on-search = Bersihkan pilihan saat mencari
settings-results-close-on-execute = Tutup jendela saat dieksekusi
settings-results-dbl-path = Buka path dengan klik ganda di kolom path
settings-results-auto-scroll = Gulir tampilan secara otomatis
settings-results-dquote-copy = Tanda kutip ganda menyalin sebagai path
settings-results-no-ext-rename = Jangan pilih ekstensi saat mengganti nama
settings-results-sort-date-desc = Urutkan tanggal menurun lebih dulu
settings-results-sort-size-desc = Urutkan ukuran menurun lebih dulu
settings-results-list-focus = Fokus daftar hasil
settings-results-icon-prio = Prioritas pemuatan ikon
settings-results-thumb-prio = Prioritas pemuatan miniatur
settings-results-ext-prio = Prioritas pemuatan informasi tambahan
settings-results-group-by-lens = Kelompokkan hasil berdasarkan lensa
settings-results-snippet-inline = Tampilkan pratinjau cuplikan sebaris

# §8.6 General → View.
settings-view-double-buffer = Buffer ganda
settings-view-alt-rows = Warna baris bergantian
settings-view-row-mouseover = Tampilkan sorot baris saat mouse di atasnya
settings-view-highlight-terms = Tampilkan istilah pencarian yang disorot
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
settings-context-menu-reveal = Tampilkan di Sourcerer
settings-context-menu-send-to = Kirim ke Sourcerer (path)

# §8.8 General → Fonts & Colors.
settings-fc-font = Fon
settings-fc-size = Ukuran
settings-fc-state-normal = Normal
settings-fc-state-highlighted = Disorot
settings-fc-state-current-sort = Urutan Saat Ini
settings-fc-state-current-sort-h = Urutan Saat Ini (Disorot)
settings-fc-state-selected = Terpilih
settings-fc-state-selected-h = Terpilih (Disorot)
settings-fc-state-inactive-selected = Terpilih Nonaktif
settings-fc-state-inactive-selected-h = Terpilih Nonaktif (Disorot)
settings-fc-foreground = Latar Depan
settings-fc-background = Latar Belakang
settings-fc-bold = Tebal
settings-fc-italic = Miring
settings-fc-default = Bawaan
settings-fc-per-lens-accent = Aksen Per-Lensa
settings-fc-theme-inherit = Balik warna kustom otomatis saat ganti tema

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Tombol Pintas Global
settings-keyboard-new-window = Tombol Pintas Jendela Baru
settings-keyboard-show-window = Tombol Pintas Tampilkan Jendela
settings-keyboard-toggle-window = Tombol Pintas Alih Jendela
settings-keyboard-show-commands = Tampilkan perintah yang berisi
settings-keyboard-add-chord = + Tambah kombinasi
settings-keyboard-remove-chord = Hapus

# §8.10 History.
settings-history-search-enable = Aktifkan riwayat pencarian
settings-history-search-keep = Simpan riwayat pencarian selama { $days } hari
settings-history-run-enable = Aktifkan riwayat menjalankan
settings-history-run-keep = Simpan riwayat menjalankan selama { $days } hari
settings-history-clear-now = Bersihkan Sekarang
settings-history-privacy-mode = Mode privasi
settings-history-per-lens = Riwayat per-lensa

# §8.11 Indexes (top-level).
settings-ix-database-location = Lokasi basis data
settings-ix-multiuser = Nama berkas basis data multi-pengguna
settings-ix-compress = Kompres basis data
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
settings-vol-auto-removable = Sertakan otomatis volume lepas-pasang baru
settings-vol-auto-remove-offline = Hapus otomatis volume luring
settings-vol-detected = Volume terdeteksi
settings-vol-include = Sertakan dalam indeks
settings-vol-include-only = Hanya sertakan (glob/Regex)
settings-vol-enable-usn = Aktifkan USN Journal
settings-vol-enable-fsevents = Aktifkan aliran FSEvents
settings-vol-enable-inotify = Aktifkan inotify (atau fanotify jika ditingkatkan)
settings-vol-buffer = Ukuran buffer journal (KB)
settings-vol-allocation-delta = Delta alokasi (KB)
settings-vol-load-recent = Muat perubahan terbaru dari journal saat memulai
settings-vol-monitor = Pantau perubahan
settings-vol-recreate-journal = Buat ulang journal
settings-vol-reset-stream = Atur ulang aliran FSEvents
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
settings-flists-format-srcb = Bundel Sourcerer (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Kecualikan berkas dan folder tersembunyi
settings-exclude-system = Kecualikan berkas dan folder sistem
settings-exclude-list-en = Aktifkan daftar pengecualian
settings-exclude-folders = Kecualikan folder
settings-exclude-include-only-files = Hanya sertakan berkas (glob)
settings-exclude-files = Kecualikan berkas (glob)
settings-exclude-os-recommended = Terapkan pengecualian yang direkomendasikan OS
settings-exclude-by-class = Kecualikan berdasarkan kelas ekstensi

# §8.16 Lenses → Filename.
settings-lf-trigram = Agresivitas pra-filter trigram
settings-lf-suffix-mem = Anggaran memori suffix-array
settings-lf-wildcard-limit = Batas perluasan wildcard
settings-lf-regex-timeout = Batas waktu Regex

# §8.17 Lenses → Content.
settings-lc-enable = Aktifkan lensa konten
settings-lc-time-budget = Anggaran waktu per dokumen
settings-lc-mem-ceiling = Batas memori per dokumen
settings-lc-snippet-len = Panjang cuplikan
settings-lc-stop-words = Bahasa stop-words
settings-lc-re-extract = Ekstrak ulang saat pengaturan berubah
settings-lc-verify-blobs = Verifikasi checksum blob teks-terekstrak saat dibaca

# §8.18 Lenses → Audio.
settings-la-enable = Aktifkan lensa audio
settings-la-lufs-ref = Standar referensi LUFS
settings-la-peak-compute = Hitung puncak melalui
settings-la-silence-thresh = Ambang batas keheningan
settings-la-re-extract-modify = Ekstrak ulang saat peristiwa Modify

# §8.19 Lenses → Similarity.
settings-ls-enable = Aktifkan lensa kemiripan
settings-ls-sig-size = Ukuran tanda tangan MinHash (k)
settings-ls-bands = Pita LSH
settings-ls-recall = Ambang batas recall
settings-ls-result-cap = Batas hasil

# §8.20 Lenses → Custom.
settings-custom-registry = Registri
settings-custom-trust = Kepercayaan
settings-custom-refresh-hashes = Segarkan hash

# §8.21-§8.22 Network.
settings-net-https-enable = Aktifkan server HTTPS
settings-net-bind = Ikat ke antarmuka
settings-net-port = Dengarkan pada port
settings-net-force-https = Paksa HTTPS
settings-net-legacy-auth = Autentikasi HTTP-basic warisan
settings-net-token-regen = Buat ulang token
settings-net-api-enable = Aktifkan server API
settings-net-legacy-ftp = Dukungan FTP/ETP polos warisan

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Pembaruan otomatis
settings-privacy-prerelease = Saluran pra-rilis
settings-privacy-network-policy = Kebijakan panggilan jaringan

# §8.24 Logs & Debug.
settings-logs-level = Tingkat log
settings-logs-location = Lokasi berkas log
settings-logs-retention = Retensi log
settings-logs-debug-overlay = Tampilkan hamparan debug
settings-logs-open-folder = Buka folder log
settings-logs-export-bundle = Ekspor bundel diagnostik

# §8.25 Backup, Export, Reset.
settings-backup-export = Ekspor pengaturan
settings-backup-import = Impor pengaturan
settings-backup-export-bookmarks = Ekspor bundel markah
settings-backup-import-bookmarks = Impor bundel markah
settings-backup-reset-all = Atur ulang semua pengaturan ke bawaan

# §8.26 Locale.
settings-locale-current = Lokal saat ini
settings-locale-rtl-preview = Pratinjau RTL
settings-locale-date-format = Format tanggal
settings-locale-number-format = Format angka

# §8.27 About.
settings-about-version = Sourcerer { $version }
settings-about-license = Lisensi
settings-about-credits = Kredit
settings-about-notices = Pemberitahuan sumber-terbuka
