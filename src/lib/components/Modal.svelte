<script lang="ts">
  import type { Snippet } from "svelte";

  type Props = {
    open: boolean;
    title: string;
    onclose: () => void;
    children: Snippet;
  };

  let { open, title, onclose, children }: Props = $props();

  let el = $state<HTMLDialogElement>();

  $effect(() => {
    if (!el) return;
    if (open && !el.open) el.showModal();
    if (!open && el.open) el.close();
  });

  function onBackdropClick(e: MouseEvent) {
    // Native <dialog> dispatches click on the dialog itself when the user
    // clicks the backdrop (the dialog is the click target, not its children).
    if (e.target === el) onclose();
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<dialog
  bind:this={el}
  onclose={onclose}
  onclick={onBackdropClick}
  aria-labelledby="modal-title"
>
  <header class="hdr">
    <h2 id="modal-title">{title}</h2>
    <button class="close" type="button" onclick={onclose} aria-label="Close">×</button>
  </header>
  <div class="body">
    {@render children()}
  </div>
</dialog>

<style>
  dialog {
    background: var(--bg-2);
    color: var(--fg-0);
    border: 1px solid var(--bg-3);
    border-radius: var(--radius-lg);
    padding: 0;
    width: min(520px, calc(100vw - var(--sp-6)));
    max-height: 80vh;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.6), var(--glow-magenta-sm);
  }

  dialog::backdrop {
    background: rgba(7, 7, 12, 0.78);
    backdrop-filter: blur(4px);
  }

  .hdr {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--sp-3) var(--sp-4);
    border-bottom: 1px solid var(--bg-3);
  }

  h2 {
    margin: 0;
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
    font-weight: 500;
    letter-spacing: var(--tracking-widest);
    text-transform: uppercase;
    color: var(--accent-magenta);
  }

  .close {
    background: transparent;
    border: 1px solid transparent;
    color: var(--fg-2);
    width: 28px;
    height: 28px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    font-size: var(--fs-lg);
    line-height: 1;
    transition: border-color var(--motion-fast), color var(--motion-fast);
  }
  .close:hover {
    border-color: var(--accent-cyan);
    color: var(--accent-cyan);
  }

  .body {
    padding: var(--sp-4);
  }
</style>
