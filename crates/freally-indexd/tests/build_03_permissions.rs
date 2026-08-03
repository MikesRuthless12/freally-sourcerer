//! Re-export of the Build 3 permission-health smoke so it runs under
//! `cargo test --workspace`. Canonical source:
//! `tests/smoke/build_03_permissions.rs`.

#[path = "../../../tests/smoke/build_03_permissions.rs"]
mod build_03_permissions;
