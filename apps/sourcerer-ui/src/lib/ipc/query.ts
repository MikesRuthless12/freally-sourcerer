import { call } from "./client";
import type {
  LensTimings,
  ParseOpts,
  ParseReport,
  QueryBatch,
  QueryRunHandle
} from "./types";

export function parse(source: string, opts: ParseOpts): Promise<ParseReport> {
  return call<ParseReport>("query_parse", { source, opts });
}

export interface RunOpts {
  strict_everything?: boolean;
  per_lens_limits?: { filename: number; content: number; audio: number; similarity: number };
}

export function run(source: string, opts: RunOpts = {}): Promise<QueryRunHandle> {
  return call<QueryRunHandle>("query_run", {
    source,
    strict_everything: opts.strict_everything ?? false,
    per_lens_limits: opts.per_lens_limits ?? null
  });
}

export function cancel(handle: string): Promise<void> {
  return call<void>("query_cancel", { handle });
}

export function lensTimings(handle: string): Promise<LensTimings> {
  return call<LensTimings>("query_lens_timings", { handle });
}

