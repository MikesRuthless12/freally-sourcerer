// Build 1 result verbs — SRC-M04 (Open with…), SRC-M05 (advanced
// copy), SRC-M06 (terminal-here + custom commands).

import { call } from "./client";
import type { AppHandler, QuoteStyle } from "./types";

/** Applications the OS registers for this file's type, in its own
 *  preference order. An empty array is normal for exotic extensions. */
export function openWithCandidates(path: string): Promise<AppHandler[]> {
  return call<AppHandler[]>("open_with_candidates", { path });
}

export function openWith(path: string, handlerId: string): Promise<void> {
  return call<void>("open_with", { path, handlerId });
}

export function copyFileContents(path: string): Promise<void> {
  return call<void>("copy_file_contents", { path });
}

export function copyFilesAsObjects(paths: string[]): Promise<void> {
  return call<void>("copy_files_as_objects", { paths });
}

export function copyPathList(paths: string[], style: QuoteStyle): Promise<void> {
  return call<void>("copy_path_list", { paths, style });
}

export function openTerminalHere(path: string): Promise<void> {
  return call<void>("open_terminal_here", { path });
}

/** Run a configured command by id. The backend resolves the id against
 *  its own persisted settings — passing the command body would let this
 *  layer choose the executable. */
export function runCustomCommand(commandId: string, path: string): Promise<void> {
  return call<void>("run_custom_command", { commandId, path });
}
