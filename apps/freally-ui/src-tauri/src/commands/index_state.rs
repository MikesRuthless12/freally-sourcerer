//! Index-state commands routed through the daemon.

use crate::daemon;

#[tauri::command]
pub fn index_state() -> Result<freally_rpc::IndexState, String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call("index.state", serde_json::Value::Null)
        .map_err(|e| e.to_string())
}

/// SRC-M13 — per-volume live-journaling health plus the advisor's output.
#[tauri::command]
pub fn index_health() -> Result<freally_rpc::IndexHealth, String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call("index.health", serde_json::Value::Null)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn index_verify() -> Result<(), String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call_void("index.verify", serde_json::Value::Null)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn index_compact() -> Result<(), String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call_void("index.compact", serde_json::Value::Null)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn index_rebuild() -> Result<(), String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call_void("index.rebuild", serde_json::Value::Null)
        .map_err(|e| e.to_string())
}
