import type { DiskSpace } from "$lib/bindings";

export type DiskLevel = "ok" | "warn" | "err";

const KB = 1024;
const MB = KB * 1024;
const GB = MB * 1024;
const TB = GB * 1024;

/** Threshold for "tight" — under 10% of capacity OR under 5 GB free. */
const WARN_FREE_BYTES = 5 * GB;
const WARN_FREE_FRACTION = 0.1;

/** Threshold for "critical" — under 500 MB free, or zero. */
const ERR_FREE_BYTES = 500 * MB;

export function diskLevel(free: number, total: number): DiskLevel {
  if (free <= 0 || free < ERR_FREE_BYTES) return "err";
  if (total > 0 && (free < WARN_FREE_BYTES || free / total < WARN_FREE_FRACTION)) {
    return "warn";
  }
  return "ok";
}

export function fmtBytes(n: number): string {
  if (!Number.isFinite(n) || n < 0) return "—";
  const units: [number, string][] = [
    [TB, "TB"],
    [GB, "GB"],
    [MB, "MB"],
    [KB, "KB"],
  ];
  for (const [unit, label] of units) {
    if (n >= unit) {
      const v = n / unit;
      return `${v.toFixed(v < 10 ? 1 : 0)} ${label}`;
    }
  }
  return `${n} B`;
}

/** "12.4 GB free of 500 GB" — used in the SettingsModal hint. */
export function describeDisk(info: DiskSpace): string {
  return `${fmtBytes(info.free_bytes)} free of ${fmtBytes(info.total_bytes)}`;
}
