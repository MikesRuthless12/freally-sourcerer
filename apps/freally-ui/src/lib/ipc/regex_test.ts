// SRC-M20 — test a candidate regex with the engine the executor uses.

import { call } from "./client";

export interface MatchSpan {
  /** Character offsets, not byte offsets — safe to slice in JS. */
  start: number;
  end: number;
}

export interface SubjectMatches {
  /** Index into the `subjects` array that was sent. */
  index: number;
  spans: MatchSpan[];
}

export interface RegexTestResult {
  valid: boolean;
  /** Compile error from the Rust engine, or null. */
  error: string | null;
  /** Only the subjects that matched. */
  matches: SubjectMatches[];
}

export function test(
  pattern: string,
  subjects: string[],
  matchCase: boolean
): Promise<RegexTestResult> {
  return call<RegexTestResult>("regex_test", {
    args: { pattern, subjects, match_case: matchCase }
  });
}
