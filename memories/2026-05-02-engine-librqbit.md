# Engine: embed librqbit

**Date:** 2026-05-02
**Status:** Decided

## Decision
Use the Rust `librqbit` crate as the BitTorrent engine, embedded in-process.

## Why
Native Rust integration with the Tauri backend (no FFI), modern async API, MIT-licensed, actively maintained, and supports magnets / DHT / PEX / LSD / μTP / SOCKS5 / UPnP / IPv6 / seeding out of the box. Sequential download is its *only* mode — making the streaming feature essentially free.

## Alternatives considered
- **libtorrent-rasterbar (C++)** — most feature-complete (BT v2, super-seeding, MSE/PE, μTP). Powers qBittorrent and Deluge. Rejected for v0.1: C++ FFI integration is heavier and we don't need the long tail of features. Reconsider if MSE/PE encryption or BT v2 becomes load-bearing.
- **anacrolix/torrent (Go)** — mature, but adds Go to the toolchain alongside Rust+TS. Coordination cost not worth it.
- **From scratch** — multi-month detour just to reach magnet+DHT+PEX parity. Rejected; not the project's purpose.

## Consequences
- **No BT v2** support yet — fine for public/personal use; document in README.
- **No MSE/PE wire encryption** — networks that throttle plaintext BT may impact us. Document; revisit if it becomes a real complaint.
- Pin the librqbit version. The crate moves quickly — review the changelog before each minor bump and keep the wrapper layer (`session.rs`) thin so swapping engines later remains feasible.
- librqbit ships its own HTTP API + web UI; we're using it purely as a library, ignoring those surfaces.
