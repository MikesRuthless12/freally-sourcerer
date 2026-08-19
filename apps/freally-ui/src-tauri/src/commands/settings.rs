//! JSON-backed settings store. Real persistence; the surface is the
//! subset of SettingsModel that Phase 11 actually consumes (Phase 12
//! settings dialog will extend).

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

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

/// The `Search →` menu's toggles.
///
/// Every field here is round-tripped through `settings_set`, which
/// re-serializes this struct and parses the result back. A flag the UI
/// knows about but this struct does not is therefore *dropped on every
/// settings write* — which is what had been happening to
/// `match_phonetic` since Build 2 added it to the TypeScript side only.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchOpts {
    pub match_case: bool,
    pub match_whole_word: bool,
    pub match_path: bool,
    pub match_diacritics: bool,
    /// SRC-M12 — match CJK names through their phonetic reading.
    #[serde(default)]
    pub match_phonetic: bool,
    /// SRC-M23 — `foobar` finds `foo-bar`.
    #[serde(default)]
    pub ignore_punctuation: bool,
    /// SRC-M23 — `myreport` finds `my report`.
    #[serde(default)]
    pub ignore_whitespace: bool,
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
/// lockstep with `SettingsState` in `apps/freally-ui/src/lib/ipc/types.ts`.
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
        // SRC-M06 user-defined result actions. Empty until the user
        // adds one in Settings → General → Custom Commands.
        ("custom_commands", serde_json::json!([])),
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
        // SRC-M24 — on by default: byte ordering puts `file10` ahead of
        // `file2`, which is the wrong answer often enough that this is
        // the sort users expect. Off restores raw ordering.
        ("natural_sort", true.into()),
        ("result_list_focus", "clamp".into()),
        ("load_icon_priority", "normal".into()),
        ("load_thumbnail_priority", "normal".into()),
        ("load_extended_information_priority", "normal".into()),
        ("group_by_lens", true.into()),
        ("show_snippet_preview_inline", true.into()),
        // SRC-M22 — the optional left sidebar (View → Sidebar).
        ("show_sidebar", false.into()),
        // Most-recent-first, capped by the UI. Not recorded at all when
        // Privacy Mode or Search History is off.
        ("recent_searches", serde_json::Value::Array(Vec::new())),
        // Bookmark ids in the order the sidebar shows them. Kept here
        // rather than on the bookmark records so drag-reorder does not
        // need a new write path through the bookmarks IPC.
        (
            "sidebar_bookmark_order",
            serde_json::Value::Array(Vec::new()),
        ),
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
        ("context_menu", context_menu_defaults()),
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
        "reveal_in_freally": entry(""),
        "send_to_freally": entry(""),
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
        let path = app_data_root(app).join("settings.json");
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

/// Where this install keeps its own files.
///
/// SRC-M17 — a portable install keeps them beside the binary; otherwise
/// it is the platform's app-data directory, with the temp directory as
/// a last resort so a broken `app_data_dir` degrades to a working app
/// rather than a panic. Created on the way out, so callers can join a
/// filename onto it and write.
pub fn app_data_root(app: &tauri::AppHandle) -> PathBuf {
    let dir = freally_rpc::portable::data_dir().unwrap_or_else(|| {
        app.path()
            .app_data_dir()
            .unwrap_or_else(|_| std::env::temp_dir().join("freally"))
    });
    let _ = std::fs::create_dir_all(&dir);
    dir
}

#[tauri::command]
pub fn settings_get(store: State<'_, SettingsStore>) -> SettingsState {
    store.state.lock().unwrap().clone()
}

/// Top-level patch keys that belong to the `SettingsState` schema.
///
/// H17: anything outside the schema is rejected, so a hostile patch
/// cannot bloat the on-disk file.
///
/// Derived from the schema rather than hand-listed. The Phase-12 half of
/// the old list was `phase_12_default_extras().keys()` written out a
/// second time, and the two had already drifted: `custom_commands` had a
/// default and a whole settings panel but never made it onto the list.
/// Because `settings_set` rejects a patch on its first unknown key and
/// the UI's `flush()` sends the entire state, that one omission failed
/// *every* settings write, not just writes that touched it.
fn allowed_patch_keys() -> &'static HashSet<String> {
    static KEYS: OnceLock<HashSet<String>> = OnceLock::new();
    KEYS.get_or_init(|| {
        // The named fields and the `#[serde(flatten)] extras` both land
        // in this one object, so serializing the defaults is the whole
        // accepted set in one step.
        let mut keys: HashSet<String> = match serde_json::to_value(SettingsState::defaults()) {
            Ok(serde_json::Value::Object(map)) => map.into_iter().map(|(k, _)| k).collect(),
            _ => HashSet::new(),
        };
        keys.extend(OPTIONAL_PATCH_KEYS.iter().map(|k| (*k).to_string()));
        keys
    })
}

/// Schema keys with no default: they exist only once the user has
/// produced a value, so they cannot be read off `defaults()`.
const OPTIONAL_PATCH_KEYS: &[&str] = &[
    // Written by the View → Window Size presets, restored on bootstrap.
    "window_size",
];

const MAX_HOTKEY_LEN: usize = 64;
const MAX_LOCALE_LEN: usize = 16;
const MAX_PROFILE_NAME_LEN: usize = 128;
const MIN_COL_WIDTH_PX: u32 = 60;
const MAX_COL_WIDTH_PX: u32 = 800;
const MIN_ZOOM: f64 = 0.5;
const MAX_ZOOM: f64 = 2.5;
// `custom_commands` reached `settings_set` completely unvalidated —
// unbounded array, unbounded strings — because until this build it never
// reached `settings_set` at all: the allowlist rejected it, which is the
// bug that stopped any setting persisting. H17's whole job is keeping a
// hostile patch from bloating the on-disk file, and this was the one
// allowlisted key it did not cover.
const MAX_CUSTOM_COMMANDS: usize = 64;
const MAX_CUSTOM_COMMAND_TEXT: usize = 512;
const MAX_CUSTOM_COMMAND_ARGS: usize = 32;

#[tauri::command]
pub fn settings_set(
    patch: serde_json::Value,
    store: State<'_, SettingsStore>,
) -> Result<SettingsState, String> {
    if let serde_json::Value::Object(ref obj) = patch {
        for k in obj.keys() {
            if !allowed_patch_keys().contains(k.as_str()) {
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

/// Shape- and size-check `custom_commands`.
///
/// This is the one allowlisted key whose value names a **program to
/// run** — `shell_actions::run_custom_command` spawns `program` with
/// `args` — so a malformed or unbounded value matters more here than
/// anywhere else in the schema.
///
/// Note what this deliberately does *not* claim. It makes the value
/// well-formed and bounded; it cannot make the key safe. Anything that
/// can write settings can name a program, and the webview is what writes
/// settings. `commands/shell_verbs.rs` takes a command *id* rather than a
/// body for exactly this reason, and that argument only holds as far as
/// the stored set is trustworthy. See `docs/SECURITY.md`.
fn validate_custom_commands(s: &SettingsState) -> Result<(), String> {
    let Some(raw) = s.extras.get("custom_commands") else {
        return Ok(());
    };
    let cmds: Vec<crate::shell_actions::CustomCommand> = serde_json::from_value(raw.clone())
        .map_err(|e| format!("custom_commands is malformed: {e}"))?;
    if cmds.len() > MAX_CUSTOM_COMMANDS {
        return Err(format!(
            "too many custom commands ({} > {MAX_CUSTOM_COMMANDS})",
            cmds.len()
        ));
    }
    for c in &cmds {
        if c.id.trim().is_empty() {
            return Err("custom command has no id".into());
        }
        for (field, value) in [("id", &c.id), ("name", &c.name), ("program", &c.program)] {
            if value.len() > MAX_CUSTOM_COMMAND_TEXT {
                return Err(format!(
                    "custom command {field} too long ({} > {MAX_CUSTOM_COMMAND_TEXT})",
                    value.len()
                ));
            }
        }
        // `extensions` is the other unbounded `Vec<String>` on this
        // struct. It never reaches a process — `applies_to` only compares
        // against it — but the point of these caps is what a hostile
        // patch can commit to disk, and an unbounded extension list
        // bloats the settings file exactly as well as an unbounded
        // `args` does. Same bound, one loop, so neither can be given the
        // cap without the other.
        for (field, list) in [("arguments", &c.args), ("extensions", &c.extensions)] {
            if list.len() > MAX_CUSTOM_COMMAND_ARGS {
                return Err(format!(
                    "custom command has too many {field} ({} > {MAX_CUSTOM_COMMAND_ARGS})",
                    list.len()
                ));
            }
            if list.iter().any(|v| v.len() > MAX_CUSTOM_COMMAND_TEXT) {
                return Err(format!("custom command {field} entry too long"));
            }
        }
    }
    Ok(())
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
    validate_custom_commands(s)?;
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
/// fields to `freally-indexd::settings.apply` so toggles like
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
        serde_json::Value::String(if state.privacy_mode {
            "lazy".to_string()
        } else {
            lens_content
                .and_then(|v| v.get("enabled"))
                .and_then(|v| v.as_bool())
                .map(|on| {
                    if on {
                        "lazy".to_string()
                    } else {
                        "disabled".to_string()
                    }
                })
                .unwrap_or_else(|| "lazy".to_string())
        }),
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

#[cfg(test)]
mod patch_key_tests {
    use super::*;

    #[test]
    fn the_keys_the_ui_actually_writes_are_all_accepted() {
        // The allowlist and the defaults were two hand-maintained lists
        // that had to agree, and did not: `custom_commands` shipped a
        // default and a whole settings panel while the allowlist rejected
        // it. Since `settings_set` rejects a patch on its first unknown key
        // and the UI sends the entire state on every save, that one gap
        // meant no setting could be written at all.
        //
        // Named keys rather than `defaults().keys()`: the allowlist is
        // *derived* from the defaults, so checking one against the other
        // would only prove the derivation is a derivation. These are keys a
        // user can actually change, spread across the named struct fields,
        // the flattened Phase-12 extras, and the defaultless set.
        let allowed = allowed_patch_keys();
        for key in [
            "theme",           // named struct field
            "search_opts",     // named, nested
            "custom_commands", // flattened extra — the regression
            "natural_sort",    // flattened extra
            "recent_searches", // flattened extra, written per keystroke
            "window_size",     // no default; only in OPTIONAL_PATCH_KEYS
        ] {
            assert!(allowed.contains(key), "settings_set would reject `{key}`");
        }
    }

    #[test]
    fn optional_keys_do_not_outlive_their_schema_entry() {
        // `OPTIONAL_PATCH_KEYS` is the one hand-maintained half left, and
        // its whole justification is "the schema has no default for this".
        // A key that gains a default belongs in the derivation instead, and
        // one whose field was deleted is an allowlist entry with nothing
        // behind it — widening the H17 gate silently and forever.
        let defaults = match serde_json::to_value(SettingsState::defaults()).unwrap() {
            serde_json::Value::Object(m) => m,
            other => panic!("defaults did not serialize to an object: {other:?}"),
        };
        for key in OPTIONAL_PATCH_KEYS {
            assert!(
                !defaults.contains_key(*key),
                "`{key}` has a default now — drop it from OPTIONAL_PATCH_KEYS"
            );
        }
    }

    #[test]
    fn keys_with_no_default_are_still_writable() {
        // `window_size` only exists once the user has picked one, so it
        // cannot be read off the defaults and has to be named.
        assert!(allowed_patch_keys().contains("window_size"));
    }

    #[test]
    fn keys_outside_the_schema_are_still_rejected() {
        // The point of the allowlist: deriving it must not turn it into
        // "accept anything".
        let allowed = allowed_patch_keys();
        assert!(!allowed.contains("not_a_setting"));
        assert!(!allowed.contains("__proto__"));
        assert!(!allowed.contains(""));
    }
}

#[cfg(test)]
mod custom_command_bounds {
    use super::*;

    fn state_with(commands: serde_json::Value) -> SettingsState {
        let mut s = SettingsState::defaults();
        s.extras.insert("custom_commands".into(), commands);
        s
    }

    #[test]
    fn a_well_formed_command_passes() {
        let mut s = state_with(serde_json::json!([{
            "id": "open-in-vscode",
            "name": "Open in VS Code",
            "program": "code",
            "args": ["{path}"],
            "extensions": []
        }]));
        assert!(validate_and_clamp(&mut s).is_ok());
    }

    #[test]
    fn the_default_empty_list_passes() {
        // Every save sends the whole state, so this runs on literally
        // every settings write.
        let mut s = SettingsState::defaults();
        assert!(validate_and_clamp(&mut s).is_ok());
    }

    #[test]
    fn a_malformed_value_is_rejected_rather_than_stored() {
        // Before this build the key never reached validation, so anything
        // at all could be written under it and `configured_command` would
        // silently find nothing at read time.
        let mut s = state_with(serde_json::json!("not an array"));
        assert!(validate_and_clamp(&mut s).is_err());
    }

    #[test]
    fn oversized_values_are_rejected() {
        let big = "x".repeat(MAX_CUSTOM_COMMAND_TEXT + 1);
        let one = |program: &str, args: serde_json::Value| serde_json::json!([{ "id": "a", "name": "a", "program": program, "args": args }]);
        let mut s = state_with(one(&big, serde_json::json!([])));
        assert!(validate_and_clamp(&mut s).is_err(), "long program accepted");

        let mut s = state_with(one("sh", serde_json::json!([big.as_str()])));
        assert!(validate_and_clamp(&mut s).is_err(), "long arg accepted");

        let args: Vec<&str> = vec!["-c"; MAX_CUSTOM_COMMAND_ARGS + 1];
        let mut s = state_with(one("sh", serde_json::json!(args)));
        assert!(validate_and_clamp(&mut s).is_err(), "arg flood accepted");

        let exts: Vec<&str> = vec!["txt"; MAX_CUSTOM_COMMAND_ARGS + 1];
        let mut s = state_with(serde_json::json!([{
            "id": "a", "name": "a", "program": "sh", "extensions": exts
        }]));
        assert!(
            validate_and_clamp(&mut s).is_err(),
            "extension flood accepted"
        );

        let mut s = state_with(serde_json::json!([{
            "id": "a", "name": "a", "program": "sh", "extensions": [big.as_str()]
        }]));
        assert!(
            validate_and_clamp(&mut s).is_err(),
            "long extension accepted"
        );

        let many: Vec<serde_json::Value> = (0..MAX_CUSTOM_COMMANDS + 1)
            .map(|i| serde_json::json!({ "id": i.to_string(), "name": "a", "program": "sh" }))
            .collect();
        let mut s = state_with(serde_json::json!(many));
        assert!(
            validate_and_clamp(&mut s).is_err(),
            "command flood accepted"
        );
    }

    #[test]
    fn an_idless_command_is_rejected() {
        // `run_custom_command` resolves by id; a blank one is unreachable
        // by design and only bloats the file.
        let mut s = state_with(serde_json::json!([{
            "id": "  ", "name": "a", "program": "sh"
        }]));
        assert!(validate_and_clamp(&mut s).is_err());
    }
}
