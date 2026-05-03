# Semantic palette — pulled into the neon-noir family

**Date:** 2026-05-03
**Status:** Decided

## Decision
Replaced the conventional traffic-light semantic colors with neon-noir-coherent values that live in the same chromatic family as the magenta + cyan brand accents.

| Token | Before | After | Notes |
|---|---|---|---|
| `--ok` | `#39FF14` (toxic green) | **`#2EFFC4`** | Neon aqua-mint. Reads as "good" (green family) but lives in the cyan family — harmonizes with `--accent-cyan` (`#08F7FE`) instead of fighting it. |
| `--warn` | `#FFB23F` (amber) | **`#FFAC1C`** | Slightly hotter / more saturated amber. Already cyberpunk-friendly; the small shift just makes it glow more. |
| `--err` | `#FF3F3F` (pure red) | **`#FF073A`** | Neon alarm-red with a small blue component (magenta undertone) — distinct from `--accent-magenta` (`#FF2A6D`) but harmonizes with it instead of clashing. |

Also added helper tokens for the recurring inline-rgba patterns so future components don't hardcode alpha values:

- `--err-bg` (soft tinted background) + `--err-border` (soft border) — for inline error banners and form-field errors.
- `--ok-bg` — for "saved" / success banners.
- `--glow-err-sm` / `--glow-err-md` — for danger button + hover glow (matches the existing magenta/cyan glow pattern).
- `--glow-ok-sm` — for finished progress-bar fill.

## Why
The user noted the conventional traffic-light palette (pure red / pure green) clashes with the neon aesthetic — the colors look conventional rather than coherent with the brand. Cyberpunk palettes typically push semantic states into the same saturated-neon-on-near-black family as the brand accents; the visual system stays unified instead of feeling stitched together.

Constraints respected:
- **Semantic distinguishability** preserved — ok / warn / err are still immediately recognizable to users (mint = good, amber = caution, alarm-red = bad), just in the project's chromatic register.
- **Contrast** still high on `--bg-0` (`#07070C`) — readable for icons, borders, and small accent text.
- **Magenta brand sparing rule** unchanged — the new err color isn't magenta, it's a distinct red, so it doesn't cannibalize the "look here" signal that magenta carries.

## Alternatives considered
- **Keep amber, only swap green + red** — minimal change but the green and red were the worst clashes and amber needed the smallest adjustment, so we batched them.
- **Use `--accent-violet` for warn** — distinctive but doesn't read as "caution" universally; cognitive load too high.
- **Replace err with hot magenta `#FF073A` near `--accent-magenta`** — would have collapsed brand and error into the same hue family. Kept distinct (the new err has more red, less blue than the magenta accent) so the brand signal isn't diluted.

## Consequences
- Toasts, modals, banners, buttons, progress bars, and state dots all pick up the new colors automatically via the token swap. No per-component visual tuning needed.
- New helper tokens (`--err-bg`, `--err-border`, `--ok-bg`, `--glow-err-sm`/`md`, `--glow-ok-sm`) become the canonical way to style inline banners and danger buttons. **Don't hardcode rgba values in components going forward** — extend the token system instead.
- The "Saved" banner in Settings uses `--ok` (mint) with `--ok-bg` background — looks clearly affirmative without the toxic-green overload.
- The danger button in RemoveConfirmModal looks meaner now (the alarm-red + slight magenta undertone glow pairs with the magenta brand, reinforcing "this is the destructive corner of the magenta family").
- If we ever add an opt-in alternate theme (Tokyo, Matrix — both noted in the original aesthetic memo), the semantic tokens are the abstraction point: change tokens.css, every component follows.
