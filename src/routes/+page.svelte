<script lang="ts">
  import { commands } from "$lib/bindings";
  import { unwrap } from "$lib/api";
  import { torrents } from "$lib/stores/torrents.svelte";
  import { ui } from "$lib/stores/ui.svelte";
  import TorrentRow from "$lib/components/TorrentRow.svelte";

  let lastError = $state<string | null>(null);

  async function act(action: "pause" | "resume" | "forget", id: number) {
    lastError = null;
    try {
      await unwrap(commands[action](id));
    } catch (err) {
      lastError = String(err);
    }
  }

  function askDelete(id: number) {
    const t = torrents.list.find((x) => x.id === id);
    if (!t) return;
    ui.askRemove({ id, name: t.name ?? t.info_hash });
  }
</script>

{#if lastError}
  <p class="err tnum">{lastError}</p>
{/if}

{#if torrents.list.length === 0}
  <div class="empty">
    <p class="empty-title">no torrents yet</p>
    <p class="dim">click <button class="empty-cta" onclick={() => ui.openAdd()}>+ Add</button> in the header, or drop a .torrent file</p>
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
    {#each torrents.list as t (t.id)}
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

<style>
  .err {
    color: var(--err);
    font-size: var(--fs-xs);
    background: rgba(255, 63, 63, 0.08);
    border: 1px solid rgba(255, 63, 63, 0.3);
    padding: var(--sp-2) var(--sp-3);
    border-radius: var(--radius-md);
    margin-bottom: var(--sp-3);
  }

  /* Must mirror TorrentRow's .grid template (including the 96px actions
   * column) so column boundaries line up. */
  .col-headers {
    display: grid;
    grid-template-columns: 18px minmax(0, 1fr) 80px 110px 110px 60px 80px 96px;
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
    text-align: center;
    padding: var(--sp-8) 0;
  }
  .empty-title {
    color: var(--fg-1);
    font-family: var(--font-mono);
    text-transform: uppercase;
    letter-spacing: var(--tracking-wider);
    margin: 0 0 var(--sp-3);
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
  }
  .empty-cta:hover {
    background: var(--accent-magenta);
    color: var(--fg-0);
  }
</style>
