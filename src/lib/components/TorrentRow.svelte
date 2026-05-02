<script lang="ts">
  import type { TorrentSummary } from "$lib/bindings";
  import ProgressBar from "$lib/components/ProgressBar.svelte";

  type Props = {
    t: TorrentSummary;
    onpause?: (id: number) => void;
    onresume?: (id: number) => void;
    onforget?: (id: number) => void;
    ondelete?: (id: number) => void;
  };

  let { t, onpause, onresume, onforget, ondelete }: Props = $props();

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
  const fmtBps = (bps: number) => `${fmtBytes(bps)}/s`;

  function fmtEta(secs: number | null): string {
    if (secs === null || !Number.isFinite(secs)) return "—";
    if (secs < 60) return `${secs}s`;
    const m = Math.floor(secs / 60);
    const s = secs % 60;
    if (m < 60) return `${m}:${s.toString().padStart(2, "0")}`;
    const h = Math.floor(m / 60);
    const mm = m % 60;
    return `${h}:${mm.toString().padStart(2, "0")}:${(secs % 60).toString().padStart(2, "0")}`;
  }

  const stateDot = $derived(
    t.finished ? "●" :
    t.state === "live" ? "●" :
    t.state === "paused" ? "◯" :
    t.state === "error" ? "✕" :
    "◐", // initializing
  );
</script>

<article
  class="row"
  class:finished={t.finished}
  data-state={t.state}
>
  <div class="grid">
    <span class="dot state-{t.state}" class:done={t.finished}>{stateDot}</span>
    <span class="name" title={t.name ?? t.info_hash}>{t.name ?? t.info_hash}</span>
    <span class="num tnum">{fmtBytes(t.total_bytes)}</span>
    <span class="num tnum dn">{t.state === "paused" ? "—" : fmtBps(t.down_bps)}</span>
    <span class="num tnum up">{t.state === "paused" ? "—" : fmtBps(t.up_bps)}</span>
    <span class="num tnum">{t.peers_live}</span>
    <span class="num tnum eta">{fmtEta(t.eta_secs)}</span>
    <span class="actions">
      {#if t.state === "paused"}
        <button class="action" title="Resume" onclick={() => onresume?.(t.id)}>▶</button>
      {:else}
        <button class="action" title="Pause" onclick={() => onpause?.(t.id)}>❚❚</button>
      {/if}
      <button class="action" title="Remove (keep files)" onclick={() => onforget?.(t.id)}>×</button>
      <button class="action danger" title="Delete + remove files" onclick={() => ondelete?.(t.id)}>⌫</button>
    </span>
  </div>

  <div class="progress">
    <ProgressBar
      pct={t.progress_pct}
      finished={t.finished}
      paused={t.state === "paused"}
    />
    <span class="pct tnum">{t.progress_pct.toFixed(1)}%</span>
  </div>

  {#if t.error}
    <p class="err tnum">{t.error}</p>
  {/if}
</article>

<style>
  .row {
    background: var(--bg-1);
    border: 1px solid var(--bg-3);
    border-radius: var(--radius-lg);
    padding: var(--sp-3) var(--sp-4);
    transition:
      border-color var(--motion-fast),
      box-shadow var(--motion-fast);
    display: flex;
    flex-direction: column;
    gap: var(--sp-2);
  }
  .row:hover {
    border-color: var(--accent-cyan);
    box-shadow: var(--glow-cyan-sm);
  }

  /* Column layout: state · name (flex) · size · down · up · peers · eta · actions */
  .grid {
    display: grid;
    grid-template-columns: 18px minmax(0, 1fr) 80px 110px 110px 60px 80px auto;
    gap: var(--sp-3);
    align-items: baseline;
    font-size: var(--fs-sm);
  }

  .dot {
    color: var(--fg-2);
    font-size: var(--fs-md);
    line-height: 1;
  }
  .dot.state-live { color: var(--accent-cyan); }
  .dot.state-paused { color: var(--fg-2); }
  .dot.state-initializing { color: var(--warn); }
  .dot.state-error { color: var(--err); }
  .dot.done { color: var(--ok); }

  .name {
    color: var(--fg-0);
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .num {
    color: var(--fg-1);
    text-align: right;
  }
  .dn { color: var(--fg-0); }
  .up { color: var(--fg-1); }
  .eta { color: var(--fg-1); }

  .actions {
    display: flex;
    gap: 2px;
  }

  .action {
    background: transparent;
    border: 1px solid var(--bg-3);
    color: var(--fg-1);
    border-radius: var(--radius-sm);
    width: 28px;
    height: 24px;
    padding: 0;
    font-size: var(--fs-xs);
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    transition: border-color var(--motion-fast), color var(--motion-fast);
  }
  .action:hover {
    border-color: var(--accent-cyan);
    color: var(--accent-cyan);
  }
  .action.danger:hover {
    border-color: var(--err);
    color: var(--err);
  }

  .progress {
    display: grid;
    grid-template-columns: 1fr 64px;
    gap: var(--sp-3);
    align-items: center;
  }
  .pct {
    text-align: right;
    color: var(--fg-1);
    font-size: var(--fs-xs);
  }

  .err {
    margin: 0;
    color: var(--err);
    font-size: var(--fs-xs);
    background: rgba(255, 63, 63, 0.08);
    border: 1px solid rgba(255, 63, 63, 0.3);
    padding: var(--sp-2) var(--sp-3);
    border-radius: var(--radius-md);
  }
</style>
