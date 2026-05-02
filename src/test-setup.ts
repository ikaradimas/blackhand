import "@testing-library/jest-dom/vitest";
import "@testing-library/svelte/vitest";

// jsdom does not implement HTMLDialogElement.showModal/close. Polyfill the
// minimum surface so that <dialog>-based components can be rendered and
// queried in tests.
if (typeof HTMLDialogElement !== "undefined") {
  if (!HTMLDialogElement.prototype.showModal) {
    HTMLDialogElement.prototype.showModal = function (this: HTMLDialogElement) {
      this.setAttribute("open", "");
    };
  }
  if (!HTMLDialogElement.prototype.close) {
    HTMLDialogElement.prototype.close = function (this: HTMLDialogElement) {
      this.removeAttribute("open");
      this.dispatchEvent(new Event("close"));
    };
  }
}
