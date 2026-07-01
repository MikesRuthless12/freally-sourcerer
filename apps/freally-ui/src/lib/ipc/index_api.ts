import { call } from "./client";
import type { IndexState } from "./types";

export function state(): Promise<IndexState> {
  return call<IndexState>("index_state");
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
