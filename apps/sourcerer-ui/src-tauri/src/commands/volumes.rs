//! Volumes panel commands — Indexes → Volumes (PRD §8.12).

use crate::daemon;

#[tauri::command]
pub fn volumes_list() -> Result<Vec<sourcerer_rpc::VolumeInfo>, String> {
    daemon_call("volumes.list", serde_json::Value::Null)
}

#[tauri::command]
pub fn volumes_update(update: sourcerer_rpc::VolumeUpdate) -> Result<(), String> {
    daemon_call_void(
        "volumes.update",
        serde_json::to_value(update).map_err(|e| e.to_string())?,
    )
}

#[tauri::command]
pub fn volumes_recreate_journal(id: String) -> Result<(), String> {
    daemon_call_void("volumes.recreate_journal", serde_json::json!({ "id": id }))
}

#[tauri::command]
pub fn volumes_reset_stream(id: String) -> Result<(), String> {
    daemon_call_void("volumes.reset_stream", serde_json::json!({ "id": id }))
}

#[tauri::command]
pub fn volumes_upgrade_fanotify() -> Result<(), String> {
    daemon_call_void("volumes.upgrade_fanotify", serde_json::Value::Null)
}

#[tauri::command]
pub fn volumes_remove(id: String) -> Result<(), String> {
    daemon_call_void("volumes.remove", serde_json::json!({ "id": id }))
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
