import { describe, it, expect } from "vitest";
import { diskLevel, fmtBytes, describeDisk } from "./disk";

const KB = 1024;
const MB = 1024 * KB;
const GB = 1024 * MB;

describe("diskLevel", () => {
  it("returns ok when free is plenty (>= 10% and >= 5 GB)", () => {
    expect(diskLevel(50 * GB, 100 * GB)).toBe("ok");
    expect(diskLevel(20 * GB, 200 * GB)).toBe("ok"); // 10%
    expect(diskLevel(100 * GB, 100 * GB)).toBe("ok"); // freshly empty disk
  });

  it("returns warn when below 10% of capacity", () => {
    expect(diskLevel(9 * GB, 100 * GB)).toBe("warn"); // 9%
  });

  it("returns warn when free < 5 GB even on a small disk", () => {
    // 4 GB free of 10 GB total — 40% but absolute is below threshold.
    expect(diskLevel(4 * GB, 10 * GB)).toBe("warn");
  });

  it("returns err when free is under 500 MB", () => {
    expect(diskLevel(400 * MB, 100 * GB)).toBe("err");
    expect(diskLevel(0, 100 * GB)).toBe("err");
  });

  it("returns err when free is exactly zero", () => {
    expect(diskLevel(0, 100 * GB)).toBe("err");
  });

  it("treats negative free as err", () => {
    expect(diskLevel(-1, 100 * GB)).toBe("err");
  });

  it("does not divide by zero on a degenerate total", () => {
    // Should fall back to absolute thresholds — if total is 0 we can't
    // compute a fraction, but free must still pass the byte check.
    expect(diskLevel(10 * GB, 0)).toBe("ok");
    expect(diskLevel(100 * MB, 0)).toBe("err");
  });
});

describe("fmtBytes", () => {
  it("formats GB / MB / KB / B with the right precision", () => {
    expect(fmtBytes(0)).toBe("0 B");
    expect(fmtBytes(512)).toBe("512 B");
    expect(fmtBytes(2 * KB)).toBe("2.0 KB");
    expect(fmtBytes(15 * KB)).toBe("15 KB");
    expect(fmtBytes(2.5 * MB)).toBe("2.5 MB");
    expect(fmtBytes(100 * GB)).toBe("100 GB");
  });

  it("handles negative or non-finite as em-dash", () => {
    expect(fmtBytes(-1)).toBe("—");
    expect(fmtBytes(NaN)).toBe("—");
    expect(fmtBytes(Infinity)).toBe("—");
  });
});

describe("describeDisk", () => {
  it("renders a 'free of total' string", () => {
    const info = { free_bytes: 10 * GB, total_bytes: 100 * GB, path: "/x" };
    expect(describeDisk(info)).toBe("10 GB free of 100 GB");
  });
});
