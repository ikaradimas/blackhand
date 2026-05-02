<script lang="ts">
  import { commands } from "$lib/bindings";
  import { unwrap } from "$lib/api";
  import Modal from "$lib/components/Modal.svelte";
  import { toasts } from "$lib/stores/toasts.svelte";
  import { ui } from "$lib/stores/ui.svelte";

  let busy = $state(false);
  let error = $state<string | null>(null);

  const target = $derived(ui.removeTarget);

  // Clear inline error whenever the target changes (next time the modal opens).
  $effect(() => {
    void target;
    error = null;
  });

  function close() {
    ui.cancelRemove();
  }

  async function confirm() {
    if (!target) return;
    busy = true;
    error = null;
    try {
      await unwrap(commands.delete(target.id));
      toasts.ok(`deleted: ${target.name}`);
      ui.cancelRemove();
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
</script>

<Modal open={target !== null} title="Delete torrent + files" onclose={close}>
  {#if target}
    <p class="lead">
      Permanently delete this torrent <strong>and the files on disk</strong>?
    </p>

    <p class="name">{target.name}</p>

    <p class="warn tnum">
      <span class="warn-icon">⚠</span>
      this cannot be undone
    </p>

    {#if error}
      <p class="err tnum">{error}</p>
    {/if}

    <div class="actions">
      <button type="button" onclick={close} disabled={busy}>Cancel</button>
      <button
        type="button"
        class="danger"
        onclick={confirm}
        disabled={busy}
      >
        {busy ? "deleting…" : "Delete + remove files"}
      </button>
    </div>
  {/if}
</Modal>

<style>
  .lead {
    margin: 0 0 var(--sp-3);
    color: var(--fg-0);
    font-size: var(--fs-sm);
    line-height: var(--lh-loose);
  }
  .lead strong {
    color: var(--err);
    font-weight: 500;
  }

  .name {
    margin: 0 0 var(--sp-3);
    padding: var(--sp-2) var(--sp-3);
    background: var(--bg-1);
    border: 1px solid var(--bg-3);
    border-radius: var(--radius-md);
    font-family: var(--font-mono);
    font-size: var(--fs-sm);
    color: var(--fg-1);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .warn {
    margin: 0 0 var(--sp-3);
    color: var(--warn);
    font-size: var(--fs-xs);
    display: flex;
    align-items: center;
    gap: var(--sp-2);
  }
  .warn-icon {
    font-family: var(--font-mono);
    font-size: var(--fs-md);
  }

  .err {
    color: var(--err);
    background: rgba(255, 63, 63, 0.08);
    border: 1px solid rgba(255, 63, 63, 0.3);
    padding: var(--sp-2) var(--sp-3);
    border-radius: var(--radius-md);
    font-size: var(--fs-xs);
    margin: 0 0 var(--sp-3);
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--sp-2);
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

  button.danger {
    background: var(--err);
    border-color: var(--err);
    color: var(--fg-0);
    box-shadow: 0 0 8px rgba(255, 63, 63, 0.4);
  }
  button.danger:hover:not(:disabled) {
    background: var(--err);
    border-color: var(--err);
    color: var(--fg-0);
    box-shadow: 0 0 14px rgba(255, 63, 63, 0.6), 0 0 28px rgba(255, 63, 63, 0.25);
  }
</style>
