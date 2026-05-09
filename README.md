# Sourcerer

> **One search. Every source. Every OS.**
>
> One search bar that conjures any file from any source on any OS — instant filename, full-text, audio attributes, and fuzzy-name matches, all from the same realtime journal-fed index.

[![ci](https://github.com/MikesRuthless12/Sourcerer/actions/workflows/ci.yml/badge.svg)](https://github.com/MikesRuthless12/Sourcerer/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-All%20Rights%20Reserved-d33)](LICENSE.md)
[![version](https://img.shields.io/badge/version-0.19.84-blue)](#status)
[![platforms](https://img.shields.io/badge/platforms-Win%20%7C%20macOS%20%7C%20Linux-success)](#)
[![rust](https://img.shields.io/badge/rust-2024%20edition-orange)](https://www.rust-lang.org)

---

**Sourcerer is the next *Everything*** — but for Windows, macOS, *and* Linux, with full-text content search, audio-attribute search, and fuzzy-name matching all riding on the same realtime journal-fed index. It's what voidtools' *Everything* would be if it had been written for the world that exists today.

---

## What Sourcerer does

Every operating system ships a search box, and they're all variously broken. Spotlight indexes content but not technical attributes. Windows Search is slow and uneven. Linux ships nothing in particular. *Everything* (voidtools) solved instant filename search on Windows fifteen years ago and has remained Windows-locked ever since.

Sourcerer is the cross-platform answer. A single Rust core subscribes to the OS filesystem journal — **NTFS USN on Windows, FSEvents on macOS, inotify+fanotify on Linux** — and maintains a unified on-disk index. Four query lenses read from the same index:

1. **Filename lens** — instant filename / glob / regex search on multi-million-file disks. Sub-16 ms results.
2. **Content lens** — full-text search across documents, code, archives (peeks inside without extracting), JSON / CSV / YAML.
3. **Audio lens** — query by length, LUFS, codec, sample rate, channels, silence ratio, peak alignment. The first general-purpose audio-attribute searcher.
4. **Similarity lens** — find near-duplicate filenames via bigram MinHash. *"I named it something like…"* actually works.

You type, and Sourcerer streams matches across all four lenses in parallel.

### The magic moment

Open Sourcerer on a fresh install. It crawls in the background. A few minutes later, the index is hot. You type **one character** in the search bar — and within 16 ms the result panel fills with filename matches, content snippets from PDFs and code, audio files matching by attribute, and fuzzy-name suggestions for things you might have meant.

A second character narrows it instantly. *That* is the moment.

```
┌──────────────────────────────────────────────────────────────────────────┐
│ Sourcerer                                                       ⚙  ⤴    │
├──────────────────────────────────────────────────────────────────────────┤
│ 🔍  proj_                                                                │
├──────────────────────────────────────────────────────────────────────────┤
│  📄  FILES (2,471)                          ⏱ 14 ms                     │
│  ─ /Documents/projects/                                                  │
│  ─ /Documents/projects/sourcerer/README.md                               │
│  ─ /Documents/notes/projects-launch-plan.md                              │
│  …                                                                       │
│                                                                          │
│  📝  CONTENT (38)                           ⏱ 22 ms                     │
│  ─ /notes/2026-05-06-brainstorm.md          "…the project lead…"         │
│  ─ /Inbox/exports/sourcerer-roadmap.pdf     "…project pricing model…"    │
│  …                                                                       │
│                                                                          │
│  🎵  AUDIO (3)                              ⏱ 8 ms                      │
│  ─ /Music/sessions/project-theme.flac       44.1 kHz · −16 LUFS · 2:14  │
│  …                                                                       │
│                                                                          │
│  ✨  YOU MIGHT MEAN (4)                     ⏱ 11 ms                     │
│  ─ "Project-AppIdeas-Brainstorm.md"         similarity 0.91             │
│  …                                                                       │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## Why "the next *Everything*"

*Everything* by voidtools is a cult-favorite Windows utility for instant filename search. Mac and Linux users have envied it for fifteen years. Multiple half-attempts have shipped — `mdfind`, `fsearch`, `recoll`, Albert — and none has captured both the speed *and* the cross-platform reach. Sourcerer's framing is direct: **the next *Everything*, but cross-platform, and with content / audio / fuzzy lenses on top.**

The "Sourcerer" name plays on three threads simultaneously:

- **Sorcerer** — wizardry, summoning. You wave a wand (your keyboard) and files appear.
- **Source** — *source* of data. Sourcerer fans out across every source on disk.
- **Source code** — directly relevant to the developer audience that loves *Everything*.

---

## Feature parity with *Everything* — and then some

Sourcerer ships **every** feature voidtools' *Everything* offers, on **all three** operating systems, plus the lenses *Everything* lacks. The bar is parity-or-better on day one of v0.19.84, not over time.

### Filename search parity (matches *Everything* 1:1)

| Capability | *Everything* (Win) | **Sourcerer (Win + macOS + Linux)** |
|---|---|---|
| Real-time journal-fed index | ✅ NTFS USN | ✅ NTFS USN + macOS FSEvents + Linux inotify+fanotify |
| Sub-second results on multi-million-file disks | ✅ | ✅ — sub-16 ms gate |
| Literal substring match | ✅ | ✅ |
| Wildcard (`*`, `?`) | ✅ | ✅ |
| Regex | ✅ | ✅ (Rust `regex` crate, PCRE-style flavor) |
| Boolean: `AND`, `OR`, `NOT`, `!` prefix | ✅ | ✅ |
| Search modifiers: `size:`, `date:`, `ext:`, `attrib:`, `content:`, `path:`, `parent:`, `child:`, … | ✅ | ✅ — superset (adds `lufs:`, `codec:`, `length:`, `similar:`) |
| Match case / whole-word / path / diacritics toggles | ✅ | ✅ |
| Sort by name / path / size / date / type / extension | ✅ | ✅ |
| Multi-column results | ✅ | ✅ |
| Bookmarks / saved searches | ✅ | ✅ |
| Quick filters (audio / video / image / document / executable / archive) | ✅ | ✅ — superset (custom filter authoring) |
| File operations: open / reveal / copy path / properties / delete | ✅ | ✅ — OS-native on each platform |
| Drag-and-drop from results | ✅ | ✅ |
| Result preview pane | ✅ (Win) | ✅ (cross-platform; uses native preview APIs) |
| Optional thumbnails | ✅ | ✅ |
| Global hotkey | ✅ | ✅ — per-OS configurable |
| Multiple languages | ✅ (~40) | ✅ — 18 ship in v0.19.84, others by community |
| Highlighting in results | ✅ | ✅ |
| Service mode (background daemon) | ✅ (Win Service) | ✅ — Win Service / launchd agent / systemd user unit |
| HTTP server for browser search | ✅ | ✅ |
| ETP/FTP server for remote search | ✅ | ✅ — modern variant: HTTPS API with token auth |
| CLI | ✅ (`es.exe`) | ✅ — `sourcerer` CLI on every OS |
| URL protocol handler (`everything://`) | ✅ | ✅ — `sourcerer://` |
| Database file format | ✅ | ✅ — open-spec, documented |
| Volume / path / extension excludes | ✅ | ✅ |
| Network folder indexing | ✅ (limited) | ✅ — first-class on all three OSes |

### Sourcerer's additions (the *"and then some"*)

| Lens | Capability |
|---|---|
| **Content** | Full-text search across documents (PDF, Office, txt, md), code, archive contents (peeks inside without extraction), structured data (JSON, CSV, YAML). |
| **Audio** | Search audio files by length, sample rate, codec, channels, average LUFS, peak amplitude, silence ratio, dynamic range — the first general-purpose audio-attribute searcher. |
| **Similarity** | Find near-duplicate filenames via bigram MinHash. *"I named it something like…"* actually works. |
| **Cross-platform** | Feature parity — Win, macOS, Linux. Same query DSL, same UI, same on-disk index format. |
| **Privacy** | Zero outbound calls. The index never leaves the machine. No telemetry, no AI services, no account. |
| **Light + dark themes** | System (default) / Light / Dark, live-switching, both pre-built into the token set. |
| **Full settings dialog** | Modeled on *Everything*'s Options panel — left-tree nav, right detail pane, OK / Cancel / Apply — with cross-OS volume detection (NTFS / ReFS / APFS / HFS+ / ext4 / Btrfs / ZFS / XFS / F2FS), per-volume journal settings, lens controls, network panel, theme picker. |

The combination is the moat. *Everything* solves filename search for one OS. Sourcerer solves filename + content + audio + fuzzy across three.

---

## Brand mark — the icon

The *Everything* icon is an instantly-recognizable cyan magnifying-glass on a dark square. Sourcerer's icon is intentionally a **sibling** of that mark — same magnifying-glass silhouette, same centered composition — but in **soft baby blue (`#89CFF0`)** instead of *Everything*'s deeper cyan. That's the entire delta: same icon family, lighter / friendlier shade, signals *"this is the Everything family, and it's for every operating system."*

| Spec | Value |
|---|---|
| Silhouette | Same magnifying-glass as *Everything* — circular lens, diagonal handle going down-right, centered composition |
| Lens / handle | `#89CFF0` baby blue · highlight `#C8E8F8` · bevel shadow `#5FA8CD` |
| Canvas | `#0E1116` dark, rounded square (~22% corner radius) |
| Detail | Subtle white reflection arc on the upper-left of the lens (no other embellishments) |
| Delivery | `.icns` (macOS), `.ico` (Windows), Hicolor PNG-set (Linux), SVG master |

The icon is *not* a parody of *Everything* — it is a respectful sibling. Launch communications credit voidtools generously and explicitly position Sourcerer as *"the next Everything, for every OS,"* not as a replacement or competitor.

---

## Architecture in one breath

```
┌─ OS Journal Subscribers ─┐    ┌─ Sourcerer Core (Rust, OS-agnostic) ─────┐
│  Windows: NTFS USN       │    │  Event normalizer                        │
│  macOS:   FSEvents       │ ─▶ │  Indexer (Tantivy + custom name index)   │ ─▶  IPC / HTTPS / CLI / Tauri UI
│  Linux:   inotify/fanot. │    │  Extractor pipeline · Query engine       │
└──────────────────────────┘    └──────────────────────────────────────────┘
```

A single `sourcerer-indexd` daemon owns the on-disk index, subscribes to the OS journal, and runs the extractor pipeline. The Tauri 2 + Svelte 5 UI, the `sourcerer` CLI, and the optional HTTPS API server are thin clients over the same query engine.

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for the full crate boundary map.

---

## Status

Sourcerer is in active phased development. The first public release will be tagged **v0.19.84**.

- [x] Phase 0 — Cross-platform scaffold
- [x] Phase 1 — NTFS USN journal subscriber (Windows)
- [x] Phase 2 — FSEvents journal subscriber (macOS)
- [x] Phase 3 — inotify + fanotify journal subscriber (Linux)
- [x] Phase 4 — Index core (Tantivy + custom name index + SQLite WAL)
- [x] Phase 5 — Filename lens
- [x] Phase 6 — Filename similarity lens
- [x] Phase 7 — Format extractor framework
- [x] Phase 8 — Document extractors (plain-text + Markdown, PDF, Office docx/xlsx/pptx, source code in 32 languages, archive peek, JSON/CSV/YAML)
- [x] Phase 9 — Audio extractor (symphonia decode + EBU R128 LUFS / true peak / silence / dynamic range; `lufs:` / `codec:` / `length:` / `rate:` / `silence:` / `dr:` query modifiers)
- [x] Phase 10 — Query language + parser (hardened recursive-descent parser; `--strict-everything` mode; `name:` / `audio:` / `content:` / `similar:` lens prefixes; AND-children selectivity reorder + lens-routing optimizer; `parse_to_report` IPC with per-token spans + multi-error reporting; 300+ voidtools regression queries + 200+ Sourcerer-DSL queries)
- [x] Phase 11 — Search UI / the magic moment (Tauri 2 + Svelte 5 desktop app: search bar with live tokenization + parse-error pill; lens-grouped results with drag-resize columns + saved profiles; quick-filter palette; bookmarks dropdown + organize dialog; first-run wizard; light/dark/system theme system; PRD §8.28 menu bar — full Everything-equivalent — both as macOS native menu and in-window menu on Win/Linux; PRD §8.29 status bar with all 7 segments + theme pip; preview pane + thumbnail column; global hotkey; `sourcerer://` URL protocol; mock IPC layer that Phase 12 swaps for real `sourcerer-indexd` RPC)
- [x] Phase 12 — Settings + real daemon IPC + custom-extractor framework + 18-locale i18n (`sourcerer-rpc` length-prefixed JSON-RPC over UDS / named pipe with peer-uid / SDDL-DACL auth; `sourcerer-indexd` library + binary refactor with `IndexdService` dispatching every PRD §8.30 method; `apps/sourcerer-ui/src-tauri` rewired as RPC client — `commands/canned.rs` deleted; 26-panel Settings dialog wires every (E) and (+) control through `SettingsDialogModel`; `crates/sourcerer-extractor-host` wasmtime sandbox + crash-counter trust state; `sourcerer` CLI as second client of the same transport; full i18n pass — 18 locales translated into native languages, English-first combobox, live RTL flip for Arabic)
- [ ] Phase 13 — Performance + cross-platform packaging + auto-update
- [ ] Phase 14 — v0.19.84 launch

See [`docs/CHANGELOG.md`](docs/CHANGELOG.md) for per-phase notes.

---

## Building from source

> Sourcerer's source is licensed for viewing only. See [`LICENSE.md`](LICENSE.md) before building or running. No redistribution rights are granted.

**Prerequisites**

- Rust 2024 edition (`rustup install stable`; toolchain pinned in `rust-toolchain.toml`)
- Node.js 22+ and pnpm 10+ for the Tauri UI
- Platform deps:
  - **Linux:** `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `libssl-dev`, `patchelf`
  - **macOS:** Xcode Command Line Tools
  - **Windows:** Visual Studio Build Tools with the C++ workload + WebView2 (preinstalled on Windows 10/11)

**Build the workspace**

```bash
cargo build --workspace
cargo test --workspace
cargo run -p xtask -- i18n-lint
```

**Build the desktop app (debug)**

```bash
cd apps/sourcerer-ui
pnpm install
pnpm tauri build --debug
```

CI runs the same flow on `windows-latest`, `macos-14`, and `ubuntu-22.04` for every push and PR.

---

## Privacy

Zero outbound network calls by default. Sourcerer never sends your filesystem layout — the index never leaves your machine. The HTTPS API server is opt-in. Auto-update is the single, opt-out exception.

See [`docs/SECURITY.md`](docs/SECURITY.md) for the full threat model and dependency policy.

---

## License

**All Rights Reserved — Mike Weaver.** Sourcerer is proprietary software. You may not copy, distribute, modify, or re-host this work without prior written permission. See [`LICENSE.md`](LICENSE.md) for the full notice. Contributions are not actively solicited; any submission is governed by the assignment clause in `LICENSE.md`.

Compiled binaries include third-party libraries under their original permissive licences (MIT / Apache-2.0 / BSD / ISC / CC0 / Unlicense / Unicode / Zlib / MPL-2.0). AGPL, GPL, SSPL, BUSL, CC-BY-NC, and CC-BY-SA dependencies are hard-banned by the project's `cargo-deny` policy. See [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md).

---

## Project documents

| File | Purpose |
|------|---------|
| [`LICENSE.md`](LICENSE.md) | All Rights Reserved — Mike Weaver proprietary licence (DRAFT) |
| [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md) | Third-party dependency attributions |
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | Crate layout & data flow |
| [`docs/CHANGELOG.md`](docs/CHANGELOG.md) | Keep-a-Changelog release notes |
| [`docs/SECURITY.md`](docs/SECURITY.md) | Threat model — privacy is a primary feature here |
| [`docs/I18N_TODO.md`](docs/I18N_TODO.md) | Per-locale translation status (18 ship-locales) |

---

*Sourcerer is © 2026 Mike Weaver. "Sourcerer" and the Sourcerer magnifying-glass mark are trade-marks of Mike Weaver. *Everything* is a trade-mark of voidtools, used here only as a comparator.*
