//! Excludes panel — Indexes → Exclude (PRD §8.15).

use crate::daemon;

#[tauri::command]
pub fn excludes_get() -> Result<sourcerer_rpc::ExcludeRules, String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call("excludes.get", serde_json::Value::Null)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn excludes_set(rules: sourcerer_rpc::ExcludeRules) -> Result<(), String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call_void(
            "excludes.set",
            serde_json::to_value(rules).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())
}
