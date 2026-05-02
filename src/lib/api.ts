// Thin convenience over the auto-generated bindings.
// `commands.foo(...)` returns Promise<{status:"ok",data}|{status:"error",error}>;
// `unwrap(...)` collapses that into Promise<T> that throws on error.

export async function unwrap<T, E>(
  p: Promise<{ status: "ok"; data: T } | { status: "error"; error: E }>,
): Promise<T> {
  const r = await p;
  if (r.status === "ok") return r.data;
  throw r.error;
}
