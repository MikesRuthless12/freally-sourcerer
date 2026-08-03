//! Re-export of the Build 3 portable-mode smoke so it runs under
//! `cargo test --workspace`. It lives in this crate because it drives
//! the real `freally-indexd` binary via `CARGO_BIN_EXE_freally-indexd`,
//! which only resolves inside the crate that declares the binary.
//! Canonical source: `tests/smoke/build_03_portable.rs`.

#[path = "../../../tests/smoke/build_03_portable.rs"]
mod build_03_portable;
