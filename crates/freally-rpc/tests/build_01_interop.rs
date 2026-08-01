//! Re-export of the OS-agnostic Build 1 interop smoke so it runs under
//! `cargo test --workspace`.
//!
//! `tests/smoke/build_01_interop.rs` is the canonical source.

#[path = "../../../tests/smoke/build_01_interop.rs"]
mod build_01_interop;
