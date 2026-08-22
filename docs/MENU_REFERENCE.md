# Menu reference

Every item in Freally Sourcerer’s menu bar and its keyboard shortcut,
grouped by menu.

> **Generated file — do not edit.** Produced by
> `scripts/gen-menu-reference.mjs` from
> `apps/freally-ui/src/lib/commands/menu_spec.ts`, which is the single
> source of truth both the in-window menu bar (Windows, Linux) and the
> native macOS menu read. Labels are resolved through
> `locales/en/freally.ftl`, so this says what the menu actually shows.
> Regenerate after changing the spec.

Shortcuts are shown as they appear in the menu. On macOS, `Ctrl` is
`Cmd` — the menu renders the platform's own modifier symbols.

A **(toggle)** item is a checkbox that remembers its state; a **(choice)**
item is one option in a group where picking one clears the others.

## File

Contains commands for working with Freally.

| Item | Shortcut |
| --- | --- |
| New Search Window | Ctrl+N |
| Open File List… | Ctrl+O |
| Close File List | — |
| Close | Ctrl+W |
| Export Results… | Ctrl+S |
| Export Index Bundle… | — |
| Exit | Ctrl+Q |

## Edit

Contains commands for editing search results.

| Item | Shortcut |
| --- | --- |
| Undo | Ctrl+Z |
| Redo | Ctrl+Shift+Z |
| Rename… | F2 |
| Cut | Ctrl+X |
| Copy | Ctrl+C |
| Paste | Ctrl+V |
| Copy to Folder… | — |
| Move to Folder… | — |
| Select All | Ctrl+A |
| Invert Selection | — |
| Advanced → Copy Full Name | — |
| Advanced → Copy Path | — |
| Advanced → Copy Filename | — |
| Advanced → Copy as JSON | — |
| Advanced → Copy with metadata | — |
| Advanced → Copy as Freally Bundle reference | — |

## View

Contains commands for manipulating the view.

| Item | Shortcut |
| --- | --- |
| Filters | — |
| Preview *(toggle)* | Alt+P |
| Sidebar *(toggle)* | — |
| Status Bar *(toggle)* | — |
| Extra Large Thumbnails | Ctrl+Shift+1 |
| Large Thumbnails | Ctrl+Shift+2 |
| Medium Thumbnails | Ctrl+Shift+3 |
| Details *(toggle)* | Ctrl+Shift+6 |
| Window Size → Small | Alt+1 |
| Window Size → Medium | Alt+2 |
| Window Size → Large | Alt+3 |
| Window Size → Auto Fit | Alt+4 |
| Zoom → Zoom In | Ctrl+= |
| Zoom → Zoom Out | Ctrl+- |
| Zoom → Reset | Ctrl+0 |
| Sort by → Name *(choice)* | Ctrl+1 |
| Sort by → Path *(choice)* | Ctrl+2 |
| Sort by → Size *(choice)* | Ctrl+3 |
| Sort by → Extension *(choice)* | Ctrl+4 |
| Sort by → Type *(choice)* | Ctrl+5 |
| Sort by → Date Modified *(choice)* | Ctrl+6 |
| Sort by → Date Created *(choice)* | Ctrl+7 |
| Sort by → Date Accessed *(choice)* | — |
| Sort by → Attributes *(choice)* | Ctrl+8 |
| Sort by → Date Recently Changed *(choice)* | Ctrl+9 |
| Sort by → Run Count *(choice)* | — |
| Sort by → Date Run *(choice)* | — |
| Sort by → File List Filename *(choice)* | — |
| Sort by → LUFS *(choice)* | Ctrl+L |
| Sort by → Length *(choice)* | Ctrl+Shift+L |
| Sort by → Similarity Score *(choice)* | — |
| Sort by → Ascending *(choice)* | — |
| Sort by → Descending *(choice)* | — |
| Go To | — |
| Refresh | F5 |
| Theme → System *(choice)* | — |
| Theme → Light *(choice)* | — |
| Theme → Dark *(choice)* | — |
| Lenses → Filename *(toggle)* | — |
| Lenses → Content *(toggle)* | — |
| Lenses → Audio *(toggle)* | — |
| Lenses → Similarity *(toggle)* | — |
| On Top → Never *(choice)* | — |
| On Top → Always *(choice)* | — |
| On Top → While Searching *(choice)* | — |

## Search

Contains search toggles.

| Item | Shortcut |
| --- | --- |
| Match Case *(toggle)* | Ctrl+I |
| Match Whole Word *(toggle)* | Ctrl+B |
| Match Path *(toggle)* | Ctrl+U |
| Match Diacritics *(toggle)* | Ctrl+M |
| Match CJK Phonetics *(toggle)* | — |
| Ignore Punctuation *(toggle)* | — |
| Ignore Whitespace *(toggle)* | — |
| Enable Regex *(toggle)* | Ctrl+R |
| Advanced Search… | — |
| Search Within Results | Ctrl+Shift+F |
| Add to Filters… | — |
| Organize Filters… | Ctrl+Shift+O |
| Everything *(toggle)* | — |
| Audio *(toggle)* | — |
| Compressed (Archive) *(toggle)* | — |
| Document *(toggle)* | — |
| Executable *(toggle)* | — |
| Folder *(toggle)* | — |
| Image *(toggle)* | — |
| Video *(toggle)* | — |
| Custom Filter… | — |

## Bookmarks

Contains commands for working with bookmarks.

| Item | Shortcut |
| --- | --- |
| Add to Bookmarks | Ctrl+D |
| Organize Bookmarks… | Ctrl+Shift+B |

## Tools

Contains tools commands.

| Item | Shortcut |
| --- | --- |
| Connect to FTP Server… | — |
| Disconnect from FTP Server | — |
| File List Editor… | — |
| Index maintenance → Index Health… | — |
| Index maintenance → Permission Health… | — |
| Index maintenance → Verify Index… | — |
| Index maintenance → Compact Index… | — |
| Index maintenance → Force Rebuild Index… | — |
| Custom Extractor Manager… | — |
| Options… | Ctrl+, |

## Help

Contains help commands.

| Item | Shortcut |
| --- | --- |
| Freally Help | F1 |
| Search Syntax | — |
| Regex Syntax | — |
| Audio Modifier Reference | — |
| Similarity Modifier Reference | — |
| Command Line Options | — |
| Freally Website | — |
| Check for Updates… | — |
| Report a Bug… | — |
| Sponsor / Donate | — |
| About Freally… | Ctrl+F1 |
