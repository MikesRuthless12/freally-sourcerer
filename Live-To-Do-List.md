# Freally Sourcerer — Live To-Do List

A living checklist that splits UI verification between the Playwright visual-smoke
gallery (`apps/freally-ui/e2e/`, run with `pnpm build && pnpm test:e2e` inside
`apps/freally-ui`) and the hands-on drills only a human on a real OS can perform.
The Playwright harness runs the built UI against a mocked IPC bridge
(`e2e/tauri-mock.js`), so it proves panels *render* — it never proves the Rust
daemon, the filesystem indexers, or OS integration actually work. Keep both
sections current: move items into the confirmed list only after a real Playwright
run passes, and check off human drills only after performing them on a real
machine.

## ✅ Playwright confirmed these render

> **Not yet populated.** The Playwright harness was scaffolded on 2026-07-19 and
> has **never actually run** against this app. Populate this section from the
> first real `pnpm test:e2e` run — list each spec/screenshot that passed. Do not
> pre-check items. The scaffolded specs cover (pending that first run):
>
> - [ ] Main window: menu bar, search bar, quick-filter chips, grouped lens results, status bar (`01-main-results`)
> - [ ] Typed query re-runs the search (`02-search-query`)
> - [ ] Settings dialog opens via Ctrl+, (`03-settings-dialog`)
> - [ ] About dialog opens via Ctrl+F1 (`04-about-dialog`)
> - [ ] Organize Bookmarks dialog with listed bookmarks (`05-organize-bookmarks`)
> - [ ] Connect-to-FTP-Server dialog via Tools menu (`06-connect-endpoint`)
> - [ ] First-run wizard for a fresh install (`07-first-run-wizard`)

## ☐ Human drills — features Playwright cannot verify

Backend, OS, and filesystem features. Each drill assumes a real build of the app
(`cargo build` + `pnpm tauri dev` or an installed release) on the named OS.

- [ ] **Real indexing of a volume** — Windows: launch the app on a machine with an NTFS `C:`, complete the first-run wizard adding `C:\Users\<you>`, wait for the status bar to reach "indexed", then search a filename you know exists deep in the tree and confirm it appears with correct size/modified date.
- [ ] **Live change journaling** — with the app open and index settled, create `journal-drill.txt` on the Desktop, wait a few seconds, search `journal-drill` and confirm the new file appears without a manual rescan; delete it and confirm it disappears.
- [ ] **Index verify / compact / rebuild** — Tools → Verify Index, then Compact Index, then Rebuild Index; confirm each completes without error and searches still return correct results afterwards.
- [ ] **Global hotkey** — set/keep the show-window hotkey (default `Super+Space`), focus another app, press the hotkey, and confirm the Freally window comes forward with the search input focused.
- [ ] **`freally://` deep link** — with the app running, open `freally://search?q=report` from a browser or `start freally://search?q=report` in a terminal; confirm the app receives it and runs the query.
- [ ] **Always on top** — View → On Top → Always; confirm the window stays above other apps; switch to While Searching and confirm it only floats while the query box is non-empty.
- [ ] **Window size persistence** — View → Window Size → Large, quit, relaunch; confirm the window reopens at the large size.
- [ ] **Export Results** — File → Export Results…, pick a location in the native save dialog, and confirm a JSON file with the current hits is written to disk.
- [ ] **Copy to clipboard** — select a result, Edit → Advanced → Copy Path, paste into a text editor and confirm the real path arrives via the OS clipboard.
- [ ] **Open / Reveal** — double-click a result to open it with the OS default app; use the context menu's reveal action and confirm Explorer/Finder/file manager opens at the file.
- [ ] **Connect to FTP/HTTPS endpoint** — run a second instance's HTTPS server (Settings → Network) on another LAN machine, Tools → Connect to FTP Server…, enter host/port/credentials, and confirm remote results arrive; then Disconnect and confirm the endpoint reverts to Local DB.
- [ ] **Tray icon behavior** — enable Run in background + tray icon; close the window and confirm the process stays alive with a tray icon that reopens the window.
- [ ] **Locale + RTL** — switch locale to العربية in Settings → Locale; confirm the UI flips to RTL and strings translate; switch back to English.
- [ ] **Audio lens on real files** — index a folder of MP3/WAV files, search `audio: lufs:>-20`, and confirm LUFS/codec/length badges show plausible measured values.
- [ ] **Content lens on real documents** — index a folder with PDFs/DOCX, search a phrase that appears only inside one document's body, and confirm the content lens finds it with a snippet.
- [ ] **Similarity lens on near-duplicates** — copy a large file, change one byte in the copy, rescan, and confirm the similarity lens pairs them with a high score.
- [ ] **Settings persistence across daemon restarts** — change a few settings (theme, default filters, an index option), quit fully (including tray), relaunch, and confirm every change survived.
- [ ] **Diagnostics bundle** — Settings → Logs & Debug → export diagnostics bundle; confirm a bundle file is written and contains logs (and no secrets).
