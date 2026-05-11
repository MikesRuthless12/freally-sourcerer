//! Sourcerer Tauri 2 backend (Phase 12 — real `sourcerer-indexd` IPC).
//!
//! Phase 11 shipped the desktop UI on top of a *mock* backend. Phase 12
//! replaced the mock with a live `sourcerer-rpc` client connected to an
//! in-process `sourcerer-indexd`. The TS contract in
//! `src/lib/ipc/types.ts` is stable across the swap; `commands/` types
//! mirror that contract byte-for-byte.

pub mod commands;
pub mod daemon;
pub mod hotkey;
pub mod menu_spec;
pub mod native_menu;
pub mod preview;
pub mod url_protocol;

use std::sync::Arc;

use tauri::Manager;

/// Real macOS-style quit. The UI's `file.exit` handler invokes this so
/// Cmd+Q quits the app process (window.close() alone doesn't on macOS).
#[tauri::command]
fn app_exit(app: tauri::AppHandle) {
    app.exit(0);
}

/// TS-side debug bridge: forwards a message to the Rust tracing stream
/// so console.log-style events show up in the cargo dev log (which is
/// what external monitors tail).
#[tauri::command]
fn log_event(tag: String, message: String) {
    tracing::info!(target: "sourcerer::trace", tag = %tag, message = %message);
}

use commands::bookmarks::BookmarksStore;
use commands::known_paths::KnownPaths;
use commands::settings::SettingsStore;

/// Single-instance enforcement: kill any other live `sourcerer-ui` /
/// `sourcerer-indexd` processes before this one boots, so the new launch
/// can take ownership of the tantivy writer lock + the RPC pipe.
#[cfg(windows)]
fn kill_other_sourcerer_instances() {
    use std::process::{Command, Stdio};
    let me = std::process::id();
    let filter = format!("PID ne {me}");
    for name in ["sourcerer-ui.exe", "sourcerer-indexd.exe"] {
        let _ = Command::new("taskkill")
            .args(["/F", "/T", "/FI", filter.as_str(), "/IM", name])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    // Give Windows a beat to release the tantivy writer lock + named pipe
    // before the new daemon tries to claim them.
    std::thread::sleep(std::time::Duration::from_millis(300));
}

#[cfg(not(windows))]
fn kill_other_sourcerer_instances() {
    // No-op for non-Windows; equivalent pkill-based logic can land later.
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                // Default raises the floor to `info` for our own crates
                // so the targeted instrumentation (sourcerer::preview,
                // sourcerer::icons, sourcerer_ui_lib) is visible without
                // forcing RUST_LOG. Third-party noisy crates stay at
                // `warn` so the console doesn't get swamped.
                .unwrap_or_else(|_| {
                    tracing_subscriber::EnvFilter::new(
                        "warn,sourcerer=info,sourcerer_ui_lib=info,sourcerer_indexd=info",
                    )
                }),
        )
        .try_init();

    // Surface Rust panics in the console with the panic message + a
    // best-effort backtrace location, so a panicking command thread
    // doesn't disappear silently.
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        tracing::error!(target: "sourcerer::panic", panic = %info,
            "Rust panic — check the location above for the panicking thread");
        prev_hook(info);
    }));

    kill_other_sourcerer_instances();

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            let handle = app.handle();
            // Boot the daemon + RPC client on a background thread so the
            // Tauri setup hook returns immediately and the window can
            // appear right away. Blocking the setup hook on a 10-15s
            // canonical-store replay would trip Windows' GUI watchdog and
            // kill the process before any HWND is created.
            // Commands that route through the daemon check
            // `daemon::get()` and surface a "daemon not ready" error
            // until the background boot finishes; the TS side retries.
            let boot_handle = handle.clone();
            std::thread::Builder::new()
                .name("sourcerer-daemon-boot".into())
                .spawn(move || match daemon::Daemon::boot(&boot_handle) {
                    Ok(d) => {
                        daemon::install(Arc::new(d));
                        tracing::info!("sourcerer daemon ready");
                    }
                    Err(e) => tracing::error!(error = %e, "daemon boot failed"),
                })
                .expect("failed to spawn daemon-boot thread");

            app.manage(BookmarksStore::new(handle));
            app.manage(SettingsStore::new(handle));
            app.manage(KnownPaths::new());

            #[cfg(target_os = "macos")]
            {
                let menu = native_menu::build_app_menu(handle)?;
                app.set_menu(menu)?;
            }
            native_menu::register_menu_event_relay(handle);

            if let Err(e) = hotkey::register_default(handle) {
                tracing::warn!("global hotkey registration failed: {e}");
            }
            if let Err(e) = url_protocol::register(handle) {
                tracing::warn!("sourcerer:// registration failed: {e}");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            app_exit,
            // Real-but-local: parse runs in-process at keystroke rate.
            commands::query::query_parse,
            // Daemon-routed query lifecycle.
            commands::query::query_run,
            commands::query::query_cancel,
            commands::query::query_lens_timings,
            // Daemon-routed index controls.
            commands::index_state::index_state,
            commands::index_state::index_verify,
            commands::index_state::index_compact,
            commands::index_state::index_rebuild,
            // Bookmarks (UI-side persistence).
            commands::bookmarks::bookmarks_list,
            commands::bookmarks::bookmarks_save,
            commands::bookmarks::bookmarks_delete,
            commands::bookmarks::bookmarks_rename,
            // Daemon-routed extractors panel.
            commands::extractors::extractors_list,
            commands::extractors::extractors_set_mode,
            // Settings (UI-side persistence + apply hooks for index-affecting fields).
            commands::settings::settings_get,
            commands::settings::settings_set,
            commands::settings::settings_reset,
            commands::settings::settings_apply_to_daemon,
            // File operations (mostly UI-side, preview/thumbnail dispatch to host).
            commands::files::files_open,
            commands::files::files_reveal,
            commands::files::files_copy_path,
            commands::files::files_copy_name,
            commands::files::files_delete,
            commands::files::files_thumbnail,
            commands::files::files_preview,
            commands::files::files_copy_text,
            commands::files::files_whitelist_user_chosen,
            // Phase 12 daemon-routed Settings → Indexes panels.
            commands::volumes::volumes_list,
            commands::volumes::volumes_update,
            commands::volumes::volumes_recreate_journal,
            commands::volumes::volumes_reset_stream,
            commands::volumes::volumes_upgrade_fanotify,
            commands::volumes::volumes_remove,
            commands::folders::folders_list,
            commands::folders::folders_add,
            commands::folders::folders_remove,
            commands::folders::folders_update,
            commands::folders::folders_rescan,
            commands::folders::folders_rescan_all,
            commands::excludes::excludes_get,
            commands::excludes::excludes_set,
            // Phase 12 daemon-routed Network panels.
            commands::network::network_status,
            commands::network::network_start_https,
            commands::network::network_stop_https,
            commands::network::network_regen_token,
            commands::network::network_start_api,
            commands::network::network_stop_api,
            // Phase 12 daemon-routed Custom Extractors panel.
            commands::custom_extractors::custom_extractors_list,
            commands::custom_extractors::custom_extractors_set_trusted,
            commands::custom_extractors::custom_extractors_refresh_hashes,
            // Phase 12 daemon-routed History panel.
            commands::history::history_get,
            commands::history::history_set,
            commands::history::history_clear,
            // Real shell-extracted icons for result rows.
            commands::icons::icon_for_ext,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Sourcerer");
}
