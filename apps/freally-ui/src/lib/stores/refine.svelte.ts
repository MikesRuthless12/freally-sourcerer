// Search within results (SRC-M02).
//
// A refinement narrows the set the daemon already returned instead of
// re-running the query. That matters for the flow it exists to serve:
// you run one broad search, then peel the answer out of it in two or
// three steps — and each step must be reversible without retyping the
// broad search.
//
// Hence chips: every refinement is a removable token, and removing the
// middle one leaves the others in place. A single "refine" text field
// that overwrote itself would force a retype every time you overshot.

import { matchesLoweredTerm } from "../util/path";

/** Chips beyond this add nothing a user can reason about, and each one
 *  costs a pass over the result set. */
const MAX_CHIPS = 8;

class RefineStore {
  chips = $state<string[]>([]);
  /** True while the refine input is open. Ctrl+Shift+F toggles it. */
  open = $state(false);
  /** Bound to the input; committed as a chip on Enter. */
  draft = $state("");

  get active(): boolean {
    return this.chips.length > 0;
  }

  /** Bumped on every `show()`. The bar's focus effect depends on it,
   *  so pressing the shortcut while the bar is already open still pulls
   *  focus back — setting `open = true` a second time changes nothing
   *  and would leave the keystrokes going to the main search box. */
  focusNonce = $state(0);

  /** Open the bar and focus its input. Idempotent — pressing the
   *  shortcut again while open keeps existing chips and re-focuses. */
  show() {
    this.open = true;
    this.focusNonce += 1;
  }

  /** Close the bar. Chips survive: the narrowed set is still what the
   *  user is looking at, and the chip stack is how they undo it. */
  hide() {
    this.open = false;
    this.draft = "";
  }

  commitDraft() {
    const term = this.draft.trim();
    this.draft = "";
    if (!term) return;
    if (this.chips.length >= MAX_CHIPS) return;
    if (this.chips.some((c) => c.toLowerCase() === term.toLowerCase())) return;
    this.chips = [...this.chips, term];
  }

  removeAt(i: number) {
    this.chips = this.chips.filter((_, idx) => idx !== i);
  }

  clear() {
    this.chips = [];
    this.draft = "";
  }

  /** Chips lowercased once, not once per row. Narrowing a 5,000-hit
   *  set with three chips would otherwise lowercase the same three
   *  strings 15,000 times. */
  needles = $derived(this.chips.map((c) => c.toLowerCase()));

  /** Does `hit` survive every chip? Matching semantics live in
   *  `matchesLoweredTerm` so the refine bar and the file-list search
   *  cannot drift apart. Each side is lowercased once per row, not
   *  once per row per chip. */
  matches(hit: { name: string; path: string }): boolean {
    const needles = this.needles;
    if (needles.length === 0) return true;
    const nameLower = hit.name.toLowerCase();
    const pathLower = hit.path.toLowerCase();
    return needles.every((n) => matchesLoweredTerm(n, nameLower, pathLower));
  }

  /** How many of `hits` survive, without materialising the survivors.
   *  The status bar reads this on every render. */
  countIn(hits: { name: string; path: string }[]): number {
    if (this.chips.length === 0) return hits.length;
    let n = 0;
    for (const h of hits) if (this.matches(h)) n++;
    return n;
  }

  apply<T extends { name: string; path: string }>(hits: T[]): T[] {
    if (this.chips.length === 0) return hits;
    return hits.filter((h) => this.matches(h));
  }
}

export const refineStore = new RefineStore();
