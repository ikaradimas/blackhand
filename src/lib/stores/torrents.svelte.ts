import { commands, events, type TorrentSummary } from "$lib/bindings";
import { unwrap } from "$lib/api";

class TorrentsStore {
  list = $state<TorrentSummary[]>([]);
  /** Becomes true after the first successful fetch or first event tick. */
  loaded = $state(false);
  #started = false;

  async start() {
    if (this.#started) return;
    this.#started = true;
    try {
      const initial = await unwrap(commands.listTorrents());
      this.list = initial.torrents;
      this.loaded = true;
    } catch {
      // initial-fetch failures are tolerable; live events take over
    }
    await events.torrentsSnapshotEvent.listen((e) => {
      this.list = e.payload.torrents;
      this.loaded = true;
    });
  }
}

export const torrents = new TorrentsStore();
