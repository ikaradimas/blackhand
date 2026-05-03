import { commands, events, type TorrentSummary } from "$lib/bindings";
import { unwrap } from "$lib/api";

class TorrentsStore {
  list = $state<TorrentSummary[]>([]);
  /** Becomes true after the first successful fetch or first event tick. */
  loaded = $state(false);
  #started = false;

  /** Force a fresh snapshot now. Safe to call from anywhere; useful for
   * surfaces (tray popup) whose webview may have missed events while
   * hidden / not yet booted. */
  async refresh() {
    try {
      const snapshot = await unwrap(commands.listTorrents());
      this.list = snapshot.torrents;
      this.loaded = true;
    } catch {
      // tolerable — caller will retry on next show
    }
  }

  async start() {
    if (this.#started) return;
    this.#started = true;
    await this.refresh();
    await events.torrentsSnapshotEvent.listen((e) => {
      this.list = e.payload.torrents;
      this.loaded = true;
    });
  }
}

export const torrents = new TorrentsStore();
