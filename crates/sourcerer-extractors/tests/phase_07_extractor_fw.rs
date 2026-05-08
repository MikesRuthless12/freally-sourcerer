//! Re-export of the OS-agnostic Phase 7 smoke tests so they run under
//! `cargo test --workspace`.
//!
//! `tests/smoke/phase_07_extractor_fw.rs` is the canonical source.

#[path = "../../../tests/smoke/phase_07_extractor_fw.rs"]
mod phase_07_extractor_fw;
