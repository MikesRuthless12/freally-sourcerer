import { invoke } from "@tauri-apps/api/core";
import type { NetworkStatus } from "./types";

export async function status(): Promise<NetworkStatus> {
  return invoke<NetworkStatus>("network_status");
}

export interface StartHttpsArgs {
  bind: string;
  port: number;
  force_https: boolean;
  legacy_auth: boolean;
}

export async function startHttps(args: StartHttpsArgs): Promise<NetworkStatus> {
  return invoke<NetworkStatus>("network_start_https", { args });
}

export async function stopHttps(): Promise<void> {
  return invoke<void>("network_stop_https");
}

export async function regenToken(): Promise<{ fingerprint: string }> {
  return invoke("network_regen_token");
}

export interface StartApiArgs {
  port: number;
  legacy_ftp: boolean;
}

export async function startApi(args: StartApiArgs): Promise<void> {
  return invoke<void>("network_start_api", { args });
}

export async function stopApi(): Promise<void> {
  return invoke<void>("network_stop_api");
}
