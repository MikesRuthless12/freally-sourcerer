//! wasmtime sandbox host.
//!
//! Each extraction call instantiates a fresh `Store` so per-call
//! resource budgets (memory + CPU fuel) start clean. Only two host
//! functions are visible to the guest:
//!
//! - `host_log(ptr: i32, len: i32)` — debug logging; truncated at 4 KiB.
//! - `host_now_ms() -> i64` — host-injected milliseconds-since-epoch
//!   (matches the request's `now_ms`); the guest cannot observe wall
//!   time independently.
//!
//! The guest exports `alloc(size) -> i32` (host calls this to allocate
//! request space) and `extract(ptr, len) -> i64` (high 32 bits =
//! result pointer, low 32 bits = result length).

use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use wasmtime::{Caller, Config, Engine, Linker, Memory, Module, Store};

use crate::manifest::Manifest;

#[derive(Debug, Error)]
pub enum HostError {
    #[error("wasmtime: {0}")]
    Wasm(String),

    #[error("guest exited with error: {0}")]
    Guest(String),

    #[error("guest exceeded memory budget: {0} MiB")]
    OutOfMemory(u32),

    #[error("guest exceeded time budget: {0} ms")]
    Timeout(u32),

    #[error("guest output is invalid utf-8 / json: {0}")]
    BadOutput(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("guest is missing export `{0}`")]
    MissingExport(&'static str),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractRequest {
    pub path: String,
    pub bytes: Vec<u8>,
    pub now_ms: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtractResponse {
    pub sections: Vec<Section>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub label: String,
    pub text: String,
}

/// State threaded through the wasmtime call.
struct CallState {
    log_buf: Vec<u8>,
}

pub struct Host {
    engine: Engine,
}

impl Host {
    pub fn new() -> Result<Self, HostError> {
        let mut cfg = Config::new();
        cfg.consume_fuel(true);
        // Cooperative epoch interruption isn't strictly necessary here
        // because we run extractions serially per file and rely on
        // fuel + memory caps; if a future phase wants real preemption
        // we'd flip this on.
        let engine = Engine::new(&cfg).map_err(|e| HostError::Wasm(e.to_string()))?;
        Ok(Self { engine })
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    /// Run one extraction inside a fresh sandbox.
    pub fn run(
        &self,
        manifest: &Manifest,
        sidecar_path: &Path,
        request: &ExtractRequest,
    ) -> Result<ExtractResponse, HostError> {
        let module = Module::from_file(&self.engine, sidecar_path)
            .map_err(|e| HostError::Wasm(e.to_string()))?;

        let mut store: Store<CallState> = Store::new(
            &self.engine,
            CallState {
                log_buf: Vec::with_capacity(1024),
            },
        );
        // Memory budget: cap each call to manifest.memory_budget_mb.
        // Wasmtime doesn't have a direct "total memory cap" on the Store,
        // but we configure the module's memory limit via the Linker's
        // resource_limiter when needed. For Phase 12 we use the simpler
        // approach of measuring memory after the call and rejecting if
        // the guest grew beyond budget.
        // CPU budget: 1 fuel unit ≈ 1 wasm instruction. Default budget
        // gives ~1B instructions per second; a 1s budget is ~1B fuel.
        let fuel = (manifest.time_budget_ms as u64) * 1_000_000;
        store
            .set_fuel(fuel)
            .map_err(|e| HostError::Wasm(e.to_string()))?;

        let mut linker: Linker<CallState> = Linker::new(&self.engine);
        // host_log(ptr, len)
        linker
            .func_wrap(
                "sourcerer",
                "host_log",
                |mut caller: Caller<'_, CallState>, ptr: i32, len: i32| {
                    let mem = match caller.get_export("memory").and_then(|e| e.into_memory()) {
                        Some(m) => m,
                        None => return,
                    };
                    let mut buf = vec![0_u8; len.min(4096) as usize];
                    let _ = mem.read(&mut caller, ptr as usize, &mut buf);
                    if let Ok(s) = std::str::from_utf8(&buf) {
                        tracing::debug!(extractor_log = s);
                        caller.data_mut().log_buf.extend_from_slice(s.as_bytes());
                    }
                },
            )
            .map_err(|e| HostError::Wasm(e.to_string()))?;
        // host_now_ms() -> i64
        let now_ms = request.now_ms as i64;
        linker
            .func_wrap("sourcerer", "host_now_ms", move || -> i64 { now_ms })
            .map_err(|e| HostError::Wasm(e.to_string()))?;

        let instance = linker
            .instantiate(&mut store, &module)
            .map_err(|e| HostError::Wasm(e.to_string()))?;

        let memory: Memory = instance
            .get_memory(&mut store, "memory")
            .ok_or(HostError::MissingExport("memory"))?;
        let alloc = instance
            .get_typed_func::<i32, i32>(&mut store, "alloc")
            .map_err(|_| HostError::MissingExport("alloc"))?;
        let extract = instance
            .get_typed_func::<(i32, i32), i64>(&mut store, "extract")
            .map_err(|_| HostError::MissingExport("extract"))?;

        let payload = serde_json::to_vec(request)
            .map_err(|e| HostError::BadOutput(format!("encode request: {e}")))?;
        let buf_ptr = alloc
            .call(&mut store, payload.len() as i32)
            .map_err(|e| classify_trap(e, manifest))?;
        memory
            .write(&mut store, buf_ptr as usize, &payload)
            .map_err(|e| HostError::Wasm(e.to_string()))?;

        let packed = extract
            .call(&mut store, (buf_ptr, payload.len() as i32))
            .map_err(|e| classify_trap(e, manifest))?;
        let resp_ptr = (packed >> 32) as usize;
        let resp_len = (packed & 0xFFFF_FFFF) as usize;
        if resp_len > 16 * 1024 * 1024 {
            return Err(HostError::BadOutput(format!(
                "result {} bytes exceeds 16 MiB cap",
                resp_len
            )));
        }
        let mut out = vec![0_u8; resp_len];
        memory
            .read(&store, resp_ptr, &mut out)
            .map_err(|e| HostError::Wasm(e.to_string()))?;

        // Memory-budget check (post hoc).
        let used_bytes = memory.data_size(&store);
        let budget = (manifest.memory_budget_mb as usize) * 1024 * 1024;
        if used_bytes > budget {
            return Err(HostError::OutOfMemory(manifest.memory_budget_mb));
        }

        let resp: ExtractResponse =
            serde_json::from_slice(&out).map_err(|e| HostError::BadOutput(e.to_string()))?;
        if let Some(msg) = &resp.error {
            return Err(HostError::Guest(msg.clone()));
        }
        Ok(resp)
    }
}

fn classify_trap(e: anyhow::Error, manifest: &Manifest) -> HostError {
    let s = e.to_string();
    if s.contains("all fuel consumed") {
        return HostError::Timeout(manifest.time_budget_ms);
    }
    if s.contains("out of memory") || s.contains("memory limit") {
        return HostError::OutOfMemory(manifest.memory_budget_mb);
    }
    HostError::Wasm(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    // The full wasmtime round-trip needs a real `*.wasm` sidecar, which
    // we don't ship as a test fixture in Phase 12 (the smoke test
    // phase_12_custom_extractor.rs builds one with `wat` instead).
    // These tests just check the `Host::new` path and trap
    // classification without instantiating.
    #[test]
    fn host_new_succeeds() {
        let _ = Host::new().expect("engine init");
    }

    #[test]
    fn classify_trap_recognizes_fuel() {
        let e = anyhow::anyhow!("wasm trap: all fuel consumed");
        let m = super::super::manifest::Manifest {
            id: "x".into(),
            display_name: "x".into(),
            version: "0".into(),
            formats: vec!["x".into()],
            magic: vec![],
            sidecar: "x.wasm".into(),
            time_budget_ms: 500,
            memory_budget_mb: 16,
        };
        match super::classify_trap(e, &m) {
            HostError::Timeout(500) => {}
            other => panic!("expected Timeout(500), got {other:?}"),
        }
    }
}
