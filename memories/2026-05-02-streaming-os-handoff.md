# Streaming: sequential download + OS default player handoff

**Date:** 2026-05-02
**Status:** Decided

## Decision
Streaming = librqbit's sequential mode (already on, only mode it supports) + an "Open in player" action that hands the file to the OS default media player via the `open` crate.

## Why
Hobby-scope project. The engine already downloads sequentially, so head-of-file pieces arrive first by default. Handing off to the system player means zero player code, zero codec compatibility headaches, zero new native dependencies.

## Alternatives considered
- **Embed mpv (libmpv)** — best playback quality, all codecs, scriptable. Rejected: adds a real native dependency with platform-specific build steps, disproportionate cost for v0.1.
- **Embed VLC (libvlc)** — similar tradeoffs to mpv.
- **HTML5 `<video>` in the webview** — lives inside the cyberpunk UI, but webview codec support is limited (many MKV/HEVC files won't play). Rejected.
- **Skip streaming entirely** — not necessary; sequential is free, handoff is one button.

## Consequences
- Streaming experience is bounded by whatever the user has installed (VLC, IINA, mpv, Movies & TV, etc.). Acceptable.
- No in-app playback UI to design — frees up scope for the parts that matter.
- If users later ask for picture-in-picture, scrubbing-while-downloading visualization, or watch-together features, the embedded-player option opens up again.
