//! Re-export of the OS-agnostic Phase 0 smoke tests so they run under
//! `cargo test --workspace`.
//!
//! `tests/smoke/phase_00_scaffold.rs` is the canonical source.

#[path = "../../../tests/smoke/phase_00_scaffold.rs"]
mod phase_00_scaffold;
