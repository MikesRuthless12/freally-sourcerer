// Keyboard shortcut → CommandId table. `mod` resolves to Ctrl on Win/Linux,
// Cmd on macOS at runtime via `isMac()`.

import type { CommandId } from "./ids";

export interface Shortcut {
  /** Lowercase key name (KeyboardEvent.key.toLowerCase() or .code). */
  key: string;
  mod?: boolean; // mod = Ctrl on Win/Linux, Cmd on macOS
  shift?: boolean;
  alt?: boolean;
  ctrl?: boolean; // explicit Ctrl regardless of platform
}

export interface Binding {
  shortcut: Shortcut;
  command: CommandId;
}

// PRD §8.28 shortcut table.
export const BINDINGS: Binding[] = [
  // File
  { shortcut: { key: "n", mod: true }, command: "file.new_window" },
  { shortcut: { key: "o", mod: true }, command: "file.open_file_list" },
  { shortcut: { key: "w", mod: true }, command: "file.close" },
  { shortcut: { key: "s", mod: true }, command: "file.export_results" },
  { shortcut: { key: "q", mod: true }, command: "file.exit" },
  // Edit
  { shortcut: { key: "x", mod: true }, command: "edit.cut" },
  { shortcut: { key: "c", mod: true }, command: "edit.copy" },
  { shortcut: { key: "v", mod: true }, command: "edit.paste" },
  { shortcut: { key: "a", mod: true }, command: "edit.select_all" },
  // View
  { shortcut: { key: "p", alt: true }, command: "view.preview" },
  { shortcut: { key: "1", mod: true, shift: true }, command: "view.thumbs.xl" },
  { shortcut: { key: "2", mod: true, shift: true }, command: "view.thumbs.l" },
  { shortcut: { key: "3", mod: true, shift: true }, command: "view.thumbs.m" },
  { shortcut: { key: "6", mod: true, shift: true }, command: "view.details" },
  { shortcut: { key: "1", alt: true }, command: "view.window_size.small" },
  { shortcut: { key: "2", alt: true }, command: "view.window_size.medium" },
  { shortcut: { key: "3", alt: true }, command: "view.window_size.large" },
  { shortcut: { key: "4", alt: true }, command: "view.window_size.auto" },
  { shortcut: { key: "=", mod: true }, command: "view.zoom.in" },
  { shortcut: { key: "-", mod: true }, command: "view.zoom.out" },
  { shortcut: { key: "0", mod: true }, command: "view.zoom.reset" },
  { shortcut: { key: "1", mod: true }, command: "view.sort.name" },
  { shortcut: { key: "2", mod: true }, command: "view.sort.path" },
  { shortcut: { key: "3", mod: true }, command: "view.sort.size" },
  { shortcut: { key: "4", mod: true }, command: "view.sort.ext" },
  { shortcut: { key: "5", mod: true }, command: "view.sort.type" },
  { shortcut: { key: "6", mod: true }, command: "view.sort.modified" },
  { shortcut: { key: "7", mod: true }, command: "view.sort.created" },
  { shortcut: { key: "8", mod: true }, command: "view.sort.attributes" },
  { shortcut: { key: "9", mod: true }, command: "view.sort.recently_changed" },
  { shortcut: { key: "l", mod: true }, command: "view.sort.lufs" },
  { shortcut: { key: "l", mod: true, shift: true }, command: "view.sort.length" },
  { shortcut: { key: "F5" }, command: "view.refresh" },
  // Search
  { shortcut: { key: "i", mod: true }, command: "search.match_case" },
  { shortcut: { key: "b", mod: true }, command: "search.match_whole_word" },
  { shortcut: { key: "u", mod: true }, command: "search.match_path" },
  { shortcut: { key: "m", mod: true }, command: "search.match_diacritics" },
  { shortcut: { key: "r", mod: true }, command: "search.enable_regex" },
  { shortcut: { key: "f", mod: true, shift: true }, command: "search.organize_filters" },
  // Bookmarks
  { shortcut: { key: "d", mod: true }, command: "bookmarks.add" },
  { shortcut: { key: "b", mod: true, shift: true }, command: "bookmarks.organize" },
  // Tools — Ctrl+, on Win/Linux to dodge the browser Print collision (M9 fix);
  // macOS keeps Cmd+, per HIG via the `mod` substitution.
  { shortcut: { key: ",", mod: true }, command: "tools.options" },
  // Help
  { shortcut: { key: "F1" }, command: "help.help" },
  { shortcut: { key: "F1", mod: true }, command: "help.about" }
];

export function isMac(): boolean {
  if (typeof navigator === "undefined") return false;
  const p = (navigator.platform || "").toLowerCase();
  return p.includes("mac");
}

export function shortcutMatches(ev: KeyboardEvent, sc: Shortcut): boolean {
  const key = (ev.key || "").toLowerCase();
  const wantKey = sc.key.toLowerCase();
  if (key !== wantKey && ev.code.toLowerCase() !== wantKey) return false;
  const mac = isMac();
  const wantMod = sc.mod === true;
  const modPressed = mac ? ev.metaKey : ev.ctrlKey;
  if (wantMod !== modPressed) return false;
  // M23 fix: every modifier must match exactly. Absence in the spec means
  // "must NOT be pressed" — otherwise Ctrl+Shift+1 matched both
  // `view.thumbs.xl` AND `view.sort.name` and ordering decided the winner.
  const wantShift = sc.shift === true;
  if (wantShift !== ev.shiftKey) return false;
  const wantAlt = sc.alt === true;
  if (wantAlt !== ev.altKey) return false;
  // Explicit ctrl: only check when set; mod has already covered the
  // platform-aware Ctrl/⌘ case above.
  if (sc.ctrl !== undefined && ev.ctrlKey !== sc.ctrl) return false;
  return true;
}

export function formatShortcut(sc: Shortcut): string {
  const parts: string[] = [];
  if (sc.mod) parts.push(isMac() ? "⌘" : "Ctrl");
  if (sc.ctrl && !sc.mod) parts.push("Ctrl");
  if (sc.shift) parts.push(isMac() ? "⇧" : "Shift");
  if (sc.alt) parts.push(isMac() ? "⌥" : "Alt");
  parts.push(sc.key.length === 1 ? sc.key.toUpperCase() : sc.key);
  return parts.join(isMac() ? "" : "+");
}
