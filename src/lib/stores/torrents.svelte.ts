import { commands, events, type TorrentSummary } from "$lib/bindings";
import { unwrap } from "$lib/api";

class TorrentsStore {
  list = $state<TorrentSummary[]>([]);
  #started = false;

  async start() {
    if (this.#started) return;
    this.#started = true;
    try {
      const initial = await unwrap(commands.listTorrents());
      this.list = initial.torrents;
    } catch {
      // initial-fetch failures are tolerable; live events take over
    }
    await events.torrentsSnapshotEvent.listen((e) => {
      this.list = e.payload.torrents;
    });
  }
}

export const torrents = new TorrentsStore();
