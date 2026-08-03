# Handoff — after Build 3 / v0.23.0 (stable)

**Written:** 2026-08-03, at the close of Build 3.
**State:** `v0.23.0` is tagged and merged to `main`. All 24 Must-Haves
(SRC-M01 … SRC-M24) are closed. Three-OS CI green on `main`.
**Not finished:** `docs/index.html` still needs promoting to 0.23.0 — see §5.2.

Read this alongside `docs/ROADMAP.md` (the task list) and
`Freally-Sourcerer-Feature-Roadmap.md` (the feature specs). This file
records only what Build 3 left open, and why — it is not a second
roadmap.

> **`docs/ROADMAP.md` is gitignored** (`.gitignore:75`) and therefore
> local-only — it is not in the repository and never has been. Edits to
> it survive on this machine and nowhere else, so a fresh clone starts
> without it and any session working from a clone will not see the task
> ticks. `Freally-Sourcerer-Feature-Roadmap.md` *is* tracked, and is
> where the checkbox state now lives for anyone else. Worth deciding
> whether that exclusion is still wanted; if the checklist is meant to
> be shared, remove the ignore rule and commit the file.

---

## 1. Bugs to fix first

### 1.1 `name^:` / `name$:` match differently depending on the rest of the query

**`crates/freally-query/src/exec.rs:1065-1066`.** The hydrated pass
evaluates the anchored modifiers as a bare `starts_with` /`ends_with`
over `to_lowercase()`:

```rust
ModifierKind::NamePrefix(needle) => row.name_lower.starts_with(&needle.to_lowercase()),
```

The name-index pass (`exec.rs:958-959`) routes the same modifiers
through `anchored_match`, which also strips diacritics and applies the
SRC-M23 ignore-punctuation / ignore-whitespace modes. So the two passes
disagree.

`needs_hydration` returns `false` for these variants, so a query using
one *alone* takes the name pass and behaves correctly. Add any modifier
that does force hydration and the whole tree switches to the other pass:

```
name^:café              → strips diacritics, matches `cafe-notes.txt`
name^:café size:>1mb    → does not strip, misses the same file
```

Same for `Ignore Punctuation` — `name^:foobar` finds `foo-bar.txt` on
its own and stops finding it once `size:` is added.

**Fix:** `eval_modifier` should call `anchored_match` with the row's
`name_lower` and the active `MatchMode`, exactly as `eval_name` does.
`eval_modifier` currently receives no `MatchMode` — check the call chain
from `eval_full`. While you are there, `ModifierKind::Child` at
`exec.rs:1064` has the same shape and the same latent inconsistency
(it predates Build 3; it became visible next to the new arms).

**Test to add:** extend `crates/freally-query/tests/wiring.rs` with a
case pairing an anchored modifier with `size:` and asserting the hit set
matches the un-paired query. That is the assertion that would have
caught this.

### 1.2 The `natural_sort` opt-out never reaches the daemon

`SortSpec.natural` (`crates/freally-query/src/opts.rs:79`) defaults to
`true` and **nothing ever sets it**. Grep confirms: the only
constructions are `SortSpec::default()` and one literal in `exec.rs`.

So Settings → Results → *Natural sort* turns off the client-side
comparator in `apps/freally-ui/src/lib/stores/sort.svelte.ts` and
nothing else. `freally search`, the CLI's `--json` output, and anything
else reading the daemon directly stay naturally ordered regardless of
the setting.

**Fix:** the channel already exists. SRC-M23 plumbed the whole
`search_opts` set through `query_run` → `QueryRunParams` →
`ExecOpts.match_mode` (`apps/freally-ui/src/lib/ipc/query.ts:24-45`,
`crates/freally-indexd/src/service.rs`). Send `natural_sort` the same
way into `ExecOpts.sort.natural`. Note the field-name trap that bit
SRC-M23: the settings key and the wire field must be mapped explicitly,
never spread — see the comment in `query.ts`.

Once the daemon honours it, consider whether
`lib/util/natural.ts` should stay at all. It exists because
`LensSection` re-sorts client-side; if that re-sort is what should be
authoritative, the Rust comparator is the one that is redundant. Pick
one to be the definition — today they are held together by a mirrored
test vector by hand.

---

## 2. Performance left on the table

None of these are user-visible faults today; all were found by the
`/simplify` efficiency pass and consciously deferred.

### 2.1 Ignore Punctuation / Whitespace full-scans the index

`crates/freally-query/src/exec.rs` — `use_seed` is false when
`MatchMode::rewrites_text()`, so the executor drops from the trigram
candidate set to `for_each_live`, walking every name in the index.

This is correct (the trigrams describe the *raw* name, which those modes
rewrite) and the modes are off by default, but a user who turns one on
converts sub-100 ms queries into whole-index walks.

**Fix:** index a punctuation/whitespace-stripped key alongside the raw
one and seed from the stripped needle when the mode is on. SRC-M12 did
exactly this for phonetic readings — see `crates/freally-index/src/phonetic.rs`
and the `U+0001` separator convention in the name index. It is
reindex-affecting, which is why it was deferred rather than rushed.

### 2.2 The needle is re-normalized per candidate row

`exec.rs` — `substring_match`, `anchored_match` and `literal_match` each
run `to_lowercase` → `strip_diacritics` → `strip_ignored` on the
**needle** inside the per-row matcher. The needle is invariant for the
whole query, so that is 1-3 throwaway `String`s plus a Unicode-category
walk per candidate row.

**Fix:** normalize each modifier's needle once when the plan is built
and hand the prepared form to the matcher. Only the target side needs
per-row work. This compounds with 2.1 — the full-scan path is exactly
where it costs most.

### 2.3 Three copies of the normalization ladder

Same three functions walk the identical case-fold → diacritic-strip →
`strip_ignored` sequence and differ only in the final operator. They
already disagree in ways nothing forces: `literal_match` is the only one
that consults `whole_word` or `match_case`. Extract one
`fn normalized(...) -> Cow<'_, str>` and make the three call sites thin.
Adding a sixth match-mode flag currently means editing four places, and
1.1 above is what happens when one is missed.

### 2.4 Smaller items

- `apps/freally-ui/src/lib/stores/recent_searches.svelte.ts` — every
  forward keystroke calls `settingsStore.patch`, which does
  `{ ...this.state, ...p }` on a ~180-key object and reassigns the
  `$state` root, invalidating every component that reads any setting.
  Hold `recent_searches` in its own small `$state` array instead.
- `crates/freally-indexd/src/permissions.rs` — `record` does a linear
  `entries.iter().any(...)` per call, from the scanner's error path.
  Bounded by `MAX_ENTRIES` (2 000) so it is not urgent; a `HashSet`
  beside the `Vec` removes it.
- `crates/freally-audio/src/peaks.rs` — a `u64` division per decoded
  frame (~8 M for a 3-minute stereo track). Track the next bucket
  boundary and increment instead.
- `apps/freally-ui/src/components/preview/MediaPlayer.svelte` — the
  `<audio>` and `<video>` branches hand-roll four state-syncing handlers
  each. Svelte's `bind:paused` / `bind:currentTime` / `bind:duration` /
  `bind:volume` do this for free and would delete ~30 lines and the
  `seekTo` helper.

---

## 3. Duplication worth collapsing

From the `/simplify` reuse and altitude passes. Each is a place where
the *next* change has to be made twice.

- **`ALLOWED_PATCH_KEYS`** (`apps/freally-ui/src-tauri/src/commands/settings.rs`)
  is exactly `phase_12_default_extras().keys()` plus the named struct
  fields. Two Rust lists that must agree by convention. Build it once
  into a `OnceLock<HashSet<String>>`. Adding a setting currently touches
  five places; this takes it to four, and the remaining four are real.
- **`open_log_file`** is byte-identical in `crates/freally-indexd/src/main.rs`
  and `apps/freally-ui/src-tauri/src/lib.rs`, differing only in the
  filename. Both crates depend on `freally-rpc`, which owns
  `portable::log_dir()` — a `portable::open_log(name)` beside it removes
  both copies.
- **`spawn_scan`** is called with the same three arguments at five sites
  in `crates/freally-indexd/src/service.rs` plus one in
  `windows_service.rs`. A private `spawn_scan(svc, root)` that pulls
  both handles off `svc.state` means a future third dependency is a
  one-line change, and a site that forgets `Some(permissions)` cannot
  silently lose permission reporting.
- **The `app_data_dir` fallback** is identical in `commands/settings.rs`
  and `commands/bookmarks.rs`. One `app_data_root(app)`.
- **The selected-hit lookup** now exists in three variants
  (`PreviewPane.svelte` twice, `quicklook.svelte.ts` once). A
  `resultsStore.hitById(id)` accessor would serve all three.
- **`MediaPlayer.svelte`** calls `invoke("files_open", …)` directly
  rather than `files.open()` from `lib/ipc/files.ts`. Every other call
  site uses the wrapper, and the raw call bypasses `setIpcMock`, so that
  path cannot be stubbed in unit tests.
- **No shared `Modal` component.** Seven dialogs plus the two Build 3
  additions each re-roll backdrop, panel, Escape handling and the
  `tabindex="-1"` a11y dance. Extracting one would also fix the six
  outstanding `a11y_interactive_supports_focus` warnings in one place.

---

## 4. Pre-existing problems this build surfaced but did not touch

Flagged rather than fixed, per the "don't improve adjacent code" rule.

- **`src-tauri` is not linted by CI.** `cargo clippy --workspace`
  excludes it (the workspace `exclude`s `apps/freally-ui/src-tauri`), so
  three clippy errors have been failing silently for some time:
  `log_event` is dead code, and `commands/icons.rs` has two
  `i32 as i32` casts. Adding a `cargo clippy` step for that manifest to
  `.github/workflows/ci.yml` is a small change with real value.
- **`log_event` does not work.** It is a `#[tauri::command]` documented
  as the TS→Rust debug bridge, but it is not in `invoke_handler`, so any
  TS call to it fails. Either register it or delete it.
- **`.gitmodules` and `vendor/freally-central/` are an untracked mess**
  in the working tree — a full clone sitting on top of five tracked
  files, plus a `.gitmodules` that names a submodule git does not have
  registered. Left alone in Build 3 because it predates it and touching
  it is not a code change. It should be resolved deliberately: either a
  real submodule, or vendored files, not both.

Still open from Build 2, unchanged:

- `auto_remove_offline` is a no-op and says so.
- Deletes have no undo — trash restore has no macOS implementation.
- `08-index-health` is `test.fixme`: the Playwright spec cannot drive
  the menu bar's one hover-only submenu. Six approaches failed. Build 3's
  new specs sidestep it by using the search-bar button and a top-level
  menu item instead. It would be fixed for real by making submenus
  click-to-pin, which would help trackpad users too.

---

## 5. Releasing — what bit this build, and will bit the next one

### 5.1 Tag with no changelog section = four failed jobs, zero artifacts

`.github/workflows/release.yml` extracts release notes from
`## [<version>]` in `docs/CHANGELOG.md` and **refuses to publish without
them**. Build 3's entries were written under `## [Unreleased]`, so
pushing `v0.23.0` failed all four platform jobs at
*"refusing to ship empty release notes"* before compiling anything.

Before tagging `v0.24.0`, rename the `## [Unreleased]` heading to
`## [0.24.0] — …` and open a fresh `## [Unreleased]` **below** it. The
extractor runs from the version heading to the next `## `, so anything
left underneath — the stale TASK-098 entry, for one — gets swallowed
into the published notes and the updater dialog if the boundary is
missing.

Because nothing had been published, the fix was to move the tag onto the
corrected commit rather than bump the version. That is only safe while
no release exists. Once artifacts are out, the repo's own rule applies:
a patch release must bump the version, because re-tagging reaches nobody
who already installed.

### 5.2 Promoting `docs/index.html` is a separate manual step, after the build

The site's download rows carry **real byte sizes**, read from the
published assets — v0.22.0's promotion commit says so explicitly, and
every size moved between 0.21.0 and 0.22.0, so carrying them over is
always wrong. So the order is:

1. Merge → tag → release workflow builds and publishes.
2. `gh release view v<version> --json assets` for the real sizes.
3. Edit `docs/index.html`: new `<article class="release latest">` block
   with seven installer links, demote the previous release to a short
   `<article class="release">` entry under `<!-- Previous releases -->`,
   and flip that version's row in the "Road to v1.0.0" `<ol
   class="timeline">` from `badge upcoming` / *Next* to `badge` /
   *Shipped &lt;date&gt; · Latest*.
4. Verify every link resolves before committing.

**This was not finished for v0.23.0.** At the time of writing the
release build (run `30780452254`) had `windows-x86_64` and
`macos-aarch64` green with the two remaining jobs still queued on runner
availability. If `docs/index.html` still shows `0.22.0` as latest, this
step is the outstanding one — and `docs/ROADMAP.md`'s "Build 3 release"
tick is ahead of reality until it is done.

### 5.3 The Windows Tantivy flake is real and will recur

`cargo test` on `windows-latest` intermittently fails with
`Tantivy("An IO error occurred: 'Access is denied. (os error 5)'")` —
Defender opening a freshly written segment while Tantivy renames it.
The CI workflow already excludes the workspace, `$RUNNER_TEMP` and the
process temp dir from real-time scanning (see the long comment on that
step; it has been hit and patched twice before), and it still happens.

It bit `wiring.rs:37` on the merge commit. The same code had passed on
the PR branch and passed again on the next `main` run, all three OSes.
**Re-run before investigating** — but if it becomes frequent rather than
occasional, the honest fix is fewer parallel Tantivy indexes in that
test file. Build 3 took `wiring.rs` from 18 tests to 27, each building
its own index, which is more concurrent pressure than it used to apply.

## 6. What comes next in the plan

`v0.23.0` closed the Must-Have gate. Per
`Freally-Sourcerer-Feature-Roadmap.md`, Build 4 is **v0.24.0 — Phase 15,
Property Lens I: Images & Video (SRC-N01 … SRC-N07)**, starting with
SRC-N01, the property-lens framework that the six after it plug into.

But note that `docs/ROADMAP.md` still carries unticked pre-stable work
that the build plan does not cover:

- **Phase 13** (TASK-100 … TASK-105): perf benches, per-OS bundling,
  daemon installer flow, auto-update wiring, distribution channels.
- **Phase 14** (TASK-106 … TASK-111): final polish, docs site, brand
  assets, launch artifacts.
- **The EULA acceptance gate** — marked REQUIRED before public release
  and not started.
- **The bug reporter (TASK-BR1) and check-for-updates (TASK-UP1)** — the
  Havoc standard; a panic hook exists with no submit path.
- **Playwright Gate 1 and Gate 2** — Gate 1 is "before v1.0/final
  stable", Gate 2 is a full workflow suite after stable.

Some of that is arguably now overdue, since "stable" has been tagged.
Worth an explicit decision about ordering before starting Build 4:
whether v0.23.0 is stable-the-feature-gate or stable-the-release, and if
the latter, whether the EULA gate and the bug reporter should land as
v0.23.1 before any Nice-to-Have work begins.

---

## 7. Verification commands

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p xtask -- i18n-lint
cargo deny check
cd apps/freally-ui && pnpm run check && pnpm run test:unit
cd apps/freally-ui/src-tauri && cargo check --all-targets   # not in CI — see §4
cd apps/freally-ui && pnpm build && PW_CHANNEL=msedge pnpm test:e2e
```

`PW_CHANNEL=msedge` is a local workaround for a broken Playwright
Chromium install on this machine; it is unset in CI. It is also better
fidelity — Tauri renders in WebView2 on Windows, which is Edge.
