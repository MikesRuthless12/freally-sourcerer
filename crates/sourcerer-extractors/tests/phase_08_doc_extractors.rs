//! Re-export of the OS-agnostic Phase 8 smoke tests so they run under
//! `cargo test --workspace`.
//!
//! `tests/smoke/phase_08_doc_extractors.rs` is the canonical source.

#[path = "../../../tests/smoke/phase_08_doc_extractors.rs"]
mod phase_08_doc_extractors;
