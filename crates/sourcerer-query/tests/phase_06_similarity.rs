//! Re-export of the OS-agnostic Phase 6 smoke tests so they run under
//! `cargo test --workspace`.
//!
//! `tests/smoke/phase_06_similarity.rs` is the canonical source.

#[path = "../../../tests/smoke/phase_06_similarity.rs"]
mod phase_06_similarity;
