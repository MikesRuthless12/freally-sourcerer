// Active file list (SRC-M03).
//
// Importing an `.efu` makes a catalogue of a *disconnected* volume
// searchable: the entries replace the daemon as the query source until
// the list is closed. That is the whole point of the format — you plug
// the drive in once, export the list, and can still answer "which drive
// was that file on?" a month later.
//
// Search over an active list runs here rather than in the daemon: the
// rows never entered the index, so there is nothing daemon-side to
// query. Matching is the same name-or-path rule the refinement chips
// use (`matchesLoweredTerm`), NOT the full DSL — `size:>1mb` against a
// loaded list is matched as literal text. The status bar names the
// loaded list so the mode is at least visible; teaching the DSL to run
// over an imported list means loading it into a real index, which is
// SRC-N69's job.

import * as ipc from "../ipc/file_lists";
import type { FileListEntry, QueryHit } from "../ipc/types";
import { baseName, extensionOf, matchesLoweredTerm } from "../util/path";

/** Hard cap on rows rendered from a list. A full-volume `.efu` runs to
 *  millions of rows; the result list is not virtualised, so we bound
 *  what reaches it and report the truncation. */
const MAX_RENDERED = 5000;

/** An entry plus the derived fields `search()` needs, computed once at
 *  load rather than per keystroke. A million-entry `.efu` searched at
 *  typing speed would otherwise re-split and re-lowercase every path on
 *  every character. */
interface IndexedEntry {
  entry: FileListEntry;
  name: string;
  nameLower: string;
  pathLower: string;
}

class FileListStore {
  /** File name of the loaded list, or null when searching live. The
   *  backend returns only the name — the full path is not something the
   *  frontend needs, and not something it should be able to replay. */
  path = $state<string | null>(null);
  entries = $state<FileListEntry[]>([]);
  /** True when the last search hit `MAX_RENDERED` and stopped early. */
  truncated = $state(false);

  #indexed: IndexedEntry[] = [];

  get active(): boolean {
    return this.path !== null;
  }

  /** File name of the loaded list, for the status bar. */
  get label(): string {
    return this.path ? baseName(this.path) : "";
  }

  /** Prompt for a list and load it. Returns the entry count, or `null`
   *  when the user dismissed the dialog. The backend owns the dialog,
   *  so there is no path to pass in. */
  async load(): Promise<number | null> {
    const { opened, name, entries } = await ipc.importList();
    if (!opened) return null;
    this.setEntries(entries);
    this.path = name;
    return entries.length;
  }

  /** Replace the loaded entries. Exposed for tests, which have no
   *  file to import. */
  setEntries(entries: FileListEntry[]) {
    this.entries = entries;
    this.#indexed = entries.map((entry) => {
      const name = baseName(entry.path);
      return {
        entry,
        name,
        nameLower: name.toLowerCase(),
        pathLower: entry.path.toLowerCase()
      };
    });
    this.truncated = false;
  }

  close() {
    this.path = null;
    this.entries = [];
    this.#indexed = [];
    this.truncated = false;
  }

  /** Rows matching `source`, as filename-lens hits. An empty query
   *  returns the head of the list — the same "show me everything"
   *  behaviour a bare `*` has against the live index. */
  search(source: string): QueryHit[] {
    const needle = source.trim().toLowerCase();
    const out: QueryHit[] = [];
    for (const row of this.#indexed) {
      if (needle && !matchesLoweredTerm(needle, row.nameLower, row.pathLower)) continue;
      if (out.length >= MAX_RENDERED) {
        this.truncated = true;
        return out;
      }
      out.push(toHit(row));
    }
    this.truncated = false;
    return out;
  }
}

function toHit(row: IndexedEntry): QueryHit {
  const ext = extensionOf(row.name).toLowerCase();
  return {
    file_id: row.entry.path,
    lens: "filename",
    name: row.name,
    path: row.entry.path,
    ext,
    size: row.entry.size,
    modified_ms: row.entry.modified_ms,
    type: ext ? ext.toUpperCase() : "FILE",
    score: 1,
    attrs: row.entry.attrs
  };
}

export const fileListStore = new FileListStore();
