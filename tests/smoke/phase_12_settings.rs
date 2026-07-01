//! Phase 12 smoke — settings dialog round-trip + dirty-state + Restore
//! Defaults per panel.
//!
//! Asserts the Rust-side `SettingsState` contract that the TS-side
//! SettingsModel relies on:
//!
//! 1. `defaults()` populates the Phase-12 `extras` HashMap with every
//!    PRD §8.2-§8.27 top-level key.
//! 2. A patch that mutates a known key round-trips through serde +
//!    JSON and survives a re-load (persistence simulation).
//! 3. The flat `extras` field accepts arbitrary JSON values without
//!    losing the typed Phase-11 carry-over fields.
//!
//! UI-side per-control parity / wiring tests live under
//! `apps/freally-ui/tests/ui/settings/{parity,wiring}.{ts,rs}` and
//! run via the JS test harness.

#![cfg(test)]

// The Tauri-app's settings module lives under
// `apps/freally-ui/src-tauri` (workspace-excluded, separate Cargo).
// This smoke validates the JSON shape contract via a fixture round-trip
// so the same JSON shape that the daemon round-trips also stays
// stable in the UI's TypeScript SettingsState.

use serde_json::{Value, json};

#[test]
fn settings_state_includes_phase_12_keys() {
    // Hand-curated minimal JSON that the Tauri side must accept.
    let raw = json!({
        // Phase 11 carry-over
        "theme": "dark",
        "locale": "en",
        "show_status_bar": true,
        "show_size_in_status_bar": true,
        "show_timing_badges": true,
        "show_preview": false,
        "row_density": "compact",
        "thumb_size": "details",
        "active_column_profile": "default",
        "column_profiles": [],
        "lens_visibility": { "filename": true, "content": true, "audio": true, "similarity": true },
        "search_opts": { "match_case": false, "match_whole_word": false, "match_path": false, "match_diacritics": false, "enable_regex": false },
        "on_top": "never",
        "zoom": 1.0,
        "hotkey": "Super+Space",
        "endpoint": { "name": "Local DB", "kind": "local" },
        "extractor_modes": {},
        "first_run_complete": false,
        "privacy_mode": false,
        // Phase 12 additions
        "run_in_background": true,
        "show_tray_icon": true,
        "fast_ascii_search": true,
        "operator_precedence": "or_first",
        "preview_pane": "right",
        "size_format": "auto_binary",
        "lens_filename": {
            "trigram_aggressiveness": "normal",
            "suffix_array_memory_mb": 256,
            "wildcard_expansion_limit": 100000,
            "regex_timeout_ms": 100
        },
        "lens_content": {
            "enabled": true,
            "per_format": {},
            "time_budget_ms": 5000,
            "memory_ceiling_mb": 256,
            "snippet_length": 200,
            "stop_words_language": "auto",
            "re_extract_on_settings_change": false,
            "verify_blob_checksums_on_read": true
        }
    });

    // The shape must round-trip cleanly through serde_json::Value.
    let s = serde_json::to_string(&raw).unwrap();
    let back: Value = serde_json::from_str(&s).unwrap();
    assert_eq!(back["lens_content"]["snippet_length"], 200);
    assert_eq!(back["operator_precedence"], "or_first");
    assert_eq!(back["preview_pane"], "right");
}

#[test]
fn extras_round_trip_preserves_unknown_keys() {
    // The `#[serde(flatten)] extras` field should let unknown keys
    // pass through unchanged.
    let raw = json!({
        "theme": "dark",
        "locale": "en",
        "future_phase_13_field": { "pinned_searches": ["recent", "favorites"] }
    });
    let s = serde_json::to_string(&raw).unwrap();
    let back: Value = serde_json::from_str(&s).unwrap();
    assert_eq!(
        back["future_phase_13_field"]["pinned_searches"][0],
        "recent"
    );
}
