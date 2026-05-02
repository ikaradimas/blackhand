<script lang="ts">
  import { onMount } from "svelte";

  import { commands } from "$lib/bindings";
  import Modal from "$lib/components/Modal.svelte";
  import PixelMark from "$lib/components/PixelMark.svelte";
  import { ui } from "$lib/stores/ui.svelte";

  let version = $state("…");

  onMount(async () => {
    try {
      version = await commands.appVersion();
    } catch {
      version = "?";
    }
  });

  function close() {
    ui.closeAbout();
  }
</script>

<Modal open={ui.aboutModal} title="About" onclose={close}>
  <div class="hero">
    <span class="mark"><PixelMark hash="blackhand-about-deterministic-seed" size={48} /></span>
    <div class="title">
      <h1>BLACKHAND</h1>
      <p class="ver tnum">v{version}</p>
    </div>
  </div>

  <p class="desc">
    A neon-noir BitTorrent client.
    Powered by <span class="code">librqbit</span> and Tauri 2.
  </p>

  <dl class="kv">
    <dt>Engine</dt>
    <dd class="tnum">librqbit 8.1</dd>
    <dt>Shell</dt>
    <dd class="tnum">Tauri 2 + SvelteKit + Rust</dd>
    <dt>License</dt>
    <dd class="tnum">MIT</dd>
  </dl>

  <p class="credit">
    Built with care. Sequential download by default — head pieces arrive first.
  </p>

  <div class="actions">
    <button type="button" class="primary" onclick={close}>Close</button>
  </div>
</Modal>

<style>
  .hero {
    display: flex;
    align-items: center;
    gap: var(--sp-3);
    margin-bottom: var(--sp-3);
  }

  .mark {
    color: var(--accent-magenta);
    display: inline-flex;
    width: 48px;
    height: 48px;
  }

  .title h1 {
    margin: 0;
    font-family: var(--font-display);
    font-size: var(--fs-xl);
    font-weight: 700;
    letter-spacing: var(--tracking-widest);
    color: var(--accent-magenta);
    text-shadow: var(--glow-magenta-sm);
  }

  .ver {
    margin: 4px 0 0;
    color: var(--accent-cyan);
    font-size: var(--fs-xs);
    letter-spacing: var(--tracking-wider);
  }

  .desc {
    color: var(--fg-1);
    font-size: var(--fs-sm);
    line-height: var(--lh-loose);
    margin: 0 0 var(--sp-4);
  }
  .code {
    font-family: var(--font-mono);
    color: var(--fg-0);
    background: var(--bg-1);
    padding: 1px 6px;
    border-radius: var(--radius-sm);
  }

  .kv {
    display: grid;
    grid-template-columns: 100px 1fr;
    gap: var(--sp-2) var(--sp-3);
    margin: 0 0 var(--sp-4);
  }
  .kv dt {
    color: var(--fg-2);
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
    letter-spacing: var(--tracking-wider);
    text-transform: uppercase;
  }
  .kv dd {
    color: var(--fg-1);
    font-size: var(--fs-sm);
    margin: 0;
  }

  .credit {
    color: var(--fg-2);
    font-size: var(--fs-xs);
    font-family: var(--font-mono);
    margin: 0 0 var(--sp-4);
  }

  .actions {
    display: flex;
    justify-content: flex-end;
  }

  button {
    background: var(--bg-2);
    border: 1px solid var(--bg-3);
    color: var(--fg-0);
    border-radius: var(--radius-md);
    padding: var(--sp-2) var(--sp-4);
    font-size: var(--fs-sm);
    font-family: inherit;
    cursor: pointer;
  }
  button.primary {
    background: var(--accent-magenta);
    border-color: var(--accent-magenta);
    color: var(--fg-0);
    box-shadow: var(--glow-magenta-sm);
  }
  button.primary:hover {
    background: var(--accent-magenta-hover);
    border-color: var(--accent-magenta-hover);
  }
</style>
