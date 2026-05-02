import { commands, events, type SessionStats } from "$lib/bindings";
import { unwrap } from "$lib/api";

class SessionStore {
  stats = $state<SessionStats | null>(null);
  #started = false;

  async start() {
    if (this.#started) return;
    this.#started = true;
    try {
      this.stats = await unwrap(commands.sessionStats());
    } catch {
      // ignore initial-fetch failures; live events will fill us in
    }
    await events.sessionStatsEvent.listen((e) => {
      this.stats = e.payload;
    });
  }
}

export const session = new SessionStore();
