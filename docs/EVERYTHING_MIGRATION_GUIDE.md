# Coming from Everything

Freally Sourcerer is built to be the search you already know, on the two
operating systems voidtools' Everything has never run on. If you have
muscle memory for Everything, most of it transfers unchanged.

This guide covers what is the same, what is different, and the three
things that will surprise you.

---

## The short version

| | Everything | Freally Sourcerer |
| --- | --- | --- |
| Runs on | Windows | Windows, macOS, Linux |
| Indexes from | NTFS USN journal | NTFS USN journal, FSEvents, inotify/fanotify |
| Searches | file names and paths | names and paths, **plus** file contents, audio properties, and near-duplicates |
| Query syntax | Everything's | Everything's, plus extensions |
| File lists | `.efu` | `.efu` in and out, plus CSV, TXT, M3U/M3U8, NDJSON |
| Price | free | see the project page |

**Your queries keep working.** The parser accepts Everything's syntax, and
`--strict-everything` will tell you if a query relies on anything that is
not in Everything.

---

## What transfers unchanged

Everything's operators work exactly as you expect.

**Plain terms** are substring matches on the file name, ANDed together.
`report 2024` finds names containing both.

**Wildcards** `*` and `?`. A query containing either is matched against the
whole name rather than as a substring, same as Everything.

**Boolean glue**: space or `AND`, `|` or `OR`, `!` or `NOT`, and
parentheses for grouping.

**Modifiers**, with Everything's spellings and aliases:

| Modifier | Aliases | Example |
| --- | --- | --- |
| `size:` | | `size:>1mb`, `size:10kb..2mb` |
| `date:` | `dm:` `dc:` `da:` | `date:today`, `dm:2024` |
| `ext:` | | `ext:rs`, `ext:jpg;png` |
| `attrib:` | `attr:` `attributes:` | `attrib:h` |
| `path:` | | `path:projects` |
| `parent:` | `folder:` | `parent:src` |
| `child:` | `name:` | `child:readme` |
| `regex:` | | `regex:^report-\d+` |
| `empty:` | | `empty:folder` |

**Match toggles** — Match Case, Match Whole Word, Match Path, Match
Diacritics — are in Search, with the same meanings.

**Quick filters** (Everything, Audio, Compressed, Document, Executable,
Folder, Picture, Video) are the same set, in the same place.

---

## What is new

None of these exist in Everything. All are optional: a query that does not
use them behaves identically.

### Three more lenses

Everything searches names. Freally searches names **and**:

- **Content** — full-text inside documents. `content:invoice` searches
  document bodies, with a hit-in-context viewer (`F3` / `Shift+F3` to step
  through matches).
- **Audio** — measured properties, not tags. `lufs:>-20`, `codec:flac`,
  `length:>5m`, `rate:48000`, `silence:`, `dr:`.
- **Similarity** — `similar:<path>` finds near-duplicates of a file by
  content, not by name.

Results are grouped per lens, each with its own timing badge, so you can
see which lens answered and how fast.

### Extra modifiers

`similar:`, `volume:`, `dupe:` / `name-dupe:` / `size-dupe:`,
`child-count:`, `descendant-count:`, `name^:` and `name$:` (prefix and
suffix anchors), and the audio family above.

### Extra match modes

**Ignore Punctuation** and **Ignore Whitespace** — `foobar` finds
`foo-bar`, `myreport` finds `my report`. **Match CJK Phonetics** lets a
latin term match a CJK name through its reading, so `wenjian` finds `文件`.
All three are off by default.

### Offline volumes

Unplug a drive and its files stay searchable. Results from an offline
volume are marked as such, so you know why you cannot open one.

---

## The three things that will surprise you

### 1. The index is not instant on first run

Everything on NTFS reads the USN journal, which is why it indexes a whole
volume in seconds. Freally does the same on NTFS — but on macOS and Linux
there is no equivalent, so the first index is a directory walk. Expect
minutes rather than seconds on a large home directory the first time, and
seconds thereafter: after the initial walk, FSEvents and inotify/fanotify
keep it live the same way the USN journal does.

### 2. `Ctrl+Z` undoes file operations, not text

Everything has no undo. Freally journals every file operation it performs
— renames, bulk renames, deletes — and `Ctrl+Z` reverses them, across
sessions. Deletes go to the OS trash and are restored from it.

One platform caveat: restoring from the trash is implemented for Windows
and freedesktop Linux, but **not macOS**. A delete on macOS is still
recorded in the history so you can see it happened; the undo is offered
only where it can actually be honoured, rather than failing after you
press it.

### 3. Some things Everything does are deliberately not here

Everything is a Windows program and some of it is Windows-shaped. Where a
feature has no cross-platform meaning it is absent rather than faked on one
OS.

---

## Bringing your data across

### File lists (`.efu`)

`File → Open File List…` reads Everything's `.efu` directly. An opened
list becomes the search target until you close it, so you can search
inside a list the same way you search the index.

`File → Export Results…` writes `.efu` back out, so a list built in Freally
opens in Everything. It also writes CSV, TXT, M3U/M3U8 (audio results, for
a music player) and NDJSON (for a script).

`.efu` stores times as Windows FILETIME; Freally converts at the boundary
in both directions, so a round trip through either program preserves them.

### Bookmarks

Everything's bookmarks are stored in its own INI and are not imported.
Re-saving them is quick: type the query, `Ctrl+D`, name it. A saved
bookmark is recalled by typing `:` followed by its name.

### Settings

Not imported — the option sets only partly overlap and a wrong guess about
what you meant is worse than asking. The Settings dialog is laid out to
mirror Everything's, so the options you care about are roughly where you
expect them.

---

## Command line

Everything ships `es.exe`. Freally ships `freally`, and it speaks the same
query language:

```
freally search "report ext:pdf size:>1mb"
freally search --strict-everything "report ext:pdf"   # reject Freally-only syntax
freally search --json --fields path,size,modified "*.rs"
```

`--strict-everything` is the migration tool: it rejects any query that uses
something voidtools' Everything does not have, and names the token. Use it
to check that a saved query is portable before you rely on it in a script
that has to run against both.

Shell completions for bash, zsh, fish and PowerShell come from the same
keyword table the parser uses, so they cannot drift from what the parser
accepts.

---

## See also

- **[Documentation](documentation.html)** — the full user guide.
- **[Menu reference](MENU_REFERENCE.md)** — every menu item and its
  shortcut, generated from the menu itself.
