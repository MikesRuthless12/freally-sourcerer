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

use commands::bookmarks::BookmarksStore;
use commands::known_paths::KnownPaths;
use commands::settings::SettingsStore;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .try_init();

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
            // Boot the daemon + RPC client. Failures here are fatal —
            // there is no fallback layer in Phase 12; the UI is built
            // around a live indexd. Surface the error before any window
            // is shown so the user sees a clear failure dialog.
            let daemon_arc = Arc::new(daemon::Daemon::boot(handle).map_err(|e| {
                tracing::error!(error = %e, "daemon boot failed");
                e
            })?);
            daemon::install(daemon_arc.clone());

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running Sourcerer");
}
