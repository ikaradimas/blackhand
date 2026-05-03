<script lang="ts">
  import { commands } from "$lib/bindings";
  import { unwrap } from "$lib/api";
  import { torrents } from "$lib/stores/torrents.svelte";
  import {
    categories,
    UNCATEGORIZED,
    FINISHED,
    PENDING,
  } from "$lib/stores/categories.svelte";
  import { toasts } from "$lib/stores/toasts.svelte";
  import { ui } from "$lib/stores/ui.svelte";
  import TorrentRow from "$lib/components/TorrentRow.svelte";
  import SkeletonRow from "$lib/components/SkeletonRow.svelte";
  import CategorySidebar from "$lib/components/CategorySidebar.svelte";

  // Refresh category counts whenever the torrent list changes.
  $effect(() => {
    void torrents.list.length;
    categories.refresh();
  });

  const filtered = $derived.by(() => {
    const f = categories.filter;
    if (f === null) return torrents.list;
    if (f === FINISHED) return torrents.list.filter((t) => t.finished);
    if (f === PENDING) return torrents.list.filter((t) => !t.finished);
    if (f === UNCATEGORIZED) return torrents.list.filter((t) => !t.category);
    return torrents.list.filter((t) => t.category === f);
  });

  async function act(action: "pause" | "resume" | "forget", id: number) {
    try {
      await unwrap(commands[action](id));
    } catch (err) {
      toasts.error(`${action} failed: ${err}`);
    }
  }

  function askDelete(id: number) {
    const t = torrents.list.find((x) => x.id === id);
    if (!t) return;
    ui.askRemove({ id, name: t.name ?? t.info_hash });
  }
</script>

<div class="layout">
  <CategorySidebar />

  <div class="content">
    {#if !torrents.loaded}
      <section class="list">
        {#each [0, 1, 2] as i}
          <SkeletonRow delay={i * 150} />
        {/each}
      </section>
    {:else if torrents.list.length === 0}
      <div class="empty">
        <span class="empty-glow"></span>
        <p class="empty-title">// awaiting input</p>
        <p class="empty-prompt">
          paste a magnet, drop a <span class="mono">.torrent</span>, or
          <button class="empty-cta" onclick={() => ui.openAdd()}>+ Add</button>
        </p>
        <p class="empty-hint dim">⌘N to open the add dialog</p>
      </div>
    {:else if filtered.length === 0}
      <div class="empty">
        <p class="empty-title">// no torrents in this category</p>
        <p class="empty-prompt dim">click "All" in the sidebar to clear the filter</p>
      </div>
    {:else}
      <header class="col-headers tnum">
        <span></span>
        <span class="hd-name">NAME</span>
        <span class="hd-num">SIZE</span>
        <span class="hd-num">↓ DOWN</span>
        <span class="hd-num">↑ UP</span>
        <span class="hd-num">PEERS</span>
        <span class="hd-num">ETA</span>
        <span></span>
      </header>

      <section class="list">
        {#each filtered as t (t.id)}
          <TorrentRow
            {t}
            onpause={(id) => act("pause", id)}
            onresume={(id) => act("resume", id)}
            onforget={(id) => act("forget", id)}
            ondelete={askDelete}
          />
        {/each}
      </section>
    {/if}
  </div>
</div>

<style>
  .layout {
    display: flex;
    gap: var(--sp-4);
    align-items: flex-start;
  }
  .content {
    flex: 1;
    min-width: 0;
  }

  /* Must mirror TorrentRow's .grid template (including the 124px actions
   * column) so column boundaries line up. */
  .col-headers {
    display: grid;
    grid-template-columns: 18px minmax(0, 1fr) 80px 110px 110px 60px 80px 124px;
    gap: var(--sp-3);
    /* Match the row's outer padding (sp-4 + 1px border) so contents line up. */
    padding: 0 calc(var(--sp-4) + 1px) var(--sp-2);
    font-size: var(--fs-xs);
    color: var(--fg-2);
    letter-spacing: var(--tracking-wider);
  }
  .hd-name { text-align: left; }
  .hd-num { text-align: right; }

  .list {
    display: flex;
    flex-direction: column;
    gap: var(--sp-3);
  }

  .empty {
    position: relative;
    text-align: center;
    padding: var(--sp-8) 0;
    overflow: hidden;
  }
  .empty-glow {
    position: absolute;
    inset: 0;
    margin: auto;
    width: 280px;
    height: 280px;
    border-radius: 50%;
    background: radial-gradient(
      circle at center,
      rgba(255, 42, 109, 0.06) 0%,
      rgba(255, 42, 109, 0.02) 40%,
      transparent 70%
    );
    pointer-events: none;
    z-index: 0;
  }
  .empty-title {
    position: relative;
    color: var(--accent-cyan);
    font-family: var(--font-mono);
    font-size: var(--fs-md);
    letter-spacing: var(--tracking-wider);
    margin: 0 0 var(--sp-3);
  }
  .empty-prompt {
    position: relative;
    color: var(--fg-1);
    font-size: var(--fs-sm);
    margin: 0;
  }
  .empty-prompt .mono {
    font-family: var(--font-mono);
    color: var(--fg-0);
  }
  .empty-hint {
    position: relative;
    margin-top: var(--sp-3);
  }
  .dim {
    color: var(--fg-2);
    font-size: var(--fs-xs);
  }
  .empty-cta {
    background: transparent;
    border: 1px solid var(--accent-magenta);
    color: var(--accent-magenta);
    border-radius: var(--radius-sm);
    padding: 2px var(--sp-2);
    font-size: var(--fs-xs);
    font-family: inherit;
    cursor: pointer;
    margin: 0 4px;
    transition: background var(--motion-fast), color var(--motion-fast),
      box-shadow var(--motion-fast);
  }
  .empty-cta:hover {
    background: var(--accent-magenta);
    color: var(--fg-0);
    box-shadow: var(--glow-magenta-sm);
  }
</style>
