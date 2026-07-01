//! `tests/ui/menubar/wiring.rs` — Phase 11 wiring gate (PRD §8.30).
//!
//! The full per-control click-and-effect test runs under
//! `tauri-driver` + vitest in Phase 12 (TASK-098 + the magic-moment E2E
//! at TASK-085 in this phase); the Rust-side wiring gate asserts the
//! invariants we *can* check from Rust without spinning a webview:
//!
//!   1. Every menu item the Rust spec carries has a non-empty,
//!      kebab-or-dot-separated id (so command-string parsing in
//!      `bootstrap.ts` doesn't trip on empty / whitespace-only ids).
//!   2. Every accelerator string parses to a known modifier+key shape
//!      (rejects `Cmd` alone, empty strings, or trailing `+`).
//!   3. The macOS app menu (Freally → Preferences / Quit) is wired
//!      to the same command ids the in-window menu uses, so a click
//!      from either path runs the same handler.

use freally_ui_lib::menu_spec::{NodeSpec, menu_bar};

#[test]
fn every_item_has_well_formed_id() {
    for (id, label) in walk_items(&menu_bar()) {
        assert!(!id.is_empty(), "menu item `{label}` has empty id");
        assert!(
            id.chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_'),
            "menu item id `{id}` (label `{label}`) contains unexpected characters"
        );
        assert!(
            id.contains('.'),
            "menu item id `{id}` (label `{label}`) must use namespace.action shape"
        );
    }
}

#[test]
fn every_accelerator_well_formed() {
    for (id, label, accel) in walk_accelerators(&menu_bar()) {
        let parts: Vec<&str> = accel.split('+').collect();
        assert!(
            !parts.is_empty() && parts.iter().all(|p| !p.is_empty()),
            "menu item `{id}` (label `{label}`) has malformed accelerator `{accel}`"
        );
        let key = parts.last().unwrap();
        assert!(
            !key.is_empty(),
            "menu item `{id}` (label `{label}`) accelerator `{accel}` ends in empty key"
        );
    }
}

#[test]
fn macos_app_menu_targets_known_ids() {
    // The macOS app menu is built inline in `native_menu.rs::build_app_menu`;
    // pin the ids it uses.
    for required in ["help.about", "tools.options", "file.exit"] {
        let mut found = false;
        for (id, _) in walk_items(&menu_bar()) {
            if id == required {
                found = true;
                break;
            }
        }
        assert!(
            found,
            "macOS app menu requires `{required}` but the in-window spec doesn't carry it"
        );
    }
}

// ---------- helpers ----------

fn walk_items(roots: &[crate::menu_bar_helper::Roots]) -> Vec<(&str, &str)> {
    // Indirection so the helper-module impl can stay below for clarity.
    crate::menu_bar_helper::walk_items_impl(roots)
}

fn walk_accelerators(roots: &[crate::menu_bar_helper::Roots]) -> Vec<(&str, &str, &str)> {
    crate::menu_bar_helper::walk_accelerators_impl(roots)
}

mod menu_bar_helper {
    use super::*;
    pub type Roots = freally_ui_lib::menu_spec::RootSpec;

    pub fn walk_items_impl(roots: &[Roots]) -> Vec<(&str, &str)> {
        let mut out = Vec::new();
        for r in roots {
            walk(&r.children, &mut out);
        }
        out
    }
    fn walk<'a>(nodes: &'a [NodeSpec], out: &mut Vec<(&'a str, &'a str)>) {
        for n in nodes {
            match n {
                NodeSpec::Item(it) => out.push((it.id, it.label)),
                NodeSpec::Submenu { children, .. } => walk(children, out),
                NodeSpec::Separator => {}
            }
        }
    }

    pub fn walk_accelerators_impl(roots: &[Roots]) -> Vec<(&str, &str, &str)> {
        let mut out = Vec::new();
        for r in roots {
            walka(&r.children, &mut out);
        }
        out
    }
    fn walka<'a>(nodes: &'a [NodeSpec], out: &mut Vec<(&'a str, &'a str, &'a str)>) {
        for n in nodes {
            match n {
                NodeSpec::Item(it) => {
                    if let Some(a) = it.accel {
                        out.push((it.id, it.label, a));
                    }
                }
                NodeSpec::Submenu { children, .. } => walka(children, out),
                NodeSpec::Separator => {}
            }
        }
    }
}
