// Multi-select type filter (Audio / Video / Image / Document / Executable /
// Archive / Folder). Backs the chip row + Search menu's filter section.
//
// Semantics:
//   - Empty set OR full set  =  no type restriction ("Everything")
//   - 1 type                 =  e.g. `audio:` prefix
//   - 2+ types               =  `(audio: OR video: …)` group
//
// "Everything" appears checked when every individual type is checked, so a
// click on Everything toggles between all-selected and none-selected.

export type TypeFilterId =
  | "audio"
  | "video"
  | "picture"
  | "document"
  | "executable"
  | "compressed"
  | "folder";

export const ALL_TYPE_FILTERS: TypeFilterId[] = [
  "audio",
  "video",
  "picture",
  "document",
  "executable",
  "compressed",
  "folder",
];

const TOKEN: Record<TypeFilterId, string> = {
  audio: "audio:",
  video: "video:",
  picture: "picture:",
  document: "document:",
  executable: "exec:",
  compressed: "archive:",
  folder: "folder:", // NOTE: parser maps `folder:` to a Parent(value) modifier,
                     // not "directories only". The Folder chip will not filter
                     // correctly until a proper folder-only modifier exists.
};

class TypeFilterStore {
  // Default: everything-mode (full set). Matches the user's mental model
  // where opening the Search menu shows a checkmark next to "Everything".
  selected = $state<Set<TypeFilterId>>(new Set(ALL_TYPE_FILTERS));

  has(id: TypeFilterId): boolean {
    return this.selected.has(id);
  }

  isNoneSelected(): boolean {
    return this.selected.size === 0;
  }

  isAllSelected(): boolean {
    return this.selected.size === ALL_TYPE_FILTERS.length;
  }

  // "Everything" is checked only when every individual type is in the set.
  // Deselect any one type and "Everything" loses its checkmark too.
  isEverythingChecked(): boolean {
    return this.selected.size === ALL_TYPE_FILTERS.length;
  }

  toggle(id: TypeFilterId) {
    const next = new Set(this.selected);
    if (next.has(id)) next.delete(id);
    else next.add(id);
    this.selected = next;
  }

  // Click "Everything": when all are selected, clear them; otherwise
  // restore the full set. Acts as a select-all / deselect-all toggle.
  toggleEverything() {
    this.selected = this.isEverythingChecked() ? new Set() : new Set(ALL_TYPE_FILTERS);
  }

  /** Replace the entire set with the given ids. Unknown ids are ignored
   *  so an older bookmark whose persisted ids don't all exist anymore
   *  still loads safely. */
  setFromIds(ids: string[]) {
    const valid = new Set<TypeFilterId>();
    for (const id of ids) {
      if ((ALL_TYPE_FILTERS as string[]).includes(id)) {
        valid.add(id as TypeFilterId);
      }
    }
    this.selected = valid;
  }

  /** Snapshot the current set as an array — what gets persisted to a
   *  bookmark. Order is the canonical ALL_TYPE_FILTERS order for
   *  deterministic JSON output. */
  toIds(): TypeFilterId[] {
    return ALL_TYPE_FILTERS.filter((id) => this.selected.has(id));
  }

  /** Parser-level fragment for the active selection.
   *  Empty string when no filter should be applied. */
  toQueryFragment(): string {
    const n = this.selected.size;
    if (n === 0 || n === ALL_TYPE_FILTERS.length) return "";
    const tokens = ALL_TYPE_FILTERS.filter((id) => this.selected.has(id)).map(
      (id) => TOKEN[id],
    );
    if (tokens.length === 1) return tokens[0]!;
    return `(${tokens.join(" OR ")})`;
  }
}

export const typeFilterStore = new TypeFilterStore();
