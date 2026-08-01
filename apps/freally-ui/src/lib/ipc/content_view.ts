// Hit-in-context viewer (SRC-M01).

import { call } from "./client";

/** One located match, by line and character offset within that line. */
export interface HitSpan {
  line: number;
  start: number;
  end: number;
}

export interface ContentDocument {
  lines: string[];
  hits: HitSpan[];
  /** True when `lines` stops short of the document. */
  truncated: boolean;
  /** Extractor that produced the text, for the viewer's footer. */
  extractor: string;
}

export function document(path: string, terms: string[]): Promise<ContentDocument> {
  return call<ContentDocument>("content_document", { path, terms });
}
