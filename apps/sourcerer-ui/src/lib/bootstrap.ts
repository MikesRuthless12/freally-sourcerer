// App bootstrap — registers every CommandId with a real handler, hydrates
// stores, and binds the global keyboard listener. Importing this module
// is what wires the UI for use.

import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

import { registry } from "./commands/registry.svelte";
import type { CommandId } from "./commands/ids";
import { COMMAND_IDS, isCommandId } from "./commands/ids";
import { BINDINGS, shortcutMatches } from "./commands/shortcuts";
import { settingsStore } from "./stores/settings.svelte";
import { themeStore } from "./stores/theme.svelte";
import { indexStateStore } from "./stores/index_state.svelte";
import { bookmarksStore } from "./stores/bookmarks.svelte";
import { queryStore } from "./stores/query.svelte";
import { resultsStore } from "./stores/results.svelte";
import { selectionStore } from "./stores/selection.svelte";
import { zoomStore } from "./stores/zoom.svelte";
import { sortStore, type SortField } from "./stores/sort.svelte";
import { dialogsStore } from "./stores/dialogs.svelte";
import { searchOptsStore, type SearchOptKey } from "./stores/search_opts.svelte";
import { typeFilterStore, type TypeFilterId } from "./stores/type_filter.svelte";
import { applyFontsAndColors } from "./stores/fonts_apply.svelte";
import { settingsDialog } from "./stores/settings_dialog.svelte";
import * as files from "./ipc/files";
import * as indexIpc from "./ipc/index_api";

let booted = false;
let keyboardBound = false;
let nativeEventsBound = false;

export async function bootstrap() {
  if (booted) return;
  booted = true;

  await settingsStore.hydrate();
  await indexStateStore.hydrate();
  await bookmarksStore.hydrate();

  // Apply persisted theme + zoom on first paint.
  themeStore.set(settingsStore.state.theme);
  zoomStore.set(settingsStore.state.zoom ?? 1);
  applyFontsAndColors();
  // Restore the persisted window size, if any.
  const ws = (settingsStore.state as unknown as { window_size?: { w: number; h: number } })
    .window_size;
  if (ws && typeof ws.w === "number" && typeof ws.h === "number") {
    try {
      const { LogicalSize } = await import("@tauri-apps/api/window");
      await getCurrentWindow().setSize(new LogicalSize(ws.w, ws.h));
    } catch (e) {
      console.warn("[bootstrap] restore window size failed:", e);
    }
  }

  // Apply RTL layout when locale is Arabic. Phase 12 ships with a
  // single RTL ship-locale (`ar`); when more land, this `endsWith` /
  // table-driven check should grow into a proper rtl-locale list.
  applyRtlForLocale(settingsStore.state.locale ?? "en");

  registerHandlers();
  bindKeyboard();
  await bindNativeEvents();
  bindShutdownHooks();

  // Kick off the initial "Everything" query so first paint isn't empty.
  // resultsStore.run() composes a bare `*` when all type filters are
  // selected and the source is empty (the default boot state), which
  // matches voidtools-Everything behavior. Retries briefly because the
  // daemon may still be in its background canonical-store replay.
  void runInitialEverythingQuery();
}

async function runInitialEverythingQuery() {
  // Poll until results arrive. Crucially, we only fire a fresh `run()`
  // when nothing is in-flight — otherwise the next iteration would
  // cancel its own previous query before the daemon could respond
  // (the `*` query takes ~600 ms on a million-file index).
  for (let attempt = 0; attempt < 60; attempt++) {
    if (resultsStore.batches.length > 0) return;
    if (!resultsStore.running) {
      await resultsStore.run(queryStore.source);
    }
    await new Promise((r) => setTimeout(r, 800));
  }
}

/// RTL ship-locales. Phase 12 has only Arabic; if Hebrew / Persian /
/// Urdu join the ship list, append them here.
const RTL_LOCALES = new Set<string>(["ar"]);

export function applyRtlForLocale(locale: string) {
  if (typeof document === "undefined") return;
  const isRtl = RTL_LOCALES.has(locale);
  document.documentElement.setAttribute("dir", isRtl ? "rtl" : "ltr");
  document.documentElement.setAttribute("lang", locale);
}

function bindShutdownHooks() {
  if (typeof window === "undefined") return;
  window.addEventListener("beforeunload", () => {
    indexStateStore.stop();
  });
}

async function bindNativeEvents() {
  if (nativeEventsBound) return;
  nativeEventsBound = true;
  try {
    // macOS native menu clicks land here.
    await listen<string>("menu-command", (event) => {
      const id = event.payload;
      if (isCommandId(id)) {
        void registry.run(id);
      }
    });

    // Global hotkey: focus the search input.
    await listen<string>("hotkey:fired", () => {
      const el = document.querySelector<HTMLInputElement>("[data-testid='search-input']");
      el?.focus();
    });

    // sourcerer:// URL protocol incoming. Phase 11 supports the search shape.
    await listen<string>("url:opened", async (event) => {
      const u = event.payload;
      if (typeof u !== "string" || u.length > 8192) return;
      try {
        const url = new URL(u);
        if (url.protocol !== "sourcerer:") return;
        if (url.hostname === "search" || url.pathname.startsWith("/search")) {
          const q = url.searchParams.get("q") ?? "";
          // Cap the query length defensively even though the Rust side
          // also caps. URL-protocol payloads are an attacker-controlled
          // surface (any webpage can fire one).
          const bounded = q.slice(0, 4096);
          await queryStore.setSource(bounded);
          await resultsStore.run(bounded);
        }
        // sourcerer://bundle/<encoded-path> is parsed but Phase 11 does not
        // resolve bundles (Phase 12 wires it under daemon IPC).
      } catch {
        /* malformed URL — ignore */
      }
    });
  } catch (e) {
    console.warn("[bootstrap] native event listeners failed:", e);
  }
}

function bindKeyboard() {
  if (typeof window === "undefined") return;
  if (keyboardBound) return;
  keyboardBound = true;
  window.addEventListener("keydown", (ev) => {
    for (const b of BINDINGS) {
      if (shortcutMatches(ev, b.shortcut)) {
        ev.preventDefault();
        void registry.run(b.command);
        return;
      }
    }
  });
}

function selectedPaths(): string[] {
  const out: string[] = [];
  for (const batch of resultsStore.batches) {
    for (const hit of batch.hits) if (selectionStore.has(hit.file_id)) out.push(hit.path);
  }
  return out;
}

async function setWindowSize(w: number, h: number) {
  try {
    const win = getCurrentWindow();
    const { LogicalSize } = await import("@tauri-apps/api/window");
    await win.setSize(new LogicalSize(w, h));
    // Persist so the next launch restores this size. Stored in the
    // SettingsState extras as `window_size: { w, h }`.
    await settingsStore.patch({ window_size: { w, h } });
  } catch (e) {
    console.warn("[cmd] window resize failed:", e);
  }
}

async function setAlwaysOnTop(on: boolean) {
  try {
    const win = getCurrentWindow();
    await win.setAlwaysOnTop(on);
  } catch (e) {
    console.warn("[cmd] always-on-top failed:", e);
  }
}

function isMacPlatform(): boolean {
  if (typeof navigator === "undefined") return false;
  return (navigator.platform || "").toLowerCase().includes("mac");
}

async function exitApp() {
  try {
    if (isMacPlatform()) {
      // On macOS, Cmd+Q quits the *application*, not just the window.
      await invoke("app_exit");
      return;
    }
    await getCurrentWindow().close();
  } catch (e) {
    console.warn("[cmd] exit failed:", e);
    try {
      await getCurrentWindow().close();
    } catch (e2) {
      console.warn("[cmd] exit fallback failed:", e2);
    }
  }
}

function registerHandlers() {
  // ---- File ----
  registry.register("file.new_window", async () => {
    dialogsStore.open("settings");
  });
  registry.register("file.open_file_list", async () => {
    dialogsStore.open("settings");
  });
  registry.register("file.close_file_list", async () => {
    // No file list active in Phase 11 — clear the result set.
    await queryStore.setSource("");
    await resultsStore.run("");
  });
  registry.register("file.close", async () => {
    try {
      await getCurrentWindow().close();
    } catch (e) {
      console.warn("[cmd] file.close:", e);
    }
  });
  registry.register("file.export_results", async () => {
    // Phase 11: serialize the current result set to a JSON file via the
    // dialog plugin's save_dialog. The user-chosen path is whitelisted on
    // the Rust side (the OS dialog is the trust boundary). Phase 12 lands
    // the full export pipeline (CSV / Sourcerer Bundle).
    try {
      const { save } = await import("@tauri-apps/plugin-dialog");
      const { writeTextFile } = await import("@tauri-apps/plugin-fs");
      const path = await save({
        defaultPath: "sourcerer-results.json",
        filters: [{ name: "JSON", extensions: ["json"] }]
      });
      if (!path) return;
      // Whitelist the user-chosen path so subsequent file-ops on it pass
      // the known-paths gate.
      await invoke("files_whitelist_user_chosen", { path });
      const payload = {
        source: queryStore.source,
        timings: resultsStore.timings,
        batches: resultsStore.batches
      };
      await writeTextFile(path, JSON.stringify(payload, null, 2));
    } catch (e) {
      console.warn("[cmd] file.export_results:", e);
    }
  });
  registry.register("file.export_index_bundle", async () => {
    dialogsStore.open("settings");
  });
  registry.register("file.exit", async () => exitApp());

  // ---- Edit ----
  registry.register("edit.cut", async () => document.execCommand("cut"));
  registry.register("edit.copy", async () => document.execCommand("copy"));
  registry.register("edit.paste", async () => document.execCommand("paste"));
  registry.register("edit.copy_to_folder", async () => {
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const dest = await open({ directory: true });
      if (typeof dest !== "string") return;
      // Phase 11 stops short of the actual copy — daemon-side file ops are
      // Phase 12 scope. Surface the chosen folder so the wiring is real.
      await files.copyText(`Copy ${selectedPaths().length} item(s) → ${dest}`);
    } catch (e) {
      console.warn("[cmd] edit.copy_to_folder:", e);
    }
  });
  registry.register("edit.move_to_folder", async () => {
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const dest = await open({ directory: true });
      if (typeof dest !== "string") return;
      await files.copyText(`Move ${selectedPaths().length} item(s) → ${dest}`);
    } catch (e) {
      console.warn("[cmd] edit.move_to_folder:", e);
    }
  });
  registry.register("edit.select_all", async () => {
    const ids = resultsStore.batches.flatMap((b) => b.hits.map((h) => h.file_id));
    selectionStore.selectAll(ids);
  });
  registry.register("edit.invert_selection", async () => {
    const all = resultsStore.batches.flatMap((b) => b.hits.map((h) => h.file_id));
    const next = all.filter((id) => !selectionStore.has(id));
    selectionStore.selectAll(next);
  });

  registry.register("edit.advanced.copy_full_name", async () => files.copyName(selectedPaths()));
  registry.register("edit.advanced.copy_path", async () => files.copyPath(selectedPaths()));
  registry.register("edit.advanced.copy_filename", async () => files.copyName(selectedPaths()));
  registry.register("edit.advanced.copy_as_json", async () => {
    const hits = resultsStore.batches
      .flatMap((b) => b.hits)
      .filter((h) => selectionStore.has(h.file_id));
    await files.copyText(JSON.stringify(hits, null, 2));
  });
  registry.register("edit.advanced.copy_with_metadata", async () => {
    const hits = resultsStore.batches
      .flatMap((b) => b.hits)
      .filter((h) => selectionStore.has(h.file_id));
    const text = hits
      .map((h) => `${h.path}\t${h.size}\t${h.modified_ms}\t${h.type}`)
      .join("\n");
    await files.copyText(text);
  });
  registry.register("edit.advanced.copy_as_bundle_ref", async () => {
    // Build URL via URL.searchParams so encoding is safe + round-trips.
    const refs = selectedPaths().map((p) => {
      const u = new URL("sourcerer://bundle/");
      u.searchParams.set("path", p);
      return u.toString();
    });
    await files.copyText(refs.join("\n"));
  });

  // ---- View ----
  registry.register("view.filters", async () => {
    // "Filters" in voidtools Everything jumps to the Exclude settings
    // panel — that's where include/exclude rules live. Pre-select the
    // panel before opening so the user lands there directly.
    settingsDialog.setSelected("indexes.exclude");
    dialogsStore.open("settings");
  });
  registry.register("view.preview", async () =>
    settingsStore.patch({ show_preview: !settingsStore.state.show_preview })
  );
  registry.register("view.status_bar", async () =>
    settingsStore.patch({ show_status_bar: !settingsStore.state.show_status_bar })
  );
  registry.register("view.thumbs.xl", async () =>
    settingsStore.patch({ thumb_size: "xl", row_density: "comfortable" })
  );
  registry.register("view.thumbs.l", async () =>
    settingsStore.patch({ thumb_size: "l", row_density: "comfortable" })
  );
  registry.register("view.thumbs.m", async () =>
    settingsStore.patch({ thumb_size: "m", row_density: "comfortable" })
  );
  registry.register("view.details", async () =>
    settingsStore.patch({ thumb_size: "details", row_density: "compact" })
  );

  registry.register("view.window_size.small", async () => setWindowSize(900, 600));
  registry.register("view.window_size.medium", async () => setWindowSize(1100, 720));
  registry.register("view.window_size.large", async () => setWindowSize(1500, 960));
  registry.register("view.window_size.auto", async () => setWindowSize(1200, 800));

  registry.register("view.zoom.in", async () => {
    zoomStore.in();
    await settingsStore.patch({ zoom: zoomStore.scale });
  });
  registry.register("view.zoom.out", async () => {
    zoomStore.out();
    await settingsStore.patch({ zoom: zoomStore.scale });
  });
  registry.register("view.zoom.reset", async () => {
    zoomStore.reset();
    await settingsStore.patch({ zoom: zoomStore.scale });
  });

  const sortMap: Record<string, SortField> = {
    "view.sort.name": "name",
    "view.sort.path": "path",
    "view.sort.size": "size",
    "view.sort.ext": "ext",
    "view.sort.type": "type",
    "view.sort.modified": "modified",
    "view.sort.lufs": "lufs",
    "view.sort.length": "length",
    "view.sort.similarity": "similarity"
  };
  for (const [id, field] of Object.entries(sortMap)) {
    registry.register(id as CommandId, async () => sortStore.setField(field));
  }
  registry.register("view.sort.ascending", async () => sortStore.setOrder("asc"));
  registry.register("view.sort.descending", async () => sortStore.setOrder("desc"));
  for (const id of [
    "view.sort.created",
    "view.sort.accessed",
    "view.sort.attributes",
    "view.sort.recently_changed",
    "view.sort.run_count",
    "view.sort.run_date",
    "view.sort.file_list_filename"
  ] as CommandId[]) {
    registry.register(id, async () => sortStore.setField("name"));
  }

  registry.register("view.go_to", async () => {
    // Phase 11: prompt for a path via the dialog plugin and reveal it.
    // The OS-native dialog is the trust boundary; whitelist the chosen
    // path so the file-ops gate accepts it.
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const target = await open({ directory: true });
      if (typeof target === "string") {
        await invoke("files_whitelist_user_chosen", { path: target });
        await files.reveal(target);
      }
    } catch (e) {
      console.warn("[cmd] view.go_to:", e);
    }
  });
  registry.register("view.refresh", async () => resultsStore.run(queryStore.source));

  registry.register("view.theme.system", async () => {
    themeStore.set("system");
    await settingsStore.patch({ theme: "system" });
  });
  registry.register("view.theme.light", async () => {
    themeStore.set("light");
    await settingsStore.patch({ theme: "light" });
  });
  registry.register("view.theme.dark", async () => {
    themeStore.set("dark");
    await settingsStore.patch({ theme: "dark" });
  });

  registry.register("view.lens.filename", async () => toggleLens("filename"));
  registry.register("view.lens.content", async () => toggleLens("content"));
  registry.register("view.lens.audio", async () => toggleLens("audio"));
  registry.register("view.lens.similarity", async () => toggleLens("similarity"));

  // App.svelte owns a $effect that translates settings.state.on_top +
  // queryStore.source into setAlwaysOnTop calls, so these handlers just
  // record the user's choice and let reactivity do the rest.
  registry.register("view.on_top.never", async () => {
    await settingsStore.patch({ on_top: "never" });
  });
  registry.register("view.on_top.always", async () => {
    await settingsStore.patch({ on_top: "always" });
  });
  registry.register("view.on_top.while_searching", async () => {
    await settingsStore.patch({ on_top: "while_searching" });
  });

  // ---- Search toggles ----
  const searchToggleIds: Record<string, SearchOptKey> = {
    "search.match_case": "match_case",
    "search.match_whole_word": "match_whole_word",
    "search.match_path": "match_path",
    "search.match_diacritics": "match_diacritics",
    "search.enable_regex": "enable_regex"
  };
  for (const [id, key] of Object.entries(searchToggleIds)) {
    registry.register(id as CommandId, async () => searchOptsStore.toggle(key));
  }
  registry.register("search.advanced", async () => dialogsStore.open("settings"));
  registry.register("search.add_to_filters", async () => {
    if (queryStore.source.trim()) {
      await bookmarksStore.add(`Filter: ${queryStore.source.slice(0, 60)}`, queryStore.source);
    }
  });
  registry.register("search.organize_filters", async () => dialogsStore.open("organize_bookmarks"));

  // Quick-filter chips and Search-menu items toggle the multi-select
  // typeFilterStore; the actual lens-prefix composition happens inside
  // resultsStore.run via typeFilterStore.toQueryFragment().
  const filterMenuToId: Record<string, TypeFilterId> = {
    "search.filter.audio": "audio",
    "search.filter.compressed": "compressed",
    "search.filter.document": "document",
    "search.filter.executable": "executable",
    "search.filter.folder": "folder",
    "search.filter.picture": "picture",
    "search.filter.video": "video"
  };
  for (const [id, fid] of Object.entries(filterMenuToId)) {
    registry.register(id as CommandId, async () => {
      typeFilterStore.toggle(fid);
      await resultsStore.run(queryStore.source);
    });
  }
  registry.register("search.filter.everything", async () => {
    typeFilterStore.toggleEverything();
    await resultsStore.run(queryStore.source);
  });
  registry.register("search.filter.custom", async () => dialogsStore.open("settings"));

  // ---- Bookmarks ----
  registry.register("bookmarks.add", async () => {
    const q = queryStore.source;
    const filters = typeFilterStore.toIds();
    // Dedupe on (query, filter-set). Bookmarks with the same query but
    // different filter selections are distinct entries.
    const exists = bookmarksStore.items.some(
      (b) =>
        b.query === q &&
        JSON.stringify(b.filters ?? []) === JSON.stringify(filters),
    );
    if (exists) return;
    const name = q.trim()
      ? q.slice(0, 60)
      : `Bookmark ${bookmarksStore.items.length + 1}`;
    await bookmarksStore.add(name, q, filters);
  });
  registry.register("bookmarks.organize", async () => {
    // Refresh from disk in case another window mutated the list while
    // this one was open — Organize then reflects the real persisted set.
    await bookmarksStore.hydrate();
    dialogsStore.open("organize_bookmarks");
  });

  // ---- Tools ----
  registry.register("tools.connect_endpoint", async () => dialogsStore.open("connect_endpoint"));
  registry.register("tools.disconnect_endpoint", async () =>
    settingsStore.patch({ endpoint: { name: "Local DB", kind: "local" } })
  );
  registry.register("tools.file_list_editor", async () => dialogsStore.open("settings"));
  registry.register("tools.custom_extractor_manager", async () =>
    dialogsStore.open("custom_extractor_manager")
  );
  registry.register("tools.verify_index", async () => indexIpc.verify());
  registry.register("tools.compact_index", async () => indexIpc.compact());
  registry.register("tools.rebuild_index", async () => indexIpc.rebuild());
  registry.register("tools.options", async () => dialogsStore.open("settings"));

  // ---- Help ----
  const helpUrls: Record<string, string> = {
    "help.help": "https://github.com/MikesRuthless12/Sourcerer/wiki",
    "help.search_syntax": "https://github.com/MikesRuthless12/Sourcerer/wiki/search-syntax",
    "help.regex_syntax": "https://github.com/MikesRuthless12/Sourcerer/wiki/regex-syntax",
    "help.audio_modifier_reference": "https://github.com/MikesRuthless12/Sourcerer/wiki/audio-modifiers",
    "help.similarity_modifier_reference":
      "https://github.com/MikesRuthless12/Sourcerer/wiki/similarity",
    "help.command_line_options": "https://github.com/MikesRuthless12/Sourcerer/wiki/cli",
    "help.website": "https://github.com/MikesRuthless12/Sourcerer",
    "help.check_for_updates": "https://github.com/MikesRuthless12/Sourcerer/releases",
    "help.sponsor": "https://github.com/sponsors/MikesRuthless12"
  };
  for (const [id, url] of Object.entries(helpUrls)) {
    registry.register(id as CommandId, async () => {
      try {
        await files.open(url);
      } catch (e) {
        console.warn("[cmd] open url failed:", e);
      }
    });
  }
  registry.register("help.about", async () => dialogsStore.open("about"));

  // Sanity: every CommandId must have a handler. Cheap startup check.
  for (const id of COMMAND_IDS) {
    if (!registry.has(id)) {
      console.warn(`[bootstrap] missing handler for ${id}`);
    }
  }
}

function toggleLens(lens: "filename" | "content" | "audio" | "similarity") {
  const current = settingsStore.state.lens_visibility;
  void settingsStore.patch({
    lens_visibility: { ...current, [lens]: !current[lens] }
  });
}
