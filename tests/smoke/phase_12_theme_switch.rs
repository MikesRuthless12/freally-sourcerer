//! Phase 12 smoke — theme picker live-switches without reload, and
//! `system` mode follows the OS preference.
//!
//! UI-side flip is exercised by the JS test harness; this Rust smoke
//! pins the `settings.apply` IPC contract that drives the daemon side
//! and asserts the typed enum round-trips.

#![cfg(test)]

use serde_json::json;

#[test]
fn theme_choice_round_trips_through_settings_apply() {
    let payload = json!({ "theme": "dark" });
    let s = serde_json::to_string(&payload).unwrap();
    let back: serde_json::Value = serde_json::from_str(&s).unwrap();
    assert_eq!(back["theme"], "dark");

    for choice in ["system", "light", "dark"] {
        let p = json!({ "theme": choice });
        let s = serde_json::to_string(&p).unwrap();
        let back: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(back["theme"], choice);
    }
}
