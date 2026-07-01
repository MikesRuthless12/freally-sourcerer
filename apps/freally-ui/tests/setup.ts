// Vitest setup — clears stateful singletons between tests so cross-test
// pollution doesn't sneak in once the suite grows.

import { beforeEach } from "vitest";
import { setIpcMock } from "../src/lib/ipc/client";

beforeEach(() => {
  if (typeof localStorage !== "undefined") {
    localStorage.clear();
  }
  if (typeof document !== "undefined") {
    document.documentElement.removeAttribute("data-theme");
  }
  setIpcMock(null);
});
