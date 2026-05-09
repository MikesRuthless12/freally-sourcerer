//! Lenses → Custom panel — Wasm-sandboxed community extractors (PRD §8.20).

use serde::Deserialize;

use crate::daemon;

#[tauri::command]
pub fn custom_extractors_list() -> Result<Vec<sourcerer_rpc::CustomExtractorEntry>, String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call("custom_extractors.list", serde_json::Value::Null)
        .map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct SetTrustedArgs {
    pub id: String,
    pub trusted: bool,
}

#[tauri::command]
pub fn custom_extractors_set_trusted(args: SetTrustedArgs) -> Result<(), String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call_void(
            "custom_extractors.set_trusted",
            serde_json::json!({ "id": args.id, "trusted": args.trusted }),
        )
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn custom_extractors_refresh_hashes() -> Result<(), String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call_void("custom_extractors.refresh_hashes", serde_json::Value::Null)
        .map_err(|e| e.to_string())
}
