import { call } from "./client";
import type { SettingsState } from "./types";

export function get(): Promise<SettingsState> {
  return call<SettingsState>("settings_get");
}

export function set(patch: Partial<SettingsState>): Promise<SettingsState> {
  return call<SettingsState>("settings_set", { patch });
}

export function reset(): Promise<SettingsState> {
  return call<SettingsState>("settings_reset");
}
