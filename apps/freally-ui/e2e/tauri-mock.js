// A minimal Tauri v2 IPC mock so the REAL built UI renders in a plain browser
// (Playwright) for the visual-smoke gallery. It shims `window.__TAURI_INTERNALS__`
// with an `invoke` that returns canned, valid data per command, plus an event
// system good enough for `listen()` so the daemon's streamed `query:batch` /
// `query:done` notifications can be replayed. UI-render coverage only — the
// Rust daemon is covered by `cargo test` + tests/smoke.
//
// Runs via Playwright addInitScript (before the app bundle loads).
// URL params: `?wizard=1` leaves `first_run_complete` false so the
// first-run wizard shows (default fixture is an existing user).
(() => {
  const params = new URLSearchParams(location.search);
  const showWizard = params.get("wizard") === "1";

  // ---- event system: `plugin:event|listen` registers a transformCallback
  // id per event name; emit() replays payloads into those callbacks. ----
  let cbId = 0;
  const listeners = {}; // event name -> [handler ids]

  function emit(event, payload) {
    for (const id of listeners[event] || []) {
      const cb = window[`_${id}`];
      if (typeof cb === "function") cb({ event, id, payload });
    }
  }

  // ---- canned fixtures (shapes mirror src/lib/ipc/types.ts) ----

  // settings_get may return a PARTIAL SettingsState — the store spreads it
  // over its FALLBACK. Dark theme pins deterministic screenshots.
  const settings = {
    theme: "dark",
    first_run_complete: !showWizard,
  };

  const NOW = 1752900000000; // fixed clock for deterministic dates

  const hit = (n, lens, name, path, ext, size, type, score, attrs) => ({
    file_id: `f-${lens}-${n}`,
    lens,
    name,
    path,
    ext,
    size,
    modified_ms: NOW - n * 86_400_000,
    type,
    score,
    attrs: attrs || 0,
  });

  const HITS = {
    filename: [
      hit(1, "filename", "quarterly-report.pdf", "C:\\Users\\mike\\Documents\\quarterly-report.pdf", "pdf", 1_482_311, "PDF Document", 0.98),
      hit(2, "filename", "report-draft.docx", "C:\\Users\\mike\\Documents\\report-draft.docx", "docx", 88_420, "Word Document", 0.95),
      hit(3, "filename", "Projects", "C:\\Users\\mike\\Projects", "", 0, "Folder", 0.93, 0x10),
      hit(4, "filename", "notes.txt", "C:\\Users\\mike\\Desktop\\notes.txt", "txt", 2_130, "Text Document", 0.9),
      hit(5, "filename", "holiday-photo.jpg", "C:\\Users\\mike\\Pictures\\holiday-photo.jpg", "jpg", 3_214_881, "JPEG Image", 0.88),
      hit(6, "filename", "setup-installer.exe", "C:\\Users\\mike\\Downloads\\setup-installer.exe", "exe", 48_211_002, "Application", 0.85),
      hit(7, "filename", "archive-2025.zip", "D:\\Backups\\archive-2025.zip", "zip", 812_441_990, "ZIP Archive", 0.82),
      hit(8, "filename", "main.rs", "C:\\Users\\mike\\Projects\\sourcerer\\src\\main.rs", "rs", 14_882, "Rust Source", 0.8),
    ],
    content: [
      hit(1, "content", "meeting-minutes.md", "C:\\Users\\mike\\Documents\\meeting-minutes.md", "md", 9_120, "Markdown", 0.91),
      hit(2, "content", "invoice-0142.pdf", "C:\\Users\\mike\\Documents\\invoices\\invoice-0142.pdf", "pdf", 220_114, "PDF Document", 0.87),
      hit(3, "content", "config.toml", "C:\\Users\\mike\\Projects\\sourcerer\\config.toml", "toml", 1_882, "TOML Config", 0.8),
    ],
    audio: [
      hit(1, "audio", "podcast-episode-12.mp3", "C:\\Users\\mike\\Music\\podcast-episode-12.mp3", "mp3", 58_114_002, "MP3 Audio", 0.9),
      hit(2, "audio", "voice-memo.wav", "C:\\Users\\mike\\Music\\memos\\voice-memo.wav", "wav", 12_004_882, "WAV Audio", 0.84),
    ],
    similarity: [
      hit(1, "similarity", "holiday-photo (copy).jpg", "C:\\Users\\mike\\Pictures\\dupes\\holiday-photo (copy).jpg", "jpg", 3_214_881, "JPEG Image", 0.97),
      hit(2, "similarity", "report-draft-final.docx", "C:\\Users\\mike\\Documents\\old\\report-draft-final.docx", "docx", 87_990, "Word Document", 0.9),
    ],
  };

  const TIMINGS = { filename_ms: 6, content_ms: 21, audio_ms: 9, similarity_ms: 12, total_ms: 48 };

  let queryN = 0;

  const RESP = {
    settings_get: () => settings,
    settings_set: (args) => args && args.patch ? args.patch : settings,
    settings_reset: () => settings,
    settings_apply_to_daemon: () => null,

    index_state: () => ({
      phase: "indexed",
      files_indexed: 1_250_431,
      files_total: 1_250_431,
      message: "Index up to date",
    }),
    index_verify: () => null,
    index_compact: () => null,
    index_rebuild: () => null,

    bookmarks_list: () => [
      { id: "bm-1", name: "Big videos", query: "video: size:>1gb", created_ms: NOW - 40 * 86_400_000, filters: [] },
      { id: "bm-2", name: "Loud tracks", query: "audio: lufs:>-8", created_ms: NOW - 12 * 86_400_000, filters: ["audio"] },
    ],
    bookmarks_save: (args) => ({
      id: `bm-${Math.random().toString(36).slice(2, 8)}`,
      name: (args && args.name) || "Bookmark",
      query: (args && args.query) || "",
      created_ms: NOW,
      filters: (args && args.filters) || [],
    }),
    bookmarks_delete: () => null,
    bookmarks_rename: () => null,

    query_parse: (args) => ({
      source: (args && args.source) || "",
      strict_everything: false,
      ast: { kind: "true" },
      tokens: [],
      errors: [],
    }),
    query_run: () => {
      const handle = `mock-q-${++queryN}`;
      // Emit after the store has recorded the handle (run() sets
      // `running` right after this invoke resolves).
      setTimeout(() => {
        for (const lens of ["filename", "content", "audio", "similarity"]) {
          emit("query:batch", { handle, lens, hits: HITS[lens], done: true });
        }
        emit("query:done", { handle, timings: TIMINGS });
      }, 40);
      return { handle };
    },
    query_cancel: () => null,
    query_lens_timings: () => TIMINGS,

    volumes_list: () => [
      {
        id: "vol-c",
        label: "Windows (C:)",
        mount_point: "C:\\",
        fs_kind: "NTFS",
        used_bytes: 412_000_000_000,
        total_bytes: 1_000_000_000_000,
        status: "indexed",
        indexed: true,
        journal_enabled: true,
        journal_buffer_kb: 32768,
        allocation_delta_kb: 8192,
        include_only: null,
        load_recent_changes: true,
        monitor_changes: true,
      },
    ],
    volumes_update: () => null,
    volumes_recreate_journal: () => null,
    volumes_reset_stream: () => null,
    volumes_upgrade_fanotify: () => null,
    volumes_remove: () => null,

    folders_list: () => [
      {
        id: "fld-1",
        path: "D:\\Media",
        monitor_changes: true,
        buffer_kb: 1024,
        rescan_on_full_buffer: true,
        rescan_schedule: { kind: "never" },
      },
    ],
    folders_add: () => null,
    folders_remove: () => null,
    folders_update: () => null,
    folders_rescan: () => null,
    folders_rescan_all: () => null,

    excludes_get: () => ({
      exclude_hidden: true,
      exclude_system: true,
      list_enabled: false,
      folders: [],
      include_only_files: null,
      exclude_files: null,
    }),
    excludes_set: () => null,

    network_status: () => ({
      https_running: false,
      https_bind: null,
      https_port: null,
      https_token_fingerprint: null,
      api_running: false,
      api_port: null,
    }),

    history_get: () => ({
      search_history_enabled: true,
      search_history_keep_days: 90,
      run_history_enabled: true,
      run_history_keep_days: 90,
      privacy_mode: false,
      per_lens: { filename: true, content: true, audio: true, similarity: true },
    }),
    history_set: () => null,
    history_clear: () => null,

    extractors_list: () => [
      { id: "pdf", display_name: "PDF text", mode: "eager", formats: ["pdf"] },
      { id: "office", display_name: "Office documents", mode: "lazy", formats: ["docx", "xlsx", "pptx"] },
    ],
    extractors_set_mode: () => null,
    custom_extractors_list: () => [],
    custom_extractors_set_trusted: () => null,
    custom_extractors_refresh_hashes: () => null,

    icon_for_ext: () => null,
    files_thumbnail: () => null,
    files_preview: () => ({ kind: "unsupported", message: "No preview in the mock." }),
    files_whitelist_user_chosen: () => null,
  };

  function respond(cmd, args) {
    if (cmd in RESP) return RESP[cmd](args);
    if (cmd === "plugin:event|listen") {
      const ev = args && args.event;
      const handler = args && args.handler;
      if (ev && typeof handler === "number") {
        (listeners[ev] = listeners[ev] || []).push(handler);
      }
      return typeof handler === "number" ? handler : 0;
    }
    // Other plugins (window, os, dialog, …) + unknown mutations → resolve
    // null; console noise from optional calls is acceptable in the gallery.
    return null;
  }

  window.__TAURI_INTERNALS__ = {
    invoke: (cmd, args) => Promise.resolve(respond(cmd, args)),
    transformCallback: (cb) => {
      const id = ++cbId;
      window[`_${id}`] = cb;
      return id;
    },
    convertFileSrc: (path, protocol) => `${protocol || "asset"}://localhost/${path}`,
    metadata: {
      currentWindow: { label: "main" },
      currentWebview: { windowLabel: "main", label: "main" },
    },
    plugins: {},
  };
})();
