//! What build am I running, and where does it keep its files?
//!
//! Answers both from the running process rather than from constants the
//! UI carries. The version comes from `tauri.conf.json` via
//! `package_info()`, which is the same value the installer and the
//! updater manifest use — the About panel used to hardcode it and had
//! drifted three releases behind.
//!
//! SRC-M17 adds the second half: whether this is a portable install, and
//! the folder it writes to. A user who plugs a stick into someone else's
//! machine needs to be able to confirm, in the app, that nothing is
//! landing on the host.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppEnvironment {
    /// Semver from the bundle manifest, e.g. `0.23.0`.
    pub version: String,
    /// True when this process runs in portable mode (SRC-M17).
    pub portable: bool,
    /// The `Data/` folder a portable install writes to. `None` when not
    /// portable, and also when the executable's own path could not be
    /// read — in which case `portable` is false too.
    pub data_dir: Option<String>,
}

#[tauri::command]
pub fn app_environment(app: tauri::AppHandle) -> AppEnvironment {
    let data_dir = freally_rpc::portable::data_dir();
    AppEnvironment {
        version: app.package_info().version.to_string(),
        portable: data_dir.is_some(),
        data_dir: data_dir.map(|d| d.display().to_string()),
    }
}
