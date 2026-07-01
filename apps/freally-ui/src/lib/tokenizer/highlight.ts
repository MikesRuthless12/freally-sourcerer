// Maps a ParseReport's token stream to colored span descriptors for the
// search bar's mirror layer. Returns a list of {text, className} segments
// that cover the entire source (gaps filled with whitespace spans).

import type { ParseReport, TokenInfo } from "../ipc/types";

export interface HighlightSegment {
  text: string;
  className: string;
  isError: boolean;
}

const KIND_CLASS: Record<string, string> = {
  literal: "tok-literal",
  quoted: "tok-quoted",
  wildcard: "tok-wildcard",
  regex: "tok-regex",
  modifier: "tok-modifier",
  quick_filter: "tok-quick-filter",
  lens_prefix: "tok-lens-prefix",
  l_paren: "tok-paren",
  r_paren: "tok-paren",
  bang: "tok-operator",
  and: "tok-operator",
  or: "tok-operator",
  not: "tok-operator"
};

export function highlight(source: string, report: ParseReport | null): HighlightSegment[] {
  if (!source) return [];
  if (!report) return [{ text: source, className: "tok-pending", isError: false }];

  const errorRanges: { start: number; end: number }[] = report.errors.map((e) => ({
    start: e.span.start,
    end: e.span.end
  }));
  const isError = (start: number, end: number): boolean =>
    errorRanges.some((r) => !(end <= r.start || start >= r.end));

  const tokens = [...report.tokens].sort((a, b) => a.span.start - b.span.start);
  const segs: HighlightSegment[] = [];
  let cursor = 0;

  for (const tok of tokens) {
    if (tok.span.start > cursor) {
      const gap = source.slice(cursor, tok.span.start);
      if (gap.length > 0) {
        segs.push({ text: gap, className: "tok-whitespace", isError: false });
      }
    }
    const text = source.slice(tok.span.start, tok.span.end);
    const cls = KIND_CLASS[(tok.kind as { kind: string }).kind] ?? "tok-default";
    segs.push({ text, className: cls, isError: isError(tok.span.start, tok.span.end) });
    cursor = tok.span.end;
  }

  if (cursor < source.length) {
    segs.push({ text: source.slice(cursor), className: "tok-trailing", isError: false });
  }
  return segs;
}

export function firstError(report: ParseReport | null): { message: string; offset: number } | null {
  if (!report || report.errors.length === 0) return null;
  // Sort by span position so multi-error reports surface the leftmost
  // error first, regardless of the parser's emit order.
  const sorted = [...report.errors].sort((a, b) => a.span.start - b.span.start);
  const e = sorted[0]!;
  return { message: e.message, offset: e.span.start };
}

// Util to test whether a token info corresponds to a specific kind.
export function tokenKind(t: TokenInfo): string {
  return (t.kind as { kind: string }).kind;
}
