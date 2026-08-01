// Hit-in-context viewer state (SRC-M01).
//
// Holds the extracted document, the located matches, and which match is
// current. F3 / Shift+F3 move the cursor; the modal scrolls to follow.

import * as ipc from "../ipc/content_view";
import type { ContentDocument, HitSpan } from "../ipc/content_view";
import type { ParseReport, TokenInfo } from "../ipc/types";
import { queryStore } from "./query.svelte";
import { refineStore } from "./refine.svelte";

export type { ContentDocument, HitSpan };

/** Literal search terms from the current query, plus any active refine
 *  chips. Wildcards, regexes, and modifiers are excluded: the viewer
 *  highlights what the user typed as text, and a `size:>1mb` token is
 *  not something that appears inside a document. */
export function literalTerms(report: ParseReport | null): string[] {
  const out: string[] = [];
  const push = (s: string) => {
    const t = s.trim();
    if (t && !out.some((x) => x.toLowerCase() === t.toLowerCase())) out.push(t);
  };
  for (const tok of report?.tokens ?? []) {
    if (isTextToken(tok)) push(tok.text.replace(/^["']|["']$/g, ""));
  }
  for (const chip of refineStore.chips) push(chip);
  return out;
}

function isTextToken(tok: TokenInfo): boolean {
  return tok.kind.kind === "literal" || tok.kind.kind === "quoted";
}

class ContentViewStore {
  open = $state(false);
  path = $state("");
  name = $state("");
  doc = $state<ContentDocument | null>(null);
  loading = $state(false);
  error = $state<string | null>(null);
  /** Index into `doc.hits`. -1 when the document has no matches. */
  current = $state(-1);

  get hitCount(): number {
    return this.doc?.hits.length ?? 0;
  }

  async show(path: string, name: string) {
    this.open = true;
    this.path = path;
    this.name = name;
    this.doc = null;
    this.error = null;
    this.current = -1;
    this.loading = true;
    try {
      const doc = await ipc.document(path, literalTerms(queryStore.report));
      this.doc = doc;
      // Jump-to-first-hit on open: the point of the viewer is the
      // matches, so landing at line 1 of a 400-page PDF would waste
      // the click.
      this.current = doc.hits.length > 0 ? 0 : -1;
    } catch (e) {
      this.error = String(e);
    } finally {
      this.loading = false;
    }
  }

  close() {
    this.open = false;
    this.doc = null;
    this.error = null;
    this.current = -1;
  }

  /** Bumped on every `step()`. The viewer's scroll effect reads it so
   *  F3 re-centres the current match even when the index does not
   *  change — a one-match document would otherwise never scroll back. */
  scrollNonce = $state(0);

  /** Move `by` matches, wrapping at both ends so F3 past the last hit
   *  returns to the first rather than dead-ending. */
  step(by: number) {
    const n = this.hitCount;
    if (n === 0) return;
    this.current = (((this.current + by) % n) + n) % n;
    this.scrollNonce += 1;
  }
}

export const contentViewStore = new ContentViewStore();
