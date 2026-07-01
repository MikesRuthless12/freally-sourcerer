import { invoke } from "@tauri-apps/api/core";
import type { HistoryConfig, HistoryUpdate } from "./types";

export async function get(): Promise<HistoryConfig> {
  return invoke<HistoryConfig>("history_get");
}

export async function set(update: HistoryUpdate): Promise<void> {
  return invoke<void>("history_set", { update });
}

export async function clear(): Promise<void> {
  return invoke<void>("history_clear");
}
