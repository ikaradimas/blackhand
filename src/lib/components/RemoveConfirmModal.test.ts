import { describe, it, expect, beforeEach, vi } from "vitest";
import { render, fireEvent, waitFor } from "@testing-library/svelte";

const deleteFn = vi.fn();

vi.mock("$lib/bindings", () => ({
  commands: {
    delete: (id: number) => deleteFn(id),
  },
}));

import { ui } from "$lib/stores/ui.svelte";
import { toasts } from "$lib/stores/toasts.svelte";
import RemoveConfirmModal from "./RemoveConfirmModal.svelte";

describe("RemoveConfirmModal", () => {
  beforeEach(() => {
    deleteFn.mockReset();
    ui.cancelRemove();
    toasts.list = [];
  });

  it("does not render its body when no removeTarget is set", () => {
    const { queryByText } = render(RemoveConfirmModal);
    expect(queryByText(/this cannot be undone/i)).toBeNull();
  });

  it("renders the target name when askRemove is called", async () => {
    const { findByText } = render(RemoveConfirmModal);
    ui.askRemove({ id: 5, name: "ubuntu.iso" });
    expect(await findByText("ubuntu.iso")).toBeInTheDocument();
  });

  it("Cancel closes the modal without calling delete", async () => {
    const { findByText } = render(RemoveConfirmModal);
    ui.askRemove({ id: 5, name: "x" });
    const cancel = await findByText("Cancel");
    await fireEvent.click(cancel);
    expect(deleteFn).not.toHaveBeenCalled();
    expect(ui.removeTarget).toBeNull();
  });

  it("Confirm calls commands.delete and clears the target on success", async () => {
    deleteFn.mockResolvedValueOnce({ status: "ok", data: null });
    const { findByText } = render(RemoveConfirmModal);
    ui.askRemove({ id: 7, name: "torrent" });
    const confirm = await findByText("Delete + remove files");
    await fireEvent.click(confirm);
    await waitFor(() => expect(deleteFn).toHaveBeenCalledWith(7));
    await waitFor(() => expect(ui.removeTarget).toBeNull());
  });

  it("renders an inline error and keeps the target when delete fails", async () => {
    deleteFn.mockResolvedValueOnce({ status: "error", error: "denied" });
    const { findByText } = render(RemoveConfirmModal);
    ui.askRemove({ id: 7, name: "x" });
    const confirm = await findByText("Delete + remove files");
    await fireEvent.click(confirm);
    expect(await findByText("denied")).toBeInTheDocument();
    expect(ui.removeTarget).not.toBeNull();
  });

  it("clears the inline error when a new target is set", async () => {
    deleteFn.mockResolvedValueOnce({ status: "error", error: "first" });
    const { findByText, queryByText } = render(RemoveConfirmModal);
    ui.askRemove({ id: 1, name: "a" });
    await fireEvent.click(await findByText("Delete + remove files"));
    expect(await findByText("first")).toBeInTheDocument();

    ui.askRemove({ id: 2, name: "b" });
    await waitFor(() => expect(queryByText("first")).toBeNull());
  });
});
