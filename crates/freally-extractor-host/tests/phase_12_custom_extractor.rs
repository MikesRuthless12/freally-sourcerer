//! Re-export of the OS-agnostic Phase 12 smoke for the custom-extractor
//! framework so it runs under `cargo test --workspace`. Canonical:
//! `tests/smoke/phase_12_custom_extractor.rs`.

#[path = "../../../tests/smoke/phase_12_custom_extractor.rs"]
mod phase_12_custom_extractor;
