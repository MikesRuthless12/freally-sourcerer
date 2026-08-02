//! Re-export of the OS-agnostic Build 2 index-health smoke so it runs
//! under `cargo test --workspace`. Canonical source:
//! `tests/smoke/build_02_index_health.rs`.

#[path = "../../../tests/smoke/build_02_index_health.rs"]
mod build_02_index_health;
