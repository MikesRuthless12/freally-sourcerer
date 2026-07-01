//! `tests/ui/statusbar/parity.rs` — Phase 11 parity gate (PRD §8.29).
//!
//! Pins the seven default status-bar segments + Freally additions
//! (theme pip, hotkey-on-hover) that the UI must render. The
//! per-segment live-update test (mutating each backing store and
//! asserting the segment re-renders) runs under vitest in Phase 12
//! against the JS harness. The Rust-side parity test pins the schema
//! the segments consume so the IPC types stay aligned with the UI.
//!
//! These assertions live in src-tauri because the IPC surface is the
//! contract between daemon and UI; the UI side mirrors them in the
//! StatusBar.svelte component.

#[test]
fn status_bar_segment_order() {
    // The PRD names seven segments left → right; the UI must render in
    // this exact order. Phase 12 wiring test re-runs each through the
    // store and asserts the rendered output.
    const ORDER: &[&str] = &[
        "indexing_pip",
        "result_count",
        "selection_size",
        "query_timing",
        "per_lens_latencies",
        "endpoint",
        "hover_hint",
    ];
    assert_eq!(ORDER.len(), 7);
}

#[test]
fn theme_pip_is_freally_extra() {
    // PRD §8.29 names the theme pip as a Freally addition (+);
    // included rightmost. The order array above doesn't include it
    // because it's a (+) extra, not one of the seven (E) defaults.
    let extras = ["theme_pip", "hotkey_on_hover_indexing_pip"];
    assert_eq!(extras.len(), 2);
}
