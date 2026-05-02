import { describe, it, expect, vi } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import { tick } from "svelte";
import Modal from "./Modal.svelte";
import { createRawSnippet } from "svelte";

/**
 * Build a Snippet from a static HTML string so we can pass children to Modal
 * from a plain TS test without needing a wrapper .svelte file.
 */
function snippetFromHtml(html: string) {
  return createRawSnippet(() => ({ render: () => html }));
}

describe("Modal", () => {
  it("renders the title text and children when open", async () => {
    const { container, getByText } = render(Modal, {
      open: true,
      title: "Add Torrent",
      onclose: () => {},
      children: snippetFromHtml("<p>hello body</p>"),
    });
    await tick();
    expect(getByText("Add Torrent")).toBeInTheDocument();
    expect(getByText("hello body")).toBeInTheDocument();
    expect(container.querySelector("dialog")).not.toBeNull();
  });

  it("calls onclose when the × button is clicked", async () => {
    const onclose = vi.fn();
    const { getByLabelText } = render(Modal, {
      open: true,
      title: "T",
      onclose,
      children: snippetFromHtml("<span>x</span>"),
    });
    await tick();
    await fireEvent.click(getByLabelText("Close"));
    expect(onclose).toHaveBeenCalledTimes(1);
  });

  it("calls onclose when the dialog backdrop is clicked", async () => {
    const onclose = vi.fn();
    const { container } = render(Modal, {
      open: true,
      title: "T",
      onclose,
      children: snippetFromHtml("<span>x</span>"),
    });
    await tick();
    const dialog = container.querySelector("dialog")!;
    // Modal's onBackdropClick treats a click whose target IS the dialog
    // (not a child) as a backdrop click.
    await fireEvent.click(dialog);
    expect(onclose).toHaveBeenCalledTimes(1);
  });

  it("does NOT call onclose when clicking inside the body", async () => {
    const onclose = vi.fn();
    const { getByText } = render(Modal, {
      open: true,
      title: "T",
      onclose,
      children: snippetFromHtml("<button>inner</button>"),
    });
    await tick();
    await fireEvent.click(getByText("inner"));
    expect(onclose).not.toHaveBeenCalled();
  });

  it("opens the dialog via showModal when open=true", async () => {
    const { container } = render(Modal, {
      open: true,
      title: "T",
      onclose: () => {},
      children: snippetFromHtml("<p>x</p>"),
    });
    await tick();
    const dialog = container.querySelector("dialog")! as HTMLDialogElement;
    expect(dialog.hasAttribute("open")).toBe(true);
  });
});
