<script lang="ts">
  import "$lib/design/tokens.css";
  import "$lib/design/effects.css";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  import AppHeader from "$lib/components/AppHeader.svelte";
  import AddTorrentModal from "$lib/components/AddTorrentModal.svelte";
  import SettingsModal from "$lib/components/SettingsModal.svelte";
  import RemoveConfirmModal from "$lib/components/RemoveConfirmModal.svelte";
  import AboutModal from "$lib/components/AboutModal.svelte";
  import ToastStack from "$lib/components/ToastStack.svelte";
  import { categories } from "$lib/stores/categories.svelte";
  import { session } from "$lib/stores/session.svelte";
  import { torrents } from "$lib/stores/torrents.svelte";
  import { ui } from "$lib/stores/ui.svelte";

  let { children } = $props();

  // The tray-popup window loads /tray-popup and must skip the main shell.
  let isPopup = $derived(page.url.pathname.startsWith("/tray-popup"));

  onMount(() => {
    document.body.classList.add("fx-scanlines");
    session.start();
    torrents.start();
    if (!isPopup) {
      categories.refresh();
    }
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
    // The popup window doesn't get the main app's shortcuts.
    if (isPopup) return;
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

{#if isPopup}
  {@render children()}
{:else}
  <AppHeader />
  <main>
    {@render children()}
  </main>
  <AddTorrentModal />
  <SettingsModal />
  <RemoveConfirmModal />
  <AboutModal />
  <ToastStack />
{/if}

<style>
  main {
    max-width: var(--content-max);
    margin: 0 auto;
    padding: var(--sp-5) var(--sp-5) var(--sp-7);
  }
</style>
