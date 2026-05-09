# Sourcerer — 日本語.

app-name = Sourcerer
tagline = ひとつの検索で、あらゆるソースを、あらゆるOSで。
window-title = Sourcerer
search-placeholder = 検索…
about-version = バージョン { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = 準備完了
status-indexed = インデックス済み({ $count } ファイル)
status-indexing = インデックス作成中… { $done }/{ $total }
status-paused = 一時停止
status-error = エラー
status-result-count-one = { $count } 件の結果
status-result-count-many = { $count } 件の結果
status-selection = ・{ $count } 件選択中
status-selection-size = 選択中: { $size }
status-query-timing = クエリ: { $ms } ms
status-endpoint-local = ローカルDB
status-endpoint-remote = API: { $name }

menu-file = ファイル
menu-edit = 編集
menu-view = 表示
menu-search = 検索
menu-bookmarks = ブックマーク
menu-tools = ツール
menu-help = ヘルプ

theme-system = システム
theme-light = ライト
theme-dark = ダーク

lens-filename = ファイル名
lens-content = コンテンツ
lens-audio = オーディオ
lens-similarity = 類似度

parse-error-empty = クエリを入力してください。
parse-error-unknown = この付近の構文を認識できません。

action-open = 開く
action-reveal = フォルダーで表示
action-copy-path = パスをコピー
action-copy-name = 名前をコピー
action-delete = 削除

quick-filter-audio = オーディオ
quick-filter-video = ビデオ
quick-filter-image = 画像
quick-filter-document = ドキュメント
quick-filter-executable = 実行ファイル
quick-filter-archive = アーカイブ

wizard-title = Sourcerer へようこそ
wizard-step-roots = インデックスする対象を選択
wizard-step-hotkey = グローバルホットキーを選択
wizard-step-locale = 言語を選択
wizard-step-theme = テーマを選択
wizard-finish = 完了

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = オプション
settings-search-placeholder = オプションを検索…
settings-restore-defaults = デフォルトに戻す
settings-ok = OK
settings-cancel = キャンセル
settings-apply = 適用

# Tree nav groups (PRD §8.1.1).
settings-group-general = 一般
settings-group-indexes = インデックス
settings-group-lenses = レンズ
settings-group-network = ネットワーク

# Tree nav leaves.
settings-node-ui = UI
settings-node-home = ホーム
settings-node-search = 検索
settings-node-results = 結果
settings-node-view = 表示
settings-node-context-menu = コンテキストメニュー
settings-node-fonts-colors = フォントと配色
settings-node-keyboard = キーボード
settings-node-history = 履歴
settings-node-indexes-top = (トップレベル)
settings-node-volumes = ボリューム
settings-node-folders = フォルダー
settings-node-file-lists = ファイルリスト
settings-node-exclude = 除外
settings-node-https-server = HTTP / HTTPS サーバー
settings-node-etp-api = ETP / FTP API
settings-node-privacy = プライバシーと更新
settings-node-logs = ログとデバッグ
settings-node-backup = バックアップ・エクスポート・リセット
settings-node-locale = ロケール
settings-node-about = バージョン情報

# §8.2 General → UI.
settings-ui-theme = テーマ
settings-ui-run-bg = バックグラウンドで実行
settings-ui-show-tray = トレイ/メニューバーアイコンを表示
settings-ui-single-click-tray = トレイ/メニューバーをシングルクリックで開く
settings-ui-new-window-from-tray = トレイアイコンから新しいウィンドウを開く
settings-ui-new-window-on-launch = Sourcerer 起動時に新しいウィンドウを開く
settings-ui-search-as-you-type = 入力しながら検索
settings-ui-select-on-mouse-click = マウスクリックで検索文字列を選択
settings-ui-focus-on-activate = アクティブ化時に検索にフォーカス
settings-ui-full-row-select = 行全体を選択
settings-ui-single-click-open = シングルクリックで開く
settings-ui-underline-titles = アイコンタイトルに下線を表示
settings-ui-row-density = 結果の表示密度
settings-ui-row-density-compact = コンパクト (32 px)
settings-ui-row-density-comfortable = ゆったり (44 px)
settings-ui-show-timing-badges = レンズごとのタイミングバッジを表示
settings-ui-anim-crossfade = テーマ切替時のクロスフェードアニメーション

# §8.3 General → Home.
settings-home-match-case = 大文字と小文字を区別
settings-home-match-whole-word = 単語全体に一致
settings-home-match-path = パスに一致
settings-home-match-diacritics = ダイアクリティカルマークを区別
settings-home-match-regex = 正規表現で一致
settings-home-search = 検索(カスタムデフォルトクエリ)
settings-home-filter = フィルター
settings-home-sort = 並べ替え
settings-home-view = 表示
settings-home-index = インデックス
settings-home-default-lens-visibility = レンズの既定の表示設定
settings-home-default-lens-result-limits = レンズの既定の結果上限

# §8.4 General → Search.
settings-search-fast-ascii = 高速 ASCII 検索
settings-search-mp-sep = 検索語にパス区切り文字が含まれる場合はパスに一致
settings-search-mw-fn = ワイルドカード使用時はファイル名全体に一致
settings-search-lit-ops = 演算子をリテラルとして許可
settings-search-paren = 丸括弧によるグループ化を許可
settings-search-env = 環境変数を展開
settings-search-fwd-slash = スラッシュをバックスラッシュに置換
settings-search-precedence = 演算子の優先順位
settings-search-strict-everything = Everything 構文の厳密モード
settings-search-auto-regex = Regex を自動検出
settings-search-mod-comp = 修飾子の入力補完
settings-search-parse-tree = ホバー時にパースツリーを表示

# §8.5 General → Results.
settings-results-hide-empty = 検索が空のときは結果を非表示
settings-results-clear-on-search = 検索時に選択を解除
settings-results-close-on-execute = 実行時にウィンドウを閉じる
settings-results-dbl-path = パス列のダブルクリックでパスを開く
settings-results-auto-scroll = 表示を自動的にスクロール
settings-results-dquote-copy = ダブルクォートでパスとしてコピー
settings-results-no-ext-rename = 名前変更時に拡張子を選択しない
settings-results-sort-date-desc = 日付は降順を優先して並べ替え
settings-results-sort-size-desc = サイズは降順を優先して並べ替え
settings-results-list-focus = 結果リストのフォーカス
settings-results-icon-prio = アイコン読み込みの優先度
settings-results-thumb-prio = サムネイル読み込みの優先度
settings-results-ext-prio = 拡張情報の読み込み優先度
settings-results-group-by-lens = レンズで結果をグループ化
settings-results-snippet-inline = スニペットプレビューをインラインで表示

# §8.6 General → View.
settings-view-double-buffer = ダブルバッファリング
settings-view-alt-rows = 行の色を交互にする
settings-view-row-mouseover = 行のマウスオーバーを表示
settings-view-highlight-terms = 検索語をハイライト表示
settings-view-status-show-selected = ステータスバーに選択中の項目を表示
settings-view-rc-with-sel = 選択件数とともに結果件数を表示
settings-view-status-show-size = ステータスバーにサイズを表示
settings-view-tooltips = ツールチップを表示
settings-view-update-on-scroll = スクロール直後に表示を更新
settings-view-size-format = サイズの表示形式
settings-view-selection-rect = 選択矩形
settings-view-audio-badges = オーディオ行に LUFS / codec / 長さのバッジを表示
settings-view-similarity-score = 類似度行に MinHash 類似度スコアを表示
settings-view-preview-pane = プレビューペイン

# §8.7 General → Context Menu.
settings-context-menu-visibility = 表示
settings-context-menu-show = 表示する
settings-context-menu-shift = Shift キー押下時のみ表示
settings-context-menu-hide = 非表示
settings-context-menu-command = コマンドマクロ
settings-context-menu-open-folders = 開く(フォルダー)
settings-context-menu-open-files = 開く(ファイル)
settings-context-menu-open-path = パスを開く
settings-context-menu-explore = エクスプローラーで開く
settings-context-menu-explore-path = パスをエクスプローラーで開く
settings-context-menu-copy-name = 名前をクリップボードにコピー
settings-context-menu-copy-path = パスをクリップボードにコピー
settings-context-menu-copy-full-name = フルネームをクリップボードにコピー
settings-context-menu-reveal = Sourcerer で表示
settings-context-menu-send-to = Sourcerer に送信(パス)

# §8.8 General → Fonts & Colors.
settings-fc-font = フォント
settings-fc-size = サイズ
settings-fc-state-normal = 通常
settings-fc-state-highlighted = ハイライト
settings-fc-state-current-sort = 現在のソート
settings-fc-state-current-sort-h = 現在のソート(ハイライト)
settings-fc-state-selected = 選択中
settings-fc-state-selected-h = 選択中(ハイライト)
settings-fc-state-inactive-selected = 非アクティブ選択
settings-fc-state-inactive-selected-h = 非アクティブ選択(ハイライト)
settings-fc-foreground = 前景色
settings-fc-background = 背景色
settings-fc-bold = 太字
settings-fc-italic = 斜体
settings-fc-default = デフォルト
settings-fc-per-lens-accent = レンズごとのアクセントカラー
settings-fc-theme-inherit = テーマ切替時にカスタムカラーを自動反転

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = グローバルホットキー
settings-keyboard-new-window = 新しいウィンドウのホットキー
settings-keyboard-show-window = ウィンドウ表示のホットキー
settings-keyboard-toggle-window = ウィンドウ切替のホットキー
settings-keyboard-show-commands = 含まれるコマンドを表示
settings-keyboard-add-chord = + コードを追加
settings-keyboard-remove-chord = 削除

# §8.10 History.
settings-history-search-enable = 検索履歴を有効化
settings-history-search-keep = 検索履歴を { $days } 日間保持
settings-history-run-enable = 実行履歴を有効化
settings-history-run-keep = 実行履歴を { $days } 日間保持
settings-history-clear-now = 今すぐクリア
settings-history-privacy-mode = プライバシーモード
settings-history-per-lens = レンズごとの履歴

# §8.11 Indexes (top-level).
settings-ix-database-location = データベースの場所
settings-ix-multiuser = マルチユーザーデータベースのファイル名
settings-ix-compress = データベースを圧縮
settings-ix-recent-changes = 最近の変更をインデックス
settings-ix-file-size = ファイルサイズをインデックス
settings-ix-fast-size-sort = 高速サイズソート
settings-ix-folder-size = フォルダーサイズをインデックス
settings-ix-fast-folder-size-sort = 高速フォルダーサイズソート
settings-ix-date-created = 作成日時をインデックス
settings-ix-fast-date-created = 高速作成日時ソート
settings-ix-date-modified = 更新日時をインデックス
settings-ix-fast-date-modified = 高速更新日時ソート
settings-ix-date-accessed = アクセス日時をインデックス
settings-ix-fast-date-accessed = 高速アクセス日時ソート
settings-ix-attributes = 属性をインデックス
settings-ix-fast-attributes = 高速属性ソート
settings-ix-fast-path-sort = 高速パスソート
settings-ix-fast-extension-sort = 高速拡張子ソート
settings-ix-force-rebuild = 強制再構築
settings-ix-compact = インデックスを最適化
settings-ix-verify = インデックスを検証
settings-ix-integrity-policy = インデックス整合性ポリシー
settings-ix-memory-budget = インデクサーのメモリ予算
settings-ix-throttle = バックグラウンドインデックスのスロットリング

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = 新しい固定ボリュームを自動的に追加
settings-vol-auto-removable = 新しいリムーバブルボリュームを自動的に追加
settings-vol-auto-remove-offline = オフラインのボリュームを自動的に削除
settings-vol-detected = 検出されたボリューム
settings-vol-include = インデックスに含める
settings-vol-include-only = 次のみを含める(glob/regex)
settings-vol-enable-usn = USN ジャーナルを有効化
settings-vol-enable-fsevents = FSEvents ストリームを有効化
settings-vol-enable-inotify = inotify を有効化(昇格時は fanotify)
settings-vol-buffer = ジャーナルバッファサイズ (KB)
settings-vol-allocation-delta = 割り当てデルタ (KB)
settings-vol-load-recent = 起動時にジャーナルから最近の変更を読み込む
settings-vol-monitor = 変更を監視
settings-vol-recreate-journal = ジャーナルを再作成
settings-vol-reset-stream = FSEvents ストリームをリセット
settings-vol-upgrade-fanotify = fanotify にアップグレード(polkit)
settings-vol-remove = 削除

# §8.13 Indexes → Folders.
settings-folders-watched = 監視中のフォルダー
settings-folders-add = 追加…
settings-folders-rescan-now = 今すぐ再スキャン
settings-folders-rescan-all = すべて今すぐ再スキャン
settings-folders-monitor = 変更の監視を試みる
settings-folders-buffer = バッファサイズ
settings-folders-rescan-on-full = バッファ満杯時に再スキャン

# §8.14 Indexes → File Lists.
settings-flists-add = 追加…
settings-flists-monitor = 変更を監視
settings-flists-editor = ファイルリストエディター…
settings-flists-format = ファイルリストの形式
settings-flists-format-text = テキスト(1行に1パス)
settings-flists-format-json = JSON(メタデータ付き)
settings-flists-format-srcb = Sourcerer バンドル (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = 隠しファイルとフォルダーを除外
settings-exclude-system = システムファイルとフォルダーを除外
settings-exclude-list-en = 除外リストを有効化
settings-exclude-folders = フォルダーを除外
settings-exclude-include-only-files = 次のファイルのみを含める(glob)
settings-exclude-files = ファイルを除外(glob)
settings-exclude-os-recommended = OS 推奨の除外設定を適用
settings-exclude-by-class = 拡張子クラスで除外

# §8.16 Lenses → Filename.
settings-lf-trigram = trigram プリフィルターの強度
settings-lf-suffix-mem = サフィックス配列のメモリ予算
settings-lf-wildcard-limit = ワイルドカード展開の上限
settings-lf-regex-timeout = Regex タイムアウト

# §8.17 Lenses → Content.
settings-lc-enable = コンテンツレンズを有効化
settings-lc-time-budget = ドキュメントごとの時間予算
settings-lc-mem-ceiling = ドキュメントごとのメモリ上限
settings-lc-snippet-len = スニペットの長さ
settings-lc-stop-words = ストップワードの言語
settings-lc-re-extract = 設定変更時に再抽出
settings-lc-verify-blobs = 抽出テキスト blob のチェックサムを読み込み時に検証

# §8.18 Lenses → Audio.
settings-la-enable = オーディオレンズを有効化
settings-la-lufs-ref = LUFS 基準規格
settings-la-peak-compute = ピークの算出方法
settings-la-silence-thresh = 無音しきい値
settings-la-re-extract-modify = Modify イベント発生時に再抽出

# §8.19 Lenses → Similarity.
settings-ls-enable = 類似度レンズを有効化
settings-ls-sig-size = MinHash シグネチャサイズ (k)
settings-ls-bands = LSH バンド数
settings-ls-recall = 再現率しきい値
settings-ls-result-cap = 結果の上限

# §8.20 Lenses → Custom.
settings-custom-registry = レジストリ
settings-custom-trust = 信頼設定
settings-custom-refresh-hashes = ハッシュを更新

# §8.21-§8.22 Network.
settings-net-https-enable = HTTPS サーバーを有効化
settings-net-bind = バインドするインターフェイス
settings-net-port = リッスンするポート
settings-net-force-https = HTTPS を強制
settings-net-legacy-auth = レガシー HTTP ベーシック認証
settings-net-token-regen = トークンを再生成
settings-net-api-enable = API サーバーを有効化
settings-net-legacy-ftp = レガシー平文 FTP/ETP のサポート

# §8.23 Privacy & Updates.
settings-privacy-auto-update = 自動更新
settings-privacy-prerelease = プレリリースチャンネル
settings-privacy-network-policy = ネットワーク通信ポリシー

# §8.24 Logs & Debug.
settings-logs-level = ログレベル
settings-logs-location = ログファイルの場所
settings-logs-retention = ログの保持期間
settings-logs-debug-overlay = デバッグオーバーレイを表示
settings-logs-open-folder = ログフォルダーを開く
settings-logs-export-bundle = 診断バンドルをエクスポート

# §8.25 Backup, Export, Reset.
settings-backup-export = 設定をエクスポート
settings-backup-import = 設定をインポート
settings-backup-export-bookmarks = ブックマークバンドルをエクスポート
settings-backup-import-bookmarks = ブックマークバンドルをインポート
settings-backup-reset-all = すべての設定を既定値にリセット

# §8.26 Locale.
settings-locale-current = 現在のロケール
settings-locale-rtl-preview = RTL プレビュー
settings-locale-date-format = 日付形式
settings-locale-number-format = 数値形式

# §8.27 About.
settings-about-version = Sourcerer { $version }
settings-about-license = ライセンス
settings-about-credits = クレジット
settings-about-notices = オープンソースの告知
