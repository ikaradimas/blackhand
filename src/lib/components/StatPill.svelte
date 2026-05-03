<script lang="ts">
  type Props = {
    label: string;
    value: string;
    accent?: "magenta" | "cyan" | "neutral" | "warn" | "err";
    title?: string;
  };
  let { label, value, accent = "neutral", title }: Props = $props();
</script>

<div class="pill" data-accent={accent} {title}>
  <span class="label">{label}</span>
  <span class="value tnum">{value}</span>
</div>

<style>
  .pill {
    display: inline-flex;
    align-items: baseline;
    gap: var(--sp-2);
    padding: var(--sp-1) var(--sp-3);
    background: var(--bg-1);
    border: var(--border-faint);
    border-radius: var(--radius-md);
    font-size: var(--fs-sm);
    line-height: 1;
  }

  .label {
    color: var(--fg-2);
    font-size: var(--fs-xs);
    letter-spacing: var(--tracking-wider);
    text-transform: uppercase;
  }

  .value {
    color: var(--fg-0);
    font-size: var(--fs-sm);
  }

  .pill[data-accent="magenta"] .value {
    color: var(--accent-magenta);
  }
  .pill[data-accent="cyan"] .value {
    color: var(--accent-cyan);
  }
  .pill[data-accent="warn"] .value {
    color: var(--warn);
  }
  .pill[data-accent="err"] {
    border-color: var(--err-border);
  }
  .pill[data-accent="err"] .value {
    color: var(--err);
  }
</style>
