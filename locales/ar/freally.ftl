# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = بحث واحد. كل المصادر. كل أنظمة التشغيل.
window-title = Freally Sourcerer
search-placeholder = بحث…
about-version = الإصدار { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = جاهز
status-indexed = تمت الفهرسة ({ $count } ملف)
status-indexing = جارٍ الفهرسة… { $done }/{ $total }
status-paused = متوقّف مؤقتًا
status-error = خطأ
status-result-count-one = نتيجة واحدة ({ $count })
status-result-count-many = { $count } نتيجة
status-selection = · { $count } محدّد
status-selection-size = المحدّد: { $size }
status-query-timing = الاستعلام: { $ms } مللي ثانية
status-endpoint-local = قاعدة بيانات محلية
status-endpoint-remote = واجهة API: { $name }

menu-file = ملف
menu-edit = تحرير
menu-view = عرض
menu-search = بحث
menu-bookmarks = الإشارات المرجعية
menu-tools = أدوات
menu-help = مساعدة

theme-system = النظام
theme-light = فاتح
theme-dark = داكن

lens-filename = اسم الملف
lens-content = المحتوى
lens-audio = الصوت
lens-similarity = التشابه

parse-error-empty = اكتب استعلامًا للبدء.
parse-error-unknown = صيغة غير معروفة قرب هذا الموضع.

action-open = فتح
action-reveal = إظهار في المجلد
action-copy-path = نسخ المسار
action-copy-name = نسخ الاسم
action-delete = حذف

quick-filter-audio = صوت
quick-filter-video = فيديو
quick-filter-image = صورة
quick-filter-document = مستند
quick-filter-executable = ملف تنفيذي
quick-filter-archive = أرشيف

wizard-title = مرحبًا بك في Freally
wizard-step-roots = اختر ما تريد فهرسته
wizard-step-hotkey = اختر مفتاح اختصار عامًّا
wizard-step-locale = اختر لغتك
wizard-step-theme = اختر سمة
wizard-finish = إنهاء

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = الخيارات
settings-search-placeholder = البحث في الخيارات…
settings-restore-defaults = استعادة الإعدادات الافتراضية
settings-ok = موافق
settings-cancel = إلغاء
settings-apply = تطبيق

# Tree nav groups (PRD §8.1.1).
settings-group-general = عام
settings-group-indexes = الفهارس
settings-group-lenses = العدسات
settings-group-network = الشبكة

# Tree nav leaves.
settings-node-ui = الواجهة
settings-node-home = الرئيسية
settings-node-search = البحث
settings-node-results = النتائج
settings-node-view = العرض
settings-node-context-menu = قائمة السياق
settings-node-fonts-colors = الخطوط والألوان
settings-node-keyboard = لوحة المفاتيح
settings-node-history = السجل
settings-node-indexes-top = (المستوى الأعلى)
settings-node-volumes = وحدات التخزين
settings-node-folders = المجلدات
settings-node-file-lists = قوائم الملفات
settings-node-exclude = الاستبعاد
settings-node-https-server = خادم HTTP / HTTPS
settings-node-etp-api = واجهة ETP / FTP
settings-node-privacy = الخصوصية والتحديثات
settings-node-logs = السجلات والتصحيح
settings-node-backup = النسخ الاحتياطي والتصدير وإعادة التعيين
settings-node-locale = اللغة والمنطقة
settings-node-about = حول

# §8.2 General → UI.
settings-ui-theme = السمة
settings-ui-run-bg = التشغيل في الخلفية
settings-ui-show-tray = إظهار أيقونة شريط المهام / شريط القوائم
settings-ui-single-click-tray = نقرة واحدة على شريط المهام / شريط القوائم
settings-ui-new-window-from-tray = فتح نافذة جديدة من أيقونة شريط المهام
settings-ui-new-window-on-launch = فتح نافذة جديدة عند تشغيل Freally
settings-ui-search-as-you-type = البحث أثناء الكتابة
settings-ui-select-on-mouse-click = تحديد البحث عند النقر بالفأرة
settings-ui-focus-on-activate = التركيز على البحث عند التنشيط
settings-ui-full-row-select = تحديد الصف بالكامل
settings-ui-single-click-open = الفتح بنقرة واحدة
settings-ui-underline-titles = تسطير عناوين الأيقونات
settings-ui-row-density = كثافة النتائج
settings-ui-row-density-compact = مضغوط (32 بكسل)
settings-ui-row-density-comfortable = مريح (44 بكسل)
settings-ui-show-timing-badges = إظهار شارات التوقيت لكل عدسة
settings-ui-anim-crossfade = تلاشٍ متحرّك بين السمات

# §8.3 General → Home.
settings-home-match-case = مطابقة حالة الأحرف
settings-home-match-whole-word = مطابقة الكلمة بالكامل
settings-home-match-path = مطابقة المسار
settings-home-match-diacritics = مطابقة التشكيل
settings-home-match-regex = مطابقة التعبير النمطي
settings-home-search = البحث (استعلام افتراضي مخصّص)
settings-home-filter = التصفية
settings-home-sort = الترتيب
settings-home-view = العرض
settings-home-index = الفهرس
settings-home-default-lens-visibility = الظهور الافتراضي للعدسات
settings-home-default-lens-result-limits = حدود النتائج الافتراضية للعدسات

# §8.4 General → Search.
settings-search-fast-ascii = بحث ASCII سريع
settings-search-mp-sep = مطابقة المسار عند احتواء مصطلح البحث على فاصل مسار
settings-search-mw-fn = مطابقة اسم الملف بالكامل عند استخدام أحرف البدل
settings-search-lit-ops = السماح بالعوامل الحرفية
settings-search-paren = السماح بالتجميع بالأقواس الدائرية
settings-search-env = توسيع متغيرات البيئة
settings-search-fwd-slash = استبدال الشرطات المائلة الأمامية بالخلفية
settings-search-precedence = أسبقية العوامل
settings-search-strict-everything = وضع صيغة Everything الصارم
settings-search-auto-regex = الكشف التلقائي عن التعبير النمطي
settings-search-mod-comp = إكمال المُعدِّلات
settings-search-parse-tree = إظهار شجرة التحليل عند التمرير

# §8.5 General → Results.
settings-results-hide-empty = إخفاء النتائج عند خلو البحث
settings-results-clear-on-search = مسح التحديد عند البحث
settings-results-close-on-execute = إغلاق النافذة عند التنفيذ
settings-results-dbl-path = فتح المسار بنقرة مزدوجة في عمود المسار
settings-results-auto-scroll = تمرير العرض تلقائيًا
settings-results-dquote-copy = النسخ بين علامتي اقتباس مزدوجتين كمسار
settings-results-no-ext-rename = عدم تحديد الامتداد عند إعادة التسمية
settings-results-sort-date-desc = ترتيب التاريخ تنازليًا أولًا
settings-results-sort-size-desc = ترتيب الحجم تنازليًا أولًا
settings-results-list-focus = التركيز على قائمة النتائج
settings-results-icon-prio = أولوية تحميل الأيقونات
settings-results-thumb-prio = أولوية تحميل الصور المصغّرة
settings-results-ext-prio = أولوية تحميل المعلومات الموسّعة
settings-results-group-by-lens = تجميع النتائج حسب العدسة
settings-results-snippet-inline = إظهار معاينة المقتطف ضمن السطر

# §8.6 General → View.
settings-view-double-buffer = تخزين مؤقت مزدوج
settings-view-alt-rows = تلوين الصفوف بالتناوب
settings-view-row-mouseover = إظهار تمرير الفأرة فوق الصف
settings-view-highlight-terms = إبراز مصطلحات البحث
settings-view-status-show-selected = إظهار العنصر المحدّد في شريط الحالة
settings-view-rc-with-sel = إظهار عدد النتائج مع عدد المحدّد
settings-view-status-show-size = إظهار الحجم في شريط الحالة
settings-view-tooltips = إظهار التلميحات
settings-view-update-on-scroll = تحديث العرض فور التمرير
settings-view-size-format = تنسيق الحجم
settings-view-selection-rect = مستطيل التحديد
settings-view-audio-badges = إظهار شارات LUFS / المرمّز / المدة على صفوف الصوت
settings-view-similarity-score = إظهار درجة تشابه MinHash على صفوف التشابه
settings-view-preview-pane = جزء المعاينة

# §8.7 General → Context Menu.
settings-context-menu-visibility = الظهور
settings-context-menu-show = إظهار
settings-context-menu-shift = الإظهار فقط عند الضغط على Shift
settings-context-menu-hide = إخفاء
settings-context-menu-command = ماكرو أوامر
settings-context-menu-open-folders = فتح (المجلدات)
settings-context-menu-open-files = فتح (الملفات)
settings-context-menu-open-path = فتح المسار
settings-context-menu-explore = استكشاف
settings-context-menu-explore-path = استكشاف المسار
settings-context-menu-copy-name = نسخ الاسم إلى الحافظة
settings-context-menu-copy-path = نسخ المسار إلى الحافظة
settings-context-menu-copy-full-name = نسخ الاسم الكامل إلى الحافظة
settings-context-menu-reveal = إظهار في Freally
settings-context-menu-send-to = إرسال إلى Freally (المسار)

# §8.8 General → Fonts & Colors.
settings-fc-font = الخط
settings-fc-size = الحجم
settings-fc-state-normal = عادي
settings-fc-state-highlighted = مُبرَز
settings-fc-state-current-sort = الترتيب الحالي
settings-fc-state-current-sort-h = الترتيب الحالي (مُبرَز)
settings-fc-state-selected = محدّد
settings-fc-state-selected-h = محدّد (مُبرَز)
settings-fc-state-inactive-selected = محدّد غير نشط
settings-fc-state-inactive-selected-h = محدّد غير نشط (مُبرَز)
settings-fc-foreground = المقدّمة
settings-fc-background = الخلفية
settings-fc-bold = عريض
settings-fc-italic = مائل
settings-fc-default = افتراضي
settings-fc-per-lens-accent = لون مميّز لكل عدسة
settings-fc-theme-inherit = قلب الألوان المخصّصة تلقائيًا عند تبديل السمة

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = مفتاح اختصار عام
settings-keyboard-new-window = مفتاح اختصار النافذة الجديدة
settings-keyboard-show-window = مفتاح اختصار إظهار النافذة
settings-keyboard-toggle-window = مفتاح اختصار تبديل النافذة
settings-keyboard-show-commands = إظهار الأوامر التي تحتوي على
settings-keyboard-add-chord = + إضافة تسلسل مفاتيح
settings-keyboard-remove-chord = إزالة

# §8.10 History.
settings-history-search-enable = تفعيل سجل البحث
settings-history-search-keep = الاحتفاظ بسجل البحث لمدة { $days } يوم
settings-history-run-enable = تفعيل سجل التشغيل
settings-history-run-keep = الاحتفاظ بسجل التشغيل لمدة { $days } يوم
settings-history-clear-now = مسح الآن
settings-history-privacy-mode = وضع الخصوصية
settings-history-per-lens = سجل لكل عدسة

# §8.11 Indexes (top-level).
settings-ix-database-location = موقع قاعدة البيانات
settings-ix-multiuser = اسم ملف قاعدة البيانات متعدّدة المستخدمين
settings-ix-compress = ضغط قاعدة البيانات
settings-ix-recent-changes = فهرسة التغييرات الأخيرة
settings-ix-file-size = فهرسة حجم الملف
settings-ix-fast-size-sort = ترتيب سريع حسب الحجم
settings-ix-folder-size = فهرسة حجم المجلد
settings-ix-fast-folder-size-sort = ترتيب سريع حسب حجم المجلد
settings-ix-date-created = فهرسة تاريخ الإنشاء
settings-ix-fast-date-created = ترتيب سريع حسب تاريخ الإنشاء
settings-ix-date-modified = فهرسة تاريخ التعديل
settings-ix-fast-date-modified = ترتيب سريع حسب تاريخ التعديل
settings-ix-date-accessed = فهرسة تاريخ الوصول
settings-ix-fast-date-accessed = ترتيب سريع حسب تاريخ الوصول
settings-ix-attributes = فهرسة السمات
settings-ix-fast-attributes = ترتيب سريع حسب السمات
settings-ix-fast-path-sort = ترتيب سريع حسب المسار
settings-ix-fast-extension-sort = ترتيب سريع حسب الامتداد
settings-ix-force-rebuild = فرض إعادة البناء
settings-ix-compact = ضغط الفهرس
settings-ix-verify = التحقّق من الفهرس
settings-ix-integrity-policy = سياسة سلامة الفهرس
settings-ix-memory-budget = ميزانية الذاكرة للمفهرس
settings-ix-throttle = تقييد الفهرسة في الخلفية

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = تضمين وحدات التخزين الثابتة الجديدة تلقائيًا
settings-vol-auto-removable = تضمين وحدات التخزين القابلة للإزالة الجديدة تلقائيًا
settings-vol-auto-remove-offline = إزالة وحدات التخزين غير المتصلة تلقائيًا
settings-vol-detected = وحدات التخزين المكتشفة
settings-vol-include = تضمين في الفهرس
settings-vol-include-only = التضمين فقط (glob/regex)
settings-vol-enable-usn = تفعيل سجل USN
settings-vol-enable-fsevents = تفعيل تدفق FSEvents
settings-vol-enable-inotify = تفعيل inotify (أو fanotify عند الترقية)
settings-vol-buffer = حجم مخزّن السجل المؤقت (KB)
settings-vol-allocation-delta = فرق التخصيص (KB)
settings-vol-load-recent = تحميل التغييرات الأخيرة من السجل عند بدء التشغيل
settings-vol-monitor = مراقبة التغييرات
settings-vol-recreate-journal = إعادة إنشاء السجل
settings-vol-reset-stream = إعادة تعيين تدفق FSEvents
settings-vol-upgrade-fanotify = الترقية إلى fanotify (polkit)
settings-vol-remove = إزالة

# §8.13 Indexes → Folders.
settings-folders-watched = المجلدات المراقَبة
settings-folders-add = إضافة…
settings-folders-rescan-now = إعادة الفحص الآن
settings-folders-rescan-all = إعادة فحص الكل الآن
settings-folders-monitor = محاولة مراقبة التغييرات
settings-folders-buffer = حجم المخزّن المؤقت
settings-folders-rescan-on-full = إعادة الفحص عند امتلاء المخزّن المؤقت

# §8.14 Indexes → File Lists.
settings-flists-add = إضافة…
settings-flists-monitor = مراقبة التغييرات
settings-flists-editor = محرّر قائمة الملفات…
settings-flists-format = تنسيق قائمة الملفات
settings-flists-format-text = نص (مسار واحد لكل سطر)
settings-flists-format-json = JSON (مع البيانات الوصفية)
settings-flists-format-srcb = حزمة Freally (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = استبعاد الملفات والمجلدات المخفية
settings-exclude-system = استبعاد ملفات ومجلدات النظام
settings-exclude-list-en = تفعيل قائمة الاستبعاد
settings-exclude-folders = استبعاد المجلدات
settings-exclude-include-only-files = تضمين الملفات فقط (glob)
settings-exclude-files = استبعاد الملفات (glob)
settings-exclude-os-recommended = تطبيق الاستبعادات الموصى بها من نظام التشغيل
settings-exclude-by-class = الاستبعاد حسب فئة الامتداد

# §8.16 Lenses → Filename.
settings-lf-trigram = شدّة التصفية المسبقة بالثلاثيات
settings-lf-suffix-mem = ميزانية ذاكرة مصفوفة اللواحق
settings-lf-wildcard-limit = حدّ توسيع أحرف البدل
settings-lf-regex-timeout = مهلة التعبير النمطي

# §8.17 Lenses → Content.
settings-lc-enable = تفعيل عدسة المحتوى
settings-lc-time-budget = ميزانية الوقت لكل مستند
settings-lc-mem-ceiling = الحدّ الأقصى للذاكرة لكل مستند
settings-lc-snippet-len = طول المقتطف
settings-lc-stop-words = لغة كلمات التوقف
settings-lc-re-extract = إعادة الاستخراج عند تغيير الإعدادات
settings-lc-verify-blobs = التحقّق من اختبارات تجزئة كتل النص المستخرج عند القراءة

# §8.18 Lenses → Audio.
settings-la-enable = تفعيل عدسة الصوت
settings-la-lufs-ref = معيار مرجع LUFS
settings-la-peak-compute = حساب الذروة عبر
settings-la-silence-thresh = عتبة الصمت
settings-la-re-extract-modify = إعادة الاستخراج عند حدث التعديل

# §8.19 Lenses → Similarity.
settings-ls-enable = تفعيل عدسة التشابه
settings-ls-sig-size = حجم توقيع MinHash (k)
settings-ls-bands = نطاقات LSH
settings-ls-recall = عتبة الاستدعاء
settings-ls-result-cap = الحدّ الأقصى للنتائج

# §8.20 Lenses → Custom.
settings-custom-registry = السجلّ
settings-custom-trust = الثقة
settings-custom-refresh-hashes = تحديث التجزئات

# §8.21-§8.22 Network.
settings-net-https-enable = تفعيل خادم HTTPS
settings-net-bind = الربط بالواجهات
settings-net-port = الاستماع على المنفذ
settings-net-force-https = فرض HTTPS
settings-net-legacy-auth = مصادقة HTTP الأساسية القديمة
settings-net-token-regen = إعادة توليد الرمز المميّز
settings-net-api-enable = تفعيل خادم API
settings-net-legacy-ftp = دعم FTP/ETP النصي القديم

# §8.23 Privacy & Updates.
settings-privacy-auto-update = التحديث التلقائي
settings-privacy-prerelease = قناة الإصدارات الأولية
settings-privacy-network-policy = سياسة اتصالات الشبكة

# §8.24 Logs & Debug.
settings-logs-level = مستوى السجل
settings-logs-location = موقع ملف السجل
settings-logs-retention = الاحتفاظ بالسجل
settings-logs-debug-overlay = إظهار تراكب التصحيح
settings-logs-open-folder = فتح مجلد السجل
settings-logs-export-bundle = تصدير حزمة التشخيص

# §8.25 Backup, Export, Reset.
settings-backup-export = تصدير الإعدادات
settings-backup-import = استيراد الإعدادات
settings-backup-export-bookmarks = تصدير حزمة الإشارات المرجعية
settings-backup-import-bookmarks = استيراد حزمة الإشارات المرجعية
settings-backup-reset-all = إعادة تعيين كل الإعدادات إلى الافتراضي

# §8.26 Locale.
settings-locale-current = اللغة والمنطقة الحالية
settings-locale-rtl-preview = معاينة من اليمين إلى اليسار
settings-locale-date-format = تنسيق التاريخ
settings-locale-number-format = تنسيق الأرقام

# §8.27 About.
settings-about-version = Freally { $version }
settings-about-license = الترخيص
settings-about-credits = شكر وتقدير
settings-about-notices = إشعارات المصادر المفتوحة

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = معالج التشغيل الأول
wizard-step-of-total = الخطوة { $step } من { $total }
wizard-roots-hint = أضف المجلدات أو وحدات التخزين التي تريد أن يراقبها Freally. يمكنك تغيير ذلك لاحقًا من إعدادات الفهارس.
wizard-browse = استعراض…
wizard-roots-placeholder = …أو الصق مسارًا
wizard-roots-add = إضافة
wizard-roots-remove = إزالة
wizard-roots-empty = لم تُضبط أي جذور بعد.
wizard-locale-hint = يتوفّر Freally بـ 18 لغة. يمكنك التبديل لاحقًا.
wizard-theme-hint = يتبع وضع النظام إعداد مظهر نظام التشغيل لديك.
wizard-back = رجوع
wizard-next = التالي

# Status bar polish.
statusbar-hotkey-hint = مفتاح الاختصار: { $hotkey }
statusbar-cycle-theme = تبديل السمة
statusbar-indexed-suffix = مفهرس

# Results / lenses.
lens-expand = توسيع العدسة
lens-collapse = طيّ العدسة
lens-no-matches = لا توجد مطابقات في هذه العدسة.

# Preview pane.
preview-header = معاينة
preview-loading = جارٍ التحميل…
preview-select-file = حدّد ملفًا لمعاينته.
preview-unavailable = لا تتوفّر معاينة

# Bookmarks.
bookmarks-label = ★ الإشارات المرجعية
bookmarks-empty-hint = لا توجد إشارات مرجعية بعد. اضغط Ctrl+D لحفظ الاستعلام الحالي.
bookmarks-organize-title = تنظيم الإشارات المرجعية
bookmarks-organize-empty = لا توجد إشارات مرجعية بعد.
bookmarks-rename = إعادة تسمية
bookmarks-close = إغلاق

# Settings tree extras.
settings-group-history = السجل
settings-group-privacy = الخصوصية والتحديثات
settings-group-logs = السجلات والتصحيح
settings-group-backup = النسخ الاحتياطي والتصدير وإعادة التعيين
settings-tree-custom-lens = مخصّص
settings-unsaved-changes = تغييرات غير محفوظة

# About dialog.
about-dialog-title = Freally
about-copyright = حقوق النشر © 2026 Mike Weaver. جميع الحقوق محفوظة.
about-close = إغلاق

# Connect endpoint dialog.
connect-ftp-title = الاتصال بخادم FTP
connect-ftp-host = المضيف:
connect-ftp-port = المنفذ:
connect-ftp-username = اسم المستخدم:
connect-ftp-password = كلمة المرور:
connect-ftp-link-type = نوع الاتصال:

# UI panel.
ui-hint = السمة، تكامل شريط المهام / شريط القوائم، البحث أثناء الكتابة، كثافة الصفوف. تطابق مباشر مع voidtools-Everything إضافةً إلى إضافات Freally المميّزة بـ (+).
ui-section-theme = السمة
ui-theme-system-default = النظام (افتراضي)
ui-section-tray = شريط المهام / شريط القوائم
ui-section-search-behavior = سلوك البحث
ui-section-result-rows = صفوف النتائج
ui-single-click-system-default = إعدادات النظام (افتراضي)
ui-single-click-always = نقرة واحدة دائمًا
ui-single-click-always-double = نقرة مزدوجة دائمًا
ui-underline-always = دائمًا
ui-underline-on-hover = عند التمرير
ui-underline-never = أبدًا

# Home panel.
home-hint = إعدادات افتراضية تُحمَّل عند تشغيل التطبيق — يمكن لكل قائمة منسدلة أن تلتزم بـ "استخدام آخر قيمة" أو تثبيت قيمة ثابتة. ظهور العدسات / حدود النتائج من إضافات Freally (+).
home-section-match = إعدادات المطابقة الافتراضية
home-section-search-sort = إعدادات البحث والترتيب الافتراضية
home-search-placeholder = فارغ افتراضيًا
home-section-index = مصدر الفهرس
home-file-list-path = مسار قائمة الملفات
home-https-endpoint = رابط نقطة نهاية واجهة HTTPS API
home-endpoint-token = الرمز المميّز (تظهر البصمة)

# Backup panel.
backup-section-settings = الإعدادات (+)
backup-section-bookmarks = الإشارات المرجعية + المُستخرِجات المخصّصة (+)
backup-section-reset = إعادة التعيين
backup-toast-exported = تم تصدير الإعدادات إلى { $path }
backup-toast-export-failed = فشل التصدير: { $error }
backup-toast-imported = تم استيراد الإعدادات
backup-toast-import-failed = فشل الاستيراد: { $error }
backup-toast-bookmarks-exported = تم تصدير الإشارات المرجعية
backup-toast-bookmarks-export-failed = فشل تصدير الإشارات المرجعية: { $error }
backup-toast-bookmarks-imported = تم استيراد الإشارات المرجعية
backup-toast-bookmarks-import-failed = فشل استيراد الإشارات المرجعية: { $error }
backup-confirm-reset = إعادة تعيين كل الإعدادات إلى الافتراضي؟ لا يمكن التراجع عن ذلك (تبقى نافذة الحوار مفتوحة).
backup-toast-reset = تمت إعادة تعيين كل الإعدادات

# Keyboard panel.
keyboard-section-global = مفاتيح الاختصار العامة
keyboard-placeholder-example = Super+Space
keyboard-section-commands = الأوامر
keyboard-placeholder-command = معرّف الأمر (مثل file.export_results)
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = سجل البحث
history-section-run = سجل التشغيل
history-section-privacy = الخصوصية (+)
history-record-filename = تسجيل سجل عدسة اسم الملف
history-record-content = تسجيل سجل عدسة المحتوى
history-record-audio = تسجيل سجل عدسة الصوت
history-record-similarity = تسجيل سجل عدسة التشابه

# Locale panel.
locale-section-language = اللغة (+)
locale-section-time-date = الوقت / التاريخ (+)
locale-date-os = افتراضي نظام التشغيل
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = مخصّص
locale-date-custom-format = تنسيق مخصّص
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = الأرقام (+)
locale-number-os = افتراضي نظام التشغيل
locale-number-custom = مخصّص
locale-thousands-sep = فاصل الآلاف
locale-decimal-sep = الفاصل العشري

# Folders panel.
folders-hint = مجلدات مراقَبة إضافية تتجاوز وحدات التخزين الافتراضية.
folders-list-title = المجلدات المراقَبة
folders-empty = لم تُضَف أي مجلدات بعد.
folders-remove = إزالة
folders-section-title-dynamic = إعدادات { $path }
folders-section-schedule = جدول إعادة الفحص
folders-schedule-daily = كل يوم في الساعة HH:MM
folders-schedule-hours = كل N ساعة
folders-schedule-never = أبدًا
folders-hour = الساعة
folders-minute = الدقيقة
folders-hours = ساعات
folders-id-label = معرّف المجلد (للقراءة فقط)
folders-select-prompt = حدّد مجلدًا لتهيئته.
folders-section-extras = إضافات Freally (+)
folders-extras-note = إعادة الفحص عند الاستئناف من وضع السكون مفعّلة افتراضيًا في هذا الإصدار؛ سينضمّ المفتاح إلى عناصر التحكم على مستوى المجلد في تحسينات المرحلة 13.

# Volumes panel.
volumes-hint = نظير متعدّد المنصّات للوحات NTFS / ReFS في voidtools-Everything. يكتشف تلقائيًا NTFS / ReFS / exFAT / FAT32 (ويندوز)، وAPFS / HFS+ (ماك)، وext4 / Btrfs / ZFS / XFS / F2FS (لينكس).
volumes-section-auto-include = التضمين التلقائي
volumes-list-title = وحدات التخزين المكتشفة
volumes-detecting = جارٍ الاكتشاف…
volumes-empty = لم تُكتشف أي وحدات تخزين.
volumes-select-prompt = حدّد وحدة تخزين لتهيئتها.

# About panel polish.
about-section-version = الإصدار (+)
about-section-license = الترخيص (+)
about-license-text = Mike Weaver — جميع الحقوق محفوظة. هذا برنامج خاص ومملوك.
about-license-spdx = SPDX: { $spdx }
about-section-credits = شكر وتقدير (+)
about-credits-inspired = مستوحى من Everything من voidtools.
about-credits-voidtools = voidtools.com
about-credits-repo = مستودع المشروع

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = يحتوي على أوامر للعمل مع Freally.
menu-file-new-window = نافذة بحث جديدة
menu-file-open-list = فتح قائمة ملفات…
menu-file-close-list = إغلاق قائمة الملفات
menu-file-close = إغلاق
menu-file-export-results = تصدير النتائج…
menu-file-export-bundle = تصدير حزمة الفهرس…
menu-file-exit = خروج

# Edit menu.
menu-edit-hint = يحتوي على أوامر لتحرير نتائج البحث.
menu-edit-cut = قص
menu-edit-copy = نسخ
menu-edit-paste = لصق
menu-edit-copy-to-folder = نسخ إلى مجلد…
menu-edit-move-to-folder = نقل إلى مجلد…
menu-edit-select-all = تحديد الكل
menu-edit-invert-selection = عكس التحديد
menu-edit-advanced = متقدّم
menu-edit-copy-full-name = نسخ الاسم الكامل
menu-edit-copy-path = نسخ المسار
menu-edit-copy-filename = نسخ اسم الملف
menu-edit-copy-as-json = نسخ كـ JSON
menu-edit-copy-with-metadata = نسخ مع البيانات الوصفية
menu-edit-copy-as-bundle-ref = نسخ كمرجع حزمة Freally

# View menu.
menu-view-hint = يحتوي على أوامر للتحكّم في العرض.
menu-view-filters = عوامل التصفية
menu-view-preview = معاينة
menu-view-status-bar = شريط الحالة
menu-view-thumbs-xl = صور مصغّرة كبيرة جدًا
menu-view-thumbs-l = صور مصغّرة كبيرة
menu-view-thumbs-m = صور مصغّرة متوسطة
menu-view-details = تفاصيل
menu-view-window-size = حجم النافذة
menu-view-window-size-hint = يحتوي على أوامر لضبط حجم النافذة.
menu-view-window-small = صغير
menu-view-window-medium = متوسط
menu-view-window-large = كبير
menu-view-window-auto = ملاءمة تلقائية
menu-view-zoom = تكبير
menu-view-zoom-hint = يحتوي على أوامر لضبط حجم الخط والأيقونات.
menu-view-zoom-in = تكبير
menu-view-zoom-out = تصغير
menu-view-zoom-reset = إعادة تعيين
menu-view-sort-by = الترتيب حسب
menu-view-sort-by-hint = يحتوي على أوامر لترتيب قائمة النتائج.
menu-view-sort-name = الاسم
menu-view-sort-path = المسار
menu-view-sort-size = الحجم
menu-view-sort-ext = الامتداد
menu-view-sort-type = النوع
menu-view-sort-modified = تاريخ التعديل
menu-view-sort-created = تاريخ الإنشاء
menu-view-sort-accessed = تاريخ الوصول
menu-view-sort-attributes = السمات
menu-view-sort-recently-changed = تاريخ آخر تغيير
menu-view-sort-run-count = عدد مرات التشغيل
menu-view-sort-run-date = تاريخ التشغيل
menu-view-sort-file-list-filename = اسم ملف قائمة الملفات
menu-view-sort-lufs = LUFS
menu-view-sort-length = المدة
menu-view-sort-similarity = درجة التشابه
menu-view-sort-asc = تصاعدي
menu-view-sort-desc = تنازلي
menu-view-go-to = الانتقال إلى
menu-view-refresh = تحديث
menu-view-theme = السمة
menu-view-theme-hint = التبديل بين سمات النظام أو الفاتحة أو الداكنة.
menu-view-lenses = العدسات
menu-view-lenses-hint = تبديل ظهور كل عدسة في قائمة النتائج.
menu-view-on-top = في المقدّمة
menu-view-on-top-hint = يحتوي على أوامر لإبقاء هذه النافذة فوق النوافذ الأخرى.
menu-view-on-top-never = أبدًا
menu-view-on-top-always = دائمًا
menu-view-on-top-while-searching = أثناء البحث

# Search menu.
menu-search-hint = يحتوي على مفاتيح تبديل البحث.
menu-search-match-case = مطابقة حالة الأحرف
menu-search-match-whole-word = مطابقة الكلمة بالكامل
menu-search-match-path = مطابقة المسار
menu-search-match-diacritics = مطابقة التشكيل
menu-search-enable-regex = تفعيل التعبير النمطي
menu-search-advanced = بحث متقدّم…
menu-search-add-to-filters = إضافة إلى عوامل التصفية…
menu-search-organize-filters = تنظيم عوامل التصفية…
menu-search-filter-everything = الكل
menu-search-filter-archive = مضغوط (أرشيف)
menu-search-filter-folder = مجلد
menu-search-filter-custom = عامل تصفية مخصّص…

# Bookmarks menu.
menu-bookmarks-hint = يحتوي على أوامر للعمل مع الإشارات المرجعية.
menu-bookmarks-add = إضافة إلى الإشارات المرجعية
menu-bookmarks-organize = تنظيم الإشارات المرجعية…

# Tools menu.
menu-tools-hint = يحتوي على أوامر الأدوات.
menu-tools-connect = الاتصال بخادم FTP…
menu-tools-disconnect = قطع الاتصال بخادم FTP
menu-tools-file-list-editor = محرّر قائمة الملفات…
menu-tools-index-maintenance = صيانة الفهرس
menu-tools-index-maintenance-hint = أدوات صيانة الفهرس.
menu-tools-verify-index = التحقّق من الفهرس…
menu-tools-compact-index = ضغط الفهرس…
menu-tools-rebuild-index = فرض إعادة بناء الفهرس…
menu-tools-custom-extractor = مدير المُستخرِجات المخصّصة…
menu-tools-custom-extractor-hint = إدارة المُستخرِجات المخصّصة المعزولة في بيئة Wasm.
menu-tools-options = الخيارات…

# Help menu.
menu-help-hint = يحتوي على أوامر المساعدة.
menu-help-help = مساعدة Freally
menu-help-search-syntax = صيغة البحث
menu-help-regex-syntax = صيغة التعبير النمطي
menu-help-audio-ref = مرجع مُعدِّلات الصوت
menu-help-similarity-ref = مرجع مُعدِّلات التشابه
menu-help-cli-options = خيارات سطر الأوامر
menu-help-website = موقع Freally
menu-help-check-updates = التحقّق من التحديثات…
menu-help-sponsor = رعاية / تبرّع
menu-help-about = حول Freally…

# Result column headers (short forms used in the table header row).
column-name = الاسم
column-path = المسار
column-size = الحجم
column-modified = آخر تعديل
column-type = النوع
column-ext = الامتداد
column-sort-by = الترتيب حسب { $name }
column-resize = تغيير حجم عمود { $name }

# Section subtitle bars used inside multiple settings panels.
section-behavior = السلوك
section-rendering = العرض المرئي
section-status-bar = شريط الحالة
section-display-format = تنسيق العرض
section-loading-priority = أولوية التحميل
section-compatibility = التوافق
section-storage = التخزين
section-index-fields = حقول الفهرس
section-maintenance = الصيانة
section-logging = التسجيل
section-tools = الأدوات
section-privacy = الخصوصية
section-auto-update = التحديث التلقائي (+)
section-bind = الربط
section-lens = العدسة
section-budgets = الميزانيات
section-other = أخرى
section-per-format-mode = الوضع لكل تنسيق
section-loudness = مستوى الصوت
section-tuning = الضبط (+)
section-minhash-lsh = معاملات MinHash + LSH (+)
section-top-level = المستوى الأعلى
section-file-globs = أنماط الملفات
section-file-list-settings = إعدادات قائمة الملفات المحدّدة
section-editor-format = المحرّر + التنسيق (E + +)
section-api-server = خادم API (E مُكيّف)
section-freally-extras = إضافات Freally (+)
section-freally-additions = إضافات Freally (+)
section-freally-extensions = امتدادات Freally (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = استخدام آخر قيمة
opt-use-last-value-default = استخدام آخر قيمة (افتراضي)
opt-low = منخفض
opt-normal-default = عادي (افتراضي)
opt-high = مرتفع
opt-disabled = معطّل
opt-off = إيقاف
opt-on-battery = عند العمل على البطارية
opt-always = دائمًا
opt-clamp-default = تثبيت (افتراضي)
opt-wrap = التفاف
opt-none = بلا
opt-strict-refuse = صارم (رفض الاستعلامات عند التلف)
opt-lenient-warn = متساهل (تحذير مع متابعة الاستعلام)
opt-system-default = افتراضي النظام
opt-drag-select = التحديد بالسحب
opt-auto-binary = تلقائي (ثنائي)
opt-auto-decimal = تلقائي (عشري)

# Unit suffixes shown next to number inputs.
unit-days = أيام
unit-b = B
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-tb = TB

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = فوري
opt-lazy-default = كسول (افتراضي)
opt-on = تشغيل
opt-on-default = تشغيل (افتراضي)
opt-all = الكل
opt-weekly = أسبوعيًا
opt-monthly = شهريًا
opt-name-asc = الاسم تصاعديًا
opt-name-desc = الاسم تنازليًا
opt-size-asc = الحجم تصاعديًا
opt-size-desc = الحجم تنازليًا
opt-modified-asc = تاريخ التعديل تصاعديًا
opt-modified-desc = تاريخ التعديل تنازليًا
opt-compact = مضغوط
opt-comfortable = مريح
opt-details = تفاصيل
opt-thumbnails = صور مصغّرة
opt-local-db-default = قاعدة بيانات محلية (افتراضي)
opt-file-list = قائمة ملفات
opt-https-endpoint = نقطة نهاية واجهة HTTPS API
opt-right-default = يمين (افتراضي)
opt-bottom = أسفل
opt-or-and-default = OR > AND (افتراضي)
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128 (افتراضي)
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = أفلام البث (-23)
opt-true-peak = الذروة الحقيقية (إفراط أخذ العينات 4×، افتراضي)
opt-sample-peak = ذروة العينة
opt-auto-per-doc = تلقائي (لكل مستند)
opt-log-error = خطأ
opt-log-warn = تحذير
opt-log-info-default = معلومات (افتراضي)
opt-log-debug = تصحيح
opt-log-trace = تتبّع
