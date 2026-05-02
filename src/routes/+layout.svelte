<script lang="ts">
  import "$lib/design/tokens.css";
  import "$lib/design/effects.css";
  import { onMount } from "svelte";

  import AppHeader from "$lib/components/AppHeader.svelte";
  import { session } from "$lib/stores/session.svelte";
  import { torrents } from "$lib/stores/torrents.svelte";

  let { children } = $props();

  onMount(() => {
    document.body.classList.add("fx-scanlines");
    session.start();
    torrents.start();
  });
</script>

<AppHeader />
<main>
  {@render children()}
</main>

<style>
  main {
    max-width: var(--content-max);
    margin: 0 auto;
    padding: var(--sp-5) var(--sp-5) var(--sp-7);
  }
</style>
