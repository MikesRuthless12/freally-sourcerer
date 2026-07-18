# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = 一次搜索，搜遍所有来源，畅行所有系统。
window-title = Freally Sourcerer
search-placeholder = 搜索…
about-version = 版本 { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = 就绪
status-indexed = 已建立索引（{ $count } 个文件）
status-indexing = 正在建立索引… { $done }/{ $total }
status-paused = 已暂停
status-error = 错误
status-result-count-one = { $count } 个结果
status-result-count-many = { $count } 个结果
status-selection = · 已选择 { $count } 项
status-selection-size = 已选择：{ $size }
status-query-timing = 查询：{ $ms } 毫秒
status-endpoint-local = 本地数据库
status-endpoint-remote = API：{ $name }

menu-file = 文件
menu-edit = 编辑
menu-view = 视图
menu-search = 搜索
menu-bookmarks = 书签
menu-tools = 工具
menu-help = 帮助

theme-system = 跟随系统
theme-light = 浅色
theme-dark = 深色

lens-filename = 文件名
lens-content = 内容
lens-audio = 音频
lens-similarity = 相似度

parse-error-empty = 输入查询内容以开始。
parse-error-unknown = 此处附近存在无法识别的语法。

action-open = 打开
action-reveal = 在文件夹中显示
action-copy-path = 复制路径
action-copy-name = 复制名称
action-delete = 删除

quick-filter-audio = 音频
quick-filter-video = 视频
quick-filter-image = 图像
quick-filter-document = 文档
quick-filter-executable = 可执行文件
quick-filter-archive = 压缩包

wizard-title = 欢迎使用 Freally
wizard-step-roots = 选择要建立索引的内容
wizard-step-hotkey = 设置全局热键
wizard-step-locale = 选择语言
wizard-step-theme = 选择主题
wizard-finish = 完成

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = 选项
settings-search-placeholder = 搜索选项…
settings-restore-defaults = 恢复默认值
settings-ok = 确定
settings-cancel = 取消
settings-apply = 应用

# Tree nav groups (PRD §8.1.1).
settings-group-general = 常规
settings-group-indexes = 索引
settings-group-lenses = 透镜
settings-group-network = 网络

# Tree nav leaves.
settings-node-ui = 界面
settings-node-home = 主页
settings-node-search = 搜索
settings-node-results = 结果
settings-node-view = 视图
settings-node-context-menu = 右键菜单
settings-node-fonts-colors = 字体与颜色
settings-node-keyboard = 键盘
settings-node-history = 历史记录
settings-node-indexes-top = （顶层）
settings-node-volumes = 卷
settings-node-folders = 文件夹
settings-node-file-lists = 文件列表
settings-node-exclude = 排除
settings-node-https-server = HTTP / HTTPS 服务器
settings-node-etp-api = ETP / FTP API
settings-node-privacy = 隐私与更新
settings-node-logs = 日志与调试
settings-node-backup = 备份、导出与重置
settings-node-locale = 区域设置
settings-node-about = 关于

# §8.2 General → UI.
settings-ui-theme = 主题
settings-ui-run-bg = 在后台运行
settings-ui-show-tray = 显示托盘 / 菜单栏图标
settings-ui-single-click-tray = 单击托盘 / 菜单栏
settings-ui-new-window-from-tray = 从托盘图标打开新窗口
settings-ui-new-window-on-launch = 启动 Freally 时打开新窗口
settings-ui-search-as-you-type = 边输入边搜索
settings-ui-select-on-mouse-click = 鼠标单击时选择搜索内容
settings-ui-focus-on-activate = 激活时聚焦搜索框
settings-ui-full-row-select = 整行选择
settings-ui-single-click-open = 单击打开
settings-ui-underline-titles = 为图标标题添加下划线
settings-ui-row-density = 结果密度
settings-ui-row-density-compact = 紧凑（32 px）
settings-ui-row-density-comfortable = 舒适（44 px）
settings-ui-show-timing-badges = 显示各透镜的耗时标记
settings-ui-anim-crossfade = 主题切换淡入淡出动画

# §8.3 General → Home.
settings-home-match-case = 区分大小写
settings-home-match-whole-word = 全字匹配
settings-home-match-path = 匹配路径
settings-home-match-diacritics = 匹配变音符号
settings-home-match-regex = 匹配正则表达式
settings-home-search = 搜索（自定义默认查询）
settings-home-filter = 筛选
settings-home-sort = 排序
settings-home-view = 视图
settings-home-index = 索引
settings-home-default-lens-visibility = 默认透镜可见性
settings-home-default-lens-result-limits = 默认透镜结果上限

# §8.4 General → Search.
settings-search-fast-ascii = 快速 ASCII 搜索
settings-search-mp-sep = 当搜索词包含路径分隔符时匹配路径
settings-search-mw-fn = 使用通配符时匹配完整文件名
settings-search-lit-ops = 允许字面运算符
settings-search-paren = 允许圆括号分组
settings-search-env = 展开环境变量
settings-search-fwd-slash = 将正斜杠替换为反斜杠
settings-search-precedence = 运算符优先级
settings-search-strict-everything = 严格 Everything 语法模式
settings-search-auto-regex = 自动检测正则表达式
settings-search-mod-comp = 修饰符自动补全
settings-search-parse-tree = 悬停时显示语法分析树

# §8.5 General → Results.
settings-results-hide-empty = 搜索为空时隐藏结果
settings-results-clear-on-search = 搜索时清除选择
settings-results-close-on-execute = 执行后关闭窗口
settings-results-dbl-path = 在路径列中双击打开路径
settings-results-auto-scroll = 自动滚动视图
settings-results-dquote-copy = 复制时为路径加双引号
settings-results-no-ext-rename = 重命名时不选中扩展名
settings-results-sort-date-desc = 按日期排序时默认降序
settings-results-sort-size-desc = 按大小排序时默认降序
settings-results-list-focus = 结果列表焦点
settings-results-icon-prio = 图标加载优先级
settings-results-thumb-prio = 缩略图加载优先级
settings-results-ext-prio = 扩展信息加载优先级
settings-results-group-by-lens = 按透镜分组结果
settings-results-snippet-inline = 内联显示片段预览

# §8.6 General → View.
settings-view-double-buffer = 双重缓冲
settings-view-alt-rows = 隔行换色
settings-view-row-mouseover = 显示鼠标悬停行
settings-view-highlight-terms = 高亮显示搜索词
settings-view-status-show-selected = 在状态栏中显示选中项
settings-view-rc-with-sel = 同时显示结果数与选中数
settings-view-status-show-size = 在状态栏中显示大小
settings-view-tooltips = 显示工具提示
settings-view-update-on-scroll = 滚动后立即更新显示
settings-view-size-format = 大小格式
settings-view-selection-rect = 框选矩形
settings-view-audio-badges = 在音频行显示 LUFS / 编解码器 / 时长标记
settings-view-similarity-score = 在相似度行显示 MinHash 相似度得分
settings-view-preview-pane = 预览窗格

# §8.7 General → Context Menu.
settings-context-menu-visibility = 可见性
settings-context-menu-show = 显示
settings-context-menu-shift = 仅在按住 Shift 时显示
settings-context-menu-hide = 隐藏
settings-context-menu-command = 命令宏
settings-context-menu-open-folders = 打开（文件夹）
settings-context-menu-open-files = 打开（文件）
settings-context-menu-open-path = 打开路径
settings-context-menu-explore = 浏览
settings-context-menu-explore-path = 浏览路径
settings-context-menu-copy-name = 将名称复制到剪贴板
settings-context-menu-copy-path = 将路径复制到剪贴板
settings-context-menu-copy-full-name = 将完整名称复制到剪贴板
settings-context-menu-reveal = 在 Freally 中显示
settings-context-menu-send-to = 发送到 Freally（路径）

# §8.8 General → Fonts & Colors.
settings-fc-font = 字体
settings-fc-size = 大小
settings-fc-state-normal = 正常
settings-fc-state-highlighted = 高亮
settings-fc-state-current-sort = 当前排序
settings-fc-state-current-sort-h = 当前排序（高亮）
settings-fc-state-selected = 已选中
settings-fc-state-selected-h = 已选中（高亮）
settings-fc-state-inactive-selected = 非活动已选中
settings-fc-state-inactive-selected-h = 非活动已选中（高亮）
settings-fc-foreground = 前景色
settings-fc-background = 背景色
settings-fc-bold = 粗体
settings-fc-italic = 斜体
settings-fc-default = 默认
settings-fc-per-lens-accent = 各透镜强调色
settings-fc-theme-inherit = 切换主题时自动翻转自定义颜色

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = 全局热键
settings-keyboard-new-window = 新建窗口热键
settings-keyboard-show-window = 显示窗口热键
settings-keyboard-toggle-window = 切换窗口热键
settings-keyboard-show-commands = 显示包含以下内容的命令
settings-keyboard-add-chord = + 添加组合键
settings-keyboard-remove-chord = 移除

# §8.10 History.
settings-history-search-enable = 启用搜索历史记录
settings-history-search-keep = 保留搜索历史记录 { $days } 天
settings-history-run-enable = 启用运行历史记录
settings-history-run-keep = 保留运行历史记录 { $days } 天
settings-history-clear-now = 立即清除
settings-history-privacy-mode = 隐私模式
settings-history-per-lens = 各透镜历史记录

# §8.11 Indexes (top-level).
settings-ix-database-location = 数据库位置
settings-ix-multiuser = 多用户数据库文件名
settings-ix-compress = 压缩数据库
settings-ix-recent-changes = 为最近的更改建立索引
settings-ix-file-size = 为文件大小建立索引
settings-ix-fast-size-sort = 快速按大小排序
settings-ix-folder-size = 为文件夹大小建立索引
settings-ix-fast-folder-size-sort = 快速按文件夹大小排序
settings-ix-date-created = 为创建日期建立索引
settings-ix-fast-date-created = 快速按创建日期排序
settings-ix-date-modified = 为修改日期建立索引
settings-ix-fast-date-modified = 快速按修改日期排序
settings-ix-date-accessed = 为访问日期建立索引
settings-ix-fast-date-accessed = 快速按访问日期排序
settings-ix-attributes = 为属性建立索引
settings-ix-fast-attributes = 快速按属性排序
settings-ix-fast-path-sort = 快速按路径排序
settings-ix-fast-extension-sort = 快速按扩展名排序
settings-ix-force-rebuild = 强制重建
settings-ix-compact = 压实索引
settings-ix-verify = 校验索引
settings-ix-integrity-policy = 索引完整性策略
settings-ix-memory-budget = 索引器内存预算
settings-ix-throttle = 后台索引节流

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = 自动包含新的固定卷
settings-vol-auto-removable = 自动包含新的可移动卷
settings-vol-auto-remove-offline = 自动移除离线卷
settings-vol-detected = 检测到的卷
settings-vol-include = 包含在索引中
settings-vol-include-only = 仅包含（glob/正则）
settings-vol-enable-usn = 启用 USN 日志
settings-vol-enable-fsevents = 启用 FSEvents 流
settings-vol-enable-inotify = 启用 inotify（提权后则用 fanotify）
settings-vol-buffer = 日志缓冲区大小（KB）
settings-vol-allocation-delta = 分配增量（KB）
settings-vol-load-recent = 启动时从日志加载最近的更改
settings-vol-monitor = 监视更改
settings-vol-recreate-journal = 重新创建日志
settings-vol-reset-stream = 重置 FSEvents 流
settings-vol-upgrade-fanotify = 升级到 fanotify（polkit）
settings-vol-remove = 移除

# §8.13 Indexes → Folders.
settings-folders-watched = 监视的文件夹
settings-folders-add = 添加…
settings-folders-rescan-now = 立即重新扫描
settings-folders-rescan-all = 立即重新扫描全部
settings-folders-monitor = 尝试监视更改
settings-folders-buffer = 缓冲区大小
settings-folders-rescan-on-full = 缓冲区满时重新扫描

# §8.14 Indexes → File Lists.
settings-flists-add = 添加…
settings-flists-monitor = 监视更改
settings-flists-editor = 文件列表编辑器…
settings-flists-format = 文件列表格式
settings-flists-format-text = 文本（每行一个路径）
settings-flists-format-json = JSON（含元数据）
settings-flists-format-srcb = Freally 捆绑包（.srcb）

# §8.15 Indexes → Exclude.
settings-exclude-hidden = 排除隐藏的文件和文件夹
settings-exclude-system = 排除系统文件和文件夹
settings-exclude-list-en = 启用排除列表
settings-exclude-folders = 排除文件夹
settings-exclude-include-only-files = 仅包含文件（glob）
settings-exclude-files = 排除文件（glob）
settings-exclude-os-recommended = 应用操作系统推荐的排除项
settings-exclude-by-class = 按扩展名类别排除

# §8.16 Lenses → Filename.
settings-lf-trigram = 三元组预筛选强度
settings-lf-suffix-mem = 后缀数组内存预算
settings-lf-wildcard-limit = 通配符展开上限
settings-lf-regex-timeout = 正则表达式超时

# §8.17 Lenses → Content.
settings-lc-enable = 启用内容透镜
settings-lc-time-budget = 每个文档的时间预算
settings-lc-mem-ceiling = 每个文档的内存上限
settings-lc-snippet-len = 片段长度
settings-lc-stop-words = 停用词语言
settings-lc-re-extract = 设置更改时重新提取
settings-lc-verify-blobs = 读取时校验已提取文本块的校验和

# §8.18 Lenses → Audio.
settings-la-enable = 启用音频透镜
settings-la-lufs-ref = LUFS 参考标准
settings-la-peak-compute = 峰值计算方式
settings-la-silence-thresh = 静音阈值
settings-la-re-extract-modify = 发生修改事件时重新提取

# §8.19 Lenses → Similarity.
settings-ls-enable = 启用相似度透镜
settings-ls-sig-size = MinHash 签名大小（k）
settings-ls-bands = LSH 频带数
settings-ls-recall = 召回阈值
settings-ls-result-cap = 结果上限

# §8.20 Lenses → Custom.
settings-custom-registry = 注册表
settings-custom-trust = 信任
settings-custom-refresh-hashes = 刷新哈希值

# §8.21-§8.22 Network.
settings-net-https-enable = 启用 HTTPS 服务器
settings-net-bind = 绑定到网络接口
settings-net-port = 监听端口
settings-net-force-https = 强制使用 HTTPS
settings-net-legacy-auth = 传统 HTTP 基本认证
settings-net-token-regen = 重新生成令牌
settings-net-api-enable = 启用 API 服务器
settings-net-legacy-ftp = 传统明文 FTP/ETP 支持

# §8.23 Privacy & Updates.
settings-privacy-auto-update = 自动更新
settings-privacy-prerelease = 预发布通道
settings-privacy-network-policy = 网络调用策略

# §8.24 Logs & Debug.
settings-logs-level = 日志级别
settings-logs-location = 日志文件位置
settings-logs-retention = 日志保留时长
settings-logs-debug-overlay = 显示调试浮层
settings-logs-open-folder = 打开日志文件夹
settings-logs-export-bundle = 导出诊断捆绑包

# §8.25 Backup, Export, Reset.
settings-backup-export = 导出设置
settings-backup-import = 导入设置
settings-backup-export-bookmarks = 导出书签捆绑包
settings-backup-import-bookmarks = 导入书签捆绑包
settings-backup-reset-all = 将所有设置重置为默认值

# §8.26 Locale.
settings-locale-current = 当前区域设置
settings-locale-rtl-preview = 从右至左预览
settings-locale-date-format = 日期格式
settings-locale-number-format = 数字格式

# §8.27 About.
settings-about-version = Freally { $version }
settings-about-license = 许可证
settings-about-credits = 鸣谢
settings-about-notices = 开源声明

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = 首次运行向导
wizard-step-of-total = 第 { $step } 步，共 { $total } 步
wizard-roots-hint = 添加你希望 Freally 监视的文件夹或卷。稍后可在“索引”设置中更改。
wizard-browse = 浏览…
wizard-roots-placeholder = …或粘贴路径
wizard-roots-add = 添加
wizard-roots-remove = 移除
wizard-roots-empty = 尚未配置任何根目录。
wizard-locale-hint = Freally 提供 18 种语言。稍后可随时切换。
wizard-theme-hint = “跟随系统”将采用操作系统的外观设置。
wizard-back = 上一步
wizard-next = 下一步

# Status bar polish.
statusbar-hotkey-hint = 热键：{ $hotkey }
statusbar-cycle-theme = 切换主题
statusbar-indexed-suffix = 已建立索引

# Results / lenses.
lens-expand = 展开透镜
lens-collapse = 折叠透镜
lens-no-matches = 此透镜中无匹配项。

# Preview pane.
preview-header = 预览
preview-loading = 正在加载…
preview-select-file = 选择一个文件以预览。
preview-unavailable = 无可用预览

# Bookmarks.
bookmarks-label = ★ 书签
bookmarks-empty-hint = 尚无书签。按 Ctrl+D 保存当前查询。
bookmarks-organize-title = 整理书签
bookmarks-organize-empty = 尚无书签。
bookmarks-rename = 重命名
bookmarks-close = 关闭

# Settings tree extras.
settings-group-history = 历史记录
settings-group-privacy = 隐私与更新
settings-group-logs = 日志与调试
settings-group-backup = 备份、导出与重置
settings-tree-custom-lens = 自定义
settings-unsaved-changes = 未保存的更改

# About dialog.
about-dialog-title = Freally
about-copyright = 版权所有 © 2026 Mike Weaver。保留所有权利。
about-close = 关闭

# Connect endpoint dialog.
connect-ftp-title = 连接到 FTP 服务器
connect-ftp-host = 主机：
connect-ftp-port = 端口：
connect-ftp-username = 用户名：
connect-ftp-password = 密码：
connect-ftp-link-type = 连接类型：

# UI panel.
ui-hint = 主题、托盘 / 菜单栏集成、边输入边搜索、行密度。与 voidtools-Everything 直接对等，并附带标有 (+) 的 Freally 新增功能。
ui-section-theme = 主题
ui-theme-system-default = 跟随系统（默认）
ui-section-tray = 托盘 / 菜单栏
ui-section-search-behavior = 搜索行为
ui-section-result-rows = 结果行
ui-single-click-system-default = 系统设置（默认）
ui-single-click-always = 始终单击
ui-single-click-always-double = 始终双击
ui-underline-always = 始终
ui-underline-on-hover = 悬停时
ui-underline-never = 从不

# Home panel.
home-hint = 应用启动时加载的默认值——每个下拉菜单都可固定为“使用上次的值”或固定的某个值。透镜可见性 / 结果上限为 Freally 新增功能 (+)。
home-section-match = 匹配默认值
home-section-search-sort = 搜索与排序默认值
home-search-placeholder = 默认为空
home-section-index = 索引来源
home-file-list-path = 文件列表路径
home-https-endpoint = HTTPS API 端点 URL
home-endpoint-token = 令牌（显示指纹）

# Backup panel.
backup-section-settings = 设置 (+)
backup-section-bookmarks = 书签 + 自定义提取器 (+)
backup-section-reset = 重置
backup-toast-exported = 已将设置导出到 { $path }
backup-toast-export-failed = 导出失败：{ $error }
backup-toast-imported = 已导入设置
backup-toast-import-failed = 导入失败：{ $error }
backup-toast-bookmarks-exported = 已导出书签
backup-toast-bookmarks-export-failed = 书签导出失败：{ $error }
backup-toast-bookmarks-imported = 已导入书签
backup-toast-bookmarks-import-failed = 书签导入失败：{ $error }
backup-confirm-reset = 将所有设置重置为默认值？此操作无法撤销（对话框将保持打开）。
backup-toast-reset = 已重置所有设置

# Keyboard panel.
keyboard-section-global = 全局热键
keyboard-placeholder-example = Super+Space
keyboard-section-commands = 命令
keyboard-placeholder-command = 命令 id（例如 file.export_results）
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = 搜索历史记录
history-section-run = 运行历史记录
history-section-privacy = 隐私 (+)
history-record-filename = 记录文件名透镜历史记录
history-record-content = 记录内容透镜历史记录
history-record-audio = 记录音频透镜历史记录
history-record-similarity = 记录相似度透镜历史记录

# Locale panel.
locale-section-language = 语言 (+)
locale-section-time-date = 时间 / 日期 (+)
locale-date-os = 操作系统默认
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = 自定义
locale-date-custom-format = 自定义格式
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = 数字 (+)
locale-number-os = 操作系统默认
locale-number-custom = 自定义
locale-thousands-sep = 千位分隔符
locale-decimal-sep = 小数点分隔符

# Folders panel.
folders-hint = 默认卷之外的其他监视文件夹。
folders-list-title = 监视的文件夹
folders-empty = 尚未添加任何文件夹。
folders-remove = 移除
folders-section-title-dynamic = { $path } 的设置
folders-section-schedule = 重新扫描计划
folders-schedule-daily = 每天 HH:MM
folders-schedule-hours = 每 N 小时
folders-schedule-never = 从不
folders-hour = 小时
folders-minute = 分钟
folders-hours = 小时
folders-id-label = 文件夹 ID（只读）
folders-select-prompt = 选择一个文件夹以进行配置。
folders-section-extras = Freally 附加功能 (+)
folders-extras-note = 在此版本中，从睡眠恢复时重新扫描默认已启用；该开关将在 Phase 13 的完善阶段并入文件夹级控件。

# Volumes panel.
volumes-hint = voidtools-Everything 的 NTFS / ReFS 面板的跨平台对应功能。自动检测 NTFS / ReFS / exFAT / FAT32（Win）、APFS / HFS+（macOS）、ext4 / Btrfs / ZFS / XFS / F2FS（Linux）。
volumes-section-auto-include = 自动包含
volumes-list-title = 检测到的卷
volumes-detecting = 正在检测…
volumes-empty = 未检测到任何卷。
volumes-select-prompt = 选择一个卷以进行配置。

# About panel polish.
about-section-version = 版本 (+)
about-section-license = 许可证 (+)
about-license-text = Mike Weaver — 保留所有权利。这是专有软件。
about-license-spdx = SPDX：{ $spdx }
about-section-credits = 鸣谢 (+)
about-credits-inspired = 灵感源自 voidtools 的 Everything。
about-credits-voidtools = voidtools.com
about-credits-repo = 项目仓库

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = 包含用于操作 Freally 的命令。
menu-file-new-window = 新建搜索窗口
menu-file-open-list = 打开文件列表…
menu-file-close-list = 关闭文件列表
menu-file-close = 关闭
menu-file-export-results = 导出结果…
menu-file-export-bundle = 导出索引捆绑包…
menu-file-exit = 退出

# Edit menu.
menu-edit-hint = 包含用于编辑搜索结果的命令。
menu-edit-cut = 剪切
menu-edit-copy = 复制
menu-edit-paste = 粘贴
menu-edit-copy-to-folder = 复制到文件夹…
menu-edit-move-to-folder = 移动到文件夹…
menu-edit-select-all = 全选
menu-edit-invert-selection = 反向选择
menu-edit-advanced = 高级
menu-edit-copy-full-name = 复制完整名称
menu-edit-copy-path = 复制路径
menu-edit-copy-filename = 复制文件名
menu-edit-copy-as-json = 复制为 JSON
menu-edit-copy-with-metadata = 连同元数据一起复制
menu-edit-copy-as-bundle-ref = 复制为 Freally 捆绑包引用

# View menu.
menu-view-hint = 包含用于操控视图的命令。
menu-view-filters = 筛选器
menu-view-preview = 预览
menu-view-status-bar = 状态栏
menu-view-thumbs-xl = 超大缩略图
menu-view-thumbs-l = 大缩略图
menu-view-thumbs-m = 中等缩略图
menu-view-details = 详细信息
menu-view-window-size = 窗口大小
menu-view-window-size-hint = 包含用于调整窗口大小的命令。
menu-view-window-small = 小
menu-view-window-medium = 中
menu-view-window-large = 大
menu-view-window-auto = 自动适应
menu-view-zoom = 缩放
menu-view-zoom-hint = 包含用于调整字体和图标大小的命令。
menu-view-zoom-in = 放大
menu-view-zoom-out = 缩小
menu-view-zoom-reset = 重置
menu-view-sort-by = 排序方式
menu-view-sort-by-hint = 包含用于对结果列表进行排序的命令。
menu-view-sort-name = 名称
menu-view-sort-path = 路径
menu-view-sort-size = 大小
menu-view-sort-ext = 扩展名
menu-view-sort-type = 类型
menu-view-sort-modified = 修改日期
menu-view-sort-created = 创建日期
menu-view-sort-accessed = 访问日期
menu-view-sort-attributes = 属性
menu-view-sort-recently-changed = 最近更改日期
menu-view-sort-run-count = 运行次数
menu-view-sort-run-date = 运行日期
menu-view-sort-file-list-filename = 文件列表文件名
menu-view-sort-lufs = LUFS
menu-view-sort-length = 时长
menu-view-sort-similarity = 相似度得分
menu-view-sort-asc = 升序
menu-view-sort-desc = 降序
menu-view-go-to = 转到
menu-view-refresh = 刷新
menu-view-theme = 主题
menu-view-theme-hint = 在跟随系统、浅色或深色主题之间切换。
menu-view-lenses = 透镜
menu-view-lenses-hint = 切换结果列表中各透镜的可见性。
menu-view-on-top = 置顶
menu-view-on-top-hint = 包含用于使此窗口保持在其他窗口之上的命令。
menu-view-on-top-never = 从不
menu-view-on-top-always = 始终
menu-view-on-top-while-searching = 搜索时

# Search menu.
menu-search-hint = 包含搜索开关。
menu-search-match-case = 区分大小写
menu-search-match-whole-word = 全字匹配
menu-search-match-path = 匹配路径
menu-search-match-diacritics = 匹配变音符号
menu-search-enable-regex = 启用正则表达式
menu-search-advanced = 高级搜索…
menu-search-add-to-filters = 添加到筛选器…
menu-search-organize-filters = 整理筛选器…
menu-search-filter-everything = 全部
menu-search-filter-archive = 已压缩（压缩包）
menu-search-filter-folder = 文件夹
menu-search-filter-custom = 自定义筛选器…

# Bookmarks menu.
menu-bookmarks-hint = 包含用于操作书签的命令。
menu-bookmarks-add = 添加到书签
menu-bookmarks-organize = 整理书签…

# Tools menu.
menu-tools-hint = 包含工具命令。
menu-tools-connect = 连接到 FTP 服务器…
menu-tools-disconnect = 从 FTP 服务器断开连接
menu-tools-file-list-editor = 文件列表编辑器…
menu-tools-index-maintenance = 索引维护
menu-tools-index-maintenance-hint = 索引维护工具。
menu-tools-verify-index = 校验索引…
menu-tools-compact-index = 压实索引…
menu-tools-rebuild-index = 强制重建索引…
menu-tools-custom-extractor = 自定义提取器管理器…
menu-tools-custom-extractor-hint = 管理 Wasm 沙箱化的自定义提取器。
menu-tools-options = 选项…

# Help menu.
menu-help-hint = 包含帮助命令。
menu-help-help = Freally 帮助
menu-help-search-syntax = 搜索语法
menu-help-regex-syntax = 正则表达式语法
menu-help-audio-ref = 音频修饰符参考
menu-help-similarity-ref = 相似度修饰符参考
menu-help-cli-options = 命令行选项
menu-help-website = Freally 网站
menu-help-check-updates = 检查更新…
menu-help-sponsor = 赞助 / 捐赠
menu-help-about = 关于 Freally…

# Result column headers (short forms used in the table header row).
column-name = 名称
column-path = 路径
column-size = 大小
column-modified = 修改日期
column-type = 类型
column-ext = 扩展名
column-sort-by = 按{ $name }排序
column-resize = 调整{ $name }列宽

# Section subtitle bars used inside multiple settings panels.
section-behavior = 行为
section-rendering = 渲染
section-status-bar = 状态栏
section-display-format = 显示格式
section-loading-priority = 加载优先级
section-compatibility = 兼容性
section-storage = 存储
section-index-fields = 索引字段
section-maintenance = 维护
section-logging = 日志记录
section-tools = 工具
section-privacy = 隐私
section-auto-update = 自动更新 (+)
section-bind = 绑定
section-lens = 透镜
section-budgets = 预算
section-other = 其他
section-per-format-mode = 各格式模式
section-loudness = 响度
section-tuning = 调优 (+)
section-minhash-lsh = MinHash + LSH 参数 (+)
section-top-level = 顶层
section-file-globs = 文件 glob
section-file-list-settings = 选定文件列表的设置
section-editor-format = 编辑器 + 格式（E + +）
section-api-server = API 服务器（E 改编）
section-freally-extras = Freally 附加功能 (+)
section-freally-additions = Freally 新增功能 (+)
section-freally-extensions = Freally 扩展功能 (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = 使用上次的值
opt-use-last-value-default = 使用上次的值（默认）
opt-low = 低
opt-normal-default = 正常（默认）
opt-high = 高
opt-disabled = 已禁用
opt-off = 关闭
opt-on-battery = 使用电池时
opt-always = 始终
opt-clamp-default = 钳制（默认）
opt-wrap = 环绕
opt-none = 无
opt-strict-refuse = 严格（损坏时拒绝查询）
opt-lenient-warn = 宽松（警告但仍查询）
opt-system-default = 系统默认
opt-drag-select = 拖动选择
opt-auto-binary = 自动（二进制）
opt-auto-decimal = 自动（十进制）

# Unit suffixes shown next to number inputs.
unit-days = 天
unit-b = B
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-tb = TB

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = 急切
opt-lazy-default = 延迟（默认）
opt-on = 开启
opt-on-default = 开启（默认）
opt-all = 全部
opt-weekly = 每周
opt-monthly = 每月
opt-name-asc = 名称升序
opt-name-desc = 名称降序
opt-size-asc = 大小升序
opt-size-desc = 大小降序
opt-modified-asc = 修改日期升序
opt-modified-desc = 修改日期降序
opt-compact = 紧凑
opt-comfortable = 舒适
opt-details = 详细信息
opt-thumbnails = 缩略图
opt-local-db-default = 本地数据库（默认）
opt-file-list = 文件列表
opt-https-endpoint = HTTPS API 端点
opt-right-default = 右侧（默认）
opt-bottom = 底部
opt-or-and-default = OR > AND（默认）
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128（默认）
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = 广播影片 (-23)
opt-true-peak = 真峰值（4× 过采样，默认）
opt-sample-peak = 采样峰值
opt-auto-per-doc = 自动（按文档）
opt-log-error = 错误
opt-log-warn = 警告
opt-log-info-default = 信息（默认）
opt-log-debug = 调试
opt-log-trace = 跟踪

# More Freally apps (Central inside panel) — host chrome
menu-help-more-apps = 更多 Freally 应用…
moreapps-title = 更多 Freally 应用
