# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = एक खोज। हर स्रोत। हर OS।
window-title = Freally Sourcerer
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
status-query-timing = क्वेरी: { $ms } ms
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

lens-filename = फ़ाइल नाम
lens-content = सामग्री
lens-audio = ऑडियो
lens-similarity = समानता

parse-error-empty = आरंभ करने के लिए कोई क्वेरी लिखें।
parse-error-unknown = यहाँ अपरिचित सिंटैक्स है।

action-open = खोलें
action-reveal = फ़ोल्डर में दिखाएँ
action-copy-path = पथ कॉपी करें
action-copy-name = नाम कॉपी करें
action-delete = हटाएँ

quick-filter-audio = ऑडियो
quick-filter-video = वीडियो
quick-filter-image = छवि
quick-filter-document = दस्तावेज़
quick-filter-executable = एक्ज़ीक्यूटेबल
quick-filter-archive = संग्रह

wizard-title = Freally में आपका स्वागत है
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
settings-node-privacy = गोपनीयता और अपडेट
settings-node-logs = लॉग और डिबग
settings-node-backup = बैकअप, निर्यात, रीसेट
settings-node-locale = लोकेल
settings-node-about = परिचय

# §8.2 General → UI.
settings-ui-theme = थीम
settings-ui-run-bg = पृष्ठभूमि में चलाएँ
settings-ui-show-tray = ट्रे / मेनू-बार आइकन दिखाएँ
settings-ui-single-click-tray = ट्रे / मेनू बार पर सिंगल क्लिक
settings-ui-new-window-from-tray = ट्रे आइकन से नई विंडो खोलें
settings-ui-new-window-on-launch = Freally लॉन्च करते समय नई विंडो खोलें
settings-ui-search-as-you-type = टाइप करते ही खोजें
settings-ui-select-on-mouse-click = माउस क्लिक पर खोज चुनें
settings-ui-focus-on-activate = सक्रिय करने पर खोज पर फ़ोकस करें
settings-ui-full-row-select = पूरी पंक्ति चुनें
settings-ui-single-click-open = सिंगल क्लिक से खोलें
settings-ui-underline-titles = आइकन शीर्षकों को रेखांकित करें
settings-ui-row-density = परिणाम घनत्व
settings-ui-row-density-compact = सघन (32 px)
settings-ui-row-density-comfortable = आरामदायक (44 px)
settings-ui-show-timing-badges = प्रति लेंस समय बैज दिखाएँ
settings-ui-anim-crossfade = एनिमेटेड थीम क्रॉस-फ़ेड

# §8.3 General → Home.
settings-home-match-case = केस मिलाएँ
settings-home-match-whole-word = पूरा शब्द मिलाएँ
settings-home-match-path = पथ मिलाएँ
settings-home-match-diacritics = डायाक्रिटिक्स मिलाएँ
settings-home-match-regex = regex मिलाएँ
settings-home-search = खोज (कस्टम डिफ़ॉल्ट क्वेरी)
settings-home-filter = फ़िल्टर
settings-home-sort = क्रमबद्ध करें
settings-home-view = दृश्य
settings-home-index = अनुक्रमणिका
settings-home-default-lens-visibility = डिफ़ॉल्ट लेंस दृश्यता
settings-home-default-lens-result-limits = डिफ़ॉल्ट लेंस परिणाम सीमाएँ

# §8.4 General → Search.
settings-search-fast-ascii = तेज़ ASCII खोज
settings-search-mp-sep = जब खोज शब्द में पथ विभाजक हो तो पथ मिलाएँ
settings-search-mw-fn = वाइल्डकार्ड उपयोग करते समय पूरा फ़ाइल नाम मिलाएँ
settings-search-lit-ops = लिटरल ऑपरेटरों की अनुमति दें
settings-search-paren = गोल कोष्ठक समूहन की अनुमति दें
settings-search-env = एनवायरनमेंट वेरिएबल विस्तृत करें
settings-search-fwd-slash = फॉरवर्ड स्लैश को बैकस्लैश से बदलें
settings-search-precedence = ऑपरेटर प्राथमिकता
settings-search-strict-everything = सख्त Everything सिंटैक्स मोड
settings-search-auto-regex = regex स्वतः पहचानें
settings-search-mod-comp = मॉडिफ़ायर पूर्णता
settings-search-parse-tree = होवर पर पार्स-ट्री दिखाएँ

# §8.5 General → Results.
settings-results-hide-empty = खोज खाली होने पर परिणाम छिपाएँ
settings-results-clear-on-search = खोज पर चयन साफ़ करें
settings-results-close-on-execute = निष्पादन पर विंडो बंद करें
settings-results-dbl-path = पथ कॉलम में डबल क्लिक से पथ खोलें
settings-results-auto-scroll = दृश्य को स्वतः स्क्रॉल करें
settings-results-dquote-copy = पथ के रूप में कॉपी करते समय डबल कोट लगाएँ
settings-results-no-ext-rename = नाम बदलते समय एक्सटेंशन न चुनें
settings-results-sort-date-desc = पहले तिथि अवरोही क्रम में लगाएँ
settings-results-sort-size-desc = पहले आकार अवरोही क्रम में लगाएँ
settings-results-list-focus = परिणाम सूची फ़ोकस
settings-results-icon-prio = आइकन लोड प्राथमिकता
settings-results-thumb-prio = थंबनेल लोड प्राथमिकता
settings-results-ext-prio = विस्तृत जानकारी लोड प्राथमिकता
settings-results-group-by-lens = परिणामों को लेंस अनुसार समूहित करें
settings-results-snippet-inline = स्निपेट पूर्वावलोकन इनलाइन दिखाएँ

# §8.6 General → View.
settings-view-double-buffer = डबल बफ़र
settings-view-alt-rows = वैकल्पिक पंक्ति रंग
settings-view-row-mouseover = पंक्ति माउसओवर दिखाएँ
settings-view-highlight-terms = हाइलाइट किए गए खोज शब्द दिखाएँ
settings-view-status-show-selected = स्थिति बार में चयनित आइटम दिखाएँ
settings-view-rc-with-sel = परिणाम गणना को चयन गणना के साथ दिखाएँ
settings-view-status-show-size = स्थिति बार में आकार दिखाएँ
settings-view-tooltips = टूलटिप दिखाएँ
settings-view-update-on-scroll = स्क्रॉल करने के तुरंत बाद प्रदर्शन अपडेट करें
settings-view-size-format = आकार प्रारूप
settings-view-selection-rect = चयन आयत
settings-view-audio-badges = ऑडियो पंक्तियों पर LUFS / कोडेक / अवधि बैज दिखाएँ
settings-view-similarity-score = समानता पंक्तियों पर MinHash समानता स्कोर दिखाएँ
settings-view-preview-pane = पूर्वावलोकन फलक

# §8.7 General → Context Menu.
settings-context-menu-visibility = दृश्यता
settings-context-menu-show = दिखाएँ
settings-context-menu-shift = केवल Shift दबाए रखने पर दिखाएँ
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
settings-context-menu-reveal = Freally में दिखाएँ
settings-context-menu-send-to = Freally को भेजें (पथ)

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
settings-fc-bold = बोल्ड
settings-fc-italic = इटैलिक
settings-fc-default = डिफ़ॉल्ट
settings-fc-per-lens-accent = प्रति-लेंस एक्सेंट
settings-fc-theme-inherit = थीम बदलने पर कस्टम रंग स्वतः पलटें

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = ग्लोबल हॉटकी
settings-keyboard-new-window = नई विंडो हॉटकी
settings-keyboard-show-window = विंडो दिखाने की हॉटकी
settings-keyboard-toggle-window = विंडो टॉगल करने की हॉटकी
settings-keyboard-show-commands = ये शब्द वाले कमांड दिखाएँ
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
settings-ix-multiuser = बहु-उपयोगकर्ता डेटाबेस फ़ाइल नाम
settings-ix-compress = डेटाबेस संपीड़ित करें
settings-ix-recent-changes = हाल के परिवर्तन अनुक्रमित करें
settings-ix-file-size = फ़ाइल आकार अनुक्रमित करें
settings-ix-fast-size-sort = तेज़ आकार क्रमबद्धन
settings-ix-folder-size = फ़ोल्डर आकार अनुक्रमित करें
settings-ix-fast-folder-size-sort = तेज़ फ़ोल्डर आकार क्रमबद्धन
settings-ix-date-created = निर्माण तिथि अनुक्रमित करें
settings-ix-fast-date-created = तेज़ निर्माण तिथि क्रमबद्धन
settings-ix-date-modified = संशोधन तिथि अनुक्रमित करें
settings-ix-fast-date-modified = तेज़ संशोधन तिथि क्रमबद्धन
settings-ix-date-accessed = एक्सेस तिथि अनुक्रमित करें
settings-ix-fast-date-accessed = तेज़ एक्सेस तिथि क्रमबद्धन
settings-ix-attributes = विशेषताएँ अनुक्रमित करें
settings-ix-fast-attributes = तेज़ विशेषता क्रमबद्धन
settings-ix-fast-path-sort = तेज़ पथ क्रमबद्धन
settings-ix-fast-extension-sort = तेज़ एक्सटेंशन क्रमबद्धन
settings-ix-force-rebuild = पुनर्निर्माण के लिए बाध्य करें
settings-ix-compact = अनुक्रमणिका संहत करें
settings-ix-verify = अनुक्रमणिका सत्यापित करें
settings-ix-integrity-policy = अनुक्रमणिका अखंडता नीति
settings-ix-memory-budget = अनुक्रमक के लिए मेमोरी बजट
settings-ix-throttle = पृष्ठभूमि अनुक्रमण थ्रॉटल

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = नए स्थिर वॉल्यूम स्वतः शामिल करें
settings-vol-auto-removable = नए हटाने योग्य वॉल्यूम स्वतः शामिल करें
settings-vol-auto-remove-offline = ऑफ़लाइन वॉल्यूम स्वतः हटाएँ
settings-vol-detected = पता लगाए गए वॉल्यूम
settings-vol-include = अनुक्रमणिका में शामिल करें
settings-vol-include-only = केवल शामिल करें (glob/regex)
settings-vol-enable-usn = USN Journal सक्षम करें
settings-vol-enable-fsevents = FSEvents स्ट्रीम सक्षम करें
settings-vol-enable-inotify = inotify सक्षम करें (या उन्नत होने पर fanotify)
settings-vol-buffer = Journal बफ़र आकार (KB)
settings-vol-allocation-delta = आवंटन डेल्टा (KB)
settings-vol-load-recent = स्टार्टअप पर journal से हाल के परिवर्तन लोड करें
settings-vol-monitor = परिवर्तनों की निगरानी करें
settings-vol-recreate-journal = journal पुनः बनाएँ
settings-vol-reset-stream = FSEvents स्ट्रीम रीसेट करें
settings-vol-upgrade-fanotify = fanotify में अपग्रेड करें (polkit)
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
settings-flists-format-text = टेक्स्ट (प्रति पंक्ति एक पथ)
settings-flists-format-json = JSON (मेटाडेटा सहित)
settings-flists-format-srcb = Freally Bundle (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = छिपी फ़ाइलें और फ़ोल्डर बहिष्कृत करें
settings-exclude-system = सिस्टम फ़ाइलें और फ़ोल्डर बहिष्कृत करें
settings-exclude-list-en = बहिष्करण सूची सक्षम करें
settings-exclude-folders = फ़ोल्डर बहिष्कृत करें
settings-exclude-include-only-files = केवल फ़ाइलें शामिल करें (glob)
settings-exclude-files = फ़ाइलें बहिष्कृत करें (glob)
settings-exclude-os-recommended = OS-अनुशंसित बहिष्करण लागू करें
settings-exclude-by-class = एक्सटेंशन वर्ग अनुसार बहिष्कृत करें

# §8.16 Lenses → Filename.
settings-lf-trigram = ट्राइग्राम पूर्व-फ़िल्टर तीव्रता
settings-lf-suffix-mem = प्रत्यय-सरणी मेमोरी बजट
settings-lf-wildcard-limit = वाइल्डकार्ड विस्तार सीमा
settings-lf-regex-timeout = regex टाइमआउट

# §8.17 Lenses → Content.
settings-lc-enable = सामग्री लेंस सक्षम करें
settings-lc-time-budget = प्रति दस्तावेज़ समय बजट
settings-lc-mem-ceiling = प्रति दस्तावेज़ मेमोरी सीमा
settings-lc-snippet-len = स्निपेट लंबाई
settings-lc-stop-words = स्टॉप-वर्ड भाषा
settings-lc-re-extract = सेटिंग बदलने पर पुनः निकालें
settings-lc-verify-blobs = पढ़ते समय निकाले गए टेक्स्ट ब्लॉब चेकसम सत्यापित करें

# §8.18 Lenses → Audio.
settings-la-enable = ऑडियो लेंस सक्षम करें
settings-la-lufs-ref = LUFS संदर्भ मानक
settings-la-peak-compute = पीक की गणना इसके द्वारा
settings-la-silence-thresh = मौन सीमा
settings-la-re-extract-modify = Modify घटना पर पुनः निकालें

# §8.19 Lenses → Similarity.
settings-ls-enable = समानता लेंस सक्षम करें
settings-ls-sig-size = MinHash हस्ताक्षर आकार (k)
settings-ls-bands = LSH बैंड
settings-ls-recall = रिकॉल सीमा
settings-ls-result-cap = परिणाम सीमा

# §8.20 Lenses → Custom.
settings-custom-registry = रजिस्ट्री
settings-custom-trust = विश्वास
settings-custom-refresh-hashes = हैश ताज़ा करें

# §8.21-§8.22 Network.
settings-net-https-enable = HTTPS सर्वर सक्षम करें
settings-net-bind = इंटरफ़ेस से बाँधें
settings-net-port = इस पोर्ट पर सुनें
settings-net-force-https = HTTPS के लिए बाध्य करें
settings-net-legacy-auth = लीगेसी HTTP-बेसिक प्रमाणीकरण
settings-net-token-regen = टोकन पुनर्जनन
settings-net-api-enable = API सर्वर सक्षम करें
settings-net-legacy-ftp = लीगेसी सादा FTP/ETP समर्थन

# §8.23 Privacy & Updates.
settings-privacy-auto-update = स्वतः-अपडेट
settings-privacy-prerelease = प्री-रिलीज़ चैनल
settings-privacy-network-policy = नेटवर्क कॉल नीति

# §8.24 Logs & Debug.
settings-logs-level = लॉग स्तर
settings-logs-location = लॉग फ़ाइल स्थान
settings-logs-retention = लॉग प्रतिधारण
settings-logs-debug-overlay = डिबग ओवरले दिखाएँ
settings-logs-open-folder = लॉग फ़ोल्डर खोलें
settings-logs-export-bundle = डायग्नोस्टिक्स बंडल निर्यात करें

# §8.25 Backup, Export, Reset.
settings-backup-export = सेटिंग निर्यात करें
settings-backup-import = सेटिंग आयात करें
settings-backup-export-bookmarks = बुकमार्क बंडल निर्यात करें
settings-backup-import-bookmarks = बुकमार्क बंडल आयात करें
settings-backup-reset-all = सभी सेटिंग को डिफ़ॉल्ट पर रीसेट करें

# §8.26 Locale.
settings-locale-current = वर्तमान लोकेल
settings-locale-rtl-preview = RTL पूर्वावलोकन
settings-locale-date-format = तिथि प्रारूप
settings-locale-number-format = संख्या प्रारूप

# §8.27 About.
settings-about-version = Freally { $version }
settings-about-license = लाइसेंस
settings-about-credits = श्रेय
settings-about-notices = ओपन-सोर्स सूचनाएँ

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = प्रथम-रन विज़ार्ड
wizard-step-of-total = चरण { $step } / { $total }
wizard-roots-hint = वे फ़ोल्डर या वॉल्यूम जोड़ें जिन्हें आप Freally से देखना चाहते हैं। आप इसे बाद में अनुक्रमणिका सेटिंग से बदल सकते हैं।
wizard-browse = ब्राउज़ करें…
wizard-roots-placeholder = …या कोई पथ चिपकाएँ
wizard-roots-add = जोड़ें
wizard-roots-remove = हटाएँ
wizard-roots-empty = अभी तक कोई रूट कॉन्फ़िगर नहीं किया गया।
wizard-locale-hint = Freally 18 भाषाओं में उपलब्ध है। आप बाद में बदल सकते हैं।
wizard-theme-hint = सिस्टम आपके OS स्वरूप सेटिंग का अनुसरण करता है।
wizard-back = पीछे
wizard-next = आगे

# Status bar polish.
statusbar-hotkey-hint = हॉटकी: { $hotkey }
statusbar-cycle-theme = थीम बदलें
statusbar-indexed-suffix = अनुक्रमित

# Results / lenses.
lens-expand = लेंस विस्तृत करें
lens-collapse = लेंस संक्षिप्त करें
lens-no-matches = इस लेंस में कोई मिलान नहीं।

# Preview pane.
preview-header = पूर्वावलोकन
preview-loading = लोड हो रहा है…
preview-select-file = पूर्वावलोकन के लिए कोई फ़ाइल चुनें।
preview-unavailable = कोई पूर्वावलोकन उपलब्ध नहीं

# Bookmarks.
bookmarks-label = ★ बुकमार्क
bookmarks-empty-hint = अभी तक कोई बुकमार्क नहीं। वर्तमान क्वेरी सहेजने के लिए Ctrl+D दबाएँ।
bookmarks-organize-title = बुकमार्क व्यवस्थित करें
bookmarks-organize-empty = अभी तक कोई बुकमार्क नहीं।
bookmarks-rename = नाम बदलें
bookmarks-close = बंद करें

# Settings tree extras.
settings-group-history = इतिहास
settings-group-privacy = गोपनीयता और अपडेट
settings-group-logs = लॉग और डिबग
settings-group-backup = बैकअप, निर्यात, रीसेट
settings-tree-custom-lens = कस्टम
settings-unsaved-changes = असहेजे गए परिवर्तन

# About dialog.
about-dialog-title = Freally
about-copyright = Copyright © 2026 Mike Weaver. All rights reserved.
about-close = बंद करें

# Connect endpoint dialog.
connect-ftp-title = FTP सर्वर से कनेक्ट करें
connect-ftp-host = होस्ट:
connect-ftp-port = पोर्ट:
connect-ftp-username = उपयोगकर्ता नाम:
connect-ftp-password = पासवर्ड:
connect-ftp-link-type = लिंक प्रकार:

# UI panel.
ui-hint = थीम, ट्रे / मेनू-बार एकीकरण, टाइप करते ही खोज, पंक्ति घनत्व। प्रत्यक्ष voidtools-Everything समानता के साथ (+) से चिह्नित Freally अतिरिक्त सुविधाएँ।
ui-section-theme = थीम
ui-theme-system-default = सिस्टम (डिफ़ॉल्ट)
ui-section-tray = ट्रे / मेनू बार
ui-section-search-behavior = खोज व्यवहार
ui-section-result-rows = परिणाम पंक्तियाँ
ui-single-click-system-default = सिस्टम सेटिंग (डिफ़ॉल्ट)
ui-single-click-always = हमेशा सिंगल क्लिक
ui-single-click-always-double = हमेशा डबल क्लिक
ui-underline-always = हमेशा
ui-underline-on-hover = होवर पर
ui-underline-never = कभी नहीं

# Home panel.
home-hint = ऐप लॉन्च पर लोड किए गए डिफ़ॉल्ट — हर ड्रॉपडाउन "अंतिम मान उपयोग करें" पर टिक सकता है या कोई स्थिर मान पिन कर सकता है। लेंस दृश्यता / परिणाम सीमाएँ Freally अतिरिक्त सुविधाएँ हैं (+)।
home-section-match = मिलान डिफ़ॉल्ट
home-section-search-sort = खोज और क्रमबद्धन डिफ़ॉल्ट
home-search-placeholder = डिफ़ॉल्ट रूप से खाली
home-section-index = अनुक्रमणिका स्रोत
home-file-list-path = फ़ाइल सूची पथ
home-https-endpoint = HTTPS API एंडपॉइंट URL
home-endpoint-token = टोकन (फ़िंगरप्रिंट दिखाया गया)

# Backup panel.
backup-section-settings = सेटिंग (+)
backup-section-bookmarks = बुकमार्क + कस्टम एक्सट्रैक्टर (+)
backup-section-reset = रीसेट
backup-toast-exported = सेटिंग { $path } में निर्यात की गईं
backup-toast-export-failed = निर्यात विफल: { $error }
backup-toast-imported = सेटिंग आयात की गईं
backup-toast-import-failed = आयात विफल: { $error }
backup-toast-bookmarks-exported = बुकमार्क निर्यात किए गए
backup-toast-bookmarks-export-failed = बुकमार्क निर्यात विफल: { $error }
backup-toast-bookmarks-imported = बुकमार्क आयात किए गए
backup-toast-bookmarks-import-failed = बुकमार्क आयात विफल: { $error }
backup-confirm-reset = सभी सेटिंग को डिफ़ॉल्ट पर रीसेट करें? इसे पूर्ववत नहीं किया जा सकता (डायलॉग खुला रहता है)।
backup-confirm-import-commands = यह सेटिंग फ़ाइल कस्टम कमांड जोड़ती है जो आपके कंप्यूटर पर प्रोग्राम चला सकते हैं: { $programs }. इन्हें आयात करें?
backup-toast-reset = सभी सेटिंग रीसेट कर दी गईं

# Keyboard panel.
keyboard-section-global = ग्लोबल हॉटकी
keyboard-placeholder-example = Super+Space
keyboard-section-commands = कमांड
keyboard-placeholder-command = कमांड id (उदा. file.export_results)
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = खोज इतिहास
history-section-run = रन इतिहास
history-section-privacy = गोपनीयता (+)
history-record-filename = फ़ाइल नाम-लेंस इतिहास रिकॉर्ड करें
history-record-content = सामग्री-लेंस इतिहास रिकॉर्ड करें
history-record-audio = ऑडियो-लेंस इतिहास रिकॉर्ड करें
history-record-similarity = समानता-लेंस इतिहास रिकॉर्ड करें

# Locale panel.
locale-section-language = भाषा (+)
locale-section-time-date = समय / तिथि (+)
locale-date-os = OS डिफ़ॉल्ट
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = कस्टम
locale-date-custom-format = कस्टम प्रारूप
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = संख्याएँ (+)
locale-number-os = OS डिफ़ॉल्ट
locale-number-custom = कस्टम
locale-thousands-sep = हज़ार विभाजक
locale-decimal-sep = दशमलव विभाजक

# Folders panel.
folders-hint = डिफ़ॉल्ट वॉल्यूम के अतिरिक्त निगरानी किए जाने वाले फ़ोल्डर।
folders-list-title = निगरानी किए गए फ़ोल्डर
folders-empty = अभी तक कोई फ़ोल्डर नहीं जोड़ा गया।
folders-remove = हटाएँ
folders-section-title-dynamic = { $path } के लिए सेटिंग
folders-section-schedule = पुनः स्कैन अनुसूची
folders-schedule-daily = हर दिन HH:MM बजे
folders-schedule-hours = हर N घंटे
folders-schedule-never = कभी नहीं
folders-hour = घंटा
folders-minute = मिनट
folders-hours = घंटे
folders-id-label = फ़ोल्डर ID (केवल-पठन)
folders-select-prompt = कॉन्फ़िगर करने के लिए कोई फ़ोल्डर चुनें।
folders-section-extras = Freally अतिरिक्त (+)
folders-extras-note = इस बिल्ड में नींद से जागने पर पुनः स्कैन डिफ़ॉल्ट रूप से सक्षम है; यह टॉगल Phase 13 के पॉलिश पास में फ़ोल्डर-स्तरीय नियंत्रणों के साथ जुड़ेगा।

# Volumes panel.
volumes-hint = voidtools-Everything के NTFS / ReFS फलकों का क्रॉस-प्लेटफ़ॉर्म समकक्ष। NTFS / ReFS / exFAT / FAT32 (Win), APFS / HFS+ (macOS), ext4 / Btrfs / ZFS / XFS / F2FS (Linux) का स्वतः पता लगाता है।
volumes-section-auto-include = स्वतः-शामिल करें
volumes-list-title = पता लगाए गए वॉल्यूम
volumes-detecting = पता लगाया जा रहा है…
volumes-empty = कोई वॉल्यूम नहीं मिला।
volumes-select-prompt = कॉन्फ़िगर करने के लिए कोई वॉल्यूम चुनें।

# About panel polish.
about-section-version = संस्करण (+)
about-section-license = लाइसेंस (+)
about-license-text = Mike Weaver — All Rights Reserved. This is proprietary software.
about-license-spdx = SPDX: { $spdx }
about-section-credits = श्रेय (+)
about-credits-inspired = voidtools द्वारा Everything से प्रेरित।
about-credits-voidtools = voidtools.com
about-credits-repo = प्रोजेक्ट रिपॉज़िटरी

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = Freally के साथ काम करने के कमांड शामिल हैं।
menu-file-new-window = नई खोज विंडो
menu-file-open-list = फ़ाइल सूची खोलें…
menu-file-close-list = फ़ाइल सूची बंद करें
menu-file-close = बंद करें
menu-file-export-results = परिणाम निर्यात करें…
menu-file-export-bundle = अनुक्रमणिका बंडल निर्यात करें…
menu-file-exit = बाहर निकलें

# Edit menu.
menu-edit-hint = खोज परिणाम संपादित करने के कमांड शामिल हैं।
menu-edit-cut = काटें
menu-edit-copy = कॉपी करें
menu-edit-paste = चिपकाएँ
menu-edit-copy-to-folder = फ़ोल्डर में कॉपी करें…
menu-edit-move-to-folder = फ़ोल्डर में ले जाएँ…
menu-edit-select-all = सभी चुनें
menu-edit-invert-selection = चयन उलटें
menu-edit-advanced = उन्नत
menu-edit-copy-full-name = पूरा नाम कॉपी करें
menu-edit-copy-path = पथ कॉपी करें
menu-edit-copy-filename = फ़ाइल नाम कॉपी करें
menu-edit-copy-as-json = JSON के रूप में कॉपी करें
menu-edit-copy-with-metadata = मेटाडेटा सहित कॉपी करें
menu-edit-copy-as-bundle-ref = Freally Bundle संदर्भ के रूप में कॉपी करें

# View menu.
menu-view-hint = दृश्य में बदलाव के कमांड शामिल हैं।
menu-view-filters = फ़िल्टर
menu-view-preview = पूर्वावलोकन
menu-view-status-bar = स्थिति बार
menu-view-thumbs-xl = अतिरिक्त बड़े थंबनेल
menu-view-thumbs-l = बड़े थंबनेल
menu-view-thumbs-m = मध्यम थंबनेल
menu-view-details = विवरण
menu-view-window-size = विंडो आकार
menu-view-window-size-hint = विंडो का आकार समायोजित करने के कमांड शामिल हैं।
menu-view-window-small = छोटा
menu-view-window-medium = मध्यम
menu-view-window-large = बड़ा
menu-view-window-auto = स्वतः फ़िट
menu-view-zoom = ज़ूम
menu-view-zoom-hint = फ़ॉन्ट और आइकन आकार समायोजित करने के कमांड शामिल हैं।
menu-view-zoom-in = ज़ूम इन
menu-view-zoom-out = ज़ूम आउट
menu-view-zoom-reset = रीसेट करें
menu-view-sort-by = इसके अनुसार क्रमबद्ध करें
menu-view-sort-by-hint = परिणाम सूची क्रमबद्ध करने के कमांड शामिल हैं।
menu-view-sort-name = नाम
menu-view-sort-path = पथ
menu-view-sort-size = आकार
menu-view-sort-ext = एक्सटेंशन
menu-view-sort-type = प्रकार
menu-view-sort-modified = संशोधन तिथि
menu-view-sort-created = निर्माण तिथि
menu-view-sort-accessed = एक्सेस तिथि
menu-view-sort-attributes = विशेषताएँ
menu-view-sort-recently-changed = हाल में परिवर्तित तिथि
menu-view-sort-run-count = रन गणना
menu-view-sort-run-date = रन तिथि
menu-view-sort-file-list-filename = फ़ाइल सूची फ़ाइल नाम
menu-view-sort-lufs = LUFS
menu-view-sort-length = अवधि
menu-view-sort-similarity = समानता स्कोर
menu-view-sort-asc = आरोही
menu-view-sort-desc = अवरोही
menu-view-go-to = यहाँ जाएँ
menu-view-refresh = ताज़ा करें
menu-view-theme = थीम
menu-view-theme-hint = सिस्टम, हल्की या गहरी थीम के बीच बदलें।
menu-view-lenses = लेंस
menu-view-lenses-hint = परिणाम सूची में प्रत्येक लेंस की दृश्यता टॉगल करें।
menu-view-on-top = ऊपर रखें
menu-view-on-top-hint = इस विंडो को अन्य विंडो के ऊपर रखने के कमांड शामिल हैं।
menu-view-on-top-never = कभी नहीं
menu-view-on-top-always = हमेशा
menu-view-on-top-while-searching = खोज के दौरान

# Search menu.
menu-search-hint = खोज टॉगल शामिल हैं।
menu-search-match-case = केस मिलाएँ
menu-search-match-whole-word = पूरा शब्द मिलाएँ
menu-search-match-path = पथ मिलाएँ
menu-search-match-diacritics = डायाक्रिटिक्स मिलाएँ
menu-search-enable-regex = regex सक्षम करें
menu-search-advanced = उन्नत खोज…
menu-search-add-to-filters = फ़िल्टर में जोड़ें…
menu-search-organize-filters = फ़िल्टर व्यवस्थित करें…
menu-search-filter-everything = Everything
menu-search-filter-archive = संपीड़ित (संग्रह)
menu-search-filter-folder = फ़ोल्डर
menu-search-filter-custom = कस्टम फ़िल्टर…

# Bookmarks menu.
menu-bookmarks-hint = बुकमार्क के साथ काम करने के कमांड शामिल हैं।
menu-bookmarks-add = बुकमार्क में जोड़ें
menu-bookmarks-organize = बुकमार्क व्यवस्थित करें…

# Tools menu.
menu-tools-hint = उपकरण कमांड शामिल हैं।
menu-tools-connect = FTP सर्वर से कनेक्ट करें…
menu-tools-disconnect = FTP सर्वर से डिस्कनेक्ट करें
menu-tools-file-list-editor = फ़ाइल सूची संपादक…
menu-tools-index-maintenance = अनुक्रमणिका अनुरक्षण
menu-tools-index-maintenance-hint = अनुक्रमणिका अनुरक्षण उपकरण।
menu-tools-verify-index = अनुक्रमणिका सत्यापित करें…
menu-tools-compact-index = अनुक्रमणिका संहत करें…
menu-tools-rebuild-index = अनुक्रमणिका पुनर्निर्माण के लिए बाध्य करें…
menu-tools-custom-extractor = कस्टम एक्सट्रैक्टर प्रबंधक…
menu-tools-custom-extractor-hint = Wasm-सैंडबॉक्स कस्टम एक्सट्रैक्टर प्रबंधित करें।
menu-tools-options = विकल्प…

# Help menu.
menu-help-hint = सहायता कमांड शामिल हैं।
menu-help-help = Freally सहायता
menu-help-search-syntax = खोज सिंटैक्स
menu-help-regex-syntax = regex सिंटैक्स
menu-help-audio-ref = ऑडियो मॉडिफ़ायर संदर्भ
menu-help-similarity-ref = समानता मॉडिफ़ायर संदर्भ
menu-help-cli-options = कमांड लाइन विकल्प
menu-help-website = Freally वेबसाइट
menu-help-check-updates = अपडेट जाँचें…
menu-help-sponsor = प्रायोजन / दान करें
menu-help-about = Freally के बारे में…

# Result column headers (short forms used in the table header row).
column-name = नाम
column-path = पथ
column-size = आकार
column-modified = संशोधित
column-type = प्रकार
column-ext = एक्स्ट
column-sort-by = { $name } अनुसार क्रमबद्ध करें
column-resize = { $name } कॉलम का आकार बदलें

# Section subtitle bars used inside multiple settings panels.
section-behavior = व्यवहार
section-rendering = रेंडरिंग
section-status-bar = स्थिति बार
section-display-format = प्रदर्शन प्रारूप
section-loading-priority = लोडिंग प्राथमिकता
section-compatibility = संगतता
section-storage = भंडारण
section-index-fields = अनुक्रमणिका फ़ील्ड
section-maintenance = अनुरक्षण
section-logging = लॉगिंग
section-tools = उपकरण
section-privacy = गोपनीयता
section-auto-update = स्वतः-अपडेट (+)
section-bind = बाँधें
section-lens = लेंस
section-budgets = बजट
section-other = अन्य
section-per-format-mode = प्रति-प्रारूप मोड
section-loudness = प्रबलता
section-tuning = ट्यूनिंग (+)
section-minhash-lsh = MinHash + LSH पैरामीटर (+)
section-top-level = शीर्ष-स्तर
section-file-globs = फ़ाइल glob
section-file-list-settings = चयनित फ़ाइल सूची के लिए सेटिंग
section-editor-format = संपादक + प्रारूप (E + +)
section-api-server = API सर्वर (E अनुकूलित)
section-freally-extras = Freally अतिरिक्त (+)
section-freally-additions = Freally अतिरिक्त सुविधाएँ (+)
section-freally-extensions = Freally एक्सटेंशन (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = अंतिम मान उपयोग करें
opt-use-last-value-default = अंतिम मान उपयोग करें (डिफ़ॉल्ट)
opt-low = निम्न
opt-normal-default = सामान्य (डिफ़ॉल्ट)
opt-high = उच्च
opt-disabled = अक्षम
opt-off = बंद
opt-on-battery = बैटरी पर होने पर
opt-always = हमेशा
opt-clamp-default = क्लैंप (डिफ़ॉल्ट)
opt-wrap = रैप
opt-none = कोई नहीं
opt-strict-refuse = सख्त (भ्रष्टाचार पर क्वेरी अस्वीकार करें)
opt-lenient-warn = उदार (चेतावनी दें पर क्वेरी करें)
opt-system-default = सिस्टम डिफ़ॉल्ट
opt-drag-select = ड्रैग-चयन
opt-auto-binary = स्वतः (बाइनरी)
opt-auto-decimal = स्वतः (दशमलव)

# Unit suffixes shown next to number inputs.
unit-days = दिन
unit-b = B
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-tb = TB

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = तत्पर
opt-lazy-default = आलसी (डिफ़ॉल्ट)
opt-on = चालू
opt-on-default = चालू (डिफ़ॉल्ट)
opt-all = सभी
opt-weekly = साप्ताहिक
opt-monthly = मासिक
opt-name-asc = नाम आरोही
opt-name-desc = नाम अवरोही
opt-size-asc = आकार आरोही
opt-size-desc = आकार अवरोही
opt-modified-asc = संशोधन तिथि आरोही
opt-modified-desc = संशोधन तिथि अवरोही
opt-compact = सघन
opt-comfortable = आरामदायक
opt-details = विवरण
opt-thumbnails = थंबनेल
opt-local-db-default = स्थानीय डेटाबेस (डिफ़ॉल्ट)
opt-file-list = फ़ाइल सूची
opt-https-endpoint = HTTPS API एंडपॉइंट
opt-right-default = दायाँ (डिफ़ॉल्ट)
opt-bottom = नीचे
opt-or-and-default = OR > AND (डिफ़ॉल्ट)
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128 (डिफ़ॉल्ट)
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = प्रसारण फ़िल्म (-23)
opt-true-peak = ट्रू पीक (4× ओवरसैंपलिंग, डिफ़ॉल्ट)
opt-sample-peak = सैंपल पीक
opt-auto-per-doc = स्वतः (प्रति-दस्तावेज़)
opt-log-error = त्रुटि
opt-log-warn = चेतावनी
opt-log-info-default = जानकारी (डिफ़ॉल्ट)
opt-log-debug = डिबग
opt-log-trace = ट्रेस


# ---------------------------------------------------------------------
# Build 1 — v0.21.0 (SRC-M01 … SRC-M08)
# Hit-in-context viewer, search-within-results, file-list interop,
# Open with…, advanced copy verbs, terminal-here + custom commands,
# duplicate groups, and the emptiness modifiers.
# Machine-translated draft — pending the pre-v1.0 reviewer pass.
dupe-group-summary = { $count } प्रतियाँ · { $bytes }
status-file-list = फ़ाइल सूची: { $name }
status-file-list-truncated = पहली 5,000 प्रविष्टियाँ दिखाई जा रही हैं
refine-label = परिणामों में
refine-input-label = मौजूदा परिणाम सीमित करें
refine-placeholder = सीमित करने के लिए लिखें, जोड़ने के लिए Enter
refine-count = { $total } में से { $shown }
refine-remove-chip = { $term } फ़िल्टर हटाएँ
refine-clear = साफ़ करें
refine-close = परिष्करण पट्टी बंद करें
menu-search-within-results = परिणामों में खोजें
ctx-menu-label = परिणाम क्रियाएँ
ctx-open = खोलें
ctx-reveal = फ़ाइल प्रबंधक में दिखाएँ
ctx-view-hits = मिलान संदर्भ सहित देखें
ctx-open-with = इससे खोलें
ctx-open-with-none = कोई पंजीकृत अनुप्रयोग नहीं
ctx-loading = लोड हो रहा है…
ctx-copy-contents = सामग्री कॉपी करें
ctx-copy-as-file = फ़ाइल के रूप में कॉपी करें
ctx-copy-path-list = पथ सूची कॉपी करें
ctx-quote-lines = प्रति पंक्ति एक
ctx-quote-quoted = उद्धरण चिह्नों में
ctx-quote-space = स्पेस से अलग
ctx-quote-escaped = शेल के लिए एस्केप
ctx-terminal-here = यहाँ टर्मिनल खोलें
verb-failed = { $verb } विफल: { $error }
hitviewer-title = { $name } में मिलान
hitviewer-no-matches = कोई मिलान नहीं
hitviewer-match-position = { $total } में से मिलान { $current }
hitviewer-prev = पिछला
hitviewer-next = अगला
hitviewer-loading = निकाला जा रहा है…
hitviewer-extractor = { $id } द्वारा निकाला गया
hitviewer-truncated = प्रदर्शन हेतु दस्तावेज़ छोटा किया गया
hitviewer-keys = F3 अगला · Shift+F3 पिछला · Esc बंद
close = बंद करें
toast-dismiss = हटाएँ
toast-file-list-opened = { $name } खोला गया — { $count } प्रविष्टियाँ
toast-file-list-open-failed = फ़ाइल सूची नहीं खुल सकी: { $error }
toast-export-done = { $written } परिणाम { $format } के रूप में निर्यात किए गए
toast-export-lossy = { $total } में से { $written } परिणाम { $format } के रूप में निर्यात किए गए
toast-export-failed = निर्यात विफल: { $error }
toast-copied-paths = { $count } पथ कॉपी किए गए
toast-copied-contents = { $name } की सामग्री कॉपी की गई
toast-copied-files = { $count } फ़ाइलें कॉपी की गईं
settings-node-custom-commands = कस्टम कमांड
settings-custom-commands-hint = यहाँ जोड़े गए कमांड परिणाम के राइट-क्लिक मेनू में दिखते हैं। तर्कों में { "{" }path{ "}" }, { "{" }dir{ "}" }, { "{" }name{ "}" }, { "{" }stem{ "}" } और { "{" }ext{ "}" } का उपयोग करें।
settings-custom-commands-empty = अभी कोई कस्टम कमांड नहीं है।
settings-custom-commands-unnamed = बिना नाम का कमांड
settings-custom-commands-name = नाम
settings-custom-commands-program = प्रोग्राम
settings-custom-commands-args = तर्क
settings-custom-commands-extensions = केवल इन एक्सटेंशन के लिए
settings-custom-commands-add = कमांड जोड़ें
did-you-mean-label = क्या आपका मतलब था
menu-search-match-phonetic = CJK ध्वन्यात्मकता मिलाएँ

# SRC-M13 — Index Health panel + rebuild advisor
menu-tools-index-health = सूचकांक स्वास्थ्य…
health-title = सूचकांक स्वास्थ्य
health-loading = सूचकांक स्वास्थ्य पढ़ा जा रहा है…
health-all-good = सभी निगरानी वाले स्थान अद्यतित हैं।
health-no-watched-roots = अभी तक कोई फ़ोल्डर अनुक्रमित नहीं हो रहा।
health-col-volume = स्थान
health-col-status = स्थिति
health-col-lag = विलंब
health-col-last-event = अंतिम परिवर्तन
health-col-applied = लागू
health-col-dropped = छूटे
health-col-queue = कतार
health-status-live = लाइव
health-status-scan-only = केवल स्कैन
health-lag-value = { $last } मिसे (शिखर { $max } मिसे)
health-no-events-yet = अभी कोई परिवर्तन नहीं
health-never = कभी नहीं
health-ago = { $secs } से पहले
health-legend = विलंब वह समय है जो डिस्क पर हुए परिवर्तन से लेकर उसके परिणामों में दिखने तक लगता है। «लागू» अनुक्रमित घटनाएँ गिनता है, विलय की गई घटनाएँ कोष्ठक में हैं।
health-extraction-untracked = सामग्री-निष्कर्षण बैकलॉग अभी ट्रैक नहीं किया जाता।
health-extraction-backlog = सामग्री-निष्कर्षण बैकलॉग: { $count }
health-fix-rebuild = सूचकांक पुनर्निर्मित करें
health-advice-journal-stream-reset = ऑपरेटिंग सिस्टम ने { $root } पर परिवर्तन-धारा हटा दी। इस बीच हुए परिवर्तन सूचकांक में नहीं हैं — पुनर्निर्माण उन्हें वापस लाता है।
health-advice-events-dropped = { $root } पर { $count } परिवर्तन अनुक्रमित होने से तेज़ आए और छूट गए। पुनर्निर्माण उन्हें वापस लाता है।
health-advice-not-monitoring = { $root } पर परिवर्तन ट्रैक नहीं हो रहे, इसलिए परिणाम केवल दोबारा स्कैन करने पर अद्यतन होते हैं।
health-advice-high-lag = { $root } पर परिवर्तनों को परिणामों में दिखने में { $count } मिसे तक लग रहे हैं।
health-advice-queue-saturated = { $root } की परिवर्तन-कतार लगभग भरी है ({ $count } प्रतीक्षारत); परिवर्तन छूटने लग सकते हैं।

# SRC-M14 — offline removable-volume catalogs
results-offline-badge = ऑफ़लाइन — { $name }
results-offline-badge-title = इस फ़ाइल वाली ड्राइव कनेक्ट नहीं है। यह प्रविष्टि उस डिवाइस के लिए Freally के कैटलॉग से आई है।

# SRC-M15 bulk rename + SRC-M16 undo/redo
menu-edit-undo = पूर्ववत करें
menu-edit-redo = फिर से करें
menu-edit-bulk-rename = नाम बदलें…
rename-title = { $count } चयनित का नाम बदलें
rename-find = खोजें
rename-replace = इससे बदलें
rename-use-regex = नियमित अभिव्यक्ति
rename-ignore-case = बड़े-छोटे अक्षर अनदेखा करें
rename-part = इस पर लागू करें
rename-part-stem = एक्सटेंशन रहित नाम
rename-part-full = पूरा फ़ाइल नाम
rename-case = अक्षर रूप
rename-case-none = अपरिवर्तित
rename-case-lower = छोटे अक्षर
rename-case-upper = बड़े अक्षर
rename-case-title = प्रत्येक शब्द बड़ा
rename-counter-start = गणक आरंभ
rename-hint = गणक के लिए { "{" }n{ "}" } और उसे भरने के लिए { "{" }n:03{ "}" } का उपयोग करें। नियमित अभिव्यक्ति में $1 एक समूह जोड़ता है।
rename-col-before = पहले
rename-col-after = बाद में
rename-col-status = स्थिति
rename-status-ok = नाम बदला जाएगा
rename-status-unchanged = अपरिवर्तित
rename-status-invalid = अमान्य नाम
rename-status-collision = दो फ़ाइलों का नाम एक जैसा हो जाएगा
rename-status-exists = इस नाम की फ़ाइल पहले से है
rename-invalid-empty = नाम खाली रह जाएगा
rename-invalid-path-separator = नाम में फ़ोल्डर विभाजक नहीं हो सकता
rename-invalid-dot-name = नाम . या .. नहीं हो सकता
rename-invalid-forbidden-character = नाम में अस्वीकृत वर्ण है
rename-invalid-reserved-name = नाम ऑपरेटिंग सिस्टम द्वारा आरक्षित है
rename-invalid-bad-pattern = खोज पैटर्न अमान्य है
rename-summary = { $count } फ़ाइलों का नाम बदला जाएगा।
rename-blocked = नाम बदलने से पहले चिह्नित पंक्तियाँ ठीक करें। तब तक कुछ नहीं बदलेगा।
rename-apply = नाम बदलें
rename-toast-done = { $count } फ़ाइलों के नाम बदले गए
rename-nothing-selected = पहले कुछ परिणाम चुनें।
ops-toast-undone = पूर्ववत — { $count } फ़ाइलें वापस की गईं
ops-toast-redone = पुनः किया — { $count } फ़ाइलों के नाम फिर बदले गए
about-portable-on = पोर्टेबल मोड चालू है
about-portable-hint = आपकी अनुक्रमणिका, सेटिंग्स और बुकमार्क इसी फ़ोल्डर में रहते हैं। इस कंप्यूटर पर कुछ भी नहीं लिखा जाता।
settings-results-natural-sort = प्राकृतिक क्रम (file10 से पहले file2)
menu-search-ignore-punctuation = विराम-चिह्न अनदेखा करें
menu-search-ignore-whitespace = रिक्त स्थान अनदेखा करें
regex-builder-title = रेगुलर एक्सप्रेशन बिल्डर
regex-builder-placeholder = कोई पैटर्न लिखें, जैसे ^report-\d+
regex-builder-match-count = दिखाए गए { $total } नामों में से { $matched } मेल खाते हैं
regex-builder-no-results = पहले खोजें — बिल्डर आपके पैटर्न को स्क्रीन पर दिख रहे नामों पर आज़माता है।
regex-builder-cheatsheet = त्वरित संदर्भ
regex-builder-flavor-note = Rust सिंटैक्स। बैकरेफ़रेंस और लुकअराउंड नहीं — खोज इंजन इन्हें समर्थित नहीं करता।
regex-builder-use = यह पैटर्न इस्तेमाल करें
regex-cheat-any = कोई भी एक वर्ण
regex-cheat-classes = अंक, शब्द-वर्ण, रिक्त स्थान
regex-cheat-set = समुच्चय में से एक, या समुच्चय से बाहर
regex-cheat-repeat = शून्य या अधिक, एक या अधिक, वैकल्पिक
regex-cheat-count = ठीक इतने, कम से कम, या एक सीमा
regex-cheat-anchor = नाम का आरंभ और अंत
regex-cheat-alt = इनमें से कोई एक
regex-cheat-inline-flag = यहाँ से केस अनदेखा करें
regex-cheat-noncapturing = कैप्चर किए बिना समूह बनाएँ
regex-cheat-escape = विशेष वर्ण को एस्केप करें
quicklook-title = त्वरित झलक
quicklook-position = { $total } में से { $at }
quicklook-hint = तीर कुंजियाँ परिणामों में चलती हैं · स्पेस या Esc बंद करता है
quicklook-open-native = macOS क्विक लुक में खोलें
menu-view-sidebar = साइडबार
sidebar-title = साइडबार
sidebar-bookmarks = बुकमार्क
sidebar-filters = फ़िल्टर
sidebar-volumes = वॉल्यूम
sidebar-recent = हाल की खोजें
sidebar-no-bookmarks = अभी कोई बुकमार्क नहीं
sidebar-no-volumes = कोई वॉल्यूम अनुक्रमित नहीं
sidebar-no-recent = अभी कुछ नहीं
sidebar-clear-recent = साफ़ करें
menu-tools-permission-health = अनुमति स्थिति…
permissions-title = अनुमति स्थिति
permissions-all-clear = आपने Freally से जो कुछ अनुक्रमित करने को कहा, वह सब पढ़ा जा सका।
permissions-summary = पहुँच अस्वीकृत होने के कारण { $denied } फ़ोल्डर नहीं पढ़े जा सके, और { $other } अन्य कारणों से विफल रहे। उनके भीतर की फ़ाइलें अनुक्रमणिका में नहीं हैं और परिणामों में नहीं दिखेंगी।
permissions-truncated = { $dropped } और गिने गए, पर सूचीबद्ध नहीं किए गए।
permissions-how-to-fix = इसे कैसे ठीक करें
permissions-by-volume = कहाँ
permissions-group-counts = { $denied } अस्वीकृत · { $other } अन्य
permissions-macos-granted = Freally के पास पहले से पूर्ण डिस्क पहुँच है। नीचे दिए फ़ोल्डर किसी और चीज़ से सुरक्षित हैं — उनका स्वामी और अनुमतियाँ जाँचें।
permissions-macos-steps = जब तक आप पूर्ण डिस्क पहुँच नहीं देते, macOS इन फ़ोल्डरों की रक्षा करता है। गोपनीयता और सुरक्षा → पूर्ण डिस्क पहुँच खोलें, Freally चालू करें, फिर Freally बंद करके दोबारा खोलें और फिर से स्कैन करें।
permissions-macos-open-settings = पूर्ण डिस्क पहुँच सेटिंग्स खोलें
permissions-macos-undetermined = Freally यह तय नहीं कर सका कि पूर्ण डिस्क पहुँच पहले से दी गई है या नहीं, इसलिए शायद समस्या यह न हो।
permissions-linux-steps = आपका उपयोगकर्ता ये पथ नहीं पढ़ सकता। फ़ोल्डर का स्वामी और मोड जाँचें, या फ़ाइल-सिस्टम को ऐसी अनुमतियों के साथ माउंट करें जिनसे आपका खाता गुज़र सके। सिस्टम-व्यापी पथों को अनुक्रमित करने के लिए डीमन को उच्च विशेषाधिकारों के साथ चलाना पड़ सकता है।
permissions-windows-steps = एक Windows पहुँच नियम आपके खाते को अस्वीकार करता है। कौन-सा नियम लागू है यह देखने के लिए फ़ोल्डर के गुण → सुरक्षा खोलें, या अनुक्रमणिका सेवा को ऐसे उपयोगकर्ता के रूप में चलाएँ जो इसे पढ़ सके।
permissions-generic-steps = ऑपरेटिंग सिस्टम ने इन पथों को पढ़ने से मना कर दिया। उनकी अनुमतियाँ जाँचें।
status-paths-skipped = { $count } फ़ोल्डर छोड़े गए
media-open-externally = सिस्टम प्लेयर में खोलें
media-play-pause = चलाएँ या रोकें
media-seek = आगे-पीछे करें
media-loop = दोहराएँ
media-volume = आवाज़
