//! Re-export of the OS-agnostic Phase 10 smoke tests so they run
//! under `cargo test --workspace`.
//!
//! `tests/smoke/phase_10_query.rs` is the canonical source. The
//! Phase 10 surface lives entirely under `freally-query` (parser
//! hardening, optimizer, lens prefixes, ParseReport IPC); we mirror
//! the smoke under the per-crate `tests/` directory so the crate's
//! own test runner picks it up alongside the cross-OS smoke harness.

#[path = "../../../tests/smoke/phase_10_query.rs"]
mod phase_10_query;
