import { invoke } from "@tauri-apps/api/core";
import type { ExcludeRules } from "./types";

export async function get(): Promise<ExcludeRules> {
  return invoke<ExcludeRules>("excludes_get");
}

export async function set(rules: ExcludeRules): Promise<void> {
  return invoke<void>("excludes_set", { rules });
}
