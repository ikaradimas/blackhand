import { describe, it, expect } from "vitest";
import { render } from "@testing-library/svelte";
import PixelMark from "./PixelMark.svelte";

describe("PixelMark", () => {
  it("renders an svg with a 0..5 viewBox", () => {
    const { container } = render(PixelMark, { hash: "abc123" });
    const svg = container.querySelector("svg");
    expect(svg).not.toBeNull();
    expect(svg!.getAttribute("viewBox")).toBe("0 0 5 5");
  });

  it("respects the size prop", () => {
    const { container } = render(PixelMark, { hash: "abc", size: 24 });
    const svg = container.querySelector("svg")!;
    expect(svg.getAttribute("width")).toBe("24");
    expect(svg.getAttribute("height")).toBe("24");
  });

  it("defaults size to 18 when not provided", () => {
    const { container } = render(PixelMark, { hash: "abc" });
    const svg = container.querySelector("svg")!;
    expect(svg.getAttribute("width")).toBe("18");
  });

  it("is deterministic — same hash produces the same rect set", () => {
    const a = render(PixelMark, { hash: "deadbeef" });
    const b = render(PixelMark, { hash: "deadbeef" });

    const rectsA = Array.from(a.container.querySelectorAll("rect")).map(
      (r) => `${r.getAttribute("x")},${r.getAttribute("y")}`,
    );
    const rectsB = Array.from(b.container.querySelectorAll("rect")).map(
      (r) => `${r.getAttribute("x")},${r.getAttribute("y")}`,
    );
    expect(rectsA.sort()).toEqual(rectsB.sort());
  });

  it("different hashes generally produce different patterns", () => {
    // Probabilistically true; with two hand-picked hashes we can be deterministic.
    const a = render(PixelMark, { hash: "0000" });
    const b = render(PixelMark, { hash: "ffff" });
    const countA = a.container.querySelectorAll("rect").length;
    const countB = b.container.querySelectorAll("rect").length;
    // The hashes "0000" and "ffff" hit different bit patterns; at minimum we
    // expect *some* difference in the rendered set.
    const rectsA = Array.from(a.container.querySelectorAll("rect"))
      .map((r) => `${r.getAttribute("x")},${r.getAttribute("y")}`)
      .sort()
      .join("|");
    const rectsB = Array.from(b.container.querySelectorAll("rect"))
      .map((r) => `${r.getAttribute("x")},${r.getAttribute("y")}`)
      .sort()
      .join("|");
    expect(rectsA === rectsB && countA === countB).toBe(false);
  });

  it("is vertically mirrored across the center column", () => {
    const { container } = render(PixelMark, { hash: "mirror-test" });
    const cells = new Set<string>();
    for (const r of container.querySelectorAll("rect")) {
      cells.add(`${r.getAttribute("x")},${r.getAttribute("y")}`);
    }
    // For every (x,y) where x !== 2, the mirrored cell (4-x, y) must also be present.
    for (const cell of cells) {
      const [x, y] = cell.split(",").map(Number);
      if (x === 2) continue;
      expect(cells.has(`${4 - x},${y}`)).toBe(true);
    }
  });

  it("only emits rects in the 0..4 x/y range", () => {
    const { container } = render(PixelMark, { hash: "any" });
    for (const r of container.querySelectorAll("rect")) {
      const x = Number(r.getAttribute("x"));
      const y = Number(r.getAttribute("y"));
      expect(x).toBeGreaterThanOrEqual(0);
      expect(x).toBeLessThanOrEqual(4);
      expect(y).toBeGreaterThanOrEqual(0);
      expect(y).toBeLessThanOrEqual(4);
    }
  });
});
