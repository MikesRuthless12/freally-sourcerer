// SRC-M15 bulk rename + SRC-M16 undo/redo.
//
// The rule crosses the wire; destination paths never do. The backend
// derives every new name itself and re-derives it at apply time, so what
// the preview shows is a display of the backend's own plan rather than
// an instruction it trusts.

import { call } from "./client";
import type {
  OperationListing,
  RenamePreview,
  RenameOutcome,
  RenameRule,
  UndoOutcome
} from "./types";

export function preview(paths: string[], rule: RenameRule): Promise<RenamePreview> {
  return call<RenamePreview>("files_rename_preview", { paths, rule });
}

export function apply(paths: string[], rule: RenameRule): Promise<RenameOutcome> {
  return call<RenameOutcome>("files_rename_apply", { paths, rule });
}

export function opsList(): Promise<OperationListing> {
  return call<OperationListing>("ops_list");
}

export function undo(id: string): Promise<UndoOutcome> {
  return call<UndoOutcome>("ops_undo", { id });
}

export function redo(id: string): Promise<UndoOutcome> {
  return call<UndoOutcome>("ops_redo", { id });
}
