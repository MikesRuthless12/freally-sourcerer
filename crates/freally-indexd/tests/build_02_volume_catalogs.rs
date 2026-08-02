//! Re-export of the OS-agnostic Build 2 volume-catalog smoke so it runs
//! under `cargo test --workspace`. Canonical source:
//! `tests/smoke/build_02_volume_catalogs.rs`.

#[path = "../../../tests/smoke/build_02_volume_catalogs.rs"]
mod build_02_volume_catalogs;
