import { describe, it, expect, beforeEach, vi } from "vitest";
import { render, fireEvent, waitFor } from "@testing-library/svelte";
import { tick } from "svelte";

const addMagnet = vi.fn();
const addTorrentFile = vi.fn();

vi.mock("$lib/bindings", () => ({
  commands: {
    addMagnet: (uri: string) => addMagnet(uri),
    addTorrentFile: (bytes: number[]) => addTorrentFile(bytes),
  },
}));

import { ui } from "$lib/stores/ui.svelte";
import { toasts } from "$lib/stores/toasts.svelte";
import AddTorrentModal from "./AddTorrentModal.svelte";

describe("AddTorrentModal", () => {
  beforeEach(() => {
    addMagnet.mockReset();
    addTorrentFile.mockReset();
    ui.closeAdd();
    toasts.list = [];
  });

  it("renders with the dialog closed initially", async () => {
    const { container } = render(AddTorrentModal);
    await tick();
    const dialog = container.querySelector("dialog");
    expect(dialog?.hasAttribute("open")).toBe(false);
  });

  it("opens the dialog when ui.openAdd is called", async () => {
    const { container } = render(AddTorrentModal);
    ui.openAdd();
    await tick();
    expect(container.querySelector("dialog")?.hasAttribute("open")).toBe(true);
  });

  it("Add button is disabled with no magnet and no file", async () => {
    const { findByText } = render(AddTorrentModal);
    ui.openAdd();
    const addBtn = (await findByText("Add")) as HTMLButtonElement;
    expect(addBtn.disabled).toBe(true);
  });

  it("typing a magnet enables the Add button", async () => {
    const { findByText, container } = render(AddTorrentModal);
    ui.openAdd();
    await tick();
    const input = container.querySelector(
      'input[type="text"]',
    ) as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "magnet:?xt=urn:btih:abc" } });
    const addBtn = (await findByText("Add")) as HTMLButtonElement;
    expect(addBtn.disabled).toBe(false);
  });

  it("submitting calls commands.addMagnet and closes on success", async () => {
    addMagnet.mockResolvedValueOnce({
      status: "ok",
      data: { id: 1, info_hash: "abc", name: "Movie" },
    });
    const { findByText, container } = render(AddTorrentModal);
    ui.openAdd();
    await tick();
    const input = container.querySelector(
      'input[type="text"]',
    ) as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "magnet:?xt=urn:btih:abc" } });
    await fireEvent.click(await findByText("Add"));
    await waitFor(() =>
      expect(addMagnet).toHaveBeenCalledWith("magnet:?xt=urn:btih:abc"),
    );
    await waitFor(() => expect(ui.addModal).toBe(false));
  });

  it("renders an inline error when addMagnet fails", async () => {
    addMagnet.mockResolvedValueOnce({ status: "error", error: "bad magnet" });
    const { findByText, container } = render(AddTorrentModal);
    ui.openAdd();
    await tick();
    const input = container.querySelector(
      'input[type="text"]',
    ) as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "magnet:?xt=urn:btih:abc" } });
    await fireEvent.click(await findByText("Add"));
    expect(await findByText("bad magnet")).toBeInTheDocument();
    // Modal stays open so the user can correct.
    expect(ui.addModal).toBe(true);
  });

  it("rejects non-.torrent files with an inline error", async () => {
    const { findByText, container } = render(AddTorrentModal);
    ui.openAdd();
    await tick();
    const fileInput = container.querySelector(
      'input[type="file"]',
    ) as HTMLInputElement;

    const file = new File(["nope"], "movie.mp4", { type: "video/mp4" });
    Object.defineProperty(fileInput, "files", { value: [file] });
    await fireEvent.change(fileInput);

    expect(
      await findByText(/expected \.torrent file, got "movie\.mp4"/i),
    ).toBeInTheDocument();
    expect(addTorrentFile).not.toHaveBeenCalled();
  });

  it("typing in the magnet input clears a stale error", async () => {
    addMagnet.mockResolvedValueOnce({ status: "error", error: "stale" });
    const { findByText, queryByText, container } = render(AddTorrentModal);
    ui.openAdd();
    await tick();
    const input = container.querySelector(
      'input[type="text"]',
    ) as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "magnet:?xt=urn:btih:abc" } });
    await fireEvent.click(await findByText("Add"));
    expect(await findByText("stale")).toBeInTheDocument();

    await fireEvent.input(input, { target: { value: "magnet:?xt=urn:btih:abcd" } });
    await waitFor(() => expect(queryByText("stale")).toBeNull());
  });
});
