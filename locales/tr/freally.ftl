# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = Tek arama. Her kaynak. Her işletim sistemi.
window-title = Freally Sourcerer
search-placeholder = Ara…
about-version = Sürüm { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = Hazır
status-indexed = Dizine alındı ({ $count } dosya)
status-indexing = Dizine alınıyor… { $done }/{ $total }
status-paused = Duraklatıldı
status-error = Hata
status-result-count-one = { $count } sonuç
status-result-count-many = { $count } sonuç
status-selection = · { $count } seçildi
status-selection-size = Seçilen: { $size }
status-query-timing = Sorgu: { $ms } ms
status-endpoint-local = Yerel VT
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

lens-filename = Dosya adı
lens-content = İçerik
lens-audio = Ses
lens-similarity = Benzerlik

parse-error-empty = Başlamak için bir sorgu yazın.
parse-error-unknown = Buradaki söz dizimi tanınmadı.

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

wizard-title = Freally'a Hoş Geldiniz
wizard-step-roots = Dizine alınacakları seçin
wizard-step-hotkey = Bir genel kısayol seçin
wizard-step-locale = Dilinizi seçin
wizard-step-theme = Bir tema seçin
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
settings-node-home = Giriş
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
settings-ui-show-tray = Tepsi / menü çubuğu simgesini göster
settings-ui-single-click-tray = Tepsiye / menü çubuğuna tek tıklama
settings-ui-new-window-from-tray = Tepsi simgesinden yeni pencere aç
settings-ui-new-window-on-launch = Freally başlatılırken yeni pencere aç
settings-ui-search-as-you-type = Yazarken ara
settings-ui-select-on-mouse-click = Fare tıklamasında aramayı seç
settings-ui-focus-on-activate = Etkinleştirildiğinde aramaya odaklan
settings-ui-full-row-select = Tüm satırı seç
settings-ui-single-click-open = Tek tıklamayla aç
settings-ui-underline-titles = Simge başlıklarının altını çiz
settings-ui-row-density = Sonuç yoğunluğu
settings-ui-row-density-compact = Sıkışık (32 px)
settings-ui-row-density-comfortable = Rahat (44 px)
settings-ui-show-timing-badges = Mercek başına zamanlama rozetlerini göster
settings-ui-anim-crossfade = Animasyonlu tema geçişi

# §8.3 General → Home.
settings-home-match-case = Büyük/küçük harf eşleştir
settings-home-match-whole-word = Tam sözcük eşleştir
settings-home-match-path = Yol eşleştir
settings-home-match-diacritics = Aksan işaretlerini eşleştir
settings-home-match-regex = Düzenli ifade eşleştir
settings-home-search = Arama (özel varsayılan sorgu)
settings-home-filter = Filtre
settings-home-sort = Sırala
settings-home-view = Görünüm
settings-home-index = Dizin
settings-home-default-lens-visibility = Varsayılan mercek görünürlüğü
settings-home-default-lens-result-limits = Varsayılan mercek sonuç sınırları

# §8.4 General → Search.
settings-search-fast-ascii = Hızlı ASCII araması
settings-search-mp-sep = Arama terimi yol ayırıcı içerdiğinde yol eşleştir
settings-search-mw-fn = Joker karakter kullanırken tam dosya adını eşleştir
settings-search-lit-ops = Değişmez işleçlere izin ver
settings-search-paren = Yuvarlak parantezle gruplamaya izin ver
settings-search-env = Ortam değişkenlerini genişlet
settings-search-fwd-slash = Eğik çizgileri ters eğik çizgiyle değiştir
settings-search-precedence = İşleç önceliği
settings-search-strict-everything = Katı Everything söz dizimi modu
settings-search-auto-regex = Düzenli ifadeyi otomatik algıla
settings-search-mod-comp = Değiştirici tamamlamaları
settings-search-parse-tree = Üzerine gelince ayrıştırma ağacını göster

# §8.5 General → Results.
settings-results-hide-empty = Arama boşken sonuçları gizle
settings-results-clear-on-search = Aramada seçimi temizle
settings-results-close-on-execute = Çalıştırınca pencereyi kapat
settings-results-dbl-path = Yol sütununda çift tıklamayla yolu aç
settings-results-auto-scroll = Görünümü otomatik kaydır
settings-results-dquote-copy = Çift tırnaklı kopyayı yol olarak al
settings-results-no-ext-rename = Yeniden adlandırırken uzantıyı seçme
settings-results-sort-date-desc = Önce tarihe göre azalan sırala
settings-results-sort-size-desc = Önce boyuta göre azalan sırala
settings-results-list-focus = Sonuç listesi odağı
settings-results-icon-prio = Simge yükleme önceliği
settings-results-thumb-prio = Küçük resim yükleme önceliği
settings-results-ext-prio = Genişletilmiş bilgi yükleme önceliği
settings-results-group-by-lens = Sonuçları merceğe göre grupla
settings-results-snippet-inline = Satır içinde parça önizlemesi göster

# §8.6 General → View.
settings-view-double-buffer = Çift arabellek
settings-view-alt-rows = Satırları dönüşümlü renklendir
settings-view-row-mouseover = Fare üzerine gelince satırı göster
settings-view-highlight-terms = Vurgulanan arama terimlerini göster
settings-view-status-show-selected = Seçili öğeyi durum çubuğunda göster
settings-view-rc-with-sel = Sonuç sayısını seçim sayısıyla birlikte göster
settings-view-status-show-size = Boyutu durum çubuğunda göster
settings-view-tooltips = İpuçlarını göster
settings-view-update-on-scroll = Kaydırdıktan hemen sonra ekranı güncelle
settings-view-size-format = Boyut biçimi
settings-view-selection-rect = Seçim dikdörtgeni
settings-view-audio-badges = Ses satırlarında LUFS / codec / uzunluk rozetlerini göster
settings-view-similarity-score = Benzerlik satırlarında MinHash benzerlik puanını göster
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
settings-context-menu-reveal = Freally'da Göster
settings-context-menu-send-to = Freally'a Gönder (yol)

# §8.8 General → Fonts & Colors.
settings-fc-font = Yazı Tipi
settings-fc-size = Boyut
settings-fc-state-normal = Normal
settings-fc-state-highlighted = Vurgulanan
settings-fc-state-current-sort = Geçerli Sıralama
settings-fc-state-current-sort-h = Geçerli Sıralama (Vurgulanan)
settings-fc-state-selected = Seçili
settings-fc-state-selected-h = Seçili (Vurgulanan)
settings-fc-state-inactive-selected = Etkin Olmayan Seçili
settings-fc-state-inactive-selected-h = Etkin Olmayan Seçili (Vurgulanan)
settings-fc-foreground = Ön Plan
settings-fc-background = Arka Plan
settings-fc-bold = Kalın
settings-fc-italic = İtalik
settings-fc-default = Varsayılan
settings-fc-per-lens-accent = Mercek Başına Vurgu
settings-fc-theme-inherit = Tema değişiminde özel renkleri otomatik çevir

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Genel Kısayol
settings-keyboard-new-window = Yeni pencere Kısayolu
settings-keyboard-show-window = Pencere göster Kısayolu
settings-keyboard-toggle-window = Pencere aç/kapat Kısayolu
settings-keyboard-show-commands = Şunu içeren komutları göster
settings-keyboard-add-chord = + Akor ekle
settings-keyboard-remove-chord = Kaldır

# §8.10 History.
settings-history-search-enable = Arama geçmişini etkinleştir
settings-history-search-keep = Arama geçmişini { $days } gün sakla
settings-history-run-enable = Çalıştırma geçmişini etkinleştir
settings-history-run-keep = Çalıştırma geçmişini { $days } gün sakla
settings-history-clear-now = Şimdi Temizle
settings-history-privacy-mode = Gizlilik modu
settings-history-per-lens = Mercek başına geçmiş

# §8.11 Indexes (top-level).
settings-ix-database-location = Veritabanı konumu
settings-ix-multiuser = Çok kullanıcılı veritabanı dosya adı
settings-ix-compress = Veritabanını sıkıştır
settings-ix-recent-changes = Son değişiklikleri dizine al
settings-ix-file-size = Dosya boyutunu dizine al
settings-ix-fast-size-sort = Hızlı boyut sıralaması
settings-ix-folder-size = Klasör boyutunu dizine al
settings-ix-fast-folder-size-sort = Hızlı klasör boyutu sıralaması
settings-ix-date-created = Oluşturulma tarihini dizine al
settings-ix-fast-date-created = Hızlı oluşturulma tarihi sıralaması
settings-ix-date-modified = Değiştirilme tarihini dizine al
settings-ix-fast-date-modified = Hızlı değiştirilme tarihi sıralaması
settings-ix-date-accessed = Erişilme tarihini dizine al
settings-ix-fast-date-accessed = Hızlı erişilme tarihi sıralaması
settings-ix-attributes = Öznitelikleri dizine al
settings-ix-fast-attributes = Hızlı öznitelik sıralaması
settings-ix-fast-path-sort = Hızlı yol sıralaması
settings-ix-fast-extension-sort = Hızlı uzantı sıralaması
settings-ix-force-rebuild = Yeniden Oluşturmaya Zorla
settings-ix-compact = Dizini Sıkıştır
settings-ix-verify = Dizini Doğrula
settings-ix-integrity-policy = Dizin bütünlüğü ilkesi
settings-ix-memory-budget = Dizinleyici için bellek bütçesi
settings-ix-throttle = Arka plan dizinleme kısıtlaması

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Yeni sabit birimleri otomatik dahil et
settings-vol-auto-removable = Yeni çıkarılabilir birimleri otomatik dahil et
settings-vol-auto-remove-offline = Çevrimdışı birimleri otomatik kaldır
settings-vol-detected = Algılanan birimler
settings-vol-include = Dizine dahil et
settings-vol-include-only = Yalnızca dahil et (glob/regex)
settings-vol-enable-usn = USN Günlüğünü etkinleştir
settings-vol-enable-fsevents = FSEvents akışını etkinleştir
settings-vol-enable-inotify = inotify'ı etkinleştir (yetki varsa fanotify)
settings-vol-buffer = Günlük arabellek boyutu (KB)
settings-vol-allocation-delta = Ayırma farkı (KB)
settings-vol-load-recent = Başlangıçta günlükten son değişiklikleri yükle
settings-vol-monitor = Değişiklikleri izle
settings-vol-recreate-journal = Günlüğü yeniden oluştur
settings-vol-reset-stream = FSEvents akışını sıfırla
settings-vol-upgrade-fanotify = fanotify'a yükselt (polkit)
settings-vol-remove = Kaldır

# §8.13 Indexes → Folders.
settings-folders-watched = İzlenen klasörler
settings-folders-add = Ekle…
settings-folders-rescan-now = Şimdi Yeniden Tara
settings-folders-rescan-all = Tümünü Şimdi Yeniden Tara
settings-folders-monitor = Değişiklikleri izlemeyi dene
settings-folders-buffer = Arabellek boyutu
settings-folders-rescan-on-full = Arabellek dolunca yeniden tara

# §8.14 Indexes → File Lists.
settings-flists-add = Ekle…
settings-flists-monitor = Değişiklikleri izle
settings-flists-editor = Dosya Listesi Düzenleyicisi…
settings-flists-format = Dosya listesi biçimi
settings-flists-format-text = Metin (satır başına bir yol)
settings-flists-format-json = JSON (üst veriyle)
settings-flists-format-srcb = Freally Paketi (.srcb)

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
settings-lf-trigram = Üçlü gram ön filtre saldırganlığı
settings-lf-suffix-mem = Sonek dizisi bellek bütçesi
settings-lf-wildcard-limit = Joker karakter genişletme sınırı
settings-lf-regex-timeout = Düzenli ifade zaman aşımı

# §8.17 Lenses → Content.
settings-lc-enable = İçerik merceğini etkinleştir
settings-lc-time-budget = Belge başına süre bütçesi
settings-lc-mem-ceiling = Belge başına bellek üst sınırı
settings-lc-snippet-len = Parça uzunluğu
settings-lc-stop-words = Durdurma sözcükleri dili
settings-lc-re-extract = Ayar değişikliğinde yeniden ayıkla
settings-lc-verify-blobs = Okumada ayıklanan metin blob sağlamalarını doğrula

# §8.18 Lenses → Audio.
settings-la-enable = Ses merceğini etkinleştir
settings-la-lufs-ref = LUFS referans standardı
settings-la-peak-compute = Tepe değerini şununla hesapla
settings-la-silence-thresh = Sessizlik eşiği
settings-la-re-extract-modify = Değiştirme olayında yeniden ayıkla

# §8.19 Lenses → Similarity.
settings-ls-enable = Benzerlik merceğini etkinleştir
settings-ls-sig-size = MinHash imza boyutu (k)
settings-ls-bands = LSH bantları
settings-ls-recall = Geri çağırma eşiği
settings-ls-result-cap = Sonuç üst sınırı

# §8.20 Lenses → Custom.
settings-custom-registry = Kayıt Defteri
settings-custom-trust = Güven
settings-custom-refresh-hashes = Karmaları yenile

# §8.21-§8.22 Network.
settings-net-https-enable = HTTPS sunucusunu etkinleştir
settings-net-bind = Arayüzlere bağlan
settings-net-port = Bağlantı noktasını dinle
settings-net-force-https = HTTPS'yi zorunlu kıl
settings-net-legacy-auth = Eski HTTP-basic kimlik doğrulaması
settings-net-token-regen = Belirteci yeniden oluştur
settings-net-api-enable = API sunucusunu etkinleştir
settings-net-legacy-ftp = Eski düz FTP/ETP desteği

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Otomatik güncelleme
settings-privacy-prerelease = Ön sürüm kanalı
settings-privacy-network-policy = Ağ çağrıları ilkesi

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
settings-backup-export-bookmarks = Yer imi paketini dışa aktar
settings-backup-import-bookmarks = Yer imi paketini içe aktar
settings-backup-reset-all = Tüm ayarları varsayılanlara sıfırla

# §8.26 Locale.
settings-locale-current = Geçerli yerel ayar
settings-locale-rtl-preview = RTL önizlemesi
settings-locale-date-format = Tarih biçimi
settings-locale-number-format = Sayı biçimi

# §8.27 About.
settings-about-version = Freally { $version }
settings-about-license = Lisans
settings-about-credits = Katkıda Bulunanlar
settings-about-notices = Açık kaynak bildirimleri

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = İlk çalıştırma sihirbazı
wizard-step-of-total = Adım { $step } / { $total }
wizard-roots-hint = Freally'ın izlemesini istediğiniz klasörleri veya birimleri ekleyin. Bunu daha sonra Dizinler ayarlarından değiştirebilirsiniz.
wizard-browse = Gözat…
wizard-roots-placeholder = …veya bir yol yapıştırın
wizard-roots-add = Ekle
wizard-roots-remove = Kaldır
wizard-roots-empty = Henüz yapılandırılmış kök yok.
wizard-locale-hint = Freally 18 dilde sunulur. Daha sonra değiştirebilirsiniz.
wizard-theme-hint = Sistem, işletim sisteminizin görünüm ayarını izler.
wizard-back = Geri
wizard-next = İleri

# Status bar polish.
statusbar-hotkey-hint = Kısayol: { $hotkey }
statusbar-cycle-theme = Temayı değiştir
statusbar-indexed-suffix = dizine alındı

# Results / lenses.
lens-expand = Merceği genişlet
lens-collapse = Merceği daralt
lens-no-matches = Bu mercekte eşleşme yok.

# Preview pane.
preview-header = Önizleme
preview-loading = Yükleniyor…
preview-select-file = Önizlemek için bir dosya seçin.
preview-unavailable = Önizleme yok

# Bookmarks.
bookmarks-label = ★ Yer İmleri
bookmarks-empty-hint = Henüz yer imi yok. Geçerli sorguyu kaydetmek için Ctrl+D'ye basın.
bookmarks-organize-title = Yer İmlerini Düzenle
bookmarks-organize-empty = Henüz yer imi yok.
bookmarks-rename = Yeniden Adlandır
bookmarks-close = Kapat

# Settings tree extras.
settings-group-history = Geçmiş
settings-group-privacy = Gizlilik ve Güncellemeler
settings-group-logs = Günlükler ve Hata Ayıklama
settings-group-backup = Yedekleme, Dışa Aktarma, Sıfırlama
settings-tree-custom-lens = Özel
settings-unsaved-changes = kaydedilmemiş değişiklikler

# About dialog.
about-dialog-title = Freally
about-copyright = Telif Hakkı © 2026 Mike Weaver. Tüm hakları saklıdır.
about-close = Kapat

# Connect endpoint dialog.
connect-ftp-title = FTP Sunucusuna Bağlan
connect-ftp-host = Ana bilgisayar:
connect-ftp-port = Bağlantı noktası:
connect-ftp-username = Kullanıcı adı:
connect-ftp-password = Parola:
connect-ftp-link-type = Bağlantı türü:

# UI panel.
ui-hint = Tema, tepsi / menü çubuğu tümleştirmesi, yazarken arama, satır yoğunluğu. Doğrudan voidtools-Everything denkliği artı (+) ile işaretlenmiş Freally eklentileri.
ui-section-theme = Tema
ui-theme-system-default = Sistem (varsayılan)
ui-section-tray = Tepsi / Menü Çubuğu
ui-section-search-behavior = Arama Davranışı
ui-section-result-rows = Sonuç Satırları
ui-single-click-system-default = Sistem ayarları (varsayılan)
ui-single-click-always = Her zaman tek tıklama
ui-single-click-always-double = Her zaman çift tıklama
ui-underline-always = Her zaman
ui-underline-on-hover = Üzerine gelince
ui-underline-never = Asla

# Home panel.
home-hint = Uygulama başlangıcında yüklenen varsayılanlar — her açılır menü "Son değeri kullan" seçeneğinde kalabilir veya sabit bir değere sabitlenebilir. Mercek görünürlüğü / sonuç sınırları Freally eklentileridir (+).
home-section-match = Eşleştirme Varsayılanları
home-section-search-sort = Arama ve Sıralama Varsayılanları
home-search-placeholder = Varsayılan olarak boş
home-section-index = Dizin Kaynağı
home-file-list-path = Dosya listesi yolu
home-https-endpoint = HTTPS API uç nokta URL'si
home-endpoint-token = Belirteç (parmak izi gösterilir)

# Backup panel.
backup-section-settings = Ayarlar (+)
backup-section-bookmarks = Yer İmleri + Özel Ayıklayıcılar (+)
backup-section-reset = Sıfırla
backup-toast-exported = Ayarlar { $path } konumuna aktarıldı
backup-toast-export-failed = Dışa aktarma başarısız: { $error }
backup-toast-imported = Ayarlar içe aktarıldı
backup-toast-import-failed = İçe aktarma başarısız: { $error }
backup-toast-bookmarks-exported = Yer imleri dışa aktarıldı
backup-toast-bookmarks-export-failed = Yer imi dışa aktarımı başarısız: { $error }
backup-toast-bookmarks-imported = Yer imleri içe aktarıldı
backup-toast-bookmarks-import-failed = Yer imi içe aktarımı başarısız: { $error }
backup-confirm-reset = Tüm ayarlar varsayılanlara sıfırlansın mı? Bu geri alınamaz (iletişim kutusu açık kalır).
backup-toast-reset = Tüm ayarlar sıfırlandı

# Keyboard panel.
keyboard-section-global = Genel Kısayollar
keyboard-placeholder-example = Super+Space
keyboard-section-commands = Komutlar
keyboard-placeholder-command = komut kimliği (ör. file.export_results)
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = Arama Geçmişi
history-section-run = Çalıştırma Geçmişi
history-section-privacy = Gizlilik (+)
history-record-filename = Dosya adı merceği geçmişini kaydet
history-record-content = İçerik merceği geçmişini kaydet
history-record-audio = Ses merceği geçmişini kaydet
history-record-similarity = Benzerlik merceği geçmişini kaydet

# Locale panel.
locale-section-language = Dil (+)
locale-section-time-date = Saat / Tarih (+)
locale-date-os = İşletim sistemi varsayılanı
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = Özel
locale-date-custom-format = Özel biçim
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = Sayılar (+)
locale-number-os = İşletim sistemi varsayılanı
locale-number-custom = Özel
locale-thousands-sep = Binlik ayırıcı
locale-decimal-sep = Ondalık ayırıcı

# Folders panel.
folders-hint = Varsayılan birimlerin ötesinde ek izlenen klasörler.
folders-list-title = İzlenen klasörler
folders-empty = Henüz klasör eklenmedi.
folders-remove = Kaldır
folders-section-title-dynamic = { $path } için ayarlar
folders-section-schedule = Yeniden tarama zamanlaması
folders-schedule-daily = Her gün SS:DD'de
folders-schedule-hours = Her N saatte bir
folders-schedule-never = Asla
folders-hour = Saat
folders-minute = Dakika
folders-hours = Saat
folders-id-label = Klasör kimliği (salt okunur)
folders-select-prompt = Yapılandırmak için bir klasör seçin.
folders-section-extras = Freally Ekstraları (+)
folders-extras-note = Uykudan devam ederken yeniden tarama bu sürümde varsayılan olarak etkindir; geçiş anahtarı, Faz 13'ün cilalama aşamasında klasör düzeyindeki denetimlere katılacaktır.

# Volumes panel.
volumes-hint = voidtools-Everything'in NTFS / ReFS panellerinin platformlar arası karşılığı. NTFS / ReFS / exFAT / FAT32 (Win), APFS / HFS+ (macOS), ext4 / Btrfs / ZFS / XFS / F2FS (Linux) otomatik algılanır.
volumes-section-auto-include = Otomatik dahil etme
volumes-list-title = Algılanan birimler
volumes-detecting = Algılanıyor…
volumes-empty = Birim algılanmadı.
volumes-select-prompt = Yapılandırmak için bir birim seçin.

# About panel polish.
about-section-version = Sürüm (+)
about-section-license = Lisans (+)
about-license-text = Mike Weaver — Tüm Hakları Saklıdır. Bu, tescilli bir yazılımdır.
about-license-spdx = SPDX: { $spdx }
about-section-credits = Katkıda Bulunanlar (+)
about-credits-inspired = voidtools tarafından geliştirilen Everything'den esinlenilmiştir.
about-credits-voidtools = voidtools.com
about-credits-repo = Proje deposu

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = Freally ile çalışmaya yönelik komutları içerir.
menu-file-new-window = Yeni Arama Penceresi
menu-file-open-list = Dosya Listesi Aç…
menu-file-close-list = Dosya Listesini Kapat
menu-file-close = Kapat
menu-file-export-results = Sonuçları Dışa Aktar…
menu-file-export-bundle = Dizin Paketini Dışa Aktar…
menu-file-exit = Çıkış

# Edit menu.
menu-edit-hint = Arama sonuçlarını düzenlemeye yönelik komutları içerir.
menu-edit-cut = Kes
menu-edit-copy = Kopyala
menu-edit-paste = Yapıştır
menu-edit-copy-to-folder = Klasöre Kopyala…
menu-edit-move-to-folder = Klasöre Taşı…
menu-edit-select-all = Tümünü Seç
menu-edit-invert-selection = Seçimi Ters Çevir
menu-edit-advanced = Gelişmiş
menu-edit-copy-full-name = Tam Adı Kopyala
menu-edit-copy-path = Yolu Kopyala
menu-edit-copy-filename = Dosya Adını Kopyala
menu-edit-copy-as-json = JSON Olarak Kopyala
menu-edit-copy-with-metadata = Üst Veriyle Kopyala
menu-edit-copy-as-bundle-ref = Freally Paketi referansı olarak kopyala

# View menu.
menu-view-hint = Görünümü değiştirmeye yönelik komutları içerir.
menu-view-filters = Filtreler
menu-view-preview = Önizleme
menu-view-status-bar = Durum Çubuğu
menu-view-thumbs-xl = Çok Büyük Küçük Resimler
menu-view-thumbs-l = Büyük Küçük Resimler
menu-view-thumbs-m = Orta Küçük Resimler
menu-view-details = Ayrıntılar
menu-view-window-size = Pencere Boyutu
menu-view-window-size-hint = Pencere boyutunu ayarlamaya yönelik komutları içerir.
menu-view-window-small = Küçük
menu-view-window-medium = Orta
menu-view-window-large = Büyük
menu-view-window-auto = Otomatik Sığdır
menu-view-zoom = Yakınlaştır
menu-view-zoom-hint = Yazı tipi ve simge boyutunu ayarlamaya yönelik komutları içerir.
menu-view-zoom-in = Yakınlaştır
menu-view-zoom-out = Uzaklaştır
menu-view-zoom-reset = Sıfırla
menu-view-sort-by = Sıralama ölçütü
menu-view-sort-by-hint = Sonuç listesini sıralamaya yönelik komutları içerir.
menu-view-sort-name = Ad
menu-view-sort-path = Yol
menu-view-sort-size = Boyut
menu-view-sort-ext = Uzantı
menu-view-sort-type = Tür
menu-view-sort-modified = Değiştirilme Tarihi
menu-view-sort-created = Oluşturulma Tarihi
menu-view-sort-accessed = Erişilme Tarihi
menu-view-sort-attributes = Öznitelikler
menu-view-sort-recently-changed = Son Değiştirilme Tarihi
menu-view-sort-run-count = Çalıştırma Sayısı
menu-view-sort-run-date = Çalıştırılma Tarihi
menu-view-sort-file-list-filename = Dosya Listesi Dosya Adı
menu-view-sort-lufs = LUFS
menu-view-sort-length = Uzunluk
menu-view-sort-similarity = Benzerlik Puanı
menu-view-sort-asc = Artan
menu-view-sort-desc = Azalan
menu-view-go-to = Git
menu-view-refresh = Yenile
menu-view-theme = Tema
menu-view-theme-hint = Sistem, açık veya koyu temalar arasında geçiş yapın.
menu-view-lenses = Mercekler
menu-view-lenses-hint = Sonuç listesindeki her merceğin görünürlüğünü aç/kapat.
menu-view-on-top = Üstte
menu-view-on-top-hint = Bu pencereyi diğer pencerelerin üstünde tutmaya yönelik komutları içerir.
menu-view-on-top-never = Asla
menu-view-on-top-always = Her zaman
menu-view-on-top-while-searching = Arama Yaparken

# Search menu.
menu-search-hint = Arama geçişlerini içerir.
menu-search-match-case = Büyük/Küçük Harf Eşleştir
menu-search-match-whole-word = Tam Sözcük Eşleştir
menu-search-match-path = Yol Eşleştir
menu-search-match-diacritics = Aksan İşaretlerini Eşleştir
menu-search-enable-regex = Düzenli İfadeyi Etkinleştir
menu-search-advanced = Gelişmiş Arama…
menu-search-add-to-filters = Filtrelere Ekle…
menu-search-organize-filters = Filtreleri Düzenle…
menu-search-filter-everything = Everything
menu-search-filter-archive = Sıkıştırılmış (Arşiv)
menu-search-filter-folder = Klasör
menu-search-filter-custom = Özel Filtre…

# Bookmarks menu.
menu-bookmarks-hint = Yer imleriyle çalışmaya yönelik komutları içerir.
menu-bookmarks-add = Yer İmlerine Ekle
menu-bookmarks-organize = Yer İmlerini Düzenle…

# Tools menu.
menu-tools-hint = Araç komutlarını içerir.
menu-tools-connect = FTP Sunucusuna Bağlan…
menu-tools-disconnect = FTP Sunucusu Bağlantısını Kes
menu-tools-file-list-editor = Dosya Listesi Düzenleyicisi…
menu-tools-index-maintenance = Dizin bakımı
menu-tools-index-maintenance-hint = Dizin bakım araçları.
menu-tools-verify-index = Dizini Doğrula…
menu-tools-compact-index = Dizini Sıkıştır…
menu-tools-rebuild-index = Dizini Yeniden Oluşturmaya Zorla…
menu-tools-custom-extractor = Özel Ayıklayıcı Yöneticisi…
menu-tools-custom-extractor-hint = Wasm korumalı alandaki özel ayıklayıcıları yönetin.
menu-tools-options = Seçenekler…

# Help menu.
menu-help-hint = Yardım komutlarını içerir.
menu-help-help = Freally Yardımı
menu-help-search-syntax = Arama Söz Dizimi
menu-help-regex-syntax = Düzenli İfade Söz Dizimi
menu-help-audio-ref = Ses Değiştirici Başvurusu
menu-help-similarity-ref = Benzerlik Değiştirici Başvurusu
menu-help-cli-options = Komut Satırı Seçenekleri
menu-help-website = Freally Web Sitesi
menu-help-check-updates = Güncellemeleri Denetle…
menu-help-sponsor = Sponsor Ol / Bağış Yap
menu-help-about = Freally Hakkında…

# Result column headers (short forms used in the table header row).
column-name = Ad
column-path = Yol
column-size = Boyut
column-modified = Değiştirilme
column-type = Tür
column-ext = Uzantı
column-sort-by = { $name } ölçütüne göre sırala
column-resize = { $name } sütununu yeniden boyutlandır

# Section subtitle bars used inside multiple settings panels.
section-behavior = Davranış
section-rendering = İşleme
section-status-bar = Durum Çubuğu
section-display-format = Görüntüleme Biçimi
section-loading-priority = Yükleme Önceliği
section-compatibility = Uyumluluk
section-storage = Depolama
section-index-fields = Dizin Alanları
section-maintenance = Bakım
section-logging = Günlük Kaydı
section-tools = Araçlar
section-privacy = Gizlilik
section-auto-update = Otomatik güncelleme (+)
section-bind = Bağlama
section-lens = Mercek
section-budgets = Bütçeler
section-other = Diğer
section-per-format-mode = Biçim Başına Mod
section-loudness = Ses Yüksekliği
section-tuning = İnce Ayar (+)
section-minhash-lsh = MinHash + LSH Parametreleri (+)
section-top-level = Üst düzey
section-file-globs = Dosya glob'ları
section-file-list-settings = Seçili dosya listesi için ayarlar
section-editor-format = Düzenleyici + Biçim (E + +)
section-api-server = API Sunucusu (E uyarlanmış)
section-freally-extras = Freally Ekstraları (+)
section-freally-additions = Freally Eklentileri (+)
section-freally-extensions = Freally Uzantıları (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = Son değeri kullan
opt-use-last-value-default = Son değeri kullan (varsayılan)
opt-low = Düşük
opt-normal-default = Normal (varsayılan)
opt-high = Yüksek
opt-disabled = Devre dışı
opt-off = Kapalı
opt-on-battery = Pilde çalışırken
opt-always = Her zaman
opt-clamp-default = Sınırla (varsayılan)
opt-wrap = Kaydır
opt-none = Yok
opt-strict-refuse = Katı (bozulmada sorguları reddet)
opt-lenient-warn = Hoşgörülü (uyar ama sorgula)
opt-system-default = Sistem varsayılanı
opt-drag-select = Sürükleyerek seç
opt-auto-binary = Otomatik (ikili)
opt-auto-decimal = Otomatik (ondalık)

# Unit suffixes shown next to number inputs.
unit-days = gün
unit-b = B
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-tb = TB

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = Erken
opt-lazy-default = Tembel (varsayılan)
opt-on = Açık
opt-on-default = Açık (varsayılan)
opt-all = Tümü
opt-weekly = Haftalık
opt-monthly = Aylık
opt-name-asc = Ad artan
opt-name-desc = Ad azalan
opt-size-asc = Boyut artan
opt-size-desc = Boyut azalan
opt-modified-asc = Değiştirilme tarihi artan
opt-modified-desc = Değiştirilme tarihi azalan
opt-compact = Sıkışık
opt-comfortable = Rahat
opt-details = Ayrıntılar
opt-thumbnails = Küçük resimler
opt-local-db-default = Yerel veritabanı (varsayılan)
opt-file-list = Dosya listesi
opt-https-endpoint = HTTPS API uç noktası
opt-right-default = Sağ (varsayılan)
opt-bottom = Alt
opt-or-and-default = OR > AND (varsayılan)
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128 (varsayılan)
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = Yayın filmi (-23)
opt-true-peak = Gerçek tepe (4× aşırı örnekleme, varsayılan)
opt-sample-peak = Örnek tepe
opt-auto-per-doc = Otomatik (belge başına)
opt-log-error = Hata
opt-log-warn = Uyarı
opt-log-info-default = Bilgi (varsayılan)
opt-log-debug = Hata Ayıklama
opt-log-trace = İzleme
