//! Re-export of the TASK-103 updater-config smoke. It lives in this
//! crate because the configuration it checks is this crate's
//! `tauri.conf.json`. Canonical source:
//! `tests/smoke/phase_13_updater.rs`.

#[path = "../../../../tests/smoke/phase_13_updater.rs"]
mod phase_13_updater;
