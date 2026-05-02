# BlackHand — Implementation Plan

A cross-platform desktop BitTorrent client with a neon-noir cyberpunk identity.
Personal/hobby ambition: ship something usable, then iterate.

---

## 1. Locked-in decisions

| Dimension | Decision |
|---|---|
| Platforms | Windows, macOS, Linux (desktop only) |
| Shell | Tauri 2.x |
| Backend lang | Rust |
| Torrent engine | `librqbit` (embedded as a crate) |
| Frontend | Svelte 5 + Vite + TypeScript (recommendation — see §4) |
| Streaming | Sequential download (free with librqbit) + open file with OS default player |
| Aesthetic | Neon Noir — magenta + cyan on near-black, restrained scanlines, mono for numbers |
| Ambition | Hobby; lean MVP first, polish second, breadth later |

Out of scope for v0.1: RSS, multi-tracker search, Tor/I2P, web remote UI, mobile, embedded media player, BT v2, MSE/PE encryption.

---

## 2. Feature research — what real users use

Survey of qBittorrent, Transmission, Deluge, μTorrent, rTorrent/ruTorrent, WebTorrent. Features ranked by how often you see them praised, complained about when missing, or ranked top in user surveys.

**Tier 1 — table-stakes (any MVP needs these):**
- Add by magnet link + `.torrent` file (drag-and-drop, paste, "Open with…" registration)
- Per-torrent: pause / resume / remove (with optional file deletion) / force re-announce
- Per-torrent file selection with priorities (skip / normal / high)
- Global + per-torrent speed limits (up/down)
- Configurable listen port, max global / per-torrent peers, max active torrents
- Default download directory; per-torrent override at add time
- Persistent state across restarts (resume in-flight torrents)
- System tray with quick-actions and minimize-to-tray

**Tier 2 — heavy users expect:**
- Categories / tags / labels (qBit's killer organizational feature)
- Watch folder (auto-add `.torrent` files dropped in a directory)
- Move on completion (separate downloading dir vs library dir)
- Tracker list view with add/remove/force-update
- Bandwidth scheduler (alt rate limits on a weekly grid)
- Statistics: session totals, all-time, ratio, share time
- IP filter / blocklist
- Pre-allocate disk space toggle
- "Sequential download" toggle and "first/last piece priority" for streaming
- Peer list with country flags, client ID, progress

**Tier 3 — power features (polish/differentiation):**
- RSS auto-download with regex/episode rules
- Built-in search across trackers (plugin model)
- Web/remote UI for headless or LAN access
- Scripting / external program on torrent complete
- Anonymity: SOCKS5 binding, VPN/interface kill-switch, Tor/I2P
- Super-seeding mode
- Torrent creation wizard
- BT v2 / hybrid torrents

**For BlackHand v0.1 we ship Tier 1 only.** v0.2 picks selectively from Tier 2.

---

## 3. Architecture

```
┌─────────────────────────────────────────────────────────┐
│  Tauri shell (window, tray, menu, file associations,    │
│   protocol handler for magnet:, single-instance lock)   │
├──────────────────────────┬──────────────────────────────┤
│  Rust backend (in-proc)  │  Svelte frontend (webview)   │
│  ┌────────────────────┐  │  ┌────────────────────────┐  │
│  │ blackhand-core     │  │  │ Routes / pages         │  │
│  │  - Session         │  │  │  - /downloads          │  │
│  │  - Torrent state   │  │  │  - /torrent/:id        │  │
│  │  - Settings store  │◄─┼─►│  - /settings           │  │
│  │  - Event bus       │  │  │ Components / styles    │  │
│  └────────┬───────────┘  │  └────────────────────────┘  │
│           │              │                              │
│  ┌────────▼───────────┐  │  Tauri events ◄───── 1-way   │
│  │ librqbit (embed)   │  │  Tauri commands ─────► 2-way │
│  └────────────────────┘  │                              │
└──────────────────────────┴──────────────────────────────┘
```

### 3.1 Backend module layout

```
src-tauri/
  Cargo.toml
  src/
    main.rs              // Tauri bootstrap, command/event registration
    commands.rs          // #[tauri::command] handlers (thin shims)
    session.rs           // librqbit Session lifecycle, persistence
    torrents.rs          // Add / pause / resume / remove, file priorities
    stats.rs             // Snapshot + diff loop pushed to UI as events
    settings.rs          // Serde-backed settings, atomic save, defaults
    paths.rs             // OS-correct config/data dirs (dirs crate)
    streaming.rs         // "Open with system player" — open crate
    tray.rs              // Tray menu, click handlers
    protocol.rs          // magnet: URL handler registration
    error.rs             // thiserror enum mapped to user-facing messages
```

### 3.2 Frontend module layout

```
src/
  app.html
  routes/
    +layout.svelte        // chrome: title bar, tray-aware window controls
    +page.svelte          // /downloads — main list
    torrent/[id]/+page.svelte
    settings/+page.svelte
  lib/
    api.ts                // typed wrapper over invoke() / event listeners
    stores/
      torrents.ts         // reactive list, derived from event stream
      session.ts
      settings.ts
    components/
      TorrentRow.svelte
      ProgressBar.svelte  // segmented neon bar
      StatPill.svelte
      Bracket.svelte      // corner-cut chrome decoration
      CommandPalette.svelte
    design/
      tokens.css          // CSS custom properties (palette, type, spacing)
      effects.css         // scanlines, glow utilities
      fonts.css
```

### 3.3 IPC contract (Tauri commands + events)

Commands (UI → Rust, return a typed result):

| Command | Payload | Returns |
|---|---|---|
| `add_magnet` | `{ uri, save_path?, paused? }` | `TorrentSummary` |
| `add_file` | `{ bytes, save_path?, paused? }` | `TorrentSummary` |
| `pause` / `resume` / `remove` | `{ id, delete_files? }` | `()` |
| `set_file_priority` | `{ id, file_idx, priority }` | `()` |
| `set_speed_limits` | `{ id?, up_kib?, down_kib? }` | `()` |
| `force_reannounce` | `{ id }` | `()` |
| `list_torrents` | `()` | `TorrentSummary[]` |
| `get_torrent` | `{ id }` | `TorrentDetail` |
| `get_settings` / `save_settings` | … | … |
| `open_in_player` | `{ id, file_idx }` | `()` |
| `reveal_in_finder` | `{ id, file_idx? }` | `()` |

Events (Rust → UI, pushed at ~2 Hz when window visible, ~0.2 Hz when minimized):

| Event | Payload |
|---|---|
| `torrent:stats` | per-torrent up/down/peers/eta/state delta |
| `torrent:added` / `torrent:removed` | id |
| `torrent:completed` | id (used for tray notification + glitch-on-complete UI cue) |
| `session:stats` | global up/down totals, swarm count |

### 3.4 Persistence

- `~/Library/Application Support/blackhand/` (macOS), `%APPDATA%\blackhand\` (Win), `~/.config/blackhand/` (Linux)
- `settings.toml` — user-editable
- `session/` — librqbit's own state (it manages resume data)
- `state.json` — UI-side metadata: categories, custom names, last-sort, column widths

---

## 4. Frontend stack rationale

Recommendation: **Svelte 5 + Vite + TypeScript**.

- Smallest runtime — matches Tauri's "lean binary" promise
- Custom UI work (we're building a non-standard component vocabulary) is easier in Svelte than React; less ceremony, runes give clean reactivity
- Built-in transitions/motion fit the cyberpunk microinteraction language
- If you'd rather stay closer to mainstream, swap for **React 18 + Vite** — Framer Motion is a strong asset there. Plan above doesn't depend on the choice; only `lib/components/` and the store API differ.

State management: lightweight Svelte stores; no Redux/Zustand needed at this scale.

Routing: SvelteKit in SPA mode (`adapter-static` + `ssr: false`), or plain `svelte-spa-router` if SvelteKit feels heavy.

---

## 5. Visual design system — Neon Noir

### 5.1 Color tokens

```
--bg-0           #07070C   /* app background */
--bg-1           #0E0E16   /* surface (rows, cards) */
--bg-2           #181826   /* surface raised (modals, popovers) */
--bg-3           #232336   /* hover / pressed */

--fg-0           #ECECF5   /* primary text */
--fg-1           #B4B4D0   /* secondary text */
--fg-2           #6E6E8C   /* tertiary / placeholder */
--fg-dim         #3D3D55   /* dividers, faint chrome */

--accent-magenta #FF2A6D   /* primary accent — active state, CTA */
--accent-cyan    #08F7FE   /* secondary accent — info, links */
--accent-violet  #B14EFF   /* tertiary — used very sparingly */

--ok             #39FF14   /* completed / seeding healthy */
--warn           #FFB23F   /* stalled / low peers */
--err            #FF3F3F   /* error / unreachable tracker */

--glow-magenta   0 0 12px rgba(255,42,109,0.45)
--glow-cyan      0 0 12px rgba(8,247,254,0.35)
```

### 5.2 Typography

- **Body:** Inter (variable). Fallback: system-ui.
- **Numbers / IDs / paths:** JetBrains Mono. *Always* tabular figures (`font-variant-numeric: tabular-nums`) for any cell that updates.
- **Display (sparingly — page titles, app name only):** Orbitron 600. Letter-spacing 0.08em. Never for body or labels.
- Sizes: 11/13/14/16/20/28 px scale. Default body 13.

### 5.3 Motion + effects

- **Scanlines:** 2% opacity, 2px lines, only on backgrounds and decorative chrome — *never* over body text or numeric tables.
- **Glow:** active/selected items get a 1px magenta border + soft shadow. Hover state: cyan rim.
- **Glitch:** reserved for state transitions only — torrent just added (~250ms desync), torrent just completed (~600ms RGB-split flash). Never decorative, never on text the user is reading.
- **Bracket chrome:** corner-cut decorations on panels (`clip-path`), and `[ ]` / `< >` accent characters around active selectors. Cheap, unmistakably cyberpunk.
- **Progress bars:** segmented (24 cells), magenta fill while downloading, cyan rim when seeding, green when complete. Each cell flips on independently, not a smooth gradient.
- All motion respects `prefers-reduced-motion`.

### 5.4 Clarity guardrails (so it stays readable)

1. Magenta = "look here." If a screen has more than ~3 magenta elements, demote one.
2. Body text is `--fg-0` on `--bg-0/1`. No tinted body text. No green-on-black walls.
3. Tabular numbers, right-aligned in tables. Speeds always show unit (`12.4 MB/s`).
4. Scanlines + glow are the chrome, not the content. Content is high-contrast and quiet.
5. Effects scale with state: idle is 90% calm, action is 10% loud.

### 5.5 Reference layout (main download list)

```
┌─ BLACKHAND ──────────────────────────────────┬─ ▼ ▲ 12.4 / 0.8 MB/s ─┐
│ [ + Add ]  [ ▶ Resume all ]  [ ❚❚ Pause all ]│  3 active · 5 total   │
├──────────────────────────────────────────────┴───────────────────────┤
│ ●  Name                              Size    ↓ Speed   Peers  ETA    │
├──────────────────────────────────────────────────────────────────────┤
│ ●  ubuntu-25.04-desktop-amd64.iso   4.7 GB  12.4 MB/s   8/41  03:14  │
│    ███████████████░░░░░░░░░  62%                                     │
│                                                                      │
│ ◐  debian-12.5.0-netinst.iso        642 MB   3.1 MB/s   5/18  06:02  │
│    █████████░░░░░░░░░░░░░░░  41%                                     │
│                                                                      │
│ ◯  archlinux-2025.05.01.iso         1.1 GB   ❚❚ paused   —    —      │
│    ░░░░░░░░░░░░░░░░░░░░░░░░   0%                                     │
└──────────────────────────────────────────────────────────────────────┘
   selected row → magenta rim + glow; others → bg-1 with cyan hover
```

---

## 6. MVP feature list (v0.1)

- [ ] Add via magnet (paste / `magnet:` protocol handler / drag URL)
- [ ] Add via `.torrent` file (file picker / drag-and-drop / "Open with…")
- [ ] Default download directory (settings); per-add override
- [ ] List view with name, size, %, ↓ speed, ↑ speed, peers, ETA, state
- [ ] Per-torrent: pause, resume, remove (with delete-files confirm), force reannounce
- [ ] Per-torrent detail page: files tab (with per-file priority), peers tab, trackers tab, info tab
- [ ] Per-file priority: skip / normal / high (high = front of sequential queue)
- [ ] Global speed limits (up/down) in settings; per-torrent overrides on detail page
- [ ] Listen port + max-connections settings
- [ ] Resume in-flight torrents on app start
- [ ] System tray: show/hide window, pause-all, resume-all, quit
- [ ] Notification on torrent complete (native, via Tauri notification API)
- [ ] "Open in player" button → calls OS default for the selected file
- [ ] "Reveal in Finder/Explorer" action
- [ ] Single-instance lock (second launch focuses the running window and forwards the magnet)
- [ ] Cyberpunk neon-noir theme as described in §5

---

## 7. Stretch — v0.2 candidate features

Pick 2–3 max for a single milestone; don't blanket-add.

- Categories / tags with a sidebar filter
- Watch folder (auto-import `.torrent` files)
- Bandwidth scheduler (weekly grid)
- Move-on-completion (separate temp + library dirs)
- Embedded peer-list with country flags + client ID parsing
- Command palette (⌘K) — fits the aesthetic perfectly
- Theme variants: "Tokyo" warmer, "Matrix" green
- IP filter / blocklist import

---

## 8. Implementation phases

### Phase 0 — Scaffold (½ day) — ✅ done 2026-05-02
- [x] `pnpm create tauri-app` → svelte-ts template (SvelteKit SPA mode), name `blackhand`
- [x] `git init`, first commit (`4bacadb`)
- [x] Add deps: `librqbit` 8.1, `tokio`, `serde`, `serde_json`, `thiserror`, `anyhow`, `dirs` — used `tauri-plugin-opener` (Tauri-native) instead of the Rust `open` crate
- [x] Set window defaults: 1280×800, min 960×600, dark theme, native decorations, `#07070C` bg
- [x] Boot smoke test: `pnpm tauri dev` compiles in 1m 03s and launches a window with the BLACKHAND wordmark

### Phase 1 — Engine integration (1–2 days) — ✅ done 2026-05-02
- [x] ~~Wrap `librqbit::Session` behind a `SessionHandle` actor~~ — superseded: store `Arc<librqbit::Api>` as Tauri managed state directly (Api is librqbit's purpose-built desktop facade)
- [x] Implement `add_magnet`, `add_torrent_file`, `list_torrents`, `get_torrent`, `pause`, `resume`, `forget`, `delete`, `session_stats`
- [x] Persistence: `dirs::data_dir() / "blackhand"`, `SessionPersistenceConfig::Json`, `fastresume: true` — verified via Cmd-Q + relaunch test
- [x] Stats event loop @ 2 Hz emitting `torrents:snapshot` + `session:stats` (full snapshot, not deltas — fine at hobby scale)
- [x] Manual test: add → progress → pause/resume/forget/delete → quit → relaunch → resume — all confirmed working

### Phase 2 — Core UI (2–3 days) — ✅ done 2026-05-02
- [x] Design tokens (`src/lib/design/tokens.css` + `effects.css`) — palette, type, spacing, glow, scanlines
- [x] Layout shell + global stats pills + bulk actions (Resume-all / Pause-all / + Add / gear)
- [x] Torrent list with live stats — typed events into Svelte 5 rune stores → `TorrentRow` + 32-cell segmented `ProgressBar` + 5×5 pixel-art identicon
- [x] Add-torrent modal (magnet + drag-and-drop or click-to-browse for `.torrent` files)
- [ ] ~~Detail page: files / peers / trackers / info tabs~~ — deferred to Phase 3 (selection scaffolding kept in CSS for easy rewire)
- [x] Settings page (modal) — download dir, listen port range, UPnP/DHT toggles, global up/down KB/s limits with live-apply
- [x] Confirmation dialog for delete (forget stays one-click)
- [x] **Bonus:** typed IPC via `tauri-specta` (DTO layer + auto-generated `src/lib/bindings.ts`)

### Phase 3 — Streaming & system integration (1 day) — ✅ done 2026-05-02
- [x] Sequential mode is librqbit's default — head pieces arrive first for free; verified during Phase 1 streaming via `open` works
- [x] Per-row "Open folder" via `@tauri-apps/plugin-opener::openPath`; per-file "Open in player" lands as part of the detail panel (Apply + click filename in v0.2)
- [x] `magnet://` protocol handler via `tauri-plugin-deep-link` + `bundle.fileAssociations` for `.torrent`
- [x] Single-instance lock via `tauri-plugin-single-instance` (`deep-link` feature) — forwards magnet URLs and brings the running window forward; argv parser handles `.torrent` file paths from "Open with…"
- [x] **Bonus:** native completion notifications via `tauri-plugin-notification` (one banner per finish transition)
- [x] **Bonus:** system tray with Show / Pause-all / Resume-all / Quit menu and left-click visibility toggle
- [x] **Bonus:** per-torrent detail route at `/torrent/[id]` with info card and file include/skip toggles + Apply

### Phase 4 — Polish + ship (1–2 days)
- [ ] Empty states, error toasts, loading skeletons (with subtle scanline)
- [ ] Keyboard shortcuts: ⌘N add, Space pause/resume selected, Del remove
- [ ] About panel; version string
- [ ] Crash/error reporting to local log file
- [ ] Build + sign + notarize (macOS) — the long pole; budget a full extra day
- [ ] Code-sign on Windows (optional for personal use; SmartScreen will warn without)
- [ ] AppImage + `.deb` for Linux
- [ ] First-release tag `v0.1.0`

Total realistic estimate for a focused solo dev: **~7–10 working days** to v0.1, plus signing/distribution friction.

---

## 9. Risks + open questions

- **librqbit API drift.** It's actively developed. Pin the version and review the changelog before each minor bump.
- **No MSE/PE encryption.** Some networks throttle BT. Document this in the README; consider migrating to libtorrent-rasterbar later if it bites.
- **macOS notarization.** First-time setup is fiddly (Apple Developer cert, `notarytool`). Don't underestimate.
- **Windows defender / SmartScreen.** Unsigned binaries scare users. For personal use it's fine; for sharing, budget for a code-signing cert (~$80–200/yr).
- **Open question:** custom window chrome (frameless + draw our own title bar) vs native chrome? Native is simpler and respects platform conventions; custom is more cyberpunk. **Recommend native chrome for v0.1**, revisit later.
- **Open question:** SvelteKit vs plain Svelte+Vite. SvelteKit's adapter-static is fine; plain Vite is lighter. **Recommend SvelteKit for routing convenience**, but it's a 30-minute swap either way.

---

## 10. Done criteria for v0.1

You can:
1. Double-click a `magnet:` link in your browser → BlackHand opens, adds it, starts downloading.
2. See live progress, peers, ETA, with a UI that feels like a cyberpunk terminal but you can read it for an hour without eye strain.
3. Pause, resume, remove. Quit and relaunch — your torrents pick up where they left off.
4. Hit "play" on a downloading video and the OS player opens it, streaming from the head.
5. Hand the binary to a friend on Mac/Windows/Linux and they can run it.

If those five hold, ship it.

---

## Review

### 2026-05-02 — Phase 3 complete
- 7 commits land the OS-integration surface: magnet+single-instance, open-folder, notifications, tray, file association, detail panel.
- macOS dev caveat carried forward: URL scheme + file association registration require a real bundle. `pnpm tauri build --debug` is the one-step way to install for end-to-end testing of the click-magnet-in-browser and "Open with…" flows.
- Trackers + peer list deferred to Phase 4/v0.2 — librqbit's public Api doesn't expose tracker URLs cleanly, and peers as a list (vs the count we already show) is polish.
- Detail-page selection wiring re-enabled; row is now an `<a>` and the action buttons stop propagation so they don't fire navigation.
- Memo: `memories/2026-05-02-phase3-system-integration.md` captures the deep-link/single-instance pairing, the macOS bundle gotcha, and the file-association argv path.

### 2026-05-02 — Phase 2 complete
- 9 commits cover Phase 2: typed IPC foundation → bindings pipeline → tokens → shell → list → identicons → modal → settings → remove confirm → bandwidth-live + alignment + bulk actions.
- Frontend now reads as a real cyberpunk-styled torrent client: pixel-art identicons per torrent, segmented neon progress bars, sticky header with global stats + bulk controls, three modal flows (add / settings / remove confirm).
- One real surprise debugged in the open: librqbit's `Speed.mbps` is *MiB/s* not megabits — display was off by ~8×. Captured in `memories/2026-05-02-phase2-typed-ipc-and-ui.md` so future-Claude doesn't repeat it.
- Detail panel deferred to Phase 3. Selection wiring removed from `+page.svelte` but design-system CSS hooks (.row.selected, magenta rim) remain for trivial re-add.
- Open question carried into Phase 3: do we use a Tauri `tauri-plugin-store` for the settings file, or stay with hand-rolled `serde_json` + `dirs::data_dir`? Current approach is simpler and works; revisit only if cross-process locking becomes relevant.

### 2026-05-02 — Phase 1 complete
- Engine integration end-to-end: add (magnet), see live stats tick at 2 Hz, pause/resume/forget/delete, persistence survives a full quit+relaunch.
- Plan §3.1 SessionHandle actor abandoned in favor of `Arc<librqbit::Api>` — see `memories/2026-05-02-phase1-engine-integration.md` for rationale and config details.
- Frontend currently uses a temporary minimal test UI in `src/routes/+page.svelte`; the real UI is Phase 2 work.
- App icon updated to cyan + magenta per user request (`memories/2026-05-02-icon-design.md`).
- Open question carried into Phase 2: do we generate TS types from Rust (ts-rs / similar) or keep hand-written types? Decide when the command surface stabilizes.

### 2026-05-02 — Phase 0 complete
- Scaffold committed at `4bacadb`. `pnpm check` clean (0/0/0), `cargo check` clean, `tauri dev` builds in ~1m and launches the window.
- Two plan deviations, both captured in `memories/2026-05-02-phase0-scaffold-choices.md`:
  - Used SvelteKit (template default) rather than plain Svelte+Vite — recommended path anyway, no cost.
  - Swapped the Rust `open` crate for `tauri-plugin-opener` (shipped by the template; cleaner story for "open in player").
- Minor scaffold fixups: added `@types/node` (SvelteKit's auto-tsconfig wants it), removed a now-stale `@ts-expect-error` in `vite.config.js`, gitignored `.claude/`.
- Open question deferred to Phase 1: do we use the Tauri commands list from plan §3.3 verbatim, or restructure once we see librqbit's actual API surface? Will decide as part of building the `SessionHandle` actor.
