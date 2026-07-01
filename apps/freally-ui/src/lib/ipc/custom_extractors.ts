import { invoke } from "@tauri-apps/api/core";
import type { CustomExtractorEntry } from "./types";

export async function list(): Promise<CustomExtractorEntry[]> {
  return invoke<CustomExtractorEntry[]>("custom_extractors_list");
}

export async function setTrusted(id: string, trusted: boolean): Promise<void> {
  return invoke<void>("custom_extractors_set_trusted", { args: { id, trusted } });
}

export async function refreshHashes(): Promise<void> {
  return invoke<void>("custom_extractors_refresh_hashes");
}
