<script lang="ts">
  type Props = { delay?: number };
  let { delay = 0 }: Props = $props();
</script>

<article class="row" style:--delay="{delay}ms">
  <div class="grid">
    <span class="cell sq" style:width="18px" style:height="18px"></span>
    <span class="cell name"></span>
    <span class="cell num size"></span>
    <span class="cell num"></span>
    <span class="cell num"></span>
    <span class="cell num peers"></span>
    <span class="cell num eta"></span>
    <span class="cell actions"></span>
  </div>
  <div class="bar"></div>
</article>

<style>
  .row {
    background: var(--bg-1);
    border: 1px solid var(--bg-3);
    border-radius: var(--radius-lg);
    padding: var(--sp-3) var(--sp-4);
    display: flex;
    flex-direction: column;
    gap: var(--sp-2);
  }

  .grid {
    display: grid;
    grid-template-columns: 18px minmax(0, 1fr) 80px 110px 110px 60px 80px 124px;
    gap: var(--sp-3);
    align-items: center;
  }

  .cell {
    display: block;
    height: 12px;
    background: linear-gradient(
      90deg,
      var(--bg-2) 0%,
      var(--bg-3) 50%,
      var(--bg-2) 100%
    );
    background-size: 200% 100%;
    border-radius: var(--radius-sm);
    animation: shimmer 1.6s ease-in-out infinite;
    animation-delay: var(--delay);
  }

  .sq {
    border-radius: 2px;
  }

  .name { width: 60%; }
  .size { width: 50px; margin-left: auto; }
  .peers { width: 30px; margin-left: auto; }
  .eta { width: 60px; margin-left: auto; }
  .actions { width: 100%; height: 22px; }

  .num { justify-self: end; }

  .bar {
    height: 8px;
    background: linear-gradient(
      90deg,
      var(--bg-2) 0%,
      var(--bg-3) 50%,
      var(--bg-2) 100%
    );
    background-size: 200% 100%;
    border-radius: var(--radius-sm);
    animation: shimmer 1.6s ease-in-out infinite;
    animation-delay: var(--delay);
  }

  @keyframes shimmer {
    0% { background-position: 200% 0; }
    100% { background-position: -200% 0; }
  }

  @media (prefers-reduced-motion: reduce) {
    .cell, .bar {
      animation: none;
    }
  }
</style>
