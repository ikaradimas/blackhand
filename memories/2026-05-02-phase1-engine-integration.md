# Phase 1: librqbit::Api as managed state, no custom actor

**Date:** 2026-05-02
**Status:** Decided

## Decision
Use `Arc<librqbit::Api>` directly as Tauri managed state. Tauri commands take `State<'_, Arc<Api>>` and call `Api` methods directly. The stats event loop owns a cloned `Arc<Api>` and emits two events at 2 Hz: `torrents:snapshot` (full list with stats) and `session:stats` (global counters). No custom actor wrapper, no mpsc channel layer.

## Why
The implementation plan (§3.1) sketched a `SessionHandle` actor wrapping `librqbit::Session` via Tokio mpsc. After reading the librqbit 8.1 source we found `librqbit::Api` — a facade librqbit ships *specifically for desktop embedding* (it powers librqbit's own rqbit desktop app). It already wraps `Arc<Session>`, exposes async methods (pause/start/forget/delete/add/list/stats), and returns serde-serializable response types. Building our own actor on top would have been pure redundancy.

## Alternatives considered
- **Custom `SessionHandle` actor (per the plan)** — rejected. Adds an indirection that buys nothing once `Arc<Api>` is already thread-safe, async, and serde-friendly.
- **Per-torrent stats events with delta diffing** — rejected for v0.1. Cheap full-snapshot emission at 2 Hz is fine for hobby-scale (≤ tens of torrents). Revisit when we hit a torrent count where serialization shows up in profiles.
- **`thiserror`-typed error enum for command shims** — deferred. Currently using `Result<T, String>` via `.map_err(|e| e.to_string())`. The command surface is too unstable for typed errors to pay off yet; will add when the frontend needs to branch on error variants (likely Phase 2/3).
- **Direct `tracing` dependency for emit-failure logs** — skipped. Used `eprintln!` so stats-loop emit failures show up in the dev console without pulling tracing as a direct dep (it's already transitive via librqbit).

## Concrete configuration
- `Session::new_with_opts(downloads, opts)` where:
  - `downloads = ~/Downloads/BlackHand` (created on first launch via `dirs::download_dir()`)
  - `SessionOptions { fastresume: true, persistence: Some(Json { folder: data_dir/session }), enable_upnp_port_forwarding: true, ..default }`
- `data_dir = dirs::data_dir() / "blackhand"` (e.g. `~/Library/Application Support/blackhand/` on macOS)
- Default DHT, PEX, LSD, and listen-port range left at librqbit defaults
- Stats poll interval: 500 ms with `MissedTickBehavior::Skip`

## Tauri command surface (frontend-facing)
| Command | Args | Return |
|---|---|---|
| `add_magnet` | `uri: String` | `serde_json::Value` (ApiAddTorrentResponse) |
| `add_torrent_file` | `bytes: Vec<u8>` | same |
| `list_torrents` | — | `serde_json::Value` (TorrentListResponse) |
| `get_torrent` | `id: TorrentIdOrHash` | `serde_json::Value` (TorrentDetailsResponse) |
| `pause` / `resume` / `forget` / `delete` | `id: TorrentIdOrHash` | `()` |
| `session_stats` | — | `serde_json::Value` (SessionStatsSnapshot) |

Note: `forget` keeps files on disk; `delete` removes them. Mirrors librqbit's split.

## Consequences
- Frontend currently defines TypeScript types ad hoc against `serde_json::Value` returns. Tighter typing arrives when we generate TS from Rust (e.g. `ts-rs` or hand-written specta-ish module) — that's a Phase 2 candidate, not v0.1 work.
- Snapshot emission at 2 Hz pushes a few KB per tick per torrent. Acceptable for hobby use; should diff once we cross ~50 torrents.
- Persistence + fastresume are *load-bearing*: the integration test confirmed quitting Cmd-Q and relaunching restores torrents and resumes them where they left off. If anyone touches `SessionPersistenceConfig` later, re-test the resume path.
- librqbit's default features (`default-tls`, `http-api-client`) don't pull in the embedded HTTP API server (that's behind the `http-api` feature). We are *only* using the library/Api surface, not librqbit's bundled web UI.
- `Api::new` has a `tracing-subscriber-utils`-gated parameter; we're not enabling that feature, so the constructor is `Api::new(session, None)` (just session + log-reload tx).
