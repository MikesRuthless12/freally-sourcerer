//! Re-export of the OS-agnostic Build 2 rename/undo smoke so it runs
//! under `cargo test --workspace`. Canonical source:
//! `tests/smoke/build_02_rename_undo.rs`.

#[path = "../../../tests/smoke/build_02_rename_undo.rs"]
mod build_02_rename_undo;
