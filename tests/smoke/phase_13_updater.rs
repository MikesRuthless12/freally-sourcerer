//! TASK-103 smoke — the updater's signing configuration.
//!
//! What can be checked without a network is the *shape* of the trust
//! anchor, and that turns out to be exactly where this has gone wrong
//! before. The key that signed every release through `v0.23.1` existed
//! only as a GitHub repository secret, which cannot be read back; when
//! the repository was deleted it was destroyed, and in-app updating
//! broke for every install of those versions — silently, as a signature
//! error rather than a failed download.
//!
//! Two rules came out of that, and these tests are them:
//!
//! 1. The embedded public key must be a well-formed minisign key whose
//!    **key id is the one we mean**. A truncated or re-encoded pubkey
//!    still parses as base64 and still ships; it just rejects every
//!    update, and nothing in the app says so.
//! 2. The endpoint must be the `releases/latest` redirect over HTTPS.
//!    Publishing a release without `--latest` already breaks that URL
//!    once; hard-coding a version here would break it permanently.
//!
//! The half that needs a network — do the signatures on a *published*
//! release actually carry this key id — is `scripts/verify-updater-chain.mjs`,
//! which is a release-time step rather than a CI one.

use std::path::PathBuf;

/// The key releases are signed with from `v0.23.2` onward. Its private
/// half and password live in `../freally-updater-key-BACKUP/`, outside
/// this repository. If this constant ever needs changing, the key was
/// rotated, and the rule in that folder's README applies: a pubkey with
/// no matching `.pub` in the backup is a key you do not have.
const EXPECTED_KEY_ID: &str = "E5035E2194EB7E04";

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !p.join("Cargo.toml").exists() || !p.join("docs").exists() {
        if !p.pop() {
            panic!("could not locate workspace root from CARGO_MANIFEST_DIR");
        }
    }
    p
}

fn updater_config() -> serde_json::Value {
    let path = workspace_root()
        .join("apps")
        .join("freally-ui")
        .join("src-tauri")
        .join("tauri.conf.json");
    let raw = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    let conf: serde_json::Value = serde_json::from_str(&raw).expect("tauri.conf.json parses");
    conf["plugins"]["updater"].clone()
}

fn b64(s: &str) -> Vec<u8> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(s)
        .expect("base64")
}

#[test]
fn the_embedded_pubkey_is_the_key_we_mean() {
    let pubkey_b64 = updater_config()["pubkey"]
        .as_str()
        .expect("updater.pubkey is a string")
        .to_string();
    let text = String::from_utf8(b64(&pubkey_b64)).expect("pubkey decodes to minisign text");

    // A minisign public key is two lines: an untrusted comment naming the
    // key id, then the base64 payload. Both are checked, because they can
    // disagree — the comment is free text and a copy-paste can pair one
    // key's comment with another key's bytes.
    let mut lines = text.lines();
    let comment = lines.next().expect("comment line");
    let payload = lines.next().expect("payload line");
    assert!(
        comment.contains(EXPECTED_KEY_ID),
        "pubkey comment names a different key: {comment}"
    );

    // Payload layout: 2-byte algorithm, 8-byte key id (little-endian),
    // 32-byte key.
    let raw = b64(payload);
    assert_eq!(
        raw.len(),
        42,
        "minisign public key is 42 bytes, got {}",
        raw.len()
    );
    let mut id = raw[2..10].to_vec();
    id.reverse();
    let id_hex = id.iter().map(|b| format!("{b:02X}")).collect::<String>();
    assert_eq!(
        id_hex, EXPECTED_KEY_ID,
        "the pubkey's bytes carry a different key id than its comment claims"
    );
}

#[test]
fn the_endpoint_follows_the_latest_release_rather_than_a_version() {
    let endpoints = updater_config()["endpoints"]
        .as_array()
        .expect("updater.endpoints is an array")
        .clone();
    assert!(!endpoints.is_empty(), "no updater endpoint configured");
    for e in &endpoints {
        let url = e.as_str().expect("endpoint is a string");
        assert!(
            url.starts_with("https://"),
            "updater endpoint must be HTTPS — the signature is the only other \
             thing standing between a user and an attacker-supplied installer: {url}"
        );
        assert!(
            url.contains("/releases/latest/download/latest.json"),
            "the endpoint must follow the `latest` redirect, not a pinned \
             version — a pinned one stops finding updates the release after \
             it is written: {url}"
        );
    }
}

#[test]
fn updater_artifacts_are_produced_by_the_bundler() {
    // Without this the release builds installers nobody can update *to*:
    // `latest.json` is generated from the updater artifacts, so turning
    // this off empties the manifest rather than failing the build.
    let path = workspace_root()
        .join("apps")
        .join("freally-ui")
        .join("src-tauri")
        .join("tauri.conf.json");
    let conf: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(
        conf["bundle"]["createUpdaterArtifacts"],
        serde_json::Value::Bool(true)
    );
}
