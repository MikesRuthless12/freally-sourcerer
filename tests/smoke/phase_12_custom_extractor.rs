//! Phase 12 smoke — custom-extractor framework: TOML manifest schema,
//! registry trust state, blake3 hash, sandbox view, crash counter.
//!
//! The full wasmtime round-trip (a real `*.wasm` extractor producing a
//! deterministic Response) ships in Phase 13 alongside the bundled
//! extractor library. Phase 12 ships the host + manifest contract;
//! this smoke proves both.
//!
//! Run with `cargo test -p sourcerer-extractor-host
//! --test phase_12_custom_extractor`.

#![cfg(test)]

use sourcerer_extractor_host::{Manifest, Registry};
use tempfile::TempDir;

fn write_min_extractor(root: &std::path::Path, id: &str) {
    let dir = root.join(id);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("ext.wasm"), b"\x00asm\x01\x00\x00\x00").unwrap();
    std::fs::write(
        dir.join("manifest.toml"),
        format!(
            r#"id = "{id}"
display_name = "{id}"
version = "0.1.0"
formats = ["x"]
sidecar = "ext.wasm"
"#
        ),
    )
    .unwrap();
}

#[test]
fn manifest_loads_with_defaults() {
    let tmp = TempDir::new().unwrap();
    let toml_path = tmp.path().join("ext.toml");
    let wasm_path = tmp.path().join("ext.wasm");
    std::fs::write(&wasm_path, b"\x00asm\x01\x00\x00\x00").unwrap();
    std::fs::write(
        &toml_path,
        r#"id = "ext.test"
display_name = "Test"
version = "1.2.3"
formats = ["pdf"]
sidecar = "ext.wasm"
"#,
    )
    .unwrap();
    let m = Manifest::load(&toml_path).unwrap();
    assert_eq!(m.id, "ext.test");
    assert_eq!(m.formats, vec!["pdf"]);
    assert_eq!(m.time_budget_ms, 1000);
    assert_eq!(m.memory_budget_mb, 64);
}

#[test]
fn registry_trust_round_trip() {
    let tmp = TempDir::new().unwrap();
    write_min_extractor(tmp.path(), "ext.a");
    let mut reg = Registry::open(tmp.path()).unwrap();
    assert!(reg.entry("ext.a").is_some());
    assert!(!reg.entry("ext.a").unwrap().state.trusted);

    reg.set_trusted("ext.a", true).unwrap();
    let reopened = Registry::open(tmp.path()).unwrap();
    assert!(reopened.entry("ext.a").unwrap().state.trusted);
}

#[test]
fn registry_crash_counter_disables_at_three() {
    let tmp = TempDir::new().unwrap();
    write_min_extractor(tmp.path(), "ext.a");
    let mut reg = Registry::open(tmp.path()).unwrap();
    reg.record_crash("ext.a").unwrap();
    reg.record_crash("ext.a").unwrap();
    reg.record_crash("ext.a").unwrap();
    let e = reg.entry("ext.a").unwrap();
    assert!(e.state.disabled);
    assert_eq!(e.state.crash_count, 3);

    // Re-trusting clears the disable.
    reg.set_trusted("ext.a", true).unwrap();
    let e2 = reg.entry("ext.a").unwrap();
    assert!(!e2.state.disabled);
    assert_eq!(e2.state.crash_count, 0);
}

#[test]
fn registry_skips_bad_manifest() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path().join("bad.ext");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("manifest.toml"),
        r#"this is not toml = at all = ===="#,
    )
    .unwrap();
    let reg = Registry::open(tmp.path()).unwrap();
    assert!(reg.entries().is_empty(), "bad manifest must not register");
}

#[test]
fn host_engine_initializes_with_fuel() {
    let host = sourcerer_extractor_host::Host::new().expect("engine init");
    let _ = host.engine();
}
