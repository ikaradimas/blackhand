<script lang="ts">
  import StatPill from "$lib/components/StatPill.svelte";
  import { session } from "$lib/stores/session.svelte";
  import { torrents } from "$lib/stores/torrents.svelte";
  import { ui } from "$lib/stores/ui.svelte";
  import { commands } from "$lib/bindings";
  import { unwrap } from "$lib/api";

  async function pauseAll() {
    await Promise.allSettled(
      torrents.list.map((t) => unwrap(commands.pause(t.id))),
    );
  }
  async function resumeAll() {
    await Promise.allSettled(
      torrents.list.map((t) => unwrap(commands.resume(t.id))),
    );
  }

  const anyTorrents = $derived(torrents.list.length > 0);
  const anyPaused = $derived(torrents.list.some((t) => t.state === "paused"));
  const anyActive = $derived(torrents.list.some((t) => t.state !== "paused" && !t.finished));

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
</script>

<header class="hdr">
  <div class="brand">
    <span class="bracket">[</span>
    <span class="wordmark">BLACKHAND</span>
    <span class="bracket">]</span>
    <span class="dim">// torrent.client</span>
  </div>

  <div class="right">
    {#if session.stats}
      <div class="stats">
        <StatPill
          label="↓"
          value={fmtBps(session.stats.down_bps)}
          accent={session.stats.down_bps > 0 ? "magenta" : "neutral"}
        />
        <StatPill
          label="↑"
          value={fmtBps(session.stats.up_bps)}
          accent={session.stats.up_bps > 0 ? "cyan" : "neutral"}
        />
        <StatPill label="peers" value={String(session.stats.peers_live)} />
      </div>
    {/if}
    <div class="bulk">
      <button
        type="button"
        title="Resume all"
        aria-label="Resume all"
        onclick={resumeAll}
        disabled={!anyPaused}
      >▶</button>
      <button
        type="button"
        title="Pause all"
        aria-label="Pause all"
        onclick={pauseAll}
        disabled={!anyActive}
      >❚❚</button>
    </div>
    <button class="add" type="button" onclick={() => ui.openAdd()} disabled={false}>+ Add</button>
    <button class="gear" type="button" onclick={() => ui.openSettings()} aria-label="Settings" title="Settings">
      <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
        <path d="M8 5.5a2.5 2.5 0 1 0 0 5 2.5 2.5 0 0 0 0-5Zm0 4a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3Z"/>
        <path d="m13.55 8.94.87-.5a.5.5 0 0 0 .18-.68l-1.5-2.6a.5.5 0 0 0-.68-.18l-.87.5a5.5 5.5 0 0 0-1.41-.82V3.66a.5.5 0 0 0-.5-.5h-3a.5.5 0 0 0-.5.5v1a5.5 5.5 0 0 0-1.41.82l-.87-.5a.5.5 0 0 0-.68.18l-1.5 2.6a.5.5 0 0 0 .18.68l.87.5a5.5 5.5 0 0 0 0 1.64l-.87.5a.5.5 0 0 0-.18.68l1.5 2.6a.5.5 0 0 0 .68.18l.87-.5a5.5 5.5 0 0 0 1.41.82v1a.5.5 0 0 0 .5.5h3a.5.5 0 0 0 .5-.5v-1a5.5 5.5 0 0 0 1.41-.82l.87.5a.5.5 0 0 0 .68-.18l1.5-2.6a.5.5 0 0 0-.18-.68l-.87-.5a5.5 5.5 0 0 0 0-1.64ZM8 12a4 4 0 1 1 0-8 4 4 0 0 1 0 8Z"/>
      </svg>
    </button>
  </div>
</header>

<style>
  .hdr {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--sp-4);
    padding: 0 var(--sp-5);
    height: var(--header-height);
    background: var(--bg-0);
    border-bottom: var(--border-faint);
    position: sticky;
    top: 0;
    z-index: var(--z-elevated);
  }

  .brand {
    display: flex;
    align-items: baseline;
    gap: var(--sp-2);
  }

  .bracket {
    color: var(--accent-cyan);
    font-family: var(--font-mono);
    font-size: var(--fs-xl);
    font-weight: 300;
    text-shadow: var(--glow-cyan-sm);
  }

  .wordmark {
    font-family: var(--font-display);
    font-weight: 700;
    font-size: var(--fs-lg);
    letter-spacing: var(--tracking-widest);
    color: var(--accent-magenta);
    text-shadow: var(--glow-magenta-sm);
  }

  .dim {
    color: var(--fg-2);
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
    letter-spacing: var(--tracking-wider);
    margin-left: var(--sp-2);
  }

  .right {
    display: flex;
    gap: var(--sp-3);
    align-items: center;
  }

  .stats {
    display: flex;
    gap: var(--sp-2);
  }

  .add {
    background: var(--accent-magenta);
    border: 1px solid var(--accent-magenta);
    color: var(--fg-0);
    border-radius: var(--radius-md);
    padding: var(--sp-2) var(--sp-3);
    font-size: var(--fs-sm);
    font-family: inherit;
    cursor: pointer;
    box-shadow: var(--glow-magenta-sm);
    transition: background var(--motion-fast), box-shadow var(--motion-fast);
  }
  .add:hover {
    background: var(--accent-magenta-hover);
    border-color: var(--accent-magenta-hover);
    box-shadow: var(--glow-magenta-md);
  }

  .gear {
    background: transparent;
    border: 1px solid var(--bg-3);
    color: var(--fg-1);
    border-radius: var(--radius-md);
    width: 32px;
    height: 32px;
    padding: 0;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    transition: border-color var(--motion-fast), color var(--motion-fast);
  }
  .gear:hover {
    border-color: var(--accent-cyan);
    color: var(--accent-cyan);
  }

  .bulk {
    display: inline-flex;
    gap: 2px;
  }
  .bulk button {
    background: transparent;
    border: 1px solid var(--bg-3);
    color: var(--fg-1);
    border-radius: var(--radius-md);
    width: 32px;
    height: 32px;
    padding: 0;
    cursor: pointer;
    font-family: inherit;
    font-size: var(--fs-sm);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    transition: border-color var(--motion-fast), color var(--motion-fast);
  }
  .bulk button:hover:not(:disabled) {
    border-color: var(--accent-cyan);
    color: var(--accent-cyan);
  }
  .bulk button:disabled {
    opacity: 0.35;
    cursor: not-allowed;
  }
</style>
