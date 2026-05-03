import { commands, events, type SessionStats } from "$lib/bindings";
import { unwrap } from "$lib/api";

class SessionStore {
  stats = $state<SessionStats | null>(null);
  #started = false;

  async refresh() {
    try {
      this.stats = await unwrap(commands.sessionStats());
    } catch {
      // tolerable
    }
  }

  async start() {
    if (this.#started) return;
    this.#started = true;
    await this.refresh();
    await events.sessionStatsEvent.listen((e) => {
      this.stats = e.payload;
    });
  }
}

export const session = new SessionStore();
