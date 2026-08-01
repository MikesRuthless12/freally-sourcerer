// File-list interop (SRC-M03). Serialisation lives in Rust
// (`freally_rpc::filelist`) so the CLI, daemon, and this app all write
// byte-identical files.
//
// Note there is no `path` parameter on either call: the backend opens
// the dialog itself and acts on the path it read from it. A path routed
// through this layer would be a path the backend has to trust us about,
// and `file_list_export` overwrites what it is pointed at.

import { call } from "./client";
import type { FileListEntry, FileListFormat, QueryHit } from "./types";

export interface ExportSummary {
  /** False when the user dismissed the save dialog. */
  saved: boolean;
  written: number;
  format: FileListFormat;
  lossy: boolean;
}

export interface ImportSummary {
  /** False when the user dismissed the open dialog. */
  opened: boolean;
  /** File name of the opened list, for the status bar. */
  name: string;
  entries: FileListEntry[];
}

/** Prompt for a destination, then write `hits` there. */
export function exportList(hits: QueryHit[]): Promise<ExportSummary> {
  return call<ExportSummary>("file_list_export", { hits });
}

/** Prompt for a file list, then read it back. */
export function importList(): Promise<ImportSummary> {
  return call<ImportSummary>("file_list_import", {});
}
