# Scope: lean MVP for v0.1 (Tier-1 features only)

**Date:** 2026-05-02
**Status:** Decided

## Decision
v0.1 ships only Tier-1 ("table-stakes") features: add via magnet/file, pause/resume/remove, file selection with priorities, global + per-torrent speed limits, port + max-peers settings, persistent resume, system tray, magnet protocol handler, "open in player," "reveal in finder/explorer," single-instance lock. Everything else is deferred.

Explicitly out of scope for v0.1: RSS, multi-tracker search, Tor/I2P/SOCKS5-binding/kill-switch, web remote UI, mobile companion, embedded media player, BT v2, MSE/PE encryption, categories/tags, watch folder, bandwidth scheduler, IP filter.

## Why
Hobby-scope project. Optimization target is the shortest credible path to a usable, shippable v0.1 — *not* feature parity with qBittorrent. Adding Tier-2/3 features now would multiply implementation time and surface area for bugs without proving the product hangs together.

## Alternatives considered
- **Polished open-source release** (MVP + table-stakes-plus: categories, scheduler, watch folders, theming, optional web UI) — rejected as v0.1; reasonable v0.2+ direction.
- **Full qBittorrent competitor** — rejected; months of work, mismatched with hobby scope.
- **Learning-focused** (build engine pieces from scratch) — rejected; engine is `librqbit`'s job.

## Consequences
- v0.1 estimate: ~7–10 working days of focused solo work + signing/notarization friction. See `tasks/todo.md` §8.
- The deferred Tier-2/3 list is the v0.2+ candidate pool. When picking the next milestone, take 2–3 features max — don't blanket-add.
- When the user asks to add a feature mid-v0.1, default response is "is this Tier-1, or do we defer?" — and only expand scope with explicit go-ahead.
