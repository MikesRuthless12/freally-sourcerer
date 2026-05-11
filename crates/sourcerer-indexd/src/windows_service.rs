//! Windows Service Control Manager wiring for `sourcerer-indexd`.
//!
//! - `install()`  — registers `Sourcerer-Indexd` with the SCM.
//! - `uninstall()` — deletes the registration.
//! - `run_as_service()` — invoked by SCM via the registered ImagePath
//!   (`<exe> service`); spins up the dispatcher.
//!
//! The service body is a placeholder loop in Phase 1: it transitions to
//! `RUNNING`, sleeps until SCM signals stop, and transitions to `STOPPED`.
//! Phase 4 will wire actual per-volume subscribers + the index core here.

#![cfg(windows)]

use std::ffi::{OsStr, c_void};
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};

use anyhow::{Context, Result};
use sourcerer_index::service_index_root;
use sourcerer_rpc::{Server, ServerConfig, SocketPath, service_pipe_name, service_sddl};

use sourcerer_indexd::scanner;
use sourcerer_indexd::{DaemonOptions, DaemonState, IndexdService};
use windows::Win32::Foundation::{ERROR_SERVICE_DOES_NOT_EXIST, NO_ERROR};
use windows::Win32::System::Services::{
    ChangeServiceConfig2W, CloseServiceHandle, CreateServiceW, DeleteService, ENUM_SERVICE_TYPE,
    OpenSCManagerW, OpenServiceW, RegisterServiceCtrlHandlerExW, SC_HANDLE, SC_MANAGER_ALL_ACCESS,
    SERVICE_ACCEPT_SHUTDOWN, SERVICE_ACCEPT_STOP, SERVICE_ALL_ACCESS, SERVICE_AUTO_START,
    SERVICE_CONFIG, SERVICE_CONTROL_INTERROGATE, SERVICE_CONTROL_SHUTDOWN, SERVICE_CONTROL_STOP,
    SERVICE_DESCRIPTIONW, SERVICE_ERROR_NORMAL, SERVICE_RUNNING, SERVICE_START_PENDING,
    SERVICE_STATUS, SERVICE_STATUS_CURRENT_STATE, SERVICE_STATUS_HANDLE, SERVICE_STOP_PENDING,
    SERVICE_STOPPED, SERVICE_TABLE_ENTRYW, SERVICE_WIN32_OWN_PROCESS, SetServiceStatus,
    StartServiceCtrlDispatcherW,
};
use windows::core::PCWSTR;

const SERVICE_NAME: &str = "Sourcerer-Indexd";
const SERVICE_DISPLAY_NAME: &str = "Sourcerer Indexer";
const SERVICE_DESCRIPTION: &str =
    "Maintains the Sourcerer realtime file index by subscribing to NTFS USN journals.";

/// Installs the service. Idempotent over the typical failure modes — if a
/// previous run left a registration behind, the user runs `uninstall` first.
pub fn install(binary_override: Option<&str>) -> Result<()> {
    let binary_path = match binary_override {
        Some(p) => PathBuf::from(p),
        None => current_exe()?,
    };
    let binary_path = binary_path
        .to_str()
        .context("service binary path contains non-Unicode characters")?
        .to_string();
    // SCM expects a full command line; we run with the `service` subcommand.
    let image_path = format!("\"{binary_path}\" service");

    unsafe {
        let scm = OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_ALL_ACCESS)
            .context("OpenSCManagerW (need elevation? this requires admin / UAC)")?;
        let _scm_guard = ScHandleGuard(scm);

        let name_w = wide_z(SERVICE_NAME);
        let display_w = wide_z(SERVICE_DISPLAY_NAME);
        let path_w = wide_z(&image_path);
        let service = CreateServiceW(
            scm,
            PCWSTR(name_w.as_ptr()),
            PCWSTR(display_w.as_ptr()),
            SERVICE_ALL_ACCESS,
            ENUM_SERVICE_TYPE(SERVICE_WIN32_OWN_PROCESS.0),
            SERVICE_AUTO_START,
            SERVICE_ERROR_NORMAL,
            PCWSTR(path_w.as_ptr()),
            PCWSTR::null(),
            None,
            PCWSTR::null(),
            PCWSTR::null(),
            PCWSTR::null(),
        )
        .context("CreateServiceW failed")?;
        let _svc_guard = ScHandleGuard(service);

        let mut desc_w = wide_z(SERVICE_DESCRIPTION);
        let mut desc = SERVICE_DESCRIPTIONW {
            lpDescription: windows::core::PWSTR(desc_w.as_mut_ptr()),
        };
        // SERVICE_CONFIG_DESCRIPTION = 1.
        let _ = ChangeServiceConfig2W(
            service,
            SERVICE_CONFIG(1),
            Some(&mut desc as *mut _ as *mut c_void),
        );
    }

    println!("Installed service `{SERVICE_NAME}` -> `{image_path}` (start type: auto).");
    Ok(())
}

/// Removes the service registration.
pub fn uninstall() -> Result<()> {
    unsafe {
        let scm = OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_ALL_ACCESS)
            .context("OpenSCManagerW (need elevation?)")?;
        let _scm_guard = ScHandleGuard(scm);
        let name_w = wide_z(SERVICE_NAME);
        let service = match OpenServiceW(scm, PCWSTR(name_w.as_ptr()), SERVICE_ALL_ACCESS) {
            Ok(s) => s,
            Err(e) if e.code() == ERROR_SERVICE_DOES_NOT_EXIST.to_hresult() => {
                println!("Service `{SERVICE_NAME}` is not installed; nothing to do.");
                return Ok(());
            }
            Err(e) => return Err(e).context("OpenServiceW failed"),
        };
        let _svc_guard = ScHandleGuard(service);
        DeleteService(service).context("DeleteService failed")?;
    }
    println!("Uninstalled service `{SERVICE_NAME}`.");
    Ok(())
}

/// Entry point invoked when SCM starts our process. Hands control to
/// `StartServiceCtrlDispatcherW` which calls back into `service_main`.
pub fn run_as_service() -> Result<()> {
    let mut name_w = wide_z(SERVICE_NAME);
    let table = [
        SERVICE_TABLE_ENTRYW {
            lpServiceName: windows::core::PWSTR(name_w.as_mut_ptr()),
            lpServiceProc: Some(service_main),
        },
        SERVICE_TABLE_ENTRYW::default(),
    ];
    unsafe {
        StartServiceCtrlDispatcherW(table.as_ptr())
            .context("StartServiceCtrlDispatcherW failed")?;
    }
    Ok(())
}

static SERVICE_HANDLE: OnceLock<usize> = OnceLock::new();
static STOP_REQUESTED: AtomicBool = AtomicBool::new(false);

unsafe extern "system" fn service_main(_argc: u32, _argv: *mut windows::core::PWSTR) {
    let name_w = wide_z(SERVICE_NAME);
    // SAFETY: SCM holds the contract that this function is invoked exactly
    // once per service start, so the static handle is uncontested. Pointer
    // arguments are SCM-owned for the lifetime of this call.
    let h = match unsafe {
        RegisterServiceCtrlHandlerExW(PCWSTR(name_w.as_ptr()), Some(service_ctrl_handler), None)
    } {
        Ok(h) => h,
        Err(_) => return,
    };

    SERVICE_HANDLE.set(h.0 as usize).ok();

    // Initialize logging to a file under PROGRAMDATA — service runs as
    // SYSTEM and has no stderr to inherit. Errors here would otherwise
    // be invisible during dev.
    init_service_logging();

    set_state(SERVICE_START_PENDING, 0, 30_000);

    // Run the daemon body on a fresh tokio runtime. The runtime is
    // owned by this thread for the service's lifetime; STOP signals
    // from SCM flip `STOP_REQUESTED`, which the daemon polls.
    let runtime_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let rt = match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(e) => {
                tracing::error!(error = %e, "service: failed to build tokio runtime");
                return;
            }
        };
        rt.block_on(async {
            if let Err(e) = run_service_daemon().await {
                tracing::error!(error = %e, "service: daemon body returned error");
            }
        });
    }));
    if let Err(_panic) = runtime_result {
        tracing::error!("service: daemon panicked");
    }

    set_state(SERVICE_STOP_PENDING, 0, 5000);
    set_state(SERVICE_STOPPED, 0, 0);
}

fn init_service_logging() {
    // PROGRAMDATA\Sourcerer\logs\indexd-service.log. The service runs
    // as Local System and has no stderr to inherit, so this is the
    // only place errors show up during dev. `Mutex<File>` impls
    // `MakeWriter` cleanly (tracing-subscriber blanket impl), one
    // shared file handle, no per-event reopen.
    let mut log_dir = std::env::var_os("PROGRAMDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\ProgramData"));
    log_dir.push("Sourcerer");
    log_dir.push("logs");
    let _ = std::fs::create_dir_all(&log_dir);
    let log_path = log_dir.join("indexd-service.log");
    let file = match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        Ok(f) => f,
        Err(_) => return,
    };
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_writer(std::sync::Mutex::new(file))
        .with_ansi(false)
        .try_init();
    tracing::info!(
        log = %log_path.display(),
        "sourcerer-indexd service starting"
    );
}

async fn run_service_daemon() -> Result<()> {
    // Service writes to %PROGRAMDATA%\Sourcerer\index (system-wide) so
    // the per-user UI can still read `index.state` stats.
    let index_root = service_index_root().context("service_index_root")?;
    tracing::info!(index_root = %index_root.display(), "service: opening daemon state");

    let opts = DaemonOptions {
        index_root: Some(index_root),
        ..Default::default()
    };
    let state: Arc<DaemonState> = DaemonState::open(opts)?;

    // Well-known pipe + permissive SDDL so unelevated user processes
    // can connect to the elevated service.
    let socket = SocketPath::Pipe(service_pipe_name());
    let sddl = service_sddl();
    tracing::info!(pipe = %service_pipe_name(), "service: binding RPC pipe");

    let service_impl = Arc::new(IndexdService::new(state.clone()));
    let server = Server::new(ServerConfig {
        socket,
        sddl_override: Some(sddl),
    });
    let server_handle = server.spawn(service_impl);

    // SCM expected us to transition RUNNING by now.
    set_state(
        SERVICE_RUNNING,
        SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN,
        0,
    );

    // Kick off an initial scan for every configured folder so the
    // service does useful work on its own — the per-user UI may not
    // launch until much later (or ever, if Sourcerer is being used
    // exclusively via its CLI / HTTP endpoint).
    {
        let folders = state.folders.read().await.clone();
        if folders.is_empty() {
            tracing::info!("service: no folders configured yet; waiting for IPC");
        } else {
            tracing::info!(count = folders.len(), "service: starting initial scans");
            for f in folders {
                let path = std::path::PathBuf::from(&f.path);
                scanner::spawn_scan(state.index.clone(), &path);
            }
        }
    }

    // Wait for SCM stop or the server task to exit.
    loop {
        if STOP_REQUESTED.load(Ordering::SeqCst) {
            tracing::info!("service: stop requested by SCM");
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        if server_handle.is_finished() {
            tracing::warn!("service: RPC accept loop exited unexpectedly");
            break;
        }
    }

    server_handle.abort();
    let _ = state.persist().await;
    tracing::info!("service: clean shutdown");
    Ok(())
}

unsafe extern "system" fn service_ctrl_handler(
    control: u32,
    _event_type: u32,
    _event_data: *mut c_void,
    _context: *mut c_void,
) -> u32 {
    match control {
        SERVICE_CONTROL_STOP | SERVICE_CONTROL_SHUTDOWN => {
            STOP_REQUESTED.store(true, Ordering::SeqCst);
            set_state(SERVICE_STOP_PENDING, 0, 1500);
            NO_ERROR.0
        }
        SERVICE_CONTROL_INTERROGATE => NO_ERROR.0,
        _ => NO_ERROR.0,
    }
}

// SAFETY: `service_main` is called exactly once by SCM during a service
// start; sharing across threads doesn't apply because SCM serializes the
// call.

fn set_state(state: SERVICE_STATUS_CURRENT_STATE, accepts: u32, wait_hint_ms: u32) {
    let Some(raw) = SERVICE_HANDLE.get().copied() else {
        return;
    };
    let handle = SERVICE_STATUS_HANDLE(raw as *mut c_void);
    let status = SERVICE_STATUS {
        dwServiceType: ENUM_SERVICE_TYPE(SERVICE_WIN32_OWN_PROCESS.0),
        dwCurrentState: state,
        dwControlsAccepted: accepts,
        dwWin32ExitCode: NO_ERROR.0,
        dwServiceSpecificExitCode: 0,
        dwCheckPoint: 0,
        dwWaitHint: wait_hint_ms,
    };
    let _ = unsafe { SetServiceStatus(handle, &status) };
}

struct ScHandleGuard(SC_HANDLE);

impl Drop for ScHandleGuard {
    fn drop(&mut self) {
        let _ = unsafe { CloseServiceHandle(self.0) };
    }
}

fn current_exe() -> Result<PathBuf> {
    std::env::current_exe().context("std::env::current_exe")
}

fn wide_z(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
