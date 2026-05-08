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
### Fixed (Phase 5 review pass)

- **[all platforms]** Phase 5 query parser now treats `!` after `)` as a prefix-NOT — `(a)!b` parses as `(a) AND !b` to match voidtools-Everything's documented behavior. Previously the byte-and-token boundary check omitted `RParen`, so the trailing `!b` collapsed into a single literal `!b` and the negation was silently dropped (Standing Rule #8 regression). New parser-test `bang_after_rparen_is_not` locks the contract in.
- **[all platforms]** Phase 5 plan cache no longer mutates the cached plan when `match_mode.match_path` is on. The seed-clear that lets a path-search bypass the trigram pre-filter now runs at execute-time only — the `ExecPlan` stays a pure function of the query string, so two concurrent callers with the same query but different `match_path` settings can no longer poison each other's cached plan. New wiring-test `plan_cache_survives_match_path_toggle` covers it.
- **[all platforms]** Phase 5 `parse_iso_day` rejects calendar-impossible days (Feb 30, Apr 31, non-leap Feb 29, …) up-front via a new `days_in_month` validator (Howard Hinnant's epoch-day arithmetic accepted any 1-31 day for any month, silently rolling overflow forward). Voidtools rejects these — Standing Rule #8 regression. New parser-test `invalid_calendar_days_reject` covers leap-year + month-end cases.
- **[all platforms]** Phase 5 modifier reservation list extended to cover the voidtools-Everything muscle-memory tokens (`wfn:`, `wholefilename:`, `case:`, `count:`, `dupe:`, `nodiacritics:`) so users typing them get a typed `QueryError::UnsupportedModifier` at execute time rather than a parse error. Parses-but-fails-loudly is the Standing Rule #8 contract until each of those toggles ships its lens-owning phase. New parser-test `voidtools_reserved_toggles_parse` covers the family.
- **[all platforms]** Phase 5 `eval_full` now lower-cases the candidate path once per row when `match_path` is on, instead of re-lower-casing for every AND / OR child node — the path-lower string is hoisted into `NameEvaluator::matches_full` and threaded through `eval_full` as `Option<&str>`. Cuts a per-AND-child `to_lowercase()` allocation that scaled with query depth.
- **[all platforms]** Phase 5 `eval_modifier::Reserved` now `debug_assert!`s when reached — `validate_supported` is the documented gate at the top of `execute()`, and a Reserved modifier reaching evaluation means a caller built a `Query` AST by hand and bypassed the gate. The previous silent-`false` arm is dead-code in the supported call paths and now fails loudly under `cfg(debug_assertions)`.

- **[all platforms]** Phase 5 filename lens (`sourcerer-query`) — voidtools-Everything-shaped DSL parser + executor over the Phase-4 index. `parse(s)` builds a `Query` AST covering literal substring / wildcard (`*`, `?`) / regex (`regex:` prefix) terms, boolean glue (`AND` / `OR` / `NOT` / `!` prefix, implicit-AND between adjacent atoms, parenthesised groups), and modifier predicates: `size:` (with `>`, `<`, `>=`, `<=`, `=`, `b`/`kb`/`mb`/`gb`/`tb` units), `date:` (relative aliases `today` / `yesterday` / `thisweek` / `lastweek` / `thismonth` / `lastmonth` / `thisyear` / `lastyear` plus absolute `YYYY-MM-DD` with comparator), `ext:` (single or `;`-separated list, `.`-stripped), `attrib:` (Windows-letter set `R`/`H`/`S`/`A`/`D`/`C`/`E`/`T`/`O`/`L`), `path:` / `parent:` / `child:` (substring matchers; `name:` / `folder:` aliases honoured). Quick-filter aliases `audio:` / `video:` / `image:` / `document:` / `executable:` / `archive:` expand to predefined extension sets. Future-lens modifiers (`content:` / `lufs:` / `codec:` / `channels:` / `samplerate:` / `length:` / `similar:` / `duration:` / `type:` / `lang:`) parse but are gated by `validate_supported`, surfacing `QueryError::UnsupportedModifier` until their owning phase ships. `execute(idx, query, ExecOpts)` plans the query (longest literal substring becomes the trigram seed; OR breaks the seed into a live-row scan), pulls candidates from the custom name index via the new `for_each_candidate_named` / `for_each_live` borrowed-bytes APIs, runs the name-side predicates, hydrates survivors via the new `Store::get_many` batched IN-clause fetch, evaluates the full-record predicates, applies `SortSpec` (name / path / size / date / type / ext, asc/desc), and streams results through `ResultSet::first_batch` / `collect`. `MatchMode` toggles (`match_case` / `whole_word` / `match_path` / `match_diacritics`) layer at execute time; `match_path` widens the search target to the canonicalised full path by skipping the name-index pre-filter (Phase-13 perf-pass note); `match_diacritics: false` strips combining marks via NFKD before substring comparison. 16-entry LRU `PlanCache` (`PlanCache::default16`) keys on the trimmed query string and reuses the parsed AST + plan on hot re-typing. New runtime deps (all permissive, deny.toml-allowlisted): `regex = "1"` (MIT/Apache-2.0), `unicode-normalization = "0.1"` (MIT/Apache-2.0). `crates/sourcerer-index` `name_index` swapped its trigram intersection from `BTreeSet` to a sorted-postings two-pointer merge (Build-Guide §`name_index` PERF note) and exposes `name_bytes` / `for_each_candidate_named` / `for_each_live` for the lens; `Store::get_many` chunks 250 ids per IN-clause to stay under SQLite's `SQLITE_MAX_VARIABLE_NUMBER`. `xtask gen-fixture` synthesises a deterministic SplitMix64 file-record stream for the Phase-5 perf bench (`cargo bench -p sourcerer-query --bench filename_lens`); the bench prints per-scenario P50 / P99 with FAIL markers and only exits non-zero when `SOURCERER_BENCH_GATE=1`. The `tests/voidtools_compat.rs` fixture pins 50 real Everything queries — Standing Rule #8 regression gate; `tests/wiring.rs` covers the executor end-to-end against a `tempfile`-backed index; `tests/smoke/phase_05_filename_lens.rs` is the OS-agnostic smoke that runs on every CI matrix entry.
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
