// TASK-BR1 — the bug reporter's IPC surface.
//
// Through `call` rather than raw `invoke`, like every other command
// family here, so `setIpcMock` can stub it in a unit test. The DTOs
// mirror `apps/freally-ui/src-tauri/src/bugreport.rs` field for field.

import { call } from "./client";

/** Whether the last run left a crash behind.
 *
 *  Only this one field: everything the dialog *displays* comes from
 *  `preview()`, which builds it with the same Rust code the submit path
 *  uses — a preview assembled separately would be a preview of nothing. */
export interface BugReportContext {
  pendingCrash: string | null;
}

/** Where a report can be sent. Each opens a pre-filled draft and stops. */
export type ReportTarget = "github" | "gmail" | "email";

export function context(): Promise<BugReportContext> {
  return call<BugReportContext>("bug_report_context");
}

/** The exact text that would be sent, subject line included. */
export function preview(description: string, includeCrash: boolean): Promise<string> {
  return call<string>("bug_report_preview", { description, includeCrash });
}

/** Open a pre-filled draft. **Sends nothing** — the user still clicks
 *  send in their own client. */
export function submit(
  target: ReportTarget,
  description: string,
  includeCrash: boolean
): Promise<void> {
  return call<void>("bug_report_submit", { target, description, includeCrash });
}

/** Discard the pending crash report(s). */
export function clearCrash(): Promise<void> {
  return call<void>("bug_report_clear_crash");
}
