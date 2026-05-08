# Sourcerer — Security Model

## Threat model

Sourcerer continuously indexes the user's filesystem and, with the content lens enabled, extracts text from documents. The index is one of the most personal artifacts on the user's machine — it represents a near-total map of their files. Threat model is rich.

| Threat | Mitigation |
|--------|-----------|
| Local attacker reads index on disk | Index files written with mode 0600 (Unix) / user-only ACL (Win); located under `%LOCALAPPDATA%` / `~/Library/Application Support/` / `~/.local/share/` (per-user, not system-wide). Optional at-rest encryption available behind a settings toggle (age-encrypted index, key wrapped by passphrase / Keychain / Credential Manager). |
| Malware exfiltrates the index | The daemon runs as the user, not SYSTEM/root. Malware running as the same user has the same access to the underlying files anyway — encrypting the index doesn't help. The architectural mitigation is keeping the index under the user's home, not in shared locations. |
| Malicious IPC client | Pipe / socket ACL'd to the user; server validates command schema strictly; no path arguments accepted that escape the configured index roots; no shell-out APIs. |
| HTTPS server abuse | OFF by default. When enabled: requires per-launch random token shown only in the UI; bound to localhost by default; CORS allowlist is opt-in. Listen on `127.0.0.1` / `[::1]` only unless the user explicitly binds an external interface. |
| URL protocol handler abuse (`sourcerer://`) | The handler opens the UI focused on a query — it does not execute paths or trigger downloads. Query strings are parsed by the same query parser as the search bar, with the same length / complexity bounds. |
| TOCTOU between journal event and index write | Event ordering preserved within a volume via the per-volume USN cursor / FSEvents stream-ID / inotify watch descriptor. Cross-volume events ordered by wall-clock with a clear "best-effort ordering" disclaimer. |
| **(Phase 1)** Stale USN cursor after journal recreation | The Windows subscriber persists a `(volume_serial, journal_id, next_usn)` triple under `%LOCALAPPDATA%\Sourcerer\cursors\`. On restart, if the volume's reported `UsnJournalID` no longer matches, the subscriber discards `next_usn` and bootstraps from the MFT; if `next_usn < FirstUsn` (journal wrap), the subscriber re-seeds to `FirstUsn`. Either case is logged at `info`. The cursor write is process-crash-safe (tmp-then-rename) but **not power-fail-safe** — a hard power loss between rename and the on-disk flush can lose the most recent cursor update; the subscriber recovers by replaying USN events from the previously-persisted position, or by re-bootstrapping the MFT if the journal was recreated. |
| **(Phase 1)** USN journal access requires elevation | Opening `\\.\<drive>:` for `FSCTL_QUERY_USN_JOURNAL` requires admin / SYSTEM. The packaged `Sourcerer-Indexd` Windows Service runs as LocalSystem by default; the foreground `sourcerer-indexd run --volume` mode surfaces a clear `Access is denied` error rather than silently failing. The service install path goes through SCM (`OpenSCManagerW` + `CreateServiceW`) and triggers the standard UAC prompt — no privilege-escalation primitives are introduced. |
| **(Phase 2)** Stale FSEvents stream cursor across volume changes | The macOS subscriber persists a `(root, device, last_event_id, fs_name, bootstrap_complete)` tuple under `~/Library/Application Support/Sourcerer/cursors/<root_hash>.json`. On open, if the watched root's current `stat.st_dev` no longer matches the persisted `device`, the cursor is discarded and the subscriber re-bootstraps; if the persisted `last_event_id` predates the volume's FSEvents history horizon, the kernel emits `MustScanSubDirs`, which triggers an inline subtree rescan. The cursor write is process-crash-safe (tmp-then-rename) but **not power-fail-safe** — same caveat as Phase 1's USN cursor; the `MustScanSubDirs` recovery path covers the lossy edge. |
| **(Phase 2)** macOS FSEvents agent runs without elevation | The launchd plist at `~/Library/LaunchAgents/io.mikeweaver.sourcerer.indexd.plist` is a per-user agent — `launchctl load -w` runs it as the logged-in user, never root. FSEvents per-user streams give us full coverage of the user's home tree without any kernel-mode hook. `Install` / `Uninstall` write only inside `~/Library/`; no system-wide files are touched. The plist's `ProgramArguments` is the absolute path of the running `sourcerer-indexd` binary captured at install time — there's no shell expansion, no PATH lookup, and the `service` subcommand argument is fixed. |
| Index tampering | Tantivy segment hashes verified on commit. Custom name index has an mmap'd checksum trailer verified on open; mismatch triggers WAL replay and surfaces a clear UI error rather than silently using corrupt data. |
| Extractor crashes on hostile input (zip-bomb, malformed PDF, etc.) | Each extractor runs in a panicked-OK Tokio task with a per-document time budget (default 5 s) and memory ceiling (default 256 MB). Extraction failure marks the document `extracted: failed`, never crashes the daemon. |
| Symlink / junction loops during scan | Inode-set on Unix, junction-target tracking on Windows; loop detection skips offending entries with a logged warning. |
| Privilege-escalation via fanotify (Linux) | Sourcerer never runs as root. fanotify upgrade requires user-initiated polkit elevation; Sourcerer-Indexd remains a user unit. The polkit prompt explains exactly what's being elevated and why. |
| AGPL dependency drift (PDF rendering trap) | `cargo-deny` hard-blocks AGPL; CI fails on regressions. We ship pdf-rs (MIT), not mupdf or pdfium. |
| Update-channel attack | Updates signed with a fixed public key embedded in the binary; private key in CI secret; updates served from GitHub Releases over HTTPS only. |
| Dev-leaked telemetry | **Zero outbound network calls** by default. Auto-update is the single exception (1×/24 h to GitHub Releases); fully opt-out in settings. |
| Untrusted custom extractor (Wasm sidecar, Phase 12) | Custom extractors run in a wasmtime sandbox with no network, no filesystem-write, no clock beyond what's provided; a clear "this extractor is community-supplied" badge appears in the UI when one is enabled. |

## Out-of-scope (v0.19.84)

- Physical device theft. (Mitigation is full-disk encryption, not a feature of this app — though optional index encryption is available.)
- Hardware key-logging.
- Compromised OS kernel.
- Side-channel attacks against the on-disk index.

## Dependency policy

`deny.toml` allows: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, CC0, Unlicense, Unicode-DFS-2016, Zlib, MPL-2.0.

`deny.toml` denies: GPL-2.0, GPL-3.0, AGPL-3.0, SSPL-1.0, BUSL-1.1, CC-BY-NC, CC-BY-SA.

`cargo-audit` runs weekly in CI. `cargo-vet` (Mozilla + Embark imports) gates releases.

## Code-signing posture

v0.19.84 ships unsigned MSI + NSIS (Windows), unsigned dmg + pkg (macOS), unsigned deb + rpm + AppImage (Linux). The install docs explain per-OS unsigned-installer prompts:

- **Windows.** SmartScreen reputation builds organically as downloads accumulate. Service install requires UAC (unavoidable, one-time at install).
- **macOS.** Gatekeeper requires right-click → Open on first launch. Documented prominently. Apple Developer Program ($99/yr) deferred to Phase X.
- **Linux.** No equivalent gating; AppImage runs directly; Flatpak / Snap submissions (Phase 13) bring their own signing.

Paid signing (Azure Trusted Signing for Win ~$10/mo, Apple Developer Program for macOS) is a documented Phase X upgrade once revenue justifies it. Hooks left commented in `release.yml`.

## Reporting vulnerabilities

`mythodikalone@gmail.com` subject `SEC: Sourcerer`. PGP fingerprint published with v0.19.84. 90-day responsible-disclosure policy. Coordinated cross-platform disclosure if a vulnerability is platform-specific.

## Hardening flags

```toml
[profile.release]
panic = "abort"
strip = "symbols"
codegen-units = 1
lto = "fat"
overflow-checks = true
debug-assertions = false
```

Per-OS linker flags:

- **Windows.** Stack probes, CFG, /guard:cf, /DYNAMICBASE, /HIGHENTROPYVA. Daemon binary additionally: /CETCOMPAT.
- **macOS.** PIE, stack-check, hardened-runtime entitlements declared in Info.plist (no sandbox in v0.19.84 — full filesystem access is the entire product).
- **Linux.** PIE (`-pie`), stack-clash protection (`-fstack-clash-protection`), full RELRO (`-Wl,-z,relro,-z,now`), `_FORTIFY_SOURCE=2`.

## Privacy posture (user-facing)

The marketing claim is *zero outbound calls*. The single exception is auto-update, which is opt-out. The HTTPS server is opt-in. The custom-extractor framework's Wasm sandbox blocks network. No telemetry, no analytics, no AI services, no account, no sign-in. The first-run wizard explicitly tells the user this in plain language.
