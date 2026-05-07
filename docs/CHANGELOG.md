# Changelog — Sourcerer

All notable changes documented here. Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versioning: [SemVer](https://semver.org).

---

## [Unreleased]

### Added

- **[all platforms]** Phase 0 scaffold: Cargo workspace; Tauri 2 + Svelte 5 UI shell at 1100×720 dark; 18 locale `.ftl` stubs; `xtask` (`i18n-lint`, `third-party-notices`, `icon-build`, `release`); 3-OS GitHub Actions CI; `deny.toml` license policy (AGPL hard-banned); baby-blue magnifying-glass icon family. First public tag will be **v0.19.84**.
- **[Windows-only]** Phase 1 NTFS USN journal subscriber (`sourcerer-journal-win`): `JournalSubscriber::open` queries the journal via `FSCTL_QUERY_USN_JOURNAL`, `bootstrap()` enumerates the MFT via `FSCTL_ENUM_USN_DATA`, `subscribe()` streams incremental events via `FSCTL_READ_USN_JOURNAL`, and a per-volume cursor (volume serial + journal ID + next USN) persists under `%LOCALAPPDATA%\Sourcerer\cursors\<serial>.json` with rename-atomic save. Reason flags map to `JournalEvent::{Create, Modify, Delete, Rename, AttrChange}`. Will be balanced by the macOS FSEvents subscriber in Phase 2 and Linux inotify/fanotify subscriber in Phase 3.
- **[Windows-only]** `sourcerer-indexd` Service Control Manager wiring: `install` / `uninstall` / `service` subcommands register and run the `Sourcerer-Indexd` Windows Service (auto-start, accepts SCM stop). Phase 4 fills in the per-volume subscriber + index core inside the service body.

### Changed

- **[all platforms]** `sourcerer-journal` facade now re-exports the canonical `JournalEvent` / `JournalError` / `JournalSubscriber` from the Windows subscriber on `cfg(windows)`; non-Windows hosts keep a typed-but-stubbed surface so the workspace builds clean cross-OS.

### Changed

- _(empty)_

### Fixed

- _(empty)_

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
