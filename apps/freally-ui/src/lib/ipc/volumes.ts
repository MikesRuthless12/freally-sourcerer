import { invoke } from "@tauri-apps/api/core";
import type { CatalogInfo, VolumeInfo, VolumeUpdate } from "./types";

export async function list(): Promise<VolumeInfo[]> {
  return invoke<VolumeInfo[]>("volumes_list");
}

/** SRC-M14 — every device indexed, including unplugged ones. */
export async function catalogs(): Promise<CatalogInfo[]> {
  return invoke<CatalogInfo[]>("catalogs_list");
}

export async function update(payload: VolumeUpdate): Promise<void> {
  return invoke<void>("volumes_update", { update: payload });
}

export async function recreateJournal(id: string): Promise<void> {
  return invoke<void>("volumes_recreate_journal", { id });
}

export async function resetStream(id: string): Promise<void> {
  return invoke<void>("volumes_reset_stream", { id });
}

export async function upgradeFanotify(): Promise<void> {
  return invoke<void>("volumes_upgrade_fanotify");
}

export async function remove(id: string): Promise<void> {
  return invoke<void>("volumes_remove", { id });
}
