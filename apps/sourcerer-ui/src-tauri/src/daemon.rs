//! Spawns the `sourcerer-indexd` binary as a sidecar process and opens a
//! `sourcerer-rpc` client to it.
//!
//! Phase 12 architecture (sidecar mode — the Tauri shell never compiles
//! the heavy daemon dependencies):
//!
//! - Boot: spawn `sourcerer-indexd run` as a child process. The Tauri
//!   shell does NOT depend on `sourcerer-indexd` as a Rust crate — only
//!   on `sourcerer-rpc` (transport types + client) and `sourcerer-query`
//!   (in-process parser). This keeps the src-tauri compile fast on
//!   every CI matrix entry; otherwise pulling in tantivy + wasmtime +
//!   tree-sitter through the daemon dep tree would push Linux Tauri
//!   builds past the runner's reasonable budget.
//! - Locate the `sourcerer-indexd` binary via the env override
//!   `SOURCERER_INDEXD_BIN`, or by searching `cargo`'s
//!   `target/{debug,release}` next to the running executable, or by
//!   relying on `PATH`. Binding socket / pipe at the per-OS default.
//! - Connect: open one `sourcerer-rpc` client connection that all
//!   `#[tauri::command]` handlers share.
//! - Notifications: a long-lived task subscribes to the client's
//!   notification stream and re-emits each one as a Tauri event so the
//!   Svelte side can listen via `listen("query:batch", ...)`.

use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;

use sourcerer_rpc::{Client, ClientHandle, SocketPath, default_socket_path};
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::OnceCell;

pub struct Daemon {
    pub client: ClientHandle,
    /// Held so the sidecar process is killed when the Tauri app exits.
    /// Wrapped in `Mutex` so `Drop` can take ownership of the child.
    _child: parking_lot::Mutex<Option<Child>>,
    /// Held so the notification re-emitter task is kept alive.
    _notif_join: tokio::task::JoinHandle<()>,
    /// Multi-thread runtime owned by the Daemon. Tauri commands enter
    /// this runtime via `Daemon::block_on`.
    pub runtime: Arc<tokio::runtime::Runtime>,
    pub socket: SocketPath,
}

impl Daemon {
    /// Boot the daemon connection. On Windows, prefers the installed
    /// elevated service (well-known pipe `\\.\pipe\sourcerer-indexd`);
    /// falls back to spawning an unelevated child process if the
    /// service isn't running. This is the lever that gives users
    /// Everything-grade speed once they've installed the service.
    pub fn boot(app: &AppHandle) -> anyhow::Result<Self> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("sourcerer-rpc-client")
            .build()?;
        let runtime = Arc::new(runtime);
        let app_for_emit = app.clone();

        // 1) Service-pipe fast path. If the elevated service is
        //    running, connect to it and skip spawning a child entirely.
        #[cfg(windows)]
        {
            let service_socket = SocketPath::Pipe(sourcerer_rpc::service_pipe_name());
            let probe = runtime.block_on(async {
                tokio::time::timeout(
                    std::time::Duration::from_millis(500),
                    Client::connect(service_socket.clone()),
                )
                .await
            });
            if let Ok(Ok(client)) = probe {
                tracing::info!(pipe = %sourcerer_rpc::service_pipe_name(), "connected to sourcerer-indexd service");
                let mut stream = client.notifications();
                let app_for_emit2 = app_for_emit.clone();
                let notif_join = runtime.spawn(async move {
                    while let Some(n) = stream.next().await {
                        let payload = n.params.unwrap_or(serde_json::Value::Null);
                        if let Err(e) = app_for_emit2.emit(&n.method, payload) {
                            tracing::warn!(method = %n.method, error = %e, "tauri emit failed");
                        }
                    }
                });
                return Ok(Self {
                    client,
                    _child: parking_lot::Mutex::new(None),
                    _notif_join: notif_join,
                    runtime,
                    socket: service_socket,
                });
            }
            tracing::info!("sourcerer-indexd service not detected; falling back to child process");
        }

        // 2) Per-user child-spawn fallback (unelevated; uses walkdir).
        let socket = pick_socket(app);
        let bin = locate_indexd_binary().ok_or_else(|| {
            anyhow::anyhow!(
                "could not find `sourcerer-indexd` binary; set `SOURCERER_INDEXD_BIN` or place \
                 it next to the running app"
            )
        })?;
        // Pass the socket path explicitly via `--socket` so a smoke test
        // or dev session that overrides `SOURCERER_RPC_SOCKET` is honored
        // by the spawned daemon.
        let socket_arg = match &socket {
            SocketPath::Path(p) => p.display().to_string(),
            SocketPath::Pipe(p) => p.clone(),
        };
        let mut command = Command::new(&bin);
        command.arg("run").arg("--socket").arg(&socket_arg);
        if let Some(d) = app.path().app_data_dir().ok().map(|d| d.join("index")) {
            command.arg("--index-root").arg(d);
        }
        command.stdin(Stdio::null());
        // Inherit stdout/stderr so the daemon's logs flow into the
        // Tauri app's console. In production the sidecar bundling
        // story will redirect them to `<index_root>/logs/`.
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());
        let child = command
            .spawn()
            .map_err(|e| anyhow::anyhow!("failed to spawn `{}`: {e}", bin.display()))?;

        let socket_for_connect = socket.clone();
        let (client, notif_join) = runtime.block_on(async move {
            wait_for_socket(&socket_for_connect).await;
            // Elevated startups can take several seconds — the daemon may
            // replay its canonical store before opening the pipe. Retry
            // the connect for up to ~20s so we don't lose to that race.
            let client = {
                let mut attempt = 0u32;
                loop {
                    match Client::connect(socket_for_connect.clone()).await {
                        Ok(c) => break c,
                        Err(_) if attempt < 500 => {
                            attempt += 1;
                            tokio::time::sleep(std::time::Duration::from_millis(40)).await;
                        }
                        Err(e) => return Err::<_, anyhow::Error>(e.into()),
                    }
                }
            };
            let mut stream = client.notifications();
            let app = app_for_emit.clone();
            let notif_join = tokio::spawn(async move {
                while let Some(n) = stream.next().await {
                    let payload = n.params.unwrap_or(serde_json::Value::Null);
                    if let Err(e) = app.emit(&n.method, payload) {
                        tracing::warn!(method = %n.method, error = %e, "tauri emit failed");
                    }
                }
            });
            Ok::<_, anyhow::Error>((client, notif_join))
        })?;
        Ok(Self {
            client,
            _child: parking_lot::Mutex::new(Some(child)),
            _notif_join: notif_join,
            runtime,
            socket,
        })
    }

    pub fn block_on<F, T>(&self, fut: F) -> T
    where
        F: std::future::Future<Output = T>,
    {
        self.runtime.block_on(fut)
    }

    /// Convenience: clone the client and run a closure that uses it on
    /// the daemon's runtime.
    pub fn call<P, R>(&self, method: &'static str, params: P) -> Result<R, sourcerer_rpc::RpcError>
    where
        P: serde::Serialize + Send + 'static,
        R: for<'de> serde::Deserialize<'de> + Send + 'static,
    {
        let client = self.client.clone();
        self.runtime
            .block_on(async move { client.call(method, params).await })
    }

    pub fn call_void<P>(
        &self,
        method: &'static str,
        params: P,
    ) -> Result<(), sourcerer_rpc::RpcError>
    where
        P: serde::Serialize + Send + 'static,
    {
        let _: serde_json::Value = self.call(method, params)?;
        Ok(())
    }
}

impl Drop for Daemon {
    fn drop(&mut self) {
        // Kill the spawned daemon so it doesn't outlive the Tauri app.
        // If the daemon is responsive it would exit cleanly via
        // `daemon.shutdown` RPC; killing the child handles the
        // unresponsive case.
        if let Some(mut child) = self._child.lock().take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

/// Determine where to listen — per-OS default unless `SOURCERER_RPC_SOCKET`
/// is set (smoke tests / dev sessions use the env override).
fn pick_socket(app: &AppHandle) -> SocketPath {
    if let Ok(path) = std::env::var("SOURCERER_RPC_SOCKET") {
        if path.starts_with(r"\\.\pipe\") || path.starts_with(r"\\?\pipe\") {
            return SocketPath::Pipe(path);
        }
        return SocketPath::Path(PathBuf::from(path));
    }
    let _ = app;
    default_socket_path()
}

/// Locate the `sourcerer-indexd` executable. Order:
///
///  1. `SOURCERER_INDEXD_BIN` env var (smoke tests / dev sessions).
///  2. Sibling of the current Tauri executable (production bundles).
///  3. `target/{debug,release}/sourcerer-indexd[.exe]` walking up from
///     `cargo manifest dir` (cargo run / cargo tauri dev workflow).
///  4. `PATH` lookup (last resort; matches the deployed-binary case).
fn locate_indexd_binary() -> Option<PathBuf> {
    if let Ok(v) = std::env::var("SOURCERER_INDEXD_BIN") {
        let p = PathBuf::from(v);
        if p.exists() {
            return Some(p);
        }
    }
    let exe_name = if cfg!(windows) {
        "sourcerer-indexd.exe"
    } else {
        "sourcerer-indexd"
    };
    if let Ok(cur) = std::env::current_exe() {
        if let Some(dir) = cur.parent() {
            let sibling = dir.join(exe_name);
            if sibling.exists() {
                return Some(sibling);
            }
            for profile in ["debug", "release"] {
                let candidate = dir.join("..").join(profile).join(exe_name);
                if candidate.exists() {
                    return Some(candidate);
                }
            }
        }
    }
    // PATH fallback.
    if let Ok(path_var) = std::env::var("PATH") {
        let sep = if cfg!(windows) { ';' } else { ':' };
        for dir in path_var.split(sep) {
            let candidate = PathBuf::from(dir).join(exe_name);
            if candidate.exists() {
                return Some(candidate);
            }
        }
    }
    None
}

async fn wait_for_socket(socket: &SocketPath) {
    use std::time::Duration;
    let mut tries = 0;
    loop {
        let ready = match socket {
            SocketPath::Path(p) => p.exists(),
            // On Windows the named-pipe is created at listen time; the
            // client side retries on PIPE_BUSY / NOT_FOUND anyway.
            SocketPath::Pipe(_) => true,
        };
        if ready {
            return;
        }
        if tries > 250 {
            tracing::warn!(?socket, "socket did not appear; client.connect will fail");
            return;
        }
        tokio::time::sleep(Duration::from_millis(40)).await;
        tries += 1;
    }
}

/// Lazily-initialized handle used by `[#tauri::command]` bodies. We
/// can't store `Daemon` directly in `tauri::State` and pass it across
/// `tokio::spawn` boundaries simultaneously; the OnceCell pattern lets
/// every command body resolve to the same daemon.
static DAEMON: OnceCell<Arc<Daemon>> = OnceCell::const_new();

pub fn install(daemon: Arc<Daemon>) {
    let _ = DAEMON.set(daemon);
}

pub fn get() -> Option<Arc<Daemon>> {
    DAEMON.get().cloned()
}
