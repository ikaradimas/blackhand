<script lang="ts">
  import "$lib/design/tokens.css";
  import "$lib/design/effects.css";
  import { onMount } from "svelte";

  let { children } = $props();

  onMount(() => {
    document.body.classList.add("fx-scanlines");
  });
</script>

{@render children()}
