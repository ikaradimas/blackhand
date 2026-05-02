# Phase 2: typed IPC + neon-noir UI

**Date:** 2026-05-02
**Status:** Decided

## Decision
Phase 2 settled on three architectural choices that the rest of the project will build on:

1. **Typed IPC via `tauri-specta` 2.0-rc.24** — generated `src/lib/bindings.ts` is the single source of truth for the command/event surface. Frontend imports `commands.foo(...)` and `events.bar.listen(...)` instead of stringly-typed `invoke()` calls.
2. **Custom DTOs over librqbit's response types** — `TorrentSummary`, `TorrentSnapshot`, `SessionStats`, `AppSettings` etc. live in `src-tauri/src/types.rs` (and `settings.rs`) with `serde + specta::Type`. Conversion from librqbit's `TorrentListResponse` / `SessionStatsSnapshot` happens via `From` impls so the wire shape is decoupled from upstream evolution.
3. **Snapshot-style stats events at 2 Hz** — every tick we emit a full `TorrentsSnapshotEvent` + `SessionStatsEvent`. Simpler than diffing; fine until ~50 torrents.

## Why
- The user picked single-source-of-truth typing explicitly. tauri-specta also gives typed event listeners (not just types) which `ts-rs` doesn't, so the DX win compounds.
- DTOs let us only ship fields the UI actually renders (smaller payloads, simpler frontend types) and insulate us from librqbit's frequent crate updates.
- Hobby-scale torrent counts make snapshot emission trivial. Re-evaluate if profiles say otherwise.

## Alternatives considered
- **`ts-rs` + hand-written invoke wrappers** — types only, no command/event bindings. Rejected: significantly more glue per command.
- **Re-export librqbit response types directly in IPC** — would couple our wire contract to the upstream crate; any breaking change in librqbit forces a frontend churn.
- **Per-torrent delta events** — more efficient at scale, but premature optimization for v0.1.

## Bindings generation pipeline
- Builder constructed in `src-tauri/src/lib.rs::make_specta_builder()`
- Exported every `tauri dev` launch (debug-only branch)
- Also exported by `cargo test export_bindings` → wrapped as `pnpm gen-bindings` for headless regeneration
- Output path: `src/lib/bindings.ts` (also imported into stores/components via `$lib/bindings`)

## Frontend architecture decisions
- **Svelte 5 runes** in `.svelte.ts` files for stores (`session`, `torrents`, `ui`). Single global instance per store; auto-subscribes to typed events on first `start()` call from `+layout.svelte`.
- **Native `<dialog>` element** for modals — free `::backdrop`, ESC handling, accessible focus management. Three modals share a generic `Modal.svelte` wrapper (`AddTorrentModal`, `SettingsModal`, `RemoveConfirmModal`).
- **5×5 vertically-mirrored pixel-art identicons** for the per-torrent state mark (`PixelMark.svelte`). Deterministic from `info_hash` so a torrent always renders the same pattern. Color encodes state; tooltip on hover gives the state name.
- **Discriminated-union command results** (`{status:"ok"|"error"}`) get collapsed by `src/lib/api.ts::unwrap()` into a normal Promise that throws on error.
- **Design tokens in CSS custom properties** (`src/lib/design/tokens.css`) — palette, type scale, spacing, motion, glow shadows. `effects.css` provides scanline overlay + glitch keyframes. Both globally imported via `+layout.svelte`.

## librqbit gotchas worth remembering
- **`Speed.mbps` is *mebibytes per second*, not megabits.** Display impl prints `"{:.2} MiB/s"`. Convert with `× 1024 × 1024`, never `× 125_000`. Cost us a ~8× display bug.
- **Most session-level settings are restart-only** (port range, DHT, UPnP, download dir) — no public runtime setter. Settings UI surfaces this with a cyan "restart required" banner that only appears when those fields actually changed.
- **Bandwidth limits *do* live-update** via `Session.ratelimits.set_upload_bps()` / `set_download_bps()`. Wire `save_settings` to call those alongside the JSON write.
- **`save_settings(new: ...)` collided with a JS reserved word** when specta emitted the param name verbatim into TS. Renamed to `settings`. Defensive note for future commands.

## Out-of-scope (intentionally deferred from Phase 2)
- Detail panel / per-torrent file selection UI
- RSS, search, watch folder, scheduler, IP filter, Tor/I2P
- BT v2, MSE/PE wire encryption (waiting on librqbit)
- Tray, magnet protocol handler, single-instance lock — Phase 3
- Code signing / notarization / install bundles — Phase 4

## Consequences
- Adding a new command is now: write Rust fn with `#[tauri::command] #[specta::specta]`, add to `collect_commands![...]` in `make_specta_builder`, run `pnpm gen-bindings`, use it on the frontend with full types.
- Adding a new event: derive `tauri_specta::Event` on a serde+Type struct, add to `collect_events![...]`, emit via `MyEvent(payload).emit(&app_handle)`.
- librqbit major-version bumps shouldn't require frontend changes unless we extend the DTO surface.
- The 2 Hz snapshot is profile-friendly to re-tune (single constant in `stats.rs::TICK`).
