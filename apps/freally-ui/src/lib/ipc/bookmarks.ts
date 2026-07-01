import { call } from "./client";
import type { Bookmark } from "./types";

export function list(): Promise<Bookmark[]> {
  return call<Bookmark[]>("bookmarks_list");
}

export function save(name: string, query: string, filters?: string[]): Promise<Bookmark> {
  return call<Bookmark>("bookmarks_save", { name, query, filters: filters ?? [] });
}

export function remove(id: string): Promise<void> {
  return call<void>("bookmarks_delete", { id });
}

export function rename(id: string, name: string): Promise<void> {
  return call<void>("bookmarks_rename", { id, name });
}

export function whitelistUserChosen(path: string): Promise<void> {
  return call<void>("files_whitelist_user_chosen", { path });
}
