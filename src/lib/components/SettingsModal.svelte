<script lang="ts">
  import { commands, type AppSettings } from "$lib/bindings";
  import { unwrap } from "$lib/api";
  import Modal from "$lib/components/Modal.svelte";
  import { toasts } from "$lib/stores/toasts.svelte";
  import { ui } from "$lib/stores/ui.svelte";

  let settings = $state<AppSettings | null>(null);
  /** Snapshot at load/last-save, used to detect which fields changed. */
  let baseline = $state<AppSettings | null>(null);
  let busy = $state(false);
  let saved = $state(false);
  let lastSaveNeedsRestart = $state(false);

  // Lazy-load on first open; refresh whenever opened so external file edits are reflected.
  $effect(() => {
    if (ui.settingsModal) {
      saved = false;
      load();
    }
  });

  async function load() {
    try {
      const s = await unwrap(commands.getSettings());
      settings = s;
      baseline = structuredClone(s);
    } catch (e) {
      toasts.error(`couldn't load settings: ${e}`);
    }
  }

  function nonBandwidthChanged(a: AppSettings, b: AppSettings): boolean {
    return (
      a.download_dir !== b.download_dir ||
      a.listen_port_min !== b.listen_port_min ||
      a.listen_port_max !== b.listen_port_max ||
      a.enable_upnp !== b.enable_upnp ||
      a.enable_dht !== b.enable_dht
    );
  }

  async function save() {
    if (!settings || !baseline) return;
    busy = true;
    try {
      const needsRestart = nonBandwidthChanged(settings, baseline);
      await unwrap(commands.saveSettings(settings));
      lastSaveNeedsRestart = needsRestart;
      baseline = structuredClone(settings);
      saved = true;
    } catch (e) {
      toasts.error(`save failed: ${e}`);
    } finally {
      busy = false;
    }
  }

  async function restart() {
    try {
      await commands.restartApp();
    } catch (e) {
      toasts.error(`restart failed: ${e}`);
    }
  }

  function close() {
    ui.closeSettings();
  }
</script>

<Modal open={ui.settingsModal} title="Settings" onclose={close}>
  {#if !settings}
    <p class="dim">loading…</p>
  {:else}
    <form onsubmit={(e) => { e.preventDefault(); save(); }}>
      <section class="group">
        <h3>Storage</h3>
        <label class="field">
          <span class="lbl">Download directory</span>
          <input
            type="text"
            placeholder="(default: ~/Downloads/BlackHand)"
            bind:value={settings.download_dir}
            disabled={busy}
          />
          <span class="hint">leave blank to use the OS default</span>
        </label>
      </section>

      <section class="group">
        <h3>Network</h3>
        <div class="row-fields">
          <label class="field">
            <span class="lbl">Listen port — min</span>
            <input
              type="number"
              min="0"
              max="65535"
              bind:value={settings.listen_port_min}
              disabled={busy}
            />
          </label>
          <label class="field">
            <span class="lbl">max</span>
            <input
              type="number"
              min="0"
              max="65535"
              bind:value={settings.listen_port_max}
              disabled={busy}
            />
          </label>
        </div>
        <span class="hint">0/0 = librqbit auto-selects</span>

        <label class="check">
          <input type="checkbox" bind:checked={settings.enable_upnp} disabled={busy} />
          <span>UPnP port forwarding</span>
        </label>
        <label class="check">
          <input type="checkbox" bind:checked={settings.enable_dht} disabled={busy} />
          <span>DHT (distributed hash table)</span>
        </label>
      </section>

      <section class="group">
        <h3>Bandwidth</h3>
        <div class="row-fields">
          <label class="field">
            <span class="lbl">Download limit (KB/s)</span>
            <input
              type="number"
              min="0"
              bind:value={settings.download_limit_kbps}
              disabled={busy}
            />
          </label>
          <label class="field">
            <span class="lbl">Upload limit (KB/s)</span>
            <input
              type="number"
              min="0"
              bind:value={settings.upload_limit_kbps}
              disabled={busy}
            />
          </label>
        </div>
        <span class="hint">0 = unlimited</span>
      </section>

      {#if saved && lastSaveNeedsRestart}
        <div class="restart-banner">
          <span>Saved. Network &amp; storage changes apply on next launch.</span>
          <button type="button" class="restart" onclick={restart}>Restart now</button>
        </div>
      {:else if saved}
        <div class="saved-banner">
          <span>Saved. Bandwidth limits applied live.</span>
        </div>
      {/if}

      <div class="actions">
        <button type="button" onclick={close}>Close</button>
        <button type="submit" class="primary" disabled={busy}>
          {busy ? "saving…" : "Save"}
        </button>
      </div>
    </form>
  {/if}
</Modal>

<style>
  form {
    display: flex;
    flex-direction: column;
    gap: var(--sp-4);
  }

  .group {
    display: flex;
    flex-direction: column;
    gap: var(--sp-2);
  }

  h3 {
    margin: 0 0 var(--sp-1);
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
    font-weight: 500;
    letter-spacing: var(--tracking-wider);
    text-transform: uppercase;
    color: var(--accent-cyan);
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: var(--sp-1);
  }

  .row-fields {
    display: flex;
    gap: var(--sp-3);
    align-items: end;
  }

  .lbl {
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
    color: var(--fg-2);
    letter-spacing: var(--tracking-wider);
    text-transform: uppercase;
  }

  .hint {
    font-size: var(--fs-xs);
    color: var(--fg-2);
  }

  input[type="text"],
  input[type="number"] {
    padding: var(--sp-2) var(--sp-3);
    background: var(--bg-1);
    border: 1px solid var(--bg-3);
    color: var(--fg-0);
    border-radius: var(--radius-md);
    font-family: var(--font-mono);
    font-size: var(--fs-sm);
    transition: border-color var(--motion-fast), box-shadow var(--motion-fast);
  }
  input[type="text"]:focus,
  input[type="number"]:focus {
    outline: none;
    border-color: var(--accent-cyan);
    box-shadow: 0 0 0 1px var(--accent-cyan), var(--glow-cyan-sm);
  }

  input[type="number"] {
    width: 110px;
  }

  .check {
    display: flex;
    align-items: center;
    gap: var(--sp-2);
    color: var(--fg-1);
    font-size: var(--fs-sm);
    cursor: pointer;
  }
  .check input[type="checkbox"] {
    accent-color: var(--accent-magenta);
    cursor: pointer;
  }

  .restart-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--sp-3);
    padding: var(--sp-2) var(--sp-3);
    background: rgba(8, 247, 254, 0.06);
    border: 1px solid var(--accent-cyan);
    border-radius: var(--radius-md);
    color: var(--fg-1);
    font-size: var(--fs-xs);
  }

  .saved-banner {
    padding: var(--sp-2) var(--sp-3);
    background: rgba(57, 255, 20, 0.06);
    border: 1px solid var(--ok);
    border-radius: var(--radius-md);
    color: var(--fg-1);
    font-size: var(--fs-xs);
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--sp-2);
    margin-top: var(--sp-2);
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

  button.restart {
    padding: 4px var(--sp-3);
    font-size: var(--fs-xs);
    background: var(--accent-cyan);
    border-color: var(--accent-cyan);
    color: var(--bg-0);
  }
  button.restart:hover {
    background: var(--accent-cyan-hover);
    border-color: var(--accent-cyan-hover);
    color: var(--bg-0);
  }

  .dim {
    color: var(--fg-2);
    font-size: var(--fs-sm);
    text-align: center;
    padding: var(--sp-5);
  }
</style>
