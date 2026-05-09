//! In-process embedding of `sourcerer-indexd` plus the
//! `sourcerer-rpc` client the Tauri commands route through.
//!
//! Phase 12 architecture:
//!
//! - Boot: spawn the daemon as an in-process tokio task at the per-OS
//!   default socket path. The transport is real (UDS / named pipe with
//!   peer-uid auth) but loops back to the same process.
//! - Connect: open one client connection that all `#[tauri::command]`
//!   handlers share.
//! - Notifications: a long-lived task subscribes to the client's
//!   notification stream and re-emits each one as a Tauri event so the
//!   Svelte side can listen via `listen("query:batch", ...)`.
//!
//! For Phase 13 packaging, the daemon will be split into a sidecar
//! process; the Tauri side stops calling `sourcerer_indexd::spawn_at`
//! and instead spawns the binary, then connects to the same socket.
//! The `Daemon` struct's surface stays identical across that swap.

use std::path::PathBuf;
use std::sync::Arc;

use sourcerer_indexd::{DaemonOptions, DaemonState};
use sourcerer_rpc::{Client, ClientHandle, SocketPath, default_socket_path};
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::OnceCell;

pub struct Daemon {
    pub state: Arc<DaemonState>,
    pub client: ClientHandle,
    /// Held so the daemon's accept loop is not dropped before app exit.
    _server_join: tokio::task::JoinHandle<()>,
    /// Held so the notification re-emitter survives across calls.
    _notif_join: tokio::task::JoinHandle<()>,
    /// Multi-thread runtime owned by the Daemon. Tauri commands enter
    /// this runtime via `Daemon::block_on`.
    pub runtime: Arc<tokio::runtime::Runtime>,
    pub socket: SocketPath,
}

impl Daemon {
    /// Boot the daemon + connect a client. The returned `Daemon` is
    /// stored in `app.manage()` and shared across every Tauri command
    /// via `State<'_, Daemon>`.
    pub fn boot(app: &AppHandle) -> anyhow::Result<Self> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("sourcerer-daemon")
            .build()?;
        let runtime = Arc::new(runtime);
        let app_for_emit = app.clone();
        let socket = pick_socket(app);
        let socket_clone = socket.clone();
        let opts = DaemonOptions {
            index_root: app
                .path()
                .app_data_dir()
                .ok()
                .map(|d| d.join("index")),
            ..Default::default()
        };
        let (state, server_join, client, notif_join) = runtime.block_on(async move {
            let state = DaemonState::open(opts)?;
            let server_join = sourcerer_indexd::spawn_at(state.clone(), socket_clone.clone()).await?;
            // A small wait so the listener bind completes before we
            // try to connect. The listener only writes its socket file
            // once it has bound; the connect attempt below races that
            // moment in practice.
            wait_for_socket(&socket_clone).await;
            let client = Client::connect(socket_clone.clone()).await?;
            // Spawn the notification re-emitter.
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
            Ok::<_, anyhow::Error>((state, server_join, client, notif_join))
        })?;
        Ok(Self {
            state,
            client,
            _server_join: server_join,
            _notif_join: notif_join,
            runtime,
            socket,
        })
    }

    /// Run an async block on the daemon's runtime. Tauri commands wrap
    /// their body in this so they can `await` the RPC client without
    /// needing every command to be `async`.
    pub fn block_on<F, T>(&self, fut: F) -> T
    where
        F: std::future::Future<Output = T>,
    {
        self.runtime.block_on(fut)
    }

    /// Convenience: clone the client and run a closure that uses it on
    /// the daemon's runtime. This avoids the move/borrow conflict that
    /// arises from `Arc<Daemon>::block_on` capturing `Arc<Daemon>` by
    /// move in the same expression.
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

/// Determine where to listen — we use the per-OS default unless an env
/// override is set (used by smoke tests to avoid stomping on a running
/// production daemon).
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

async fn wait_for_socket(socket: &SocketPath) {
    use std::time::Duration;
    let mut tries = 0;
    loop {
        let ready = match socket {
            SocketPath::Path(p) => p.exists(),
            // On Windows the named-pipe is created at listen time, but
            // ConnectNamedPipe blocks until a client connects; the
            // client side will retry on PIPE_BUSY anyway. We don't gate
            // on existence here.
            SocketPath::Pipe(_) => true,
        };
        if ready {
            return;
        }
        if tries > 50 {
            tracing::warn!(?socket, "socket did not appear; client.connect will fail");
            return;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
        tries += 1;
    }
}

/// Lazily-initialized handle used by `[#tauri::command]` bodies that
/// need an `Arc<Daemon>`. We can't store `Daemon` directly in
/// `tauri::State` and also pass it across `tokio::spawn` boundaries —
/// the runtime is owned by `Daemon`, and we want the spawned tasks to
/// outlive the command return. Instead, we put `Arc<Daemon>` inside a
/// `OnceCell` so every command body resolves to the same daemon.
static DAEMON: OnceCell<Arc<Daemon>> = OnceCell::const_new();

pub fn install(daemon: Arc<Daemon>) {
    let _ = DAEMON.set(daemon);
}

pub fn get() -> Option<Arc<Daemon>> {
    DAEMON.get().cloned()
}
