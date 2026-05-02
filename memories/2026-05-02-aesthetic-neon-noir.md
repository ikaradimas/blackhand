# Aesthetic: Neon Noir

**Date:** 2026-05-02
**Status:** Decided

## Decision
Neon Noir cyberpunk visual identity: magenta (`#FF2A6D`) + cyan (`#08F7FE`) accents on near-black (`#07070C` / `#0E0E16`) surfaces, mono numerics with tabular figures, restrained scanline overlay (≤2% opacity), glitch effects only on state transitions. Full token list in `tasks/todo.md` §5.

## Why
User explicitly asked for cyberpunk *balanced with visual clarity*. Neon Noir gives a strong, identifiable identity while keeping body text high-contrast and numeric tables legible — which is what a torrent client mostly is.

## Alternatives considered
- **Matrix Terminal** (phosphor green on black, mono everywhere) — strong hacker-cred but poorly suited to dense data tables; long sessions strain the eyes. Rejected.
- **Tokyo / Bladerunner** (warm navy + amber + teal, JP signage accents) — atmospheric but less stark; could revisit later as an alt theme.
- **Glitch / CRT heavy** (chromatic aberration, scanlines, flicker on text) — directly conflicts with the clarity requirement. Rejected.

## Consequences
- Magenta is the "look here" signal — must be used sparingly. Hard rule: ≤3 magenta elements per screen.
- No tinted body text. No green-on-black walls. Effects are chrome, not content.
- Scanlines never overlay numeric tables or body copy; only backgrounds and decorative chrome.
- Glitch effects reserved for state transitions (just-added, just-completed). Never decorative.
- Tokyo and Matrix remain candidates as opt-in alternate themes in a later milestone.
