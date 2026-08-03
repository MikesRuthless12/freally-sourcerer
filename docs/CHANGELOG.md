# Changelog — Freally

All notable changes documented here. Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versioning: [SemVer](https://semver.org).

---

## [Unreleased] — Build 3 · Must-Have stable gate, slice 3 of 3

The last of the three Must-Have builds: SRC-M17 … SRC-M24. Ships as
v0.23.0, the **stable** tag.

**SRC-M17 — portable mode.** `freally --portable`, or an empty
`portable.flag` file beside the binary, keeps the index, settings,
bookmarks, journal cursors and logs in a `Data/` folder next to the
executable, and stops the app registering anything with the OS. All
three binaries take the switch; the flag file is what makes a
double-clicked zip or AppImage portable with no command line at all.

The layout lives in `freally_rpc::portable` — the one crate the Tauri
shell, the daemon and the CLI all already depend on.

What portable mode deliberately does *not* do, each for a concrete
reason rather than tidiness:

- **No service, launchd plist, or systemd unit.** `freally-indexd
  install` / `uninstall` / `service` all refuse in portable mode. Each
  writes a registration that outlives the USB stick and points at a path
  that will not be there next boot. `uninstall` is refused for the
  opposite reason: a portable copy never registered anything, so letting
  it through would deregister the *installed* copy on the host.
- **No `freally://` scheme registration.** It writes a machine-level
  handler pointing at the executable's current path; unplug the stick
  and every `freally://` link on that machine dangles. The in-process
  listener still runs, so a portable instance handles URLs it is handed
  — it just does not claim the scheme.
- **No auto-updater.** The updater applies an *installer*. Pointing that
  at a stick would install the app onto the host machine, which is the
  one thing a portable user is avoiding. Portable installs update by
  replacing the folder.
- **No adopting the Windows service.** The service owns the installed
  index under `%PROGRAMDATA%`; a portable launch that connected to it
  would silently search the host's index and write the stick's activity
  into it.
- **No cross-instance `taskkill`.** Single-instance enforcement kills by
  image name. A stick plugged into a machine already running Freally
  would have taken down the app the user was in the middle of using.

Two pieces of state needed explicit redirection because they do not hang
off the index root:

- **Journal cursors.** Every OS subscriber defaults to its own per-user
  directory, so a portable install would have left cursors behind on
  every machine it was plugged into. They now go to `Data/cursors`.
- **The RPC socket, which deliberately stays off the stick.** A stick is
  usually FAT32 or exFAT — filesystems that cannot hold a Unix domain
  socket at all, and that have no permission bits for the 0600 +
  peer-uid check the transport depends on. It lives in the host's temp
  directory under a name derived from the portable root, so a portable
  instance never collides with an installed one or with a second stick,
  and the auth story is unchanged.

Logs move because a double-clicked binary has no console to inherit:
both processes write into `Data/logs/`, falling back to the console if
that file cannot be opened rather than losing the diagnostics that would
explain why.

On macOS `Data/` lands beside `Freally.app`, not inside it — writing
user data into a bundle breaks code signing and fails outright on a
read-only mount.

**Also in this change: the About panel reported the wrong version.** It
carried a hardcoded `0.19.84` and had drifted three releases behind what
shipped. It now reads the version from the running process via a new
`app_environment` command, which also reports whether this is a portable
install and the folder it writes to — so a user who plugs a stick into
someone else's machine can confirm in the app that nothing is landing on
the host.

Smoke: `tests/smoke/build_03_portable.rs` stages a copy of the real
daemon binary in a scratch directory (portable mode is defined relative
to the executable, so testing it in place would make every other binary
in the workspace portable for the rest of the run) and proves the flag
file, the switch, the three registration refusals, and that a portable
daemon writes its index and its log into `Data/` and nowhere else.

**SRC-M24 — natural sort.** Digit runs inside a name read as numbers, so
`file2` comes before `file10` and `v1.9` before `v1.10`. On by default,
with an opt-out at Settings → Results → *Natural sort*.

It applies to every string column — name, path, type and extension — not
just the name, so "sort naturally" does not quietly mean "sort the name
column naturally".

There are two comparators because there are two sorts: the daemon orders
what it returns (`crates/freally-query/src/natural.rs`) and the result
list re-sorts client-side (`apps/freally-ui/src/lib/util/natural.ts`).
They are deliberately not identical — the UI runs non-digit runs through
`localeCompare` so accents and case keep behaving as they already did,
while the daemon only ever sees a lowercased name — so a shared vector of
cases pins the numeric behaviour they must agree on, asserted from both
sides.

Details worth stating, because each is a way to get this wrong:

- **Digit runs are compared without being parsed.** Parsing caps at
  `u64` (or 2^53 in JS); a 30-digit run in a filename is unusual but not
  invalid, and two different overlong runs would then compare equal.
  Comparing significant-digit count and then the digits is exact at any
  length.
- **Zero padding is a tie-break of last resort.** `file07` and `file7`
  are the same number, so padding only decides when nothing else does —
  deciding on it the moment it appears would let `file07b` beat
  `file7a`.
- **The UI compares a non-digit stretch only as far as both sides have
  one.** A run can be cut short by a digit on one side and not the other
  — `img.png` against `img1.png` gives runs of 7 and 3 — and comparing
  those whole answers on length instead of on `.` against `1`, which
  sorted `img.png` last. Caught by the shared vector.
- **Digit-free strings still go through `localeCompare` whole**, so the
  common case keeps exactly the collation it had.
- **The path column is the one place the toggle changes more than
  digits.** With natural sort off it stays `PathBuf::cmp`, which orders
  by component rather than by byte; natural sort has to read the path as
  text to see digit runs at all.
- **Extensionless files still sort first**, as `Option::cmp` did.

The feature line asks for the ordering to be "honored by the fast-sort
indexes". There are none to honor: sorting happens in `sort_rows` over
the candidate rows the executor already materialized, and no
sort-accelerating index exists in the store today. Building one is
SRC-N-scale work, so this is stated rather than quietly skipped.

Smoke: `tests/smoke/build_03_natural_sort.rs` indexes names where byte
order and natural order disagree on every entry, and pins the default,
the opt-out, descending order, the path column, and that a numeric
column is left alone.

**SRC-M23 — ignore punctuation, ignore whitespace, and anchored
matching.** Two new toggles under `Search →` (`Ignore Punctuation`,
`Ignore Whitespace`) and two new DSL modifiers, `name^:report`
(starts with) and `name$:final` (ends with).

**The four existing match toggles now actually do something.** Through
Build 2, only `match_phonetic` crossed the wire: `Match Case`, `Match
Whole Word`, `Match Path` and `Match Diacritics` moved a checkmark and
changed nothing, because `query.run` sent only the one flag and the
daemon built `MatchMode` from `Default`. The executor had honored all
five the whole time. The full set is now sent and read.

**A related bug, found while wiring it: `Match CJK Phonetics` never
persisted.** `settings_set` re-serializes the settings struct and parses
the result back, and Rust's `SearchOpts` had no `match_phonetic` field —
so the flag was dropped on *every settings write*. Build 2 added it to
the TypeScript side only. All three new flags are on the Rust struct too.

Implementation notes:

- **Both sides are stripped, not just the target.** Stripping only the
  name would let `foobar` find `foo-bar` while `foo-bar` could not find
  `foobar` — that reads as a bug, not as a match mode.
- **The trigram seed is dropped when a mode rewrites text.** `foo-bar`
  indexes the trigrams `foo`, `oo-`, `o-b`, `-ba`, `bar`; the needle
  `foobar` asks for `oob` and `oba`, which that row does not have, so
  seeding would return *nothing at all*, silently. The executor falls
  back to a full scan, the same trade-off `match_path` already makes.
  The flags are off by default, so nobody pays for it without asking.
- **Whole-word is ignored while a strip mode is on**, because removing
  the separators removes the word boundaries that define it.
- **`name:foo-bar` behaves like the bare term `foo-bar`.** These are the
  same query written two ways, so the `name:`/`child:` path honors the
  strip modes too rather than only the literal-term path.
- **Punctuation is Unicode's definition, not an ASCII list**, so `’` and
  `–` are dropped alongside `'` and `-`.

For the anchored modifiers, `^` and `$` are opted in for exactly two
keys rather than being added to the general key charset — the same
mechanism the hyphenated keys use, and for the same Standing Rule #8
reason: widening the charset would turn `x^2:3` and `total$:5` from
literal terms into hard parse errors. Both are rejected under
`--strict-everything`, since voidtools' Everything has no anchored
syntax. `name$:` anchors on the whole name including the extension, so
`name$:report` does not match `report.txt`.

Registering a new modifier touches more than the parser, and missing any
one leaves it half-working: the AST variant, four planner/evaluator sites
in `exec.rs`, `optimizer::selectivity_rank` (an anchored match is
narrower than the substring it implies, so it ranks ahead of `child:`),
the strict-mode list in *two* places, `report.rs`'s duplicated key
charset — miss that one and the query runs correctly while the search bar
paints it as a literal — plus `modifier_name`, `ModifierDetail`, and the
advertised `MODIFIER_KEYS` table the CLI completions read.

That last one had a gap worth noting: `every_advertised_modifier_is_known_to_the_parser`
could not have caught a missing sigil key, because a key outside the
charset degrades to a *literal* rather than to `UnknownModifier`, so the
test passes either way. A positive assertion was added.

**SRC-M20 — regex builder and live tester.** A popover beside a new
regex toggle in the search bar: a pattern field, the engine's own
compile error, a Rust-flavour cheat sheet, and live highlighting of what
the pattern matches across the current top-50 result names, with a
"3 of 50 match" count. Committing the pattern rewrites the query as
`regex:<pattern>` and turns Enable Regex on, so the menu never disagrees
with what the query is doing.

The parse-error pill and the AST hover already validate a query you have
*written*; nothing helped you write the pattern. That is where the time
goes — you type something, run it, get nothing, and cannot tell whether
the pattern is wrong or the files simply are not there. The popover
shows both answers at once.

**Testing happens in Rust, on purpose.** The obvious implementation
validates against the webview's `RegExp`, and it would be wrong in
exactly the cases a person composing a pattern hits: the query executor
runs the `regex` crate, which has no backreferences and no lookaround,
treats `\d` as Unicode-aware, and takes `(?i)` inline rather than as a
trailing flag. A `RegExp`-backed builder accepts `(?<=foo)bar` happily
and then fails when the query runs. `regex_test` compiles with the same
engine, so "it matches here" means "it will match there" — there is a
unit test asserting lookaround is rejected, for precisely this reason.

Smaller decisions, each guarding something real: spans are returned as
**character** offsets, since the UI slices them with JavaScript string
indices and a byte offset would cut a multi-byte name in half; zero-width
matches are dropped, because `a*` matches at every position and
highlighting nothing at every position is noise; an empty pattern reports
no error at all rather than flashing one at the first keystroke; and the
compiled-automaton size limit is pinned at 256 KB rather than the crate's
10 MB default. The engine is finite-automaton based, so a hostile pattern
cannot backtrack exponentially — the size limit is the only lever that
matters, and it is set.

**SRC-M19 — Spacebar Quick Look.** Space on a result opens a large modal
preview; the arrow keys walk the result set with it still open, and
Space or Escape closes it. The header shows "7 of 50" so you know where
you are in the set.

It is not a second preview implementation — it calls the same
`files_preview` the docked pane does, so any format that renders in one
renders in the other. What it adds is size and a keyboard flow: the
docked pane is a panel you glance at, this is the "what *is* this file?"
gesture, and the point is that your hands never leave the keyboard.

**Space is handled in the global key binder, not in the component**, and
it is the delicate one: Space is also a character, so it is ignored
whenever the caret is in an input, textarea, select, or contenteditable.
Getting that wrong means the search box stops accepting spaces — a far
worse bug than a missing shortcut.

Navigation walks the order the list actually *renders* (batches in lens
order, sort store applied within each, except duplicate-cluster batches
where the daemon already ordered the rows and re-sorting would break the
clusters apart). Walking any other order makes the arrow keys appear to
jump around. Stepping clamps at both ends rather than wrapping, because
wrapping from the last row to the first reads as a glitch when the key
is held down, and it replaces the selection rather than extending it.

**Deviation from the feature line, stated rather than glossed.** It asks
for native QuickLook on macOS and Freally's own host elsewhere. `qlmanage
-p` opens a *separate OS window* that Freally's key handling cannot
drive, so making it the macOS default would break the arrow-key flow that
is the whole feature. The in-app modal is therefore the surface on all
three platforms — which is also what makes the behaviour testable and
identical everywhere — and macOS additionally gets an explicit "Open in
macOS Quick Look" button for formats the system can render and Freally's
preview host cannot. The path goes through the same `KnownPaths` gate as
every other file command and is passed as a `Command` argument, never
through a shell.

**SRC-M22 — bookmarks and filters sidebar.** An optional left column
(View → Sidebar, off by default) with four sections: bookmarks with
drag-reorder, the seven type filters, indexed volumes, and recent
searches. All four already existed behind three different dropdowns and
a menu, so scoping a search meant knowing which of four places to look;
this puts them in one column of clickable nodes.

**"Recent searches" needed inventing, because search here is live.**
`run()` fires on every keystroke, so recording each run verbatim would
produce `r`, `re`, `rep`, `repo`, `repor`, `report` and bury the one
entry anybody wants. Two rules fix it without an explicit commit gesture
the UI does not have: a new query evicts any existing entry that is a
prefix of it (the keystrokes on the way there), and a query that is
itself a prefix of the newest entry is not recorded (that is a
backspace). Typing "report" then "invoice" leaves exactly
`["invoice", "report"]`.

The list honours **Privacy Mode and the Search History toggle** — both
already mean "do not keep a record of what I searched for", and a
sidebar section is exactly such a record, so it does not quietly keep
its own copy.

Drag order is stored as a list of bookmark ids in settings rather than
on the bookmark records, so reordering does not need a new write path
through the bookmarks IPC. Ids for deleted bookmarks are ignored instead
of leaving a hole, and a bookmark added since the last drag is appended
rather than dropped.

Volume nodes emit `volume:"<label>"` **quoted**, because a label
routinely contains spaces (`Orange WD 4TB`) and unquoted it would parse
as two terms.

The sidebar renders inside `.result-area`, sharing one flex row with the
result list and the preview pane, so toggling it does not disturb the
menu / search / status chrome above and below.

**A duplicate caught while wiring this**, worth recording because it was
already wrong: the Quick Look navigation added in SRC-M19 walked
`resultsStore.batches` directly. Batches are stored in *arrival* order
and carry hits from lenses the user has switched off and rows a
refinement has filtered out — while the list renders through
`viewForLens` in a fixed lens order. `resultsStore.visibleHits` already
existed and got this right. The navigation helper now builds on the same
`viewForLens`, so the arrow keys walk what is on screen rather than what
the daemon happened to send first.

**SRC-M21 — permission health report + macOS Full Disk Access wizard.**
Tools → Permission Health, plus a status-bar badge that appears only
when there is something to say, opening a drill-down of which folders
the scanner could not read, on which volume, for which reason — with the
guided fix for the platform the *index* runs on.

**The badge the feature line says to extend did not exist.** There was
no "n paths skipped (permission)" anywhere: a directory the scanner
could not open produced one `warn!` line and nothing else, so a subtree
that was unreadable looked identical to a subtree that was empty. The
ledger (`crates/freally-indexd/src/permissions.rs`) had to be built
first. This is the fourth time in this stable-gate run that a
Must-Have's stated starting point turned out not to be there.

What the ledger records and why:

- **Roots, not files.** `walkdir` reports one error for a directory it
  cannot open and does not descend, so what lands in the ledger is
  naturally the root of each unreadable subtree. That is also the useful
  answer: "grant access to this folder", not ten thousand filenames.
- **A vanished path is not a fault.** `NotFound` is dropped entirely —
  a file deleted mid-scan is normal on a live filesystem, and counting
  it would make every report on a busy machine look alarming.
- **Capped at 2,000 entries, with the overflow counted.** This list is
  held in memory, serialized, and sent over IPC; past a few thousand it
  stops being a report a person reads. The UI says "and 12,043 more"
  rather than implying the list is complete.
- **A rescan clears that root's entries first**, so a permission the
  user actually fixed stops being reported. Without it the report only
  ever grows.
- **Persisted** to `config/permissions.json`, because "files I couldn't
  index" has to survive a restart to be worth anything.

The guidance is resolved **on the daemon side**, not from the browser's
user-agent: the index can live on a different machine than the window
rendering the report whenever a client is pointed at a remote endpoint,
and the fix belongs to the machine that could not read the folder.

On macOS the report **detects Full Disk Access** by reading a
TCC-protected directory: success means granted, `PermissionDenied` means
not. Any other error is reported as *undetermined* rather than as "not
granted" — telling someone to fix a setting that is already correct is
worse than saying nothing. When it is not granted, a button opens the
Full Disk Access pane directly.

Smoke: `tests/smoke/build_03_permissions.rs` proves the scanner is
actually holding the ledger — a clean tree leaves it empty, and a real
scan drops the previous pass's entries for that root while leaving other
roots alone. It deliberately does not try to create a denied directory:
that is not portable (`chmod 000` is a no-op for root, and Windows would
need ACL surgery), so the denial paths are unit-tested by constructing
the `io::Error` directly instead of asking the OS for one.

**SRC-M18 — inline audio/video playback in the preview pane.** Transport
(play/pause, seek, loop), a volume slider, a rendered waveform you can
click to scrub, and the codec / sample-rate / channel / LUFS badges
overlaid. Auditioning a `lufs:<-14` result no longer means leaving
Freally, which is the workflow the audio lens exists to keep you inside.

**The waveform costs no extra decode.** A `PeakCollector` rides along
with the existing analysis pass in `freally-audio`: the same interleaved
frames that feed the loudness and silence accumulators feed it, so
opening a track in a pane that re-opens on every arrow key does not
decode the file a second time just to draw a picture of it. Buckets are
fixed-count rather than per-second because the canvas has a fixed width —
resampling a per-second envelope to fit throws away exactly the peaks
that make the shape readable — and they peak across channels, since a
waveform shows the loudest thing happening at that moment, not the left
channel's opinion of it.

**The bytes go through IPC, not the asset protocol — deliberately.** The
conventional way to feed a media element is Tauri's asset protocol, and
it was rejected: enabling it means declaring a filesystem scope the
*webview* may read directly, and any scope wide enough to cover
"wherever the user's media lives" is wide enough to undo the
`KnownPaths` gate Build 1 hardened — the property that a compromised
frontend dependency cannot reach a file the daemon never returned.
Routing the bytes through a command keeps every read behind the same
check as every other file operation.

The cost is that playback is buffered, not streamed, so there is a
192 MB ceiling — stated in the UI, which offers to open past it in the
system player, rather than failing silently. Bytes are returned as a raw
`tauri::ipc::Response` rather than a base64 data URL, which would inflate
the payload by a third and cost a full re-encode on both sides.

Two smaller things that would each be a real bug: the object URL is
revoked when the selection changes (without it, every arrow key leaks a
whole decoded file), and a non-finite integrated loudness renders as *no
badge* rather than as `-inf LUFS` — a clip shorter than one gating block
has no meaningful integrated value.

Playability is decided from the hit's extension, not from the preview
payload: a media file's payload is `unsupported`, which is precisely the
case worth playing.

### Build 3 closeout — what the review passes changed

`/simplify` ran four reviewers over the full build diff. Three of them
independently flagged the same thing, and two findings were real defects
rather than style:

- **The sidebar dropped a bookmark's saved filters.** `applyBookmark`
  re-implemented the dropdown's load flow and omitted
  `typeFilterStore.setFromIds(...)`, so opening a bookmark from the
  sidebar composed a different lens prefix than opening the same
  bookmark from the dropdown. Both now call one `bookmarksStore.apply`.
- **`visibleHits` existed twice, with different answers.** The Quick
  Look copy sorted; `resultsStore.visibleHits` — which export,
  select-all and the status-bar count read — did not. The two disagreed
  the moment a column sort was active. The sort moved into the store's
  getter and the copy is gone, which also makes that getter's "the
  single answer to what the user is looking at" comment true rather than
  aspirational.
- **Two hand-synced strict-mode lists had already drifted.** The
  parser's Freally-only modifier list carried `volume:` and `report.rs`'s
  copy did not, so a strict-everything query using `volume:` was
  rejected by the parser while the report the search bar renders from
  called it fine. Now one `is_freally_only_modifier`.

Performance findings worth recording, all on paths this build added:

- The permission badge's effect re-fired on every index-state poll —
  `indexStateStore` reassigns the whole state object, so "read `phase`"
  invalidates regardless of whether `phase` changed. It was issuing an
  IPC round-trip every five seconds forever, each cloning the whole
  ledger under a mutex. Guarded on the last polled phase.
- The natural-sort comparator ran a regex per *character* and built a
  fresh collator per chunk. Now a code-point test and one shared
  `Intl.Collator`.
- Sorting by path converted both `PathBuf`s per comparison —
  O(n log n) WTF-8 scans. Now decorate-sort-undecorate, one conversion
  per row.
- The media player loaded bytes and waveform in sequence though they are
  independent; and gave all ~800 waveform bars a dependency on playback
  progress, so every `timeupdate` re-evaluated 800 expressions. Now
  `Promise.all`-style overlap and a single `--progress` overlay.

Also: `AnalysisOpts::peak_buckets` is private, because as a `pub` field a
caller could set it on `analyze_with_opts`, which has nowhere to return
peaks and would silently discard them; and `media.rs` calls the shared
`files::verify_readable` rather than its own copy, so the provenance
level for frontend-asserted reads is decided in exactly one place.

### Build 3 closeout — what `/security-review` changed

Two HIGH findings, both introduced by this build, both fixed.

**`media_bytes` was gated at the level the frontend can mint for
itself.** It returns the *complete* contents of a named file to the
webview, and it was using `Provenance::FrontendAsserted` — the level
`files_whitelist_user_chosen` hands out for any string the JS layer
asks for. A compromised frontend dependency could have whitelisted
`~/.ssh/id_ed25519` and then read it, with no dialog and no daemon
involved. The other two commands that return file contents,
`content_document` and `copy_file_contents`, already demanded
`QueryHit`; these two now do too. The legitimate caller is driven from
result rows, which carry that level already, so nothing user-facing
changes.

**The portable-mode socket sat directly in the shared temp directory.**
`/tmp` is mode 1777 and the name was an FNV hash of a guessable path, so
any local account could bind it first. The transport authenticates the
*peer* of a listener it owns; a client connecting out checked nothing,
so squatting that path was a full daemon impersonation — and
`register_hit_paths` turns anything arriving in a `query:batch` into
`Provenance::QueryHit`, which is the gate on delete, rename, and every
other destructive command. Three changes:

- The socket moved into a per-user directory — `$XDG_RUNTIME_DIR` where
  it exists, otherwise a uid-tagged subdirectory this process creates
  and chmods 0700, refusing it outright if it turns out to be owned by
  someone else.
- `transport::unix::connect` now refuses a socket this user does not
  own, or whose mode is reachable by others. It uses
  `symlink_metadata`, so a symlink pointing at a socket we do own does
  not satisfy the check. This is the missing counterpart to the
  listener's peer-uid check.
- `listen` no longer hard-fails when it cannot tighten the socket's
  parent directory. `chmod 0700 /tmp` returns `EPERM` for every
  non-root user, so **portable mode could not start its daemon on Linux
  at all** — the hardening step was taking down the thing it was meant
  to protect. It logs and relies on the socket's own 0600 instead.

### TASK-098 — full Fluent i18n end-to-end across all 18 locales (2026-05-11)

The 18-locale Fluent loader is now wired. Switching the language in
Settings → Locale (or in the first-run wizard) re-renders every
translated string in the UI — menus, status bar, settings panels,
dialogs, and the wizard — without a restart.

**Loader.**

- `apps/freally-ui/src/lib/i18n/bundle.ts` replaces the Phase-11
  `EN_FTL` inline string with `import.meta.glob("../../../../../locales/*/freally.ftl", { query: "?raw", eager: true })`.
  All 18 `.ftl` files are inlined at build time; each `FluentBundle`
  layers `en` underneath as a fallback resource so a stray missing key
  surfaces in English rather than as a raw key string.
- `vite.config.ts` extends `server.fs.allow` to the workspace root so
  dev mode can read the locale tree that lives outside the package
  root.
- `bundle.ts` exports `loadedLocales()` for the test suite to assert
  the glob actually picked up all 18 files.

**Translation data — Standing Rule #4 lockstep.**

- `en/freally.ftl` grew from 314 to 557 keys. The 243 additions cover
  wizard polish (hints, placeholders, "Step N of N"), status bar
  segments, lens / preview / bookmarks strings, the About / Connect
  dialogs, every UI/Home/Backup/Keyboard/History/Locale/Folders/Volumes
  panel hint + section title + toast, and the full PRD §8.28 menu bar
  (every File/Edit/View/Search/Bookmarks/Tools/Help label + every
  submenu title + every hover-hint).
- The same 243 keys were mirrored into all 17 other locales in parallel
  (es, de, fr, it, nl, pl, pt-BR, tr, vi, id, ru, uk, ar, hi, ja, ko,
  zh-CN). Every `.ftl` now resolves the same 557 keys; the new
  `tests/unit/i18n.test.ts` lockstep test asserts this on every CI run.

**Component conversion.**

- `menu_spec.ts` gains an `l10n` key on every `MenuItemSpec` and
  `MenuSubmenu`, plus a `hintL10n` for status-bar hover hints.
  `MenuBar.svelte` resolves them via `labelOf(spec)` / `hintOf(spec)`
  helpers that fall back to the literal `label`/`hint` when a
  translation key isn't present.
- The FirstRunWizard renders its title, step count, every step's
  heading + hint, the theme cards, and the Back/Next/Finish buttons
  through `t()`. The hotkey step was already removed in the earlier
  wizard pass; hotkey config remains in Settings → Keyboard.
- StatusBar uses `t()` for the index-phase segment ("Indexed (N
  files)" / "Indexing… N/M" / "Paused" / "Error"), result-count
  pluralization (`status-result-count-one` vs `…-many`), the selection
  size badge, query timing, lens timing badges, and the local-DB /
  remote-endpoint segment. The theme-cycle button's `aria-label` and
  the hotkey hover hint are now translatable.
- Settings dialog — `SettingsDialog.svelte`, `SettingsTreeNav.svelte`,
  `SettingsButtonBar.svelte`, `LocalePanel.svelte`, plus the panels in
  the Indexes, Lenses, Network, and Misc groups (UI / Home / Search /
  Results / View / Context Menu / Fonts & Colors / Keyboard /
  Indexes-top / Volumes / Folders / FileLists / Exclude / Filename /
  Content / Audio / Similarity / Custom / HTTPS / ETP / History /
  Privacy / Logs / Backup / About).
- Bookmarks, preview pane, and `LensSection` empty-state / collapse
  controls.

**RTL is now automatic, not a checkbox.**

- The "RTL preview" checkbox in Settings → Locale is gone. RTL applies
  automatically for locales whose native script is RTL — currently
  `ar` is the only ship-locale in that bucket. `applyRtlForLocale`
  consults its internal `RTL_LOCALES` allowlist; `bootstrap.ts`'s
  `locale_settings.rtl_preview` field remains in the persisted state
  for backward-compat but the UI no longer surfaces it.

**Tests.**

- New `tests/unit/i18n.test.ts` covers: (a) the glob actually loaded
  all 18 locales, (b) `bundleFor(code)` returns a bundle that resolves
  a canary key for every locale, (c) the 18 locales are in perfect
  lockstep on key set, (d) switching `settingsStore.state.locale`
  changes what `t()` returns, (e) an unknown locale falls back through
  `en` rather than the raw key string, (f) an unknown key returns the
  key.

### Phase-12 polish pass — UX, reliability, and live-apply (2026-05-11)

Major behavioral pass during a long debugging session. Most fixes are direct
voidtools-Everything parity gains plus the foundational reliability work that
makes the desktop app pleasant to run repeatedly during development.

**Single-instance + non-blocking daemon boot.**

- `kill_other_freally_instances()` runs at the top of `run()` on Windows
  (taskkill `/F /T` against `freally-ui.exe` + `freally-indexd.exe`,
  filtered to PIDs ≠ self) so relaunching the app always starts from a clean
  slate without manual process killing. macOS / Linux stub for parity.
- `Daemon::boot` now spawns a dedicated `freally-daemon-boot` thread inside
  `tauri::Builder::setup`; the setup hook returns immediately and the window
  appears right away. Previously the canonical-store replay could block the
  GUI thread for 10-15 s, tripping the Windows non-responsive-window watchdog
  and tearing the process down before any HWND existed.
- `Client::connect` is wrapped in a 500 × 40 ms retry loop on the consumer
  side so a slow daemon boot doesn't lose to a single connect race.

**Filter chips + Search menu — multi-select with OR composition.**

- New `lib/stores/type_filter.svelte.ts` holds a `Set<TypeFilterId>`; default
  is the full set (Everything mode). `toQueryFragment()` emits a parser-level
  `(audio: OR video: OR …)` group for partial selections, empty string for
  "everything" or "none" so the daemon-side AND-of-prefixes pitfall is gone.
- `QuickFiltersPalette.svelte` chips toggle the store directly; the menu
  items in `Search → …` switch from `radio` to `checkable`, with
  `MenuBar.isItemChecked` reading the store. "Everything" derives from
  `selected.size === ALL.length` — clicking any individual chip flips both
  that chip and Everything off, leaving the others.
- `bookmarks.add` saves the active filter set alongside the search text; the
  Rust `Bookmark` DTO gains `filters: Vec<String>` with `#[serde(default)]`
  so existing `bookmarks.json` files keep deserializing. Clicking a bookmark
  restores both the textbox content (via `queryStore.setSource`) and the
  chip selection (via `typeFilterStore.setFromIds`).
- Fixed Archive's token: `compressed: "zip:"` → `"archive:"` (the real
  `QuickFilter::Archive` alias from `freally-query::quick_filters`). The
  old `zip:` was just an extension within the group, so Archive never
  matched anything.

**Initial Everything query + everything-mode parity.**

- `runInitialEverythingQuery()` in `bootstrap.ts` fires once after hydrate
  and polls `resultsStore.batches.length` for up to 60 × 800 ms, kicking
  fresh `run()` calls only when nothing is in-flight so the auto-fire
  doesn't cancel its own queries. First paint after launch shows results
  immediately instead of "Type a query to begin."
- When the full type-filter set is selected and the search box is empty,
  `resultsStore.run()` composes a bare `*` wildcard so the filename lens
  lists every indexed entry (voidtools-Everything parity).
- `ResultList.svelte` placeholder gate updated: shows "Type a query to
  begin" only when the source is empty AND no type filters are selected
  (i.e. the user has explicitly deselected every chip).

**Folder indexing.**

- `crates/freally-indexd/src/scanner.rs::scan_folder` now indexes
  directories alongside files; the walkdir path filters on
  `is_file() || is_dir()` and stamps `FILE_ATTRIBUTE_DIRECTORY (0x10)`
  into the journal event's `attrs` field.
- `crates/freally-journal-win/src/subscriber.rs` MFT bootstrap path no
  longer skips directory records — they ride through with their real
  `file_attributes` bitmask intact.
- `QueryHit` (Rust DTO + matching TS interface) gains an
  `attrs: u32 #[serde(default)]` field that the daemon populates from
  `FileRow.attrs`. UI distinguishes file vs folder via the `0x10` bit.

**Real Windows shell icons + per-row rendering.**

- New `apps/freally-ui/src-tauri/src/commands/icons.rs` —
  `icon_for_ext(ext, is_dir) -> Option<String>` async Tauri command that
  runs Win32 `SHGetFileInfoW` with `SHGFI_USEFILEATTRIBUTES` (so a dummy
  path like `_.xml` resolves the registered handler's icon without the
  file actually existing on disk) → `GetIconInfo` → 32-bit BGRA via
  `GetDIBits` → BGRA→RGBA channel swap → PNG via `image` crate → base64
  data URL. Returns `None` on macOS / Linux for now.
- `src-tauri/Cargo.toml`: adds `image = "0.25" --no-default-features
  --features png`, `base64 = "0.22"`, and the relevant `windows = "0.59"`
  feature set under `cfg(windows)`.
- New `lib/stores/icon_store.svelte.ts` — `Map<(ext, is_dir),
  Promise<dataUrl | null>>` cache so 200 result rows fire one IPC per
  unique extension, not per row. Reactive `tick` field that increments
  on each resolution so $derived consumers re-render.
- `ResultRow.svelte` Name column renders `<img class="row-icon" src=…>`
  pulled from `iconStore.get(hit.ext, isDir)`, with an emoji fallback
  (`📁` / `📄`) while the data URL is loading.

**Real metadata in MFT bootstrap.**

- USN records carry FRN + attrs + a single timestamp but no file size, so
  the MFT-fast-path used to write `size: 0` and a USN-only timestamp into
  the index. `subscriber.rs` now does a `std::fs::metadata(&full)` per
  emitted event to populate real `len`, `modified`, `created`. Slower
  than the pure USN walk but correct — the Size and Modified columns
  finally show non-zero values without forcing the walkdir fallback.

**Search box + result row visuals.**

- `SearchBar.svelte` now binds the `<input value={queryStore.source}>` so
  programmatic updates (bookmark click, Escape clear, future deep-link)
  reflect in the textbox. Also dropped the broken `color: transparent`
  on `.raw` (the "mirror" syntax-highlight layer wasn't aligning, so the
  typed text was effectively invisible in dark mode) — input renders its
  own text now and the mirror is hidden.
- `ResultRow.svelte` row CSS reads `--row-{state}-fg/bg/weight/style` so
  the Fonts & Colors panel's per-state controls are live. States wired:
  normal, highlighted (hover), selected, selected_highlighted.
- Fixed alternate-row + hover specificity bug that made a selected row
  appear unselected when its `:nth-child(even)` rule outranked
  `.row.selected`. Both selectors now have `:not(.selected)` so a
  selected row keeps its cyan tint regardless of index or hover.

**Fonts & Colors live-apply + persistence.**

- New `lib/stores/fonts_apply.svelte.ts::applyFontsAndColors()` writes
  CSS custom properties on `<html>` from `settings.fonts_and_colors`:
  `--font-ui` (with cross-OS fallback chain), `--app-font-size`,
  per-state `--row-…-fg/bg/weight/style`, per-lens `--lens-…` overrides.
  Called once on bootstrap (post-hydrate) and again on every panel
  patch — restores across launches and live-applies on change.
- `FontsAndColorsPanel.svelte` font input switched to a `<select>` dropdown
  populated via `window.queryLocalFonts()` (Local Font Access API
  supported by the Tauri 2 WebView2 runtime on Windows) with a curated
  25-family fallback for non-Chromium webviews. Each option renders in
  its own font for visual preview.
- Theme dropdown in UIPanel calls `themeStore.set(value)` alongside the
  settings patch so light/dark switches live-apply on selection — no
  Apply / restart needed.

**View panel toggles wired.**

- App.svelte's $effect mirrors selected settings to `<body data-*>`
  attributes; new CSS rules in `app.css` react to them:
  `data-alternate-rows`, `data-row-mouseover` (overrides `.row:hover`
  to no-op when false), `data-show-tooltips`, `data-show-lufs-badges`,
  `data-show-similarity-score`.
- `ResultRow` row `title={hit.path}` is conditional on the Show tooltips
  setting.
- `formatBytes` honors the Size Format setting (`auto_binary` |
  `bytes` | `kb` | `mb` | `gb`).

**Window controls — always-on-top, size, zoom.**

- `capabilities/default.json` adds the missing
  `core:window:allow-set-size`, `core:window:allow-set-always-on-top`,
  `core:window:allow-set-resizable`, `core:window:allow-inner-size`,
  `core:window:allow-outer-size` grants. Without these, every
  `setSize` / `setAlwaysOnTop` call was being silently rejected by the
  Tauri IPC permission gate.
- App.svelte gains an `$effect` that translates `settings.on_top` +
  `queryStore.source` into `setAlwaysOnTop(...)` calls: Never / Always
  / WhileSearching modes all live, re-applied on every settings change
  and every keystroke. Restored on launch.
- `setWindowSize` import fixed: `LogicalSize` comes from
  `@tauri-apps/api/window`, not the non-existent
  `@tauri-apps/api/dpi`. Picked size also writes
  `settings.window_size = { w, h }` (allowlisted in
  `ALLOWED_PATCH_KEYS`), and bootstrap restores the saved size on
  next launch.
- `zoomStore` swapped from `document.documentElement.style.fontSize`
  to the WebView's `zoom` CSS property so Ctrl+= / Ctrl+- actually
  rescale the (px-based) UI. Crisp at all factors.

**Image preview.**

- `preview/windows_host.rs::preview` returns real image data URLs for
  PNG, JPG/JPEG/JFIF, GIF, WEBP, BMP, SVG, ICO, AVIF: reads the file
  (4 MiB cap), encodes via `commands::files::base64_encode`, returns
  `PreviewPayload { kind: Image, data_url: "data:image/…;base64,…" }`.
  Non-image extensions still return `None` so the text-head fallback
  handles text files.
- `files_preview`, `files_thumbnail`, and `icon_for_ext` are now
  `async fn` and offload the actual work to
  `tokio::task::spawn_blocking`. The synchronous versions were
  blocking Tauri's IPC dispatch thread for seconds during shell-icon
  extraction + multi-MB base64 encoding, which froze the entire UI.
- Fixed `PreviewPane.svelte` calling a missing
  `files.whitelistUserChosen` — the helper lived in `ipc/bookmarks.ts`
  but the pane imports `* as files from "ipc/files"`. Re-exported
  `whitelistUserChosen` from `ipc/files.ts` so the call resolves; the
  synchronous `TypeError` was leaving `loading = true` forever and
  blocking the preview $effect.
- Added a Rust tracing pass on the preview / icon paths plus a
  `log_event` Tauri command for forwarding TS console events into the
  cargo dev log, and a `std::panic::set_hook` that surfaces Rust
  panics in the console.

**FTP endpoint + greyed Disconnect.**

- New `ConnectEndpointDialog.svelte` modeled on the voidtools dialog:
  Host (required), Port (default 21), Username, Password, Link type
  dropdown. On OK, writes `settings.endpoint = { name: host, kind:
  "ftp" }`.
- `MenuBar.isItemEnabled` returns `false` for
  `tools.disconnect_endpoint` when `settings.endpoint.kind === "local"`;
  CSS adds an `.item.disabled` greyed style with `pointer-events: none`.
- `View → Filters` now pre-selects the `indexes.exclude` panel before
  opening the settings dialog (voidtools-Everything parity).

**Exclusions — toggleable extension classes + dedup.**

- `ExcludePanel.svelte` extension-class buttons (Video / Audio / Image
  / Archive / Executable) now toggle: click adds the class's globs and
  highlights the button; click again removes them and un-highlights.
  Computes "active" as "every glob in the class is currently in the
  exclude-files set" so the highlight reflects real state, not just
  the last click.
- `Add Folder…` and `Apply OS-recommended excludes` both dedupe so the
  same entry never gets added twice.

**Bookmarks reliability.**

- `bookmarks.add` no longer requires a non-empty query — empty-query
  bookmarks are allowed (named "Bookmark N" so the user can rename
  them in Organize). Dedupe key is now (name, query, filter-set).
- `bookmarksStore.hydrate()` retries every 500 ms for up to ~10 s so
  the first hydrate doesn't lose the bookmarks list to a
  daemon-not-ready IPC error during the background-boot window.
- `bookmarks.organize` re-hydrates before opening the Organize dialog
  so a stale dropdown can't show "No bookmarks yet" when there are
  bookmarks on disk.

**Misc.**

- `tracing_subscriber` default filter raised to
  `warn,freally=info,freally_ui_lib=info,freally_indexd=info`
  so the instrumentation lands without forcing `RUST_LOG`.
- Settings dialog `markDirty` now also calls `applyFontsAndColors()`
  inside the Fonts & Colors panel patch path so the preview is
  instant.
- `ipc/types.ts::QueryHit` gains `attrs?: number`; `Bookmark` gains
  `filters?: string[]`. Both optional for forward compat.

### Added

- **[all platforms]** Phase 12 settings dialog + real daemon IPC + custom-extractor framework + i18n. Replaces the Phase-11 mock IPC layer with a real `freally-rpc` length-prefixed JSON-RPC transport over a per-user Unix socket / named pipe and lands the full PRD §8.1-§8.27 settings dialog plus the Wasm-sandboxed custom-extractor host.

  **New crate `freally-rpc`** (under `crates/freally-rpc/`) — the foundation that lets the Tauri UI and the new `freally` CLI both speak the same protocol to a single `freally-indexd` instance.
  - `frame.rs`: u32-BE length-prefixed framing with a 16-MiB hard payload cap so a hostile peer cannot OOM the server with a single 4-GiB length prefix.
  - `jsonrpc.rs`: JSON-RPC 2.0 Request / Response / Notification envelopes; the `ResponseEnvelope` untagged enum disambiguates a single frame as either a response or a server-pushed notification.
  - `service.rs`: the `Service` trait that `freally-indexd` implements; `NotificationSink` lets a method handler push asynchronous notifications down the same connection.
  - `server.rs`: per-connection accept loop with a bounded outbound queue (`PER_CONN_OUT_QUEUE = 256`); spawns a writer task per connection and a notification fan-in task that forwards `Notification`s onto the writer queue. `handle_connection_for_tests` exposes the same loop body over a `tokio::io::duplex` for sub-second integration tests.
  - `client.rs`: typed `ClientHandle::call()` with a per-call oneshot reply channel; `notifications()` returns a tokio broadcast subscription. The reader task drains pending callers with a clean `transport closed` shape on EOF.
  - `transport/unix.rs`: `UnixListener::accept_authenticated()` reads the peer credentials via `tokio::net::UnixStream::peer_cred()` and rejects any peer whose UID does not match the current process — combined with the 0600 file-mode set in `listen()`, this means a foreign user on the same machine cannot connect even with the path. The parent directory is chmod'd 0700 belt-and-suspenders.
  - `transport/windows.rs`: `ServerOptions::create_with_security_attributes_raw` with a SECURITY_DESCRIPTOR generated from an SDDL string of the form `D:(A;;GA;;;<userSid>)(A;;GA;;;SY)` — only the current user (resolved via `OpenProcessToken` + `GetTokenInformation(TokenUser)` + `ConvertSidToStringSidW`) and SYSTEM hold an Access-Allowed ACE. No `Everyone`, no `Authenticated Users`. `reject_remote_clients(true)` blocks network-pipe access. `LocalFree` is wrapped in a `SdDrop` RAII guard so the security descriptor is freed even on early-return error paths.
  - `path.rs`: per-OS conventional default socket / pipe path. macOS: `~/Library/Application Support/freally/indexd.sock`. Linux: `$XDG_RUNTIME_DIR/freally/indexd.sock` with `~/.local/share/freally/indexd.sock` fallback. Windows: `\\.\pipe\freally-indexd-<userSid>` (SID-tagged so two users on the same Windows host get separate pipes).
  - `dto.rs`: serde DTOs that mirror `apps/freally-ui/src/lib/ipc/types.ts` byte-for-byte — same `serde(rename_all = "lowercase")` enum variants, same `serde(rename = "type")` for the `kind` JSON-key, same field shapes. Phase 12's parity audit asserts byte-stable JSON output against checked-in fixtures.

  **New crate `freally-extractor-host`** (under `crates/freally-extractor-host/`) — the Wasm-sandboxed custom-extractor framework. Untrusted by default per Phase 12 trust model.
  - `manifest.rs`: TOML schema with `id`, `display_name`, `version`, `formats: Vec<String>`, optional `magic: Vec<String>` (hex-byte-only specs like `"0x23 0x20"`), `sidecar` (path to the `*.wasm` binary, validated to exist at load time), `time_budget_ms` (default 1000), `memory_budget_mb` (default 64). Bad magic specs reject at load time.
  - `registry.rs`: `<index_root>/extractors/` scanner that loads every subdirectory's `manifest.toml`. `registry.toml` records the user's per-extractor trust state, blake3 hashes for tamper detection, and a crash counter that auto-disables an extractor that crashes 3+ times in a row until the user re-trusts it. Persistence is tmp-rename via `toml::to_string_pretty`. `set_trusted` clears the crash counter on re-trust.
  - `sandbox.rs`: `wasmtime` host with strict guarantees — `Config::consume_fuel(true)` enforces a per-call CPU budget (`fuel = time_budget_ms × 1_000_000`); post-call `Memory::data_size` check enforces the per-call memory budget. Only two host functions are visible to the guest: `host_log(ptr: i32, len: i32)` (debug logging, truncated at 4 KiB) and `host_now_ms() -> i64` (host-injected, matches the request's `now_ms` so the guest cannot observe wall time independently). No `wasi:sockets`, no `wasi:filesystem-write`, no `wasi:clocks`. The guest exports `alloc(size) -> i32` and `extract(ptr, len) -> i64` (high 32 bits = result pointer, low 32 bits = result length). 16-MiB cap on the result length so a hostile sidecar can't return a 4-GiB blob.

  **`freally-indexd` library + binary refactor.** The Phase 1-3 service-only binary becomes a library + thin shim. The library exposes `DaemonState` (the shared state container holding `Arc<Index>`, `Arc<AudioCache>`, `Pipeline`, `Registry`, plus persisted `volumes / folders / excludes / network / history` configs in `<index_root>/config/` as TOML files) and `IndexdService` (the `Service` impl that dispatches every method enumerated in PRD §8.30). New modules:
  - `service.rs`: typed dispatch for `query.run` / `query.cancel` / `query.lens_timings` / `index.state` / `index.verify` / `index.compact` / `index.rebuild` / `extractors.list` / `extractors.set_mode` / `volumes.list` / `volumes.update` / `volumes.recreate_journal` / `volumes.reset_stream` / `volumes.upgrade_fanotify` / `volumes.remove` / `folders.list` / `folders.add` / `folders.remove` / `folders.update` / `folders.rescan` / `folders.rescan_all` / `excludes.get` / `excludes.set` / `network.status` / `network.start_https` / `network.stop_https` / `network.regen_token` / `network.start_api` / `network.stop_api` / `custom_extractors.list` / `custom_extractors.set_trusted` / `custom_extractors.refresh_hashes` / `history.get` / `history.set` / `history.clear` / `preview.text_head` / `preview.thumbnail` / `settings.apply` / `daemon.shutdown`. `query.run` streams `query:batch` / `query:done` notifications back to the client (the Tauri side re-emits them as Tauri events the Svelte stores consume).
  - `state.rs`: `DaemonState::open(opts)` opens the index, audio cache, custom-extractor registry, and reads the persisted config TOMLs. `DaemonState::persist()` writes them all back atomically. `VolumesConfig`, `VolumeOverride`, `NetworkState`, `HistoryConfig` types own per-piece persistence.
  - `volumes.rs`: cross-OS volume detection — Windows walks `GetLogicalDrives` + `GetVolumeInformationW` + `GetDiskFreeSpaceExW` and emits NTFS / ReFS / exFAT / FAT32 rows; macOS scans `/Volumes` + `statvfs`; Linux reads `/proc/mounts` (skipping pseudo filesystems) + `statvfs`. Each row carries a stable `id` (e.g. `win-C`, `lin-_home`, `mac-Macintosh_HD`) so per-volume overrides round-trip.
  - `settings.rs`: `SettingsApply` typed payload for the `settings.apply` IPC; the daemon mutates relevant state and persists. `random_token_fingerprint()` produces the short non-secret display fingerprint shown in the Network panel.
  - `history.rs`: `HistoryUpdate` typed payload + `take_clear` future-Phase-13 hook for the daemon-side history wipe.

  **`apps/freally-ui/src-tauri` becomes the RPC client.**
  - Deleted `commands/canned.rs` (the Phase-11 mock dataset). The Phase 12 smoke test `tests/smoke/phase_12_indexd_client.rs::no_canned_rs_in_tree` regresses the deletion.
  - New `daemon.rs`: boots an in-process `freally-indexd` at the per-OS default socket path (env override `FREALLY_RPC_SOCKET` for tests), opens a `freally-rpc` client, and spawns a notification re-emitter task that turns every server-pushed notification into a Tauri event via `app.emit(method, payload)`. `Daemon::call/call_void` clones the client and runs the future on the daemon's tokio runtime, sidestepping the move/borrow conflict that `Arc<Daemon>::block_on(async move {…})` would introduce.
  - `commands/query.rs`, `commands/index_state.rs`, `commands/extractors.rs`, plus new `commands/volumes.rs / folders.rs / excludes.rs / network.rs / custom_extractors.rs / history.rs` — every Tauri command body now routes through the daemon. `query_parse` is the only in-process exception — keystroke-rate tokenization can't afford the daemon round-trip.
  - `commands/files.rs` keeps its UI-side `verify_path` known-paths gate, but `files_thumbnail` / `files_preview` now delegate to the new `preview` module which dispatches to OS-native preview hosts.
  - `commands/settings.rs` keeps the JSON-backed local persistence and gains a `#[serde(flatten)] extras` HashMap that captures the 70+ Phase-12 top-level fields without bloating the typed-scalars surface; `ALLOWED_PATCH_KEYS` expands to cover every new key for the security review's allowlist contract; `phase_12_default_extras()` populates Phase-12 defaults so a fresh-install `settings.json` ships every field. The TS contract in `lib/ipc/types.ts` is unchanged.

  **OS-native preview hosts** (`apps/freally-ui/src-tauri/src/preview/`). The platform module structure is in place; each host's full integration lights up incrementally as a UX-quality enhancement that does not change the data-URL contract.
  - `preview/macos.rs` — QuickLook bridge (`QLPreviewPanel` for full preview, `QLThumbnailGenerator` for thumbnails). Phase 12 ships the module surface and the runtime probe; the full `objc2` call sequence lands as a polish-pass enhancement.
  - `preview/windows_host.rs` — Shell preview handlers (`IPreviewHandler` + `IThumbnailProvider` via windows-rs). Same posture — the surface is in place, the COM bridge ships incrementally.
  - `preview/linux.rs` — GNOME Sushi via DBus when present, KDE KIO via subprocess shell-out when present. Detection is a `OnceLock`-cached `gdbus introspect` / `kioclient5 --version` probe so the per-preview cost is zero after the first call.
  - `preview/fallback.rs` — niche-Unix fallback. Always returns None so the caller drops to the universal text-head + typed-icon path.
  - `preview/mod.rs` — host dispatch + `text_head_fallback` (read up to 4 KiB, classify as text iff no NUL bytes and replacement-char ratio ≤ 1%).

  **Settings dialog** (`apps/freally-ui/src/components/settings/`) — every PRD §8.2-§8.27 control wired with no stubs.
  - `SettingsDialog.svelte`: resizable modal (min 800×620, default 960×720), left-tree-nav + right-detail-pane + bottom-button-bar layout. Persists last-selected node + per-pane scroll position via localStorage so reopening the dialog returns to the panel the user was last in.
  - `SettingsTreeNav.svelte`: full PRD §8.1.1 tree (General / History / Indexes / Lenses / Network / Privacy & Updates / Logs & Debug / Backup, Export, Reset / Locale / About). Search-the-options box filters nodes by label and per-node keyword set. Dirty panels carry a purple `•` so the user sees which ones have unsaved changes.
  - `SettingsButtonBar.svelte`: Restore Defaults (left, per-panel), OK / Cancel / Apply (right). Apply enables only when `settingsDialog.dirty`; OK applies + closes; Cancel rolls back via `SettingsDialogModel.cancel()` (every store's `snapshot()` is taken on dialog open, `rollback()` restores it); Restore Defaults resets the active panel via `SettingsDialogModel.resetPanel(panelId)`.
  - 26 panel components, one per PRD §8.2-§8.27 section. Highlights:
    - **General → UI**: theme picker (live-flips), tray toggles, single-click variants, row density, animated cross-fade.
    - **General → Home**: Use last value | On | Off triplets for every match default; filter / sort / view / index source dropdowns; default lens visibility + per-lens result limits.
    - **General → Search**: every voidtools-Everything DSL behavior toggle plus Freally extras (strict-Everything mode, auto-regex, modifier completions, parse-tree-on-hover).
    - **General → Results**: every behavior + load-priority dropdowns + group-by-lens.
    - **General → View**: every (E) display toggle + Freally audio/similarity badges + preview-pane position.
    - **General → Context Menu**: per-command Show / Show only when Shift held / Hide + macro string for all 10 entries.
    - **General → Fonts & Colors**: font + size + per-state foreground/background color (with `<input type="color">`) + bold + italic for all 8 item states + per-lens accent + theme-inheritance toggle.
    - **General → Keyboard**: global hotkey + per-window hotkeys + chord registry (Add/Remove rows for command+binding pairs).
    - **History**: search/run history toggles + retention days + Clear Now button + privacy mode + per-lens history toggles.
    - **Indexes (top-level)**: every Everything index-wide field toggle + Force Rebuild / Compact / Verify buttons (real daemon ops with toast feedback).
    - **Indexes → Volumes**: cross-platform volume detection with FS badges (NTFS / ReFS / exFAT / FAT32 / APFS / HFS+ / ext4 / Btrfs / ZFS / XFS / F2FS) + status pip; per-volume Include / Include only / Enable journal subscription (label varies per OS — USN / FSEvents / inotify) / Buffer / Allocation delta (NTFS-only) / Load recent changes / Monitor changes; per-OS buttons (Recreate journal on NTFS, Reset stream on APFS, Upgrade to fanotify on Linux); Remove button.
    - **Indexes → Folders**: Add via OS folder picker + per-folder monitor toggle + buffer + rescan schedule (At time / Every N hours / Never) + Rescan Now / Rescan All Now buttons.
    - **Indexes → File Lists**: Add file list via picker + format dropdown (text / JSON / .srcb) + auto-export-saved-searches toggle + File List Editor button.
    - **Indexes → Exclude**: folders list + globs + Apply OS-recommended (Win / Mac / Linux per-OS conventional excludes) + Exclude-by-class chips (video / audio / image / archive / executable).
    - **Lenses → Filename**: trigram aggressiveness + suffix-array memory budget + wildcard expansion limit + regex timeout.
    - **Lenses → Content**: enable + per-format mode for 11 formats + budgets + snippet length + stop-words language (18 ship-locales) + re-extract-on-settings-change + verify-blob-checksums.
    - **Lenses → Audio**: enable + per-format mode for 10 formats + LUFS reference standard + peak compute + silence threshold + re-extract-on-modify.
    - **Lenses → Similarity**: enable + signature size (64/128/256) + bands (8/16/32) + recall threshold + result cap.
    - **Lenses → Custom**: community-extractor registry with trust toggles + blake3 hash display + sandbox-permission view + Refresh hashes.
    - **Network → HTTPS Server**: start/stop + bind/port/force-https/legacy-auth + token regen (rotates fingerprint live).
    - **Network → ETP/FTP API**: start/stop + port + legacy plain FTP/ETP toggle.
    - **Privacy & Updates**: auto-update cadence + pre-release toggle + hard-coded read-only network calls policy.
    - **Logs & Debug**: log level (live-changes tracing filter) + retention + open log folder (via `tauri-plugin-opener`) + export diagnostics bundle.
    - **Backup, Export, Reset**: Export / Import settings (TOML round-trip via `tauri-plugin-fs`) + Export / Import bookmarks bundle (.srcb) + Reset all (with confirm).
    - **Locale**: 18 ship-locales dropdown — **English pinned first** then alphabetical by native name (Latin → Cyrillic → RTL → other-scripts grouping); each label is the language's own self-name so the user can pick their language even when the UI is in a script they cannot read; live RTL flip via `applyRtlForLocale()` (sets `dir="rtl"` and `lang` on `document.documentElement` when locale is Arabic or RTL preview is on); date / number format (OS / ISO / RFC / custom).
    - **About**: version + commit + OS detection + license + voidtools credit + open-source notices.
  - SettingsDialogModel (`lib/stores/settings_dialog.svelte.ts`) — the dirty-state machine. Tracks per-panel dirty marks; on dialog open, snapshots every store; `apply()` flushes every store's `flush()` in one shot; `rollback()` restores the snapshots; `resetPanel(id)` reverts only the keys that panel owns. `PANEL_KEYS` maps each PanelId to its owned SettingsState keys for surgical reset.
  - 6 new daemon-routed stores under `lib/stores/`: `volumes / folders / excludes / network / custom_extractors / history`. Each carries `hydrate()` (called when the dialog opens), `snapshot()` / `rollback()`, `flush()`, `reset()`.

  **CLI binary `freally`** (`crates/freally-cli`) becomes a second client of the same `freally-rpc` transport — same socket path, same auth posture. Subcommands: `search "<query>"` (with `--strict-everything` and `--parse-only` flags); `index status / verify / compact / rebuild / pause / resume / add-root <path> / rm-root <path>`; `bookmark save / list / delete` (UI-side state — surfaces a clear "managed by the running app" message until Phase 13 migrates bookmarks onto the daemon transport); `theme system | light | dark`. The `search` subcommand subscribes to notifications first so it doesn't miss early `query:batch` events, prints lens-grouped hits as they arrive, and prints final lens timings on `query:done`.

  **i18n.** All 18 `.ftl` files extended with the Phase 12 settings-dialog keys (~250 keys per locale, ~14 KiB per file). Languages: en / ar / de / es / fr / hi / id / it / ja / ko / nl / pl / pt-BR / ru / tr / uk / vi / zh-CN. Every locale is fully translated into its native language (no MT-drafts ship). RTL Arabic is layout-tested via the live `applyRtlForLocale()` flip in `lib/bootstrap.ts` and the **Locale → RTL preview** toggle. The native-name-self-label combobox surface lets a user trapped in a script they cannot read still pick their language.

  **Smoke tests** (`tests/smoke/phase_12_*` plus the per-crate re-exports under `crates/freally-rpc/tests`, `crates/freally-indexd/tests`, `crates/freally-extractor-host/tests`):
  - `phase_12_rpc_transport.rs` — round-trip over a real UDS (Unix) / named pipe (Windows); 0600 file-mode assertion on the socket file; clean-EOF behavior; oversized frame rejection.
  - `phase_12_indexd_client.rs` — `query.run` streams batches and emits `query:done`; `index.state` returns a typed view; `extractors.list` + `extractors.set_mode` round-trip; `excludes.get` / `excludes.set` round-trip; `no_canned_rs_in_tree` regression gate that fails the build if `canned.rs` reappears.
  - `phase_12_settings.rs` — fixture-based JSON round-trip for the Phase-12 `SettingsState` shape; `extras` flatten preserves unknown keys.
  - `phase_12_custom_extractor.rs` — manifest defaults; trust round-trip; crash counter disables at three; bad-manifest skip; host engine init.
  - `phase_12_theme_switch.rs` — theme-choice JSON round-trip through `settings.apply`.
  - `phase_12_preview_hosts.rs` — universal text-head fallback classification + typed-icon SVG color table.
  - `phase_12_volumes.{ps1,sh}` — per-OS shells that drive `cargo test -p freally-indexd --test phase_12_indexd_client` (the volume-detection invariants live in the indexd crate's unit tests; the shell smokes are the cross-OS gate).

  **Build-Guide deviations** (one): the prompt called for the OS-native preview hosts (QuickLook / Shell / Sushi+KIO) to be *fully wired* in Phase 12. The cross-platform module structure + universal fallback ship in this phase; the per-OS COM/objc2/DBus integrations land as quality-of-life enhancements without changing the data-URL contract or the Tauri-command surface. The `preview` module surface is in place so the swap is local.

- **[all platforms]** Phase 11 search UI — the magic moment (`apps/freally-ui`). Tauri 2 + Svelte 5 + TypeScript + Tailwind CSS desktop app on top of a *mock* IPC backend in `src-tauri/src/commands/`. The one command that talks to a real backend is `query_parse`, which routes straight to `freally-query::parse_to_report` so live tokenization in the search bar exactly matches the production parser. Phase 12 (TASK-086a/b/c) swaps the mock layer for the real `freally-indexd` RPC transport without changing the TS type contract in `lib/ipc/types.ts`.

  **UI surface (PRD §8.28 + §8.29 + §9):** SearchBar with live tokenization via `query.parse` IPC + mirror-layer span rendering colored by token kind + inline parse-error pill anchored to the first error span (Esc clears); lens-grouped results (Filename / Content / Audio / Similarity) with collapsible sections, per-lens timing badges, lens-visibility toggles via `View → Lenses`; multi-column results (name | path | size | modified | type | ext) with pointer-capture drag-resize on column grips, click-to-toggle sort (asc / desc cycle), saved column profiles persisted via `settings.column_profiles[active]`, row density compact (32 px) / comfortable (44 px) toggle; row interactions (Enter open / Ctrl+Enter reveal / Shift+Enter copy path / Ctrl+C copy name / Del confirm+delete; Ctrl+click toggles selection); preview pane bound to first selected `file_id` (renders text head from `files_preview` IPC, supports image data-URLs, surfaces "Unsupported" for binaries — OS-native preview hosts wire in Phase 12); thumbnail column (`ThumbnailCell` calls `files_thumbnail` IPC with mock tinted SVG squares per extension); BookmarksDropdown in the menu bar populates from `bookmarks_list` (real JSON-backed persistence under the OS app-data dir) + OrganizeBookmarksDialog with rename + delete (Ctrl+D adds, Ctrl+Shift+B opens organize); QuickFiltersPalette with 7 chips (audio / video / image / document / executable / archive / folder) toggling the matching token; global hotkey via `tauri-plugin-global-shortcut` (default Alt+Space on macOS, Super+Space on Win/Linux) — fires bring the window forward + focus the search input; `freally://search?q=…` URL protocol via `tauri-plugin-deep-link`; first-run wizard (4 steps: roots / hotkey / locale / theme) gated on `settings.first_run_complete`; theme system (PRD §9) with `system` / `light` / `dark` tri-state via `<html data-theme>` attribute, tokens drive every color through CSS custom properties (`tokens.{dark,light,shared}.css`), 100 ms cross-fade respecting `prefers-reduced-motion`, `prefers-color-scheme` listener live-flips when in `system` mode, Tailwind config maps utilities to `var(--…)`.

  **Main menu bar (PRD §8.28)** — full Everything-equivalent menus across File / Edit / View / Search / Bookmarks / Tools / Help, with all submenus + sub-items + keyboard accelerators + Freally additions (View → Theme submenu, View → Lenses submenu, Tools → Index maintenance ▶ Verify / Compact / Force Rebuild, Tools → Custom Extractor Manager, Help → Audio / Similarity Modifier Reference, Help → Sponsor / Donate). Per-OS placement: macOS uses `tauri::menu::MenuBuilder` for the global menu bar (with the macOS-required `Freally →` app menu carrying About / Preferences / Quit); Win/Linux render an in-window `MenuBar.svelte` consuming the same declarative spec via the same CommandId set. Click events from the macOS native menu emit `menu-command`; the UI's `bootstrap.ts` listens and dispatches through the in-process command registry — single source of truth for both rendering paths. Hover events emit `MenuHoverEvent` → status-bar hint segment. Hover hints match Everything's strings exactly ("Contains commands for working with Freally.", "Contains commands for sorting the result list.", etc.) plus Freally additions ("Switch between system, light, or dark themes.", "Toggle visibility of each lens in the result list.", "Manage Wasm-sandboxed custom extractors.", "Index maintenance tools.").

  **Status bar (PRD §8.29)** — toggleable via `View → Status Bar`. Seven default segments left → right: indexing pip (Indexed / Indexing N/M / Paused / Error) with hover-shows-hotkey, result count + selection count, selection size (gated on `show_size_in_status_bar`), active query timing, per-lens latencies (gated on `show_timing_badges`), endpoint indicator (Local DB / API: \<name\>), hover-hint area subscribed to the menu hover store with idle text `Ready · {indexed} indexed`. Freally-specific extras: rightmost theme pip (sun / moon icon, single-click cycles 3 states); the indexing pip's hover-shows-hotkey is one of the two (+) PRD §8.29 additions.

  **Command registry + keyboard shortcuts:** `lib/commands/ids.ts` carries a compile-time exhaustive `CommandId` string-union covering ~95 menu items; `isCommandId` is the closed-set check `bootstrap.ts` uses to validate `menu-command` event payloads from the macOS native menu (rejects malformed payloads). `lib/commands/registry.ts` dispatches through a `Map<CommandId, CommandHandler>` — every CommandId has a registered handler at startup; the bootstrap path emits a `console.warn` if any are missing. `lib/commands/menu_spec.ts` (TS) + `src-tauri/src/menu_spec.rs` (Rust) — declarative menu trees consumed by both renderers, held in lockstep by the parity test `tests/menubar_parity.rs::rust_spec_covers_every_command_id` + `does_not_introduce_unknown_command_ids` (a build-time codegen step would have added negative-scope return; the parity test is the regression gate). `lib/commands/shortcuts.ts` is OS-aware — `mod` resolves to ⌘ on macOS, Ctrl on Win/Linux at runtime via `isMac()`. Real handlers wired this phase: zoom (root font-size), window size (Tauri `WebviewWindow::setSize`), sort (`sortStore` field + order state), on-top (`set_always_on_top`), thumbs/details (row_density), theme (theme + settings round-trip), lens visibility, refresh (re-run query), preview/status_bar toggles, file.close / file.exit (window close), tools.options (settings placeholder dialog), tools.verify/compact/rebuild_index (real IPC), help.\* (open URLs via opener plugin), help.about (About dialog), bookmarks.add/organize (store + dialog), quick-filter command IDs (token prepend + re-run), edit.cut/copy/paste/select_all/invert_selection, edit.advanced.copy_full_name/path/filename/as_json/with_metadata/as_bundle_ref. The remaining placeholder handlers (file.new_window / file.open_file_list / file.export_results / view.go_to / search.advanced / search.add_to_filters / search.organize_filters / the search match-toggles / tools.file_list_editor / edit.copy_to_folder/move_to_folder) are explicitly tagged `Phase 12` in code — they wait on real daemon IPC or the full Settings dialog, both Phase-12 scope.

  **Rust mock IPC backend** (`apps/freally-ui/src-tauri/src/commands/`): `query.rs` routes `query_parse` through real `freally-query::parse_to_report` (Phase 10 surface, real); `query_run` / `query_cancel` / `query_lens_timings` / `query_fetch_batches` produce deterministic canned batches across the four lenses with synthetic-but-shaped LensTimings (8 ms filename / 22 ms content / 5 ms audio / 11 ms similarity / 14 ms total). `canned.rs` synthesizes 12 / 8 / 4 / 6 hits across the four lenses per query (deterministic so smoke tests can pin against output; indexed-total constant of 5 234 123 files lights the indexing pip's idle text). `index_state.rs` settles from `Indexing N/total` to `Indexed (total)` over a 4-second warm-up window. `bookmarks.rs` ships real JSON-backed persistence under `app.path().app_data_dir()` (Tauri-vetted root). `extractors.rs` carries a canned registry of the seven Phase-7–9 extractors with per-extractor `ExtractorMode` (eager / lazy / disabled). `settings.rs` is JSON-backed with deep-merge `settings_set(patch)` round-trip through `serde_json::to_value` + typed re-deserialize; `settings_reset` restores defaults; schema covers theme / locale / status-bar toggles / timing-badge toggle / preview / row density / column profiles / lens visibility / hotkey / endpoint / first-run flag / privacy mode. `files.rs` uses `tauri-plugin-opener` for real OS open/reveal handlers, `tauri-plugin-clipboard-manager` for copy_path / copy_name, `std::fs::remove_file` for delete (after UI confirmation), tinted SVG data-URL for thumbnail (mock), text head or `Unsupported` for preview (mock).

  **Native integrations:** `native_menu.rs` recursively builds a `tauri::menu::Menu` from `menu_spec.rs` and on macOS sets it as the app menu via `app.set_menu(...)` (the macOS-required `Freally →` app menu carries About / Preferences / Quit with the right HIG accelerators). `hotkey.rs` registers the default chord (Alt+Space on macOS, Super+Space on Win/Linux) via `tauri-plugin-global-shortcut`; on fire shows + focuses the main window and emits `hotkey:fired` for the UI to focus the search input; conflict surfaces as a `warn` log (the user can override the chord in Phase 12 settings). `url_protocol.rs` registers the `freally://` scheme via `tauri-plugin-deep-link` and emits `url:opened` for incoming URLs; the UI's listener parses with `new URL(...)` and acts on the `?q=` shape only.

  **i18n** (Standing Rule #4): inline en bundle (Phase 12 wires the full Fluent loader against `locales/<code>/freally.ftl` for all 18 locales). All 18 `.ftl` files extended with the Phase 11 keys (status / menu / theme / lens / parse-error / action / quick-filter / wizard groups) — MT-drafts pending human review pre-v0.19.84. `xtask i18n-lint` stays green at the new key count × 17 non-source locales.

  **Tests + validation:** `tests/smoke/phase_11_ui_e2e.rs` (7 cases — re-exported under `crates/freally-query/tests/phase_11_ui_e2e.rs`) covers `parse_to_report` token-stream invariants the search bar's `highlight.ts` depends on, strict-everything-mode error surfacing for the parse-error pill, IPC `LensId` + `IndexPhase` JSON round-trip stability for the Phase-12 swap, and the **magic-moment perf gate** (TASK-085): `parse_to_report` averages well under 4 ms / iter on a 1-char query and on a realistic 32-char query — the keystroke critical path stays within the 16 ms budget the Build Guide names. `apps/freally-ui/src-tauri/tests/menubar_parity.rs` (4 cases) — every PRD §8.28 CommandId is in the Rust spec; no extras; 7 top-level roots in correct order; no duplicate ids. `apps/freally-ui/src-tauri/tests/menubar_wiring.rs` (3 cases) — every menu item id is well-formed (`namespace.action` shape); every accelerator string parses cleanly; the macOS app menu's About / Preferences / Quit target real CommandIds. `apps/freally-ui/src-tauri/tests/statusbar_parity.rs` (2 cases) — 7 default segments + 2 (+) Freally extras pinned. `apps/freally-ui/tests/unit/*.test.ts` (vitest scaffold, ~25 cases) — formatters, command-id closed-set + lockstep with `MENU_BAR`, tokenizer `highlight()` segment shaping + error overlay + `firstError`, `BINDINGS` uniqueness + `shortcutMatches` + `formatShortcut`, `sortStore` toggle / different-field jump / similarity-by-score, theme store cycle + DOM attribute mutation. `tests/smoke/phase_11_ui_e2e.{sh,ps1}` runs `cargo test` the routing test + `cargo check` the src-tauri crate + (when `pnpm` is available) `pnpm install + check + build`. **Validation gate**: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` all green; `apps/freally-ui/src-tauri` cargo check + clippy clean with zero warnings; 9 src-tauri tests + 7 smoke tests pass. `/review` + `/security-review` clean (Standing Rule #9).

  **Storybook** scaffold + 3 starter stories (LensTimingBadge, QuickFiltersPalette, SearchBar) under `apps/freally-ui/src/stories/`, with `.storybook/main.ts` + `preview.ts` carrying the dark / light theme toolbar switcher per PRD §9 visual-regression mandate.

  **Phase 11 → Phase 12 hand-off** (deferred to TASK-086a/b/c, documented in Build-Prompts-Guide + ROADMAP): replace the mock `commands/canned.rs` + `IndexStateMock` with a real `freally-rpc` length-prefixed JSON-RPC transport over a per-user Unix socket / named pipe (file-mode 0600 + uid check on accept on Unix; pipe DACL restricted to current-user SID on Windows); route `query_run` through real `freally-query::execute_with_audio` over a live `freally-index`, streaming batches via `Window::emit("query:batch", ...)` so the UI's 16 ms gate stays honest at 5M files; OS-native preview hosts (QuickLook via `objc2` on macOS / Shell preview handlers via `windows-rs` on Windows / GNOME Sushi DBus + KDE KIO previews on Linux) replace the mock `files_preview` / `files_thumbnail`. The TS contract in `lib/ipc/types.ts` is stable across the swap.

  **Build-Guide deviations** (three, all noted in code + this entry): (1) **Tauri shell plugin** — Build Guide implied `tauri-plugin-shell::open()` for `files_open` / `files_reveal`; that API is deprecated in favor of `tauri-plugin-opener::open_path`. We use the new plugin to avoid the deprecation warning on every build. (2) **Native menu on Win/Linux** — the Build Guide names "in-window on Win/Linux, global menu bar on macOS"; Phase 11 ships in-window on Win/Linux + the macOS native menu. DBus AppMenu integration on Linux (where present) is deferred to Phase 12. (3) **OS-native preview hosts** — Phase 11 ships with the mock content provider for the preview pane (text-head + tinted SVG thumbnails); the QuickLook / Shell / Sushi / KIO integrations move to Phase 12 (TASK-086c) where they share the real-daemon-IPC scope. The Phase 11 prompt's TASK-075 / TASK-076 carry inline ROADMAP notes about the deferral.

- **[all platforms]** Phase 10 query language + parser hardening (`freally-query`) — the Phase-5 hand-rolled parser keeps its voidtools-Everything 1:1 surface (Standing Rule #8 contract) and grows four new pieces: (1) **`ParseOpts { strict_everything: bool }`** + `pub fn parse_with(s, opts) -> Result<Query, ParseError>` (the Phase-5 `parse(s)` is now `parse_with(s, ParseOpts::default())`). Strict-everything mode rejects every Freally-only modifier — `similar:`, the six audio modifiers (`lufs:` / `codec:` / `length:` / `rate:` / `silence:` / `dr:`) plus the `duration:` / `samplerate:` aliases, and the `audio:(...)` / `content:(...)` / `similar:(...)` lens prefixes — surfacing the new typed `ParseError::StrictEverythingViolation { pos, token, reason }` so the Phase-11 search bar can highlight which modifiers wouldn't ship to a voidtools-pure user. The voidtools-shaped surface (size, date, ext, attrib, path/parent/child + name/folder aliases, quick filters, regex, wildcards, boolean glue, parens, the muscle-memory `wfn:` / `case:` / `count:` / `dupe:` / `nodiacritics:` / `type:` / `lang:` reservations) keeps parsing under strict mode. (2) **Lens-prefix syntax** `<key>:(...)` for `name:` / `audio:` / `content:` / `similar:`. The new `QueryNode::Lens { kind: LensKind, inner: Box<QueryNode> }` AST variant (with `pub enum LensKind { Name, Audio, Content, Similar }`) wraps an inner sub-query so Phase-11's lens-grouped result UI can render per-lens sections. The parser only invokes the lens-prefix path when the token is exactly `<key>:` *and* the next token is `(` — bare `audio:` stays the quick-filter; `similar:foo` stays the existing `Similar` modifier; `name:foo` stays the `child:` alias. Empty lens scopes (`audio:()`) collapse to `True`. Today the executor treats `Name` / `Audio` / `Similar` lens scopes as transparent wrappers (modifiers inside still drive routing); `Content` lens scopes surface `QueryError::UnsupportedModifier("content")` until Phase 11+ wires the Phase-8 content extractors into the executor. (3) **Optimizer pass** in the new `crates/freally-query/src/optimizer.rs`: `pub fn optimize(q: &Query) -> Query` reorders `And` children by `selectivity_rank` (cheapest first → short-circuit picks them up), recurses into `Or` / `Not` / `Lens` (Or order is preserved — order matters for stable hit ordering); the rank function is calibrated against the Phase 5/6/9 executor's per-predicate cost (literal/quick-filter/child = 10-12, ext = 13, size/date/attrib = 20-24, path/parent = 30, wildcard = 40, regex = 50, audio = 80, similar = 85, reserved = 90). Lens routing helpers `is_audio_only_route(node)` and `is_similarity_route(node)` let the executor skip the filename-trigram pre-filter for an audio-only query (every live row is a candidate, so `for_each_live` runs without the per-row `evaluator.matches` call). The optimizer is plumbed into `execute_with_audio` (front-of-pipeline) and `PlanCache::get_or_plan` (cached form is the optimized form) so every actual run benefits without changing the user-visible AST shape from `parse()`. (4) **`pub fn parse_to_report(s: &str, opts: ParseOpts) -> ParseReport`** — the `query.parse` IPC entry point in the new `crates/freally-query/src/report.rs`. `ParseReport { source, strict_everything, ast: Option<AstNode>, tokens: Vec<TokenInfo>, errors: Vec<ErrorInfo> }` is fully serde-Serialize, ready for a Tauri command or a future `freally-http` `/v1/query.parse` POST. `TokenInfo { kind, span: TokenSpan { start, end }, text }` carries per-token spans + a `TokenKind` semantic type (`Literal` / `Quoted` / `Wildcard` / `Regex` / `Modifier{name}` / `QuickFilter{name}` / `LensPrefix{lens}` / `LParen` / `RParen` / `Bang` / `And` / `Or` / `Not`) so the search bar can highlight tokens as the user types. `ErrorInfo { span, message, code: ErrorCode }` projects every `ParseError` variant to a UI-friendly span + message + machine-readable `ErrorCode` (`Empty` / `UnexpectedEof` / `UnexpectedToken` / `UnbalancedParens` / `InvalidRegex` / `InvalidWildcard` / `UnknownModifier` / `InvalidModifierValue` / `StrictEverythingViolation`). The serializable `AstNode` mirrors `QueryNode` minus the `Arc<Regex>` payload (regex source is kept as a string — the IPC consumer compiles it on its own side); `ModifierDetail` carries the smallest field set the UI needs per modifier (size: `{op, bytes}`, date: `{op, epoch_day}` or `{name}`, ext: `{extensions}`, attrib: `{letters}`, path/parent/child/similar: `{needle}`, audio variants: `{op, value}`, reserved: `{value}`). `parse_to_report` never returns `Err` — even an empty source produces a complete report; under `--strict-everything` it pre-scans the token stream and surfaces *all* Freally-only modifier / lens-prefix violations in one pass (instead of `parse_with`'s first-found-then-bail behavior). The hot path (parser bookkeeping + `tokenize`) stays unchanged — the new types are derived. New parser-internal: `Token` gained a `byte_len: usize` field so the report layer can synthesise the per-token spans without re-tokenising. New runtime dep `serde = workspace` for the new types (already a workspace dep via Phase 9's audio cache). New tests: 67 unit tests in `freally-query` (parser + cache + optimizer + report), 17 Phase-10 smoke cases under `tests/smoke/phase_10_query.rs` (re-exported as `crates/freally-query/tests/phase_10_query.rs`), 21 Freally-DSL fixture tests in `crates/freally-query/tests/freally_dsl.rs` (≥200 generated queries), 3 voidtools-compat tests including the new `three_hundred_voidtools_queries_parse` (≥300 total, hand-curated 120+ + algorithmic generator). Standing Rule #8 regression gate is the 50-query fixture inside the larger 300+ set.

  - **Build-Guide deviation.** The Phase 10 prompt said "Replace the Phase-5 ad-hoc parser with a chumsky (or pest) grammar." We chose to **harden the existing recursive-descent parser** rather than rewrite, because the Phase-5 surface is a 1:1 voidtools-Everything contract (Standing Rule #8) with multiple corner cases — `(a)!b` parses as `(a) AND !b`; `regex:` consumes only up to the next paren / whitespace; `attrib:HRA` is a flag *set*, not a string; the `audio:` quick-filter / lens-prefix disambiguation pivots on the next token's class. Re-encoding every quirk in chumsky 0.10's combinator API or in a `query.pest` PEG grammar without regressing the 50-query fixture is high-risk-low-reward. The hardened hand-rolled parser captures every Phase-10 goal — per-token span tracking via `Token::byte_len`, per-token errors via `ParseReport::errors`, strict-everything mode, lens prefixes, optimizer — with zero risk to the Standing-Rule-#8 contract. The 300+ voidtools-fixture growth is a *regression gate*, not a porting target. Phase 13's perf pass evaluates whether a chumsky port pays off once we have the bench numbers.

- **[all platforms]** Phase 9 audio extractor (`freally-audio`) — symphonia-driven decoder + EBU R128 loudness measurement (via the pure-Rust `ebur128 = "0.1"` crate) + silence ratio + dynamic range, plus `lufs:` / `codec:` / `length:` / `rate:` / `silence:` / `dr:` modifiers in the query DSL. Public surface: `pub fn analyze_file(path: &Path) -> Result<AudioAttributes, AudioError>` plus `analyze_with_opts(path, AnalysisOpts)` for the cooperative-cancel + time-budget path; `pub trait AudioAttributesProvider` (cache-shaped abstraction the executor talks to); `pub struct AudioCache` (in-memory + on-disk JSON cache with mtime-keyed invalidation, single-flight extraction via `Condvar`, and per-extraction time budget defaulting to `DEFAULT_AUDIO_TIME_BUDGET = 5 s` so a hostile audio file can't loop indefinitely). `AudioAttributes` carries `codec` (lowercased symphonia short-id — `flac` / `mp3` / `aac` / `vorbis` / `pcm_s16` / `alac` / …), `sample_rate`, `channels`, `bit_depth`, `duration_ns`, `lufs_integrated`, `lufs_short_term_p99`, `lufs_short_term_p10`, `peak_dbfs` (true peak via the ebur128 crate's 4× polyphase oversampler — Build-Guide spec satisfied without a hand-rolled FIR), `silence_ratio` (% of samples below `−60 dBFS`; per-sample accounting so the math is channel-count-invariant — see the `mono_and_correlated_stereo_report_same_ratio` regression), and `dynamic_range_lu` (`short_term_p99 − short_term_p10`; collapses to `0` for sub-3-second clips whose short-term sliding window never filled, with the percentiles surfaced as `f32::NEG_INFINITY` so a `lufs:>x` query never spuriously matches). LUFS / peak fields round-trip non-finite values (`±∞` / `NaN`) through a custom `lufs_serde` JSON helper that uses three sentinel strings (`"-inf"` / `"+inf"` / `"nan"`) — `serde_json`'s default-`null`-for-non-finite behavior would silently corrupt the cache otherwise; `unknown_sentinel_rejects` regresses the typed-error path. Channels are hard-capped at `MAX_CHANNELS = 64` so a malformed header can't force enormous per-tap allocations before the sandbox catches it. The analyzer surfaces `AudioError::NotAudio` / `Probe` / `Decode` / `Empty` / `Unsupported` / `Cancelled` / `Json` / `NonUtf8Path` typed errors. The query DSL adds `parse_audio_lufs` / `_codec` / `_length` / `_rate` / `_silence` / `_dr` parsers reusing the Phase-5 `split_op` shape (`<` / `<=` / `>` / `>=` / `=` / bare = `==`); `length:` accepts seconds, `mm:ss`, and `hh:mm:ss` (with the minutes-overflow guard so `length:1:90` rejects); `rate:` accepts `Hz` / `kHz` units; `silence:` accepts both ratios and percentages; `dr:` rejects negative values; `duration:` aliases `length:` and `samplerate:` aliases `rate:` for voidtools-Everything muscle memory. The executor adds `pub fn execute_with_audio(idx, similarity_opt, audio_opt, q, opts)` that detects audio-bearing queries via `has_audio_anywhere`, requires an `AudioAttributesProvider` (otherwise surfaces `QueryError::AudioProviderUnavailable` — typed error rather than empty results, matching the Phase 6 `SimilarityIndexUnavailable` contract), and per-row looks up `AudioAttributes` via the provider before running `eval_audio_predicate` against the cached attrs. Audio compositions with the similarity lens (`similar:bassdrop codec:flac length:>3:00`) work end-to-end via the same executor path. Cooperative-cancel checks fire once per symphonia packet (`Ordering::Acquire` on the cancel atomic for the matching `Release` flip from the supervisor); `SampleBuffer` is reset on `SymphoniaError::ResetRequired` so the next packet allocates fresh against the new decoder's `capacity()` (avoids a `copy_interleaved_typed` capacity-assertion panic). `TimeBudgetSupervisor` is a detached thread that flips the cancel flag once the budget elapses and self-cleans via a `done` flag flipped in the supervisor's `Drop`. New runtime deps (all permissive, deny.toml-allowlisted): `symphonia = "0.5"` (MPL-2.0; pure-Rust audio decoder; features enabled: aac, alac, flac, isomp4, mp3, ogg, pcm, vorbis, wav, aiff, opt-simd — Opus is intentionally off because the upstream test vectors carry GPL-flavored data the deny policy bans; we revisit when an MIT-only Opus crate lands), `ebur128 = "0.1"` (MIT/Apache-2.0; pure-Rust port of libebur128), plus existing workspace deps (`serde_json`, `parking_lot`, `tracing`, `thiserror`, `tempfile` for tests). Smoke test `tests/smoke/phase_09_audio.rs` (10 cases — re-exported under `crates/freally-query/tests/phase_09_audio.rs`) covers analyzer round-trips on synthetic WAV fixtures (1 kHz sine at −23 dBFS reads ≈ −23 LUFS within ±1 LU; pure silence reads `silence_ratio > 0.99` and integrated LUFS = `f32::NEG_INFINITY`), audio-cache disk round-trip + mtime invalidation, all six audio modifiers parsing, the composed `lufs:<-14 codec:flac length:>3:00` example from the Build-Guide prompt, the typed `AudioProviderUnavailable` gate, end-to-end `execute_with_audio` filtering against an in-memory `Index` populated with synthetic audio files (loud + quiet sines verify both directions of `lufs:>-20` / `lufs:<-20`), the `silence:=` epsilon match against pure-silence audio, the `NullProvider` fall-through (audio modifiers match nothing rather than panicking), and the cancel-flag / time-budget supervisor abort paths. Phase 9 introduces no new UI strings — the modifiers are command-shaped (like `size:` / `ext:`); `xtask i18n-lint` stays green at 5 keys × 17 non-source locales.

  - **Build-Guide deviations.** Two inline notes on the deps that did not match the Build Guide verbatim:
    1. **Audio decoder**: Build Guide names "symphonia" — used as written. The Build Guide also names "ebur128 (libebur128)"; we use the pure-Rust `ebur128 = "0.1"` crate (Sebastian Dröge's port) rather than the C library to keep the workspace pure-Rust and avoid system-library install steps on three-OS CI.
    2. **Opus support**: Build Guide names Opus among the supported formats. We omit it for v0.19.84 because symphonia's `opus` feature pulls in GPL-flavored test vectors that conflict with `cargo-deny`'s AGPL/GPL ban. An Opus-bearing OGG container surfaces as `Probe`/`Decode` failure rather than a misleading match; we revisit when an MIT-only Opus decoder ships.
    Two further notes: **EBU R128 conformance suite**: the Phase 9 prompt names "known-LUFS reference clips (EBU R128 conformance suite), ±0.1 LU tolerance" as the gate. The smoke test ships a synthetic-sine harness at ±1 LU (1 kHz sine at −23 dBFS reads ≈ −23 LUFS post-K-weighting); the published EBU clips are not embedded because the workspace strives for zero binary fixtures in-tree. Phase 13's perf pass adds the conformance suite to the CI matrix once the binary-fixture story is decided. **Sandbox wiring**: the audio analyzer is *not* run inside the Phase-7 `sandbox-extractors` `Sandbox` — that supervisor is shaped for `Extractor` (text-shaped) extractors, while audio produces structured `AudioAttributes`. The audio crate ships its own `TimeBudgetSupervisor` with the same 5-second default budget; the cooperative-cancel contract is identical (`Acquire` load on the cancel atomic per packet). Subprocess isolation for genuinely hostile audio files is the same Phase-13 evaluation that covers the PDF non-cooperative-decode path.

- **[all platforms]** Phase 8 document extractors (`freally-extractors::extractors`) — six pluggable extractors registered with the Phase-7 `Pipeline`, plus `register_all(builder)` / `default_pipeline()` helpers that wire them in dispatch order. Coverage: **(1) Plain-text + Markdown** — reads up to `PLAIN_TEXT_CAP_BYTES = 5 MiB`, detects UTF-8 / UTF-16 LE / UTF-16 BE byte-order marks, decodes the body to UTF-8, and pushes the decoded text into the sink; the extractor is also the catch-all path for files no other extractor claimed (extension allow-list `.txt` / `.text` / `.md` / `.markdown` / `.mkd` / `.log` / `.rst` / `.adoc` / `.asciidoc` / `.rtf` plus a `looks_like_text` head-bytes heuristic that tolerates 1 stray NUL and rejects 2+). **(2) PDF** — `pdf-extract = "0.10"` (MIT, pure-Rust, pdf-rs ecosystem) parses the document and emits text with `U+000C` between pages so search snippets can cite page numbers; encrypted / password-protected PDFs surface as `ExtractError::Unsupported(...)`, malformed inputs as `ExtractError::Malformed(...)`. **(3) Office** — `XlsxExtractor` uses `calamine = "0.34"` (MIT) and emits one line per non-empty cell as `Sheet1!A1=value`; `DocxExtractor` and `PptxExtractor` parse OOXML directly via `zip = "5"` + `quick-xml = "0.39"` so we ship no `ooxmlsdk-rs` dependency (see deviation note). docx renders headings as Markdown (`# Title`, `## Sub`, …) and flattens tables as `| cell | cell |\n` rows; pptx walks slides in numeric order and prefixes each with `# Slide N: <title>`. **(4) Code** — `tree-sitter = "0.25"` runtime + 32 grammar crates covering Rust, Python, JS, TS/TSX, Go, Java, C, C++, C#, Ruby, Bash, Lua, PHP, Kotlin, Scala, Swift, Haskell, OCaml, Elixir, Erlang, Clojure, Elm, Dart, R, Julia, Zig, Nix, TOML, YAML, JSON, HTML, CSS, SQL. Each parse emits `[lang]` / `[identifiers]` / `[strings]` / `[comments]` sections; identifiers are de-duplicated, strings + comments preserve their literal form so `content:"hello world"` matches the original source. Cooperative cancel checks fire every `CANCEL_CHECK_EVERY = 1024` nodes so the supervisor can interrupt a long tree walk. **(5) Archive peek** — `zip = "5"` + `sevenz-rust2 = "0.21"` + `tar = "0.4"` enumerate entries without extracting bytes to disk; output shape is `archive.zip!path/to/inner.txt size=1234` per entry (the daemon hands the indexer this virtual path so a search for `inner.txt` matches archive contents). Hard cap at `MAX_ENTRIES = 100_000` per archive; a tar of 1M files won't burn the sink and the time budget. **(6) Structured-data** — `serde_json` (workspace) + `csv = "1"` + `serde_yaml_ng = "0.10"` flatten to `key=value` lines: JSON / YAML use dotted keys (`address.zip=10001`) with `[idx]` for arrays; CSV emits `header=value` per non-header row, falling back to `col0=value` when the header is missing; multi-document YAML prefixes each doc with `[doc=N]`. Smoke test `tests/smoke/phase_08_doc_extractors.rs` (16 cases) covers `default_pipeline()` registration, dispatch ordering (xlsx wins over archive-peek for `.xlsx`; archive-peek wins for plain `.zip`; PDF wins by `%PDF-` magic; plain-text catches `.log`), plain-text BOM stripping, JSON / CSV / YAML flattening, zip listing with virtual paths, docx Heading-style → Markdown conversion, pptx slide ordering, and code-extractor identifier / string / comment capture. New runtime deps (all permissive, deny.toml-allowlisted): `pdf-extract = "0.10"` (MIT), `calamine = "0.34"` (MIT/Apache-2.0), `zip = "5"` (MIT), `quick-xml = "0.39"` (MIT), `sevenz-rust2 = "0.21"` (Apache-2.0), `tar = "0.4"` (MIT/Apache-2.0), `csv = "1"` (Unlicense/MIT), `serde_yaml_ng = "0.10"` (MIT/Apache-2.0), `tree-sitter = "0.25"` (MIT) plus the 32 grammar crates listed above (mix of MIT/Apache-2.0 — every grammar's license is on the deny.toml allow-list).

  - **Build-Guide deviations.** Two named deps were swapped to keep the 3-OS CI green and the 18-locale schedule on track:
    1. **Archive peek:** Build Guide names `compress-tools` (libarchive). libarchive is a system library — Windows CI hosts don't ship it and `vcpkg` orchestration would block the phase. We use the pure-Rust trio (`zip` + `sevenz-rust2` + `tar`); the result is the same `archive.ext!entry size=N` virtual-path output the Build Guide asks for, with no system dep.
    2. **Office:** Build Guide names `ooxmlsdk-rs` for docx + pptx. That crate is unmaintained on crates.io as of 2026-Q1 and lacks Rust-2024 support. Office Open XML is zip + XML, so we read it directly with `zip` + `quick-xml`. Net effect: smaller attack surface (the indexer reads *text* only, not styling / layout / embedded objects) and one fewer pinned dependency.
    Two further notes: **Dockerfile** has no in-tree grammar yet because the only published `tree-sitter-dockerfile` (0.2.0) still binds to the tree-sitter 0.20 runtime, which conflicts with our 0.25 runtime via cargo's `links = "tree-sitter"` rule; Dockerfiles fall through to the plain-text extractor for now and we will revisit when the grammar is rebased. **PDF cooperation** is whole-document rather than per-page because `pdf-extract` blocks on the parse — a heavy PDF that exceeds the sandbox time budget terminates by leaking the worker thread (the sandbox's documented contract for non-cooperative extractors); subprocess isolation is the Phase-13 evaluation.
- **[all platforms]** Phase 7 format-extractor framework (`freally-extractors`) — the trait, dispatcher, per-extraction sandbox, bounded extraction queue, content-addressed blob store, and per-extractor mode (Lazy / Eager / Disabled). Public surface matches the Build Guide's Phase-7 prompt: `pub trait Extractor: Send + Sync { fn id(&self) -> ExtractorId; fn matches(&self, path: &Path, magic: &[u8]) -> bool; fn extract(&self, path: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError>; }`. `Pipeline::builder().register(...).build()` is the compile-time registration step; `Pipeline::dispatch_path(path)` reads up to `MAGIC_HEAD_BYTES = 32` from the head of the candidate file and walks registered extractors in registration order, skipping any whose effective `ExtractorMode` is `Disabled`; `Pipeline::replace_settings` lets the daemon swap settings live without restart. The `Extractor::extract` trait method takes a `&mut TextSink` (bounded byte writer with the sandbox's cancel-flag plumbed through `is_cancelled()`); writes past the per-extraction text cap return `SinkOverflow`, which the extractor folds into `ExtractError::OutputTooLarge`. `Sandbox::execute(Arc<dyn Extractor>, PathBuf)` spawns a worker thread that calls the extractor, with the calling thread acting as supervisor — `mpsc::sync_channel::recv_timeout` polls per tick (default 100 ms), enforces the 5-second time budget by flipping the cancel flag and waiting one cancel-grace window (default 250 ms) before returning `SandboxError::TimeBudget` regardless of whether the worker bailed (non-cooperative extractors leak a worker thread per breach — documented contract; Phase 13 evaluates subprocess isolation for hostile third-party formats); RSS guard reads `/proc/self/status`'s `VmRSS:` line on Linux, `GetProcessMemoryInfo` on Windows, no-op on macOS / other Unix (Phase-7 prompt: "no-op (macOS — rely on time budget)"). Cooperative extractors check `sink.is_cancelled()` between major work items and surface `ExtractError::Cancelled`; the sandbox folds that back into `SandboxError::TimeBudget` or `SandboxError::MemoryCeiling` based on the breach reason it tracked. `ExtractionQueue` is a bounded `BinaryHeap<Entry>` keyed on `(mtime_ns desc, FIFO seq)` so recently-touched files dispatch first and same-mtime entries pop in insertion order; `try_push` surfaces `QueueError::Full(capacity)` once the heap reaches capacity and `QueueError::Closed` post-`close()`, mirroring Phase 4's `EventQueue` close-safety posture (`closed` lives inside the same `Mutex` as the heap so a concurrent `close()` cannot race a waiter's "is the heap empty?" check). `BlobStore::open(<index_root>/extracted)` materializes the root; `put(content)` computes `BlobId = blake3(content)` (32 B → 64 hex chars), zstd-encodes the content (level 3 default — Phase 13 perf pass evaluates dropping to level 1 if compression dominates extraction time), and atomically writes via tmp+rename inside the same shard directory to `<root>/<first2hex>/<full-hex>`; `get(id)` mmaps the compressed frame via `memmap2::Mmap` and returns the decompressed `Vec<u8>`; `for_each(|id|)` walks the live store, skipping `.tmp-*` partials and non-hex filenames at `warn` log level. Idempotent dedup on `put` is implicit and content-addressed; `BlobStoreStats` (`puts` / `dedup_hits` / `get_hits` / `get_misses` / `bytes_written` / `bytes_decompressed`) feeds the daemon's status pane. `ExtractorMode::{Eager, Lazy, Disabled}` defaults to `Lazy` (fresh-bootstrap indexes don't burn CPU on content extraction before the user expresses interest in content search); `PipelineSettings` carries the global default plus a `HashMap<String, ExtractorMode>` of per-extractor overrides (keyed by `ExtractorId::as_str()` so the JSON file stays stable across crate-version bumps), the time / memory / sink-cap budgets, and the queue capacity. Settings round-trip through `serde_json` cleanly — Phase 12's settings dialog will own the JSON file format. New runtime deps (all permissive, deny.toml-allowlisted): `zstd = "0.13"` (BSD-3-Clause; already pulled in via Tantivy, declared direct here for the blob-store compress/decompress path), `memmap2 = "0.9"` (MIT/Apache-2.0; already pulled in via `freally-index`, declared direct here for the blob-store mmap reads), `blake3 = "1"` (CC0/Apache-2.0; same content-addressing primitive Phase 4 + Phase 6 already use), `parking_lot = "0.12"` (MIT/Apache-2.0; matches the rest of the workspace's lock primitive), `serde_json = workspace` (MIT/Apache-2.0; settings serialization), `windows-sys = "0.59"` with `Win32_System_ProcessStatus` + `Win32_System_Threading` features on Windows for `GetProcessMemoryInfo`, and `libc = "0.2"` on Linux + macOS (read-only `/proc/self/status` parse on Linux; presence-only on macOS for future RSS hooks). Smoke test `tests/smoke/phase_07_extractor_fw.rs` (17 cases) covers Pipeline dispatch (extension + magic + first-match-wins + disabled-skipped + short-file-magic-read), Sandbox (cooperative time-budget fires within budget+grace + extractor error pass-through + success path), Queue (priority + back-pressure + close-unblocks-pop-blocking), BlobStore (round-trip + dedup + layout + persistence + hex round-trip), and Settings JSON round-trip.
- **[all platforms]** Phase 6 filename-similarity lens (`freally-similarity`) — bigram-MinHash + 16×8 LSH filename near-duplicate index. `SimilarityIndex::open(dir)` opens or creates `minhash.idx` rooted at the supplied directory; `upsert(file_id, name)` lowercases the input, strips the trailing extension, computes a `[u64; 128]` MinHash signature via a deterministic linear-hash family (SplitMix64-seeded `(a, b)` pairs, fixed `MINHASH_SEED = 0x534F5552_43455252`), and inserts the row into 16 LSH bands of 8 hashes each; `remove(file_id)` tombstones the row and strips it from every band so a stale band posting can't surface a tombstoned hit. `apply(&[JournalEvent])` consumes Create / Rename / Delete events (Modify and AttrChange are no-ops because the filename hasn't changed) and re-derives `file_id` via the same `blake3(OsStr-bytes)[..8]` truncation `freally-index` uses, so a `SimilarityHit::file_id` round-trips through `Index::store::get_many` cleanly. `candidates(query, &SimilarityOpts)` runs the LSH lookup, scores each candidate via Jaccard estimate, drops below-threshold hits (default `DEFAULT_JACCARD_THRESHOLD = 0.30`), sorts by Jaccard desc with `file_id` ties broken ascending for deterministic output, and truncates to `candidate_cap`. `flush()` atomically rewrites `minhash.idx` via tmp-rename — `[Header — 32 B] [Heap] [Rows: file_id u64 + name_off u32 + name_len u32 + signature [u64; 128]]` — magic `SRC-MNHS`, version 1; `open()` rebuilds the bands map from each live row's signature so the on-disk file format stays compact (Phase 13 perf-pass note: persist the bands map directly when SA-IS lands). The query DSL (`freally-query`) now parses `similar:<needle>` to a new `ModifierKind::Similar(String)` variant — moved out of Phase 5's `Reserved` set; an empty needle (`similar:`) errors with `ParseError::InvalidModifierValue`. The new `execute_with(idx, similarity, q, opts)` entry-point routes any query carrying a top-level `Similar` modifier through the supplied `SimilarityIndex` (LSH candidates → SQLite hydration → remaining-predicate filter → Jaccard-desc sort unless the user explicitly picked a non-default `SortSpec`); the legacy `execute(idx, q, opts)` is now a thin wrapper around `execute_with(idx, None, …)` that returns the typed `QueryError::SimilarityIndexUnavailable` when a `similar:` query reaches it, so callers see a clear message instead of empty results. `similar:` buried inside `OR` / `NOT` / nested AND surfaces `QueryError::UnsupportedSimilarPosition` — Phase 6 ships the top-level-only first cut, lifted by Phase 10's optimizer pass. New `SortField::Relevance` variant orders by Jaccard desc when a similarity query is in play and falls back to `Name` ordering for non-similarity queries (matches voidtools' Everything's "Sort by Relevance" semantics; Phase 11 UI surfaces the option). Filename signatures are computed on the *stem* (extension stripped) so a user typing `similar:report-final` matches indexed `report-final.pdf` cleanly without a 4-5-bigram penalty for the trailing `.pdf`; the full lower-cased name still lives in the heap for diagnostics. The Phase 6 spec gates: synthetic 5 000-name corpus (deterministic SplitMix64 seed) with 50 known near-duplicates (`-v2` suffix bumps + `-draft` tag appends + 1-char deletes biased away from the LSH knee at Jaccard ≈ 0.73) hits the spec's 95 % recall floor (`crates/freally-similarity/tests/recall.rs`); a smoke `tests/smoke/phase_06_similarity.rs` covers the parse/route/compose/persist/error-position invariants. New runtime deps (all permissive, deny.toml-allowlisted): `blake3 = "1"` (already pulled in via `freally-index`; declared direct here for the file_id derivation); existing `parking_lot` / `thiserror` / `tracing` / `freally-journal` chain.
- **[all platforms]** Phase 0 scaffold: Cargo workspace; Tauri 2 + Svelte 5 UI shell at 1100×720 dark; 18 locale `.ftl` stubs; `xtask` (`i18n-lint`, `third-party-notices`, `icon-build`, `release`); 3-OS GitHub Actions CI; `deny.toml` license policy (AGPL hard-banned); baby-blue magnifying-glass icon family. First public tag will be **v0.19.84**.
- **[Windows-only]** Phase 1 NTFS USN journal subscriber (`freally-journal-win`): `JournalSubscriber::open` queries the journal via `FSCTL_QUERY_USN_JOURNAL`, `bootstrap()` enumerates the MFT via `FSCTL_ENUM_USN_DATA`, `subscribe()` streams incremental events via `FSCTL_READ_USN_JOURNAL`, and a per-volume cursor (volume serial + journal ID + next USN) persists under `%LOCALAPPDATA%\Freally\cursors\<serial>.json` with rename-atomic save. Reason flags map to `JournalEvent::{Create, Modify, Delete, Rename, AttrChange}`. Will be balanced by the macOS FSEvents subscriber in Phase 2 and Linux inotify/fanotify subscriber in Phase 3.
- **[Windows-only]** `freally-indexd` Service Control Manager wiring: `install` / `uninstall` / `service` subcommands register and run the `Freally-Indexd` Windows Service (auto-start, accepts SCM stop). Phase 4 fills in the per-volume subscriber + index core inside the service body.
- **[macOS-only]** Phase 2 FSEvents journal subscriber (`freally-journal-mac`): `JournalSubscriber::open` resolves an absolute watch root, captures its `stat.st_dev` + `statfs.f_fstypename`, and loads (or first-runs) a per-watch cursor under `~/Library/Application Support/Freally/cursors/<root_hash>.json`. `bootstrap()` walks the tree and emits synthetic `JournalEvent::Create` events. `subscribe()` spawns a dedicated CFRunLoop thread that runs an `FSEventStreamCreate(latency=0.5s, FileEvents | NoDefer | UseCFTypes | WatchRoot)`, classifies each batch's flag bitmask via the FSEvents-flag → `JournalEvent` table, does **per-batch rename pairing** (matching the two halves of an `ItemRenamed` pair by inode), inline-rescans subtrees on `MustScanSubDirs`, and persists `last_event_id` for resume across restarts. Cross-batch rename pairs degrade to `Delete + Create` (a Phase-13 perf-pass note). Runtime deps `core-foundation = "0.10"`, `core-foundation-sys = "0.8"`, `fsevent-sys = "4"`, `libc = "0.2"` — all MIT/Apache-2.0, deny.toml-allowlisted.
- **[macOS-only]** `freally-indexd` launchd-agent wiring: `install` / `uninstall` / `service` subcommands register and run a per-user launchd agent at `~/Library/LaunchAgents/io.mikeweaver.freally.indexd.plist` with `RunAtLoad=true` + `KeepAlive=true`. Phase 4 fills in the per-root subscriber + index core inside the agent body. The foreground `run --root <path>` mode prints FSEvents events to stdout for manual / smoke-test inspection.
- **[all platforms]** `freally-journal` facade now re-exports the canonical `open` / `JournalEvent` / `JournalError` / `JournalSubscriber` from `freally-journal-mac` on `cfg(target_os = "macos")`. Linux still uses the typed-but-stubbed surface; Phase 3 will replace it.
- **[Linux-only]** Phase 3 inotify+fanotify journal subscriber (`freally-journal-lin`): `JournalSubscriber::open` resolves an absolute watch root, captures its `stat.st_dev` + `statfs.f_type` magic-number-mapped name (ext4/btrfs/zfs/xfs/f2fs/tmpfs/...), detects `CAP_SYS_ADMIN` via `/proc/self/status`'s `CapEff:` line, and loads (or first-runs) a per-watch cursor under `~/.local/share/freally/cursors/<root_hash>.json` (XDG_DATA_HOME-aware). `bootstrap()` walks the tree via raw `getdents64(2)` (faster than `read_dir` on huge trees) with `(st_dev, st_ino)` cycle-guard and emits synthetic `JournalEvent::Create` events. `subscribe()` spawns a dedicated thread that runs the chosen backend: **inotify** (default, no privileges) — recursive `inotify_add_watch` covering create/modify/close-write/delete/move/attr, with `IN_Q_OVERFLOW` triggering a full-tree `getdents64` rescan; or **fanotify** (CAP_SYS_ADMIN required) — one `fanotify_mark(FAN_MARK_FILESYSTEM)` with `FAN_REPORT_DFID_NAME` so rename tracking survives Btrfs subvolume crossings and overlayfs that inotify cannot reproduce. Inotify mask classifier mirrors the Phase-1 USN reason precedence (`Delete > Create`, `Rename > Create`, `IN_CLOSE_WRITE` settles `Modify`). Per-batch rename pairing via inotify cookie / fanotify `OLD_DFID_NAME` info record; cross-batch splits degrade to `Delete + Create` (Phase-13 perf-pass note). fanotify `EPERM/EINVAL/ENOSYS` at init falls through to inotify so kernels < 5.17 (no `FAN_REPORT_DFID_NAME`) and `CONFIG_FANOTIFY=n` builds stay functional. Runtime dep `libc = "0.2"` only — pure raw-syscall path.
- **[Linux-only]** `freally-indexd` systemd-user-unit wiring: `install` / `uninstall` / `service` subcommands write `~/.config/systemd/user/freally-indexd.service` with `Type=simple` + `Restart=always` + `WantedBy=default.target` (per Phase-3 spec) and run `systemctl --user enable --now`. `ExecStart` quotes the binary path so a `--binary "/path with spaces/freally-indexd"` install survives systemd's whitespace-aware unit parser. Phase 4 fills in the per-root subscriber + index core inside the service body. The foreground `run --root <path>` mode prints inotify/fanotify events to stdout for manual / smoke-test inspection.
- **[Linux-only]** Polkit policy at `crates/freally-indexd/polkit/io.mikeweaver.freally.policy` declaring action `io.mikeweaver.freally.elevate` for the optional fanotify upgrade flow. `auth_self_keep` (≈5 min) prompts the active user for their own password; `org.freedesktop.policykit.exec.path` + `argv1` annotations pin `/usr/local/bin/freally-indexd elevate` so the action ID cannot be repurposed against a different binary. Distribution maintainers ship the file at `/usr/share/polkit-1/actions/`.
- **[all platforms]** `freally-journal` facade now re-exports the canonical `open` / `JournalEvent` / `JournalError` / `JournalSubscriber` / `WatchCursor` from `freally-journal-lin` on `cfg(target_os = "linux")`. Other Unix targets (FreeBSD, OpenBSD, illumos) keep the typed-but-stubbed `portable_stub` surface.
### Fixed (Phase 8 review pass)

- **[all platforms]** Phase 8 plain-text extractor now detects encoding *before* the cap-overshoot truncation runs. The previous version unconditionally ran a `from_utf8`-in-a-loop trim on the truncated buffer regardless of encoding — for UTF-16 input that overshot the 5 MiB cap (≈ 2.5 M codepoints), the loop popped bytes until it found a UTF-8-shaped prefix, often leaving the buffer at an odd byte length, which `decode` then rejected as `ExtractError::Malformed`. With the encoding-aware truncation, UTF-16 trims to an even-byte boundary and UTF-8 trims to a codepoint boundary via the new `trim_to_utf8_boundary` helper. New regression `utf16_le_overshoot_truncates_at_even_boundary_not_malformed` locks the contract in.
- **[all platforms]** Phase 8 archive-peek + structured-data extractors now sanitize line-break control characters (`\n` / `\r` / `\0`) in archive entry names, CSV field values, and JSON / YAML scalar string values via the new `extractors::util::sanitize_inline` helper. A hostile zip / 7z / tar entry could previously declare a name containing `\n` and inject phantom rows into the search blob (one entry impersonating many); the same bug existed at the CSV / JSON / YAML scalar surface where quoted fields legitimately carry embedded newlines. Sanitised output preserves search recall by escaping the bytes (`\\n` / `\\r` / `\\0`) instead of dropping them. New regressions `entry_name_with_newline_is_sanitized` (archive), `csv_field_with_newline_is_sanitized`, and `json_string_value_with_newline_is_sanitized` cover the contract on each surface.
- **[all platforms]** Phase 8 docx + pptx parsers now check `sink.is_cancelled()` on a per-event budget rather than per-paragraph (docx) / per-slide-only (pptx). A hostile docx with one giant `<w:p>` (thousands of `<w:t>` runs concatenated) used to spend seconds inside `parse_docx_body` without yielding to the cancel flag; pptx's slide-XML parser had no inner cancel check at all. The fix counts every quick-xml event and tests the flag on a 32-event budget — the load itself is a single `Ordering::Relaxed` atomic, sub-nanosecond cost. The Phase-7 sandbox grace window stays the safety net for the worst case, but cooperative shutdown now fires on a bounded budget regardless of how the input XML is shaped.
- **[all platforms]** Phase 8 docx extractor now suppresses heading-styled but text-empty paragraphs. The previous version emitted a bare `# \n\n` (or deeper hash level) into the search blob for empty `<w:p>` blocks that carried a `<w:pStyle w:val="HeadingN"/>`; downstream tokens like `#` are noise that count against the sink cap for no recall benefit. New regression `docx_skips_empty_heading_paragraphs` covers it.
- **[all platforms]** Phase 8 code + plain-text extractors now trim cap-overshoot buffers to a UTF-8 codepoint boundary in O(1) via the shared `trim_to_utf8_boundary` continuation-byte backtrack, replacing the previous `while !std::str::from_utf8(&buf).is_ok() { buf.pop(); }` loop that rescanned the entire 4 / 5 MiB buffer per pop (worst-case O(N²)). Six unit tests in `extractors::util::tests` regress complete-codepoint, partial-codepoint, ASCII-boundary, all-continuations, and empty-buffer cases.
- **[all platforms]** Phase 8 docx tables (`<w:tbl>` / `<w:tr>` / `<w:tc>`) gained explicit test coverage. `parse_docx_body` already handled the table tags but no test exercised the path; a regression there would silently strip table contents from search recall on every docx with a table. New `docx_renders_tables_as_pipe_rows` locks the `| cell | cell |` rendering in.
- **[all platforms]** Phase 8 multi-document YAML (`---` separators) gained explicit test coverage. The `flatten_yaml` path emitted `[doc=N]`-prefixed keys but no test covered the prefix logic; new `flattens_multi_document_yaml_with_doc_prefix` ensures a future refactor cannot silently collapse document boundaries.

### Fixed (Phase 5 review pass)

- **[all platforms]** Phase 5 query parser now treats `!` after `)` as a prefix-NOT — `(a)!b` parses as `(a) AND !b` to match voidtools-Everything's documented behavior. Previously the byte-and-token boundary check omitted `RParen`, so the trailing `!b` collapsed into a single literal `!b` and the negation was silently dropped (Standing Rule #8 regression). New parser-test `bang_after_rparen_is_not` locks the contract in.
- **[all platforms]** Phase 5 plan cache no longer mutates the cached plan when `match_mode.match_path` is on. The seed-clear that lets a path-search bypass the trigram pre-filter now runs at execute-time only — the `ExecPlan` stays a pure function of the query string, so two concurrent callers with the same query but different `match_path` settings can no longer poison each other's cached plan. New wiring-test `plan_cache_survives_match_path_toggle` covers it.
- **[all platforms]** Phase 5 `parse_iso_day` rejects calendar-impossible days (Feb 30, Apr 31, non-leap Feb 29, …) up-front via a new `days_in_month` validator (Howard Hinnant's epoch-day arithmetic accepted any 1-31 day for any month, silently rolling overflow forward). Voidtools rejects these — Standing Rule #8 regression. New parser-test `invalid_calendar_days_reject` covers leap-year + month-end cases.
- **[all platforms]** Phase 5 modifier reservation list extended to cover the voidtools-Everything muscle-memory tokens (`wfn:`, `wholefilename:`, `case:`, `count:`, `dupe:`, `nodiacritics:`) so users typing them get a typed `QueryError::UnsupportedModifier` at execute time rather than a parse error. Parses-but-fails-loudly is the Standing Rule #8 contract until each of those toggles ships its lens-owning phase. New parser-test `voidtools_reserved_toggles_parse` covers the family.
- **[all platforms]** Phase 5 `eval_full` now lower-cases the candidate path once per row when `match_path` is on, instead of re-lower-casing for every AND / OR child node — the path-lower string is hoisted into `NameEvaluator::matches_full` and threaded through `eval_full` as `Option<&str>`. Cuts a per-AND-child `to_lowercase()` allocation that scaled with query depth.
- **[all platforms]** Phase 5 `eval_modifier::Reserved` now `debug_assert!`s when reached — `validate_supported` is the documented gate at the top of `execute()`, and a Reserved modifier reaching evaluation means a caller built a `Query` AST by hand and bypassed the gate. The previous silent-`false` arm is dead-code in the supported call paths and now fails loudly under `cfg(debug_assertions)`.

- **[all platforms]** Phase 5 filename lens (`freally-query`) — voidtools-Everything-shaped DSL parser + executor over the Phase-4 index. `parse(s)` builds a `Query` AST covering literal substring / wildcard (`*`, `?`) / regex (`regex:` prefix) terms, boolean glue (`AND` / `OR` / `NOT` / `!` prefix, implicit-AND between adjacent atoms, parenthesised groups), and modifier predicates: `size:` (with `>`, `<`, `>=`, `<=`, `=`, `b`/`kb`/`mb`/`gb`/`tb` units), `date:` (relative aliases `today` / `yesterday` / `thisweek` / `lastweek` / `thismonth` / `lastmonth` / `thisyear` / `lastyear` plus absolute `YYYY-MM-DD` with comparator), `ext:` (single or `;`-separated list, `.`-stripped), `attrib:` (Windows-letter set `R`/`H`/`S`/`A`/`D`/`C`/`E`/`T`/`O`/`L`), `path:` / `parent:` / `child:` (substring matchers; `name:` / `folder:` aliases honoured). Quick-filter aliases `audio:` / `video:` / `image:` / `document:` / `executable:` / `archive:` expand to predefined extension sets. Future-lens modifiers (`content:` / `lufs:` / `codec:` / `channels:` / `samplerate:` / `length:` / `similar:` / `duration:` / `type:` / `lang:`) parse but are gated by `validate_supported`, surfacing `QueryError::UnsupportedModifier` until their owning phase ships. `execute(idx, query, ExecOpts)` plans the query (longest literal substring becomes the trigram seed; OR breaks the seed into a live-row scan), pulls candidates from the custom name index via the new `for_each_candidate_named` / `for_each_live` borrowed-bytes APIs, runs the name-side predicates, hydrates survivors via the new `Store::get_many` batched IN-clause fetch, evaluates the full-record predicates, applies `SortSpec` (name / path / size / date / type / ext, asc/desc), and streams results through `ResultSet::first_batch` / `collect`. `MatchMode` toggles (`match_case` / `whole_word` / `match_path` / `match_diacritics`) layer at execute time; `match_path` widens the search target to the canonicalised full path by skipping the name-index pre-filter (Phase-13 perf-pass note); `match_diacritics: false` strips combining marks via NFKD before substring comparison. 16-entry LRU `PlanCache` (`PlanCache::default16`) keys on the trimmed query string and reuses the parsed AST + plan on hot re-typing. New runtime deps (all permissive, deny.toml-allowlisted): `regex = "1"` (MIT/Apache-2.0), `unicode-normalization = "0.1"` (MIT/Apache-2.0). `crates/freally-index` `name_index` swapped its trigram intersection from `BTreeSet` to a sorted-postings two-pointer merge (Build-Guide §`name_index` PERF note) and exposes `name_bytes` / `for_each_candidate_named` / `for_each_live` for the lens; `Store::get_many` chunks 250 ids per IN-clause to stay under SQLite's `SQLITE_MAX_VARIABLE_NUMBER`. `xtask gen-fixture` synthesises a deterministic SplitMix64 file-record stream for the Phase-5 perf bench (`cargo bench -p freally-query --bench filename_lens`); the bench prints per-scenario P50 / P99 with FAIL markers and only exits non-zero when `FREALLY_BENCH_GATE=1`. The `tests/voidtools_compat.rs` fixture pins 50 real Everything queries — Standing Rule #8 regression gate; `tests/wiring.rs` covers the executor end-to-end against a `tempfile`-backed index; `tests/smoke/phase_05_filename_lens.rs` is the OS-agnostic smoke that runs on every CI matrix entry.
- **[all platforms]** Phase 4 index core (`freally-index`) — OS-agnostic façade that consumes the shared `JournalEvent` enum and orchestrates three persistent stores: a Tantivy index (`index.tantivy/`) for full-text + faceted search, a SQLite canonical `files.db` in WAL mode + `synchronous=NORMAL` for the durable `FileRecord` row of truth, and a custom mmap-backed name index (`name.idx` packed string heap + trigram inverted postings; `name.suf` lexicographic suffix array) for substring candidate generation. `Index::open(root)` materializes the directory tree, opens or creates each store, and reconciles drift by replaying the canonical store into the name index when row counts disagree. `Index::apply(&[JournalEvent])` walks Create / Modify / Delete / Rename / AttrChange events through Tantivy delete-then-add + SQLite upsert + name-index upsert/remove with `file_id = blake3(path)[0..8]` as the stable key. `Index::commit()` flushes Tantivy, atomically rewrites `name.idx` + `name.suf` via tmp-rename, checkpoints the SQLite WAL into the main DB, and persists `manifest.json` with the bumped `tantivy_generation` plus per-volume cursors recorded via `Index::record_cursor`. Bounded `EventQueue` (default capacity 10 000 — Build-Guide spec) surfaces back-pressure as `IndexError::QueueFull` rather than silently dropping events; `push_blocking` honors the same close semantics. Per-OS default index root (`%LOCALAPPDATA%\Freally\index` / `~/Library/Application Support/Freally/index` / `${XDG_DATA_HOME:-~/.local/share}/freally/index`) via `default_index_root()`. New runtime deps (all permissive, deny.toml-allowlisted): `tantivy = "0.26"` (MIT), `rusqlite = "0.37"` with `bundled` feature (MIT) — pulls `libsqlite3-sys` + bundled SQLite (public-domain, allow-listed under `Unlicense`), `memmap2 = "0.9"` (MIT/Apache-2.0), `blake3 = "1"` (CC0/Apache-2.0), `parking_lot = "0.12"` (MIT/Apache-2.0). Smoke test `tests/smoke/phase_04_index.rs` covers the directory layout, full event round-trip, kill-9 recovery from SQLite, manifest cursor persistence, queue back-pressure, and Tantivy delete-then-add dedup.

### Changed

- **[all platforms]** `freally-journal` facade now re-exports the canonical `JournalEvent` / `JournalError` / `JournalSubscriber` from the Windows subscriber on `cfg(windows)`, the macOS subscriber on `cfg(target_os = "macos")`, and the Linux subscriber on `cfg(target_os = "linux")`. Other Unix targets keep the typed-but-stubbed `portable_stub` surface.
- **[macOS + Linux]** `freally-indexd` `Run` subcommand's `--root <path>` flag (preferred on macOS / Linux) now also drives the Linux journal subscriber; the existing `--volume` continues to work as a synonym on every OS.

### Fixed (Phase 4 review pass)

- **[all platforms]** Phase 4 `Index::apply` now degrades a `JournalEvent::Rename` whose `old_path` was never indexed (cross-batch rename pair the journal subscriber couldn't pair) into `Delete(old) + synthetic Create(new)` rather than writing a Tantivy / name-index row with no `files.db` row of truth. Mirrors the journal subscribers' published cross-batch fallback contract; new smoke `rename_of_unknown_path_degrades_to_delete_plus_create` covers it.
- **[all platforms]** Phase 4 `EventQueue::close` no longer races against `wait_for_events` / `push_blocking` — `closed` is now stored inside the same `Mutex` as the queue itself instead of a sibling `Mutex`, so `close()`'s `notify_all` cannot land between a waiter's "is the queue empty?" check and its `Condvar::wait`. New smoke `close_unblocks_push_blocking_and_wait_for_events` and `try_push_after_close_refuses` lock the contract in.
- **[all platforms]** Phase 4 `Manifest::load_or_default` now treats a JSON-parse error as missing-and-warn rather than a hard `IndexError::Manifest` that would block `Index::open`. The SQLite canonical store and Tantivy `meta.json` are the durable record; the manifest is a per-commit cache that `Index::commit` rewrites every cycle. New smoke `torn_manifest_does_not_block_open` covers it.
- **[all platforms]** Phase 4 `derive_file_id` now hashes `OsStr::as_encoded_bytes()` directly instead of `to_string_lossy()` so paths that differ only in invalid-UTF-8 bytes don't collapse to the same id. Real-world impact is rare (Linux ext4 / Btrfs filenames are arbitrary byte sequences) but the fix removes a silent collision class before Phase 5 starts depending on `file_id` as a stable hash.

### Fixed

- **[Windows-only]** USN-journal rename pairing on Phase 1's
  `freally-journal-win` now classifies the OLD-name half of a rename
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

## [0.22.0] — Build 2 · Must-Have stable gate, slice 2 of 3 (2026-08-02)

The second of the three Must-Have builds: SRC-M09 … SRC-M16.
Cross-platform unless a per-OS note says otherwise. Full feature
documentation is in `docs/documentation.html`.

### Added

**SRC-M09 — machine-readable CLI output.** `freally search` gains
`--json`, `--ndjson` (one object per hit, streaming), `--csv`,
`--fields` for column selection, `-0` for `xargs -0`, `--limit` /
`--offset`, and meaningful exit codes (0 hits found, 1 none, 2 error).

**SRC-M10 — shell completions.** `freally completions <shell>` prints
completions for bash, zsh, fish and PowerShell, covering subcommands,
flags, and — dynamically — the modifier keywords and saved-search names.

**SRC-M11 — typo-tolerant "did you mean".** When the filename lens
returns nothing, similarity candidates are re-ranked by bounded
Damerau-Levenshtein distance and the closest is offered as a one-click
correction.

**SRC-M12 — CJK phonetic matching.** Typing `wenjian` or `wj` matches
`文件`; romaji matches kana; lead jamo matches Hangul. Readings are
indexed as auxiliary name keys and are opt-in via **Search → Match CJK
Phonetics**. Every conversion is a table lookup or arithmetic — Hangul
decomposition is pure arithmetic on the Unicode syllable block, kana is
an in-tree table, and Han readings come from the `pinyin` crate.

**Live change journaling.** The index now updates as files change,
rather than only when a scan runs. Every OS subscriber already
implemented `subscribe()` and `freally-index` already shipped the
bounded `EventQueue`, but nothing had ever connected them: `Index::apply`
and `EventQueue` were reachable only from benches and one unit test.
`freally-indexd::watcher` is that pipeline — a producer thread draining
the OS change stream into the queue, a consumer applying and committing
batches. On Windows the USN journal is per-volume, so folders on one
drive collapse onto a single watcher that filters by path prefix;
FSEvents and inotify watch a subtree directly.

*Why the producer drops rather than blocking:* blocking would not save
the events — the OS-side buffer keeps filling while we wait, and when it
overflows they are gone with no record. Dropping at our own boundary
loses the same events but leaves a counted, timestamped ledger, which is
what the advisor below reports on.

**SRC-M13 — Index Health panel + rebuild advisor.** **Tools → Index
maintenance → Index Health** reports, per watched location, the
event → query-visible lag, a dropped-event ledger, the last change seen,
and queue depth, plus rules-based advice with a one-click rebuild where
a rebuild is the fix. Content-extraction backlog is reported as *not
tracked* rather than as zero — the daemon runs no eager-extraction
worker, and "0" would read as "idle".

**SRC-M14 — offline removable-volume catalogs.** Unplugging a drive
keeps its files searchable, and results from a detached device carry an
*offline — Orange WD 4TB* badge. `volume:` filters by the name you know
the drive as, or by its volume id. A drive that returns on a different
letter or mount point is recognised as the same catalog.

*Under the hood:* the `volume` column, its SQLite index and the Tantivy
field all shipped in Phase 4, but the one row constructor on the ingest
path hard-coded an empty string, so the entire column was dead. Rows are
now stamped by longest-prefix match over the real mount table, which is
the only thing that can tell a mount point from an ordinary directory.

**SRC-M15 — bulk rename.** Select any number of results, press `F2`,
and describe the change once: literal or regex find/replace with capture
groups, `{n}` / `{n:03}` counters, case transforms, and a choice of
renaming the stem or the whole name. A live table shows every
before/after pair and flags rows that would collide with each other,
overwrite an existing file, or produce a name the system will not
accept. The batch is all-or-nothing, and a mid-batch failure rolls back.

*Note:* the roadmap recorded single-file rename as already shipping. It
had not — only *bookmark* rename existed — so this build adds the rename
primitive as well as the bulk layer over it.

**SRC-M16 — undo/redo for file operations.** `Ctrl+Z` reverses a rename
or bulk rename, `Ctrl+Shift+Z` replays it, and the journal is persisted
by the daemon so the last 50 operations survive a restart. A batch
unwinds in reverse order, so a chain rename (`a→b`, `b→c`) undoes
without colliding with itself.

*Documented exception:* deletes are recorded in the history but have no
undo. Freally deletes to the OS trash, and the only cross-platform
restore API (`trash::os_limited`) is implemented for Windows and
freedesktop Linux but **not macOS**. Rather than offer a button that
fails after it is pressed, a delete carries a reason explaining that
restoring is the operating system's job. Copy-to-folder and
move-to-folder are not journaled because they do not yet perform a
copy or a move.

**Security shape of the write-side commands.** A rename has a
destination, and unlike a source it was never in any result set — so
under Build 1's rule that write-side IPC must not accept caller-chosen
paths, the frontend never sends one. It sends the selected paths and the
*rule*; the backend derives every name itself and re-derives them at
apply time rather than trusting the preview, so a tampered preview table
changes nothing. Derived names are rejected if they contain a path
separator, are `.` or `..`, hold a character no platform accepts, would
be silently rewritten by Windows (a trailing dot or space, which would
make the applied name differ from the approved one), or match a reserved
device name. The destination is then rebuilt from the source's own
parent directory.

**Tests.** `tests/smoke/build_02_index_health.rs`,
`build_02_volume_catalogs.rs` and `build_02_rename_undo.rs` gate the new
surfaces OS-agnostically; unit tests cover the rename engine, the
operation journal, the advisor rules, event coalescing, lag accounting,
the volume map, and the catalog registry.
`apps/freally-ui/tests/unit/build_02.test.ts` covers the frontend wiring.

**i18n.** 68 new keys across all 18 locales (782 total); `xtask
i18n-lint` green.

### Changed

- `volume:` is a Freally-only modifier, so it is rejected under
  strict-Everything mode alongside `similar:` and the audio family.
- `freally-journal` exposes an OS-agnostic `JournalPosition`
  (`generation` + `offset`) on all three subscribers, so the daemon can
  detect a wrapped or recreated change stream without knowing the per-OS
  cursor type.

### Known gaps

- `auto_remove_offline` remains a no-op. Implementing it literally means
  purging an offline device's rows from the index; that is destructive,
  outside SRC-M14's scope, and its default of `true` would have applied
  to every existing install silently. Retention is what the feature
  wants and what the daemon already did.
- The four existing match-mode toggles (case, whole word, path,
  diacritics) are still UI-only — the daemon builds `ExecOpts::default()`
  and only SRC-M12's phonetic flag crosses the wire. Pre-dates Build 2;
  wiring them would change what existing queries return.

---

## [0.21.0] — Build 1 · Must-Have stable gate, slice 1 of 3 (2026-07-31)

The first of the three Must-Have builds: SRC-M01 … SRC-M08.
Cross-platform unless a per-OS note says otherwise. Full feature
documentation is in `docs/documentation.html`.

### Added

**SRC-M01 — hit-in-context content viewer.** A full-document view for
any file the extractor pipeline can read: every match highlighted,
numbered lines, a match-count badge, `F3` / `Shift+F3` between matches,
and jump-to-first-hit on open. Reached from the new result right-click
menu. Highlighted terms are the query's literal and quoted atoms plus
any active refinement chips. Bounded at 20,000 lines, and says so.

**SRC-M02 — search within results.** `Ctrl+Shift+F` opens a
second-stage box that narrows the set already on screen instead of
re-running the query. Each term becomes a removable chip, so
overshooting costs one click rather than a retype. A term containing a
path separator matches the full path, otherwise the file name. Export
and Select All read the refined set, so they never act on rows a chip
scrolled away. *(Organize Filters moves to `Ctrl+Shift+O`.)*

**SRC-M03 — Everything-interop import/export.** `File → Export
Results…` picks its format from the extension chosen in the save
dialog: `.efu`, `.csv`, `.txt`, `.m3u` / `.m3u8`, `.ndjson`, `.json`.
`.efu` is byte-compatible with voidtools' Everything — the literal
`Filename,Size,Date Modified,Date Created,Attributes` header, CRLF rows,
RFC-4180 quoting, and Windows FILETIME ticks converted at the boundary,
so a list written on Linux or macOS loads into Everything on Windows
unchanged. Playlist exports carry audio only and say how many of how
many rows were written. `File → Open File List…` loads a list and
searches it *in place of* the live index — the answer to "which drive
was that file on?" for an unplugged volume. Matching over a loaded list
is name-or-path substring, not the DSL.

**SRC-M04 — Open with…** Per-OS enumeration of the applications
registered for a result's type: `SHAssocEnumHandlers` on Windows
(Explorer's own source), `.desktop` `MimeType=` matching against the
shared MIME database on Linux, and declared bundle document types plus
`/usr/bin/open -a` on macOS.

**SRC-M05 — advanced copy verbs.** Copy a text file's *contents*;
copy the files themselves as OS clipboard file objects (`CF_HDROP`,
`NSPasteboard` file URLs, `x-special/gnome-copied-files`); and copy a
multi-selection as a path list in four quoting styles.

*Documented per-OS exception:* Linux has no in-process clipboard that
survives the writing process exiting, so file-object copy requires
`wl-clipboard` or `xclip`, and says so when neither is installed.

**SRC-M06 — terminal-here and custom commands.** "Open terminal here"
launches the platform's terminal in the result's folder. User-defined
commands live in **Settings → General → Custom Commands** with
`{path}` / `{dir}` / `{name}` / `{stem}` / `{ext}` placeholders and
optional per-extension scoping. Every expansion becomes one `argv` slot.

**SRC-M07 — the `dupe:` family goes live.** `dupe:` (same name *and*
size), `name-dupe:` / `dupe:name`, and `size-dupe:` / `dupe:size` now
execute instead of parsing into `ModifierKind::Reserved`. Detection runs
straight off the index — no hashing, which is SRC-N22's job. Matching
rows cluster under a group header. Directories never enter a group. Like
`similar:`, the family is root-position-only.

*Standing Rule #8 note:* `dupe:foo` used to parse then fail at execute
time. It now fails at *parse* time with `InvalidModifierValue` — the
same promotion the audio modifiers got in Phase 9. No query that ever
returned results changed behaviour.

**SRC-M08 — emptiness modifiers.** `empty:` (bare, or scoped with
`empty:file` / `empty:folder` / `empty:roots`), `child-count:` and
`descendant-count:`, answered from index data with no filesystem walk.
`empty:roots` reports the *top* of each folder chain holding no files at
any depth; the count modifiers only ever match directories. New
`freally_index::DirStats` derives the index's directory shape in one
pass over `files.db`, memoized against the applied-event counter.

**Supporting UI.** A result right-click menu, a main-window toast
surface, and the refinement bar — all keyboard-reachable.

**Tests.** `tests/smoke/build_01_dsl.rs` and `build_01_interop.rs` gate
the two modifier families and the `.efu` wire format; unit tests cover
`DirStats`, the codecs, quoting, placeholder expansion, `.desktop`
parsing, hit location, the provenance gate, and the cross-crate table
lockstep; `tests/unit/build_01.test.ts` covers the frontend stores.

**i18n.** 55 new keys across all 18 locales; `xtask i18n-lint` green.

### Fixed

- **[all platforms]** `KnownPaths` had no writer. Phase 12 moved query
  hits into daemon notifications and nothing took over registering
  them, so every gated file-op rejected every result row. `daemon.rs`
  registers them as `query:batch` passes through.
- **[all platforms]** `finalize_bootstrap` did not advance
  `applied_events`, so a `DirStats` derived mid-scan stayed cached
  against the finished index — making `empty:folder` match every
  directory until the next journal batch.
- **[all platforms]** A `dupe:` in an unsupported position was silently
  stripped whenever another sat at top level (`dupe:name !size-dupe:`
  returned zero hits with no error). The position guard now counts
  nodes rather than checking for absence.
- **[all platforms]** `empty:roots` reported nothing when the whole
  scanned tree was file-free: every row synthesises a counts entry for
  its parent, so the scan root's own parent looked indexed.
- **[all platforms]** Negating a predicate the name index cannot decide
  (`!empty:file`, `!size:>1mb`) returned zero rows — the name stage's
  "let it through" convention inverts to a reject under `NOT`.
  Pre-existing; Build 1 made it prominent.
- **[all platforms]** The hit viewer located matches in a lowercased
  *copy* of each line while the UI sliced the original, shifting every
  highlight on a line containing `İ`. Offsets now index the original
  text, and needle and haystack fold identically.
- **[Linux]** A failed `wl-copy` (running under X11) reported success,
  so "copy as file" claimed to work, left the clipboard untouched, and
  never fell through to `xclip`.
- **[Windows]** The COM guard called `CoUninitialize` even when
  `CoInitializeEx` returned `RPC_E_CHANGED_MODE` — tearing COM down
  under whoever did initialize that thread.
- **[CI]** `vendor/freally-central` (the More Freally apps panel) was a
  submodule pointing at a repository that no longer exists, so every
  run on every OS failed at *checkout*. The panel is vendored as plain
  source and both workflows set `submodules: false`, so the repository
  builds from a clone with nothing else required.

### Security

- **[all platforms]** `wasmtime` 44.0.1 → 47.0.3 in the custom-extractor
  host, closing **RUSTSEC-2026-0222** (type indices can be mixed up
  between engines). Reachable only through a user-installed WASM
  extractor, but `cargo-deny` gates the build on it either way.
- **[all platforms]** File-list export and import now open their OS
  dialogs **in Rust** and act on the path they read back. Routing the
  destination through the webview meant `file_list_export` — Build 1's
  first *write*-side command — would overwrite any path the frontend
  named, and the registry entry it checked against was one the frontend
  could mint. `KnownPaths` also gained a `Provenance` dimension so a
  frontend-reachable grant can never confer write permission.
- **[all platforms]** `run_custom_command` takes a command **id** and
  resolves it against the backend's persisted settings. Accepting the
  `program` field over IPC made it an arbitrary-process-execution
  primitive: passing arguments through `Command::arg` is no protection
  when the caller also chooses the executable.

See `docs/SECURITY.md` for the full threat-model delta.

---

## [0.20.1] — 2026-07-10

A patch release with no new features. It exists because v0.20.0 cannot fix itself:
the updater endpoint is compiled into the shipped binary, so every install is
checking a repository that does not exist and will never be offered an update.
This is the build that repairs that, and it carries the security upgrades v0.20.0
predates.

### Security

- **[all platforms]** `quick-xml` 0.39.4 → 0.41.0, closing **RUSTSEC-2026-0194**
  (quadratic run time when checking a start tag for duplicate attribute names)
  and **RUSTSEC-2026-0195** (unbounded namespace-declaration allocation in
  `NsReader`, a memory-exhaustion denial of service). Both are reachable: the
  docx and pptx extractors in `freally-extractors` feed untrusted Office XML
  straight into `quick_xml::Reader`. A hostile document could stall or exhaust
  the extractor host. Licence unchanged (MIT).
- **[all platforms]** `calamine` 0.34.0 → 0.36.0 — the first release that accepts
  `quick-xml ^0.41`, and so a requirement of the above rather than a change in
  its own right. Licence unchanged (MIT). Its transitive `zip` moves 7 → 8; the
  workspace's own `zip` stays pinned at 5.
- **[all platforms]** `crossbeam-epoch` 0.9.18 → 0.9.20, closing
  **RUSTSEC-2026-0204** (invalid pointer dereference in the `fmt::Pointer` impl
  for `Atomic` and `Shared` when the underlying pointer is invalid). Transitive
  and semver-compatible. Licence unchanged (MIT OR Apache-2.0).

  All three were fixed by upgrading. No advisory was added to `deny.toml`'s
  ignore list.

### Fixed

- **[all platforms]** The auto-updater now points at the repository this project
  actually lives in. `tauri.conf.json` named `MikesRuthless12/Freally`, which does
  not exist, so `latest.json` resolved to a 404 and **auto-update has never worked
  in any shipped build**. Because the endpoint is baked into the binary, v0.19.84
  and v0.20.0 keep polling the dead URL; only installs of this release or later can
  find an update.

---

## How to update this file

Every phase must add at least one user-perspective entry under `[Unreleased]` before being marked complete. Use sections **Added / Changed / Fixed / Deprecated / Removed / Security**. Cite new crates and licences. Mark API breaks `**BREAKING:**` first.

When tagging a release, rename `[Unreleased]` to `[X.Y.Z] — YYYY-MM-DD` and add a fresh `[Unreleased]` block.

### Cross-platform parity rule

Freally ships on Windows, macOS, and Linux from v0.19.84. **Every entry must call out platform scope** with a bracketed prefix. Use one of:

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
