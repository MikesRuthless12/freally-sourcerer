// Menu hover store — feeds the rightmost status-bar segment with the
// description of the currently-hovered menu item. Idle text comes from
// the indexed total.

class MenuHoverStore {
  hint = $state<string | null>(null);

  set(text: string) {
    this.hint = text;
  }

  clear() {
    this.hint = null;
  }
}

export const menuHoverStore = new MenuHoverStore();
