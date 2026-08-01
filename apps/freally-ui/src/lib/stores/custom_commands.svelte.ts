// User-defined result actions (SRC-M06).
//
// Persisted through the settings store's `extras` bag rather than a
// dedicated backend table: these are pure UI configuration — the daemon
// never runs them, and the index never sees them.

import type { CustomCommand } from "../ipc/types";
import { extensionOf } from "../util/path";
import { settingsStore } from "./settings.svelte";

/** More than this and the context menu stops being a menu. */
export const MAX_CUSTOM_COMMANDS = 12;

function readAll(): CustomCommand[] {
  // Defensive: the value round-trips through the settings JSON on disk,
  // which a user can hand-edit.
  const raw: unknown = settingsStore.state.custom_commands;
  if (!Array.isArray(raw)) return [];
  return raw.filter(isCustomCommand);
}

function isCustomCommand(v: unknown): v is CustomCommand {
  if (typeof v !== "object" || v === null) return false;
  const c = v as Record<string, unknown>;
  return (
    typeof c.id === "string" &&
    typeof c.name === "string" &&
    typeof c.program === "string" &&
    Array.isArray(c.args) &&
    Array.isArray(c.extensions)
  );
}

/** Does `command` apply to `path`? Mirrors `CustomCommand::applies_to`
 *  on the Rust side so the menu and the backend agree on which commands
 *  a file gets. */
export function appliesTo(command: CustomCommand, path: string): boolean {
  if (command.extensions.length === 0) return true;
  const ext = extensionOf(path);
  if (ext === "") return false;
  // ASCII-only folding, matching Rust's `eq_ignore_ascii_case`. Full
  // `toLowerCase()` would show a menu item for `song.MÜSIK` that the
  // backend then refuses with "not configured for this file type".
  return command.extensions.some((e) => asciiLower(e) === asciiLower(ext));
}

function asciiLower(s: string): string {
  return s.replace(/[A-Z]/g, (c) => c.toLowerCase());
}

class CustomCommandsStore {
  get all(): CustomCommand[] {
    return readAll();
  }

  /** Commands that should appear in the context menu for `path`. */
  forPath(path: string): CustomCommand[] {
    return readAll().filter((c) => c.program.trim() !== "" && appliesTo(c, path));
  }

  async add(): Promise<CustomCommand | null> {
    const existing = readAll();
    if (existing.length >= MAX_CUSTOM_COMMANDS) return null;
    const created: CustomCommand = {
      // `crypto.randomUUID` is available in every Tauri webview; the
      // counter fallback keeps unit tests (jsdom) working.
      id: globalThis.crypto?.randomUUID?.() ?? `cmd-${existing.length + 1}`,
      name: "",
      program: "",
      args: ["{path}"],
      extensions: []
    };
    await this.replace([...existing, created]);
    return created;
  }

  async update(id: string, patch: Partial<CustomCommand>) {
    await this.replace(readAll().map((c) => (c.id === id ? { ...c, ...patch } : c)));
  }

  async remove(id: string) {
    await this.replace(readAll().filter((c) => c.id !== id));
  }

  async replace(next: CustomCommand[]) {
    await settingsStore.patch({ custom_commands: next });
  }
}

export const customCommandsStore = new CustomCommandsStore();
