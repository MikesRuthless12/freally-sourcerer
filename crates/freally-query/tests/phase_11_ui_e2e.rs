//! Re-export of the OS-agnostic Phase 11 smoke tests so they run
//! under `cargo test --workspace`.
//!
//! `tests/smoke/phase_11_ui_e2e.rs` is the canonical source. The
//! Rust-side Phase 11 surface that the workspace can validate is the
//! `freally-query::parse_to_report` routing the UI's search bar
//! depends on; we mirror the smoke under the per-crate `tests/`
//! directory so the crate's own test runner picks it up alongside
//! the cross-OS smoke harness.

#[path = "../../../tests/smoke/phase_11_ui_e2e.rs"]
mod phase_11_ui_e2e;
