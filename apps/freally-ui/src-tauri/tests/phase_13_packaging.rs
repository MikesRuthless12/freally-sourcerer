//! Re-export of the TASK-105 packaging smoke. It lives in this crate
//! because the version it reads is this crate's, via `tauri.conf.json`.
//! Canonical source: `tests/smoke/phase_13_packaging.rs`.

#[path = "../../../../tests/smoke/phase_13_packaging.rs"]
mod phase_13_packaging;
