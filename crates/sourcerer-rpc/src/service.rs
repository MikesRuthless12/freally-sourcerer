//! `Service` — the trait `sourcerer-indexd` implements to dispatch
//! incoming RPC method calls.
//!
//! Each connection runs an event loop that reads frames, parses them as
//! `Request`s, calls `handle_call`, and writes the resulting `Response`
//! back. The service may also send asynchronous `Notification`s through
//! the per-connection `NotificationSink` (e.g. streaming
//! `query:batch` / `query:done` events that the UI emits as Tauri
//! events).

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use tokio::sync::mpsc;

use crate::error::RpcError;
use crate::jsonrpc::Notification;

/// Server-side handle to push notifications down a single connection.
#[derive(Clone)]
pub struct NotificationSink {
    tx: mpsc::Sender<Notification>,
}

impl NotificationSink {
    pub(crate) fn new(tx: mpsc::Sender<Notification>) -> Self {
        Self { tx }
    }

    /// Try to push a notification. Drops the notification if the
    /// connection's outbound queue is full or closed.
    pub async fn send(&self, n: Notification) -> Result<(), RpcError> {
        self.tx
            .send(n)
            .await
            .map_err(|_| RpcError::TransportClosed)
    }

    pub fn try_send(&self, n: Notification) -> Result<(), RpcError> {
        self.tx
            .try_send(n)
            .map_err(|_| RpcError::TransportClosed)
    }
}

/// Generic RPC dispatch trait.
pub trait Service: Send + Sync + 'static {
    fn handle_call(
        self: Arc<Self>,
        method: String,
        params: Value,
        sink: NotificationSink,
    ) -> Pin<Box<dyn Future<Output = Result<Value, RpcError>> + Send>>;
}
