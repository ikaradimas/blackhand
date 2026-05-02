<script lang="ts">
  import { commands } from "$lib/bindings";
  import { unwrap } from "$lib/api";
  import Modal from "$lib/components/Modal.svelte";
  import { toasts } from "$lib/stores/toasts.svelte";
  import { ui } from "$lib/stores/ui.svelte";

  let magnet = $state("");
  let pickedFile = $state<{ name: string; size: number; bytes: number[] } | null>(null);
  let dragOver = $state(false);
  let busy = $state(false);
  let error = $state<string | null>(null);
  let magnetInput = $state<HTMLInputElement>();

  $effect(() => {
    if (ui.addModal && magnetInput) {
      queueMicrotask(() => magnetInput?.focus());
    }
  });

  function reset() {
    magnet = "";
    pickedFile = null;
    busy = false;
    error = null;
  }

  function close() {
    reset();
    ui.closeAdd();
  }

  async function ingestFile(file: File) {
    if (!file.name.toLowerCase().endsWith(".torrent")) {
      error = `expected .torrent file, got "${file.name}"`;
      return;
    }
    const bytes = Array.from(new Uint8Array(await file.arrayBuffer()));
    pickedFile = { name: file.name, size: file.size, bytes };
    magnet = "";
    error = null;
  }

  async function onPick(e: Event) {
    const input = e.target as HTMLInputElement;
    const file = input.files?.[0];
    if (file) await ingestFile(file);
    input.value = ""; // allow re-picking same file
  }

  async function onDrop(e: DragEvent) {
    e.preventDefault();
    dragOver = false;
    const file = e.dataTransfer?.files?.[0];
    if (file) await ingestFile(file);
  }

  function fmtBytes(n: number): string {
    const u = ["B", "KB", "MB", "GB"];
    let i = 0;
    let v = n;
    while (v >= 1024 && i < u.length - 1) {
      v /= 1024;
      i++;
    }
    return `${v.toFixed(v < 10 ? 2 : 1)} ${u[i]}`;
  }

  const canAdd = $derived(magnet.trim().length > 0 || pickedFile !== null);

  async function add(e?: Event) {
    e?.preventDefault();
    if (!canAdd) return;
    busy = true;
    error = null;
    try {
      if (magnet.trim()) {
        const r = await unwrap(commands.addMagnet(magnet.trim()));
        toasts.ok(`added: ${r.name ?? r.info_hash}`);
      } else if (pickedFile) {
        const r = await unwrap(commands.addTorrentFile(pickedFile.bytes));
        toasts.ok(`added: ${r.name ?? r.info_hash}`);
      }
      close();
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
</script>

<Modal open={ui.addModal} title="Add Torrent" onclose={close}>
  <form onsubmit={add}>
    <label class="field">
      <span class="label">Magnet link</span>
      <input
        type="text"
        placeholder="magnet:?xt=urn:btih:…"
        bind:this={magnetInput}
        bind:value={magnet}
        oninput={() => {
          pickedFile = null;
          error = null;
        }}
        disabled={busy}
      />
    </label>

    <div class="divider"><span>or</span></div>

    <label
      class="dropzone"
      class:over={dragOver}
      ondragover={(e) => {
        e.preventDefault();
        dragOver = true;
      }}
      ondragleave={() => (dragOver = false)}
      ondrop={onDrop}
    >
      {#if pickedFile}
        <div class="picked">
          <span class="picked-name">{pickedFile.name}</span>
          <span class="tnum picked-size">{fmtBytes(pickedFile.size)}</span>
          <button
            type="button"
            class="picked-clear"
            onclick={(e) => {
              e.preventDefault();
              pickedFile = null;
            }}>×</button>
        </div>
      {:else}
        <span class="drop-text">
          drop a <span class="mono">.torrent</span> file here, or
          <span class="link">click to browse</span>
        </span>
      {/if}
      <input
        type="file"
        accept=".torrent"
        onchange={onPick}
        disabled={busy}
      />
    </label>

    {#if error}
      <p class="err tnum">{error}</p>
    {/if}

    <div class="actions">
      <button type="button" onclick={close}>Cancel</button>
      <button
        type="submit"
        class="primary"
        disabled={busy || !canAdd}
      >
        {busy ? "adding…" : "Add"}
      </button>
    </div>
  </form>
</Modal>

<style>
  form {
    display: flex;
    flex-direction: column;
    gap: var(--sp-3);
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: var(--sp-2);
  }

  .label {
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
    color: var(--fg-2);
    letter-spacing: var(--tracking-wider);
    text-transform: uppercase;
  }

  input[type="text"] {
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

  .divider {
    display: flex;
    align-items: center;
    color: var(--fg-2);
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
    text-transform: uppercase;
    letter-spacing: var(--tracking-wider);
  }
  .divider::before,
  .divider::after {
    content: "";
    flex: 1;
    height: 1px;
    background: var(--bg-3);
  }
  .divider span {
    padding: 0 var(--sp-3);
  }

  .dropzone {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--sp-5) var(--sp-4);
    border: 1px dashed var(--bg-3);
    border-radius: var(--radius-md);
    background: var(--bg-1);
    color: var(--fg-1);
    font-size: var(--fs-sm);
    cursor: pointer;
    transition: border-color var(--motion-fast), background var(--motion-fast);
  }
  .dropzone.over {
    border-color: var(--accent-cyan);
    background: rgba(8, 247, 254, 0.05);
  }
  .dropzone:hover {
    border-color: var(--accent-cyan);
  }

  .dropzone input[type="file"] {
    position: absolute;
    inset: 0;
    opacity: 0;
    cursor: pointer;
  }

  .drop-text {
    color: var(--fg-1);
  }
  .mono {
    font-family: var(--font-mono);
    color: var(--fg-0);
  }
  .link {
    color: var(--accent-cyan);
    text-decoration: underline;
    text-underline-offset: 3px;
  }

  .picked {
    display: flex;
    align-items: center;
    gap: var(--sp-3);
    width: 100%;
    z-index: 1;
  }
  .picked-name {
    flex: 1;
    color: var(--fg-0);
    font-family: var(--font-mono);
    font-size: var(--fs-sm);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .picked-size {
    color: var(--fg-2);
    font-size: var(--fs-xs);
  }
  .picked-clear {
    background: transparent;
    border: 1px solid var(--bg-3);
    color: var(--fg-2);
    width: 24px;
    height: 24px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    line-height: 1;
    z-index: 2;
    position: relative;
  }
  .picked-clear:hover {
    border-color: var(--err);
    color: var(--err);
  }

  .err {
    color: var(--err);
    background: rgba(255, 63, 63, 0.08);
    border: 1px solid rgba(255, 63, 63, 0.3);
    padding: var(--sp-2) var(--sp-3);
    border-radius: var(--radius-md);
    font-size: var(--fs-xs);
    margin: 0;
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
    transition: border-color var(--motion-fast), color var(--motion-fast),
      background var(--motion-fast);
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
    color: var(--fg-0);
    box-shadow: var(--glow-magenta-md);
  }
</style>
