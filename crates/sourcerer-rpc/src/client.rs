//! RPC client.
//!
//! `Client::connect(socket)` returns a `ClientHandle` that you can clone
//! freely. Every `ClientHandle::call(method, params)` writes a `Request`
//! frame and awaits the matching `Response`. Server-pushed
//! `Notification`s drop into a broadcast channel that callers subscribe
//! to via `ClientHandle::notifications()`.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use serde::Serialize;
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncWrite, split};
use tokio::sync::{Mutex, broadcast, mpsc, oneshot};
use tokio::task::JoinHandle;

use crate::error::{RpcError, RpcResult};
use crate::frame::{FrameReader, FrameWriter};
use crate::jsonrpc::{Notification, Request, RequestId, Response, ResponseEnvelope};
use crate::path::SocketPath;

const NOTIFICATION_BUS_CAPACITY: usize = 1024;
const OUTBOUND_QUEUE_CAPACITY: usize = 256;

#[derive(Clone)]
pub struct ClientHandle {
    inner: Arc<Inner>,
}

pub struct Client;

struct Inner {
    next_id: AtomicU64,
    pending: Arc<Mutex<HashMap<RequestId, oneshot::Sender<Response>>>>,
    out: mpsc::Sender<Request>,
    notifications: broadcast::Sender<Notification>,
    _bg: JoinHandle<()>,
}

pub struct NotificationStream {
    rx: broadcast::Receiver<Notification>,
}

impl NotificationStream {
    pub async fn next(&mut self) -> Option<Notification> {
        match self.rx.recv().await {
            Ok(n) => Some(n),
            Err(broadcast::error::RecvError::Closed) => None,
            // Lagging is non-fatal — drop and resync.
            Err(broadcast::error::RecvError::Lagged(_)) => self.rx.recv().await.ok(),
        }
    }
}

impl Client {
    pub async fn connect(socket: SocketPath) -> RpcResult<ClientHandle> {
        match socket {
            #[cfg(unix)]
            SocketPath::Path(p) => {
                let stream = crate::transport::unix::connect(&p).await?;
                Ok(ClientHandle::from_stream(stream))
            }
            #[cfg(windows)]
            SocketPath::Pipe(p) => {
                let stream = crate::transport::windows::connect(&p).await?;
                Ok(ClientHandle::from_stream(stream))
            }
            #[cfg(unix)]
            SocketPath::Pipe(_) => Err(RpcError::Other(
                "named-pipe socket on Unix is not supported".into(),
            )),
            #[cfg(windows)]
            SocketPath::Path(_) => Err(RpcError::Other(
                "filesystem socket on Windows is not supported".into(),
            )),
        }
    }
}

impl ClientHandle {
    /// Start a client over an arbitrary AsyncRead+AsyncWrite. Used by
    /// the `Client::connect` shim and by tests over `tokio::io::duplex`.
    pub fn from_stream<T>(stream: T) -> Self
    where
        T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        let (rd, wr) = split(stream);
        let pending: Arc<Mutex<HashMap<RequestId, oneshot::Sender<Response>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let (out_tx, mut out_rx) = mpsc::channel::<Request>(OUTBOUND_QUEUE_CAPACITY);
        let (notif_tx, _) = broadcast::channel::<Notification>(NOTIFICATION_BUS_CAPACITY);

        let pending_writer = pending.clone();
        let writer_task = tokio::spawn(async move {
            let mut writer = FrameWriter::new(wr);
            while let Some(req) = out_rx.recv().await {
                let payload = match serde_json::to_string(&req) {
                    Ok(s) => s,
                    Err(e) => {
                        tracing::error!(error = %e, "client serializing request");
                        // Reply locally with InternalError so callers don't
                        // wait forever for a request the wire never saw.
                        if let Some(tx) = pending_writer.lock().await.remove(&req.id) {
                            let _ = tx.send(Response::err(
                                req.id,
                                crate::jsonrpc::ErrorObject::new(
                                    crate::error::codes::INTERNAL_ERROR,
                                    "client serialization failed",
                                ),
                            ));
                        }
                        continue;
                    }
                };
                if let Err(e) = writer.write_frame(&payload).await {
                    tracing::warn!(error = %e, "client writer failed");
                    break;
                }
            }
        });

        let pending_reader = pending.clone();
        let notif_for_reader = notif_tx.clone();
        let reader_task = tokio::spawn(async move {
            let mut reader = FrameReader::new(rd);
            loop {
                let frame = match reader.read_frame().await {
                    Ok(Some(f)) => f,
                    Ok(None) => break,
                    Err(e) => {
                        tracing::warn!(error = %e, "client reader failed");
                        break;
                    }
                };
                let env: ResponseEnvelope = match serde_json::from_str(&frame) {
                    Ok(e) => e,
                    Err(e) => {
                        tracing::warn!(error = %e, "client parse error");
                        continue;
                    }
                };
                match env {
                    ResponseEnvelope::Response(r) => {
                        if let Some(tx) = pending_reader.lock().await.remove(&r.id) {
                            let _ = tx.send(r);
                        }
                    }
                    ResponseEnvelope::Notification(n) => {
                        // Lagged receivers are tolerated.
                        let _ = notif_for_reader.send(n);
                    }
                }
            }
            // Reader closed → flush all pending callers with a clean
            // TransportClosed shape.
            for (id, tx) in pending_reader.lock().await.drain() {
                let _ = tx.send(Response::err(
                    id,
                    crate::jsonrpc::ErrorObject::new(
                        crate::error::codes::INTERNAL_ERROR,
                        "transport closed",
                    ),
                ));
            }
        });

        let bg = tokio::spawn(async move {
            let _ = writer_task.await;
            let _ = reader_task.await;
        });

        let inner = Inner {
            next_id: AtomicU64::new(1),
            pending,
            out: out_tx,
            notifications: notif_tx,
            _bg: bg,
        };
        Self {
            inner: Arc::new(inner),
        }
    }

    /// Subscribe to server-sent notifications. Each subscriber gets its
    /// own broadcast receiver — late subscribers see only notifications
    /// sent after they subscribed.
    pub fn notifications(&self) -> NotificationStream {
        NotificationStream {
            rx: self.inner.notifications.subscribe(),
        }
    }

    /// Issue a typed request. Params and result are serde-shaped.
    pub async fn call<P, R>(&self, method: &str, params: P) -> RpcResult<R>
    where
        P: Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        let v = self.call_raw(method, serde_json::to_value(params)?).await?;
        Ok(serde_json::from_value(v)?)
    }

    pub async fn call_raw(&self, method: &str, params: Value) -> RpcResult<Value> {
        let id = self.inner.next_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = oneshot::channel::<Response>();
        self.inner.pending.lock().await.insert(id, tx);
        let req = Request::new(id, method, params);
        if self.inner.out.send(req).await.is_err() {
            self.inner.pending.lock().await.remove(&id);
            return Err(RpcError::TransportClosed);
        }
        let resp = match rx.await {
            Ok(r) => r,
            Err(_) => return Err(RpcError::TransportClosed),
        };
        if let Some(err) = resp.error {
            return Err(RpcError::Remote {
                code: err.code,
                message: err.message,
                data: err.data,
            });
        }
        Ok(resp.result.unwrap_or(Value::Null))
    }
}
