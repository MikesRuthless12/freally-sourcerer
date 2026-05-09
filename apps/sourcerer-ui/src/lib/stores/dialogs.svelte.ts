// Modal dialog store — single-active-modal pattern. Phase 11 surfaces three
// modals (about / organize-bookmarks / settings-placeholder); the wizard
// has its own gate via settings.first_run_complete.

export type ModalId =
  | null
  | "about"
  | "organize_bookmarks"
  | "settings"
  | "connect_endpoint"
  | "custom_extractor_manager";

class DialogsStore {
  active = $state<ModalId>(null);

  open(id: Exclude<ModalId, null>) {
    this.active = id;
  }
  close() {
    this.active = null;
  }
}

export const dialogsStore = new DialogsStore();
