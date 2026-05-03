import { commands, type DiskSpace } from "$lib/bindings";
import { unwrap } from "$lib/api";

const POLL_MS = 30_000;

class DiskStore {
  info = $state<DiskSpace | null>(null);
  #timer: number | null = null;
  #started = false;

  async refresh() {
    try {
      this.info = await unwrap(commands.diskSpace(null));
    } catch {
      this.info = null;
    }
  }

  start() {
    if (this.#started) return;
    this.#started = true;
    void this.refresh();
    this.#timer = window.setInterval(() => void this.refresh(), POLL_MS);
  }
}

export const disk = new DiskStore();
