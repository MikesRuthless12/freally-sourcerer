// Thin typed wrapper around Tauri's invoke. Centralizing the surface lets
// tests stub it via `setIpcMock(...)` without touching call sites.

import { invoke } from "@tauri-apps/api/core";

type Invoker = <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;

let invoker: Invoker = invoke as Invoker;

export function setIpcMock(fn: Invoker | null) {
  invoker = fn ?? (invoke as Invoker);
}

export function call<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  return invoker<T>(cmd, args);
}
