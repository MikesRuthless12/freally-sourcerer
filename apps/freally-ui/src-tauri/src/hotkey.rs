//! Global hotkey registration. Default chord per OS — overridable from
//! Settings. On fire, brings the main window forward and emits a
//! `hotkey:fired` event so the UI can focus the search bar.

use tauri::{AppHandle, Emitter, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

pub fn default_chord() -> &'static str {
    if cfg!(target_os = "macos") {
        "Alt+Space"
    } else {
        // Windows + Linux both use Super+Space (Windows: Win key; Linux: Super).
        "Super+Space"
    }
}

pub fn register_default<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let chord = default_chord();
    let app_clone = app.clone();
    app.global_shortcut()
        .on_shortcut(chord, move |_app, _shortcut, event| {
            if event.state() != ShortcutState::Pressed {
                return;
            }
            if let Some(window) = app_clone.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
                let _ = window.emit("hotkey:fired", chord);
            }
        })
        .map_err(|e| e.to_string())?;
    Ok(())
}
