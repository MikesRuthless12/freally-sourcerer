# Freally Sourcerer — Feature Roadmap: Must-Haves, Nice-to-Haves & Post-Stable Phases

**Date:** 2026-07-09
**Product:** Freally Sourcerer (Havoc Software / Mike Weaver) — "the next voidtools Everything, for every OS"
**Baseline:** Everything-parity table is **DONE** (README parity table); Phases 0–12 complete; Phase 13 (perf/packaging/auto-update) and Phase 14 (launch) remain before stable **v0.23.0** *(originally v0.19.84 — superseded: the app is already at v0.20.1, so the 2026-07-19 versioned build plan below re-pins stable to the last Must-Have build)*.

This document proposes what comes **next** — features that do **not** appear anywhere in README.md, PRD.md, ROADMAP.md, PRODUCT-VISION.md, ARCHITECTURE.md, or CHANGELOG.md. Where a proposal extends something that already exists (folder-size indexing, the preview pane, the HTTPS endpoint connection, the reserved `dupe:`/`count:` tokens, the MinHash similarity lens, the §8.24 debug overlay), the **new delta is stated explicitly**. Everything in the existing parity table, PRD §8.x settings/menus/status-bar, the four lenses, the ROADMAP's Phase 0–14 tasks, the bug-report/EULA/installer/Playwright addenda, and the §14 deferred specs (network shares 1.1, cloud buckets 1.2, X-05 image-palette lens, A-07 video perceptual-hash lens, X-04 AST-pattern lens) is treated as already planned and is **not** re-proposed here.

---

> **⚠️ DoD amendment — 2026-07-15: every phase's Definition of Done includes a `/simplify` pass.**
> Run **`/simplify` on the full phase diff** — in unison with **`/code-review`** and
> **`/security-review`** — and apply its cleanups (reuse over duplication, no speculative
> abstraction/config, remove orphans the change created) before the phase is called done and
> before any push. Standing DoD item for **all remaining and future phases**.

## How to read this

- **Must-Haves (SRC-M01…SRC-M24)** are the *stable gate*: close them **before tagging stable v0.23.0** (they land as Builds 1–3 in the versioned build plan below), alongside the existing Phase 13/14 work. They are the gaps reviewers will hit in the first hour when comparing Freally against Everything 1.5 alpha, Listary 6, and Agent Ransack — small enough for a solo developer to land inside the pre-stable window, big enough that shipping without them undercuts the "parity-or-better" launch story.
- **Nice-to-Haves (SRC-N01…SRC-N82)** ship **after** stable, sequenced into themed post-stable phases (Phase 15 → Phase 25). Every Nice-to-Have appears in exactly one phase, every phase ends with a verifiable Definition of Done, and — per the versioned build plan below — every phase ships as exactly one versioned build/release (Builds 4–14, ending at v1.0.0).
- **Non-negotiable constraints reaffirmed for every feature below:**
  - **No AI/ML.** No embeddings, no semantic search, no OCR. Tree-sitter parsing, blake3/perceptual hashing, classic IR (trigrams, BM25-style scoring), EXIF/ID3 metadata parsing, and DSP (FFT, loudness) are all fine — they are deterministic algorithms, not models.
  - **$0 forever, to Mike and to users.** No paid APIs, services, or infrastructure. Permissive-license dependencies only — the existing `cargo-deny` hard-ban on GPL/AGPL/SSPL/BUSL applies to every new crate named here.
  - **Local-only.** The index never leaves the machine. No cloud, no accounts, no telemetry. The very few LAN features are explicitly labeled **(LAN-only, no internet)** and are opt-in.
  - **Solo-dev buildable.** No kernel drivers, no paid OS programs, no certificate-store shenanigans beyond what Phase 13 already plans.
- Every feature carries either *(Inspired by: X)* — naming the researched competitor(s) it was mined from — or *(Freally-unique — none of the researched apps have this)*.

---

## Versioned build plan — one release per build, ending at v1.0.0

> **⚠️ Versioning amendment — 2026-07-19 (Mike): every build below ships as exactly one release with one version bump.**
> The app sits at **v0.20.1** today, so the old stable target v0.19.84 is stale — this ladder supersedes it.
> Must-Have builds come first because they close the stable gate: the 24 stable-gate features, kept in ID order,
> sliced into three equal builds of 8, with **stable tagged at v0.23.0** (the last Must-Have build). Every
> post-stable phase (Phase 15 → Phase 25) already sizes 5–10 features, so **each phase = exactly one build** —
> no splits or merges were needed. The minor version bumps once per build (v0.21.0 → v0.33.0), and the final
> build — Phase 25 — ships as **v1.0.0** exactly. Backlog items stay outside the builds, as before. No features
> were added, dropped, or re-themed: this is a release-slicing overlay on the existing lists, nothing more.

> **⚠️ DoD amendment — 2026-07-19: `/simplify` is part of EVERY build's Definition of Done.**
> The 2026-07-15 amendment at the top of this document already binds every *phase* — which covers
> Builds 4–14 (Phases 15–25) — but Builds 1–3 are Must-Have slices, not phases, so this restates the
> rule at the build level: **every build in the table below (Builds 1–14, v0.21.0 → v1.0.0) runs
> `/simplify` on the full build diff — in unison with `/code-review` and `/security-review` — and
> applies its cleanups (reuse over duplication, no speculative abstraction/config, remove orphans the
> change created) before the build's release is tagged and shipped.** Standing DoD item for every
> build in this plan and any build added later.

| Done | Build | Version | Theme | Features |
|---|---|---|---|---|
| [x] | 1 | v0.21.0 | Must-Have stable gate — slice 1 of 3 | SRC-M01 … SRC-M08 |
| [x] | 2 | v0.22.0 | Must-Have stable gate — slice 2 of 3 | SRC-M09 … SRC-M16 |
| [x] | 3 | v0.23.0 | Must-Have stable gate — slice 3 of 3 · **stable tag** | SRC-M17 … SRC-M24 |
| [ ] | 4 | v0.24.0 | Phase 15 — Property Lens I: Images & Video | SRC-N01 … SRC-N07 |
| [ ] | 5 | v0.25.0 | Phase 16 — Property Lens II: Documents, Binaries & Packages | SRC-N08 … SRC-N15 |
| [ ] | 6 | v0.26.0 | Phase 17 — Music Tags & Audio Pro | SRC-N16 … SRC-N21 |
| [ ] | 7 | v0.27.0 | Phase 18 — Duplicates & Disk Hygiene | SRC-N22 … SRC-N29 |
| [ ] | 8 | v0.28.0 | Phase 19 — Tags, Notes & Collections | SRC-N30 … SRC-N36 |
| [ ] | 9 | v0.29.0 | Phase 20 — Power Query: Macros, Snapshots, Monitors & Hashes | SRC-N37 … SRC-N45 |
| [ ] | 10 | v0.30.0 | Phase 21 — Views & Windows | SRC-N46 … SRC-N53 |
| [ ] | 11 | v0.31.0 | Phase 22 — Launcher & OS Integration | SRC-N54 … SRC-N60 |
| [ ] | 12 | v0.32.0 | Phase 23 — CLI, TUI & Automation | SRC-N61 … SRC-N67 |
| [ ] | 13 | v0.33.0 | Phase 24 — LAN, Catalogs & Index Pro | SRC-N68 … SRC-N75 |
| [ ] | 14 | **v1.0.0** | Phase 25 — Code & Knowledge Lenses | SRC-N76 … SRC-N82 |

---

## Competitor research summary

Live web research, 2026-07-09. Features listed are the ones actually mined into this roadmap.

| App | What it is | Standout features mined |
|---|---|---|
| **voidtools Everything 1.5 alpha** (Win) | The instant-filename-search gold standard; 1.5 alpha is a huge leap over the 1.4 parity baseline Freally already matches | Property indexing/searching/sorting (`width:`, `height:`, `framerate:`, `artist:`, `album:`, `author:`, `binary-type:`, `bpm:` …), Find Duplicates + `dupe:` family, `empty:`/`child-count:`/`descendant-count:`, index journal, offline volumes, undo system, natural sort, ignore-punctuation/whitespace, prefix/suffix matching, weighted searches, search preprocessor, hard-link tracking, ADS search functions, bookmarks sidebar, `add-column:`/layouts, filter colors, import/export settings, Everything Server |
| **Listary 6** (Win) | Search-as-you-type utility fused into the OS shell | File-dialog **Quick Switch** (Open/Save dialogs jump to the active Explorer folder), in-dialog search, launcher actions on results, pinyin matching, find-as-you-type inside Explorer |
| **Fluent Search** (Win) | Rich desktop search engine with "search processes/tabs" reach | Search-within-results, result operations/verbs, quake-style overlay UI, tagged searches, preview cards |
| **PowerToys Run / Command Palette** (Win, MS) | Microsoft's free launcher palette | Launcher mode (apps/settings/actions), extension-style plugins, quake-style summon, calculator/quick actions in palette |
| **Flow Launcher / Wox** (Win) | Community keyboard launchers with plugin stores | Everything-backed file search plugin pattern, shell-command mode, action keywords, plugin ecosystem ergonomics |
| **Alfred 5** (macOS) | Veteran mac launcher | **File buffer** (collect files across searches, act once), file actions panel, workflows-without-cloud |
| **Raycast** (macOS) | Modern mac command bar | Quicklinks, per-result action palette, clipboard/path copy ergonomics |
| **LaunchBar 6** (macOS) | Classic abbreviation-based launcher | Send-to actions, instant-send file staging, abbreviation matching discipline |
| **Spotlight** (macOS) | Built-in content search | Baseline to beat; property predicates (`kMDItem*`) inform the property-lens shape |
| **HoudahSpot 6** (macOS) | Pro Spotlight front-end | Column/criteria builder over file properties, saved templates, text-content preview with hit list, Finder-tag search, Full-Disk-Access guidance |
| **Find Any File** (macOS) | Non-indexed deep finder | Searches what Spotlight can't reach (root mode), hierarchical results view honesty about permissions |
| **Agent Ransack / FileLocator Lite** (Win) | Content-search stalwart | **Hit-in-context preview with line numbers and prev/next-hit navigation**, boolean content expressions, tabbed searches (Pro), hex view (Pro), scheduled/automated searches (Pro) |
| **DocFetcher** (cross-platform) | Open-source desktop full-text search | Format breadth (.eml/.mbox, ODF, RTF, EPUB), portable-mode operation, indexed-folder honesty |
| **Recoll** (Linux/xdg) | Xapian-based full-text veteran | Query language over rich formats, mbox/maildir extraction, result snippets with term highlighting |
| **AnyTXT Searcher** (Win) | Local full-text with instant preview | Zero-config content preview pane with match highlight; format coverage expectations |
| **UltraSearch** (Win) | MFT-direct search, no index service | Per-column quick filters, folder tree drill-down, file-version/product info columns |
| **WizFile / WizTree** (Win) | MFT-speed file list & disk-space analyzer | **Treemap disk-usage view**, top-largest-files list, per-extension space breakdown — all at MFT speed |
| **grepWin** (Win) | Regex search **and replace** in files | Multi-file regex replace with backup, regex test/help UI, capture-group rename patterns |
| **ripgrep / fd / fzf / ripgrep-all** (CLI) | The modern CLI search toolchain | `--json`/NDJSON output, `-0` null separation, shell completions, `.gitignore` semantics, fzf-style interactive TUI with preview pane, rga's "search inside everything" attitude |
| **Czkawka** (cross-platform) | Fast Rust dupe/junk finder | Tiered duplicate detection (size → partial hash → full hash), similar-images via perceptual hash, similar-music via fingerprint, empty files/dirs, broken symlinks, review-then-delete workflow |
| **Tabbles** (Win) | Tag-based file organization | Tag hierarchies, auto-tagging rules, per-file comments, tag combos as virtual folders |
| **TagSpaces** (cross-platform) | Offline file tagging/notes | Sidecar/filename tagging that survives sync, color tags, no-cloud discipline |

**Primary sources consulted:** [Everything 1.5 features](https://www.voidtools.com/everything-1.5/) · [Everything search functions](https://www.voidtools.com/support/everything/search_functions/) · [Everything Find Duplicates](https://www.voidtools.com/support/everything/find_duplicates/) · [Everything 1.5 forum](https://www.voidtools.com/forum/viewforum.php?f=12) · [Listary Quick Switch docs](https://help.listary.com/quick-switch) · [Listary changelog](https://dl.listary.net/changelog.html) · [Fluent Search features guide](https://fluentsearch.net/posts/fluent-search-features-guide) · [PowerToys Run](https://learn.microsoft.com/en-us/windows/powertoys/run) · [PowerToys Command Palette](https://learn.microsoft.com/en-us/windows/powertoys/command-palette/overview) · [Flow Launcher](https://www.flowlauncher.com/) · [Raycast Quicklinks manual](https://manual.raycast.com/quicklinks) · [HoudahSpot](https://www.houdah.com/houdahSpot/) · [Agent Ransack information](https://www.mythicsoft.com/agentransack/information/) · [FileLocator Pro information](https://www.mythicsoft.com/filelocatorpro/information/) · [Recoll](https://www.recoll.org/) · [WizTree](https://www.diskanalyzer.com/) · [grepWin](https://tools.stefankueng.com/grepWin.html) · [Czkawka](https://github.com/qarmin/czkawka) · [TagSpaces](https://www.tagspaces.org/products/lite/) · [fzf](https://github.com/junegunn/fzf) · [ripgrep-all fzf integration](https://github.com/phiresky/ripgrep-all/wiki/fzf-Integration)

---

## Must-Have features (stable-gate)

Close these before tagging stable v0.23.0. Each is new versus the existing docs; deltas against adjacent existing work are stated. Per the versioned build plan, they land in ID order as three builds of 8 — one release and one version bump each — with the stable tag on Build 3 (v0.23.0).

### Build 1 — v0.21.0 (SRC-M01 … SRC-M08)

- [x] **SRC-M01 — Hit-in-context content viewer.** Full-document view for content-lens results: every match highlighted, line numbers, prev/next-hit navigation (F3/Shift+F3), jump-to-first-hit on open, and a per-file match-count badge. This is the delta beyond the existing 200-char inline snippet and the preview pane, which show only one fragment with no hit navigation. *(Inspired by: Agent Ransack / FileLocator Lite)*

- [x] **SRC-M02 — Search within results.** A second-stage query box (Ctrl+Shift+F) that narrows the current result set instead of re-running from scratch, rendered as a removable chip stack so refinements compose and peel off one at a time. *(Inspired by: FileLocator Pro; Fluent Search)*

- [x] **SRC-M03 — Everything-interop import/export (.efu, CSV, TXT, M3U/M3U8, NDJSON).** Extend the existing `Export Results…` menu item and the File Lists panel (currently Text/JSON/.srcb only) with Everything's `.efu` format both ways — import an `.efu` as a searchable file list and export any result set to `.efu`, CSV, TXT, M3U/M3U8 (audio results become playlists), or NDJSON. The delta is the interop formats; export/file-lists themselves already exist. *(Inspired by: voidtools Everything file lists)*

- [x] **SRC-M04 — "Open with…" on results.** Context-menu submenu listing registered handlers for the file type on all three OSes (Win `IOpenWithLauncher`/registry, macOS `LSCopyApplicationURLsForURL`, Linux `.desktop` MIME associations), plus a per-extension "always use this app in Freally" override. FR-013 today ships open/reveal/copy/properties/delete only. *(Inspired by: Listary 6 actions; OS file managers)*

- [x] **SRC-M05 — Copy contents / copy as file / copy path list.** Three new clipboard verbs beyond the existing copy-path/copy-name: copy a text file's *contents*, copy the file itself as an OS clipboard file object (CF_HDROP / NSPasteboard file URL / GNOME `x-special/gnome-copied-files`), and copy a multi-selection as a path list with quoting style options (quoted, escaped, one-per-line, space-separated). *(Inspired by: voidtools Everything advanced copy; Raycast)*

- [x] **SRC-M06 — Terminal-here + custom command actions.** "Open terminal here" on any result (Windows Terminal/PowerShell, Terminal.app/iTerm2, x-terminal-emulator) plus user-defined actions: named commands with `{path}`/`{dir}`/`{name}` placeholders, optional per-extension scoping, and a keyboard-assignable slot — all local processes, no network. *(Inspired by: Listary 6; Flow Launcher shell actions)*

- [x] **SRC-M07 — Duplicate finder v1 (`dupe:` family goes live).** Implement the currently *reserved, parse-only* `dupe:` / `size-dupe:` / `name-dupe:` tokens: same-name, same-size, and same-name+size duplicate detection straight from the existing index (no hashing yet), with a grouped results view that clusters each duplicate set under a header row. Hash-confirmed tiers and the review center land post-stable (SRC-N22/N23). *(Inspired by: Everything 1.5 Find Duplicates)*

- [x] **SRC-M08 — Empty & emptiness modifiers (`empty:`, `child-count:`, `descendant-count:`).** Find empty files, empty folders, and folders by child/descendant count directly from index data, including "roots of empty subtrees" so nested empty chains collapse to their top. The DSL today has `size:` only; `count:` is reserved but unimplemented. *(Inspired by: Everything 1.5)*

### Build 2 — v0.22.0 (SRC-M09 … SRC-M16)

- [x] **SRC-M09 — CLI machine-readable output.** `freally search` gains `--json` (single document), `--ndjson` (one object per hit, streaming), `--csv`, `--fields name,path,size,mtime,…` column selection, `-0` null-separated paths for `xargs -0`, `--limit/--offset`, and meaningful exit codes (0 hits found / 1 no hits / 2 error). The current CLI streams human-oriented batches only. *(Inspired by: ripgrep, fd, es.exe export flags)*

- [x] **SRC-M10 — Shell completions.** Generated completions for bash, zsh, fish, and PowerShell covering subcommands, flags, and — dynamically — modifier keywords (`size:`, `lufs:`, `codec:` …) and saved-search names, installed by the packages and printable via `freally completions <shell>`. *(Inspired by: fd, fzf, ripgrep)*

- [x] **SRC-M11 — Typo-tolerant "did you mean" fallback.** When the filename lens returns zero hits, automatically re-rank similarity-lens candidates with bounded Damerau-Levenshtein edit distance and surface a one-click "did you mean *freallly* → *freally*?" strip. Delta versus the existing MinHash similarity lens: that lens is a separate result group the user must think to read; this wires edit-distance correction into the filename lens's empty state. *(Inspired by: fzf/Fluent Search fuzzy matching; extends the MinHash lens)*

- [x] **SRC-M12 — CJK phonetic matching (pinyin / romaji / jamo).** Optional per-locale match modes: type `wenjian` or `wj` to match 文件 (full pinyin + initials), romaji to match kana, and jamo-initial matching for Korean — built from permissive Unicode data tables (Unihan readings, kana tables, jamo decomposition is pure algorithm), indexed as auxiliary name keys. Ships as an opt-in toggle next to the existing diacritics toggle. *(Inspired by: Listary 6 pinyin search)*

- [x] **SRC-M13 — Index health panel v1 + rebuild advisor.** A Tools → Index Health panel showing, per volume: journal event lag (event→query-visible ms), dropped/coalesced event counters, last-event timestamp, extraction backlog depth, and a rules-based advisor ("USN journal wrapped on C: — rebuild recommended") with a one-click fix. Delta versus §8.24's debug overlay (global latency/memory/queue numbers only): per-volume lag, drop ledger, and actionable advice. *(Freally-unique — none of the researched apps have this)*

- [x] **SRC-M14 — Offline removable-volume catalogs v1.** When a USB/external volume unplugs, keep its subtree searchable as a named catalog: results show an "offline — *Orange WD 4TB*" badge, `volume:` filters by catalog, and the answer to "which drive was that file on?" is one query away. Delta versus today: the PRD only ghost-caches unmounted volumes as an edge case and offers "automatically remove offline volumes"; this makes offline retention a deliberate, named, per-device feature (full catalog manager arrives in SRC-N69). *(Inspired by: Everything 1.5 offline volumes — extended cross-platform)*

- [x] **SRC-M15 — Bulk rename on result set.** Multi-select → Rename: regex find/replace with capture groups, `{n}`/`{n:03}` counters, case transforms, live before/after preview table with conflict detection, and one-step undo of the whole batch. Single-file inline rename exists ("do not select extension when renaming"); batch rename does not. *(Inspired by: PowerToys PowerRename; Everything 1.5 multi-file rename)*

- [x] **SRC-M16 — Undo/redo for file operations.** An operation journal for Freally-initiated actions — delete (to OS trash), rename, bulk rename, copy-to/move-to-folder — with Ctrl+Z/Ctrl+Shift+Z, an undo history popover, and cross-session persistence of the last N operations. Today delete/rename ship with no undo. *(Inspired by: Everything 1.5 undo system)*

### Build 3 — v0.23.0 (SRC-M17 … SRC-M24) — stable tag

- [x] **SRC-M17 — Portable mode.** A zip/AppImage-friendly mode: `freally --portable` (or a `portable.flag` beside the binary) keeps index, settings, bookmarks, and logs in a `Data/` folder next to the executable, skips service/registry/launchd/systemd registration, and survives running from a USB stick on all three OSes. *(Inspired by: voidtools Everything portable zip; Agent Ransack Lite portable)*

- [x] **SRC-M18 — Inline audio/video playback in the preview pane.** Transport controls (play/pause/seek/loop), a rendered waveform for audio (reusing the symphonia decode path), volume, and the existing LUFS/codec badges overlaid — so Audio-Engineer Aria can audition `lufs:<-14` results without leaving Freally. Delta versus the existing preview pane: it renders static previews; it does not play media uniformly on the three OSes. *(Inspired by: HoudahSpot 6 preview)*

- [x] **SRC-M19 — Spacebar Quick Look.** Press Space on any result for a large modal preview; arrow keys move through results while the modal stays open; Space again dismisses. Uses native QuickLook on macOS and Freally's own preview host on Windows/Linux. Delta versus the docked preview pane: a transient, keyboard-flow modal. *(Inspired by: macOS Quick Look; HoudahSpot; Find Any File)*

- [x] **SRC-M20 — Regex builder & live tester.** A popover next to the regex toggle: pattern input with syntax coloring, a cheat-sheet of the Rust-regex flavor, and live match highlighting against the current top-50 result names before committing the query. Delta versus the existing parse-error pill and AST hover: those validate the DSL; nothing helps compose the regex itself. *(Inspired by: grepWin regex help)*

- [x] **SRC-M21 — Permission health report + macOS Full Disk Access wizard.** Turn the existing "n paths skipped (permission)" badge into a drill-down report (which subtrees, which reason, per volume), with guided fixes: a macOS Full Disk Access walkthrough with live detection, a Linux fanotify/polkit explainer (hooking existing elevation), and a Windows ACL note — plus a saved "files I couldn't index" list so users know what search *can't* see. *(Inspired by: HoudahSpot / Find Any File permission handling)*

- [x] **SRC-M22 — Bookmarks & filters sidebar.** An optional left sidebar (View → Sidebar) listing bookmarks, quick filters, volumes/catalogs, and recent searches as clickable nodes with drag-reorder — one-click scoping instead of the dropdown-only bookmarks UI that ships today. *(Inspired by: Everything 1.5 bookmarks sidebar)*

- [x] **SRC-M23 — Ignore-punctuation / ignore-whitespace / prefix-suffix matching.** Three new match toggles beside case/whole-word/path/diacritics: ignore punctuation (`foo-bar` matches `foobar`), ignore whitespace, and explicit prefix (`name^:`) / suffix (`name$:`) matching modes in the DSL. *(Inspired by: Everything 1.5 match options)*

- [x] **SRC-M24 — Natural sort.** Numeric-aware name/path ordering (`file2` before `file10`, `v1.9` before `v1.10`) as the default name sort with a settings opt-out, applied consistently in every column sort and honored by the fast-sort indexes. *(Inspired by: Everything 1.5 natural sort)*

---

## Nice-to-Have features

Post-stable. Grouped here in ID order; phase sequencing follows in the next section.

### Property lens — images & video

- [ ] **SRC-N01 — Property-lens framework.** A fifth lens: lazily-extracted, journal-invalidated file *properties* (numeric/text/enum) with a `property:` DSL namespace, sortable property columns, per-property index on/off toggles, and per-format extraction budgets — the substrate every property feature below plugs into, mirroring how the audio lens already caches technical attributes. *(Inspired by: Everything 1.5 property indexing)*

- [ ] **SRC-N02 — Image dimension properties.** `width:`, `height:`, `dimensions:`, `aspect-ratio:`, `orientation:landscape|portrait|square`, `megapixels:` parsed from headers (PNG/JPEG/GIF/WebP/BMP/TIFF/AVIF) without full decode. Distinct from the deferred X-05 image-*content* (palette) lens — this reads container metadata only. *(Inspired by: Everything 1.5 `width:`/`height:`)*

- [ ] **SRC-N03 — EXIF capture properties.** `date-taken:`, `camera-make:`, `camera-model:`, `iso:`, `aperture:`, `focal-length:`, `flash:`, plus a has-GPS flag — via a permissive-license EXIF parser, fully offline (no map tiles, no geocoding). *(Inspired by: Everything 1.5 EXIF properties; HoudahSpot criteria)*

- [ ] **SRC-N04 — Image format internals.** `bit-depth:`/`bpp:`, color space, has-alpha, animated (frame count), progressive/interlaced, and embedded-ICC-profile presence — the "why is this PNG 40 MB" toolkit. *(Inspired by: Everything 1.5 `bpp:`)*

- [ ] **SRC-N05 — Video technical properties.** `video-codec:`, container, `duration:`, `resolution:`/`framerate:`, `video-bitrate:`, audio-track count, subtitle-track presence — parsed from MP4/MKV/WebM/AVI headers with permissive parsers (mp4parse, matroska crates). Distinct from the deferred A-07 perceptual-hash video lens: metadata only, no frame analysis. *(Inspired by: Everything 1.5 `framerate:`/video properties)*

- [ ] **SRC-N06 — Property columns & auto-layouts.** Any indexed property can become a result column (`add column` from the header context menu); per-quick-filter automatic column sets (Pictures shows dimensions, Video shows duration/resolution) with save/recall. Delta versus existing column profiles: profiles cover the six fixed columns; this makes columns data-driven from the property registry. *(Inspired by: Everything 1.5 `add-column:` / layouts)*

- [ ] **SRC-N07 — Result-set statistics popover.** Click the status-bar result count for aggregate analytics of the current result set: total bytes, count and size by extension/kind, largest/oldest/newest, and property histograms (e.g., resolution distribution) — computed lens-side, instant. *(Freally-unique — none of the researched apps have this)*

### Property lens — documents, binaries & packages

- [ ] **SRC-N08 — Document properties.** `author:`, `title:`, `subject:`, `keywords:`, `page-count:`, `word-count:`, `created-with:` (producer/application) from PDF info dictionaries and OOXML `core.xml`/`app.xml` — riding the existing PDF/Office extractors, stored as properties rather than body text. *(Inspired by: Everything 1.5 `author:`; HoudahSpot)*

- [ ] **SRC-N09 — Executable & binary properties.** `binary-type:` (x86/x64/ARM64, PE/Mach-O/ELF), version-info product/company/file-version, and signature status + publisher CN — Authenticode via WinVerifyTrust on Windows, codesign/notarization check via Security.framework on macOS; Linux shows binary-type only (documented exception: no universal signing scheme). *(Inspired by: Everything 1.5 `binary-type:`)*

- [ ] **SRC-N10 — Installer/package properties.** MSI product name/version/manufacturer (permissive `msi` crate), `.deb`/`.rpm` package name-version-arch from headers, macOS `.pkg` bundle IDs — so "which MSI is the v2.3 installer?" is a query, not a spelunk. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N11 — Shortcut & link resolution.** Index `.lnk` targets/arguments (permissive `lnk` crate), `.desktop` `Exec=`, symlink/alias targets; add a Target column, `link-target:` search, and a `broken-links:` finder for dangling targets. *(Inspired by: Czkawka broken symlinks; Everything shortcut target column)*

- [ ] **SRC-N12 — Archive properties.** `entry-count:`, compression ratio, `encrypted-archive:` flag, and multi-volume detection surfaced from the existing archive-peek extractor (which currently indexes entry *names* only) as sortable properties. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N13 — Font metadata properties.** Family, style/weight, glyph count, supported-scripts summary, and embedding-permission bits from TTF/OTF/WOFF2 via `ttf-parser` (MIT) — the designer's "which of my 4,000 fonts have Cyrillic" query. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N14 — Email extractor (.eml / .mbox).** Parse RFC-822 messages and mbox stores: From/To/Subject/Date become properties (`from:`, `subject:`, `mail-date:`), bodies feed the content lens, attachments list as virtual entries like archive-peek does. *(Inspired by: DocFetcher / Recoll mail indexing)*

- [ ] **SRC-N15 — SQLite content peek.** For `.db`/`.sqlite` files: index table names, column names, row counts (properties) and optionally the first N rows of text columns (content lens, off by default) — read-only, `immutable=1` open, hard time budget. *(Freally-unique — none of the researched apps have this)*

### Music tags & audio pro

- [ ] **SRC-N16 — Music tag properties.** `artist:`, `album:`, `title:`, `album-artist:`, `genre:`, `year:`, `track:`, `composer:`, `bpm:` (tag-read), `initial-key:` from ID3v2/Vorbis comments/MP4 atoms/FLAC tags via `lofty` (MIT/Apache). Delta versus the audio lens: it measures *signal* attributes (LUFS/codec/silence); this reads *tag* metadata — together they make `artist:radiohead lufs:<-14` possible. *(Inspired by: Everything 1.5 `artist:`/`album:`/`track:`)*

- [ ] **SRC-N17 — Embedded artwork surfacing.** Extract cover art as the thumbnail for tagged audio, show it in preview/Quick Look, and add `has-artwork:` + artwork-dimensions properties for library QC. *(Inspired by: Everything 1.5 media properties)*

- [ ] **SRC-N18 — Cue/playlist awareness.** Parse `.cue` sheets into virtual track entries (searchable per-track titles/times) and index `.m3u`/`.pls` playlist membership so `playlist-contains:` finds every playlist referencing a file — plus a broken-playlist-entry finder. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N19 — Similar-audio finder.** Acoustic near-duplicate grouping via Chromaprint-style fingerprints from a permissive Rust reimplementation (`rusty-chromaprint`) computed during the existing decode pass — classic DSP hashing, fully offline, no AcoustID lookups ever. Finds the same master at different bitrates/containers. *(Inspired by: Czkawka similar-music)*

- [ ] **SRC-N20 — Loudness compliance reports.** Batch-check any result set against a chosen target (−14 Spotify / −16 Apple / −23 EBU, reusing the existing per-standard settings) and export a CSV/JSON report of integrated LUFS, true peak, and pass/fail deltas. Delta: the metrics already exist per-file; the batch compliance report does not. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N21 — Audio QC flags.** New derived detectors on the existing decode pass: clipping (true peak > −0.1 dBFS), long leading/trailing silence, DC offset, and channel imbalance — exposed as `qc:clipping`-style modifiers and a QC column for session-folder triage. *(Freally-unique — none of the researched apps have this)*

### Duplicates & disk hygiene

- [ ] **SRC-N22 — Duplicate finder tiers 2–3 (hash-confirmed).** Escalating pipeline on top of SRC-M07 groups: size match → partial blake3 (first+last 128 KiB) → full blake3 confirmation, with cached hashes invalidated by journal events, progress UI, and cancel — million-file-safe by design. *(Inspired by: Czkawka tiered hashing; Everything 1.5 Find Duplicates)*

- [ ] **SRC-N23 — Duplicate review center.** A dedicated review workspace: groups with per-file context (path age, volume), auto-select rules (keep newest / oldest / shortest path / one per volume / protected folders never selected), space-reclaim preview, and bulk delete-to-trash — destructive actions always via OS trash + SRC-M16 undo. *(Inspired by: Czkawka review workflow)*

- [ ] **SRC-N24 — Hardlink & reflink awareness.** Detect same-inode/same-file-ID groups so hardlinked files are never reported as reclaimable duplicates; add `hardlink-count:` search, a linked-badge, and a "hardlink these duplicates" action on same-volume groups (with reflink/clonefile on APFS/Btrfs/XFS where supported). *(Inspired by: Everything 1.5 hard-link tracking; Czkawka)*

- [ ] **SRC-N25 — Similar-images finder.** Perceptual near-duplicate grouping via DCT dHash/pHash (permissive `image_hasher`) with a similarity threshold slider and side-by-side compare — classic transform hashing, not ML. Distinct from the deferred X-05 palette lens (dominant-color search), which remains a separate spec. *(Inspired by: Czkawka similar-images)*

- [ ] **SRC-N26 — Empty & junk sweeper.** A guided cleanup workspace combining SRC-M08 emptiness data with new detectors: zero-byte files, empty dirs (roots-of-empty view), broken shortcuts/symlinks (from SRC-N11), orphaned sidecars (`.aae`, `.thm`, `.xmp` without their master), and per-OS temp-pattern presets — review list first, trash-only deletes, undo always. *(Inspired by: Czkawka)*

- [ ] **SRC-N27 — Disk-usage treemap.** A WizTree-style treemap view fed by the *existing* folder-size index (§8.11 already indexes folder sizes — the delta is the visualization): click-to-zoom subtrees, hover details, top-100 largest files, and per-extension space breakdown, rendered on the UI's canvas layer. *(Inspired by: WizTree / WizFile)*

- [ ] **SRC-N28 — Trash/Recycle-Bin lens.** Search inside the OS trash with original-path and deleted-date columns (parsing `$Recycle.Bin` `$I` records, macOS `.DS_Store`-independent trashinfo, XDG `trashinfo`), plus one-click restore-to-original-path. Today `$Recycle.Bin` is just a recommended exclude. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N29 — Cloud-placeholder awareness.** Detect OneDrive/iCloud/Dropbox placeholder ("online-only") files via OS attributes, add a `placeholder:` modifier and cloud badge, and — critically — guarantee extractors and hash tiers *never* hydrate a placeholder (skip + report instead), preserving the zero-outbound posture. *(Freally-unique — none of the researched apps have this)*

### Tags, notes & collections

- [ ] **SRC-N30 — Tag system core.** Local tags (SQLite store keyed by stable FileId, surviving renames via the journal): `tag:` modifier, colored tag chips in rows, keyboard tagging (T on selection), bulk tag/untag on result sets, and tag autocomplete — no sidecar files unless the user opts into export. *(Inspired by: TagSpaces; Tabbles)*

- [ ] **SRC-N31 — OS-native label interop.** Read and search macOS Finder tags/labels and Linux `user.xdg.tags` xattrs as first-class tags; optional write-back keeps Finder and Freally in sync. Windows: read-only NTFS-ADS opt-in noted as a documented exception (no OS-native tag standard). *(Inspired by: HoudahSpot Finder-tag search; TagSpaces)*

- [ ] **SRC-N32 — Auto-tag rules & tag inheritance.** Rules engine: "everything matching *query* gets tag X" (evaluated on journal events, so new files auto-tag), plus folder-inherited tags with per-rule enable/disable and a dry-run preview. *(Inspired by: Tabbles auto-tagging)*

- [ ] **SRC-N33 — Per-file notes.** A markdown note attached to any FileId, edited in the preview pane, indexed into the content lens (`note:` modifier scopes to notes), surviving renames/moves; export/import as a JSON bundle for backup. *(Inspired by: Tabbles comments)*

- [ ] **SRC-N34 — Collections (result basket).** Pin arbitrary files from *any* query into named static collections; a floating basket accepts drag-drops across searches; act on a whole collection at once (open, export, tag, hand to bulk-rename) — the missing "gather then act" primitive. *(Inspired by: Alfred file buffer)*

- [ ] **SRC-N35 — Saved-search dashboard.** An optional start-page of saved-search tiles showing live counts and a small since-yesterday delta sparkline (computed locally from index queries on open — no background network, no polling when closed); click runs the search. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N36 — Conditional row color rules.** User rules mapping a query to a row color/badge ("`ext:tmp` → grey", "`tag:client-acme` → violet dot"), evaluated per-row at render. Delta versus §8.8 Fonts & Colors: that styles selection *states* and lens groups; this styles rows by *content-matching rules*. *(Inspired by: Everything 1.5 filter colors)*

### Power query — macros, snapshots, monitors & hashes

- [ ] **SRC-N37 — Parameterized query macros.** A macro editor with named parameters and prompt-on-run: `bigmedia!` expands to `size:>%size% (ext:mp4;mkv;flac)` and asks for `%size%` with a typed input. Delta versus existing custom filters (whose macro field is static Everything-parity): parameters, prompts, and a dedicated editor with test-run. *(Inspired by: Everything filter macros — extended)*

- [ ] **SRC-N38 — Search preprocessor rules.** User-defined rewrite rules applied before parsing — `img` → `ext:png;jpg;webp;avif`, `~docs` → `path:~/Documents` — with an ordered rule list, per-rule enable, and a live "what will actually run" preview under the search bar. *(Inspired by: Everything 1.5 search preprocessor)*

- [ ] **SRC-N39 — Weighted ranking controls.** Optional result-ordering boosts layered over sorts: boost by extension set, path prefix, recency, and run-count, with per-lens weight sliders and a "why is this first?" rank-explain popover. Off by default to preserve deterministic Everything-style ordering. *(Inspired by: Everything 1.5 weighted searches)*

- [ ] **SRC-N40 — Index snapshots & as-of queries.** Opt-in journal retention with periodic compacted manifests enabling `asof:2026-06-01` queries ("what existed then") within a user-set retention/disk budget — the index becomes a time machine for *names and metadata* (never file contents). Delta versus the existing startup journal replay: replay catches up the present; this retains the past. *(Inspired by: Everything 1.5 index journal — extended into time-travel)*

- [ ] **SRC-N41 — Snapshot diff & folder history view.** Pick a folder + two points in time → tree diff (added/removed/renamed/size-changed) with drill-down, answering "what did this folder look like last week" and "what changed since the backup". Builds on SRC-N40. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N42 — File-change monitor rules.** Saved query + trigger (appears/changes/disappears) → local OS notification and an activity log, with per-rule throttling/quiet hours; evaluated inside the daemon off the journal stream — zero network, zero polling cost. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N43 — Hash lens (find by checksum).** On-demand blake3/SHA-256/MD5 hashing with a hash column, `hash:`/`sha256:` search to find files *by* checksum, and journal-invalidated hash caching — verify a download, find the copy of a known file, dedupe against a reference. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N44 — Manifest verification.** Verify a folder or result set against `sha256sum`/`md5sum`/`.sfv`/BLAKE3 manifest files: report OK / changed / missing / extra with export, plus "generate manifest from result set" for the other direction. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N45 — Storage growth reports.** Per-folder size deltas between snapshots ("Downloads grew 18 GB this month"), rendered as a sortable report and as an overlay on the SRC-N27 treemap. Builds on SRC-N40. *(Inspired by: WizTree — extended with snapshot deltas)*

### Views & windows

- [ ] **SRC-N46 — Timeline view & activity heatmap.** Group results under day/week/month headers by modified/created/date-taken, with a calendar heatmap of file activity; click a day to filter. (The architecture doc reserves a render-canvas thread; the grouped timeline view itself is specified nowhere — this specifies it.) *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N47 — Media gallery mode.** A contact-sheet grid for image/video results with a continuous size slider and hover-scrub video thumbnails (keyframe strips, decoded locally). Delta versus the existing three fixed thumbnail sizes: continuous zoom, masonry layout, and scrub. *(Inspired by: Finder gallery view; Everything thumbnail views)*

- [ ] **SRC-N48 — Hex preview & binary inspector.** A hex/ASCII preview tab with magic-byte annotation ("PNG header", "ZIP local file header"), a strings-extraction sub-view, and goto-offset — for the files no text extractor claims. *(Inspired by: FileLocator Pro hex view)*

- [ ] **SRC-N49 — Split view & result compare.** Two result panes side-by-side running independent queries, with compare mode: highlight names present in A but not B (by name/size/hash), the instant "did everything copy over?" answer. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N50 — Detachable panes & named window layouts.** Tear the preview or a result pane into its own window; save/recall named layouts (window positions, columns, lens visibility, sidebar state) per task — "audio triage" vs "code hunt". *(Inspired by: Everything 1.5 multi-window + layouts)*

- [ ] **SRC-N51 — Result tabs.** Multiple query tabs per window (Ctrl+T) with pinned tabs that persist across restarts; drag a tab out to spawn a window. Delta versus today: multi-*window* exists, tabs do not. *(Inspired by: FileLocator Pro tabbed searches)*

- [ ] **SRC-N52 — Per-column quick filters.** Excel-style funnel on column headers: extension checklist, size/date range pickers, property value lists — composing into the query as chips so the DSL stays the single source of truth. *(Inspired by: UltraSearch column filters)*

- [ ] **SRC-N53 — Folder-tree drill-down panel.** An optional tree panel synchronized with results: select a branch to scope the query (`path:` chip), with per-branch result counts — browse-then-search for users who think spatially. *(Inspired by: Agent Ransack folder pane; UltraSearch)*

### Launcher & OS integration

- [ ] **SRC-N54 — Launcher mode.** A `>` palette prefix (and optional dedicated hotkey) that searches installed apps (Start Menu / `/Applications` / `.desktop`), recent documents, and Freally commands — run, reveal, or open-with; pure local enumeration, no web suggestions ever. *(Inspired by: Flow Launcher; PowerToys Run; Listary 6)*

- [ ] **SRC-N55 — Result action palette.** Tab on a result opens a ranked verb list (open-with, copy variants, compress, tag, add-to-collection, custom SRC-M06 commands) — chainable, keyboard-first, learnable by frequency (stored locally). *(Inspired by: Raycast actions; Fluent Search operations)*

- [ ] **SRC-N56 — File-dialog Quick Switch.** Inside standard Open/Save dialogs: a hotkey jumps the dialog to the folder of the active file-manager window, and typed text searches Freally with Enter injecting the picked path. Windows first-class (UIAutomation + dialog messages — user-space only, no drivers); macOS/Linux ship a clipboard-assisted fallback with the limitation documented as a per-OS exception. *(Inspired by: Listary 6 Quick Switch — its signature feature)*

- [ ] **SRC-N57 — File-manager collect-and-tag hooks.** New right-click verbs in Explorer/Finder/Nautilus-family menus: "Tag in Freally…", "Add to Freally collection", "Monitor this folder…". Delta versus §8.7: the existing hooks cover search-for-this and re-extract; these feed the tag/collection/monitor systems. *(Inspired by: Listary file-manager hooks)*

- [ ] **SRC-N58 — Browser omnibox & bookmarklet kit.** An OpenSearch descriptor + a tiny packaged WebExtension so `fr query` in the address bar hits the *existing* local HTTP server on 127.0.0.1 (loopback only, token-auth). Delta versus the existing browser-search page: keyword integration and a prebuilt extension, no new server surface. *(Inspired by: Everything HTTP server workflows)*

- [ ] **SRC-N59 — Clipboard path watcher.** Opt-in (default off): when a copied string looks like a path or filename, offer a dismissible "Search in Freally" toast; clipboard contents are never stored or logged. *(Inspired by: Listary find-as-you-type instincts)*

- [ ] **SRC-N60 — Quake-style peek overlay.** A minimal drop-down overlay (hotkey-summoned, esc-dismissed) with just the search bar and top results — for the "grab one file" flow where the full window is overkill; selection hands off to the main window. *(Inspired by: PowerToys Run; Fluent Search overlay)*

### CLI, TUI & automation

- [ ] **SRC-N61 — Interactive TUI (`freally tui`).** An fzf-style terminal picker over the daemon: live-narrowing list, preview pane (text head, hit context, metadata), multi-select, and print-paths/pipe-out — bringing the magic moment to SSH sessions and terminal diehards. *(Inspired by: fzf; ripgrep-all's fzf integration)*

- [ ] **SRC-N62 — Shell integration helpers.** `freally pick` (single-shot picker for command substitution), a `cdf` shell function (cd to picked folder), and documented recipes for bash/zsh/fish/PowerShell keybindings — installed by `freally shell-init <shell>`. *(Inspired by: fzf shell integration)*

- [ ] **SRC-N63 — CLI watch mode.** `freally watch '<query>' --ndjson` streams matching journal events (appear/change/delete) as NDJSON until interrupted — the composable building block for user scripts, sharing the SRC-N42 rules engine. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N64 — Scheduled exports.** Schedule a saved search to export (.efu/CSV/JSON/M3U) to a local path on a cadence, registered via OS-native schedulers (Task Scheduler / launchd / systemd timers) invoking the CLI — nightly library manifests, weekly cleanup lists, no daemon bloat. *(Inspired by: FileLocator Pro automation)*

- [ ] **SRC-N65 — Rule actions: run local script.** Extend monitor rules (SRC-N42) with an "execute local command" action (path + args templates), gated behind an explicit per-rule confirmation and a signed-settings warning banner — Hazel-class automation, entirely offline. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N66 — `freally doctor`.** One command that checks journal privileges (USN access, FDA, fanotify caps), service health, index integrity, disk headroom, and locale files, printing pass/fail with fix-it hints — the first thing support asks for. Delta versus the §8.24 diagnostics zip: active checks with remedies, not just log collection. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N67 — `freally explain`.** Print the parsed AST, optimizer decisions (selectivity reorder, lens routing), per-lens candidate counts, and stage timings for any query — the query-tuning companion to the existing `--parse-only`. *(Freally-unique — none of the researched apps have this)*

### LAN, catalogs & index pro

- [ ] **SRC-N68 — LAN index pairing (LAN-only, no internet).** Zero-config discovery of other Freally machines via mDNS, PIN-verified pairing with pinned self-signed certs, and a paired-machine scope in the volume dropdown. Delta versus the existing Tools → Connect to HTTPS API Endpoint: that requires manual URL+token entry; this adds discovery, mutual auth, and pinning — still never leaving the local network. *(Inspired by: Everything 1.5 Everything Server / ETP — modernized)*

- [ ] **SRC-N69 — Catalog manager pro.** The full removable-media story on SRC-M14: catalogs named per device (serial + label), user notes, a catalog browser with per-catalog stats, export/import of catalog bundles, retention of thumbnails/content snippets per catalog budget, and merge/forget operations. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N70 — Offline cache for network folders.** Keep the last-known listing of indexed network folders searchable while disconnected (offline badge, like catalogs), then reconcile on reconnect via targeted rescan. Delta versus existing network-folder indexing: today a dropped share vanishes; this retains and reconciles. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N71 — Index health dashboard pro.** Historical charts for event lag, queue depth, and extraction backlog; a dropped-event ledger with root-cause hints; and per-volume rebuild recommendations with predicted duration. Delta versus SRC-M13 v1: history, trends, and forecasting rather than point-in-time numbers. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N72 — Index encryption at rest.** Opt-in XChaCha20-Poly1305 encryption of the name index, blobs, and SQLite store, key held in the OS keychain (DPAPI / Keychain / Secret Service) — for shared machines and stolen-laptop threat models, with the perf cost measured and displayed before enabling. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N73 — ADS / xattr lens.** Enumerate NTFS alternate data streams, POSIX xattrs, and macOS resource forks: `ads:`, `ads-count:`, `xattr:` modifiers, a streams column, and stream-content peek in preview — "which files carry hidden streams" for security triage. *(Inspired by: Everything 1.5 alternate-data-stream functions)*

- [ ] **SRC-N74 — Provenance & quarantine lens.** Cross-platform "where did this file come from": Windows MOTW zone + referrer URL (Zone.Identifier), macOS quarantine flag + `com.apple.metadata:kMDItemWhereFroms`, Linux `user.xdg.origin.url` — `downloaded-from:` and `quarantined:` searches over data already on disk (no network involved). *(Inspired by: Everything 1.5 ADS search — extended to cross-OS provenance)*

- [ ] **SRC-N75 — Owner & permissions search.** `owner:` search, `readable:`/`writable:` (for the current user), a world-writable finder, and a saved "files I can't read" report wired to the SRC-M21 permission health flow — DFIR-persona catnip. *(Freally-unique — none of the researched apps have this)*

### Code & knowledge lenses

- [ ] **SRC-N76 — Symbol lens.** Index tree-sitter *declarations* (functions/classes/structs/methods) from the existing 32-language extractor into a `symbol:`/`func:`/`class:` search with go-to-line open in the user's editor. Distinct from the deferred X-04 AST-*pattern* lens spec: this is name-level declaration lookup only, no structural queries. *(Freally-unique — none of the researched apps have this; distinct from the deferred X-04 spec)*

- [ ] **SRC-N77 — TODO/FIXME lens.** Structured extraction of `TODO`/`FIXME`/`HACK`/`BUG`/`NOTE` markers (with `(author)` captures) from the code extractor's comment stream: `todo:` search, per-file marker counts, and a project-wide markers report. Delta versus today: comments are indexed as flat text; markers are not structured or countable. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N78 — Search-and-replace across results.** grepWin-class multi-file regex replace scoped to a content-lens result set: capture groups, per-file diff preview, opt-out checkboxes per file, automatic `.bak` backups, and integration with SRC-M16 undo. *(Inspired by: grepWin)*

- [ ] **SRC-N79 — Code-aware hit context.** Syntax-highlighted hit-in-context view (reusing tree-sitter highlighting) with a symbol breadcrumb ("in `fn parse_modifier`") above each match — layered on SRC-M01's viewer. *(Inspired by: ripgrep/bat-style preview)*

- [ ] **SRC-N80 — Frontmatter & sidecar metadata.** Parse YAML frontmatter in Markdown (and TOML in some static-site formats) into queryable properties (`fm.status:done`, `fm.tags:`), covering the Obsidian/Zettelkasten crowd without any app-specific integration. *(Freally-unique — none of the researched apps have this)*

- [ ] **SRC-N81 — Gitignore-aware filtering & repo metadata.** A `gitignored:` modifier and a toggle to respect `.gitignore` in results (semantics via the permissive `ignore` crate), plus repo-root badges and a current-branch column read from `.git/HEAD` via gitoxide — read-only, no libgit2. *(Inspired by: ripgrep / fd `.gitignore` semantics)*

- [ ] **SRC-N82 — Rich-document extractor expansion.** New content-lens formats: `.ipynb` (cell-aware, code+markdown+outputs), `.rtf`, OpenDocument (`.odt`/`.ods`/`.odp`), and `.epub` — closing the format gap against the open-source full-text veterans. Delta versus the existing extractor set (txt/md, PDF, OOXML, code, archives, JSON/CSV/YAML): these four families are absent today. *(Inspired by: DocFetcher / Recoll format breadth)*

---

## Post-stable phases

Every Nice-to-Have above, sequenced. Phases continue the existing numbering (Phase 14 = launch), each sized 5–10 features. Order rationale: visible differentiation first (properties, audio, dupes), then organization and power-query, then surfaces, then integration/pro. Per the versioned build plan, each phase ships as exactly one build — one release, one minor bump (Builds 4–13 = v0.24.0 … v0.33.0) — and the last phase, Phase 25, ships as **v1.0.0**.

### Phase 15 — Property Lens I: Images & Video *(post-stable wave 1)* — Build 4 · v0.24.0
Features: **SRC-N01** (property-lens framework), **SRC-N02** (image dimensions), **SRC-N03** (EXIF capture properties), **SRC-N04** (image format internals), **SRC-N05** (video technical properties), **SRC-N06** (property columns & auto-layouts), **SRC-N07** (result-set statistics popover). — 7 features.

### Definition of Done — Phase 15
1. All 7 features implemented and behaviorally identical on Windows, macOS, and Linux (no per-OS exceptions expected in this phase; any that emerge are documented in `docs/PLATFORM-NOTES.md`).
2. Sub-16 ms filename-lens latency budget preserved: Criterion regression bench green (P50 ≤ 8 ms / P99 ≤ 16 ms on the 5M-file fixture) with the property store attached.
3. Budgets stated and met: property store ≤ 120 MB per 1M media files on disk; property extraction adds ≤ 128 MB peak RSS to the daemon; extraction stays lower-priority than journal indexing (search never blocks).
4. Property extraction is lazy-by-default and journal-invalidated; a Modify event re-extracts within 5 s in the integration test.
5. Unit + integration tests green in the 3-OS CI matrix, including corrupt/truncated JPEG/PNG/MP4/MKV fixture cases (no panics, typed errors only).
6. Manual smoke checklist executed on Win 11, macOS 14, Ubuntu 22.04 (query, sort, and column-add for each new property family).
7. Docs updated: PRD property-lens section, QUERY_DSL_REFERENCE, USER_GUIDE, CHANGELOG entry, and all 18 locale files gain the new keys (`i18n-lint` green; Arabic RTL verified on the new panels).
8. Zero-outbound-calls audit passes (no new network surface; EXIF GPS data never leaves the machine).
9. `cargo-deny` license policy passes with every new parser crate enumerated in THIRD-PARTY-NOTICES.md.
10. Settings defaults sane: property lens ships enabled with lazy extraction; per-property index toggles default to the documented set; Restore Defaults covered by wiring tests.

### Phase 16 — Property Lens II: Documents, Binaries & Packages *(wave 1)* — Build 5 · v0.25.0
Features: **SRC-N08** (document properties), **SRC-N09** (executable/binary properties), **SRC-N10** (installer/package properties), **SRC-N11** (shortcut & link resolution), **SRC-N12** (archive properties), **SRC-N13** (font metadata), **SRC-N14** (email extractor), **SRC-N15** (SQLite peek). — 8 features.

### Definition of Done — Phase 16
1. All 8 features implemented on Win + macOS + Linux, with the two approved per-OS exceptions documented: signature status is Win/macOS-only (SRC-N09) and MSI properties are parsed on every OS but signature-verified only on Windows (SRC-N10).
2. Filename-lens P99 ≤ 16 ms regression bench green with all Phase 15+16 property families indexed.
3. Budgets stated and met: combined Phase 16 property + email/SQLite content data ≤ 8 % of indexed payload (the existing global index-size ceiling holds); per-document extraction budgets (5 s / 256 MB) enforced by the existing sandbox.
4. Hostile-input fuzz set green: malformed PE/Mach-O/ELF, zip-bomb MSI, oversized mbox, and adversarial SQLite files produce typed errors, never hangs or panics; `/security-review` findings addressed.
5. Unit + integration tests green in 3-OS CI; `broken-links:` correctness verified against fixture trees with dangling `.lnk`/`.desktop`/symlinks.
6. Manual smoke on the three reference OSes, including one real-world corpus each (docs folder, Applications/Program Files, a font library).
7. Docs + CHANGELOG + 18-locale keys updated; `i18n-lint` green; RTL verified.
8. Zero-outbound-calls audit passes (signature checks use OS-local trust stores only; explicitly no OCSP/CRL network fetches — offline revocation data only, documented).
9. `cargo-deny` passes; new crates (`msi`, `lnk`, `ttf-parser`, mail/mbox parser) in THIRD-PARTY-NOTICES.md.
10. Settings defaults sane: email body indexing and SQLite row peek default **off** (opt-in), properties default lazy; wiring tests cover the new toggles.

### Phase 17 — Music Tags & Audio Pro *(wave 1)* — Build 6 · v0.26.0
Features: **SRC-N16** (music tag properties), **SRC-N17** (embedded artwork), **SRC-N18** (cue/playlist awareness), **SRC-N19** (similar-audio finder), **SRC-N20** (loudness compliance reports), **SRC-N21** (audio QC flags). — 6 features.

### Definition of Done — Phase 17
1. All 6 features implemented on Win + macOS + Linux (tag parsing is OS-agnostic; no exceptions expected).
2. Filename-lens P99 ≤ 16 ms regression bench green; audio-lens query P99 budget (≤ 30 ms on 100k tagged files) stated in the PRD and met.
3. Budgets stated and met: tag + fingerprint data ≤ 40 MB per 100k audio files; fingerprinting runs in the existing low-priority audio thread pool with no search-latency regression while a library scans.
4. Similar-audio recall gate: ≥ 90 % grouping of a fixture set of re-encoded pairs (FLAC↔MP3↔Opus-container variants) with ≤ 1 % false-merge rate.
5. Unit + integration tests green in 3-OS CI, including ID3v2.2/2.3/2.4, Vorbis, and MP4-atom tag fixtures plus malformed-tag fuzz cases.
6. Manual smoke on 3 OSes with a real music library ≥ 10k tracks: `artist:` + `lufs:` composed queries, artwork thumbnails, a compliance report export, and one `.cue` virtual-track search.
7. Docs (QUERY_DSL_REFERENCE audio section, USER_GUIDE "for audio engineers") + CHANGELOG + 18 locale key sets updated; `i18n-lint` green.
8. Zero-outbound-calls audit passes — explicitly: fingerprints never leave the machine, no AcoustID/MusicBrainz lookups exist in the binary.
9. `cargo-deny` passes; `lofty` and `rusty-chromaprint` licensing verified and listed.
10. Settings defaults sane: tag properties on/lazy, fingerprinting off by default (opt-in per-volume), QC detectors on; defaults covered by wiring tests.

### Phase 18 — Duplicates & Disk Hygiene *(wave 1)* — Build 7 · v0.27.0
Features: **SRC-N22** (dupe tiers 2–3), **SRC-N23** (duplicate review center), **SRC-N24** (hardlink/reflink awareness), **SRC-N25** (similar-images finder), **SRC-N26** (empty & junk sweeper), **SRC-N27** (disk-usage treemap), **SRC-N28** (trash lens), **SRC-N29** (cloud-placeholder awareness). — 8 features.

### Definition of Done — Phase 18
1. All 8 features implemented on Win + macOS + Linux; per-OS trash formats and reflink support matrices documented (reflink action hidden on filesystems without support, per the parity contract).
2. Filename-lens P99 ≤ 16 ms regression bench green while a full-hash dupe scan runs in the background (back-pressure verified).
3. Budgets stated and met: hash cache ≤ 32 bytes/file + invalidation metadata; treemap renders a 5M-node volume in ≤ 1.5 s from the existing folder-size index; dupe scan of 1M files (tier 2) completes within the documented reference-machine envelope.
4. Safety gates verified by tests: deletes go to OS trash only, protected-folder rules block auto-select, placeholders are never hydrated by hashing/extraction (integration test with mock placeholder attributes), and every destructive batch is undoable via SRC-M16.
5. Similar-images gate: ≥ 95 % grouping on the resized/re-encoded fixture corpus at default threshold with ≤ 1 % false-merge.
6. Unit + integration tests green in 3-OS CI, including `$I` record, XDG trashinfo, and hardlink/inode fixture suites.
7. Manual smoke on 3 OSes: full dupe review → reclaim flow on a seeded 100-group corpus, treemap drill-down, trash restore round-trip.
8. Docs + CHANGELOG + 18-locale keys updated (`i18n-lint` green); treemap and review center pass the accessibility keyboard-only audit.
9. Zero-outbound-calls audit and `cargo-deny` policy both pass (`image_hasher` listed).
10. Settings defaults sane: hygiene tools are on-demand (no background scans by default); placeholder-skip is force-on and not user-disableable.

### Phase 19 — Tags, Notes & Collections *(wave 2)* — Build 8 · v0.28.0
Features: **SRC-N30** (tag system core), **SRC-N31** (OS-native label interop), **SRC-N32** (auto-tag rules & inheritance), **SRC-N33** (per-file notes), **SRC-N34** (collections/result basket), **SRC-N35** (saved-search dashboard), **SRC-N36** (conditional row color rules). — 7 features.

### Definition of Done — Phase 19
1. All 7 features implemented on Win + macOS + Linux; the one documented exception (Windows has no OS-native tag standard — ADS write-back is opt-in and clearly labeled) is in PLATFORM-NOTES.
2. Filename-lens P99 ≤ 16 ms regression bench green with `tag:` predicates in the plan cache; tag-filtered queries meet a stated P99 ≤ 20 ms on 1M files with 100k tagged.
3. Budgets stated and met: tag/note store ≤ 50 MB per 100k tagged files; rule evaluation adds ≤ 1 ms per journal event at 10 active rules.
4. Durability tests green: tags and notes survive rename/move (FileId stability), daemon kill -9, and export/import round-trip; Finder-tag two-way sync verified on APFS.
5. Unit + integration tests green in 3-OS CI, including auto-tag rule dry-run correctness and inheritance conflict cases.
6. Manual smoke on 3 OSes: keyboard tagging flow, a 3-rule auto-tag setup, a collection gathered across 3 queries then bulk-renamed, dashboard tiles live-updating.
7. Docs + CHANGELOG + 18-locale keys updated; `i18n-lint` green; RTL verified on the new sidebar/dashboard surfaces.
8. Zero-outbound-calls audit passes (dashboard counts computed on open, no pollers).
9. `cargo-deny` passes; no new native deps beyond xattr access.
10. Settings defaults sane: tag system on, label write-back off, clipboard-free defaults; Restore Defaults wiring tests cover every new toggle.

### Phase 20 — Power Query: Macros, Snapshots, Monitors & Hashes *(wave 2)* — Build 9 · v0.29.0
Features: **SRC-N37** (parameterized macros), **SRC-N38** (search preprocessor), **SRC-N39** (weighted ranking), **SRC-N40** (index snapshots & as-of queries), **SRC-N41** (snapshot diff view), **SRC-N42** (file-change monitor rules), **SRC-N43** (hash lens), **SRC-N44** (manifest verification), **SRC-N45** (storage growth reports). — 9 features.

### Definition of Done — Phase 20
1. All 9 features implemented on Win + macOS + Linux (notification delivery uses each OS's native toast API; no exceptions).
2. Filename-lens P99 ≤ 16 ms regression bench green with snapshot retention enabled; `asof:` queries meet a stated P99 ≤ 100 ms on the 5M fixture.
3. Budgets stated and met: snapshot retention respects the user-set disk budget (default 2 % of volume, hard-capped) with automatic pruning verified; hash cache invalidation correctness proven under journal churn.
4. Standing Rule #8 upheld: every query that parsed before Phase 20 still parses; preprocessor and macros are additive (300+ voidtools + 200+ Freally-DSL regression fixtures green, extended with 50 new macro/preprocessor cases).
5. Monitor rules fire within 2 s of a matching journal event in integration tests, with throttling verified; zero missed events across a 10k-event storm fixture.
6. Unit + integration tests green in 3-OS CI, including as-of/diff correctness against a scripted filesystem history.
7. Manual smoke on 3 OSes: create macro with prompt, set a monitor rule and receive a native notification, verify a folder against a sha256sum manifest, run a growth report.
8. Docs + CHANGELOG + 18-locale keys updated; QUERY_DSL_REFERENCE gains `asof:`/`hash:`/macro chapters; `i18n-lint` green.
9. Zero-outbound-calls audit and `cargo-deny` both pass; snapshots explicitly exclude file contents (metadata only) per the privacy review.
10. Settings defaults sane: snapshots off by default (opt-in with disk-budget picker), ranking boosts off, monitors persist across restarts only when enabled.

### Phase 21 — Views & Windows *(wave 2)* — Build 10 · v0.30.0
Features: **SRC-N46** (timeline & heatmap), **SRC-N47** (media gallery mode), **SRC-N48** (hex preview), **SRC-N49** (split view & compare), **SRC-N50** (detachable panes & layouts), **SRC-N51** (result tabs), **SRC-N52** (per-column quick filters), **SRC-N53** (folder-tree panel). — 8 features.

### Definition of Done — Phase 21
1. All 8 features implemented on Win + macOS + Linux with per-OS window-manager quirks handled (tiling WMs on Linux, Stage Manager on macOS) or documented.
2. Filename-lens P99 ≤ 16 ms regression bench green; new views render their first frame ≤ 100 ms after query completion on the reference machines (stated gate).
3. Budgets stated and met: gallery hover-scrub decodes stay in the bounded thumbnail pool (UI RSS increase ≤ 150 MB at full gallery); hex preview maps ≤ 4 MiB windows regardless of file size.
4. Unit + integration + UI wiring tests green in 3-OS CI (tabs/layouts persistence, compare-mode set algebra, column-filter → DSL chip round-trip).
5. Keyboard-only operation verified for every new view (accessibility audit), including screen-reader labels on timeline groups and treemap-style canvases.
6. Manual smoke on 3 OSes: tear-off preview, save/recall two layouts, tab persistence across restart, a split-view A-not-B compare on a copy job.
7. Docs + CHANGELOG + 18-locale keys updated; `i18n-lint` green; RTL flip verified on timeline and gallery.
8. Zero-outbound-calls audit passes (video scrub thumbnails decoded locally).
9. `cargo-deny` passes; no new licensing surface beyond existing media crates.
10. Settings defaults sane: Details view remains default; new views are discoverable via the View menu with defaults that match Everything muscle memory.

### Phase 22 — Launcher & OS Integration *(wave 3)* — Build 11 · v0.31.0
Features: **SRC-N54** (launcher mode), **SRC-N55** (result action palette), **SRC-N56** (file-dialog Quick Switch), **SRC-N57** (collect-and-tag hooks), **SRC-N58** (browser omnibox kit), **SRC-N59** (clipboard path watcher), **SRC-N60** (quake-style overlay). — 7 features.

### Definition of Done — Phase 22
1. All 7 features implemented on Win + macOS + Linux, with the documented exception ledger explicit: SRC-N56 is Windows-first-class with clipboard-assisted fallback on macOS/Linux (limitation text shipped in-app), per the parity contract's fallback clause.
2. Filename-lens P99 ≤ 16 ms regression bench green; overlay and launcher palettes hit a stated summon-to-interactive ≤ 150 ms gate.
3. Budgets stated and met: launcher app-enumeration cache ≤ 10 MB, refreshed on journal events for app dirs; no polling loops (CPU idle ≤ 0.5 % preserved).
4. Security/privacy review passed for the new OS hooks: UIAutomation usage, WebExtension loopback-only manifest, and clipboard watcher (opt-in, never persisted) each get a SECURITY.md row; `/security-review` findings addressed.
5. Unit + integration tests green in 3-OS CI; file-dialog injection tested against the native Open/Save dialogs of three common apps per OS.
6. Manual smoke on 3 OSes: launch an app via `>`, run an action chain (tag → collection → export), Quick Switch a save dialog (Win), omnibox search from the default browser.
7. Docs + CHANGELOG + 18-locale keys updated; `i18n-lint` green.
8. Zero-outbound-calls audit passes — the WebExtension talks to 127.0.0.1 only and is store-review-ready without any remote endpoints.
9. `cargo-deny` passes.
10. Settings defaults sane: clipboard watcher **off**, overlay hotkey unbound by default (offered in first-run and settings), file-dialog hook off until enabled with an explanation screen.

### Phase 23 — CLI, TUI & Automation *(wave 3)* — Build 12 · v0.32.0
Features: **SRC-N61** (interactive TUI), **SRC-N62** (shell integration helpers), **SRC-N63** (CLI watch mode), **SRC-N64** (scheduled exports), **SRC-N65** (rule script actions), **SRC-N66** (`freally doctor`), **SRC-N67** (`freally explain`). — 7 features.

### Definition of Done — Phase 23
1. All 7 features implemented on Win + macOS + Linux (TUI verified on Windows Terminal, iTerm2/Terminal.app, and three Linux terminal emulators; PowerShell + POSIX shells covered).
2. Filename-lens P99 ≤ 16 ms regression bench green; TUI keystroke-to-repaint ≤ 33 ms on the 5M fixture (stated gate).
3. Budgets stated and met: TUI client RSS ≤ 64 MB; `watch` mode backpressure verified against a 10k-events/s synthetic journal storm without daemon impact.
4. Script-action safety gates verified: per-rule explicit confirmation, command-line shown verbatim before first run, disabled in portable mode on untrusted machines by default; SECURITY.md updated.
5. Unit + integration tests green in 3-OS CI, including NDJSON schema snapshot tests (schema versioned and documented) and scheduler round-trips on Task Scheduler / launchd / systemd timers.
6. Manual smoke on 3 OSes: TUI pick → pipe to xargs, `cdf` helper, a nightly scheduled export firing, `doctor` catching a deliberately broken service, `explain` on a 3-lens query.
7. Docs + CHANGELOG updated including a new CLI_REFERENCE chapter; 18-locale keys updated for the few UI-side strings; `i18n-lint` green.
8. Zero-outbound-calls audit passes (schedulers invoke the local CLI only).
9. `cargo-deny` passes (TUI crate — e.g. ratatui, MIT — listed).
10. Settings defaults sane: watch/schedule/script features all opt-in; `doctor` and `explain` require no configuration.

### Phase 24 — LAN, Catalogs & Index Pro *(wave 3)* — Build 13 · v0.33.0
Features: **SRC-N68** (LAN index pairing), **SRC-N69** (catalog manager pro), **SRC-N70** (offline cache for network folders), **SRC-N71** (index health dashboard pro), **SRC-N72** (index encryption at rest), **SRC-N73** (ADS/xattr lens), **SRC-N74** (provenance & quarantine lens), **SRC-N75** (owner & permissions search). — 8 features.

### Definition of Done — Phase 24
1. All 8 features implemented on Win + macOS + Linux; per-OS metadata matrices (ADS vs xattr vs resource forks; MOTW vs quarantine vs origin-url) documented with the hide-or-fallback rule applied.
2. Filename-lens P99 ≤ 16 ms regression bench green — including with index encryption enabled (encrypted-index P99 ≤ 20 ms stated and met, and shown to the user before opt-in).
3. Budgets stated and met: catalogs ≤ 150 MB per cataloged TB at default retention; health-history store ring-buffered at ≤ 20 MB.
4. LAN pairing security review passed: mDNS advertisement is opt-in, PIN pairing with pinned certs verified against MITM tests on a lab network, remote scope is read-only query (no file download unless the existing allow-download toggle is on), and everything is clearly labeled **(LAN-only, no internet)**; `/security-review` findings addressed.
5. Unit + integration tests green in 3-OS CI, including unplug/replug catalog round-trips, share-loss reconcile, and keychain-loss recovery paths (index rebuilds rather than bricking).
6. Manual smoke on 3 OSes: pair two machines and search across, catalog a USB drive then find a file on it while unplugged, enable encryption and verify search + rebuild, run an ADS/provenance triage query.
7. Docs + CHANGELOG + 18-locale keys updated; SECURITY.md gains pairing, encryption, and provenance threat-model rows; `i18n-lint` green.
8. Zero-outbound-calls audit passes: pairing traffic never routes off-link (link-local verified), and the network-calls policy panel lists the LAN listener when enabled.
9. `cargo-deny` passes (mdns + AEAD crates listed).
10. Settings defaults sane: LAN sharing off, encryption off with an explicit trade-off screen, catalogs on-by-default for removable volumes with a first-eject consent prompt.

### Phase 25 — Code & Knowledge Lenses *(wave 3)* — Build 14 · v1.0.0
Features: **SRC-N76** (symbol lens), **SRC-N77** (TODO/FIXME lens), **SRC-N78** (search-and-replace across results), **SRC-N79** (code-aware hit context), **SRC-N80** (frontmatter metadata), **SRC-N81** (gitignore-aware filtering & repo metadata), **SRC-N82** (rich-document extractor expansion). — 7 features.

### Definition of Done — Phase 25
1. All 7 features implemented on Win + macOS + Linux (editor go-to-line handoff tested with VS Code, vim, and one JetBrains IDE per OS; graceful fallback to reveal).
2. Filename-lens P99 ≤ 16 ms regression bench green; `symbol:` queries meet a stated P99 ≤ 25 ms on a 100k-file monorepo fixture.
3. Budgets stated and met: symbol + marker index ≤ 15 % overhead on top of existing code-extractor output; `.ipynb`/EPUB extraction under the standard 5 s / 256 MB sandbox budgets.
4. Replace-safety gates verified by tests: dry-run diff is mandatory, backups written before write, cross-file undo restores byte-identical originals, files changed on disk mid-operation are skipped with a warning (mtime guard).
5. Unit + integration tests green in 3-OS CI, including tree-sitter grammar-version pinning tests so a grammar bump cannot silently change symbol extraction.
6. Manual smoke on 3 OSes: find a function by name and jump to line, run a scoped regex replace across 50 files and undo it, query `fm.status:` on an Obsidian vault, search inside an `.ipynb` and an `.epub`.
7. Docs + CHANGELOG + 18-locale keys updated; QUERY_DSL_REFERENCE gains symbol/todo/frontmatter chapters; `i18n-lint` green.
8. Zero-outbound-calls audit passes; gitoxide is verified to never touch remotes (read-only local object access).
9. `cargo-deny` passes (gitoxide MIT/Apache, `ignore` crate, EPUB/ODF parsers listed).
10. Settings defaults sane: replace feature gated behind an "enable write operations" master toggle (default off); symbol/todo lenses on with lazy extraction.

---

## Backlog — unscheduled UX ideas

_Parked 2026-07-18 (Mike) for future scheduling. These sit **outside** the counted Must-/Nice-to-Have sets and phase totals below — promote one into a numbered phase (and `product-roadmap.md`) when it's picked up._

- **Modal background-blur (IObit-style).** When any modal or dialog opens, softly blur and dim the rest of the app behind it while the dialog stays crisp on top, so attention lands on the active dialog and the window reads as layered and modern. Implement once in the shared modal shell so every dialog inherits it; honor reduced-transparency / reduced-motion preferences and ship both light- and dark-theme backdrops. A suite-wide consistency goal across the Freally apps.

## Coverage check

- **Must-Haves:** SRC-M01 … SRC-M24 → **24** features (within the 20–30 band), all stable-gate, all new versus the staged docs, deltas stated where they extend existing surfaces.
- **Nice-to-Haves:** SRC-N01 … SRC-N82 → **82** features (within the 60–85 band).
- **Grand total:** 24 + 82 = **106** features (within the required 85–120).
- **Post-stable phases:** **11** (Phase 15 → Phase 25), continuing the repo's existing phase numbering after Phase 14 (launch).
- **Phase sizing (5–10 each):** P15 = 7 · P16 = 8 · P17 = 6 · P18 = 8 · P19 = 7 · P20 = 9 · P21 = 8 · P22 = 7 · P23 = 7 · P24 = 8 · P25 = 7 → sums to 82. ✔
- **Every SRC-N appears in exactly one phase:** N01–N07 → P15; N08–N15 → P16; N16–N21 → P17; N22–N29 → P18; N30–N36 → P19; N37–N45 → P20; N46–N53 → P21; N54–N60 → P22; N61–N67 → P23; N68–N75 → P24; N76–N82 → P25. Contiguous, no gaps, no repeats. ✔
- **Builds (2026-07-19 amendment):** **14** builds total — 3 Must-Have (M01–M08 → B1, M09–M16 → B2, M17–M24 → B3) + 11 Nice-to-Have (one per phase, B4–B14) — so every Must- and Nice-to-Have feature lands in exactly one build. Contiguous, no gaps, no repeats. ✔
- **Build sizing (5–10 each):** B1 = 8 · B2 = 8 · B3 = 8 · B4 = 7 · B5 = 8 · B6 = 6 · B7 = 8 · B8 = 7 · B9 = 9 · B10 = 8 · B11 = 7 · B12 = 7 · B13 = 8 · B14 = 7 → 24 + 82 = **106**. ✔
- **Version ladder:** app is at v0.20.1 → one minor bump per build: v0.21.0 · v0.22.0 · v0.23.0 (**stable tag**, closes the Must-Have gate) · v0.24.0 … v0.33.0 · then the final build ships as **v1.0.0** exactly. 14 releases, strictly monotonic, one per build. Backlog items sit outside the builds. ✔
- **Definition of Done:** present after every phase, 10 verifiable items each, always covering: 3-OS implementation or documented exceptions; the sub-16 ms filename-lens regression bench; stated index-size/RAM budgets; green unit/integration tests; manual smoke on Win 11 / macOS 14 / Ubuntu 22.04; docs + CHANGELOG + 18-locale key updates with `i18n-lint`; the zero-outbound-calls audit; the `cargo-deny` license policy; and sane settings defaults. ✔
- **Constraints reaffirmed:** every feature above is **no-AI/ML** (parsing, hashing, DSP, and classic IR only — no embeddings, no OCR), **$0** to Mike and to users (permissive-license crates only, `cargo-deny` enforced, no services), **local-only** (the index never leaves the machine; the sole LAN feature, SRC-N68, is opt-in, mutually authenticated, and labeled *(LAN-only, no internet)*; SRC-N58 is loopback-only), and **solo-dev buildable** (no kernel drivers, no paid OS programs, user-space OS APIs only). ✔
