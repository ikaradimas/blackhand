import { describe, it, expect } from "vitest";
import { render } from "@testing-library/svelte";
import ProgressBar from "./ProgressBar.svelte";

function counts(container: HTMLElement) {
  const cells = container.querySelectorAll(".cell");
  const filled = container.querySelectorAll(".cell.on");
  return { total: cells.length, filled: filled.length };
}

describe("ProgressBar", () => {
  it("renders 32 cells by default", () => {
    const { container } = render(ProgressBar, { pct: 50 });
    expect(counts(container).total).toBe(32);
  });

  it("renders zero filled cells at 0%", () => {
    const { container } = render(ProgressBar, { pct: 0 });
    expect(counts(container).filled).toBe(0);
  });

  it("renders all 32 cells filled at 100%", () => {
    const { container } = render(ProgressBar, { pct: 100 });
    expect(counts(container).filled).toBe(32);
  });

  it("renders 16 filled cells at 50%", () => {
    const { container } = render(ProgressBar, { pct: 50 });
    expect(counts(container).filled).toBe(16);
  });

  it("clamps negative percentages to 0", () => {
    const { container } = render(ProgressBar, { pct: -10 });
    expect(counts(container).filled).toBe(0);
  });

  it("clamps percentages > 100 to 100", () => {
    const { container } = render(ProgressBar, { pct: 150 });
    expect(counts(container).filled).toBe(32);
  });

  it("respects a custom cells prop", () => {
    const { container } = render(ProgressBar, { pct: 50, cells: 10 });
    expect(counts(container).total).toBe(10);
    expect(counts(container).filled).toBe(5);
  });

  it("rounds to the nearest cell at fractional progress", () => {
    // 1/32 ≈ 3.125%; at 4% we should round to 1 cell.
    const { container } = render(ProgressBar, { pct: 4 });
    expect(counts(container).filled).toBe(1);
    // At 5% (5×32/100 = 1.6) we should round to 2.
    const { container: c2 } = render(ProgressBar, { pct: 5 });
    expect(counts(c2).filled).toBe(2);
  });

  it("sets data-finished='true' when finished", () => {
    const { container } = render(ProgressBar, { pct: 100, finished: true });
    expect(
      container.querySelector(".bar")?.getAttribute("data-finished"),
    ).toBe("true");
  });

  it("sets data-paused='true' when paused", () => {
    const { container } = render(ProgressBar, { pct: 50, paused: true });
    expect(
      container.querySelector(".bar")?.getAttribute("data-paused"),
    ).toBe("true");
  });

  it("defaults finished and paused to false", () => {
    const { container } = render(ProgressBar, { pct: 50 });
    const bar = container.querySelector(".bar")!;
    expect(bar.getAttribute("data-finished")).toBe("false");
    expect(bar.getAttribute("data-paused")).toBe("false");
  });
});
