// Global UI state — modal visibility, etc.

class UIStore {
  addModal = $state(false);

  openAdd() {
    this.addModal = true;
  }
  closeAdd() {
    this.addModal = false;
  }
}

export const ui = new UIStore();
