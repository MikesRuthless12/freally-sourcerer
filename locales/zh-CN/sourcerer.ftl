# Sourcerer — 简体中文.

app-name = Sourcerer
tagline = 一次搜索,所有来源,所有系统。
window-title = Sourcerer
search-placeholder = 搜索…
about-version = 版本 { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = 就绪
status-indexed = 已索引({ $count } 个文件)
status-indexing = 正在索引… { $done }/{ $total }
status-paused = 已暂停
status-error = 错误
status-result-count-one = { $count } 个结果
status-result-count-many = { $count } 个结果
status-selection = · 已选择 { $count } 项
status-selection-size = 已选择:{ $size }
status-query-timing = 查询:{ $ms } 毫秒
status-endpoint-local = 本地数据库
status-endpoint-remote = API:{ $name }

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

parse-error-empty = 输入查询以开始搜索。
parse-error-unknown = 此处语法无法识别。

action-open = 打开
action-reveal = 在文件夹中显示
action-copy-path = 复制路径
action-copy-name = 复制名称
action-delete = 删除

quick-filter-audio = 音频
quick-filter-video = 视频
quick-filter-image = 图片
quick-filter-document = 文档
quick-filter-executable = 可执行文件
quick-filter-archive = 压缩包

wizard-title = 欢迎使用 Sourcerer
wizard-step-roots = 选择要索引的内容
wizard-step-hotkey = 选择全局热键
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
settings-node-indexes-top = (顶层)
settings-node-volumes = 卷
settings-node-folders = 文件夹
settings-node-file-lists = 文件列表
settings-node-exclude = 排除
settings-node-https-server = HTTP / HTTPS 服务器
settings-node-etp-api = ETP / FTP API
settings-node-privacy = 隐私与更新
settings-node-logs = 日志与调试
settings-node-backup = 备份、导出、重置
settings-node-locale = 区域
settings-node-about = 关于

# §8.2 General → UI.
settings-ui-theme = 主题
settings-ui-run-bg = 后台运行
settings-ui-show-tray = 显示托盘 / 菜单栏图标
settings-ui-single-click-tray = 单击托盘 / 菜单栏
settings-ui-new-window-from-tray = 从托盘图标打开新窗口
settings-ui-new-window-on-launch = 启动 Sourcerer 时打开新窗口
settings-ui-search-as-you-type = 即时搜索
settings-ui-select-on-mouse-click = 鼠标点击时选中搜索框
settings-ui-focus-on-activate = 激活时聚焦搜索框
settings-ui-full-row-select = 整行选择
settings-ui-single-click-open = 单击打开
settings-ui-underline-titles = 图标标题加下划线
settings-ui-row-density = 结果密度
settings-ui-row-density-compact = 紧凑(32 像素)
settings-ui-row-density-comfortable = 舒适(44 像素)
settings-ui-show-timing-badges = 显示各透镜耗时标记
settings-ui-anim-crossfade = 主题切换动画过渡

# §8.3 General → Home.
settings-home-match-case = 区分大小写
settings-home-match-whole-word = 全字匹配
settings-home-match-path = 匹配路径
settings-home-match-diacritics = 区分变音符号
settings-home-match-regex = 启用 Regex
settings-home-search = 搜索(自定义默认查询)
settings-home-filter = 筛选
settings-home-sort = 排序
settings-home-view = 视图
settings-home-index = 索引
settings-home-default-lens-visibility = 默认透镜可见性
settings-home-default-lens-result-limits = 默认透镜结果数量上限

# §8.4 General → Search.
settings-search-fast-ascii = 快速 ASCII 搜索
settings-search-mp-sep = 当搜索词包含路径分隔符时匹配路径
settings-search-mw-fn = 使用通配符时匹配整个文件名
settings-search-lit-ops = 允许字面操作符
settings-search-paren = 允许圆括号分组
settings-search-env = 展开环境变量
settings-search-fwd-slash = 将正斜杠替换为反斜杠
settings-search-precedence = 操作符优先级
settings-search-strict-everything = 严格 Everything 语法模式
settings-search-auto-regex = 自动检测 Regex
settings-search-mod-comp = 修饰符自动补全
settings-search-parse-tree = 鼠标悬停时显示语法树

# §8.5 General → Results.
settings-results-hide-empty = 搜索为空时隐藏结果
settings-results-clear-on-search = 搜索时清除选择
settings-results-close-on-execute = 执行后关闭窗口
settings-results-dbl-path = 在路径列双击打开路径
settings-results-auto-scroll = 自动滚动视图
settings-results-dquote-copy = 复制时为路径加双引号
settings-results-no-ext-rename = 重命名时不选中扩展名
settings-results-sort-date-desc = 日期默认按降序排序
settings-results-sort-size-desc = 大小默认按降序排序
settings-results-list-focus = 结果列表焦点
settings-results-icon-prio = 图标加载优先级
settings-results-thumb-prio = 缩略图加载优先级
settings-results-ext-prio = 扩展信息加载优先级
settings-results-group-by-lens = 按透镜分组结果
settings-results-snippet-inline = 行内显示片段预览

# §8.6 General → View.
settings-view-double-buffer = 双缓冲
settings-view-alt-rows = 隔行变色
settings-view-row-mouseover = 显示鼠标悬停高亮
settings-view-highlight-terms = 高亮显示搜索词
settings-view-status-show-selected = 在状态栏显示选中项
settings-view-rc-with-sel = 在结果数后显示已选数量
settings-view-status-show-size = 在状态栏显示大小
settings-view-tooltips = 显示工具提示
settings-view-update-on-scroll = 滚动后立即更新显示
settings-view-size-format = 大小格式
settings-view-selection-rect = 选择矩形
settings-view-audio-badges = 在音频行显示 LUFS / codec / 时长标记
settings-view-similarity-score = 在相似度行显示 MinHash 相似度分数
settings-view-preview-pane = 预览窗格

# §8.7 General → Context Menu.
settings-context-menu-visibility = 可见性
settings-context-menu-show = 显示
settings-context-menu-shift = 仅在按住 Shift 时显示
settings-context-menu-hide = 隐藏
settings-context-menu-command = 命令宏
settings-context-menu-open-folders = 打开(文件夹)
settings-context-menu-open-files = 打开(文件)
settings-context-menu-open-path = 打开路径
settings-context-menu-explore = 浏览
settings-context-menu-explore-path = 浏览路径
settings-context-menu-copy-name = 复制名称到剪贴板
settings-context-menu-copy-path = 复制路径到剪贴板
settings-context-menu-copy-full-name = 复制完整名称到剪贴板
settings-context-menu-reveal = 在 Sourcerer 中显示
settings-context-menu-send-to = 发送到 Sourcerer(路径)

# §8.8 General → Fonts & Colors.
settings-fc-font = 字体
settings-fc-size = 字号
settings-fc-state-normal = 普通
settings-fc-state-highlighted = 高亮
settings-fc-state-current-sort = 当前排序
settings-fc-state-current-sort-h = 当前排序(高亮)
settings-fc-state-selected = 已选中
settings-fc-state-selected-h = 已选中(高亮)
settings-fc-state-inactive-selected = 非活动已选中
settings-fc-state-inactive-selected-h = 非活动已选中(高亮)
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
settings-history-search-enable = 启用搜索历史
settings-history-search-keep = 保留搜索历史 { $days } 天
settings-history-run-enable = 启用运行历史
settings-history-run-keep = 保留运行历史 { $days } 天
settings-history-clear-now = 立即清除
settings-history-privacy-mode = 隐私模式
settings-history-per-lens = 按透镜区分历史

# §8.11 Indexes (top-level).
settings-ix-database-location = 数据库位置
settings-ix-multiuser = 多用户数据库文件名
settings-ix-compress = 压缩数据库
settings-ix-recent-changes = 索引最近的变更
settings-ix-file-size = 索引文件大小
settings-ix-fast-size-sort = 大小快速排序
settings-ix-folder-size = 索引文件夹大小
settings-ix-fast-folder-size-sort = 文件夹大小快速排序
settings-ix-date-created = 索引创建日期
settings-ix-fast-date-created = 创建日期快速排序
settings-ix-date-modified = 索引修改日期
settings-ix-fast-date-modified = 修改日期快速排序
settings-ix-date-accessed = 索引访问日期
settings-ix-fast-date-accessed = 访问日期快速排序
settings-ix-attributes = 索引属性
settings-ix-fast-attributes = 属性快速排序
settings-ix-fast-path-sort = 路径快速排序
settings-ix-fast-extension-sort = 扩展名快速排序
settings-ix-force-rebuild = 强制重建
settings-ix-compact = 压缩索引
settings-ix-verify = 校验索引
settings-ix-integrity-policy = 索引完整性策略
settings-ix-memory-budget = 索引器内存预算
settings-ix-throttle = 后台索引节流

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = 自动包含新的固定卷
settings-vol-auto-removable = 自动包含新的可移动卷
settings-vol-auto-remove-offline = 自动移除离线卷
settings-vol-detected = 检测到的卷
settings-vol-include = 包含到索引中
settings-vol-include-only = 仅包含(glob/regex)
settings-vol-enable-usn = 启用 USN 日志
settings-vol-enable-fsevents = 启用 FSEvents 流
settings-vol-enable-inotify = 启用 inotify(具有提权时使用 fanotify)
settings-vol-buffer = 日志缓冲区大小(KB)
settings-vol-allocation-delta = 分配增量(KB)
settings-vol-load-recent = 启动时从日志加载最近的变更
settings-vol-monitor = 监视变更
settings-vol-recreate-journal = 重建日志
settings-vol-reset-stream = 重置 FSEvents 流
settings-vol-upgrade-fanotify = 升级到 fanotify(polkit)
settings-vol-remove = 移除

# §8.13 Indexes → Folders.
settings-folders-watched = 监视的文件夹
settings-folders-add = 添加…
settings-folders-rescan-now = 立即重新扫描
settings-folders-rescan-all = 立即重新扫描全部
settings-folders-monitor = 尝试监视变更
settings-folders-buffer = 缓冲区大小
settings-folders-rescan-on-full = 缓冲区满时重新扫描

# §8.14 Indexes → File Lists.
settings-flists-add = 添加…
settings-flists-monitor = 监视变更
settings-flists-editor = 文件列表编辑器…
settings-flists-format = 文件列表格式
settings-flists-format-text = 文本(每行一个路径)
settings-flists-format-json = JSON(包含元数据)
settings-flists-format-srcb = Sourcerer 包(.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = 排除隐藏文件和文件夹
settings-exclude-system = 排除系统文件和文件夹
settings-exclude-list-en = 启用排除列表
settings-exclude-folders = 排除的文件夹
settings-exclude-include-only-files = 仅包含的文件(glob)
settings-exclude-files = 排除的文件(glob)
settings-exclude-os-recommended = 应用系统推荐的排除项
settings-exclude-by-class = 按扩展名类别排除

# §8.16 Lenses → Filename.
settings-lf-trigram = trigram 预筛选强度
settings-lf-suffix-mem = 后缀数组内存预算
settings-lf-wildcard-limit = 通配符展开上限
settings-lf-regex-timeout = Regex 超时

# §8.17 Lenses → Content.
settings-lc-enable = 启用内容透镜
settings-lc-time-budget = 单文档时间预算
settings-lc-mem-ceiling = 单文档内存上限
settings-lc-snippet-len = 片段长度
settings-lc-stop-words = 停用词语言
settings-lc-re-extract = 设置变更后重新提取
settings-lc-verify-blobs = 读取时校验提取文本 blob 的校验和

# §8.18 Lenses → Audio.
settings-la-enable = 启用音频透镜
settings-la-lufs-ref = LUFS 参考标准
settings-la-peak-compute = 峰值计算方式
settings-la-silence-thresh = 静音阈值
settings-la-re-extract-modify = 修改事件后重新提取

# §8.19 Lenses → Similarity.
settings-ls-enable = 启用相似度透镜
settings-ls-sig-size = MinHash 签名大小(k)
settings-ls-bands = LSH 分带数
settings-ls-recall = 召回阈值
settings-ls-result-cap = 结果上限

# §8.20 Lenses → Custom.
settings-custom-registry = 注册表
settings-custom-trust = 信任
settings-custom-refresh-hashes = 刷新哈希

# §8.21-§8.22 Network.
settings-net-https-enable = 启用 HTTPS 服务器
settings-net-bind = 绑定接口
settings-net-port = 监听端口
settings-net-force-https = 强制 HTTPS
settings-net-legacy-auth = 旧式 HTTP basic 认证
settings-net-token-regen = 重新生成令牌
settings-net-api-enable = 启用 API 服务器
settings-net-legacy-ftp = 支持旧式明文 FTP/ETP

# §8.23 Privacy & Updates.
settings-privacy-auto-update = 自动更新
settings-privacy-prerelease = 预发布通道
settings-privacy-network-policy = 网络调用策略

# §8.24 Logs & Debug.
settings-logs-level = 日志级别
settings-logs-location = 日志文件位置
settings-logs-retention = 日志保留期
settings-logs-debug-overlay = 显示调试浮层
settings-logs-open-folder = 打开日志文件夹
settings-logs-export-bundle = 导出诊断包

# §8.25 Backup, Export, Reset.
settings-backup-export = 导出设置
settings-backup-import = 导入设置
settings-backup-export-bookmarks = 导出书签包
settings-backup-import-bookmarks = 导入书签包
settings-backup-reset-all = 将所有设置重置为默认值

# §8.26 Locale.
settings-locale-current = 当前区域
settings-locale-rtl-preview = RTL 预览
settings-locale-date-format = 日期格式
settings-locale-number-format = 数字格式

# §8.27 About.
settings-about-version = Sourcerer { $version }
settings-about-license = 许可证
settings-about-credits = 致谢
settings-about-notices = 开源声明
