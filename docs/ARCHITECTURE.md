# Sourcerer — Architecture

Cross-platform Rust + Tauri 2 app. Three OS-specific journal subscribers funnel into one OS-agnostic core that owns the on-disk index. Multiple surfaces (Tauri UI, CLI, HTTPS server) read from the same query engine.

## Crate boundaries

```
sourcerer-journal-win    → NTFS USN journal subscriber
sourcerer-journal-mac    → FSEvents subscriber
sourcerer-journal-lin    → inotify + fanotify subscriber
sourcerer-journal        → OS-agnostic facade + JournalEvent normalizer
sourcerer-index          → Tantivy + custom name index (trigram + suffix array)
sourcerer-extractors     → Extractor trait + dispatch + queue + blob store
sourcerer-audio          → symphonia + libebur128 audio attribute extraction
sourcerer-similarity     → bigram MinHash + LSH banding
sourcerer-query          → DSL parser (voidtools-Everything-shaped) + lens executor + 16-entry plan cache; Phase 5 ships the filename lens, future phases plug the content / audio / similarity lenses into the same surface
sourcerer-http           → HTTPS API server (axum + rustls)
sourcerer-i18n           → Fluent loader
sourcerer-cli            → CLI binary (sourcerer)
sourcerer-indexd         → daemon binary (sourcerer-indexd)
apps/sourcerer-ui        → Tauri 2 + Svelte 5 + TS GUI
xtask                    → i18n-lint, third-party-notices, icon-build, release helpers
```

## Process model

Two binaries on every OS:

- **`sourcerer-indexd`** — long-running daemon. Owns the on-disk index, subscribes to the OS journal, runs the extractor pipeline. Survives UI crashes.
  - **Windows**: registered as a Windows Service `Sourcerer-Indexd` at install time.
  - **macOS**: launchd user agent at `~/Library/LaunchAgents/io.mikeweaver.sourcerer.indexd.plist`.
  - **Linux**: systemd user unit at `~/.config/systemd/user/sourcerer-indexd.service`.
- **`sourcerer-ui`** — Tauri app, runs as the user. Connects to the daemon over IPC. Hosts UI, hotkey handler, and (when enabled) the HTTPS server.

A third binary, **`sourcerer`**, is a thin CLI client over the same IPC — `es.exe` parity for Everything users on every OS.

## IPC

A single IPC surface per OS:

- **Windows**: named pipe `\\.\pipe\sourcerer-indexd` (ACL'd to authenticated user).
- **macOS / Linux**: UNIX domain socket at `$XDG_RUNTIME_DIR/sourcerer-indexd.sock` (mode 0600).

Wire protocol: length-prefixed JSON. Every IPC command is a tiny request → response pair, plus a streaming subscription model for query-result streaming and indexing-status pushes.

## Data flow — indexing

```
OS Journal (per-OS)
  │
  ▼
sourcerer-journal-{win|mac|lin}: raw events
  │
  ▼
sourcerer-journal: JournalEvent normalizer (Create | Modify | Delete | Rename | AttrChange)
  │
  ▼
Indexer queue (bounded, with back-pressure)
  │
  ├──→ sourcerer-index: filename / path / numeric fields → Tantivy + name.idx
  │
  ├──→ sourcerer-similarity: bigram MinHash signature → minhash.idx
  │
  └──→ sourcerer-extractors: dispatch by magic-byte + ext
         │
         ├──→ Document extractor → blob store + content text → Tantivy
         │
         └──→ sourcerer-audio: decode + LUFS / peak / silence → numeric fields → Tantivy
```

Extractor work is queued and runs at lower priority than journal-event indexing — the search bar must always be query-responsive even during heavy extraction.

## Data flow — query

```
UI / CLI / HTTPS API
  │
  ▼
IPC: query.search { q, lens?, sort?, limit, offset }
  │
  ▼
sourcerer-query: parse + optimize + lens-route
  │
  ├──→ Filename lens → name.idx (trigram + suffix array) + Tantivy filename field
  │
  ├──→ Content lens → Tantivy content field
  │
  ├──→ Audio lens → Tantivy numeric audio fields + minhash filter
  │
  └──→ Similarity lens → minhash.idx (LSH banding)
  │
  ▼
Result merger → per-lens result groups + timing badges
  │
  ▼
IPC stream → UI / CLI / HTTPS
```

## Threading

- **Daemon**: Tokio multi-thread runtime.
  - One dedicated journal thread per OS subscriber.
  - One indexer-writer task (single-threaded — Tantivy's `IndexWriter` is single-writer).
  - A bounded thread pool for extractors (default `num_cpus / 2`).
  - A bounded thread pool for audio decoding + LUFS computation (default `2`).
  - One IPC accept loop + per-connection task.
- **UI**: Tauri main loop + a per-window thread for the timeline / result-rendering canvas.
- **HTTPS server**: axum on a dedicated tokio runtime when enabled.

## Persistence

```
Index root (per OS):
  Windows : %LOCALAPPDATA%\Sourcerer\index\
  macOS   : ~/Library/Application Support/Sourcerer/index/
  Linux   : ~/.local/share/sourcerer/index/

Settings:
  Windows : %APPDATA%\Sourcerer\settings.toml
  macOS   : ~/Library/Preferences/io.mikeweaver.sourcerer.toml
  Linux   : ~/.config/sourcerer/settings.toml

Logs:
  Windows : %LOCALAPPDATA%\Sourcerer\logs\
  macOS   : ~/Library/Logs/Sourcerer/
  Linux   : ~/.local/state/sourcerer/logs/  (rotating; XDG state dir)
```

Index root layout (identical across OSes):

```
index/
├─ index.tantivy/        # tantivy segment tree
├─ name.idx              # custom trigram + suffix array (mmap)
├─ files.db              # SQLite — FileRecord canonical store (WAL)
├─ extracted/            # text blobs (zstd-compressed, content-addressed)
└─ minhash.idx           # MinHash LSH bands (mmap)
```

## Cross-OS parity contract

The architecture has one rule that supersedes everything else: **every feature exposed in the UI works identically on Windows, macOS, and Linux**. When an OS lacks a primitive (CAP_SYS_ADMIN'd fanotify on Linux, certain Spotlight-only metadata on macOS, certain NTFS-only attributes on Windows), the UI surface either:

1. Provides the same control with a documented fallback path (e.g., inotify-only when fanotify is unavailable), or
2. Hides the OS-specific control entirely (no half-features per platform).

Three-OS CI is the enforcement mechanism — every PR runs against `windows-latest`, `macos-14`, and `ubuntu-22.04`.

## Security boundary

The daemon owns the index file ACLs. The IPC pipe / socket is mode-restricted to the user who started the daemon. The HTTPS server (when enabled) requires a per-launch random token. **No outbound network calls** except auto-update (1×/day to GitHub Releases over HTTPS, opt-out toggle in settings). See `docs/SECURITY.md` for the full threat model.
