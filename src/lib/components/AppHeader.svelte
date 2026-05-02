<script lang="ts">
  import StatPill from "$lib/components/StatPill.svelte";
  import { session } from "$lib/stores/session.svelte";

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

  <div class="stats">
    {#if session.stats}
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
    {/if}
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

  .stats {
    display: flex;
    gap: var(--sp-2);
  }
</style>
