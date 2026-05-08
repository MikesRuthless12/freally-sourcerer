//! Re-export of the OS-agnostic Phase 5 smoke tests so they run under
//! `cargo test --workspace`.
//!
//! `tests/smoke/phase_05_filename_lens.rs` is the canonical source.

#[path = "../../../tests/smoke/phase_05_filename_lens.rs"]
mod phase_05_filename_lens;
