//! `tests/ui/menubar/parity.rs` — Phase 11 parity gate (PRD §8.30).
//!
//! Exhaustive pin: every PRD §8.28 menu item ID must be present in the
//! Rust mirror in `src/menu_spec.rs`. The Rust spec drives the macOS
//! native menu; the TS spec at `src/lib/commands/menu_spec.ts` drives
//! the in-window menu on Win/Linux. The TS list of `CommandId`s is
//! re-asserted here (kept in lockstep manually; if either spec drifts
//! this test fails).
//!
//! Phase 12 wiring tests (per-control activation + state-effect) land
//! under the JS test harness (vitest + tauri-driver) since they need a
//! running webview; the Rust side validates structural parity only.

use freally_ui_lib::menu_spec::{all_command_ids, menu_bar};

/// Every CommandId enumerated in `lib/commands/ids.ts`. Mirrored manually
/// — this test fails the moment either side drifts.
const COMMAND_IDS: &[&str] = &[
    // File
    "file.new_window",
    "file.open_file_list",
    "file.close_file_list",
    "file.close",
    "file.export_results",
    "file.export_index_bundle",
    "file.exit",
    // Edit
    "edit.cut",
    "edit.copy",
    "edit.paste",
    "edit.copy_to_folder",
    "edit.move_to_folder",
    "edit.select_all",
    "edit.invert_selection",
    "edit.advanced.copy_full_name",
    "edit.advanced.copy_path",
    "edit.advanced.copy_filename",
    "edit.advanced.copy_as_json",
    "edit.advanced.copy_with_metadata",
    "edit.advanced.copy_as_bundle_ref",
    // View
    "view.filters",
    "view.preview",
    "view.status_bar",
    "view.thumbs.xl",
    "view.thumbs.l",
    "view.thumbs.m",
    "view.details",
    "view.window_size.small",
    "view.window_size.medium",
    "view.window_size.large",
    "view.window_size.auto",
    "view.zoom.in",
    "view.zoom.out",
    "view.zoom.reset",
    "view.sort.name",
    "view.sort.path",
    "view.sort.size",
    "view.sort.ext",
    "view.sort.type",
    "view.sort.modified",
    "view.sort.created",
    "view.sort.accessed",
    "view.sort.attributes",
    "view.sort.recently_changed",
    "view.sort.run_count",
    "view.sort.run_date",
    "view.sort.file_list_filename",
    "view.sort.lufs",
    "view.sort.length",
    "view.sort.similarity",
    "view.sort.ascending",
    "view.sort.descending",
    "view.go_to",
    "view.refresh",
    "view.theme.system",
    "view.theme.light",
    "view.theme.dark",
    "view.lens.filename",
    "view.lens.content",
    "view.lens.audio",
    "view.lens.similarity",
    "view.on_top.never",
    "view.on_top.always",
    "view.on_top.while_searching",
    // Search
    "search.match_case",
    "search.match_whole_word",
    "search.match_path",
    "search.match_diacritics",
    "search.enable_regex",
    "search.advanced",
    "search.add_to_filters",
    "search.organize_filters",
    "search.filter.everything",
    "search.filter.audio",
    "search.filter.compressed",
    "search.filter.document",
    "search.filter.executable",
    "search.filter.folder",
    "search.filter.picture",
    "search.filter.video",
    "search.filter.custom",
    // Bookmarks
    "bookmarks.add",
    "bookmarks.organize",
    // Tools
    "tools.connect_endpoint",
    "tools.disconnect_endpoint",
    "tools.file_list_editor",
    "tools.custom_extractor_manager",
    "tools.verify_index",
    "tools.compact_index",
    "tools.rebuild_index",
    "tools.options",
    // Help
    "help.help",
    "help.search_syntax",
    "help.regex_syntax",
    "help.audio_modifier_reference",
    "help.similarity_modifier_reference",
    "help.command_line_options",
    "help.website",
    "help.check_for_updates",
    "help.sponsor",
    "help.about",
];

#[test]
fn rust_spec_covers_every_command_id() {
    let rust_ids: std::collections::HashSet<&str> = all_command_ids().into_iter().collect();
    for &expected in COMMAND_IDS {
        assert!(
            rust_ids.contains(expected),
            "Rust menu_spec missing command id `{expected}` — spec drifted from PRD §8.28"
        );
    }
}

#[test]
fn rust_spec_does_not_introduce_unknown_command_ids() {
    let known: std::collections::HashSet<&str> = COMMAND_IDS.iter().copied().collect();
    for id in all_command_ids() {
        assert!(
            known.contains(id),
            "Rust menu_spec carries unknown command id `{id}` — must round-trip into ids.ts"
        );
    }
}

#[test]
fn rust_spec_seven_top_level_roots() {
    // PRD §8.28: File / Edit / View / Search / Bookmarks / Tools / Help
    let labels: Vec<&str> = menu_bar().iter().map(|r| r.label).collect();
    assert_eq!(
        labels,
        vec![
            "File",
            "Edit",
            "View",
            "Search",
            "Bookmarks",
            "Tools",
            "Help"
        ],
        "menu-bar root order drifted from PRD §8.28"
    );
}

#[test]
fn rust_spec_has_no_duplicate_ids() {
    let mut seen = std::collections::HashSet::new();
    for id in all_command_ids() {
        assert!(
            seen.insert(id),
            "duplicate command id in Rust menu_spec: {id}"
        );
    }
}
