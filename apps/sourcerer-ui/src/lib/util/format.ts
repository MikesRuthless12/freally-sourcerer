// UI-side formatters. Keep them deterministic; tests pin against them.

/** View → Display format → Size format. `auto_binary` matches voidtools
 *  Everything's behavior (picks the unit that fits); the fixed units
 *  force KB / MB / GB regardless of magnitude. */
export type SizeFormat = "auto_binary" | "bytes" | "kb" | "mb" | "gb";

export function formatBytes(n: number, fmt: SizeFormat = "auto_binary"): string {
  if (!Number.isFinite(n) || n < 0) return "—";
  if (fmt === "bytes") return `${n.toLocaleString()} B`;
  if (fmt === "kb") return `${(n / 1024).toFixed(1)} KB`;
  if (fmt === "mb") return `${(n / 1024 / 1024).toFixed(2)} MB`;
  if (fmt === "gb") return `${(n / 1024 / 1024 / 1024).toFixed(2)} GB`;
  // auto_binary — pick the unit that fits.
  if (n < 1024) return `${n} B`;
  const units = ["KB", "MB", "GB", "TB"];
  let v = n / 1024;
  let i = 0;
  while (v >= 1024 && i < units.length - 1) {
    v /= 1024;
    i++;
  }
  return `${v.toFixed(v < 10 ? 1 : 0)} ${units[i]}`;
}

export function formatDateMs(ms: number): string {
  const d = new Date(ms);
  const yyyy = d.getFullYear();
  const mm = String(d.getMonth() + 1).padStart(2, "0");
  const dd = String(d.getDate()).padStart(2, "0");
  const hh = String(d.getHours()).padStart(2, "0");
  const mi = String(d.getMinutes()).padStart(2, "0");
  return `${yyyy}-${mm}-${dd} ${hh}:${mi}`;
}

export function formatCount(n: number): string {
  return n.toLocaleString();
}

export function formatLensTiming(ms: number): string {
  if (ms < 1) return "<1 ms";
  return `${Math.round(ms)} ms`;
}
