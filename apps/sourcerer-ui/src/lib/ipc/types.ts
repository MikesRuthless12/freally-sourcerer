// Stable IPC type contract — UI side. Mirrors the Rust commands in
// src-tauri/src/commands/. Phase 11 ships a mock backend; Phase 12+ swap the
// real sourcerer-indexd implementation behind these types without touching
// the UI.

// ---- query.parse (real, routed to sourcerer-query::parse_to_report) ----

export interface TokenSpan {
  start: number;
  end: number;
}

export type TokenKind =
  | { kind: "literal" }
  | { kind: "quoted" }
  | { kind: "wildcard" }
  | { kind: "regex" }
  | { kind: "modifier"; name: string }
  | { kind: "quick_filter"; name: string }
  | { kind: "lens_prefix"; lens: string }
  | { kind: "l_paren" }
  | { kind: "r_paren" }
  | { kind: "bang" }
  | { kind: "and" }
  | { kind: "or" }
  | { kind: "not" };

export interface TokenInfo {
  kind: TokenKind;
  span: TokenSpan;
  text: string;
}

export type ErrorCode =
  | "empty"
  | "unexpected_eof"
  | "unexpected_token"
  | "unbalanced_parens"
  | "invalid_regex"
  | "invalid_wildcard"
  | "unknown_modifier"
  | "invalid_modifier_value"
  | "strict_everything_violation";

export interface ErrorInfo {
  span: TokenSpan;
  message: string;
  code: ErrorCode;
}

export type AstNode =
  | { kind: "literal"; text: string }
  | { kind: "wildcard"; raw: string }
  | { kind: "regex"; raw: string }
  | { kind: "modifier"; name: string; detail: unknown }
  | { kind: "quick_filter"; name: string }
  | { kind: "lens"; lens: string; inner: AstNode }
  | { kind: "and"; children: AstNode[] }
  | { kind: "or"; children: AstNode[] }
  | { kind: "not"; inner: AstNode }
  | { kind: "true" };

export interface ParseReport {
  source: string;
  strict_everything: boolean;
  ast: AstNode | null;
  tokens: TokenInfo[];
  errors: ErrorInfo[];
}

export interface ParseOpts {
  strict_everything: boolean;
}

// ---- query.run (mock) ----

export type LensId = "filename" | "content" | "audio" | "similarity";

export interface QueryHit {
  file_id: string;
  lens: LensId;
  name: string;
  path: string;
  ext: string;
  size: number;
  modified_ms: number;
  type: string;
  score: number;
}

export interface LensTimings {
  filename_ms: number;
  content_ms: number;
  audio_ms: number;
  similarity_ms: number;
  total_ms: number;
}

export interface QueryRunHandle {
  handle: string;
}

export interface QueryBatch {
  handle: string;
  lens: LensId;
  hits: QueryHit[];
  done: boolean;
}

export interface QueryDone {
  handle: string;
  timings: LensTimings;
}

// ---- index.state ----

export type IndexPhase = "indexing" | "indexed" | "paused" | "error";

export interface IndexState {
  phase: IndexPhase;
  files_indexed: number;
  files_total: number;
  message: string;
}

// ---- bookmarks ----

export interface Bookmark {
  id: string;
  name: string;
  query: string;
  created_ms: number;
}

// ---- extractors ----

export type ExtractorMode = "eager" | "lazy" | "disabled";

export interface ExtractorInfo {
  id: string;
  display_name: string;
  mode: ExtractorMode;
  formats: string[];
}

// ---- settings ----

export type ColumnId = "name" | "path" | "size" | "modified" | "type" | "ext";

export interface ColumnProfile {
  id: string;
  name: string;
  columns: { id: ColumnId; width_px: number; visible: boolean }[];
}

export type RowDensity = "compact" | "comfortable";

export type ThumbSize = "xl" | "l" | "m" | "details";

export interface SearchOpts {
  match_case: boolean;
  match_whole_word: boolean;
  match_path: boolean;
  match_diacritics: boolean;
  enable_regex: boolean;
}

export type OnTopMode = "never" | "always" | "while_searching";

export type LastValueOption = "use_last" | "on" | "off";
export type FilterChoice =
  | "use_last"
  | "all"
  | "audio"
  | "video"
  | "image"
  | "document"
  | "executable"
  | "archive";
export type SortChoice =
  | "use_last"
  | "name_asc"
  | "name_desc"
  | "path"
  | "size_asc"
  | "size_desc"
  | "modified_asc"
  | "modified_desc";
export type ViewChoice = "use_last" | "compact" | "comfortable" | "details" | "thumbnails";
export type IndexSourceChoice = "local" | "file_list" | "https_endpoint";
export type SingleClickOpenChoice = "system_settings" | "always_single" | "always_double";
export type UnderlineTitlesChoice = "system_settings" | "always" | "on_hover" | "never";
export type OperatorPrecedenceChoice = "or_first" | "and_first";
export type ResultListFocusChoice = "clamp" | "wrap" | "none";
export type LoadPriorityChoice = "high" | "normal" | "low" | "disabled";
export type SizeFormatChoice = "b" | "kb" | "mb" | "gb" | "auto_binary" | "auto_decimal";
export type SelectionRectChoice = "system" | "drag_select" | "none";
export type PreviewPaneChoice = "right" | "bottom" | "off";
export type AutoUpdateCadence = "default" | "weekly" | "monthly" | "off";
export type LogLevelChoice = "error" | "warn" | "info" | "debug" | "trace";
export type IntegrityPolicyChoice = "strict" | "lenient";
export type BackgroundThrottleChoice = "off" | "on_battery" | "always";
export type ContextMenuVisibility = "show" | "shift_only" | "hide";

export interface ContextMenuEntry {
  visibility: ContextMenuVisibility;
  command: string;
}

export interface ContextMenuConfig {
  open_folders: ContextMenuEntry;
  open_files: ContextMenuEntry;
  open_path: ContextMenuEntry;
  explore: ContextMenuEntry;
  explore_path: ContextMenuEntry;
  copy_name: ContextMenuEntry;
  copy_path: ContextMenuEntry;
  copy_full_name: ContextMenuEntry;
  reveal_in_sourcerer: ContextMenuEntry;
  send_to_sourcerer: ContextMenuEntry;
}

export interface RgbColor {
  r: number;
  g: number;
  b: number;
}

export interface ItemStateStyle {
  fg: RgbColor | null;
  bg: RgbColor | null;
  bold: boolean;
  italic: boolean;
}

export interface FontsAndColorsState {
  font: string;
  size_px: number;
  states: {
    normal: ItemStateStyle;
    highlighted: ItemStateStyle;
    current_sort: ItemStateStyle;
    current_sort_highlighted: ItemStateStyle;
    selected: ItemStateStyle;
    selected_highlighted: ItemStateStyle;
    inactive_selected: ItemStateStyle;
    inactive_selected_highlighted: ItemStateStyle;
  };
  per_lens_accent: Record<LensId, RgbColor | null>;
  theme_inheritance_toggle: boolean;
}

export interface KeyboardChord {
  command: string;
  binding: string;
}

export interface KeyboardState {
  new_window_hotkey: string;
  show_window_hotkey: string;
  toggle_window_hotkey: string;
  per_action: KeyboardChord[];
}

export interface IndexCoreSettings {
  database_location: string;
  multi_user_database_filename: boolean;
  compress_database: boolean;
  index_recent_changes: boolean;
  index_file_size: boolean;
  fast_size_sort: boolean;
  index_folder_size: boolean;
  fast_folder_size_sort: boolean;
  index_date_created: boolean;
  fast_date_created_sort: boolean;
  index_date_modified: boolean;
  fast_date_modified_sort: boolean;
  index_date_accessed: boolean;
  fast_date_accessed_sort: boolean;
  index_attributes: boolean;
  fast_attributes_sort: boolean;
  fast_path_sort: boolean;
  fast_extension_sort: boolean;
  integrity_policy: IntegrityPolicyChoice;
  memory_budget_mb: number;
  background_throttle: BackgroundThrottleChoice;
}

export interface FilenameLensSettings {
  trigram_aggressiveness: "low" | "normal" | "high";
  suffix_array_memory_mb: number;
  wildcard_expansion_limit: number;
  regex_timeout_ms: number;
}

export interface ContentLensSettings {
  enabled: boolean;
  per_format: Record<string, "eager" | "lazy" | "disabled">;
  time_budget_ms: number;
  memory_ceiling_mb: number;
  snippet_length: number;
  stop_words_language: string;
  re_extract_on_settings_change: boolean;
  verify_blob_checksums_on_read: boolean;
}

export interface AudioLensSettings {
  enabled: boolean;
  per_format: Record<string, "eager" | "lazy" | "disabled">;
  lufs_reference: "ebu_r128" | "atsc_a85" | "spotify" | "apple_music" | "broadcast_film";
  peak_compute: "true_peak" | "sample_peak";
  silence_threshold_dbfs: number;
  re_extract_on_modify: boolean;
}

export interface SimilarityLensSettings {
  enabled: boolean;
  signature_size: 64 | 128 | 256;
  bands: 8 | 16 | 32;
  recall_threshold: number;
  result_cap: number;
}

export interface PrivacyAndUpdatesSettings {
  auto_update: AutoUpdateCadence;
  pre_release_channel: boolean;
  // crash_reports + telemetry are always-disabled per PRD §8.23.
}

export interface LogsAndDebugSettings {
  log_level: LogLevelChoice;
  log_file_location: string;
  log_retention_mb: number;
  show_debug_overlay: boolean;
}

export interface LocaleSettings {
  locale: string;
  rtl_preview: boolean;
  date_format: "os" | "iso8601" | "rfc3339" | "custom";
  date_format_custom: string;
  number_format: "os" | "custom";
  thousands_separator: string;
  decimal_separator: string;
}

export interface SettingsState {
  // ---- General → UI (§8.2) ----
  theme: "system" | "light" | "dark";
  run_in_background: boolean;
  show_tray_icon: boolean;
  single_click_tray: boolean;
  open_new_window_from_tray: boolean;
  open_new_window_when_launching: boolean;
  search_as_you_type: boolean;
  select_search_on_mouse_click: boolean;
  focus_search_on_activate: boolean;
  full_row_select: boolean;
  single_click_open: SingleClickOpenChoice;
  underline_icon_titles: UnderlineTitlesChoice;
  row_density: RowDensity;
  show_timing_badges: boolean;
  animated_theme_crossfade: boolean;

  // ---- General → Home (§8.3) ----
  default_match_case: LastValueOption;
  default_match_whole_word: LastValueOption;
  default_match_path: LastValueOption;
  default_match_diacritics: LastValueOption;
  default_match_regex: LastValueOption;
  default_search: string;
  default_filter: FilterChoice;
  default_sort: SortChoice;
  default_view: ViewChoice;
  default_index: IndexSourceChoice;
  default_file_list: string;
  default_https_endpoint: { url: string; token_fingerprint: string };
  default_lens_visibility: Record<LensId, boolean>;
  default_lens_result_limits: Record<LensId, number>;

  // ---- General → Search (§8.4) ----
  fast_ascii_search: boolean;
  match_path_when_term_contains_separator: boolean;
  match_whole_filename_with_wildcards: boolean;
  allow_literal_operators: boolean;
  allow_round_bracket_grouping: boolean;
  expand_environment_variables: boolean;
  replace_forward_with_backslashes: boolean;
  operator_precedence: OperatorPrecedenceChoice;
  strict_everything_mode: boolean;
  auto_detect_regex: boolean;
  modifier_completions: boolean;
  show_parse_tree_on_hover: boolean;

  // ---- General → Results (§8.5) ----
  hide_results_when_empty: boolean;
  clear_selection_on_search: boolean;
  close_window_on_execute: boolean;
  open_path_with_double_click_in_path_column: boolean;
  automatically_scroll_view: boolean;
  double_quote_copy_as_path: boolean;
  do_not_select_extension_when_renaming: boolean;
  sort_date_descending_first: boolean;
  sort_size_descending_first: boolean;
  result_list_focus: ResultListFocusChoice;
  load_icon_priority: LoadPriorityChoice;
  load_thumbnail_priority: LoadPriorityChoice;
  load_extended_information_priority: LoadPriorityChoice;
  group_by_lens: boolean;
  show_snippet_preview_inline: boolean;

  // ---- General → View (§8.6) ----
  double_buffer: boolean;
  alternate_row_color: boolean;
  show_row_mouseover: boolean;
  show_highlighted_search_terms: boolean;
  show_selected_item_in_status_bar: boolean;
  show_result_count_with_selection_count: boolean;
  show_size_in_status_bar: boolean;
  show_tooltips: boolean;
  update_display_immediately_after_scrolling: boolean;
  size_format: SizeFormatChoice;
  selection_rectangle: SelectionRectChoice;
  show_lufs_codec_length_badges: boolean;
  show_minhash_similarity_score: boolean;
  preview_pane: PreviewPaneChoice;

  // ---- General → Context Menu (§8.7) ----
  context_menu: ContextMenuConfig;

  // ---- General → Fonts & Colors (§8.8) ----
  fonts_and_colors: FontsAndColorsState;

  // ---- General → Keyboard (§8.9) ----
  keyboard: KeyboardState;

  // ---- Indexes top-level (§8.11) ----
  index_core: IndexCoreSettings;

  // ---- Lenses (§8.16-§8.19) ----
  lens_filename: FilenameLensSettings;
  lens_content: ContentLensSettings;
  lens_audio: AudioLensSettings;
  lens_similarity: SimilarityLensSettings;

  // ---- Privacy & Updates (§8.23) ----
  privacy_and_updates: PrivacyAndUpdatesSettings;

  // ---- Logs & Debug (§8.24) ----
  logs_and_debug: LogsAndDebugSettings;

  // ---- Locale (§8.26) ----
  locale_settings: LocaleSettings;

  // ---- carry-over fields kept for back-compat ----
  locale: string;
  show_status_bar: boolean;
  show_preview: boolean;
  thumb_size: ThumbSize;
  active_column_profile: string;
  column_profiles: ColumnProfile[];
  lens_visibility: Record<LensId, boolean>;
  search_opts: SearchOpts;
  on_top: OnTopMode;
  zoom: number;
  hotkey: string;
  endpoint: { name: string; kind: "local" | "remote" };
  extractor_modes: Record<string, "eager" | "lazy" | "disabled">;
  first_run_complete: boolean;
  privacy_mode: boolean;
}

// ---- files ----

export interface PreviewPayload {
  kind: "text" | "image" | "unsupported";
  text?: string;
  data_url?: string;
  message?: string;
}

// ---- volumes (Phase 12) ----

export type VolumeStatus = "indexed" | "indexing" | "paused" | "offline" | "error";

export interface VolumeInfo {
  id: string;
  label: string;
  mount_point: string;
  fs_kind: string;
  used_bytes: number;
  total_bytes: number;
  status: VolumeStatus;
  indexed: boolean;
  journal_enabled: boolean;
  journal_buffer_kb: number;
  allocation_delta_kb: number | null;
  include_only: string | null;
  load_recent_changes: boolean;
  monitor_changes: boolean;
}

export interface VolumeUpdate {
  id: string;
  indexed?: boolean;
  journal_enabled?: boolean;
  journal_buffer_kb?: number;
  allocation_delta_kb?: number;
  include_only?: string;
  load_recent_changes?: boolean;
  monitor_changes?: boolean;
}

// ---- folders (Phase 12) ----

export type RescanSchedule =
  | { kind: "at_time"; hour: number; minute: number }
  | { kind: "every_hours"; hours: number }
  | { kind: "never" };

export interface WatchedFolder {
  id: string;
  path: string;
  monitor_changes: boolean;
  buffer_kb: number;
  rescan_on_full_buffer: boolean;
  rescan_schedule: RescanSchedule;
}

// ---- excludes (Phase 12) ----

export interface ExcludeRules {
  exclude_hidden: boolean;
  exclude_system: boolean;
  list_enabled: boolean;
  folders: string[];
  include_only_files: string | null;
  exclude_files: string | null;
}

// ---- network (Phase 12) ----

export interface NetworkStatus {
  https_running: boolean;
  https_bind: string | null;
  https_port: number | null;
  https_token_fingerprint: string | null;
  api_running: boolean;
  api_port: number | null;
}

// ---- custom extractors (Phase 12) ----

export interface SandboxView {
  network: boolean;
  filesystem_write: boolean;
  clock: boolean;
}

export interface CustomExtractorEntry {
  id: string;
  display_name: string;
  version: string;
  hash_blake3: string;
  formats: string[];
  time_budget_ms: number;
  memory_budget_mb: number;
  trusted: boolean;
  sandbox_view: SandboxView;
}

// ---- history (Phase 12) ----

export interface PerLensHistory {
  filename: boolean;
  content: boolean;
  audio: boolean;
  similarity: boolean;
}

export interface HistoryConfig {
  search_history_enabled: boolean;
  search_history_keep_days: number;
  run_history_enabled: boolean;
  run_history_keep_days: number;
  privacy_mode: boolean;
  per_lens: PerLensHistory;
}

export type HistoryUpdate = Partial<HistoryConfig>;
