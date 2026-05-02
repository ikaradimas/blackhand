import { describe, it, expect, beforeEach } from "vitest";
import { ui } from "./ui.svelte";

describe("ui store", () => {
  beforeEach(() => {
    ui.closeAdd();
    ui.closeSettings();
    ui.closeAbout();
    ui.cancelRemove();
  });

  it("starts with all modals closed and no remove target", () => {
    expect(ui.addModal).toBe(false);
    expect(ui.settingsModal).toBe(false);
    expect(ui.aboutModal).toBe(false);
    expect(ui.removeTarget).toBeNull();
  });

  it("openAdd / closeAdd toggles only addModal", () => {
    ui.openAdd();
    expect(ui.addModal).toBe(true);
    expect(ui.settingsModal).toBe(false);
    expect(ui.aboutModal).toBe(false);
    ui.closeAdd();
    expect(ui.addModal).toBe(false);
  });

  it("openSettings / closeSettings toggles only settingsModal", () => {
    ui.openSettings();
    expect(ui.settingsModal).toBe(true);
    expect(ui.addModal).toBe(false);
    ui.closeSettings();
    expect(ui.settingsModal).toBe(false);
  });

  it("openAbout / closeAbout toggles only aboutModal", () => {
    ui.openAbout();
    expect(ui.aboutModal).toBe(true);
    ui.closeAbout();
    expect(ui.aboutModal).toBe(false);
  });

  it("askRemove sets the target and cancelRemove clears it", () => {
    const target = { id: 7, name: "ubuntu.iso" };
    ui.askRemove(target);
    expect(ui.removeTarget).toEqual(target);
    ui.cancelRemove();
    expect(ui.removeTarget).toBeNull();
  });

  it("askRemove can replace an existing target", () => {
    ui.askRemove({ id: 1, name: "a" });
    ui.askRemove({ id: 2, name: "b" });
    expect(ui.removeTarget).toEqual({ id: 2, name: "b" });
  });
});
