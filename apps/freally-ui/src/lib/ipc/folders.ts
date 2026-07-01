import { invoke } from "@tauri-apps/api/core";
import type { WatchedFolder } from "./types";

export async function list(): Promise<WatchedFolder[]> {
  return invoke<WatchedFolder[]>("folders_list");
}

export async function add(folder: WatchedFolder): Promise<void> {
  return invoke<void>("folders_add", { folder });
}

export async function remove(id: string): Promise<void> {
  return invoke<void>("folders_remove", { id });
}

export async function update(folder: WatchedFolder): Promise<void> {
  return invoke<void>("folders_update", { folder });
}

export async function rescan(id: string): Promise<void> {
  return invoke<void>("folders_rescan", { id });
}

export async function rescanAll(): Promise<void> {
  return invoke<void>("folders_rescan_all");
}
