// Query store — current source string + last parse report. The search bar
// owns the source; the parse report is recomputed on every keystroke and
// surfaced for the token-highlight layer + parse-error pill.
//
// Sequence-guarded: every parse call carries a monotonic id; only the
// latest in-flight call is allowed to update `report`/`parsing`. Out-of-
// order responses are dropped (matters under Phase-12 real-daemon latency).

import * as ipcQuery from "../ipc/query";
import type { ParseReport } from "../ipc/types";

const MAX_QUERY_LEN = 64_000;

class QueryStore {
  source = $state("");
  strict = $state(false);
  report = $state<ParseReport | null>(null);
  parsing = $state(false);
  private seq = 0;

  async setSource(s: string) {
    if (s.length > MAX_QUERY_LEN) {
      s = s.slice(0, MAX_QUERY_LEN);
    }
    this.source = s;
    const my = ++this.seq;
    this.parsing = true;
    let next: ParseReport | null = null;
    try {
      next = await ipcQuery.parse(s, { strict_everything: this.strict });
    } catch (e) {
      console.warn("[query] parse failed:", e);
    }
    if (my !== this.seq) return; // superseded by a newer keystroke
    if (next) this.report = next;
    this.parsing = false;
  }

  setStrict(v: boolean) {
    this.strict = v;
    void this.setSource(this.source);
  }
}

export const queryStore = new QueryStore();
