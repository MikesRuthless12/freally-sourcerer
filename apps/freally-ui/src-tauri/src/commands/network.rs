//! Network panels — HTTP/HTTPS Server + ETP/FTP API (PRD §8.21–§8.22).

use serde::Deserialize;

use crate::daemon;

#[tauri::command]
pub fn network_status() -> Result<serde_json::Value, String> {
    daemon_call("network.status", serde_json::Value::Null)
}

#[derive(Debug, Deserialize)]
pub struct StartHttpsArgs {
    pub bind: String,
    pub port: u16,
    #[serde(default)]
    pub force_https: bool,
    #[serde(default)]
    pub legacy_auth: bool,
}

#[tauri::command]
pub fn network_start_https(args: StartHttpsArgs) -> Result<serde_json::Value, String> {
    daemon_call(
        "network.start_https",
        serde_json::json!({
            "bind": args.bind,
            "port": args.port,
            "force_https": args.force_https,
            "legacy_auth": args.legacy_auth,
        }),
    )
}

#[tauri::command]
pub fn network_stop_https() -> Result<(), String> {
    daemon_call_void("network.stop_https", serde_json::Value::Null)
}

#[tauri::command]
pub fn network_regen_token() -> Result<serde_json::Value, String> {
    daemon_call("network.regen_token", serde_json::Value::Null)
}

#[derive(Debug, Deserialize)]
pub struct StartApiArgs {
    pub port: u16,
    #[serde(default)]
    pub legacy_ftp: bool,
}

#[tauri::command]
pub fn network_start_api(args: StartApiArgs) -> Result<(), String> {
    daemon_call_void(
        "network.start_api",
        serde_json::json!({
            "port": args.port,
            "legacy_ftp": args.legacy_ftp,
        }),
    )
}

#[tauri::command]
pub fn network_stop_api() -> Result<(), String> {
    daemon_call_void("network.stop_api", serde_json::Value::Null)
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
