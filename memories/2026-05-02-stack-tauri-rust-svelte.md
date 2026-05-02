# Stack: Tauri 2 + Rust + Svelte 5

**Date:** 2026-05-02
**Status:** Decided

## Decision
Build BlackHand on Tauri 2, with a Rust backend and a Svelte 5 + Vite + TypeScript frontend.

## Why
Hobby-scope desktop app targeting Win/macOS/Linux. Need a real systems language for the torrent engine (engine choice depends on this) and a flexible UI layer for a custom cyberpunk look. Tauri gives small binaries, native menus/tray, and Rust co-location with `librqbit`. Svelte's small runtime matches Tauri's "lean" promise and keeps custom UI work cheap.

## Alternatives considered
- **Flutter** — would let us share code with mobile later, but mobile is out of scope, and embedding a torrent engine means FFI to Rust/C++ anyway. Net cost without the benefit.
- **Electron + Node** — fastest to prototype, but heavy memory footprint conflicts with "shippable hobby tool," and we'd still need a native sidecar for torrent work.
- **Qt (C++ or PySide6)** — what qBittorrent uses; excellent native feel. Rejected because building a custom cyberpunk visual system is meaningfully more work in Qt than on a web stack.
- **React instead of Svelte** — bigger ecosystem (Framer Motion etc.). Acceptable swap, ~30 min of plumbing change. Not a blocker either way.

## Consequences
- Rust is now the load-bearing backend language. All engine integration, persistence, and IPC handlers live there.
- Webview-rendered UI means no native widgets — full control over look, but we own every component.
- Mobile is effectively foreclosed without a rewrite. Acceptable; mobile was never in scope.
- Distribution requires per-OS code-signing/notarization (macOS especially fiddly).
