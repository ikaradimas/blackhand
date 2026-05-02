<script lang="ts">
  import { onDestroy, onMount } from "svelte";

  import { commands, events, type SessionStats, type TorrentSummary } from "$lib/bindings";
  import { unwrap } from "$lib/api";

  let torrents = $state<TorrentSummary[]>([]);
  let session = $state<SessionStats | null>(null);
  let magnet = $state("");
  let busy = $state(false);
  let lastError = $state<string | null>(null);
  let unlistens: Array<() => void> = [];

  onMount(async () => {
    unlistens.push(
      await events.torrentsSnapshotEvent.listen((e) => {
        torrents = e.payload.torrents;
      }),
    );
    unlistens.push(
      await events.sessionStatsEvent.listen((e) => {
        session = e.payload;
      }),
    );

    try {
      const initial = await unwrap(commands.listTorrents());
      torrents = initial.torrents;
      session = await unwrap(commands.sessionStats());
    } catch (e) {
      lastError = String(e);
    }
  });

  onDestroy(() => {
    for (const u of unlistens) u();
  });

  async function addMagnet(e: Event) {
    e.preventDefault();
    if (!magnet.trim()) return;
    busy = true;
    lastError = null;
    try {
      await unwrap(commands.addMagnet(magnet.trim()));
      magnet = "";
    } catch (err) {
      lastError = String(err);
    } finally {
      busy = false;
    }
  }

  async function act(action: "pause" | "resume" | "forget" | "delete", id: number) {
    lastError = null;
    try {
      await unwrap(commands[action](id));
    } catch (err) {
      lastError = String(err);
    }
  }

  function fmtBytes(n: number): string {
    if (!Number.isFinite(n)) return "—";
    const u = ["B", "KB", "MB", "GB", "TB"];
    let i = 0;
    let v = n;
    while (v >= 1024 && i < u.length - 1) {
      v /= 1024;
      i++;
    }
    return `${v.toFixed(v < 10 ? 2 : 1)} ${u[i]}`;
  }

  function fmtBps(bps: number): string {
    return `${fmtBytes(bps)}/s`;
  }

  function fmtEta(secs: number | null): string {
    if (secs === null || !Number.isFinite(secs)) return "—";
    if (secs < 60) return `${secs}s`;
    const m = Math.floor(secs / 60);
    const s = secs % 60;
    if (m < 60) return `${m}m ${s.toString().padStart(2, "0")}s`;
    const h = Math.floor(m / 60);
    const mm = m % 60;
    return `${h}h ${mm.toString().padStart(2, "0")}m`;
  }
</script>

<main class="page">
  <header>
    <div class="title">
      <h1>BLACKHAND <span class="dim">// torrent.client</span></h1>
      {#if session}
        <div class="globals">
          <span><span class="dim">↓</span> {fmtBps(session.down_bps)}</span>
          <span><span class="dim">↑</span> {fmtBps(session.up_bps)}</span>
          <span><span class="dim">peers</span> {session.peers_live}</span>
        </div>
      {/if}
    </div>
    <p class="subtle">Phase 2.3 — typed bindings via tauri-specta</p>
  </header>

  <form class="add-row" onsubmit={addMagnet}>
    <input
      type="text"
      placeholder="paste a magnet: link"
      bind:value={magnet}
      disabled={busy}
    />
    <button type="submit" disabled={busy || !magnet.trim()}>
      {busy ? "…" : "Add"}
    </button>
  </form>

  {#if lastError}
    <p class="err">{lastError}</p>
  {/if}

  <section class="list">
    {#if torrents.length === 0}
      <p class="subtle empty">no torrents yet</p>
    {:else}
      {#each torrents as t (t.id)}
        <article class="row">
          <div class="row-head">
            <span class="name">{t.name ?? t.info_hash}</span>
            <span class="state state-{t.state}">{t.state}</span>
          </div>
          <div class="bar">
            <div class="fill" style:width="{Math.min(100, t.progress_pct)}%"></div>
          </div>
          <div class="row-foot">
            <span>{t.progress_pct.toFixed(1)}%</span>
            <span>{fmtBytes(t.progress_bytes)} / {fmtBytes(t.total_bytes)}</span>
            <span>↓ {fmtBps(t.down_bps)}</span>
            <span>↑ {fmtBps(t.up_bps)}</span>
            <span>peers {t.peers_live}</span>
            <span>eta {fmtEta(t.eta_secs)}</span>
            <span class="actions">
              <button onclick={() => act("pause", t.id)}>pause</button>
              <button onclick={() => act("resume", t.id)}>resume</button>
              <button onclick={() => act("forget", t.id)}>forget</button>
              <button class="danger" onclick={() => act("delete", t.id)}>delete</button>
            </span>
          </div>
          {#if t.error}
            <p class="err">{t.error}</p>
          {/if}
        </article>
      {/each}
    {/if}
  </section>
</main>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    height: 100%;
    background: #07070C;
    color: #ECECF5;
    font-family: "Inter", ui-sans-serif, system-ui, -apple-system, sans-serif;
    font-feature-settings: "tnum", "ss01";
  }

  .page {
    max-width: 1100px;
    margin: 0 auto;
    padding: 2rem 1.5rem 4rem;
  }

  .title {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    gap: 1rem;
    flex-wrap: wrap;
  }

  header h1 {
    margin: 0 0 0.25rem;
    font-size: 1.4rem;
    letter-spacing: 0.18em;
    color: #FF2A6D;
    font-weight: 700;
  }

  .globals {
    display: flex;
    gap: 1.25rem;
    font-family: ui-monospace, "JetBrains Mono", monospace;
    font-size: 0.85rem;
    color: #ECECF5;
  }

  .dim {
    color: #6E6E8C;
    font-weight: 400;
    letter-spacing: 0.1em;
  }

  .subtle {
    color: #6E6E8C;
    font-size: 0.8rem;
    margin: 0 0 1.5rem;
    letter-spacing: 0.05em;
  }

  .add-row {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.25rem;
  }

  input[type="text"] {
    flex: 1;
    padding: 0.6rem 0.8rem;
    background: #0E0E16;
    border: 1px solid #232336;
    color: #ECECF5;
    border-radius: 4px;
    font-family: ui-monospace, "JetBrains Mono", monospace;
    font-size: 0.85rem;
  }
  input[type="text"]:focus {
    outline: none;
    border-color: #08F7FE;
    box-shadow: 0 0 0 1px #08F7FE;
  }

  button {
    background: #181826;
    color: #ECECF5;
    border: 1px solid #232336;
    border-radius: 4px;
    padding: 0.45rem 0.9rem;
    font-size: 0.8rem;
    font-family: inherit;
    cursor: pointer;
  }
  button:hover:not(:disabled) {
    border-color: #08F7FE;
    color: #08F7FE;
  }
  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  button.danger:hover:not(:disabled) {
    border-color: #FF3F3F;
    color: #FF3F3F;
  }

  form button[type="submit"] {
    background: #FF2A6D;
    border-color: #FF2A6D;
    color: #ECECF5;
  }
  form button[type="submit"]:hover:not(:disabled) {
    background: #FF3D7F;
    border-color: #FF3D7F;
    color: #ECECF5;
  }

  .err {
    color: #FF3F3F;
    font-family: ui-monospace, monospace;
    font-size: 0.8rem;
    background: rgba(255, 63, 63, 0.08);
    border: 1px solid rgba(255, 63, 63, 0.3);
    padding: 0.5rem 0.75rem;
    border-radius: 4px;
    margin-bottom: 1rem;
  }

  .list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .empty {
    text-align: center;
    padding: 3rem 0;
  }

  .row {
    background: #0E0E16;
    border: 1px solid #1A1A26;
    border-radius: 6px;
    padding: 0.85rem 1rem;
  }

  .row-head {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    gap: 1rem;
    margin-bottom: 0.5rem;
  }
  .name {
    font-size: 0.95rem;
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .state {
    font-family: ui-monospace, monospace;
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.1em;
  }
  .state-live { color: #08F7FE; }
  .state-paused { color: #6E6E8C; }
  .state-initializing { color: #FFB23F; }
  .state-error { color: #FF3F3F; }

  .bar {
    height: 6px;
    background: #1A1A26;
    border-radius: 3px;
    overflow: hidden;
    margin-bottom: 0.5rem;
  }
  .fill {
    height: 100%;
    background: linear-gradient(90deg, #FF2A6D, #B14EFF);
    transition: width 0.3s ease;
  }

  .row-foot {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem 1rem;
    align-items: center;
    font-family: ui-monospace, monospace;
    font-size: 0.78rem;
    color: #B4B4D0;
  }

  .actions {
    margin-left: auto;
    display: flex;
    gap: 0.35rem;
  }

  .actions button {
    padding: 0.2rem 0.55rem;
    font-size: 0.7rem;
  }
</style>
