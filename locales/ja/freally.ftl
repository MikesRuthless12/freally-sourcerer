# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = ひとつの検索で、あらゆるソースを、あらゆるOSで。
window-title = Freally Sourcerer
search-placeholder = 検索…
about-version = バージョン { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = 準備完了
status-indexed = インデックス済み（{ $count } 件）
status-indexing = インデックス作成中… { $done }/{ $total }
status-paused = 一時停止中
status-error = エラー
status-result-count-one = { $count } 件の結果
status-result-count-many = { $count } 件の結果
status-selection = · { $count } 件選択中
status-selection-size = 選択中: { $size }
status-query-timing = クエリ: { $ms } ミリ秒
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

parse-error-empty = クエリを入力して開始します。
parse-error-unknown = この付近に認識できない構文があります。

action-open = 開く
action-reveal = フォルダー内に表示
action-copy-path = パスをコピー
action-copy-name = 名前をコピー
action-delete = 削除

quick-filter-audio = オーディオ
quick-filter-video = ビデオ
quick-filter-image = 画像
quick-filter-document = ドキュメント
quick-filter-executable = 実行ファイル
quick-filter-archive = アーカイブ

wizard-title = Freally へようこそ
wizard-step-roots = インデックス対象を選択
wizard-step-hotkey = グローバルホットキーを選択
wizard-step-locale = 言語を選択
wizard-step-theme = テーマを選択
wizard-finish = 完了

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = オプション
settings-search-placeholder = オプションを検索…
settings-restore-defaults = 既定値に戻す
settings-ok = OK
settings-cancel = キャンセル
settings-apply = 適用

# Tree nav groups (PRD §8.1.1).
settings-group-general = 全般
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
settings-node-fonts-colors = フォントと色
settings-node-keyboard = キーボード
settings-node-history = 履歴
settings-node-indexes-top = （最上位）
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
settings-ui-show-tray = トレイ／メニューバーアイコンを表示
settings-ui-single-click-tray = トレイ／メニューバーをシングルクリック
settings-ui-new-window-from-tray = トレイアイコンから新しいウィンドウを開く
settings-ui-new-window-on-launch = Freally の起動時に新しいウィンドウを開く
settings-ui-search-as-you-type = 入力に合わせて検索
settings-ui-select-on-mouse-click = マウスクリックで検索文字列を選択
settings-ui-focus-on-activate = アクティブ化時に検索にフォーカス
settings-ui-full-row-select = 行全体を選択
settings-ui-single-click-open = シングルクリックで開く
settings-ui-underline-titles = アイコンのタイトルに下線を表示
settings-ui-row-density = 結果の表示密度
settings-ui-row-density-compact = コンパクト（32 px）
settings-ui-row-density-comfortable = ゆったり（44 px）
settings-ui-show-timing-badges = レンズごとに処理時間バッジを表示
settings-ui-anim-crossfade = テーマ切り替え時にクロスフェードアニメーション

# §8.3 General → Home.
settings-home-match-case = 大文字と小文字を区別
settings-home-match-whole-word = 単語全体に一致
settings-home-match-path = パスに一致
settings-home-match-diacritics = ダイアクリティカルマークを区別
settings-home-match-regex = 正規表現に一致
settings-home-search = 検索（既定のカスタムクエリ）
settings-home-filter = フィルター
settings-home-sort = 並べ替え
settings-home-view = 表示
settings-home-index = インデックス
settings-home-default-lens-visibility = レンズの既定の表示状態
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
settings-search-strict-everything = 厳格な Everything 構文モード
settings-search-auto-regex = 正規表現を自動検出
settings-search-mod-comp = 修飾子の補完
settings-search-parse-tree = ホバー時に構文ツリーを表示

# §8.5 General → Results.
settings-results-hide-empty = 検索が空のときは結果を非表示
settings-results-clear-on-search = 検索時に選択を解除
settings-results-close-on-execute = 実行時にウィンドウを閉じる
settings-results-dbl-path = パス列のダブルクリックでパスを開く
settings-results-auto-scroll = ビューを自動的にスクロール
settings-results-dquote-copy = ダブルクォートでパスとしてコピー
settings-results-no-ext-rename = 名前の変更時に拡張子を選択しない
settings-results-sort-date-desc = 日付は最初に降順で並べ替え
settings-results-sort-size-desc = サイズは最初に降順で並べ替え
settings-results-list-focus = 結果リストのフォーカス
settings-results-icon-prio = アイコン読み込みの優先度
settings-results-thumb-prio = サムネイル読み込みの優先度
settings-results-ext-prio = 拡張情報読み込みの優先度
settings-results-group-by-lens = 結果をレンズごとにグループ化
settings-results-snippet-inline = スニペットのプレビューをインライン表示

# §8.6 General → View.
settings-view-double-buffer = ダブルバッファリング
settings-view-alt-rows = 行の色を交互に表示
settings-view-row-mouseover = 行のマウスオーバーを表示
settings-view-highlight-terms = 検索語をハイライト表示
settings-view-status-show-selected = ステータスバーに選択項目を表示
settings-view-rc-with-sel = 結果件数と選択件数を併せて表示
settings-view-status-show-size = ステータスバーにサイズを表示
settings-view-tooltips = ツールチップを表示
settings-view-update-on-scroll = スクロール後すぐに表示を更新
settings-view-size-format = サイズの表示形式
settings-view-selection-rect = 選択矩形
settings-view-audio-badges = オーディオ行に LUFS／コーデック／長さのバッジを表示
settings-view-similarity-score = 類似度行に MinHash 類似度スコアを表示
settings-view-preview-pane = プレビューペイン

# §8.7 General → Context Menu.
settings-context-menu-visibility = 表示状態
settings-context-menu-show = 表示
settings-context-menu-shift = Shift キー押下時のみ表示
settings-context-menu-hide = 非表示
settings-context-menu-command = コマンドマクロ
settings-context-menu-open-folders = 開く（フォルダー）
settings-context-menu-open-files = 開く（ファイル）
settings-context-menu-open-path = パスを開く
settings-context-menu-explore = エクスプローラーで開く
settings-context-menu-explore-path = パスをエクスプローラーで開く
settings-context-menu-copy-name = 名前をクリップボードにコピー
settings-context-menu-copy-path = パスをクリップボードにコピー
settings-context-menu-copy-full-name = フルネームをクリップボードにコピー
settings-context-menu-reveal = Freally で表示
settings-context-menu-send-to = Freally に送る（パス）

# §8.8 General → Fonts & Colors.
settings-fc-font = フォント
settings-fc-size = サイズ
settings-fc-state-normal = 通常
settings-fc-state-highlighted = ハイライト
settings-fc-state-current-sort = 現在の並べ替え
settings-fc-state-current-sort-h = 現在の並べ替え（ハイライト）
settings-fc-state-selected = 選択中
settings-fc-state-selected-h = 選択中（ハイライト）
settings-fc-state-inactive-selected = 非アクティブな選択
settings-fc-state-inactive-selected-h = 非アクティブな選択（ハイライト）
settings-fc-foreground = 前景色
settings-fc-background = 背景色
settings-fc-bold = 太字
settings-fc-italic = 斜体
settings-fc-default = 既定
settings-fc-per-lens-accent = レンズごとのアクセント
settings-fc-theme-inherit = テーマ切り替え時にカスタム色を自動反転

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = グローバルホットキー
settings-keyboard-new-window = 新しいウィンドウのホットキー
settings-keyboard-show-window = ウィンドウ表示のホットキー
settings-keyboard-toggle-window = ウィンドウ切り替えのホットキー
settings-keyboard-show-commands = 次を含むコマンドを表示
settings-keyboard-add-chord = + コードを追加
settings-keyboard-remove-chord = 削除

# §8.10 History.
settings-history-search-enable = 検索履歴を有効化
settings-history-search-keep = 検索履歴を { $days } 日間保持
settings-history-run-enable = 実行履歴を有効化
settings-history-run-keep = 実行履歴を { $days } 日間保持
settings-history-clear-now = 今すぐ消去
settings-history-privacy-mode = プライバシーモード
settings-history-per-lens = レンズごとの履歴

# §8.11 Indexes (top-level).
settings-ix-database-location = データベースの場所
settings-ix-multiuser = マルチユーザーデータベースのファイル名
settings-ix-compress = データベースを圧縮
settings-ix-recent-changes = 最近の変更をインデックス化
settings-ix-file-size = ファイルサイズをインデックス化
settings-ix-fast-size-sort = 高速サイズ並べ替え
settings-ix-folder-size = フォルダーサイズをインデックス化
settings-ix-fast-folder-size-sort = 高速フォルダーサイズ並べ替え
settings-ix-date-created = 作成日時をインデックス化
settings-ix-fast-date-created = 高速作成日時並べ替え
settings-ix-date-modified = 更新日時をインデックス化
settings-ix-fast-date-modified = 高速更新日時並べ替え
settings-ix-date-accessed = アクセス日時をインデックス化
settings-ix-fast-date-accessed = 高速アクセス日時並べ替え
settings-ix-attributes = 属性をインデックス化
settings-ix-fast-attributes = 高速属性並べ替え
settings-ix-fast-path-sort = 高速パス並べ替え
settings-ix-fast-extension-sort = 高速拡張子並べ替え
settings-ix-force-rebuild = 強制再構築
settings-ix-compact = インデックスを最適化
settings-ix-verify = インデックスを検証
settings-ix-integrity-policy = インデックス整合性ポリシー
settings-ix-memory-budget = インデクサーのメモリ上限
settings-ix-throttle = バックグラウンドインデックス作成の制限

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = 新しい固定ボリュームを自動的に追加
settings-vol-auto-removable = 新しいリムーバブルボリュームを自動的に追加
settings-vol-auto-remove-offline = オフラインのボリュームを自動的に削除
settings-vol-detected = 検出されたボリューム
settings-vol-include = インデックスに含める
settings-vol-include-only = 次のみを含める（glob／正規表現）
settings-vol-enable-usn = USN ジャーナルを有効化
settings-vol-enable-fsevents = FSEvents ストリームを有効化
settings-vol-enable-inotify = inotify を有効化（昇格時は fanotify）
settings-vol-buffer = ジャーナルバッファサイズ（KB）
settings-vol-allocation-delta = 割り当てデルタ（KB）
settings-vol-load-recent = 起動時にジャーナルから最近の変更を読み込む
settings-vol-monitor = 変更を監視
settings-vol-recreate-journal = ジャーナルを再作成
settings-vol-reset-stream = FSEvents ストリームをリセット
settings-vol-upgrade-fanotify = fanotify にアップグレード（polkit）
settings-vol-remove = 削除

# §8.13 Indexes → Folders.
settings-folders-watched = 監視対象フォルダー
settings-folders-add = 追加…
settings-folders-rescan-now = 今すぐ再スキャン
settings-folders-rescan-all = すべて今すぐ再スキャン
settings-folders-monitor = 変更の監視を試みる
settings-folders-buffer = バッファサイズ
settings-folders-rescan-on-full = バッファが満杯になったら再スキャン

# §8.14 Indexes → File Lists.
settings-flists-add = 追加…
settings-flists-monitor = 変更を監視
settings-flists-editor = ファイルリストエディター…
settings-flists-format = ファイルリストの形式
settings-flists-format-text = テキスト（1 行に 1 パス）
settings-flists-format-json = JSON（メタデータ付き）
settings-flists-format-srcb = Freally バンドル（.srcb）

# §8.15 Indexes → Exclude.
settings-exclude-hidden = 隠しファイルとフォルダーを除外
settings-exclude-system = システムファイルとフォルダーを除外
settings-exclude-list-en = 除外リストを有効化
settings-exclude-folders = フォルダーを除外
settings-exclude-include-only-files = 次のファイルのみを含める（glob）
settings-exclude-files = ファイルを除外（glob）
settings-exclude-os-recommended = OS 推奨の除外を適用
settings-exclude-by-class = 拡張子クラスで除外

# §8.16 Lenses → Filename.
settings-lf-trigram = トライグラム事前フィルターの強度
settings-lf-suffix-mem = 接尾辞配列のメモリ上限
settings-lf-wildcard-limit = ワイルドカード展開の上限
settings-lf-regex-timeout = 正規表現のタイムアウト

# §8.17 Lenses → Content.
settings-lc-enable = コンテンツレンズを有効化
settings-lc-time-budget = ドキュメントあたりの時間上限
settings-lc-mem-ceiling = ドキュメントあたりのメモリ上限
settings-lc-snippet-len = スニペットの長さ
settings-lc-stop-words = ストップワードの言語
settings-lc-re-extract = 設定変更時に再抽出
settings-lc-verify-blobs = 読み込み時に抽出テキストブロブのチェックサムを検証

# §8.18 Lenses → Audio.
settings-la-enable = オーディオレンズを有効化
settings-la-lufs-ref = LUFS 基準規格
settings-la-peak-compute = ピークの算出方法
settings-la-silence-thresh = 無音のしきい値
settings-la-re-extract-modify = 変更イベント時に再抽出

# §8.19 Lenses → Similarity.
settings-ls-enable = 類似度レンズを有効化
settings-ls-sig-size = MinHash シグネチャサイズ（k）
settings-ls-bands = LSH バンド数
settings-ls-recall = 再現率のしきい値
settings-ls-result-cap = 結果の上限

# §8.20 Lenses → Custom.
settings-custom-registry = レジストリ
settings-custom-trust = 信頼
settings-custom-refresh-hashes = ハッシュを更新

# §8.21-§8.22 Network.
settings-net-https-enable = HTTPS サーバーを有効化
settings-net-bind = インターフェイスにバインド
settings-net-port = 待ち受けポート
settings-net-force-https = HTTPS を強制
settings-net-legacy-auth = レガシー HTTP ベーシック認証
settings-net-token-regen = トークンを再生成
settings-net-api-enable = API サーバーを有効化
settings-net-legacy-ftp = レガシー平文 FTP／ETP サポート

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
settings-locale-date-format = 日付の形式
settings-locale-number-format = 数値の形式

# §8.27 About.
settings-about-version = Freally { $version }
settings-about-license = ライセンス
settings-about-credits = クレジット
settings-about-notices = オープンソースの告知

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = 初回起動ウィザード
wizard-step-of-total = ステップ { $step } / { $total }
wizard-roots-hint = Freally に監視させるフォルダーやボリュームを追加します。これは後で「インデックス」設定から変更できます。
wizard-browse = 参照…
wizard-roots-placeholder = …またはパスを貼り付け
wizard-roots-add = 追加
wizard-roots-remove = 削除
wizard-roots-empty = ルートはまだ設定されていません。
wizard-locale-hint = Freally は 18 言語に対応しています。後で切り替えられます。
wizard-theme-hint = システムは OS の外観設定に従います。
wizard-back = 戻る
wizard-next = 次へ

# Status bar polish.
statusbar-hotkey-hint = ホットキー: { $hotkey }
statusbar-cycle-theme = テーマを切り替え
statusbar-indexed-suffix = インデックス済み

# Results / lenses.
lens-expand = レンズを展開
lens-collapse = レンズを折りたたむ
lens-no-matches = このレンズに一致するものはありません。

# Preview pane.
preview-header = プレビュー
preview-loading = 読み込み中…
preview-select-file = プレビューするファイルを選択してください。
preview-unavailable = プレビューは利用できません

# Bookmarks.
bookmarks-label = ★ ブックマーク
bookmarks-empty-hint = ブックマークはまだありません。Ctrl+D で現在のクエリを保存できます。
bookmarks-organize-title = ブックマークの整理
bookmarks-organize-empty = ブックマークはまだありません。
bookmarks-rename = 名前を変更
bookmarks-close = 閉じる

# Settings tree extras.
settings-group-history = 履歴
settings-group-privacy = プライバシーと更新
settings-group-logs = ログとデバッグ
settings-group-backup = バックアップ・エクスポート・リセット
settings-tree-custom-lens = カスタム
settings-unsaved-changes = 未保存の変更

# About dialog.
about-dialog-title = Freally
about-copyright = Copyright © 2026 Mike Weaver. All rights reserved.
about-close = 閉じる

# Connect endpoint dialog.
connect-ftp-title = FTP サーバーに接続
connect-ftp-host = ホスト:
connect-ftp-port = ポート:
connect-ftp-username = ユーザー名:
connect-ftp-password = パスワード:
connect-ftp-link-type = 接続タイプ:

# UI panel.
ui-hint = テーマ、トレイ／メニューバー統合、入力に合わせた検索、行の表示密度。voidtools-Everything と直接同等の機能に加え、(+) 印の Freally 独自機能を含みます。
ui-section-theme = テーマ
ui-theme-system-default = システム（既定）
ui-section-tray = トレイ／メニューバー
ui-section-search-behavior = 検索の動作
ui-section-result-rows = 結果の行
ui-single-click-system-default = システム設定（既定）
ui-single-click-always = 常にシングルクリック
ui-single-click-always-double = 常にダブルクリック
ui-underline-always = 常に
ui-underline-on-hover = ホバー時
ui-underline-never = しない

# Home panel.
home-hint = アプリ起動時に読み込まれる既定値です。各ドロップダウンは「最後の値を使用」のままにすることも、固定値を指定することもできます。レンズの表示状態／結果上限は Freally 独自の機能です (+)。
home-section-match = 一致の既定値
home-section-search-sort = 検索と並べ替えの既定値
home-search-placeholder = 既定では空
home-section-index = インデックスのソース
home-file-list-path = ファイルリストのパス
home-https-endpoint = HTTPS API エンドポイント URL
home-endpoint-token = トークン（フィンガープリントを表示）

# Backup panel.
backup-section-settings = 設定 (+)
backup-section-bookmarks = ブックマーク＋カスタム抽出ツール (+)
backup-section-reset = リセット
backup-toast-exported = 設定を { $path } にエクスポートしました
backup-toast-export-failed = エクスポートに失敗しました: { $error }
backup-toast-imported = 設定をインポートしました
backup-toast-import-failed = インポートに失敗しました: { $error }
backup-toast-bookmarks-exported = ブックマークをエクスポートしました
backup-toast-bookmarks-export-failed = ブックマークのエクスポートに失敗しました: { $error }
backup-toast-bookmarks-imported = ブックマークをインポートしました
backup-toast-bookmarks-import-failed = ブックマークのインポートに失敗しました: { $error }
backup-confirm-reset = すべての設定を既定値にリセットしますか？この操作は取り消せません（ダイアログは開いたままになります）。
backup-toast-reset = すべての設定をリセットしました

# Keyboard panel.
keyboard-section-global = グローバルホットキー
keyboard-placeholder-example = Super+Space
keyboard-section-commands = コマンド
keyboard-placeholder-command = コマンド ID（例: file.export_results）
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = 検索履歴
history-section-run = 実行履歴
history-section-privacy = プライバシー (+)
history-record-filename = ファイル名レンズの履歴を記録
history-record-content = コンテンツレンズの履歴を記録
history-record-audio = オーディオレンズの履歴を記録
history-record-similarity = 類似度レンズの履歴を記録

# Locale panel.
locale-section-language = 言語 (+)
locale-section-time-date = 時刻／日付 (+)
locale-date-os = OS の既定
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = カスタム
locale-date-custom-format = カスタム形式
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = 数値 (+)
locale-number-os = OS の既定
locale-number-custom = カスタム
locale-thousands-sep = 桁区切り文字
locale-decimal-sep = 小数点記号

# Folders panel.
folders-hint = 既定のボリュームに加えて監視する追加のフォルダーです。
folders-list-title = 監視対象フォルダー
folders-empty = フォルダーはまだ追加されていません。
folders-remove = 削除
folders-section-title-dynamic = { $path } の設定
folders-section-schedule = 再スキャンのスケジュール
folders-schedule-daily = 毎日 HH:MM に
folders-schedule-hours = N 時間ごと
folders-schedule-never = しない
folders-hour = 時
folders-minute = 分
folders-hours = 時間
folders-id-label = フォルダー ID（読み取り専用）
folders-select-prompt = 設定するフォルダーを選択してください。
folders-section-extras = Freally 独自機能 (+)
folders-extras-note = スリープからの復帰時の再スキャンは、このビルドでは既定で有効です。このトグルは Phase 13 の調整でフォルダーレベルのコントロールに加わります。

# Volumes panel.
volumes-hint = voidtools-Everything の NTFS／ReFS パネルに相当するクロスプラットフォーム機能です。NTFS／ReFS／exFAT／FAT32（Win）、APFS／HFS+（macOS）、ext4／Btrfs／ZFS／XFS／F2FS（Linux）を自動検出します。
volumes-section-auto-include = 自動追加
volumes-list-title = 検出されたボリューム
volumes-detecting = 検出中…
volumes-empty = ボリュームは検出されませんでした。
volumes-select-prompt = 設定するボリュームを選択してください。

# About panel polish.
about-section-version = バージョン (+)
about-section-license = ライセンス (+)
about-license-text = Mike Weaver — All Rights Reserved. これはプロプライエタリソフトウェアです。
about-license-spdx = SPDX: { $spdx }
about-section-credits = クレジット (+)
about-credits-inspired = voidtools の Everything に着想を得ています。
about-credits-voidtools = voidtools.com
about-credits-repo = プロジェクトリポジトリ

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = Freally を操作するためのコマンドが含まれます。
menu-file-new-window = 新しい検索ウィンドウ
menu-file-open-list = ファイルリストを開く…
menu-file-close-list = ファイルリストを閉じる
menu-file-close = 閉じる
menu-file-export-results = 結果をエクスポート…
menu-file-export-bundle = インデックスバンドルをエクスポート…
menu-file-exit = 終了

# Edit menu.
menu-edit-hint = 検索結果を編集するためのコマンドが含まれます。
menu-edit-cut = 切り取り
menu-edit-copy = コピー
menu-edit-paste = 貼り付け
menu-edit-copy-to-folder = フォルダーにコピー…
menu-edit-move-to-folder = フォルダーに移動…
menu-edit-select-all = すべて選択
menu-edit-invert-selection = 選択を反転
menu-edit-advanced = 詳細
menu-edit-copy-full-name = フルネームをコピー
menu-edit-copy-path = パスをコピー
menu-edit-copy-filename = ファイル名をコピー
menu-edit-copy-as-json = JSON としてコピー
menu-edit-copy-with-metadata = メタデータ付きでコピー
menu-edit-copy-as-bundle-ref = Freally バンドル参照としてコピー

# View menu.
menu-view-hint = 表示を操作するためのコマンドが含まれます。
menu-view-filters = フィルター
menu-view-preview = プレビュー
menu-view-status-bar = ステータスバー
menu-view-thumbs-xl = 特大サムネイル
menu-view-thumbs-l = 大サムネイル
menu-view-thumbs-m = 中サムネイル
menu-view-details = 詳細
menu-view-window-size = ウィンドウサイズ
menu-view-window-size-hint = ウィンドウのサイズを調整するためのコマンドが含まれます。
menu-view-window-small = 小
menu-view-window-medium = 中
menu-view-window-large = 大
menu-view-window-auto = 自動調整
menu-view-zoom = ズーム
menu-view-zoom-hint = フォントとアイコンのサイズを調整するためのコマンドが含まれます。
menu-view-zoom-in = 拡大
menu-view-zoom-out = 縮小
menu-view-zoom-reset = リセット
menu-view-sort-by = 並べ替え
menu-view-sort-by-hint = 結果リストを並べ替えるためのコマンドが含まれます。
menu-view-sort-name = 名前
menu-view-sort-path = パス
menu-view-sort-size = サイズ
menu-view-sort-ext = 拡張子
menu-view-sort-type = 種類
menu-view-sort-modified = 更新日時
menu-view-sort-created = 作成日時
menu-view-sort-accessed = アクセス日時
menu-view-sort-attributes = 属性
menu-view-sort-recently-changed = 最近変更された日時
menu-view-sort-run-count = 実行回数
menu-view-sort-run-date = 実行日時
menu-view-sort-file-list-filename = ファイルリストのファイル名
menu-view-sort-lufs = LUFS
menu-view-sort-length = 長さ
menu-view-sort-similarity = 類似度スコア
menu-view-sort-asc = 昇順
menu-view-sort-desc = 降順
menu-view-go-to = 移動
menu-view-refresh = 更新
menu-view-theme = テーマ
menu-view-theme-hint = システム・ライト・ダークのテーマを切り替えます。
menu-view-lenses = レンズ
menu-view-lenses-hint = 結果リスト内の各レンズの表示を切り替えます。
menu-view-on-top = 最前面
menu-view-on-top-hint = このウィンドウを他のウィンドウより前面に保つためのコマンドが含まれます。
menu-view-on-top-never = しない
menu-view-on-top-always = 常に
menu-view-on-top-while-searching = 検索中

# Search menu.
menu-search-hint = 検索の切り替えが含まれます。
menu-search-match-case = 大文字と小文字を区別
menu-search-match-whole-word = 単語全体に一致
menu-search-match-path = パスに一致
menu-search-match-diacritics = ダイアクリティカルマークを区別
menu-search-enable-regex = 正規表現を有効化
menu-search-advanced = 詳細検索…
menu-search-add-to-filters = フィルターに追加…
menu-search-organize-filters = フィルターの整理…
menu-search-filter-everything = すべて
menu-search-filter-archive = 圧縮（アーカイブ）
menu-search-filter-folder = フォルダー
menu-search-filter-custom = カスタムフィルター…

# Bookmarks menu.
menu-bookmarks-hint = ブックマークを操作するためのコマンドが含まれます。
menu-bookmarks-add = ブックマークに追加
menu-bookmarks-organize = ブックマークの整理…

# Tools menu.
menu-tools-hint = ツールのコマンドが含まれます。
menu-tools-connect = FTP サーバーに接続…
menu-tools-disconnect = FTP サーバーから切断
menu-tools-file-list-editor = ファイルリストエディター…
menu-tools-index-maintenance = インデックスのメンテナンス
menu-tools-index-maintenance-hint = インデックスのメンテナンスツール。
menu-tools-verify-index = インデックスを検証…
menu-tools-compact-index = インデックスを最適化…
menu-tools-rebuild-index = インデックスを強制再構築…
menu-tools-custom-extractor = カスタム抽出ツールマネージャー…
menu-tools-custom-extractor-hint = Wasm サンドボックス化されたカスタム抽出ツールを管理します。
menu-tools-options = オプション…

# Help menu.
menu-help-hint = ヘルプのコマンドが含まれます。
menu-help-help = Freally ヘルプ
menu-help-search-syntax = 検索構文
menu-help-regex-syntax = 正規表現の構文
menu-help-audio-ref = オーディオ修飾子リファレンス
menu-help-similarity-ref = 類似度修飾子リファレンス
menu-help-cli-options = コマンドラインオプション
menu-help-website = Freally ウェブサイト
menu-help-check-updates = 更新を確認…
menu-help-sponsor = スポンサー／寄付
menu-help-about = Freally について…

# Result column headers (short forms used in the table header row).
column-name = 名前
column-path = パス
column-size = サイズ
column-modified = 更新日時
column-type = 種類
column-ext = 拡張子
column-sort-by = { $name } で並べ替え
column-resize = { $name } 列のサイズを変更

# Section subtitle bars used inside multiple settings panels.
section-behavior = 動作
section-rendering = レンダリング
section-status-bar = ステータスバー
section-display-format = 表示形式
section-loading-priority = 読み込みの優先度
section-compatibility = 互換性
section-storage = ストレージ
section-index-fields = インデックスフィールド
section-maintenance = メンテナンス
section-logging = ログ記録
section-tools = ツール
section-privacy = プライバシー
section-auto-update = 自動更新 (+)
section-bind = バインド
section-lens = レンズ
section-budgets = 上限
section-other = その他
section-per-format-mode = 形式ごとのモード
section-loudness = ラウドネス
section-tuning = チューニング (+)
section-minhash-lsh = MinHash + LSH パラメーター (+)
section-top-level = 最上位
section-file-globs = ファイルの glob
section-file-list-settings = 選択したファイルリストの設定
section-editor-format = エディター＋形式 (E + +)
section-api-server = API サーバー（E を改変）
section-freally-extras = Freally 独自機能 (+)
section-freally-additions = Freally 追加機能 (+)
section-freally-extensions = Freally 拡張機能 (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = 最後の値を使用
opt-use-last-value-default = 最後の値を使用（既定）
opt-low = 低
opt-normal-default = 標準（既定）
opt-high = 高
opt-disabled = 無効
opt-off = オフ
opt-on-battery = バッテリー駆動時
opt-always = 常に
opt-clamp-default = クランプ（既定）
opt-wrap = ラップ
opt-none = なし
opt-strict-refuse = 厳格（破損時はクエリを拒否）
opt-lenient-warn = 寛容（警告するがクエリは実行）
opt-system-default = システム既定
opt-drag-select = ドラッグ選択
opt-auto-binary = 自動（2 進）
opt-auto-decimal = 自動（10 進）

# Unit suffixes shown next to number inputs.
unit-days = 日
unit-b = B
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-tb = TB

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = 先読み
opt-lazy-default = 遅延（既定）
opt-on = オン
opt-on-default = オン（既定）
opt-all = すべて
opt-weekly = 毎週
opt-monthly = 毎月
opt-name-asc = 名前 昇順
opt-name-desc = 名前 降順
opt-size-asc = サイズ 昇順
opt-size-desc = サイズ 降順
opt-modified-asc = 更新日時 昇順
opt-modified-desc = 更新日時 降順
opt-compact = コンパクト
opt-comfortable = ゆったり
opt-details = 詳細
opt-thumbnails = サムネイル
opt-local-db-default = ローカルデータベース（既定）
opt-file-list = ファイルリスト
opt-https-endpoint = HTTPS API エンドポイント
opt-right-default = 右（既定）
opt-bottom = 下
opt-or-and-default = OR > AND（既定）
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128（既定）
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = Broadcast film (-23)
opt-true-peak = トゥルーピーク（4× オーバーサンプリング、既定）
opt-sample-peak = サンプルピーク
opt-auto-per-doc = 自動（ドキュメントごと）
opt-log-error = Error
opt-log-warn = Warn
opt-log-info-default = Info（既定）
opt-log-debug = Debug
opt-log-trace = Trace

# More Freally apps (Central inside panel) — host chrome
menu-help-more-apps = その他の Freally アプリ…
moreapps-title = その他の Freally アプリ
