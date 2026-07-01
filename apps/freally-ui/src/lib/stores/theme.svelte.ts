// Theme store — system | light | dark tri-state. Applies <html data-theme>.
// `system` = no attribute, lets prefers-color-scheme drive the tokens.
//
// Persistence lives in settingsStore (single source of truth). This store
// is a reactive view + DOM applier so the theme flips instantly without
// blocking on the IPC round-trip; bootstrap.ts seeds `choice` from
// settingsStore.state.theme on first paint.

export type ThemeChoice = "system" | "light" | "dark";

function applyToDom(choice: ThemeChoice) {
  if (typeof document === "undefined") return;
  const html = document.documentElement;
  if (choice === "system") {
    html.removeAttribute("data-theme");
  } else {
    html.setAttribute("data-theme", choice);
  }
}

class ThemeStore {
  choice = $state<ThemeChoice>("system");

  set(next: ThemeChoice) {
    this.choice = next;
    applyToDom(next);
  }

  cycle() {
    const order: ThemeChoice[] = ["system", "light", "dark"];
    const idx = order.indexOf(this.choice);
    this.set(order[(idx + 1) % order.length]!);
  }
}

export const themeStore = new ThemeStore();
