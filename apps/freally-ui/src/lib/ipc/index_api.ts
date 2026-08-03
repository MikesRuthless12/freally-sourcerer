import { call } from "./client";
import type { IndexHealth, IndexState } from "./types";

export function state(): Promise<IndexState> {
  return call<IndexState>("index_state");
}

export function health(): Promise<IndexHealth> {
  return call<IndexHealth>("index_health");
}

export function verify(): Promise<void> {
  return call<void>("index_verify");
}

export function compact(): Promise<void> {
  return call<void>("index_compact");
}

export function rebuild(): Promise<void> {
  return call<void>("index_rebuild");
}

// ---- SRC-M21 permission health ----

export type SkipReason = "permission_denied" | "not_found" | "other";
export type Guidance =
  | "macos_full_disk_access"
  | "linux_permissions"
  | "windows_acl"
  | "unknown";

export interface SkippedPath {
  path: string;
  reason: SkipReason;
  /** The OS's own message, kept verbatim. */
  detail: string;
  volume: string;
}

export interface VolumeGroup {
  volume: string;
  denied: number;
  other: number;
  entries: SkippedPath[];
}

export interface PermissionReport {
  denied: number;
  other: number;
  /** Entries counted but not retained past the daemon's cap. */
  dropped: number;
  by_volume: VolumeGroup[];
  guidance: Guidance;
  /** macOS only; null where the question does not apply or could not
   *  be answered. */
  full_disk_access: boolean | null;
}

export function permissions(): Promise<PermissionReport> {
  return call<PermissionReport>("index_permissions");
}
