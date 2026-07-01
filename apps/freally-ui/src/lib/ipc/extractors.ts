import { call } from "./client";
import type { ExtractorInfo, ExtractorMode } from "./types";

export function list(): Promise<ExtractorInfo[]> {
  return call<ExtractorInfo[]>("extractors_list");
}

export function setMode(id: string, mode: ExtractorMode): Promise<void> {
  return call<void>("extractors_set_mode", { id, mode });
}
