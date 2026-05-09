//! Folders panel — Indexes → Folders (PRD §8.13).

use crate::daemon;

#[tauri::command]
pub fn folders_list() -> Result<Vec<sourcerer_rpc::WatchedFolder>, String> {
    daemon_call("folders.list", serde_json::Value::Null)
}

#[tauri::command]
pub fn folders_add(folder: sourcerer_rpc::WatchedFolder) -> Result<(), String> {
    daemon_call_void(
        "folders.add",
        serde_json::to_value(folder).map_err(|e| e.to_string())?,
    )
}

#[tauri::command]
pub fn folders_remove(id: String) -> Result<(), String> {
    daemon_call_void("folders.remove", serde_json::json!({ "id": id }))
}

#[tauri::command]
pub fn folders_update(folder: sourcerer_rpc::WatchedFolder) -> Result<(), String> {
    daemon_call_void(
        "folders.update",
        serde_json::to_value(folder).map_err(|e| e.to_string())?,
    )
}

#[tauri::command]
pub fn folders_rescan(id: String) -> Result<(), String> {
    daemon_call_void("folders.rescan", serde_json::json!({ "id": id }))
}

#[tauri::command]
pub fn folders_rescan_all() -> Result<(), String> {
    daemon_call_void("folders.rescan_all", serde_json::Value::Null)
}

fn daemon_call<T>(method: &'static str, params: serde_json::Value) -> Result<T, String>
where
    T: for<'de> serde::Deserialize<'de> + Send + 'static,
{
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon.call(method, params).map_err(|e| e.to_string())
}

fn daemon_call_void(method: &'static str, params: serde_json::Value) -> Result<(), String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon.call_void(method, params).map_err(|e| e.to_string())
}
