// Command registry. Every menu item, keyboard shortcut, and palette entry
// dispatches through `registry.run(id)`. Handlers register themselves at
// startup; tests can introspect via `registry.has(id)` / fire via `run`.

import type { CommandId } from "./ids";

export type CommandHandler = (args?: unknown) => void | Promise<void>;

class Registry {
  private map = new Map<CommandId, CommandHandler>();
  private fired = $state<{ id: CommandId; at: number } | null>(null);

  register(id: CommandId, handler: CommandHandler) {
    this.map.set(id, handler);
  }

  has(id: CommandId): boolean {
    return this.map.has(id);
  }

  async run(id: CommandId, args?: unknown): Promise<void> {
    const h = this.map.get(id);
    if (!h) {
      console.warn(`[registry] no handler for ${id}`);
      return;
    }
    this.fired = { id, at: performance.now() };
    await h(args);
  }

  /** Last-fired command — exposed for the wiring tests. */
  get lastFired(): { id: CommandId; at: number } | null {
    return this.fired;
  }
}

export const registry = new Registry();
