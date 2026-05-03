<script lang="ts">
  import StatPill from "$lib/components/StatPill.svelte";
  import { session } from "$lib/stores/session.svelte";
  import { disk } from "$lib/stores/disk.svelte";
  import { torrents } from "$lib/stores/torrents.svelte";
  import { diskLevel, fmtBytes as fmtBytesShared, describeDisk } from "$lib/disk";

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

  const totalCount = $derived(torrents.list.length);

  const diskAccent = $derived.by(() => {
    if (!disk.info) return "neutral" as const;
    const lvl = diskLevel(disk.info.free_bytes, disk.info.total_bytes);
    return lvl === "ok" ? ("neutral" as const) : lvl;
  });
  const diskValue = $derived(disk.info ? fmtBytesShared(disk.info.free_bytes) : "—");
  const diskTitle = $derived(
    disk.info ? `${describeDisk(disk.info)} · ${disk.info.path}` : "disk space unavailable",
  );
</script>

<footer class="bar">
  <div class="left tnum">
    <span class="count">{totalCount} torrent{totalCount === 1 ? "" : "s"}</span>
  </div>
  <div class="right">
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
    <StatPill label="free" value={diskValue} accent={diskAccent} title={diskTitle} />
  </div>
</footer>

<style>
  .bar {
    position: sticky;
    bottom: 0;
    z-index: var(--z-elevated);
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--sp-3);
    padding: var(--sp-2) var(--sp-5);
    background: var(--bg-1);
    border-top: 1px solid var(--bg-3);
  }

  .left {
    display: flex;
    align-items: center;
    gap: var(--sp-3);
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
    color: var(--fg-2);
    letter-spacing: var(--tracking-wider);
    text-transform: uppercase;
  }

  .right {
    display: flex;
    gap: var(--sp-2);
    align-items: center;
  }
</style>
