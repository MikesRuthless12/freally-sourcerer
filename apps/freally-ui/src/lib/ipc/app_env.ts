// What build am I running, and where does it keep its files? (SRC-M17)

import { call } from "./client";

export interface AppEnvironment {
  /** Semver from the bundle manifest — read at runtime, never hardcoded. */
  version: string;
  /** True when this is a portable install writing to `Data/`. */
  portable: boolean;
  /** The `Data/` folder, when portable. */
  data_dir: string | null;
}

export function environment(): Promise<AppEnvironment> {
  return call<AppEnvironment>("app_environment");
}
