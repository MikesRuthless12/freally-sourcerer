//! Re-export of the OS-agnostic TASK-100 limit-pushdown smoke so it runs
//! under `cargo test --workspace`. Canonical source:
//! `tests/smoke/phase_13_limit_pushdown.rs`.

#[path = "../../../tests/smoke/phase_13_limit_pushdown.rs"]
mod phase_13_limit_pushdown;
