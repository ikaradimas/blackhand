import { describe, it, expect } from "vitest";
import { unwrap } from "./api";

describe("unwrap", () => {
  it("returns data on ok status", async () => {
    const r = await unwrap(Promise.resolve({ status: "ok" as const, data: 42 }));
    expect(r).toBe(42);
  });

  it("throws the error payload on error status", async () => {
    const err = { code: "boom", message: "kaboom" };
    await expect(
      unwrap(Promise.resolve({ status: "error" as const, error: err })),
    ).rejects.toBe(err);
  });

  it("preserves typing of complex data shapes", async () => {
    const data = { id: 1, name: "x", tags: ["a", "b"] };
    const r = await unwrap(Promise.resolve({ status: "ok" as const, data }));
    expect(r).toEqual(data);
  });

  it("propagates a rejected promise as a thrown error (not wrapped)", async () => {
    const native = new Error("network");
    await expect(unwrap(Promise.reject(native))).rejects.toBe(native);
  });
});
