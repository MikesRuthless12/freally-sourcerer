//! `freally://` URL protocol handler.
//!
//! Supported shapes:
//!   * `freally://search?q=<query>&filter=<id>` — drops a query into
//!     the search bar and runs it.
//!   * `freally://bundle/<encoded-path>` — bundle reference; Phase 11
//!     just routes the parse to the UI, the actual fetch lands in
//!     Phase 12.
//!
//! The UI subscribes to `url:opened` and dispatches accordingly.

use tauri::{AppHandle, Emitter, Manager, Runtime};
use tauri_plugin_deep_link::DeepLinkExt;

pub fn register<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    // Register the scheme at runtime (Tauri 2 handles per-OS plumbing).
    //
    // SRC-M17 — not in portable mode. This writes a machine-level
    // handler (registry on Windows, LaunchServices on macOS, a .desktop
    // entry on Linux) pointing at the executable's current path. Unplug
    // the stick and every `freally://` link on that machine is a dangling
    // handler. The in-process listener below still runs, so a portable
    // instance handles URLs it is handed — it just does not claim the
    // scheme.
    if !freally_rpc::portable::is_active() {
        let _ = app.deep_link().register("freally");
    }

    let app_clone = app.clone();
    app.deep_link().on_open_url(move |event| {
        for url in event.urls() {
            // M3 fix: only act on recognized URL shapes. Drops
            // window.show()/focus() spam from arbitrary `freally://...`
            // URLs that any webpage can fire — focus-stealing UX-DoS
            // becomes a no-op.
            if url.scheme() != "freally" {
                continue;
            }
            let host = url.host_str();
            let recognized = matches!(host, Some("search") | Some("bundle"));
            if !recognized {
                continue;
            }
            // Length cap on the full URL: anything past 8 KiB is rejected
            // before window manipulation.
            let s = url.to_string();
            if s.len() > 8192 {
                continue;
            }
            if let Some(window) = app_clone.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
                let _ = window.emit("url:opened", s);
            }
        }
    });
    Ok(())
}
