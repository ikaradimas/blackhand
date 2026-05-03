<script lang="ts">
  import { onMount } from "svelte";
  import { emit } from "@tauri-apps/api/event";

  import type { TorrentSummary } from "$lib/bindings";
  import { session } from "$lib/stores/session.svelte";
  import { torrents } from "$lib/stores/torrents.svelte";
  import { disk } from "$lib/stores/disk.svelte";
  import { diskLevel, fmtBytes as fmtBytesShared } from "$lib/disk";

  // Active = currently downloading (live + not finished). Most useful at a glance.
  let active = $derived(
    torrents.list
      .filter((t) => t.state === "live" && !t.finished)
      .sort((a, b) => b.down_bps - a.down_bps)
      .slice(0, 6),
  );

  let stats = $derived(session.stats);
  let diskInfo = $derived(disk.info);
  let diskAccent = $derived(
    diskInfo ? diskLevel(diskInfo.free_bytes, diskInfo.total_bytes) : "ok",
  );

  function fmtBps(bps: number): string {
    if (bps <= 0) return "0 B/s";
    const u = ["B/s", "KB/s", "MB/s", "GB/s"];
    let i = 0;
    let v = bps;
    while (v >= 1024 && i < u.length - 1) {
      v /= 1024;
      i++;
    }
    return `${v.toFixed(v < 10 ? 1 : 0)} ${u[i]}`;
  }

  function shortName(t: TorrentSummary): string {
    return t.name ?? t.info_hash.slice(0, 8);
  }

  function reportHover(hovered: boolean) {
    void emit("tray-popup-hover", { hovered });
  }

  onMount(() => {
    // Tell the backend the popup is alive so the hide-on-leave debounce can
    // be cancelled when the user moves the cursor into us.
    document.body.addEventListener("mouseenter", () => reportHover(true));
    document.body.addEventListener("mouseleave", () => reportHover(false));
    // Refresh disk info each time the popup mounts (on first hover).
    void disk.refresh();
  });
</script>

<div class="popup">
  <header class="hd">
    <span class="brand tnum">BLACKHAND</span>
    <span class="totals tnum">
      <span class="dl">↓ {fmtBps(stats?.down_bps ?? 0)}</span>
      <span class="ul">↑ {fmtBps(stats?.up_bps ?? 0)}</span>
    </span>
  </header>

  {#if active.length === 0}
    <p class="empty">no active downloads</p>
  {:else}
    <ul class="rows">
      {#each active as t (t.id)}
        <li class="row">
          <div class="line1">
            <span class="name">{shortName(t)}</span>
            <span class="speed tnum">{fmtBps(t.down_bps)}</span>
          </div>
          <div class="bar" style="--p: {Math.max(0, Math.min(100, t.progress_pct))}%">
            <div class="fill"></div>
          </div>
        </li>
      {/each}
    </ul>
    {#if torrents.list.filter((t) => t.state === "live" && !t.finished).length > active.length}
      <p class="more tnum">
        +{torrents.list.filter((t) => t.state === "live" && !t.finished).length - active.length} more
      </p>
    {/if}
  {/if}

  {#if diskInfo}
    <footer class="disk tnum" data-level={diskAccent} title={diskInfo.path}>
      free: {fmtBytesShared(diskInfo.free_bytes)} / {fmtBytesShared(diskInfo.total_bytes)}
    </footer>
  {/if}
</div>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    background: var(--bg-0);
    color: var(--fg-0);
    font-family: var(--font-sans);
    overflow: hidden;
  }

  .popup {
    display: flex;
    flex-direction: column;
    height: 100vh;
    border: 1px solid var(--accent-magenta);
    box-shadow: var(--glow-magenta-sm);
    background: var(--bg-0);
    padding: var(--sp-3);
    gap: var(--sp-3);
    box-sizing: border-box;
  }

  .hd {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--bg-3);
    padding-bottom: var(--sp-2);
  }

  .brand {
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
    letter-spacing: var(--tracking-widest);
    color: var(--accent-magenta);
    font-weight: 500;
  }

  .totals {
    display: flex;
    gap: var(--sp-3);
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
  }
  .totals .dl {
    color: var(--accent-cyan);
  }
  .totals .ul {
    color: var(--fg-2);
  }

  .empty {
    margin: auto;
    color: var(--fg-2);
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
    letter-spacing: var(--tracking-wider);
    text-transform: uppercase;
  }

  .rows {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: var(--sp-2);
    overflow: hidden;
  }

  .row {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .line1 {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    gap: var(--sp-2);
    font-size: var(--fs-xs);
  }
  .name {
    color: var(--fg-0);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .speed {
    color: var(--accent-cyan);
    font-family: var(--font-mono);
    flex-shrink: 0;
  }

  .bar {
    height: 4px;
    background: var(--bg-2);
    border-radius: 2px;
    overflow: hidden;
  }
  .fill {
    height: 100%;
    width: var(--p);
    background: var(--accent-magenta);
    box-shadow: 0 0 4px rgba(255, 42, 109, 0.5);
    transition: width var(--motion-fast);
  }

  .more {
    margin: 0;
    text-align: center;
    color: var(--fg-2);
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
  }

  .disk {
    margin-top: auto;
    padding-top: var(--sp-2);
    border-top: 1px solid var(--bg-3);
    font-size: var(--fs-xs);
    color: var(--fg-2);
    text-align: right;
  }
  .disk[data-level="warn"] {
    color: var(--warn);
    border-top-color: rgba(255, 172, 28, 0.35);
  }
  .disk[data-level="err"] {
    color: var(--err);
    border-top-color: var(--err-border);
  }
</style>
