<script lang="ts">
  import {
    categories,
    UNCATEGORIZED,
    FINISHED,
    PENDING,
    type Filter,
  } from "$lib/stores/categories.svelte";
  import { torrents } from "$lib/stores/torrents.svelte";

  const totalCount = $derived(torrents.list.length);
  const finishedCount = $derived(torrents.list.filter((t) => t.finished).length);
  const pendingCount = $derived(torrents.list.filter((t) => !t.finished).length);
  const uncategorizedCount = $derived(
    torrents.list.filter((t) => !t.category).length,
  );

  function isActive(f: Filter): boolean {
    return categories.filter === f;
  }
</script>

<aside class="sidebar">
  <h3 class="hd">Status</h3>
  <ul class="list">
    <li>
      <button
        type="button"
        class="row"
        class:active={isActive(null)}
        onclick={() => categories.select(null)}
      >
        <span class="name">All</span>
        <span class="count tnum">{totalCount}</span>
      </button>
    </li>
    <li>
      <button
        type="button"
        class="row"
        class:active={isActive(PENDING)}
        onclick={() => categories.select(PENDING)}
      >
        <span class="name">Pending</span>
        <span class="count tnum">{pendingCount}</span>
      </button>
    </li>
    <li>
      <button
        type="button"
        class="row"
        class:active={isActive(FINISHED)}
        onclick={() => categories.select(FINISHED)}
      >
        <span class="name">Finished</span>
        <span class="count tnum">{finishedCount}</span>
      </button>
    </li>
  </ul>

  <h3 class="hd hd-spaced">Categories</h3>
  <ul class="list">
    {#if uncategorizedCount > 0}
      <li>
        <button
          type="button"
          class="row"
          class:active={isActive(UNCATEGORIZED)}
          onclick={() => categories.select(UNCATEGORIZED)}
        >
          <span class="name dim">(uncategorized)</span>
          <span class="count tnum">{uncategorizedCount}</span>
        </button>
      </li>
    {/if}

    {#each categories.list as c (c.name)}
      <li>
        <button
          type="button"
          class="row"
          class:active={isActive(c.name)}
          onclick={() => categories.select(c.name)}
        >
          <span class="name">{c.name}</span>
          <span class="count tnum">{c.count}</span>
        </button>
      </li>
    {/each}

    {#if categories.list.length === 0 && uncategorizedCount === 0}
      <li class="empty-hint">no user categories yet</li>
    {/if}
  </ul>
</aside>

<style>
  .sidebar {
    width: 200px;
    flex-shrink: 0;
    padding-right: var(--sp-3);
    border-right: 1px solid var(--bg-3);
  }

  .hd {
    margin: 0 0 var(--sp-3);
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
    font-weight: 500;
    letter-spacing: var(--tracking-widest);
    text-transform: uppercase;
    color: var(--accent-cyan);
  }
  .hd-spaced {
    margin-top: var(--sp-4);
  }

  .empty-hint {
    padding: 6px var(--sp-2);
    color: var(--fg-2);
    font-style: italic;
    font-size: var(--fs-xs);
  }

  .list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .row {
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    gap: var(--sp-2);
    padding: 6px var(--sp-2);
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--radius-sm);
    color: var(--fg-1);
    font-family: inherit;
    font-size: var(--fs-sm);
    text-align: left;
    cursor: pointer;
    transition: border-color var(--motion-fast), background var(--motion-fast),
      color var(--motion-fast);
  }
  .row:hover {
    border-color: var(--bg-3);
    color: var(--fg-0);
  }
  .row.active {
    border-color: var(--accent-magenta);
    background: rgba(255, 42, 109, 0.06);
    color: var(--fg-0);
    box-shadow: var(--glow-magenta-sm);
  }

  .name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .dim {
    color: var(--fg-2);
    font-style: italic;
  }
  .count {
    color: var(--fg-2);
    font-size: var(--fs-xs);
  }
  .row.active .count {
    color: var(--accent-magenta);
  }
</style>
