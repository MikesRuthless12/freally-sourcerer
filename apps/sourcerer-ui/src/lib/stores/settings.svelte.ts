// Settings store — round-trips through the IPC settings_get/set commands.
// Phase 12 expands the schema to cover every PRD §8.2-§8.27 control.
// Persistence is local (per Phase 11), but each `flush()` also fans out
// to the daemon for index-affecting fields via the apply hooks below.

import * as ipc from "../ipc/settings";
import { invoke } from "@tauri-apps/api/core";
import type { SettingsState } from "../ipc/types";
import type { PanelId } from "./settings_dialog.svelte";

const FALLBACK: SettingsState = {
  // Phase 11 carry-over
  theme: "system",
  locale: "en",
  show_status_bar: true,
  show_size_in_status_bar: true,
  show_timing_badges: true,
  show_preview: false,
  row_density: "compact",
  thumb_size: "details",
  active_column_profile: "default",
  column_profiles: [
    {
      id: "default",
      name: "Default",
      columns: [
        { id: "name", width_px: 360, visible: true },
        { id: "path", width_px: 400, visible: true },
        { id: "size", width_px: 100, visible: true },
        { id: "modified", width_px: 180, visible: true },
        { id: "type", width_px: 120, visible: true },
        { id: "ext", width_px: 80, visible: true }
      ]
    }
  ],
  lens_visibility: { filename: true, content: true, audio: true, similarity: true },
  search_opts: {
    match_case: false,
    match_whole_word: false,
    match_path: false,
    match_diacritics: false,
    enable_regex: false
  },
  on_top: "never",
  zoom: 1,
  hotkey: "Super+Space",
  endpoint: { name: "Local DB", kind: "local" },
  extractor_modes: {},
  first_run_complete: false,
  privacy_mode: false,
  // §8.2
  run_in_background: true,
  show_tray_icon: true,
  single_click_tray: false,
  open_new_window_from_tray: false,
  open_new_window_when_launching: false,
  search_as_you_type: true,
  select_search_on_mouse_click: true,
  focus_search_on_activate: true,
  full_row_select: true,
  single_click_open: "system_settings",
  underline_icon_titles: "system_settings",
  animated_theme_crossfade: true,
  // §8.3
  default_match_case: "use_last",
  default_match_whole_word: "use_last",
  default_match_path: "use_last",
  default_match_diacritics: "use_last",
  default_match_regex: "use_last",
  default_search: "",
  default_filter: "use_last",
  default_sort: "use_last",
  default_view: "use_last",
  default_index: "local",
  default_file_list: "",
  default_https_endpoint: { url: "", token_fingerprint: "" },
  default_lens_visibility: { filename: true, content: true, audio: true, similarity: true },
  default_lens_result_limits: { filename: 200, content: 50, audio: 20, similarity: 10 },
  // §8.4
  fast_ascii_search: true,
  match_path_when_term_contains_separator: true,
  match_whole_filename_with_wildcards: true,
  allow_literal_operators: true,
  allow_round_bracket_grouping: true,
  expand_environment_variables: true,
  replace_forward_with_backslashes: false,
  operator_precedence: "or_first",
  strict_everything_mode: false,
  auto_detect_regex: true,
  modifier_completions: true,
  show_parse_tree_on_hover: false,
  // §8.5
  hide_results_when_empty: false,
  clear_selection_on_search: true,
  close_window_on_execute: false,
  open_path_with_double_click_in_path_column: false,
  automatically_scroll_view: true,
  double_quote_copy_as_path: false,
  do_not_select_extension_when_renaming: true,
  sort_date_descending_first: true,
  sort_size_descending_first: true,
  result_list_focus: "clamp",
  load_icon_priority: "normal",
  load_thumbnail_priority: "normal",
  load_extended_information_priority: "normal",
  group_by_lens: true,
  show_snippet_preview_inline: true,
  // §8.6
  double_buffer: true,
  alternate_row_color: false,
  show_row_mouseover: false,
  show_highlighted_search_terms: true,
  show_selected_item_in_status_bar: true,
  show_result_count_with_selection_count: false,
  show_tooltips: true,
  update_display_immediately_after_scrolling: true,
  size_format: "auto_binary",
  selection_rectangle: "system",
  show_lufs_codec_length_badges: true,
  show_minhash_similarity_score: true,
  preview_pane: "right",
  // §8.7
  context_menu: {
    open_folders: { visibility: "show", command: "" },
    open_files: { visibility: "show", command: "" },
    open_path: { visibility: "show", command: "" },
    explore: { visibility: "show", command: "" },
    explore_path: { visibility: "show", command: "" },
    copy_name: { visibility: "show", command: "" },
    copy_path: { visibility: "show", command: "" },
    copy_full_name: { visibility: "show", command: "" },
    reveal_in_sourcerer: { visibility: "show", command: "" },
    send_to_sourcerer: { visibility: "show", command: "" }
  },
  // §8.8
  fonts_and_colors: {
    font: "default",
    size_px: 13,
    states: {
      normal: { fg: null, bg: null, bold: false, italic: false },
      highlighted: { fg: null, bg: null, bold: false, italic: false },
      current_sort: { fg: null, bg: null, bold: false, italic: false },
      current_sort_highlighted: { fg: null, bg: null, bold: false, italic: false },
      selected: { fg: null, bg: null, bold: false, italic: false },
      selected_highlighted: { fg: null, bg: null, bold: false, italic: false },
      inactive_selected: { fg: null, bg: null, bold: false, italic: false },
      inactive_selected_highlighted: { fg: null, bg: null, bold: false, italic: false }
    },
    per_lens_accent: { filename: null, content: null, audio: null, similarity: null },
    theme_inheritance_toggle: true
  },
  // §8.9
  keyboard: { new_window_hotkey: "", show_window_hotkey: "", toggle_window_hotkey: "", per_action: [] },
  // §8.11
  index_core: {
    database_location: "",
    multi_user_database_filename: false,
    compress_database: true,
    index_recent_changes: true,
    index_file_size: true,
    fast_size_sort: true,
    index_folder_size: false,
    fast_folder_size_sort: false,
    index_date_created: false,
    fast_date_created_sort: false,
    index_date_modified: true,
    fast_date_modified_sort: true,
    index_date_accessed: false,
    fast_date_accessed_sort: false,
    index_attributes: false,
    fast_attributes_sort: false,
    fast_path_sort: true,
    fast_extension_sort: false,
    integrity_policy: "strict",
    memory_budget_mb: 1024,
    background_throttle: "off"
  },
  // §8.16
  lens_filename: {
    trigram_aggressiveness: "normal",
    suffix_array_memory_mb: 256,
    wildcard_expansion_limit: 100000,
    regex_timeout_ms: 100
  },
  // §8.17
  lens_content: {
    enabled: true,
    per_format: {},
    time_budget_ms: 5000,
    memory_ceiling_mb: 256,
    snippet_length: 200,
    stop_words_language: "auto",
    re_extract_on_settings_change: false,
    verify_blob_checksums_on_read: true
  },
  // §8.18
  lens_audio: {
    enabled: true,
    per_format: {},
    lufs_reference: "ebu_r128",
    peak_compute: "true_peak",
    silence_threshold_dbfs: -60,
    re_extract_on_modify: true
  },
  // §8.19
  lens_similarity: { enabled: true, signature_size: 128, bands: 16, recall_threshold: 0.95, result_cap: 50 },
  // §8.23
  privacy_and_updates: { auto_update: "default", pre_release_channel: false },
  // §8.24
  logs_and_debug: { log_level: "info", log_file_location: "", log_retention_mb: 50, show_debug_overlay: false },
  // §8.26
  locale_settings: {
    locale: "en",
    rtl_preview: false,
    date_format: "os",
    date_format_custom: "",
    number_format: "os",
    thousands_separator: ",",
    decimal_separator: "."
  }
};

/// Maps each PanelId to the keys it owns. resetPanel(id) restores
/// these keys to FALLBACK and pushes through `flush()`.
const PANEL_KEYS: Record<PanelId, (keyof SettingsState)[]> = {
  "general.ui": [
    "theme",
    "run_in_background",
    "show_tray_icon",
    "single_click_tray",
    "open_new_window_from_tray",
    "open_new_window_when_launching",
    "search_as_you_type",
    "select_search_on_mouse_click",
    "focus_search_on_activate",
    "full_row_select",
    "single_click_open",
    "underline_icon_titles",
    "row_density",
    "show_timing_badges",
    "animated_theme_crossfade"
  ],
  "general.home": [
    "default_match_case",
    "default_match_whole_word",
    "default_match_path",
    "default_match_diacritics",
    "default_match_regex",
    "default_search",
    "default_filter",
    "default_sort",
    "default_view",
    "default_index",
    "default_file_list",
    "default_https_endpoint",
    "default_lens_visibility",
    "default_lens_result_limits"
  ],
  "general.search": [
    "fast_ascii_search",
    "match_path_when_term_contains_separator",
    "match_whole_filename_with_wildcards",
    "allow_literal_operators",
    "allow_round_bracket_grouping",
    "expand_environment_variables",
    "replace_forward_with_backslashes",
    "operator_precedence",
    "strict_everything_mode",
    "auto_detect_regex",
    "modifier_completions",
    "show_parse_tree_on_hover"
  ],
  "general.results": [
    "hide_results_when_empty",
    "clear_selection_on_search",
    "close_window_on_execute",
    "open_path_with_double_click_in_path_column",
    "automatically_scroll_view",
    "double_quote_copy_as_path",
    "do_not_select_extension_when_renaming",
    "sort_date_descending_first",
    "sort_size_descending_first",
    "result_list_focus",
    "load_icon_priority",
    "load_thumbnail_priority",
    "load_extended_information_priority",
    "group_by_lens",
    "show_snippet_preview_inline",
    "column_profiles",
    "active_column_profile"
  ],
  "general.view": [
    "double_buffer",
    "alternate_row_color",
    "show_row_mouseover",
    "show_highlighted_search_terms",
    "show_selected_item_in_status_bar",
    "show_result_count_with_selection_count",
    "show_size_in_status_bar",
    "show_tooltips",
    "update_display_immediately_after_scrolling",
    "size_format",
    "selection_rectangle",
    "show_lufs_codec_length_badges",
    "show_minhash_similarity_score",
    "preview_pane"
  ],
  "general.context_menu": ["context_menu"],
  "general.fonts_colors": ["fonts_and_colors"],
  "general.keyboard": ["keyboard", "hotkey"],
  history: [],
  "indexes.top": ["index_core"],
  "indexes.volumes": [],
  "indexes.folders": [],
  "indexes.file_lists": [],
  "indexes.exclude": [],
  "lenses.filename": ["lens_filename"],
  "lenses.content": ["lens_content"],
  "lenses.audio": ["lens_audio"],
  "lenses.similarity": ["lens_similarity"],
  "lenses.custom": [],
  "network.https": [],
  "network.api": [],
  privacy: ["privacy_and_updates"],
  logs: ["logs_and_debug"],
  backup: [],
  locale: ["locale_settings", "locale"],
  about: []
};

class SettingsStore {
  state = $state<SettingsState>(FALLBACK);
  loaded = $state(false);
  /// Snapshot taken when the settings dialog opens. `rollback()` restores it.
  private snapshotState: SettingsState | null = null;

  async hydrate() {
    try {
      this.state = { ...FALLBACK, ...(await ipc.get()) };
    } catch (e) {
      console.warn("[settings] using fallback (IPC unavailable):", e);
    }
    this.loaded = true;
  }

  snapshot() {
    this.snapshotState = JSON.parse(JSON.stringify(this.state)) as SettingsState;
  }

  rollback() {
    if (this.snapshotState) {
      this.state = JSON.parse(JSON.stringify(this.snapshotState)) as SettingsState;
    }
  }

  async patch(p: Partial<SettingsState>) {
    // Optimistic local update so the UI flips immediately.
    this.state = { ...this.state, ...p };
  }

  /// Persist the current state to disk + the daemon.
  async flush() {
    try {
      const sent = await ipc.set(this.state as unknown as Partial<SettingsState>);
      this.state = { ...FALLBACK, ...sent };
      // Fan out index-affecting fields to the daemon via settings.apply.
      await this.applyToDaemon();
    } catch (e) {
      console.warn("[settings] flush failed:", e);
    }
  }

  async applyToDaemon() {
    try {
      await invoke("settings_apply_to_daemon", { state: this.state });
    } catch (e) {
      console.warn("[settings] settings.apply daemon round-trip failed:", e);
    }
  }

  async reset() {
    try {
      this.state = { ...FALLBACK, ...(await ipc.reset()) };
    } catch (e) {
      console.warn("[settings] reset failed:", e);
    }
  }

  async resetPanel(panel: PanelId) {
    const keys = PANEL_KEYS[panel] ?? [];
    if (keys.length === 0) return;
    const patch: Partial<SettingsState> = {};
    for (const k of keys) {
      // SAFETY: we control PANEL_KEYS so the cast is safe.
      (patch as Record<string, unknown>)[k as string] = (
        FALLBACK as unknown as Record<string, unknown>
      )[k as string];
    }
    this.state = { ...this.state, ...patch };
    await this.flush();
  }
}

export const settingsStore = new SettingsStore();
