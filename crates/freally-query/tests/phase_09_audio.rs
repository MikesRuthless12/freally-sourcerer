//! Re-export of the OS-agnostic Phase 9 smoke tests so they run under
//! `cargo test --workspace`.
//!
//! `tests/smoke/phase_09_audio.rs` is the canonical source. The
//! Phase 9 surface spans `freally-audio` (analyzer + cache) and
//! `freally-query` (DSL + executor); we host the smoke under
//! `freally-query` because that crate already depends on
//! `freally-audio` + `freally-index` and can exercise the full
//! end-to-end path in a single test binary.

#[path = "../../../tests/smoke/phase_09_audio.rs"]
mod phase_09_audio;
