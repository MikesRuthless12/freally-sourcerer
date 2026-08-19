// The shape `query.run` puts on the wire.
//
// Tauri drops invoke arguments the Rust command does not declare, and it
// does so silently — no error, no type failure, the flag simply never
// arrives. There are two ways to trip over that, and this file has
// caught both: a name the command does not declare at all (SRC-M23's
// match-mode set, sent through all of Build 3 and read as `false` every
// time), and a name declared in snake_case when `#[tauri::command]`
// renames its arguments to **camelCase** by default and matches them
// exactly (which is why `strict_everything` and `per_lens_limits` had
// never bound either).
//
// So the top-level keys here are camelCase, matching every other invoke
// in the app — `{ ext, isDir }`, `{ handlerId }`, `{ commandId }`. The
// keys *inside* `searchOpts` stay snake_case: those are deserialized by
// serde into `QuerySearchOpts`, which the rename does not reach.

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
    expect(last?.args.searchOpts).toEqual({
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
    expect(last?.args.searchOpts).toEqual({
      match_case: false,
      whole_word: false,
      match_path: false,
      match_diacritics: false,
      match_phonetic: false,
      ignore_punctuation: false,
      ignore_whitespace: false
    });
  });

  it("sends naturalSort at the top level, defaulting to on", async () => {
    await query.run("alpha");
    expect(last?.args.naturalSort).toBe(true);
    await query.run("alpha", { natural_sort: false });
    expect(last?.args.naturalSort).toBe(false);
  });

  it("camelCases every top-level argument", async () => {
    // The whole payload, so a snake_case key added later fails here
    // rather than binding to nothing at the Rust boundary.
    await query.run("alpha");
    expect(Object.keys(last?.args ?? {}).sort()).toEqual([
      "naturalSort",
      "perLensLimits",
      "searchOpts",
      "source",
      "strictEverything"
    ]);
  });
});
