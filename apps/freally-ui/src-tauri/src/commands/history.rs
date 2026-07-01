//! History panel (PRD §8.10).

use crate::daemon;

#[tauri::command]
pub fn history_get() -> Result<serde_json::Value, String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call("history.get", serde_json::Value::Null)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn history_set(update: serde_json::Value) -> Result<(), String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call_void("history.set", update)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn history_clear() -> Result<(), String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call_void("history.clear", serde_json::Value::Null)
        .map_err(|e| e.to_string())
}
