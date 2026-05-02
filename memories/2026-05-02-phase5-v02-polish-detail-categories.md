# Phase 5: v0.2 — polish, detail panel, categories

**Date:** 2026-05-02
**Status:** Decided

## Decision
v0.2 ships three feature areas the user picked from the §7 stretch list:

1. **UI polish** — toasts for ambient/async outcomes, loading skeletons, polished empty states.
2. **Detail panel polish** — per-file Open / Reveal-in-Finder, trackers card. Peer list **deferred** (engine constraint, see below).
3. **Categories / tags** — sidebar filter on the dashboard, set/clear from the detail page, persistent across forget+re-add.

## Why
- The user was explicit: ~2–3 features for v0.2, smallest-first. UI polish first since it benefits everything; detail panel second since it extends an existing screen; categories last since it's the biggest (new persistence + sidebar layout).
- Categories was the headline organizational feature. Detail polish was the natural follow-up to v0.1's already-shipped detail page.

## Architecture / decisions worth remembering

### Toasts vs inline errors
- `src/lib/stores/toasts.svelte.ts` is the queue + auto-dismiss; `ToastStack` renders top-right with kind-tagged left borders.
- **Form-action errors stay inline** in the modal that triggered them (Add, Settings, Remove). Toasts are reserved for ambient/async outcomes — torrent completed, magnet added, etc. The user explicitly steered us here mid-build; that's the design rule going forward.

### Loading skeleton
- `torrents` store gained a `loaded` flag flipped on first successful fetch OR first event tick.
- `SkeletonRow.svelte` mirrors `TorrentRow`'s grid template with shimmering CSS gradient placeholders. `prefers-reduced-motion` disables the animation.

### Categories
- **Persistence is our own JSON file** at `$DATA_DIR/blackhand/categories.json`, keyed by `info_hash`. librqbit doesn't model categories. Categories survive forget+re-add since the key is the hash, not the engine's torrent id.
- Categories are auto-created on first assignment with a new name. No explicit "create" UI, no rename/delete from the UI yet — edit `categories.json` by hand for those (deferred to a polish pass).
- `TorrentSummary` gained `category: Option<String>` so the snapshot stream already includes the category — no extra round-trip from the frontend.
- `TorrentSnapshot::from_response(resp, store)` is the new conversion entry point. The legacy `From<TorrentListResponse>` impl stays but passes `None`, so callers without a store still work.
- Sidebar lives in `+page.svelte` only — not in `+layout.svelte` — so the detail route stays full-width.

### Detail panel polish
- Per-file open/reveal: `@tauri-apps/api/path::join(output_folder, file.name)` then `plugin-opener::openPath` or `revealItemInDir`. No backend command needed — the path components are already in the DTO and tauri's path API handles platform separators.
- Tracker list: read `ManagedTorrentShared.trackers` (public) via `mgr_handle(idx).shared().trackers`. New `get_trackers` command returns `Vec<String>`.

## Engine limitations carried forward
- **No peer list panel.** `librqbit::Api::api_peer_stats` requires a `PeerStatsFilter` argument from the `torrent_state::live::peer::stats::snapshot` module, and `torrent_state` is `mod` (private) in librqbit's `lib.rs`. The type isn't re-exported, so external callers can't construct one. Per-torrent peer **count** is still available from `TorrentSummary.peers_live` (driven by `LiveStats.snapshot.peer_stats.live`). To unblock: file an upstream issue asking librqbit to re-export `PeerStatsFilter` (or add a no-arg `api_peers` convenience).
- **No force-reannounce.** No public method on `Api` to trigger a tracker reannounce. Trackers card stays read-only.

## What's deferred to v0.3+
- Category rename/delete from the UI
- Peer list (blocked by upstream until librqbit re-exports the filter type)
- Force-reannounce (same blocker shape)
- Watch folder, bandwidth scheduler, RSS, search, Tor/I2P (still unsharted from §7)
- Real signing/notarization for distribution

## Consequences
- Anyone adding a new modal should default to **inline errors** for actions that originate inside it; toast only for cross-screen results.
- The `categories.json` file format is now an external contract — bumping field shapes will need a small migrator if it changes.
- The dashboard's two-column layout (sidebar + content) is a real layout shift. Detail page intentionally bypasses it. If we ever add more global filters (tags? state filter?), they should compose with the existing sidebar pattern.
