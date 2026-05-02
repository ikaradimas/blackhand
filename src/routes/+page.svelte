<script lang="ts">
  const version = "0.1.0";
</script>

<main class="boot">
  <div class="frame">
    <span class="bracket left">[</span>
    <h1 class="wordmark">BLACKHAND</h1>
    <span class="bracket right">]</span>
  </div>
  <p class="tagline">
    <span class="dim">//</span>
    torrent.client
    <span class="dim">·</span>
    v{version}
    <span class="dim">·</span>
    <span class="cyan">booting</span><span class="caret">_</span>
  </p>
</main>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    height: 100%;
    background: #07070C;
    color: #ECECF5;
    font-family: "Inter", ui-sans-serif, system-ui, -apple-system, sans-serif;
    font-feature-settings: "tnum", "ss01";
    overflow: hidden;
  }

  :global(body)::before {
    content: "";
    position: fixed;
    inset: 0;
    pointer-events: none;
    background: repeating-linear-gradient(
      to bottom,
      rgba(255, 255, 255, 0) 0px,
      rgba(255, 255, 255, 0) 2px,
      rgba(255, 255, 255, 0.02) 2px,
      rgba(255, 255, 255, 0.02) 3px
    );
    z-index: 1000;
  }

  .boot {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    gap: 1.25rem;
  }

  .frame {
    display: flex;
    align-items: center;
    gap: 1.5rem;
  }

  .bracket {
    color: #08F7FE;
    font-family: ui-monospace, "JetBrains Mono", "IBM Plex Mono", monospace;
    font-size: 4rem;
    font-weight: 300;
    text-shadow: 0 0 12px rgba(8, 247, 254, 0.45);
  }

  .wordmark {
    margin: 0;
    font-family: "Orbitron", ui-sans-serif, system-ui, sans-serif;
    font-size: clamp(2.5rem, 7vw, 4.5rem);
    font-weight: 700;
    letter-spacing: 0.18em;
    color: #FF2A6D;
    text-shadow:
      0 0 18px rgba(255, 42, 109, 0.55),
      0 0 36px rgba(255, 42, 109, 0.25);
  }

  .tagline {
    margin: 0;
    font-family: ui-monospace, "JetBrains Mono", "IBM Plex Mono", monospace;
    font-size: 0.875rem;
    letter-spacing: 0.1em;
    color: #B4B4D0;
  }

  .dim {
    color: #6E6E8C;
  }

  .cyan {
    color: #08F7FE;
  }

  .caret {
    display: inline-block;
    color: #08F7FE;
    animation: blink 1.1s steps(2, start) infinite;
  }

  @keyframes blink {
    to {
      visibility: hidden;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .caret {
      animation: none;
    }
  }
</style>
