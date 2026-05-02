# Phase 3: system integration + detail panel

**Date:** 2026-05-02
**Status:** Decided

## Decision
Phase 3 lands the OS-integration surface that turns BlackHand from "an app you alt-tab to" into "the app that handles torrent links and files on your machine":

- **`magnet://` deep links** via `tauri-plugin-deep-link` 2 + `tauri-plugin-single-instance` 2 (with the `deep-link` feature). Cold-start URLs go through `app.deep_link().get_current()`, runtime URLs through `on_open_url`. The single-instance plugin forwards URLs from a second launch through the deep-link channel automatically; our handler just brings the window forward.
- **`.torrent` file association** via `bundle.fileAssociations` in `tauri.conf.json` plus an `add_torrent_files_from_argv` helper that scans cold-start `std::env::args()` *and* the second-launch `argv` from the single-instance handler.
- **System tray** with menu (Show / Pause all / Resume all / Quit). Left-click toggles main-window visibility; right-click pops the menu. Pause-all / Resume-all iterate `api_torrent_list_ext` and dispatch the per-torrent action.
- **Native notifications** via `tauri-plugin-notification`. Stats loop tracks an `announced_done: HashSet<u64>`; when a torrent flips to finished and isn't in the set, fire one banner and add it. Pruning keeps the set in sync if a torrent is removed.
- **Per-torrent detail page** at `/torrent/[id]/`. Wraps the row in `<a href="…">` for SvelteKit-native navigation (action buttons use preventDefault+stopPropagation to suppress the link). Detail page shows progress, info card (hash, output folder, totals), and a files card with include/skip checkboxes + Apply that calls `update_only_files`.

## Why
- The killer flow ("click magnet in browser → BlackHand opens it") demanded the deep-link plugin pair; nothing simpler covers all three OSes.
- File associations (`.torrent`) are technically separate from URL schemes and come through argv on every desktop platform, so a single `add_torrent_files_from_argv` helper covers cold start and second launch.
- Tray + notifications are the conventional "background torrent client" UX. They're cheap with the Tauri 2 plugins.
- The detail panel was scoped to **info + file selection** for v0.1; trackers and peer list deferred — librqbit doesn't expose tracker URLs through the public Api facade, and the live peer list is more polish than essential.

## Alternatives considered
- **Manual URL-scheme registration** via custom `Info.plist` / Windows registry hacks — works but the plugin is one line, gives us the `on_open_url` callback, and handles all three platforms.
- **Tray menu via the macOS-only `tao::system_tray`** — superseded by `tauri::tray` in 2.x; portable across Mac/Windows/Linux.
- **Detail as a slide-in side panel instead of a route** — would have kept list visible. Rejected for v0.1: routes are simpler with SvelteKit, deep-linkable, and the back button is intuitive. Side-panel can be added later if it proves useful.
- **Stats poll for finished detection vs librqbit's own event stream** — librqbit doesn't expose a finished-event subscription on its public surface; polling is fine at 2 Hz.

## Concrete configuration
```json
// tauri.conf.json
"plugins": {
  "deep-link": { "desktop": { "schemes": ["magnet"] } }
},
"bundle": {
  ...
  "fileAssociations": [
    { "ext": ["torrent"], "name": "BitTorrent metainfo file",
      "role": "Editor", "mimeType": "application/x-bittorrent" }
  ]
}
```
```toml
# Cargo.toml
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-deep-link = "2"
tauri-plugin-notification = "2"

[target."cfg(...)".dependencies]
tauri-plugin-single-instance = { version = "2", features = ["deep-link"] }
```
- Capabilities now allow `core:default + opener:default + notification:default`.
- Tauri command surface gains `get_torrent_detail`, `set_only_files`. Bindings auto-update via `pnpm gen-bindings`.

## Consequences
- **macOS dev caveat:** URL scheme + file association registration only happen when the OS sees the `.app` bundle. `tauri dev` doesn't produce a registered bundle, so end-to-end testing requires `pnpm tauri build --debug` once. Document in README when README gets written.
- **Single-instance + deep-link feature flag:** if `tauri-plugin-single-instance` is ever upgraded across a major version, double-check the `deep-link` feature still routes URLs through the plugin's channel rather than argv.
- **Announced-done set is in-memory only.** A restart re-announces a finished torrent the first time we tick it. Not an issue today; revisit if it gets noisy.
- **File priorities are binary** (included/skipped) per librqbit. No three-tier high/normal/skip; that's a librqbit limitation, not ours.
- **Detail page polls `get_torrent_detail` at 1 Hz.** Could be replaced with a dedicated event for the open detail page later if profiling shows IPC pressure.
- **Trackers + peers list** are deferred to v0.2. Plan §7 already lists them as Tier-2 features.
