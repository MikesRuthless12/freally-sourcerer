//! TASK-UP1 — the check-for-updates surface.
//!
//! The updater plugin has been registered, signed and verified since
//! TASK-103, and until now **nothing ever asked it a question**:
//! `Help → Check for Updates…` opened the GitHub releases page in a
//! browser. So the whole chain — endpoint, manifest, minisign signature,
//! embedded public key — was correct and unreachable.
//!
//! Two commands, deliberately separate:
//!
//! - [`updates_check`] asks and reports. It never downloads.
//! - [`updates_install`] downloads and installs, and is only called after
//!   the user has answered a Yes/No box naming the version.
//!
//! Splitting them is the point. A single "check and update" command would
//! mean the act of looking commits you to installing, and an app that
//! replaces itself because you asked it a question is not one people
//! trust with their filesystem.
//!
//! # Why a failed check must be loud here
//!
//! The updater fails **closed and silent**: a manifest it cannot verify,
//! an endpoint it cannot reach, and "you are up to date" are all the same
//! `None` from `check()`. That is the failure mode that hid the destroyed
//! signing key from every `v0.23.1` install — the app kept reporting no
//! update, forever, and nothing said why. So a transport or signature
//! failure is returned as an `Err` and shown, rather than folded into
//! "no update available".

use serde::Serialize;
use tauri::AppHandle;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tauri_plugin_updater::UpdaterExt;

/// What the Updates panel renders. Nothing here downloads anything.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheckDto {
    /// The version running right now.
    pub current_version: String,
    /// The version the manifest advertises, when it is newer than the
    /// running one. Empty when already current.
    pub available_version: String,
    /// The release body from the manifest. Markdown, rendered by the
    /// panel as text plus clickable links — never as HTML.
    pub notes: String,
    /// Whether an update is actually available to install.
    pub is_newer: bool,
    /// Unix seconds at which this check completed, for "last checked".
    pub checked_at_unix_secs: i64,
}

fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// The updater is deliberately unavailable in portable mode, so say that
/// rather than letting `app.updater()` fail with something opaque.
///
/// A portable install updates by replacing its folder. Pointing an
/// *installer* at a USB stick would install the app onto the host
/// machine, which is the one thing a portable user is avoiding.
fn portable_guard() -> Result<(), String> {
    if freally_rpc::portable::is_active() {
        return Err(
            "This is a portable install. Update it by replacing the folder with a newer \
             download — the installer would write to the host machine instead."
                .into(),
        );
    }
    Ok(())
}

/// Ask the update endpoint what it has. Does not download.
#[tauri::command]
pub async fn updates_check(app: AppHandle) -> Result<UpdateCheckDto, String> {
    portable_guard()?;
    let current_version = app.package_info().version.to_string();
    let updater = app
        .updater()
        .map_err(|e| format!("the updater is unavailable: {e}"))?;

    // `None` is genuinely "you are current". An `Err` is a transport or
    // signature failure and must **not** be folded into that — see the
    // module note. `?` keeps them apart.
    let update = updater
        .check()
        .await
        .map_err(|e| format!("could not check for updates: {e}"))?;
    Ok(UpdateCheckDto {
        current_version,
        available_version: update
            .as_ref()
            .map(|u| u.version.clone())
            .unwrap_or_default(),
        notes: update
            .as_ref()
            .and_then(|u| u.body.clone())
            .unwrap_or_default(),
        is_newer: update.is_some(),
        checked_at_unix_secs: now_secs(),
    })
}

/// Download the available update, stop the daemon, and install it.
///
/// **This command asks for itself.** The confirmation used to live only
/// in the webview, which is the layer this project treats as the
/// attacker: `invoke("updates_install")` skipped it entirely, and what
/// it skipped was an unannounced download, an installer launch, and —
/// since the app now prefers the machine-wide daemon on Windows — a
/// `daemon.shutdown` that stops indexing for every user on the box.
///
/// The prompt strings come from the caller so they can be localised. A
/// compromised webview can therefore reword the box, but it cannot skip
/// it, which is the property that matters: nothing replaces the running
/// application without a native dialog the user answered.
///
/// The check is repeated rather than carrying an `Update` across two
/// commands. The manifest is a few kilobytes and this way there is no
/// stale handle to install from — if the release was pulled between the
/// question and the answer, this correctly finds nothing.
#[tauri::command]
pub async fn updates_install(
    app: AppHandle,
    title: String,
    message: String,
    yes_label: String,
    no_label: String,
) -> Result<bool, String> {
    portable_guard()?;
    let updater = app
        .updater()
        .map_err(|e| format!("the updater is unavailable: {e}"))?;
    let update = updater
        .check()
        .await
        .map_err(|e| format!("could not check for updates: {e}"))?
        .ok_or_else(|| "there is no update to install".to_string())?;

    // The gate. `blocking_show` is moved off the async runtime's worker:
    // it pumps a native modal loop, and running it inline would block the
    // thread the window's events arrive on.
    let dialog = app.dialog().clone();
    let approved = tauri::async_runtime::spawn_blocking(move || {
        dialog
            .message(message)
            .title(title)
            .kind(MessageDialogKind::Info)
            .buttons(MessageDialogButtons::OkCancelCustom(yes_label, no_label))
            .blocking_show()
    })
    .await
    .map_err(|e| format!("could not show the confirmation: {e}"))?;
    if !approved {
        return Ok(false);
    }

    // Download **before** stopping the daemon.
    //
    // Splitting `download_and_install` is the whole point: the download
    // is the half that fails — dropped wifi, a full disk, a signature
    // that does not verify — and the daemon has no stake in it. Stopping
    // it first meant a failed download left the indexer dead for the rest
    // of the session, because `daemon::get()` reads a `OnceCell` set once
    // at boot and there is no reconnect. Every later search failed with
    // nothing on screen connecting it to the update that did not happen.
    let bytes = update
        .download(|_chunk, _total| {}, || {})
        .await
        .map_err(|e| format!("could not download the update: {e}"))?;

    // Now stop it. `freally-indexd` is a separate executable the
    // installer also replaces, and on Windows a running image cannot be
    // overwritten — the install would fail partway with the UI replaced
    // and the daemon stale. `install` ends the process without unwinding,
    // so `Daemon`'s `Drop` never runs and cannot be relied on for this.
    //
    // Asking rather than killing: a graceful stop commits whatever the
    // watchers have pending.
    shutdown_daemon().await;

    update
        .install(bytes)
        .map_err(|e| format!("could not install the update: {e}"))?;

    // On Windows the NSIS installer takes over and ends this process
    // itself, so this is not reached — and it does **not** relaunch the
    // app for us. `installMode` is `basicUi`, which omits NSIS's `/R`
    // (restart) switch that `passive` passes; in exchange the installer
    // shows its finish page with a "run it now" checkbox, which is the
    // behaviour this app wants. The user chooses.
    //
    // On macOS and Linux the bundle is swapped in place and the running
    // binary is still the old one, so the restart below is what actually
    // puts the new version on screen.
    //
    // `restart` diverges, so `Ok(true)` is never observed: the only
    // answers a caller can see are `Ok(false)` for "the user said no" and
    // an `Err` for a failure. That is the point of returning a bool
    // rather than `()` — declining and installing must not look alike.
    app.restart();
}

/// Ask the daemon to exit, then make sure it did.
///
/// Best-effort throughout: every branch here ends with the installer
/// running regardless. A daemon that will not die is a reason to let the
/// installer report a locked file, not a reason to abandon the update
/// before it starts.
async fn shutdown_daemon() {
    let Some(daemon) = crate::daemon::get() else {
        // Nothing was ever connected, so there is nothing of ours to
        // stop — and reaching for `taskkill` here would take down a
        // *different* copy's daemon for no reason.
        return;
    };
    let mut answered = false;
    {
        let asked = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            daemon.client.call::<serde_json::Value, serde_json::Value>(
                "daemon.shutdown",
                serde_json::json!({}),
            ),
        )
        .await;
        match asked {
            Ok(_) => answered = true,
            Err(_elapsed) => {
                tracing::warn!("update: daemon did not answer `daemon.shutdown` in 5s")
            }
        }
    }
    // Whether or not it answered, give it a moment to actually leave the
    // process table and release the Tantivy writer lock and the pipe.
    tokio::time::sleep(std::time::Duration::from_millis(750)).await;
    // Only when it would not go quietly. `taskkill /IM` matches **every**
    // process with that image name the caller may terminate — a portable
    // copy running off a stick, a second install, or another user's daemon
    // if this app happens to be elevated. Reaching for it when the graceful
    // stop already worked would take those down for nothing.
    #[cfg(windows)]
    if !answered {
        kill_stale_daemon();
    }
}

/// Last resort on Windows: an image that is still running cannot be
/// replaced, and the installer's failure at that point is a half-applied
/// update. Only reached after the graceful ask above has had its five
/// seconds.
#[cfg(windows)]
fn kill_stale_daemon() {
    use std::process::{Command, Stdio};
    let _ = Command::new(crate::bugreport::system32("taskkill.exe"))
        .args(["/F", "/T", "/IM", "freally-indexd.exe"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}
