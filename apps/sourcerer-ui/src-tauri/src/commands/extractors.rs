//! Extractors panel — built-in Phase 7-9 extractors. Custom extractors
//! live in `commands/custom_extractors.rs`.

use serde::Deserialize;

use crate::daemon;

#[tauri::command]
pub fn extractors_list() -> Result<Vec<sourcerer_rpc::ExtractorInfo>, String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call("extractors.list", serde_json::Value::Null)
        .map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct SetModeArgs {
    pub id: String,
    pub mode: sourcerer_rpc::ExtractorMode,
}

#[tauri::command]
pub fn extractors_set_mode(args: SetModeArgs) -> Result<(), String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call_void(
            "extractors.set_mode",
            serde_json::json!({ "id": args.id, "mode": args.mode }),
        )
        .map_err(|e| e.to_string())
}
