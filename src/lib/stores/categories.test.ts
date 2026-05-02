import { describe, it, expect, beforeEach, vi } from "vitest";

const listCategories = vi.fn();
const setTorrentCategory = vi.fn();

vi.mock("$lib/bindings", () => ({
  commands: {
    listCategories: () => listCategories(),
    setTorrentCategory: (h: string, c: string | null) =>
      setTorrentCategory(h, c),
  },
}));

import { categories, UNCATEGORIZED } from "./categories.svelte";

describe("categories store", () => {
  beforeEach(() => {
    listCategories.mockReset();
    setTorrentCategory.mockReset();
    categories.list = [];
    categories.filter = null;
  });

  it("starts with empty list and 'All' filter (null)", () => {
    expect(categories.list).toEqual([]);
    expect(categories.filter).toBeNull();
  });

  it("UNCATEGORIZED sentinel is distinct from null", () => {
    expect(UNCATEGORIZED).not.toBeNull();
    expect(typeof UNCATEGORIZED).toBe("string");
  });

  it("select() updates filter to a category name, sentinel, or null", () => {
    categories.select("movies");
    expect(categories.filter).toBe("movies");
    categories.select(UNCATEGORIZED);
    expect(categories.filter).toBe(UNCATEGORIZED);
    categories.select(null);
    expect(categories.filter).toBeNull();
  });

  it("refresh() populates list from listCategories command", async () => {
    listCategories.mockResolvedValueOnce({
      status: "ok",
      data: [
        { name: "movies", count: 3 },
        { name: "books", count: 1 },
      ],
    });
    await categories.refresh();
    expect(categories.list).toEqual([
      { name: "movies", count: 3 },
      { name: "books", count: 1 },
    ]);
  });

  it("refresh() swallows errors and leaves list unchanged", async () => {
    categories.list = [{ name: "stale", count: 9 }];
    listCategories.mockResolvedValueOnce({
      status: "error",
      error: "boom",
    });
    await categories.refresh();
    expect(categories.list).toEqual([{ name: "stale", count: 9 }]);
  });

  it("assign() calls setTorrentCategory then refreshes", async () => {
    setTorrentCategory.mockResolvedValueOnce({ status: "ok", data: null });
    listCategories.mockResolvedValueOnce({
      status: "ok",
      data: [{ name: "movies", count: 1 }],
    });
    await categories.assign("hash123", "movies");
    expect(setTorrentCategory).toHaveBeenCalledWith("hash123", "movies");
    expect(listCategories).toHaveBeenCalled();
    expect(categories.list).toEqual([{ name: "movies", count: 1 }]);
  });

  it("assign() with null unassigns and still refreshes", async () => {
    setTorrentCategory.mockResolvedValueOnce({ status: "ok", data: null });
    listCategories.mockResolvedValueOnce({ status: "ok", data: [] });
    await categories.assign("hash123", null);
    expect(setTorrentCategory).toHaveBeenCalledWith("hash123", null);
  });

  it("assign() rejects when the underlying command errors", async () => {
    setTorrentCategory.mockResolvedValueOnce({
      status: "error",
      error: "denied",
    });
    await expect(categories.assign("h", "movies")).rejects.toBe("denied");
    expect(listCategories).not.toHaveBeenCalled();
  });
});
