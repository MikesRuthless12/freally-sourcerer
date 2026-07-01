import { describe, it, expect } from "vitest";
import {
  formatBytes,
  formatCount,
  formatDateMs,
  formatLensTiming
} from "../../src/lib/util/format";

describe("formatBytes", () => {
  it("emits B for sub-1KiB values", () => {
    expect(formatBytes(0)).toBe("0 B");
    expect(formatBytes(512)).toBe("512 B");
    expect(formatBytes(1023)).toBe("1023 B");
  });
  it("emits KB / MB / GB / TB", () => {
    expect(formatBytes(1024)).toBe("1.0 KB");
    expect(formatBytes(1024 * 1024)).toBe("1.0 MB");
    expect(formatBytes(1024 ** 3)).toBe("1.0 GB");
    expect(formatBytes(1024 ** 4)).toBe("1.0 TB");
  });
  it("drops decimal beyond 10x", () => {
    expect(formatBytes(11 * 1024 * 1024)).toBe("11 MB");
  });
});

describe("formatLensTiming", () => {
  it("emits <1 ms for sub-millisecond values", () => {
    expect(formatLensTiming(0.4)).toBe("<1 ms");
  });
  it("rounds whole milliseconds", () => {
    expect(formatLensTiming(8)).toBe("8 ms");
    expect(formatLensTiming(8.4)).toBe("8 ms");
    expect(formatLensTiming(8.6)).toBe("9 ms");
  });
});

describe("formatCount", () => {
  it("uses locale separators", () => {
    expect(formatCount(0)).toBe("0");
    expect(formatCount(1234567)).toMatch(/^1[,.]234[,.]567$/);
  });
});

describe("formatDateMs", () => {
  it("emits zero-padded YYYY-MM-DD HH:MM for a real timestamp", () => {
    // 2024-01-15 14:30:45 UTC ≈ 1705329045000 ms. The actual rendered
    // date is locale-dependent (we render in the browser's local TZ),
    // so only check the *shape*.
    const out = formatDateMs(1_705_329_045_000);
    expect(out).toMatch(/^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$/);
  });

  it("renders em-dash for the daemon's `unknown timestamp` sentinel (0)", () => {
    // The daemon clamps negative mtime_ns to 0 before u64 cast; the UI
    // treats 0 as "unknown" rather than misleadingly showing 1970.
    expect(formatDateMs(0)).toBe("—");
  });

  it("renders em-dash for missing / null / undefined / NaN inputs", () => {
    expect(formatDateMs(null)).toBe("—");
    expect(formatDateMs(undefined)).toBe("—");
    expect(formatDateMs(NaN)).toBe("—");
    expect(formatDateMs(Infinity)).toBe("—");
    expect(formatDateMs(-Infinity)).toBe("—");
  });

  it("renders em-dash for out-of-Date-range values (regression for NaN-NaN-NaN bug)", () => {
    // The original bug: u64 wrap of a negative mtime_ns produced ~1.8e19
    // ms, which is far past JS Date's ±8.64e15 valid range. `new Date`
    // returned `Invalid Date` whose getters were all NaN, rendering as
    // "NaN-NaN-NaN NaN:NaN" across every result row. Guard against any
    // value past the Date-range cliff.
    expect(formatDateMs(1.8e19)).toBe("—");
    expect(formatDateMs(8_640_000_000_000_001)).toBe("—");
  });

  it("renders em-dash for negative timestamps", () => {
    expect(formatDateMs(-1)).toBe("—");
    expect(formatDateMs(-1_000_000_000)).toBe("—");
  });
});
