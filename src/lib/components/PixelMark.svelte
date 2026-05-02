<script lang="ts">
  type Props = {
    hash: string;
    /** Pixel size of the SVG bounding box (defaults to 18). */
    size?: number;
  };

  let { hash, size = 18 }: Props = $props();

  // 5x5 grid, vertically mirrored (col 0 = col 4, col 1 = col 3, col 2 center).
  // 15 cells -> 15 bits sampled from a 32-bit hash of info_hash.
  const grid = $derived.by(() => {
    let h = 0;
    for (let i = 0; i < hash.length; i++) {
      h = ((h * 131) ^ hash.charCodeAt(i)) >>> 0;
    }
    const cells: boolean[][] = Array.from({ length: 5 }, () => new Array(5).fill(false));
    for (let col = 0; col < 3; col++) {
      for (let row = 0; row < 5; row++) {
        const on = ((h >>> (col * 5 + row)) & 1) === 1;
        cells[row][col] = on;
        if (col < 2) cells[row][4 - col] = on;
      }
    }
    return cells;
  });
</script>

<svg
  width={size}
  height={size}
  viewBox="0 0 5 5"
  shape-rendering="crispEdges"
  aria-hidden="true"
>
  {#each grid as row, y}
    {#each row as on, x}
      {#if on}
        <rect x={x} y={y} width="1" height="1" fill="currentColor" />
      {/if}
    {/each}
  {/each}
</svg>
