<script lang="ts">
  import { fly } from "svelte/transition";

  import { toasts } from "$lib/stores/toasts.svelte";
</script>

<div class="stack" aria-live="polite">
  {#each toasts.list as t (t.id)}
    <button
      type="button"
      class="toast t-{t.kind}"
      transition:fly={{ x: 240, duration: 220 }}
      onclick={() => toasts.dismiss(t.id)}
      aria-label="Dismiss notification"
    >
      <span class="kind">{t.kind}</span>
      <span class="msg">{t.message}</span>
    </button>
  {/each}
</div>

<style>
  .stack {
    position: fixed;
    top: calc(var(--header-height) + var(--sp-3));
    right: var(--sp-4);
    z-index: var(--z-tooltip);
    display: flex;
    flex-direction: column;
    gap: var(--sp-2);
    max-width: 380px;
    pointer-events: none;
  }

  .toast {
    pointer-events: auto;
    display: flex;
    flex-direction: column;
    gap: 2px;
    text-align: left;
    background: var(--bg-2);
    border: 1px solid var(--bg-3);
    border-left-width: 3px;
    border-radius: var(--radius-md);
    padding: var(--sp-2) var(--sp-3);
    color: var(--fg-0);
    font-family: inherit;
    font-size: var(--fs-sm);
    cursor: pointer;
    box-shadow: 0 6px 18px rgba(0, 0, 0, 0.5);
    transition: border-color var(--motion-fast);
  }
  .toast:hover {
    border-color: var(--fg-2);
  }

  .kind {
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
    letter-spacing: var(--tracking-wider);
    text-transform: uppercase;
    color: var(--fg-2);
  }

  .msg {
    color: var(--fg-0);
    word-break: break-word;
    overflow-wrap: anywhere;
  }

  .t-error { border-left-color: var(--err); }
  .t-error .kind { color: var(--err); }

  .t-warn { border-left-color: var(--warn); }
  .t-warn .kind { color: var(--warn); }

  .t-info { border-left-color: var(--accent-cyan); }
  .t-info .kind { color: var(--accent-cyan); }

  .t-ok { border-left-color: var(--ok); }
  .t-ok .kind { color: var(--ok); }
</style>
