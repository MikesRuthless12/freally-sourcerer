# Changelog — Sourcerer

All notable changes documented here. Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versioning: [SemVer](https://semver.org).

---

## [Unreleased]

### Added

- **[all platforms]** Phase 0 scaffold: Cargo workspace; Tauri 2 + Svelte 5 UI shell at 1100×720 dark; 18 locale `.ftl` stubs; `xtask` (`i18n-lint`, `third-party-notices`, `icon-build`, `release`); 3-OS GitHub Actions CI; `deny.toml` license policy (AGPL hard-banned); baby-blue magnifying-glass icon family. First public tag will be **v0.19.84**.
- **[Windows-only]** Phase 1 NTFS USN journal subscriber (`sourcerer-journal-win`): `JournalSubscriber::open` queries the journal via `FSCTL_QUERY_USN_JOURNAL`, `bootstrap()` enumerates the MFT via `FSCTL_ENUM_USN_DATA`, `subscribe()` streams incremental events via `FSCTL_READ_USN_JOURNAL`, and a per-volume cursor (volume serial + journal ID + next USN) persists under `%LOCALAPPDATA%\Sourcerer\cursors\<serial>.json` with rename-atomic save. Reason flags map to `JournalEvent::{Create, Modify, Delete, Rename, AttrChange}`. Will be balanced by the macOS FSEvents subscriber in Phase 2 and Linux inotify/fanotify subscriber in Phase 3.
- **[Windows-only]** `sourcerer-indexd` Service Control Manager wiring: `install` / `uninstall` / `service` subcommands register and run the `Sourcerer-Indexd` Windows Service (auto-start, accepts SCM stop). Phase 4 fills in the per-volume subscriber + index core inside the service body.
- **[macOS-only]** Phase 2 FSEvents journal subscriber (`sourcerer-journal-mac`): `JournalSubscriber::open` resolves an absolute watch root, captures its `stat.st_dev` + `statfs.f_fstypename`, and loads (or first-runs) a per-watch cursor under `~/Library/Application Support/Sourcerer/cursors/<root_hash>.json`. `bootstrap()` walks the tree and emits synthetic `JournalEvent::Create` events. `subscribe()` spawns a dedicated CFRunLoop thread that runs an `FSEventStreamCreate(latency=0.5s, FileEvents | NoDefer | UseCFTypes | WatchRoot)`, classifies each batch's flag bitmask via the FSEvents-flag → `JournalEvent` table, does **per-batch rename pairing** (matching the two halves of an `ItemRenamed` pair by inode), inline-rescans subtrees on `MustScanSubDirs`, and persists `last_event_id` for resume across restarts. Cross-batch rename pairs degrade to `Delete + Create` (a Phase-13 perf-pass note). Runtime deps `core-foundation = "0.10"`, `core-foundation-sys = "0.8"`, `fsevent-sys = "4"`, `libc = "0.2"` — all MIT/Apache-2.0, deny.toml-allowlisted.
- **[macOS-only]** `sourcerer-indexd` launchd-agent wiring: `install` / `uninstall` / `service` subcommands register and run a per-user launchd agent at `~/Library/LaunchAgents/io.mikeweaver.sourcerer.indexd.plist` with `RunAtLoad=true` + `KeepAlive=true`. Phase 4 fills in the per-root subscriber + index core inside the agent body. The foreground `run --root <path>` mode prints FSEvents events to stdout for manual / smoke-test inspection.
- **[all platforms]** `sourcerer-journal` facade now re-exports the canonical `open` / `JournalEvent` / `JournalError` / `JournalSubscriber` from `sourcerer-journal-mac` on `cfg(target_os = "macos")`. Linux still uses the typed-but-stubbed surface; Phase 3 will replace it.
- **[Linux-only]** Phase 3 inotify+fanotify journal subscriber (`sourcerer-journal-lin`): `JournalSubscriber::open` resolves an absolute watch root, captures its `stat.st_dev` + `statfs.f_type` magic-number-mapped name (ext4/btrfs/zfs/xfs/f2fs/tmpfs/...), detects `CAP_SYS_ADMIN` via `/proc/self/status`'s `CapEff:` line, and loads (or first-runs) a per-watch cursor under `~/.local/share/sourcerer/cursors/<root_hash>.json` (XDG_DATA_HOME-aware). `bootstrap()` walks the tree via raw `getdents64(2)` (faster than `read_dir` on huge trees) with `(st_dev, st_ino)` cycle-guard and emits synthetic `JournalEvent::Create` events. `subscribe()` spawns a dedicated thread that runs the chosen backend: **inotify** (default, no privileges) — recursive `inotify_add_watch` covering create/modify/close-write/delete/move/attr, with `IN_Q_OVERFLOW` triggering a full-tree `getdents64` rescan; or **fanotify** (CAP_SYS_ADMIN required) — one `fanotify_mark(FAN_MARK_FILESYSTEM)` with `FAN_REPORT_DFID_NAME` so rename tracking survives Btrfs subvolume crossings and overlayfs that inotify cannot reproduce. Inotify mask classifier mirrors the Phase-1 USN reason precedence (`Delete > Create`, `Rename > Create`, `IN_CLOSE_WRITE` settles `Modify`). Per-batch rename pairing via inotify cookie / fanotify `OLD_DFID_NAME` info record; cross-batch splits degrade to `Delete + Create` (Phase-13 perf-pass note). fanotify `EPERM/EINVAL/ENOSYS` at init falls through to inotify so kernels < 5.17 (no `FAN_REPORT_DFID_NAME`) and `CONFIG_FANOTIFY=n` builds stay functional. Runtime dep `libc = "0.2"` only — pure raw-syscall path.
- **[Linux-only]** `sourcerer-indexd` systemd-user-unit wiring: `install` / `uninstall` / `service` subcommands write `~/.config/systemd/user/sourcerer-indexd.service` with `Type=simple` + `Restart=always` + `WantedBy=default.target` (per Phase-3 spec) and run `systemctl --user enable --now`. `ExecStart` quotes the binary path so a `--binary "/path with spaces/sourcerer-indexd"` install survives systemd's whitespace-aware unit parser. Phase 4 fills in the per-root subscriber + index core inside the service body. The foreground `run --root <path>` mode prints inotify/fanotify events to stdout for manual / smoke-test inspection.
- **[Linux-only]** Polkit policy at `crates/sourcerer-indexd/polkit/io.mikeweaver.sourcerer.policy` declaring action `io.mikeweaver.sourcerer.elevate` for the optional fanotify upgrade flow. `auth_self_keep` (≈5 min) prompts the active user for their own password; `org.freedesktop.policykit.exec.path` + `argv1` annotations pin `/usr/local/bin/sourcerer-indexd elevate` so the action ID cannot be repurposed against a different binary. Distribution maintainers ship the file at `/usr/share/polkit-1/actions/`.
- **[all platforms]** `sourcerer-journal` facade now re-exports the canonical `open` / `JournalEvent` / `JournalError` / `JournalSubscriber` / `WatchCursor` from `sourcerer-journal-lin` on `cfg(target_os = "linux")`. Other Unix targets (FreeBSD, OpenBSD, illumos) keep the typed-but-stubbed `portable_stub` surface.
- **[all platforms]** Phase 4 index core (`sourcerer-index`) — OS-agnostic façade that consumes the shared `JournalEvent` enum and orchestrates three persistent stores: a Tantivy index (`index.tantivy/`) for full-text + faceted search, a SQLite canonical `files.db` in WAL mode + `synchronous=NORMAL` for the durable `FileRecord` row of truth, and a custom mmap-backed name index (`name.idx` packed string heap + trigram inverted postings; `name.suf` lexicographic suffix array) for substring candidate generation. `Index::open(root)` materializes the directory tree, opens or creates each store, and reconciles drift by replaying the canonical store into the name index when row counts disagree. `Index::apply(&[JournalEvent])` walks Create / Modify / Delete / Rename / AttrChange events through Tantivy delete-then-add + SQLite upsert + name-index upsert/remove with `file_id = blake3(path)[0..8]` as the stable key. `Index::commit()` flushes Tantivy, atomically rewrites `name.idx` + `name.suf` via tmp-rename, checkpoints the SQLite WAL into the main DB, and persists `manifest.json` with the bumped `tantivy_generation` plus per-volume cursors recorded via `Index::record_cursor`. Bounded `EventQueue` (default capacity 10 000 — Build-Guide spec) surfaces back-pressure as `IndexError::QueueFull` rather than silently dropping events; `push_blocking` honors the same close semantics. Per-OS default index root (`%LOCALAPPDATA%\Sourcerer\index` / `~/Library/Application Support/Sourcerer/index` / `${XDG_DATA_HOME:-~/.local/share}/sourcerer/index`) via `default_index_root()`. New runtime deps (all permissive, deny.toml-allowlisted): `tantivy = "0.26"` (MIT), `rusqlite = "0.37"` with `bundled` feature (MIT) — pulls `libsqlite3-sys` + bundled SQLite (public-domain, allow-listed under `Unlicense`), `memmap2 = "0.9"` (MIT/Apache-2.0), `blake3 = "1"` (CC0/Apache-2.0), `parking_lot = "0.12"` (MIT/Apache-2.0). Smoke test `tests/smoke/phase_04_index.rs` covers the directory layout, full event round-trip, kill-9 recovery from SQLite, manifest cursor persistence, queue back-pressure, and Tantivy delete-then-add dedup.

### Changed

- **[all platforms]** `sourcerer-journal` facade now re-exports the canonical `JournalEvent` / `JournalError` / `JournalSubscriber` from the Windows subscriber on `cfg(windows)`, the macOS subscriber on `cfg(target_os = "macos")`, and the Linux subscriber on `cfg(target_os = "linux")`. Other Unix targets keep the typed-but-stubbed `portable_stub` surface.
- **[macOS + Linux]** `sourcerer-indexd` `Run` subcommand's `--root <path>` flag (preferred on macOS / Linux) now also drives the Linux journal subscriber; the existing `--volume` continues to work as a synonym on every OS.

### Fixed (Phase 4 review pass)

- **[all platforms]** Phase 4 `Index::apply` now degrades a `JournalEvent::Rename` whose `old_path` was never indexed (cross-batch rename pair the journal subscriber couldn't pair) into `Delete(old) + synthetic Create(new)` rather than writing a Tantivy / name-index row with no `files.db` row of truth. Mirrors the journal subscribers' published cross-batch fallback contract; new smoke `rename_of_unknown_path_degrades_to_delete_plus_create` covers it.
- **[all platforms]** Phase 4 `EventQueue::close` no longer races against `wait_for_events` / `push_blocking` — `closed` is now stored inside the same `Mutex` as the queue itself instead of a sibling `Mutex`, so `close()`'s `notify_all` cannot land between a waiter's "is the queue empty?" check and its `Condvar::wait`. New smoke `close_unblocks_push_blocking_and_wait_for_events` and `try_push_after_close_refuses` lock the contract in.
- **[all platforms]** Phase 4 `Manifest::load_or_default` now treats a JSON-parse error as missing-and-warn rather than a hard `IndexError::Manifest` that would block `Index::open`. The SQLite canonical store and Tantivy `meta.json` are the durable record; the manifest is a per-commit cache that `Index::commit` rewrites every cycle. New smoke `torn_manifest_does_not_block_open` covers it.
- **[all platforms]** Phase 4 `derive_file_id` now hashes `OsStr::as_encoded_bytes()` directly instead of `to_string_lossy()` so paths that differ only in invalid-UTF-8 bytes don't collapse to the same id. Real-world impact is rare (Linux ext4 / Btrfs filenames are arbitrary byte sequences) but the fix removes a silent collision class before Phase 5 starts depending on `file_id` as a stable hash.

### Fixed

- **[Windows-only]** USN-journal rename pairing on Phase 1's
  `sourcerer-journal-win` now classifies the OLD-name half of a rename
  (and any `FILE_DELETE` record) as **terminal**: emit immediately
  without requiring `USN_REASON_CLOSE`. NTFS does not emit a closing
  record for the old-name session — there's nothing more to wait for
  at that path. Previously the classifier returned `Pending` for
  `RENAME_OLD_NAME` records that lacked `CLOSE`, the pairing table
  stayed empty, and the matching `RENAME_NEW_NAME | CLOSE` record
  silently dropped via `?`. Net effect: `JournalEvent::Rename` was
  never emitted for any in-tree rename. Diagnostic re-run on a real
  NTFS volume confirmed the fix; the integration test
  `realtime_create_modify_rename_delete_round_trip` now passes
  end-to-end and is no longer `#[ignore]`'d.
- **[Windows-only]** `JournalEvent::Delete` now consults the rename
  pairing table by FRN before falling back to the record's
  `build_path` result. Modern Windows uses POSIX-semantic
  `NtSetInformationFile` deletes which internally rename the file to
  a `$.dF{guid}` temp name before issuing `FILE_DELETE`; without this
  lookup the consumer would see `Delete $.dF{guid}` instead of
  `Delete <original_path>`. Defensive: the test that surfaced the
  rename bug saw classic `DeleteFile` behavior here, but the POSIX
  path can fire under file-locked / cross-process scenarios.

### Deprecated

- _(empty)_

### Removed

- _(empty)_

### Security

- _(empty)_

---

## How to update this file

Every phase must add at least one user-perspective entry under `[Unreleased]` before being marked complete. Use sections **Added / Changed / Fixed / Deprecated / Removed / Security**. Cite new crates and licences. Mark API breaks `**BREAKING:**` first.

When tagging a release, rename `[Unreleased]` to `[X.Y.Z] — YYYY-MM-DD` and add a fresh `[Unreleased]` block.

### Cross-platform parity rule

Sourcerer ships on Windows, macOS, and Linux from v0.19.84. **Every entry must call out platform scope** with a bracketed prefix. Use one of:

- `[all platforms]` — change applies to Windows, macOS, and Linux.
- `[Windows-only]`, `[macOS-only]`, `[Linux-only]` — single-OS work. Acceptable mid-phase; should be balanced by the other OSes in subsequent phases.
- `[Windows + macOS]`, `[macOS + Linux]`, `[Windows + Linux]` — partial coverage.

**Do** — explicit scope prefix:

```
Added — [all platforms] Filename-similarity lens via bigram MinHash.
Added — [Windows-only] NTFS USN journal subscriber.
```

**Don't** — no scope prefix (phase review will reject):

```
Added — Filename-similarity lens.
```

Phase reviews will reject changelogs without explicit platform scope.
