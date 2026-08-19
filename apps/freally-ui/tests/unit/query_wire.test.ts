// The shape `query.run` puts on the wire.
//
// Tauri drops invoke arguments the Rust command does not declare by
// name, and it does so silently — no error, no type failure, the flag
// simply never arrives. That is how SRC-M23's whole match-mode set was
// sent from here through all of Build 3 and read as `false` by the
// executor every time. These cases pin the argument names, so renaming
// one without renaming its counterpart in `commands/query.rs` fails
// here instead of in a bug report about a checkbox that does nothing.

import { describe, it, expect, beforeEach } from "vitest";
import { setIpcMock } from "../../src/lib/ipc/client";
import * as query from "../../src/lib/ipc/query";

let last: { cmd: string; args: Record<string, unknown> } | null = null;

beforeEach(() => {
  last = null;
  setIpcMock(<T,>(cmd: string, args?: Record<string, unknown>) => {
    last = { cmd, args: args ?? {} };
    return Promise.resolve({ handle: "h1" } as T);
  });
});

describe("query.run wire shape", () => {
  it("nests the match-mode flags under search_opts using the daemon's names", async () => {
    await query.run("alpha", {
      search_opts: {
        match_case: true,
        match_whole_word: true,
        match_path: true,
        match_diacritics: true,
        match_phonetic: true,
        ignore_punctuation: true,
        ignore_whitespace: true,
        enable_regex: true
      }
    });
    expect(last?.cmd).toBe("query_run");
    // `match_whole_word` is the settings key; `whole_word` is the wire
    // field. `enable_regex` belongs to ParseOpts and must not appear.
    expect(last?.args.search_opts).toEqual({
      match_case: true,
      whole_word: true,
      match_path: true,
      match_diacritics: true,
      match_phonetic: true,
      ignore_punctuation: true,
      ignore_whitespace: true
    });
  });

  it("defaults every match-mode flag to false when none are given", async () => {
    await query.run("alpha");
    expect(last?.args.search_opts).toEqual({
      match_case: false,
      whole_word: false,
      match_path: false,
      match_diacritics: false,
      match_phonetic: false,
      ignore_punctuation: false,
      ignore_whitespace: false
    });
  });

  it("sends natural_sort at the top level, defaulting to on", async () => {
    await query.run("alpha");
    expect(last?.args.natural_sort).toBe(true);
    await query.run("alpha", { natural_sort: false });
    expect(last?.args.natural_sort).toBe(false);
  });
});
