//! Re-export of the OS-agnostic Phase 4 smoke tests so they run under
//! `cargo test --workspace`.
//!
//! `tests/smoke/phase_04_index.rs` is the canonical source.

#[path = "../../../tests/smoke/phase_04_index.rs"]
mod phase_04_index;
