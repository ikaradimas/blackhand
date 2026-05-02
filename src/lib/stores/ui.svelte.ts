// Global UI state — modal visibility, etc.

export type RemoveTarget = { id: number; name: string };

class UIStore {
  addModal = $state(false);
  settingsModal = $state(false);
  aboutModal = $state(false);
  /** When set, a remove-with-files confirmation is pending for this torrent. */
  removeTarget = $state<RemoveTarget | null>(null);

  openAdd() {
    this.addModal = true;
  }
  closeAdd() {
    this.addModal = false;
  }
  openSettings() {
    this.settingsModal = true;
  }
  closeSettings() {
    this.settingsModal = false;
  }
  openAbout() {
    this.aboutModal = true;
  }
  closeAbout() {
    this.aboutModal = false;
  }
  askRemove(target: RemoveTarget) {
    this.removeTarget = target;
  }
  cancelRemove() {
    this.removeTarget = null;
  }
}

export const ui = new UIStore();
