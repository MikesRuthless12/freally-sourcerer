//! Re-export of the OS-agnostic Build 1 DSL smoke so it runs under
//! `cargo test --workspace`.
//!
//! `tests/smoke/build_01_dsl.rs` is the canonical source.

#[path = "../../../tests/smoke/build_01_dsl.rs"]
mod build_01_dsl;
