//! Re-export of the OS-agnostic Build 3 natural-sort smoke so it runs
//! under `cargo test --workspace`. Canonical source:
//! `tests/smoke/build_03_natural_sort.rs`.

#[path = "../../../tests/smoke/build_03_natural_sort.rs"]
mod build_03_natural_sort;
