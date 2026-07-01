//! Re-export of the OS-agnostic Phase 12 smoke tests so they run under
//! `cargo test --workspace`. Canonical source:
//! `tests/smoke/phase_12_indexd_client.rs`.

#[path = "../../../tests/smoke/phase_12_indexd_client.rs"]
mod phase_12_indexd_client;
