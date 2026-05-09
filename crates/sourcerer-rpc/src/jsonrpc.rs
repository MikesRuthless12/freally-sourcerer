//! JSON-RPC 2.0 envelope types.
//!
//! Sourcerer's RPC restricts request ids to non-negative `u64` so they
//! pair-up cleanly with the in-memory pending-request map. Notifications
//! (server → client) carry a method name and params and never expect a
//! response.

use serde::{Deserialize, Serialize};

pub const JSONRPC_VERSION: &str = "2.0";

pub type RequestId = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub jsonrpc: String,
    pub id: RequestId,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

impl Request {
    pub fn new(id: RequestId, method: impl Into<String>, params: serde_json::Value) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id,
            method: method.into(),
            params: Some(params),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub jsonrpc: String,
    pub id: RequestId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,
}

impl Response {
    pub fn ok(id: RequestId, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn err(id: RequestId, error: ErrorObject) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorObject {
    pub code: i32,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl ErrorObject {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

impl Notification {
    pub fn new(method: impl Into<String>, params: serde_json::Value) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method: method.into(),
            params: Some(params),
        }
    }
}

/// Wire envelope: a frame is exactly one of these.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseEnvelope {
    Response(Response),
    Notification(Notification),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn request_round_trip() {
        let r = Request::new(1, "query.run", json!({ "source": "foo" }));
        let s = serde_json::to_string(&r).unwrap();
        let back: Request = serde_json::from_str(&s).unwrap();
        assert_eq!(back.method, "query.run");
        assert_eq!(back.id, 1);
    }

    #[test]
    fn response_envelope_distinguishes_response_vs_notification() {
        let resp = Response::ok(1, json!({ "ok": true }));
        let s = serde_json::to_string(&resp).unwrap();
        let env: ResponseEnvelope = serde_json::from_str(&s).unwrap();
        assert!(matches!(env, ResponseEnvelope::Response(_)));

        let note = Notification::new("query:batch", json!({ "handle": "h1" }));
        let s = serde_json::to_string(&note).unwrap();
        let env: ResponseEnvelope = serde_json::from_str(&s).unwrap();
        assert!(matches!(env, ResponseEnvelope::Notification(_)));
    }
}
