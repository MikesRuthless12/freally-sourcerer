import { call } from "./client";
import type {
  LensTimings,
  ParseOpts,
  ParseReport,
  QueryBatch,
  QueryRunHandle,
  SearchOpts
} from "./types";

export function parse(source: string, opts: ParseOpts): Promise<ParseReport> {
  return call<ParseReport>("query_parse", { source, opts });
}

export interface RunOpts {
  strict_everything?: boolean;
  per_lens_limits?: { filename: number; content: number; audio: number; similarity: number };
  /** SRC-M23 — the whole `Search →` toggle set now reaches the
   *  executor. Through Build 2 only `match_phonetic` did, so Match
   *  Case and friends moved a checkmark and changed nothing else. */
  search_opts?: Partial<SearchOpts>;
  /** SRC-M24 — Settings → Results → *Natural sort*. Not part of
   *  `search_opts`: it is a top-level setting, and it changes the
   *  daemon's ordering rather than what a row matches. Omitted means
   *  "on", which is what the setting defaults to. */
  natural_sort?: boolean;
}

export function run(source: string, opts: RunOpts = {}): Promise<QueryRunHandle> {
  const m = opts.search_opts ?? {};
  // camelCase keys. `#[tauri::command]` renames its arguments to camelCase
  // by default and looks them up by exact name, so `strict_everything`
  // simply never bound to `strict_everything: bool` — it read as `None`,
  // silently, exactly like the match-mode flags did. Every other call site
  // in the app already had this right (`{ ext, isDir }`, `{ handlerId }`,
  // `{ commandId }`); this one file did not, so Strict Everything mode and
  // the per-lens limits had never reached the daemon either.
  return call<QueryRunHandle>("query_run", {
    source,
    strictEverything: opts.strict_everything ?? false,
    perLensLimits: opts.per_lens_limits ?? null,
    // Field-by-field rather than spreading `search_opts`: the settings
    // key is `match_whole_word` and the wire field is `whole_word`, so a
    // spread would drop that flag on the floor with no type error.
    // `enable_regex` is deliberately absent — it changes how the query
    // is *parsed*, not how a row is matched, and belongs to ParseOpts.
    //
    // Nested under one key, not seven flat ones, because Tauri drops
    // invoke arguments the Rust command does not declare by name — and
    // through Build 3 it declared none of these, so the flags were sent
    // and discarded. `QuerySearchOpts` in `commands/query.rs` is the
    // landing spot; it forwards these names on to the daemon unchanged.
    searchOpts: {
      match_case: m.match_case ?? false,
      whole_word: m.match_whole_word ?? false,
      match_path: m.match_path ?? false,
      match_diacritics: m.match_diacritics ?? false,
      match_phonetic: m.match_phonetic ?? false,
      ignore_punctuation: m.ignore_punctuation ?? false,
      ignore_whitespace: m.ignore_whitespace ?? false
    },
    naturalSort: opts.natural_sort ?? true
  });
}

export function cancel(handle: string): Promise<void> {
  return call<void>("query_cancel", { handle });
}

export function lensTimings(handle: string): Promise<LensTimings> {
  return call<LensTimings>("query_lens_timings", { handle });
}

