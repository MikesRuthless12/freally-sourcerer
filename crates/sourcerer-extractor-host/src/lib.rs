//! Sourcerer custom-extractor host.
//!
//! Loads community-supplied extractors from `<index_root>/extractors/`
//! and runs them inside a `wasmtime` sandbox with strict guarantees:
//!
//! - **No network.** No `wasi:sockets` capabilities are wired.
//! - **No filesystem write.** The host exposes only a read-only view
//!   of the candidate file's bytes via the request/response ABI.
//! - **No clock beyond what the host provides.** `wasi:clocks` is not
//!   wired; the host injects a deterministic `now_ms` into the request.
//!
//! Each extractor ships a TOML manifest declaring:
//!
//! ```toml
//! id          = "ext.community.markdown-tables"
//! display_name = "Community: Markdown tables"
//! version     = "1.0.0"
//! formats     = ["md", "markdown"]
//! magic       = ["0x23 0x20"]              # optional; "# " prefix
//! sidecar     = "markdown-tables.wasm"
//! time_budget_ms  = 1000
//! memory_budget_mb = 64
//! ```
//!
//! …and a `*.wasm` binary that exports two functions:
//!
//! - `extract(req_ptr: i32, req_len: i32) -> i64` — ABI: high 32 bits
//!   are the result-pointer, low 32 bits are the result-length. The
//!   result is a JSON-encoded `Response`. Any allocation lives inside
//!   the sandbox's linear memory and is freed on instance teardown.
//! - `alloc(size: i32) -> i32` — host calls this to allocate request
//!   space.
//!
//! ## Trust model
//!
//! Custom extractors are *untrusted by default*. The user must flip
//! `trusted = true` in the registry (UI: Settings → Lenses → Custom →
//! per-row Trust toggle) before the host will load the sidecar. Any
//! extractor that crashes or exceeds its budget is automatically
//! disabled until the user re-trusts it. The result row in the UI
//! carries a "community-supplied" badge so the user knows when a
//! third-party extractor served the content.

#![forbid(unsafe_op_in_unsafe_fn)]

pub mod manifest;
pub mod registry;
pub mod sandbox;

pub use manifest::{Manifest, ManifestError};
pub use registry::{Registry, RegistryEntry, RegistryError, RegistrySettings};
pub use sandbox::{ExtractRequest, ExtractResponse, Host, HostError, Section};
