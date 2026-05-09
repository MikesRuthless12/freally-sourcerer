//! JSON-backed settings store. Real persistence; the surface is the
//! subset of SettingsModel that Phase 11 actually consumes (Phase 12
//! settings dialog will extend).

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{Manager, State};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeChoice {
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RowDensity {
    Compact,
    Comfortable,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColumnId {
    Name,
    Path,
    Size,
    Modified,
    Type,
    Ext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnSpec {
    pub id: ColumnId,
    pub width_px: u32,
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnProfile {
    pub id: String,
    pub name: String,
    pub columns: Vec<ColumnSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointSpec {
    pub name: String,
    pub kind: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThumbSize {
    Xl,
    L,
    M,
    Details,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnTopMode {
    Never,
    Always,
    WhileSearching,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchOpts {
    pub match_case: bool,
    pub match_whole_word: bool,
    pub match_path: bool,
    pub match_diacritics: bool,
    pub enable_regex: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsState {
    pub theme: ThemeChoice,
    pub locale: String,
    pub show_status_bar: bool,
    pub show_size_in_status_bar: bool,
    pub show_timing_badges: bool,
    pub show_preview: bool,
    pub row_density: RowDensity,
    #[serde(default = "default_thumb_size")]
    pub thumb_size: ThumbSize,
    pub active_column_profile: String,
    pub column_profiles: Vec<ColumnProfile>,
    pub lens_visibility: HashMap<String, bool>,
    #[serde(default)]
    pub search_opts: SearchOpts,
    #[serde(default = "default_on_top")]
    pub on_top: OnTopMode,
    #[serde(default = "default_zoom")]
    pub zoom: f64,
    pub hotkey: String,
    pub endpoint: EndpointSpec,
    #[serde(default)]
    pub extractor_modes: HashMap<String, String>,
    pub first_run_complete: bool,
    pub privacy_mode: bool,
    /// Phase 12 top-level fields — every PRD §8.2-§8.27 control whose
    /// value lives in SettingsState lands here. The TS side owns the
    /// typed schema; the Rust side persists + clamps where needed.
    /// Captured via `#[serde(flatten)]` so the on-disk JSON stays flat.
    #[serde(flatten)]
    pub extras: HashMap<String, serde_json::Value>,
}

fn default_thumb_size() -> ThumbSize {
    ThumbSize::Details
}

fn default_on_top() -> OnTopMode {
    OnTopMode::Never
}

fn default_zoom() -> f64 {
    1.0
}

impl SettingsState {
    pub fn defaults() -> Self {
        let mut lens_visibility = HashMap::new();
        lens_visibility.insert("filename".into(), true);
        lens_visibility.insert("content".into(), true);
        lens_visibility.insert("audio".into(), true);
        lens_visibility.insert("similarity".into(), true);
        Self {
            theme: ThemeChoice::System,
            locale: "en".into(),
            show_status_bar: true,
            show_size_in_status_bar: true,
            show_timing_badges: true,
            show_preview: false,
            row_density: RowDensity::Compact,
            active_column_profile: "default".into(),
            column_profiles: vec![ColumnProfile {
                id: "default".into(),
                name: "Default".into(),
                columns: vec![
                    ColumnSpec {
                        id: ColumnId::Name,
                        width_px: 360,
                        visible: true,
                    },
                    ColumnSpec {
                        id: ColumnId::Path,
                        width_px: 400,
                        visible: true,
                    },
                    ColumnSpec {
                        id: ColumnId::Size,
                        width_px: 100,
                        visible: true,
                    },
                    ColumnSpec {
                        id: ColumnId::Modified,
                        width_px: 180,
                        visible: true,
                    },
                    ColumnSpec {
                        id: ColumnId::Type,
                        width_px: 120,
                        visible: true,
                    },
                    ColumnSpec {
                        id: ColumnId::Ext,
                        width_px: 80,
                        visible: true,
                    },
                ],
            }],
            lens_visibility,
            search_opts: SearchOpts::default(),
            on_top: OnTopMode::Never,
            zoom: 1.0,
            thumb_size: ThumbSize::Details,
            hotkey: default_hotkey(),
            endpoint: EndpointSpec {
                name: "Local DB".into(),
                kind: "local".into(),
            },
            extractor_modes: HashMap::new(),
            first_run_complete: false,
            privacy_mode: false,
            extras: phase_12_default_extras(),
        }
    }
}

/// Defaults for every Phase 12 top-level setting. Matches the shape the
/// TS-side SettingsStore expects on first launch. Keep this list in
/// lockstep with `SettingsState` in `apps/sourcerer-ui/src/lib/ipc/types.ts`.
fn phase_12_default_extras() -> HashMap<String, serde_json::Value> {
    let lens_vis = serde_json::json!({
        "filename": true,
        "content": true,
        "audio": true,
        "similarity": true,
    });
    let lens_limits = serde_json::json!({
        "filename": 200,
        "content": 50,
        "audio": 20,
        "similarity": 10,
    });
    let mut m = HashMap::new();
    let pairs: Vec<(&str, serde_json::Value)> = vec![
        // §8.2 General → UI
        ("run_in_background", true.into()),
        ("show_tray_icon", true.into()),
        ("single_click_tray", false.into()),
        ("open_new_window_from_tray", false.into()),
        ("open_new_window_when_launching", false.into()),
        ("search_as_you_type", true.into()),
        ("select_search_on_mouse_click", true.into()),
        ("focus_search_on_activate", true.into()),
        ("full_row_select", true.into()),
        ("single_click_open", "system_settings".into()),
        ("underline_icon_titles", "system_settings".into()),
        ("animated_theme_crossfade", true.into()),
        // §8.3 General → Home
        ("default_match_case", "use_last".into()),
        ("default_match_whole_word", "use_last".into()),
        ("default_match_path", "use_last".into()),
        ("default_match_diacritics", "use_last".into()),
        ("default_match_regex", "use_last".into()),
        ("default_search", "".into()),
        ("default_filter", "use_last".into()),
        ("default_sort", "use_last".into()),
        ("default_view", "use_last".into()),
        ("default_index", "local".into()),
        ("default_file_list", "".into()),
        (
            "default_https_endpoint",
            serde_json::json!({ "url": "", "token_fingerprint": "" }),
        ),
        ("default_lens_visibility", lens_vis.clone()),
        ("default_lens_result_limits", lens_limits),
        // §8.4 General → Search
        ("fast_ascii_search", true.into()),
        ("match_path_when_term_contains_separator", true.into()),
        ("match_whole_filename_with_wildcards", true.into()),
        ("allow_literal_operators", true.into()),
        ("allow_round_bracket_grouping", true.into()),
        ("expand_environment_variables", true.into()),
        ("replace_forward_with_backslashes", false.into()),
        ("operator_precedence", "or_first".into()),
        ("strict_everything_mode", false.into()),
        ("auto_detect_regex", true.into()),
        ("modifier_completions", true.into()),
        ("show_parse_tree_on_hover", false.into()),
        // §8.5 General → Results
        ("hide_results_when_empty", false.into()),
        ("clear_selection_on_search", true.into()),
        ("close_window_on_execute", false.into()),
        ("open_path_with_double_click_in_path_column", false.into()),
        ("automatically_scroll_view", true.into()),
        ("double_quote_copy_as_path", false.into()),
        ("do_not_select_extension_when_renaming", true.into()),
        ("sort_date_descending_first", true.into()),
        ("sort_size_descending_first", true.into()),
        ("result_list_focus", "clamp".into()),
        ("load_icon_priority", "normal".into()),
        ("load_thumbnail_priority", "normal".into()),
        ("load_extended_information_priority", "normal".into()),
        ("group_by_lens", true.into()),
        ("show_snippet_preview_inline", true.into()),
        // §8.6 General → View
        ("double_buffer", true.into()),
        ("alternate_row_color", false.into()),
        ("show_row_mouseover", false.into()),
        ("show_highlighted_search_terms", true.into()),
        ("show_selected_item_in_status_bar", true.into()),
        ("show_result_count_with_selection_count", false.into()),
        ("show_tooltips", true.into()),
        ("update_display_immediately_after_scrolling", true.into()),
        ("size_format", "auto_binary".into()),
        ("selection_rectangle", "system".into()),
        ("show_lufs_codec_length_badges", true.into()),
        ("show_minhash_similarity_score", true.into()),
        ("preview_pane", "right".into()),
        // §8.7 Context Menu — defaults populate every entry as `show`
        // with an empty command-string macro; the user sets specifics.
        (
            "context_menu",
            context_menu_defaults(),
        ),
        // §8.8 Fonts & Colors
        ("fonts_and_colors", fonts_and_colors_defaults()),
        // §8.9 Keyboard
        (
            "keyboard",
            serde_json::json!({
                "new_window_hotkey": "",
                "show_window_hotkey": "",
                "toggle_window_hotkey": "",
                "per_action": [],
            }),
        ),
        // §8.11 Indexes top-level
        (
            "index_core",
            serde_json::json!({
                "database_location": "",
                "multi_user_database_filename": false,
                "compress_database": true,
                "index_recent_changes": true,
                "index_file_size": true,
                "fast_size_sort": true,
                "index_folder_size": false,
                "fast_folder_size_sort": false,
                "index_date_created": false,
                "fast_date_created_sort": false,
                "index_date_modified": true,
                "fast_date_modified_sort": true,
                "index_date_accessed": false,
                "fast_date_accessed_sort": false,
                "index_attributes": false,
                "fast_attributes_sort": false,
                "fast_path_sort": true,
                "fast_extension_sort": false,
                "integrity_policy": "strict",
                "memory_budget_mb": 1024,
                "background_throttle": "off",
            }),
        ),
        // §8.16 Lenses → Filename
        (
            "lens_filename",
            serde_json::json!({
                "trigram_aggressiveness": "normal",
                "suffix_array_memory_mb": 256,
                "wildcard_expansion_limit": 100000,
                "regex_timeout_ms": 100,
            }),
        ),
        // §8.17 Lenses → Content
        (
            "lens_content",
            serde_json::json!({
                "enabled": true,
                "per_format": {},
                "time_budget_ms": 5000,
                "memory_ceiling_mb": 256,
                "snippet_length": 200,
                "stop_words_language": "auto",
                "re_extract_on_settings_change": false,
                "verify_blob_checksums_on_read": true,
            }),
        ),
        // §8.18 Lenses → Audio
        (
            "lens_audio",
            serde_json::json!({
                "enabled": true,
                "per_format": {},
                "lufs_reference": "ebu_r128",
                "peak_compute": "true_peak",
                "silence_threshold_dbfs": -60,
                "re_extract_on_modify": true,
            }),
        ),
        // §8.19 Lenses → Similarity
        (
            "lens_similarity",
            serde_json::json!({
                "enabled": true,
                "signature_size": 128,
                "bands": 16,
                "recall_threshold": 0.95,
                "result_cap": 50,
            }),
        ),
        // §8.23 Privacy & Updates
        (
            "privacy_and_updates",
            serde_json::json!({
                "auto_update": "default",
                "pre_release_channel": false,
            }),
        ),
        // §8.24 Logs & Debug
        (
            "logs_and_debug",
            serde_json::json!({
                "log_level": "info",
                "log_file_location": "",
                "log_retention_mb": 50,
                "show_debug_overlay": false,
            }),
        ),
        // §8.26 Locale
        (
            "locale_settings",
            serde_json::json!({
                "locale": "en",
                "rtl_preview": false,
                "date_format": "os",
                "date_format_custom": "",
                "number_format": "os",
                "thousands_separator": ",",
                "decimal_separator": ".",
            }),
        ),
    ];
    for (k, v) in pairs {
        m.insert(k.to_string(), v);
    }
    m
}

fn context_menu_defaults() -> serde_json::Value {
    fn entry(cmd: &str) -> serde_json::Value {
        serde_json::json!({ "visibility": "show", "command": cmd })
    }
    serde_json::json!({
        "open_folders": entry(""),
        "open_files": entry(""),
        "open_path": entry(""),
        "explore": entry(""),
        "explore_path": entry(""),
        "copy_name": entry(""),
        "copy_path": entry(""),
        "copy_full_name": entry(""),
        "reveal_in_sourcerer": entry(""),
        "send_to_sourcerer": entry(""),
    })
}

fn fonts_and_colors_defaults() -> serde_json::Value {
    fn item_state() -> serde_json::Value {
        serde_json::json!({
            "fg": null,
            "bg": null,
            "bold": false,
            "italic": false,
        })
    }
    serde_json::json!({
        "font": "default",
        "size_px": 13,
        "states": {
            "normal": item_state(),
            "highlighted": item_state(),
            "current_sort": item_state(),
            "current_sort_highlighted": item_state(),
            "selected": item_state(),
            "selected_highlighted": item_state(),
            "inactive_selected": item_state(),
            "inactive_selected_highlighted": item_state(),
        },
        "per_lens_accent": {
            "filename": null,
            "content": null,
            "audio": null,
            "similarity": null,
        },
        "theme_inheritance_toggle": true,
    })
}

fn default_hotkey() -> String {
    // Use Tauri's accelerator-parser names everywhere — `Super` maps to the
    // Win key on Windows and the Super key on Linux. M12 fix: don't use
    // "Win" anywhere because the accelerator parser only knows "Super" /
    // "Meta".
    if cfg!(target_os = "macos") {
        "Alt+Space".into()
    } else {
        "Super+Space".into()
    }
}

pub struct SettingsStore {
    pub path: PathBuf,
    pub state: Mutex<SettingsState>,
}

impl SettingsStore {
    pub fn new(app: &tauri::AppHandle) -> Self {
        let dir = app
            .path()
            .app_data_dir()
            .unwrap_or_else(|_| std::env::temp_dir().join("sourcerer"));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("settings.json");
        let state = std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str::<SettingsState>(&s).ok())
            .unwrap_or_else(SettingsState::defaults);
        Self {
            path,
            state: Mutex::new(state),
        }
    }
}

fn write_to_disk(path: &PathBuf, state: &SettingsState) {
    // M2 fix: tmp+rename so a crash mid-write can't truncate the file and
    // silently reset settings to defaults on next launch.
    let json = match serde_json::to_string_pretty(state) {
        Ok(j) => j,
        Err(_) => return,
    };
    let tmp = path.with_extension("json.tmp");
    if std::fs::write(&tmp, &json).is_err() {
        return;
    }
    let _ = std::fs::rename(&tmp, path);
}

/// Cross-module write helper for sibling commands that mutate settings
/// out-of-band (e.g. `extractors_set_mode` persists per-id mode here).
pub fn write_to_disk_pub(path: &PathBuf, state: &SettingsState) {
    write_to_disk(path, state);
}

#[tauri::command]
pub fn settings_get(store: State<'_, SettingsStore>) -> SettingsState {
    store.state.lock().unwrap().clone()
}

// H17: top-level patch keys must belong to the SettingsState schema. Any
// extra keys are rejected so a hostile patch can't bloat the on-disk file.
// Phase 12 added 70+ keys for the settings dialog — they're all listed here
// so the allowlist still catches typos and never-touched keys.
const ALLOWED_PATCH_KEYS: &[&str] = &[
    // Phase 11 carry-over.
    "theme",
    "locale",
    "show_status_bar",
    "show_size_in_status_bar",
    "show_timing_badges",
    "show_preview",
    "row_density",
    "thumb_size",
    "active_column_profile",
    "column_profiles",
    "lens_visibility",
    "search_opts",
    "on_top",
    "zoom",
    "hotkey",
    "endpoint",
    "extractor_modes",
    "first_run_complete",
    "privacy_mode",
    // §8.2 General → UI.
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
    "animated_theme_crossfade",
    // §8.3 General → Home.
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
    "default_lens_result_limits",
    // §8.4 General → Search.
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
    "show_parse_tree_on_hover",
    // §8.5 General → Results.
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
    // §8.6 General → View.
    "double_buffer",
    "alternate_row_color",
    "show_row_mouseover",
    "show_highlighted_search_terms",
    "show_selected_item_in_status_bar",
    "show_result_count_with_selection_count",
    "show_tooltips",
    "update_display_immediately_after_scrolling",
    "size_format",
    "selection_rectangle",
    "show_lufs_codec_length_badges",
    "show_minhash_similarity_score",
    "preview_pane",
    // §8.7-§8.9 grouped panels.
    "context_menu",
    "fonts_and_colors",
    "keyboard",
    // §8.11 Indexes top-level.
    "index_core",
    // §8.16-§8.19 Lenses.
    "lens_filename",
    "lens_content",
    "lens_audio",
    "lens_similarity",
    // §8.23-§8.26.
    "privacy_and_updates",
    "logs_and_debug",
    "locale_settings",
];

const MAX_HOTKEY_LEN: usize = 64;
const MAX_LOCALE_LEN: usize = 16;
const MAX_PROFILE_NAME_LEN: usize = 128;
const MIN_COL_WIDTH_PX: u32 = 60;
const MAX_COL_WIDTH_PX: u32 = 800;
const MIN_ZOOM: f64 = 0.5;
const MAX_ZOOM: f64 = 2.5;

#[tauri::command]
pub fn settings_set(
    patch: serde_json::Value,
    store: State<'_, SettingsStore>,
) -> Result<SettingsState, String> {
    if let serde_json::Value::Object(ref obj) = patch {
        for k in obj.keys() {
            if !ALLOWED_PATCH_KEYS.contains(&k.as_str()) {
                return Err(format!("settings_set: unknown key `{k}`"));
            }
        }
    } else if !patch.is_null() {
        return Err("settings_set: patch must be a JSON object".into());
    }

    let mut guard = store.state.lock().unwrap();
    let mut current = serde_json::to_value(&*guard).map_err(|e| e.to_string())?;
    merge(&mut current, patch);
    let mut next: SettingsState =
        serde_json::from_value(current).map_err(|e| format!("invalid settings: {e}"))?;

    validate_and_clamp(&mut next)?;

    *guard = next.clone();
    write_to_disk(&store.path, &guard);
    Ok(next)
}

fn validate_and_clamp(s: &mut SettingsState) -> Result<(), String> {
    if s.hotkey.len() > MAX_HOTKEY_LEN {
        return Err(format!(
            "hotkey too long ({} > {MAX_HOTKEY_LEN})",
            s.hotkey.len()
        ));
    }
    if s.locale.len() > MAX_LOCALE_LEN {
        return Err(format!(
            "locale too long ({} > {MAX_LOCALE_LEN})",
            s.locale.len()
        ));
    }
    if !s.zoom.is_finite() {
        return Err("zoom must be finite".into());
    }
    s.zoom = s.zoom.clamp(MIN_ZOOM, MAX_ZOOM);
    for profile in &mut s.column_profiles {
        if profile.name.len() > MAX_PROFILE_NAME_LEN {
            return Err("column profile name too long".into());
        }
        for col in &mut profile.columns {
            col.width_px = col.width_px.clamp(MIN_COL_WIDTH_PX, MAX_COL_WIDTH_PX);
        }
    }
    Ok(())
}

/// Phase 12 settings → daemon apply hook. Forwards the index-affecting
/// fields to `sourcerer-indexd::settings.apply` so toggles like
/// extractor mode / memory budget / time budget / auto-include-volume
/// flags take effect live.
#[tauri::command]
pub fn settings_apply_to_daemon(state: SettingsState) -> Result<(), String> {
    let daemon = crate::daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    let extras = &state.extras;
    let lens_content = extras.get("lens_content");
    let volumes_config = extras.get("volumes_config");
    let mut payload = serde_json::Map::new();
    payload.insert(
        "default_extractor_mode".into(),
        serde_json::Value::String(
            if state.privacy_mode {
                "lazy".to_string()
            } else {
                lens_content
                    .and_then(|v| v.get("enabled"))
                    .and_then(|v| v.as_bool())
                    .map(|on| if on { "lazy".to_string() } else { "disabled".to_string() })
                    .unwrap_or_else(|| "lazy".to_string())
            },
        ),
    );
    payload.insert(
        "extractor_memory_mb".into(),
        serde_json::Value::Number(
            (lens_content
                .and_then(|v| v.get("memory_ceiling_mb"))
                .and_then(|v| v.as_u64())
                .unwrap_or(256))
            .into(),
        ),
    );
    payload.insert(
        "extractor_time_ms".into(),
        serde_json::Value::Number(
            (lens_content
                .and_then(|v| v.get("time_budget_ms"))
                .and_then(|v| v.as_u64())
                .unwrap_or(5000))
            .into(),
        ),
    );
    if let Some(vc) = volumes_config {
        if let Some(b) = vc.get("auto_include_fixed").and_then(|v| v.as_bool()) {
            payload.insert("auto_include_fixed".into(), serde_json::Value::Bool(b));
        }
        if let Some(b) = vc.get("auto_include_removable").and_then(|v| v.as_bool()) {
            payload.insert("auto_include_removable".into(), serde_json::Value::Bool(b));
        }
        if let Some(b) = vc.get("auto_remove_offline").and_then(|v| v.as_bool()) {
            payload.insert("auto_remove_offline".into(), serde_json::Value::Bool(b));
        }
    }
    daemon
        .call_void("settings.apply", serde_json::Value::Object(payload))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn settings_reset(store: State<'_, SettingsStore>) -> SettingsState {
    let next = SettingsState::defaults();
    *store.state.lock().unwrap() = next.clone();
    write_to_disk(&store.path, &next);
    next
}

/// Maximum nesting the merge helper will descend before treating the
/// patch leaf-replace. Defense-in-depth against an attacker JSON whose
/// recursion depth slips past serde_json's parser limit (M4 fix).
const MAX_MERGE_DEPTH: u32 = 32;

fn merge(target: &mut serde_json::Value, patch: serde_json::Value) {
    merge_with_depth(target, patch, 0)
}

fn merge_with_depth(target: &mut serde_json::Value, patch: serde_json::Value, depth: u32) {
    use serde_json::Value;
    if depth > MAX_MERGE_DEPTH {
        *target = patch;
        return;
    }
    match (target, patch) {
        (Value::Object(a), Value::Object(b)) => {
            for (k, v) in b {
                merge_with_depth(a.entry(k).or_insert(Value::Null), v, depth + 1);
            }
        }
        (a, b) => *a = b,
    }
}
