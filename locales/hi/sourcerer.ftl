# Sourcerer — हिन्दी.

app-name = Sourcerer
tagline = एक खोज। हर स्रोत। हर OS।
window-title = Sourcerer
search-placeholder = खोजें…
about-version = संस्करण { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = तैयार
status-indexed = अनुक्रमित ({ $count } फ़ाइलें)
status-indexing = अनुक्रमण हो रहा है… { $done }/{ $total }
status-paused = रोका गया
status-error = त्रुटि
status-result-count-one = { $count } परिणाम
status-result-count-many = { $count } परिणाम
status-selection = · { $count } चयनित
status-selection-size = चयनित: { $size }
status-query-timing = क्वेरी: { $ms } मि.से.
status-endpoint-local = स्थानीय DB
status-endpoint-remote = API: { $name }

menu-file = फ़ाइल
menu-edit = संपादन
menu-view = दृश्य
menu-search = खोज
menu-bookmarks = बुकमार्क
menu-tools = उपकरण
menu-help = सहायता

theme-system = सिस्टम
theme-light = हल्का
theme-dark = गहरा

lens-filename = फ़ाइलनाम
lens-content = सामग्री
lens-audio = ऑडियो
lens-similarity = समानता

parse-error-empty = आरंभ करने के लिए कोई क्वेरी टाइप करें।
parse-error-unknown = यहाँ अज्ञात सिंटैक्स।

action-open = खोलें
action-reveal = फ़ोल्डर में दिखाएँ
action-copy-path = पथ कॉपी करें
action-copy-name = नाम कॉपी करें
action-delete = हटाएँ

quick-filter-audio = ऑडियो
quick-filter-video = वीडियो
quick-filter-image = छवि
quick-filter-document = दस्तावेज़
quick-filter-executable = निष्पादन योग्य
quick-filter-archive = संग्रह

wizard-title = Sourcerer में आपका स्वागत है
wizard-step-roots = चुनें कि क्या अनुक्रमित करना है
wizard-step-hotkey = एक ग्लोबल हॉटकी चुनें
wizard-step-locale = अपनी भाषा चुनें
wizard-step-theme = एक थीम चुनें
wizard-finish = समाप्त करें

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = विकल्प
settings-search-placeholder = विकल्प खोजें…
settings-restore-defaults = डिफ़ॉल्ट पुनर्स्थापित करें
settings-ok = ठीक है
settings-cancel = रद्द करें
settings-apply = लागू करें

# Tree nav groups (PRD §8.1.1).
settings-group-general = सामान्य
settings-group-indexes = अनुक्रमणिकाएँ
settings-group-lenses = लेंस
settings-group-network = नेटवर्क

# Tree nav leaves.
settings-node-ui = UI
settings-node-home = होम
settings-node-search = खोज
settings-node-results = परिणाम
settings-node-view = दृश्य
settings-node-context-menu = संदर्भ मेनू
settings-node-fonts-colors = फ़ॉन्ट और रंग
settings-node-keyboard = कीबोर्ड
settings-node-history = इतिहास
settings-node-indexes-top = (शीर्ष-स्तर)
settings-node-volumes = वॉल्यूम
settings-node-folders = फ़ोल्डर
settings-node-file-lists = फ़ाइल सूचियाँ
settings-node-exclude = बहिष्कृत करें
settings-node-https-server = HTTP / HTTPS सर्वर
settings-node-etp-api = ETP / FTP API
settings-node-privacy = गोपनीयता और अद्यतन
settings-node-logs = लॉग और डीबग
settings-node-backup = बैकअप, निर्यात, रीसेट
settings-node-locale = लोकेल
settings-node-about = परिचय

# §8.2 General → UI.
settings-ui-theme = थीम
settings-ui-run-bg = पृष्ठभूमि में चलाएँ
settings-ui-show-tray = ट्रे / मेनू-बार आइकन दिखाएँ
settings-ui-single-click-tray = ट्रे / मेनू बार पर सिंगल क्लिक
settings-ui-new-window-from-tray = ट्रे आइकन से नई विंडो खोलें
settings-ui-new-window-on-launch = Sourcerer लॉन्च करते समय नई विंडो खोलें
settings-ui-search-as-you-type = टाइप करते ही खोजें
settings-ui-select-on-mouse-click = माउस क्लिक पर खोज चयन करें
settings-ui-focus-on-activate = सक्रिय करने पर खोज पर फ़ोकस करें
settings-ui-full-row-select = पूरी पंक्ति चयन
settings-ui-single-click-open = सिंगल क्लिक पर खोलें
settings-ui-underline-titles = आइकन शीर्षकों को रेखांकित करें
settings-ui-row-density = परिणाम घनत्व
settings-ui-row-density-compact = सघन (32 px)
settings-ui-row-density-comfortable = आरामदायक (44 px)
settings-ui-show-timing-badges = प्रत्येक लेंस पर टाइमिंग बैज दिखाएँ
settings-ui-anim-crossfade = थीम बदलते समय एनिमेटेड क्रॉस-फ़ेड

# §8.3 General → Home.
settings-home-match-case = केस मिलान करें
settings-home-match-whole-word = पूरे शब्द का मिलान करें
settings-home-match-path = पथ का मिलान करें
settings-home-match-diacritics = डायक्रिटिक्स का मिलान करें
settings-home-match-regex = Regex का मिलान करें
settings-home-search = खोज (कस्टम डिफ़ॉल्ट क्वेरी)
settings-home-filter = फ़िल्टर
settings-home-sort = क्रमबद्ध करें
settings-home-view = दृश्य
settings-home-index = अनुक्रमणिका
settings-home-default-lens-visibility = डिफ़ॉल्ट लेंस दृश्यता
settings-home-default-lens-result-limits = डिफ़ॉल्ट लेंस परिणाम सीमाएँ

# §8.4 General → Search.
settings-search-fast-ascii = तेज़ ASCII खोज
settings-search-mp-sep = जब खोज शब्द में पथ विभाजक हो तो पथ का मिलान करें
settings-search-mw-fn = वाइल्डकार्ड का उपयोग करते समय पूरे फ़ाइलनाम का मिलान करें
settings-search-lit-ops = शाब्दिक ऑपरेटरों की अनुमति दें
settings-search-paren = गोल कोष्ठक समूहन की अनुमति दें
settings-search-env = पर्यावरण चर का विस्तार करें
settings-search-fwd-slash = फ़ॉरवर्ड स्लैश को बैकस्लैश से बदलें
settings-search-precedence = ऑपरेटर प्राथमिकता
settings-search-strict-everything = सख्त Everything सिंटैक्स मोड
settings-search-auto-regex = Regex स्वतः-पहचानें
settings-search-mod-comp = मॉडिफ़ायर पूर्णता
settings-search-parse-tree = होवर पर पार्स-ट्री दिखाएँ

# §8.5 General → Results.
settings-results-hide-empty = खोज खाली होने पर परिणाम छिपाएँ
settings-results-clear-on-search = खोज पर चयन साफ़ करें
settings-results-close-on-execute = निष्पादन पर विंडो बंद करें
settings-results-dbl-path = पथ कॉलम में डबल क्लिक से पथ खोलें
settings-results-auto-scroll = दृश्य को स्वचालित रूप से स्क्रॉल करें
settings-results-dquote-copy = डबल कोट को पथ के रूप में कॉपी करें
settings-results-no-ext-rename = नाम बदलते समय एक्सटेंशन का चयन न करें
settings-results-sort-date-desc = पहले तारीख घटते क्रम में क्रमबद्ध करें
settings-results-sort-size-desc = पहले आकार घटते क्रम में क्रमबद्ध करें
settings-results-list-focus = परिणाम सूची फ़ोकस
settings-results-icon-prio = आइकन लोड प्राथमिकता
settings-results-thumb-prio = थंबनेल लोड प्राथमिकता
settings-results-ext-prio = विस्तारित जानकारी लोड प्राथमिकता
settings-results-group-by-lens = परिणामों को लेंस के अनुसार समूहित करें
settings-results-snippet-inline = स्निपेट पूर्वावलोकन इनलाइन दिखाएँ

# §8.6 General → View.
settings-view-double-buffer = डबल बफ़र
settings-view-alt-rows = वैकल्पिक पंक्ति रंग
settings-view-row-mouseover = पंक्ति माउसओवर दिखाएँ
settings-view-highlight-terms = हाइलाइट किए गए खोज शब्द दिखाएँ
settings-view-status-show-selected = स्थिति बार में चयनित आइटम दिखाएँ
settings-view-rc-with-sel = परिणाम संख्या को चयन संख्या के साथ दिखाएँ
settings-view-status-show-size = स्थिति बार में आकार दिखाएँ
settings-view-tooltips = टूलटिप दिखाएँ
settings-view-update-on-scroll = स्क्रॉल करने के तुरंत बाद डिस्प्ले अद्यतन करें
settings-view-size-format = आकार प्रारूप
settings-view-selection-rect = चयन आयत
settings-view-audio-badges = ऑडियो पंक्तियों पर LUFS / codec / लंबाई बैज दिखाएँ
settings-view-similarity-score = समानता पंक्तियों पर MinHash समानता स्कोर दिखाएँ
settings-view-preview-pane = पूर्वावलोकन फलक

# §8.7 General → Context Menu.
settings-context-menu-visibility = दृश्यता
settings-context-menu-show = दिखाएँ
settings-context-menu-shift = केवल Shift दबाने पर दिखाएँ
settings-context-menu-hide = छिपाएँ
settings-context-menu-command = कमांड मैक्रो
settings-context-menu-open-folders = खोलें (फ़ोल्डर)
settings-context-menu-open-files = खोलें (फ़ाइलें)
settings-context-menu-open-path = पथ खोलें
settings-context-menu-explore = एक्सप्लोर करें
settings-context-menu-explore-path = पथ एक्सप्लोर करें
settings-context-menu-copy-name = नाम क्लिपबोर्ड पर कॉपी करें
settings-context-menu-copy-path = पथ क्लिपबोर्ड पर कॉपी करें
settings-context-menu-copy-full-name = पूरा नाम क्लिपबोर्ड पर कॉपी करें
settings-context-menu-reveal = Sourcerer में दिखाएँ
settings-context-menu-send-to = Sourcerer पर भेजें (पथ)

# §8.8 General → Fonts & Colors.
settings-fc-font = फ़ॉन्ट
settings-fc-size = आकार
settings-fc-state-normal = सामान्य
settings-fc-state-highlighted = हाइलाइट किया गया
settings-fc-state-current-sort = वर्तमान क्रम
settings-fc-state-current-sort-h = वर्तमान क्रम (हाइलाइट किया गया)
settings-fc-state-selected = चयनित
settings-fc-state-selected-h = चयनित (हाइलाइट किया गया)
settings-fc-state-inactive-selected = निष्क्रिय चयनित
settings-fc-state-inactive-selected-h = निष्क्रिय चयनित (हाइलाइट किया गया)
settings-fc-foreground = अग्रभूमि
settings-fc-background = पृष्ठभूमि
settings-fc-bold = मोटा
settings-fc-italic = तिरछा
settings-fc-default = डिफ़ॉल्ट
settings-fc-per-lens-accent = प्रति-लेंस उच्चारण
settings-fc-theme-inherit = थीम बदलने पर कस्टम रंग स्वतः पलटें

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = ग्लोबल हॉटकी
settings-keyboard-new-window = नई विंडो हॉटकी
settings-keyboard-show-window = विंडो दिखाएँ हॉटकी
settings-keyboard-toggle-window = विंडो टॉगल हॉटकी
settings-keyboard-show-commands = इन्हें युक्त कमांड दिखाएँ
settings-keyboard-add-chord = + कॉर्ड जोड़ें
settings-keyboard-remove-chord = हटाएँ

# §8.10 History.
settings-history-search-enable = खोज इतिहास सक्षम करें
settings-history-search-keep = खोज इतिहास { $days } दिनों तक रखें
settings-history-run-enable = रन इतिहास सक्षम करें
settings-history-run-keep = रन इतिहास { $days } दिनों तक रखें
settings-history-clear-now = अभी साफ़ करें
settings-history-privacy-mode = गोपनीयता मोड
settings-history-per-lens = प्रति-लेंस इतिहास

# §8.11 Indexes (top-level).
settings-ix-database-location = डेटाबेस स्थान
settings-ix-multiuser = बहु-उपयोगकर्ता डेटाबेस फ़ाइलनाम
settings-ix-compress = डेटाबेस संपीड़ित करें
settings-ix-recent-changes = हालिया परिवर्तन अनुक्रमित करें
settings-ix-file-size = फ़ाइल आकार अनुक्रमित करें
settings-ix-fast-size-sort = तेज़ आकार क्रम
settings-ix-folder-size = फ़ोल्डर आकार अनुक्रमित करें
settings-ix-fast-folder-size-sort = तेज़ फ़ोल्डर आकार क्रम
settings-ix-date-created = निर्माण तिथि अनुक्रमित करें
settings-ix-fast-date-created = तेज़ निर्माण तिथि क्रम
settings-ix-date-modified = संशोधन तिथि अनुक्रमित करें
settings-ix-fast-date-modified = तेज़ संशोधन तिथि क्रम
settings-ix-date-accessed = पहुँच तिथि अनुक्रमित करें
settings-ix-fast-date-accessed = तेज़ पहुँच तिथि क्रम
settings-ix-attributes = विशेषताएँ अनुक्रमित करें
settings-ix-fast-attributes = तेज़ विशेषताएँ क्रम
settings-ix-fast-path-sort = तेज़ पथ क्रम
settings-ix-fast-extension-sort = तेज़ एक्सटेंशन क्रम
settings-ix-force-rebuild = पुनर्निर्माण के लिए बाध्य करें
settings-ix-compact = अनुक्रमणिका संक्षिप्त करें
settings-ix-verify = अनुक्रमणिका सत्यापित करें
settings-ix-integrity-policy = अनुक्रमणिका अखंडता नीति
settings-ix-memory-budget = अनुक्रमणकर्ता के लिए मेमोरी बजट
settings-ix-throttle = पृष्ठभूमि अनुक्रमण थ्रॉटल

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = नए स्थिर वॉल्यूम स्वचालित रूप से शामिल करें
settings-vol-auto-removable = नए हटाने योग्य वॉल्यूम स्वचालित रूप से शामिल करें
settings-vol-auto-remove-offline = ऑफ़लाइन वॉल्यूम स्वचालित रूप से हटाएँ
settings-vol-detected = पहचाने गए वॉल्यूम
settings-vol-include = अनुक्रमणिका में शामिल करें
settings-vol-include-only = केवल शामिल करें (glob/regex)
settings-vol-enable-usn = USN Journal सक्षम करें
settings-vol-enable-fsevents = FSEvents स्ट्रीम सक्षम करें
settings-vol-enable-inotify = inotify सक्षम करें (या उन्नत होने पर fanotify)
settings-vol-buffer = जर्नल बफ़र आकार (KB)
settings-vol-allocation-delta = आवंटन डेल्टा (KB)
settings-vol-load-recent = स्टार्टअप पर जर्नल से हालिया परिवर्तन लोड करें
settings-vol-monitor = परिवर्तनों की निगरानी करें
settings-vol-recreate-journal = जर्नल पुनः बनाएँ
settings-vol-reset-stream = FSEvents स्ट्रीम रीसेट करें
settings-vol-upgrade-fanotify = fanotify पर अपग्रेड करें (polkit)
settings-vol-remove = हटाएँ

# §8.13 Indexes → Folders.
settings-folders-watched = निगरानी किए गए फ़ोल्डर
settings-folders-add = जोड़ें…
settings-folders-rescan-now = अभी पुनः स्कैन करें
settings-folders-rescan-all = सभी को अभी पुनः स्कैन करें
settings-folders-monitor = परिवर्तनों की निगरानी का प्रयास करें
settings-folders-buffer = बफ़र आकार
settings-folders-rescan-on-full = बफ़र भरने पर पुनः स्कैन करें

# §8.14 Indexes → File Lists.
settings-flists-add = जोड़ें…
settings-flists-monitor = परिवर्तनों की निगरानी करें
settings-flists-editor = फ़ाइल सूची संपादक…
settings-flists-format = फ़ाइल सूची प्रारूप
settings-flists-format-text = पाठ (प्रति पंक्ति एक पथ)
settings-flists-format-json = JSON (मेटाडेटा के साथ)
settings-flists-format-srcb = Sourcerer Bundle (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = छिपी फ़ाइलें और फ़ोल्डर बहिष्कृत करें
settings-exclude-system = सिस्टम फ़ाइलें और फ़ोल्डर बहिष्कृत करें
settings-exclude-list-en = बहिष्कार सूची सक्षम करें
settings-exclude-folders = फ़ोल्डर बहिष्कृत करें
settings-exclude-include-only-files = केवल फ़ाइलें शामिल करें (glob)
settings-exclude-files = फ़ाइलें बहिष्कृत करें (glob)
settings-exclude-os-recommended = OS-अनुशंसित बहिष्कार लागू करें
settings-exclude-by-class = एक्सटेंशन वर्ग द्वारा बहिष्कृत करें

# §8.16 Lenses → Filename.
settings-lf-trigram = Trigram पूर्व-फ़िल्टर तीव्रता
settings-lf-suffix-mem = Suffix-array मेमोरी बजट
settings-lf-wildcard-limit = वाइल्डकार्ड विस्तार सीमा
settings-lf-regex-timeout = Regex समय-समाप्ति

# §8.17 Lenses → Content.
settings-lc-enable = सामग्री लेंस सक्षम करें
settings-lc-time-budget = प्रति दस्तावेज़ समय बजट
settings-lc-mem-ceiling = प्रति दस्तावेज़ मेमोरी सीमा
settings-lc-snippet-len = स्निपेट लंबाई
settings-lc-stop-words = स्टॉप-शब्द भाषा
settings-lc-re-extract = सेटिंग बदलने पर पुनः निकालें
settings-lc-verify-blobs = पढ़ते समय निकाले गए-पाठ blob चेकसम सत्यापित करें

# §8.18 Lenses → Audio.
settings-la-enable = ऑडियो लेंस सक्षम करें
settings-la-lufs-ref = LUFS संदर्भ मानक
settings-la-peak-compute = पीक की गणना इस माध्यम से करें
settings-la-silence-thresh = मौन सीमा
settings-la-re-extract-modify = Modify इवेंट पर पुनः निकालें

# §8.19 Lenses → Similarity.
settings-ls-enable = समानता लेंस सक्षम करें
settings-ls-sig-size = MinHash हस्ताक्षर आकार (k)
settings-ls-bands = LSH बैंड
settings-ls-recall = रीकॉल सीमा
settings-ls-result-cap = परिणाम सीमा

# §8.20 Lenses → Custom.
settings-custom-registry = रजिस्ट्री
settings-custom-trust = विश्वास
settings-custom-refresh-hashes = हैश ताज़ा करें

# §8.21-§8.22 Network.
settings-net-https-enable = HTTPS सर्वर सक्षम करें
settings-net-bind = इंटरफ़ेस से बाँधें
settings-net-port = पोर्ट पर सुनें
settings-net-force-https = HTTPS के लिए बाध्य करें
settings-net-legacy-auth = लिगेसी HTTP-basic प्रमाणीकरण
settings-net-token-regen = टोकन पुनर्जनन
settings-net-api-enable = API सर्वर सक्षम करें
settings-net-legacy-ftp = लिगेसी सादा FTP/ETP समर्थन

# §8.23 Privacy & Updates.
settings-privacy-auto-update = स्वतः-अद्यतन
settings-privacy-prerelease = पूर्व-रिलीज़ चैनल
settings-privacy-network-policy = नेटवर्क कॉल नीति

# §8.24 Logs & Debug.
settings-logs-level = लॉग स्तर
settings-logs-location = लॉग फ़ाइल स्थान
settings-logs-retention = लॉग प्रतिधारण
settings-logs-debug-overlay = डीबग ओवरले दिखाएँ
settings-logs-open-folder = लॉग फ़ोल्डर खोलें
settings-logs-export-bundle = निदान बंडल निर्यात करें

# §8.25 Backup, Export, Reset.
settings-backup-export = सेटिंग्स निर्यात करें
settings-backup-import = सेटिंग्स आयात करें
settings-backup-export-bookmarks = बुकमार्क बंडल निर्यात करें
settings-backup-import-bookmarks = बुकमार्क बंडल आयात करें
settings-backup-reset-all = सभी सेटिंग्स को डिफ़ॉल्ट पर रीसेट करें

# §8.26 Locale.
settings-locale-current = वर्तमान लोकेल
settings-locale-rtl-preview = RTL पूर्वावलोकन
settings-locale-date-format = तारीख प्रारूप
settings-locale-number-format = संख्या प्रारूप

# §8.27 About.
settings-about-version = Sourcerer { $version }
settings-about-license = लाइसेंस
settings-about-credits = श्रेय
settings-about-notices = ओपन-सोर्स सूचनाएँ
