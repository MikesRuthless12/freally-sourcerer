//! Re-export of the TASK-102 daemon-service smoke so it runs under
//! `cargo test --workspace`. It lives in this crate because it drives
//! `freally_indexd::installed`. Canonical source:
//! `tests/smoke/phase_13_daemon_service.rs`.

#[path = "../../../tests/smoke/phase_13_daemon_service.rs"]
mod phase_13_daemon_service;
