<script lang="ts">
  type Props = {
    pct: number;          // 0..100
    finished?: boolean;   // adds green rim when complete
    paused?: boolean;     // dims the fill
    cells?: number;       // default 32
  };

  let { pct, finished = false, paused = false, cells = 32 }: Props = $props();

  const filled = $derived(Math.round((Math.max(0, Math.min(100, pct)) / 100) * cells));
</script>

<div class="bar" data-finished={finished} data-paused={paused}>
  {#each Array(cells) as _, i}
    <span class="cell" class:on={i < filled}></span>
  {/each}
</div>

<style>
  .bar {
    display: grid;
    grid-template-columns: repeat(var(--cells, 32), 1fr);
    gap: 2px;
    height: 8px;
    --cells: 32;
  }

  .cell {
    background: var(--bg-3);
    border-radius: 1px;
    transition: background var(--motion-fast);
  }

  .cell.on {
    background: var(--accent-magenta);
    box-shadow: 0 0 4px rgba(255, 42, 109, 0.5);
  }

  .bar[data-finished="true"] .cell.on {
    background: var(--ok);
    box-shadow: 0 0 4px rgba(57, 255, 20, 0.5);
  }

  .bar[data-paused="true"] .cell.on {
    background: var(--fg-2);
    box-shadow: none;
  }
</style>
