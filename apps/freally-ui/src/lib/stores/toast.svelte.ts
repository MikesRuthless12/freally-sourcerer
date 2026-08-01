// Main-window toast surface.
//
// Build 1's file-ops verbs (export a file list, copy contents, run a
// custom command) all need to report "it worked" or "it didn't" without
// stealing focus. The Settings panels each grew their own local toast
// variable in Phase 12; this is the shared one for the main window.
//
// One toast at a time — a burst of them stacking over the result list
// would obscure the thing the user just acted on.

const AUTO_DISMISS_MS = 4000;

export type ToastTone = "info" | "error";

interface Toast {
  /** Bumped on every `show()`, including when the message is identical.
   *  `ToastHost` keys on it so the node remounts — an `aria-live`
   *  region does not re-announce text that never changed, so copying
   *  two files in a row would otherwise be silent to a screen reader. */
  id: number;
  message: string;
  tone: ToastTone;
}

class ToastStore {
  current = $state<Toast | null>(null);
  private nextId = 1;
  private timer: ReturnType<typeof setTimeout> | null = null;

  show(message: string, tone: ToastTone = "info") {
    if (this.timer !== null) clearTimeout(this.timer);
    this.current = { id: this.nextId++, message, tone };
    this.timer = setTimeout(() => this.dismiss(), AUTO_DISMISS_MS);
  }

  error(message: string) {
    this.show(message, "error");
  }

  dismiss() {
    if (this.timer !== null) {
      clearTimeout(this.timer);
      this.timer = null;
    }
    this.current = null;
  }
}

export const toastStore = new ToastStore();
