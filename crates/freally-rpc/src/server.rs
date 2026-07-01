//! Server-side accept loop + per-connection task.
//!
//! Holds an `Arc<dyn Service>` and a transport-specific listener. On
//! each accepted connection it spawns a task that:
//!
//! 1. Reads frames as `Request`s.
//! 2. Spawns a per-call task that calls `service.handle_call(...)` and
//!    writes the resulting `Response`.
//! 3. Forwards `Notification`s (server → client) onto the same
//!    connection's outbound writer.
//!
//! The service can push notifications via the `NotificationSink` it
//! receives in `handle_call`, so e.g. a long-running `query.run` can
//! stream `query:batch` events as it discovers them.

use std::sync::Arc;

use serde_json::Value;
use tokio::io::{AsyncRead, AsyncWrite, split};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::error::RpcError;
use crate::frame::{FrameReader, FrameWriter};
use crate::jsonrpc::{ErrorObject, Notification, Request, Response, ResponseEnvelope};
use crate::path::SocketPath;
use crate::service::{NotificationSink, Service};

/// Per-connection outbound queue depth. Notifications and responses
/// queue here; a full queue indicates the client is reading too slowly.
const PER_CONN_OUT_QUEUE: usize = 256;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub socket: SocketPath,
    /// Windows only: optional SDDL string for the pipe's DACL. When
    /// `None`, the transport uses the current-user SDDL (per-user
    /// pipes). The Windows service sets this to `service_sddl()` so
    /// unelevated user processes can connect to the elevated pipe.
    pub sddl_override: Option<String>,
}

impl ServerConfig {
    pub fn new(socket: SocketPath) -> Self {
        Self {
            socket,
            sddl_override: None,
        }
    }
}

pub struct Server {
    cfg: ServerConfig,
}

impl Server {
    pub fn new(cfg: ServerConfig) -> Self {
        Self { cfg }
    }

    /// Bind the listener and run the accept loop until the returned
    /// `tokio::task::JoinHandle` is canceled.
    pub fn spawn<S: Service>(self, service: Arc<S>) -> JoinHandle<()> {
        let cfg = self.cfg;
        tokio::spawn(async move {
            if let Err(e) = run_accept_loop(cfg, service).await {
                tracing::error!(error = %e, "rpc server accept loop terminated");
            }
        })
    }
}

#[cfg(unix)]
async fn run_accept_loop<S: Service>(cfg: ServerConfig, service: Arc<S>) -> Result<(), RpcError> {
    use crate::transport::unix::{UnixListenerExt, listen};
    let path = match &cfg.socket {
        SocketPath::Path(p) => p.clone(),
        SocketPath::Pipe(_) => {
            return Err(RpcError::Other(
                "named-pipe socket on Unix is not supported".into(),
            ));
        }
    };
    let listener = listen(&path)?;
    tracing::info!(path = %path.display(), "rpc server listening");
    loop {
        let stream = listener.accept_authenticated().await?;
        let svc = service.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, svc).await {
                tracing::warn!(error = %e, "connection terminated");
            }
        });
    }
}

#[cfg(windows)]
async fn run_accept_loop<S: Service>(cfg: ServerConfig, service: Arc<S>) -> Result<(), RpcError> {
    use crate::transport::windows::{create_next_instance_with_sddl, listen_with_sddl};
    let pipe_name = match &cfg.socket {
        SocketPath::Pipe(n) => n.clone(),
        SocketPath::Path(_) => {
            return Err(RpcError::Other(
                "filesystem path on Windows is not supported; use a pipe name".into(),
            ));
        }
    };
    let sddl = cfg.sddl_override.clone();
    let mut current = listen_with_sddl(&pipe_name, sddl.as_deref())?;
    tracing::info!(pipe = %pipe_name, "rpc server listening");
    loop {
        current.connect().await?;
        let next = create_next_instance_with_sddl(&pipe_name, sddl.as_deref())?;
        let connected = std::mem::replace(&mut current, next);
        let svc = service.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(connected, svc).await {
                tracing::warn!(error = %e, "pipe connection terminated");
            }
        });
    }
}

/// Public test helper: drive a single connection's accept-loop body on
/// any AsyncRead+AsyncWrite (e.g. `tokio::io::duplex`). Used by the
/// Phase 12 smoke tests so we don't have to spin up a real OS socket
/// for the simpler unit-shaped tests.
pub async fn handle_connection_for_tests<S, T>(stream: T, service: Arc<S>) -> Result<(), RpcError>
where
    S: Service,
    T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    handle_connection(stream, service).await
}

async fn handle_connection<S, T>(stream: T, service: Arc<S>) -> Result<(), RpcError>
where
    S: Service,
    T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let (rd, wr) = split(stream);
    let mut reader = FrameReader::new(rd);
    let mut writer = FrameWriter::new(wr);
    let (out_tx, mut out_rx) = mpsc::channel::<ResponseEnvelope>(PER_CONN_OUT_QUEUE);

    // Writer task: drains `out_rx` and writes each envelope as a frame.
    let writer_task = tokio::spawn(async move {
        while let Some(env) = out_rx.recv().await {
            let payload = match serde_json::to_string(&env) {
                Ok(s) => s,
                Err(e) => {
                    tracing::error!(error = %e, "serializing rpc envelope");
                    continue;
                }
            };
            if let Err(e) = writer.write_frame(&payload).await {
                tracing::warn!(error = %e, "rpc writer failed");
                break;
            }
        }
    });

    // Per-connection notification fan-in: services push Notifications
    // via this sink; we forward them to the writer's queue.
    let (notif_tx, mut notif_rx) = mpsc::channel::<Notification>(PER_CONN_OUT_QUEUE);
    let out_tx_for_notif = out_tx.clone();
    let notif_task = tokio::spawn(async move {
        while let Some(n) = notif_rx.recv().await {
            if out_tx_for_notif
                .send(ResponseEnvelope::Notification(n))
                .await
                .is_err()
            {
                break;
            }
        }
    });
    let sink = NotificationSink::new(notif_tx);

    // Reader loop: parse Requests, dispatch, write Responses.
    loop {
        let frame = match reader.read_frame().await {
            Ok(Some(f)) => f,
            Ok(None) => break,
            Err(e) => {
                tracing::warn!(error = %e, "rpc reader failed");
                break;
            }
        };
        let req: Request = match serde_json::from_str(&frame) {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!(error = %e, "rpc parse error");
                let resp = Response::err(
                    0,
                    ErrorObject::new(crate::error::codes::PARSE_ERROR, e.to_string()),
                );
                let _ = out_tx.send(ResponseEnvelope::Response(resp)).await;
                continue;
            }
        };
        let svc = service.clone();
        let out = out_tx.clone();
        let sink = sink.clone();
        tokio::spawn(async move {
            let id = req.id;
            let params = req.params.unwrap_or(Value::Null);
            let result = svc.handle_call(req.method, params, sink).await;
            let resp = match result {
                Ok(v) => Response::ok(id, v),
                Err(e) => Response::err(id, error_to_object(e)),
            };
            let _ = out.send(ResponseEnvelope::Response(resp)).await;
        });
    }

    drop(out_tx);
    drop(sink);
    let _ = notif_task.await;
    let _ = writer_task.await;
    Ok(())
}

fn error_to_object(e: RpcError) -> ErrorObject {
    use crate::error::codes;
    match e {
        RpcError::MethodNotFound { method } => ErrorObject::new(
            codes::METHOD_NOT_FOUND,
            format!("rpc method `{method}` not found"),
        ),
        RpcError::Json(je) => ErrorObject::new(codes::INVALID_PARAMS, je.to_string()),
        RpcError::Remote {
            code,
            message,
            data,
        } => {
            let mut o = ErrorObject::new(code, message);
            o.data = data;
            o
        }
        other => ErrorObject::new(codes::INTERNAL_ERROR, other.to_string()),
    }
}
