<script lang="ts">
  import "$lib/design/tokens.css";
  import "$lib/design/effects.css";
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  import AppHeader from "$lib/components/AppHeader.svelte";
  import AddTorrentModal from "$lib/components/AddTorrentModal.svelte";
  import SettingsModal from "$lib/components/SettingsModal.svelte";
  import RemoveConfirmModal from "$lib/components/RemoveConfirmModal.svelte";
  import AboutModal from "$lib/components/AboutModal.svelte";
  import { session } from "$lib/stores/session.svelte";
  import { torrents } from "$lib/stores/torrents.svelte";
  import { ui } from "$lib/stores/ui.svelte";

  let { children } = $props();

  onMount(() => {
    document.body.classList.add("fx-scanlines");
    session.start();
    torrents.start();
  });

  function isTextTarget(t: EventTarget | null): boolean {
    if (!(t instanceof HTMLElement)) return false;
    const tag = t.tagName;
    return (
      tag === "INPUT" ||
      tag === "TEXTAREA" ||
      tag === "SELECT" ||
      t.isContentEditable
    );
  }

  function onKeydown(e: KeyboardEvent) {
    // Use cmd on macOS, ctrl elsewhere.
    const mod = e.metaKey || e.ctrlKey;
    if (!mod) return;

    // Don't steal shortcuts while the user types in an input.
    if (isTextTarget(e.target)) return;

    switch (e.key.toLowerCase()) {
      case "n":
        e.preventDefault();
        ui.openAdd();
        break;
      case ",":
        e.preventDefault();
        ui.openSettings();
        break;
      case "w":
        e.preventDefault();
        // Close-to-tray: hide the window, keep the app running.
        getCurrentWindow().hide();
        break;
    }
  }
</script>

<svelte:window on:keydown={onKeydown} />

<AppHeader />
<main>
  {@render children()}
</main>
<AddTorrentModal />
<SettingsModal />
<RemoveConfirmModal />
<AboutModal />

<style>
  main {
    max-width: var(--content-max);
    margin: 0 auto;
    padding: var(--sp-5) var(--sp-5) var(--sp-7);
  }
</style>
