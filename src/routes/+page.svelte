<script lang="ts">
  import { commands } from "$lib/bindings";
  import { unwrap } from "$lib/api";
  import { torrents } from "$lib/stores/torrents.svelte";
  import TorrentRow from "$lib/components/TorrentRow.svelte";

  let magnet = $state("");
  let busy = $state(false);
  let lastError = $state<string | null>(null);

  async function addMagnet(e: Event) {
    e.preventDefault();
    if (!magnet.trim()) return;
    busy = true;
    lastError = null;
    try {
      await unwrap(commands.addMagnet(magnet.trim()));
      magnet = "";
    } catch (err) {
      lastError = String(err);
    } finally {
      busy = false;
    }
  }

  async function act(action: "pause" | "resume" | "forget" | "delete", id: number) {
    lastError = null;
    try {
      await unwrap(commands[action](id));
    } catch (err) {
      lastError = String(err);
    }
  }
</script>

<form class="add-row" onsubmit={addMagnet}>
  <input
    type="text"
    placeholder="paste a magnet: link"
    bind:value={magnet}
    disabled={busy}
  />
  <button type="submit" class="primary" disabled={busy || !magnet.trim()}>
    {busy ? "…" : "+ Add"}
  </button>
</form>

{#if lastError}
  <p class="err tnum">{lastError}</p>
{/if}

{#if torrents.list.length === 0}
  <div class="empty">
    <p>no torrents yet</p>
    <p class="dim">paste a magnet link above</p>
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
        ondelete={(id) => act("delete", id)}
      />
    {/each}
  </section>
{/if}

<style>
  .add-row {
    display: flex;
    gap: var(--sp-2);
    margin-bottom: var(--sp-5);
  }

  input[type="text"] {
    flex: 1;
    padding: var(--sp-2) var(--sp-3);
    background: var(--bg-1);
    border: 1px solid var(--bg-3);
    color: var(--fg-0);
    border-radius: var(--radius-md);
    font-family: var(--font-mono);
    font-size: var(--fs-sm);
    transition: border-color var(--motion-fast), box-shadow var(--motion-fast);
  }
  input[type="text"]:focus {
    outline: none;
    border-color: var(--accent-cyan);
    box-shadow: 0 0 0 1px var(--accent-cyan), var(--glow-cyan-sm);
  }
  input[type="text"]::placeholder {
    color: var(--fg-2);
  }

  button {
    background: var(--bg-2);
    color: var(--fg-0);
    border: 1px solid var(--bg-3);
    border-radius: var(--radius-md);
    padding: var(--sp-2) var(--sp-4);
    font-size: var(--fs-sm);
    cursor: pointer;
    transition: border-color var(--motion-fast), color var(--motion-fast);
  }
  button:hover:not(:disabled) {
    border-color: var(--accent-cyan);
    color: var(--accent-cyan);
  }
  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  button.primary {
    background: var(--accent-magenta);
    border-color: var(--accent-magenta);
    color: var(--fg-0);
    box-shadow: var(--glow-magenta-sm);
  }
  button.primary:hover:not(:disabled) {
    background: var(--accent-magenta-hover);
    border-color: var(--accent-magenta-hover);
    box-shadow: var(--glow-magenta-md);
  }

  .err {
    color: var(--err);
    font-size: var(--fs-xs);
    background: rgba(255, 63, 63, 0.08);
    border: 1px solid rgba(255, 63, 63, 0.3);
    padding: var(--sp-2) var(--sp-3);
    border-radius: var(--radius-md);
    margin-bottom: var(--sp-3);
  }

  .col-headers {
    display: grid;
    grid-template-columns: 18px minmax(0, 1fr) 80px 110px 110px 60px 80px auto;
    gap: var(--sp-3);
    padding: 0 var(--sp-4) var(--sp-2);
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
    color: var(--fg-1);
  }
  .empty .dim {
    color: var(--fg-2);
    font-size: var(--fs-xs);
    margin-top: var(--sp-2);
  }
</style>
