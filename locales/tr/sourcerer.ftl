# Sourcerer — Türkçe.

app-name = Sourcerer
tagline = Tek arama. Her kaynak. Her işletim sistemi.
window-title = Sourcerer
search-placeholder = Ara…
about-version = Sürüm { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = Hazır
status-indexed = Dizine eklendi ({ $count } dosya)
status-indexing = Dizine ekleniyor… { $done }/{ $total }
status-paused = Duraklatıldı
status-error = Hata
status-result-count-one = { $count } sonuç
status-result-count-many = { $count } sonuç
status-selection = · { $count } seçili
status-selection-size = Seçilen: { $size }
status-query-timing = Sorgu: { $ms } ms
status-endpoint-local = Yerel veritabanı
status-endpoint-remote = API: { $name }

menu-file = Dosya
menu-edit = Düzen
menu-view = Görünüm
menu-search = Arama
menu-bookmarks = Yer İmleri
menu-tools = Araçlar
menu-help = Yardım

theme-system = Sistem
theme-light = Açık
theme-dark = Koyu

lens-filename = Dosya Adı
lens-content = İçerik
lens-audio = Ses
lens-similarity = Benzerlik

parse-error-empty = Başlamak için bir sorgu yazın.
parse-error-unknown = Bu noktada tanınmayan söz dizimi.

action-open = Aç
action-reveal = Klasörde göster
action-copy-path = Yolu kopyala
action-copy-name = Adı kopyala
action-delete = Sil

quick-filter-audio = Ses
quick-filter-video = Video
quick-filter-image = Görüntü
quick-filter-document = Belge
quick-filter-executable = Yürütülebilir
quick-filter-archive = Arşiv

wizard-title = Sourcerer'a Hoş Geldiniz
wizard-step-roots = Dizine eklenecekleri seçin
wizard-step-hotkey = Genel kısayol tuşunu seçin
wizard-step-locale = Dilinizi seçin
wizard-step-theme = Tema seçin
wizard-finish = Bitir

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = Seçenekler
settings-search-placeholder = Seçeneklerde ara…
settings-restore-defaults = Varsayılanları Geri Yükle
settings-ok = Tamam
settings-cancel = İptal
settings-apply = Uygula

# Tree nav groups (PRD §8.1.1).
settings-group-general = Genel
settings-group-indexes = Dizinler
settings-group-lenses = Mercekler
settings-group-network = Ağ

# Tree nav leaves.
settings-node-ui = Arayüz
settings-node-home = Başlangıç
settings-node-search = Arama
settings-node-results = Sonuçlar
settings-node-view = Görünüm
settings-node-context-menu = Bağlam Menüsü
settings-node-fonts-colors = Yazı Tipleri ve Renkler
settings-node-keyboard = Klavye
settings-node-history = Geçmiş
settings-node-indexes-top = (üst düzey)
settings-node-volumes = Birimler
settings-node-folders = Klasörler
settings-node-file-lists = Dosya Listeleri
settings-node-exclude = Hariç Tut
settings-node-https-server = HTTP / HTTPS Sunucusu
settings-node-etp-api = ETP / FTP API
settings-node-privacy = Gizlilik ve Güncellemeler
settings-node-logs = Günlükler ve Hata Ayıklama
settings-node-backup = Yedekleme, Dışa Aktarma, Sıfırlama
settings-node-locale = Yerel Ayar
settings-node-about = Hakkında

# §8.2 General → UI.
settings-ui-theme = Tema
settings-ui-run-bg = Arka planda çalıştır
settings-ui-show-tray = Sistem tepsisi / menü çubuğu simgesini göster
settings-ui-single-click-tray = Sistem tepsisine / menü çubuğuna tek tıkla
settings-ui-new-window-from-tray = Tepsi simgesinden yeni pencere aç
settings-ui-new-window-on-launch = Sourcerer başlatıldığında yeni pencere aç
settings-ui-search-as-you-type = Yazarken ara
settings-ui-select-on-mouse-click = Fareyle tıklayınca aramayı seç
settings-ui-focus-on-activate = Etkinleştirildiğinde aramayı odakla
settings-ui-full-row-select = Tüm satırı seç
settings-ui-single-click-open = Tek tıkla aç
settings-ui-underline-titles = Simge başlıklarının altını çiz
settings-ui-row-density = Sonuç yoğunluğu
settings-ui-row-density-compact = Sıkışık (32 piksel)
settings-ui-row-density-comfortable = Rahat (44 piksel)
settings-ui-show-timing-badges = Mercek başına süre rozetlerini göster
settings-ui-anim-crossfade = Animasyonlu tema geçişi

# §8.3 General → Home.
settings-home-match-case = Büyük/küçük harf eşleştir
settings-home-match-whole-word = Tam sözcük eşleştir
settings-home-match-path = Yolu eşleştir
settings-home-match-diacritics = Aksanları eşleştir
settings-home-match-regex = Regex eşleştir
settings-home-search = Arama (özel varsayılan sorgu)
settings-home-filter = Filtre
settings-home-sort = Sıralama
settings-home-view = Görünüm
settings-home-index = Dizin
settings-home-default-lens-visibility = Varsayılan mercek görünürlüğü
settings-home-default-lens-result-limits = Varsayılan mercek sonuç sınırları

# §8.4 General → Search.
settings-search-fast-ascii = Hızlı ASCII araması
settings-search-mp-sep = Arama terimi yol ayırıcı içerdiğinde yolu eşleştir
settings-search-mw-fn = Joker karakter kullanırken tam dosya adını eşleştir
settings-search-lit-ops = Düz metin operatörlerine izin ver
settings-search-paren = Yuvarlak parantez ile gruplandırmaya izin ver
settings-search-env = Ortam değişkenlerini genişlet
settings-search-fwd-slash = Eğik çizgileri ters eğik çizgilerle değiştir
settings-search-precedence = Operatör önceliği
settings-search-strict-everything = Sıkı Everything söz dizimi modu
settings-search-auto-regex = Regex'i otomatik algıla
settings-search-mod-comp = Niteleyici tamamlamaları
settings-search-parse-tree = Üzerine gelince ayrıştırma ağacını göster

# §8.5 General → Results.
settings-results-hide-empty = Arama boş olduğunda sonuçları gizle
settings-results-clear-on-search = Aramada seçimi temizle
settings-results-close-on-execute = Yürütme sırasında pencereyi kapat
settings-results-dbl-path = Yol sütununda çift tıklayarak yolu aç
settings-results-auto-scroll = Görünümü otomatik kaydır
settings-results-dquote-copy = Çift tırnaklı yol olarak kopyala
settings-results-no-ext-rename = Yeniden adlandırırken uzantıyı seçme
settings-results-sort-date-desc = Tarihi önce azalan sırada sırala
settings-results-sort-size-desc = Boyutu önce azalan sırada sırala
settings-results-list-focus = Sonuç listesi odağı
settings-results-icon-prio = Simge yükleme önceliği
settings-results-thumb-prio = Küçük resim yükleme önceliği
settings-results-ext-prio = Genişletilmiş bilgi yükleme önceliği
settings-results-group-by-lens = Sonuçları merceğe göre grupla
settings-results-snippet-inline = Satır içi parçacık önizlemesi göster

# §8.6 General → View.
settings-view-double-buffer = Çift tamponlama
settings-view-alt-rows = Alternatif satır rengi
settings-view-row-mouseover = Satır vurgusunu göster
settings-view-highlight-terms = Vurgulanmış arama terimlerini göster
settings-view-status-show-selected = Durum çubuğunda seçilen öğeyi göster
settings-view-rc-with-sel = Sonuç sayısını seçim sayısıyla birlikte göster
settings-view-status-show-size = Durum çubuğunda boyutu göster
settings-view-tooltips = Araç ipuçlarını göster
settings-view-update-on-scroll = Kaydırmadan hemen sonra ekranı güncelle
settings-view-size-format = Boyut biçimi
settings-view-selection-rect = Seçim dikdörtgeni
settings-view-audio-badges = Ses satırlarında LUFS / codec / uzunluk rozetlerini göster
settings-view-similarity-score = Benzerlik satırlarında MinHash benzerlik skorunu göster
settings-view-preview-pane = Önizleme bölmesi

# §8.7 General → Context Menu.
settings-context-menu-visibility = Görünürlük
settings-context-menu-show = Göster
settings-context-menu-shift = Yalnızca Shift basılıyken göster
settings-context-menu-hide = Gizle
settings-context-menu-command = Komut makrosu
settings-context-menu-open-folders = Aç (Klasörler)
settings-context-menu-open-files = Aç (Dosyalar)
settings-context-menu-open-path = Yolu Aç
settings-context-menu-explore = Keşfet
settings-context-menu-explore-path = Yolu Keşfet
settings-context-menu-copy-name = Adı Panoya Kopyala
settings-context-menu-copy-path = Yolu Panoya Kopyala
settings-context-menu-copy-full-name = Tam Adı Panoya Kopyala
settings-context-menu-reveal = Sourcerer'da Göster
settings-context-menu-send-to = Sourcerer'a Gönder (yol)

# §8.8 General → Fonts & Colors.
settings-fc-font = Yazı Tipi
settings-fc-size = Boyut
settings-fc-state-normal = Normal
settings-fc-state-highlighted = Vurgulanmış
settings-fc-state-current-sort = Mevcut Sıralama
settings-fc-state-current-sort-h = Mevcut Sıralama (Vurgulanmış)
settings-fc-state-selected = Seçili
settings-fc-state-selected-h = Seçili (Vurgulanmış)
settings-fc-state-inactive-selected = Etkin Olmayan Seçili
settings-fc-state-inactive-selected-h = Etkin Olmayan Seçili (Vurgulanmış)
settings-fc-foreground = Ön Plan
settings-fc-background = Arka Plan
settings-fc-bold = Kalın
settings-fc-italic = İtalik
settings-fc-default = Varsayılan
settings-fc-per-lens-accent = Mercek Başına Vurgu
settings-fc-theme-inherit = Tema değişiminde özel renkleri otomatik çevir

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Genel Kısayol Tuşu
settings-keyboard-new-window = Yeni pencere kısayol tuşu
settings-keyboard-show-window = Pencereyi göster kısayol tuşu
settings-keyboard-toggle-window = Pencereyi aç/kapat kısayol tuşu
settings-keyboard-show-commands = Şunu içeren komutları göster
settings-keyboard-add-chord = + Kombinasyon ekle
settings-keyboard-remove-chord = Kaldır

# §8.10 History.
settings-history-search-enable = Arama geçmişini etkinleştir
settings-history-search-keep = Arama geçmişini { $days } gün boyunca sakla
settings-history-run-enable = Çalıştırma geçmişini etkinleştir
settings-history-run-keep = Çalıştırma geçmişini { $days } gün boyunca sakla
settings-history-clear-now = Şimdi Temizle
settings-history-privacy-mode = Gizlilik modu
settings-history-per-lens = Mercek başına geçmiş

# §8.11 Indexes (top-level).
settings-ix-database-location = Veritabanı konumu
settings-ix-multiuser = Çok kullanıcılı veritabanı dosya adı
settings-ix-compress = Veritabanını sıkıştır
settings-ix-recent-changes = Son değişiklikleri dizine ekle
settings-ix-file-size = Dosya boyutunu dizine ekle
settings-ix-fast-size-sort = Hızlı boyut sıralaması
settings-ix-folder-size = Klasör boyutunu dizine ekle
settings-ix-fast-folder-size-sort = Hızlı klasör boyutu sıralaması
settings-ix-date-created = Oluşturulma tarihini dizine ekle
settings-ix-fast-date-created = Hızlı oluşturulma tarihi sıralaması
settings-ix-date-modified = Değiştirilme tarihini dizine ekle
settings-ix-fast-date-modified = Hızlı değiştirilme tarihi sıralaması
settings-ix-date-accessed = Erişim tarihini dizine ekle
settings-ix-fast-date-accessed = Hızlı erişim tarihi sıralaması
settings-ix-attributes = Öznitelikleri dizine ekle
settings-ix-fast-attributes = Hızlı öznitelik sıralaması
settings-ix-fast-path-sort = Hızlı yol sıralaması
settings-ix-fast-extension-sort = Hızlı uzantı sıralaması
settings-ix-force-rebuild = Yeniden Oluşturmaya Zorla
settings-ix-compact = Dizini Sıkıştır
settings-ix-verify = Dizini Doğrula
settings-ix-integrity-policy = Dizin bütünlüğü politikası
settings-ix-memory-budget = Dizinleyici için bellek bütçesi
settings-ix-throttle = Arka plan dizinleme kısıtlaması

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Yeni sabit birimleri otomatik dahil et
settings-vol-auto-removable = Yeni çıkarılabilir birimleri otomatik dahil et
settings-vol-auto-remove-offline = Çevrimdışı birimleri otomatik kaldır
settings-vol-detected = Algılanan birimler
settings-vol-include = Dizine dahil et
settings-vol-include-only = Yalnızca dahil et (glob/regex)
settings-vol-enable-usn = USN Journal'ı etkinleştir
settings-vol-enable-fsevents = FSEvents akışını etkinleştir
settings-vol-enable-inotify = inotify'ı etkinleştir (yetki yükseltilmişse fanotify)
settings-vol-buffer = Journal tampon boyutu (KB)
settings-vol-allocation-delta = Tahsis farkı (KB)
settings-vol-load-recent = Başlangıçta journal'dan son değişiklikleri yükle
settings-vol-monitor = Değişiklikleri izle
settings-vol-recreate-journal = Journal'ı yeniden oluştur
settings-vol-reset-stream = FSEvents akışını sıfırla
settings-vol-upgrade-fanotify = fanotify'a yükselt (polkit)
settings-vol-remove = Kaldır

# §8.13 Indexes → Folders.
settings-folders-watched = İzlenen klasörler
settings-folders-add = Ekle…
settings-folders-rescan-now = Şimdi Yeniden Tara
settings-folders-rescan-all = Tümünü Şimdi Yeniden Tara
settings-folders-monitor = Değişiklikleri izlemeyi dene
settings-folders-buffer = Tampon boyutu
settings-folders-rescan-on-full = Tampon dolduğunda yeniden tara

# §8.14 Indexes → File Lists.
settings-flists-add = Ekle…
settings-flists-monitor = Değişiklikleri izle
settings-flists-editor = Dosya Listesi Düzenleyicisi…
settings-flists-format = Dosya listesi biçimi
settings-flists-format-text = Metin (her satıra bir yol)
settings-flists-format-json = JSON (meta verilerle)
settings-flists-format-srcb = Sourcerer Paketi (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Gizli dosya ve klasörleri hariç tut
settings-exclude-system = Sistem dosya ve klasörlerini hariç tut
settings-exclude-list-en = Hariç tutma listesini etkinleştir
settings-exclude-folders = Klasörleri hariç tut
settings-exclude-include-only-files = Yalnızca dosyaları dahil et (glob)
settings-exclude-files = Dosyaları hariç tut (glob)
settings-exclude-os-recommended = İşletim sisteminin önerdiği hariç tutmaları uygula
settings-exclude-by-class = Uzantı sınıfına göre hariç tut

# §8.16 Lenses → Filename.
settings-lf-trigram = Trigram ön filtre agresifliği
settings-lf-suffix-mem = Sonek dizisi bellek bütçesi
settings-lf-wildcard-limit = Joker karakter genişletme sınırı
settings-lf-regex-timeout = Regex zaman aşımı

# §8.17 Lenses → Content.
settings-lc-enable = İçerik merceğini etkinleştir
settings-lc-time-budget = Belge başına süre bütçesi
settings-lc-mem-ceiling = Belge başına bellek tavanı
settings-lc-snippet-len = Parçacık uzunluğu
settings-lc-stop-words = Engellenen sözcük dili
settings-lc-re-extract = Ayar değişiminde yeniden çıkar
settings-lc-verify-blobs = Okurken çıkarılan metin blob sağlama toplamlarını doğrula

# §8.18 Lenses → Audio.
settings-la-enable = Ses merceğini etkinleştir
settings-la-lufs-ref = LUFS referans standardı
settings-la-peak-compute = Tepe değerini şununla hesapla
settings-la-silence-thresh = Sessizlik eşiği
settings-la-re-extract-modify = Değiştirme olayında yeniden çıkar

# §8.19 Lenses → Similarity.
settings-ls-enable = Benzerlik merceğini etkinleştir
settings-ls-sig-size = MinHash imza boyutu (k)
settings-ls-bands = LSH bandları
settings-ls-recall = Geri çağırma eşiği
settings-ls-result-cap = Sonuç üst sınırı

# §8.20 Lenses → Custom.
settings-custom-registry = Kayıt
settings-custom-trust = Güven
settings-custom-refresh-hashes = Karmaları yenile

# §8.21-§8.22 Network.
settings-net-https-enable = HTTPS sunucusunu etkinleştir
settings-net-bind = Arayüzlere bağla
settings-net-port = Bağlantı noktasını dinle
settings-net-force-https = HTTPS'ye zorla
settings-net-legacy-auth = Eski HTTP-basic kimlik doğrulaması
settings-net-token-regen = Belirteci yeniden oluştur
settings-net-api-enable = API sunucusunu etkinleştir
settings-net-legacy-ftp = Eski düz FTP/ETP desteği

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Otomatik güncelleme
settings-privacy-prerelease = Ön sürüm kanalı
settings-privacy-network-policy = Ağ çağrıları politikası

# §8.24 Logs & Debug.
settings-logs-level = Günlük düzeyi
settings-logs-location = Günlük dosyası konumu
settings-logs-retention = Günlük saklama süresi
settings-logs-debug-overlay = Hata ayıklama katmanını göster
settings-logs-open-folder = Günlük klasörünü aç
settings-logs-export-bundle = Tanılama paketini dışa aktar

# §8.25 Backup, Export, Reset.
settings-backup-export = Ayarları dışa aktar
settings-backup-import = Ayarları içe aktar
settings-backup-export-bookmarks = Yer imleri paketini dışa aktar
settings-backup-import-bookmarks = Yer imleri paketini içe aktar
settings-backup-reset-all = Tüm ayarları varsayılana sıfırla

# §8.26 Locale.
settings-locale-current = Mevcut yerel ayar
settings-locale-rtl-preview = RTL önizleme
settings-locale-date-format = Tarih biçimi
settings-locale-number-format = Sayı biçimi

# §8.27 About.
settings-about-version = Sourcerer { $version }
settings-about-license = Lisans
settings-about-credits = Katkıda Bulunanlar
settings-about-notices = Açık kaynak bildirimleri
