# Phase 4: polish, logging, build verified — v0.1 closed

**Date:** 2026-05-02
**Status:** Decided

## Decision
Phase 4 lands the polish layer that turns a working app into something shippable, and verifies the local build pipeline:

- **About modal + clickable version badge** in the header. `app_version` Tauri command reads `env!("CARGO_PKG_VERSION")` so the version is always in sync with `Cargo.toml`. Modal also surfaces an "Open log folder" action.
- **Keyboard shortcuts** (`+layout.svelte` global keydown listener, suppressed when typing in inputs): ⌘N add, ⌘, settings, ⌘W close-to-tray. Esc-to-close already free via native `<dialog>`.
- **Structured logging** via `tauri-plugin-log` 2 + `log` crate. Two targets: Stdout (visible during `tauri dev`) and `LogDir { file_name: "blackhand" }` (per-OS log dir, 5 MB cap, KeepOne rotation). All `eprintln!` call sites converted to `log::warn!`.
- **Local build verified** via `pnpm tauri build --debug` on macOS aarch64. The `.app` builds clean (60 MB) and ships an Info.plist with `CFBundleURLTypes` (magnet://) + `CFBundleDocumentTypes` (.torrent) registrations confirmed by PlistBuddy. The subsequent `bundle_dmg.sh` step flaked on a transient hdiutil error — cosmetic only.

## Why
- Logging was the load-bearing piece for v0.1 — without it, post-ship debugging is "ask the user to copy-paste their console." `tauri-plugin-log` is the canonical answer; the rotation/file-naming knobs are sensible.
- Build verification matters because Tauri 2's deep-link + file-association registrations only happen in a real `.app` bundle. Confirming the Info.plist entries are emitted correctly was the point of the build-verify step, not the .dmg.
- About + version badge + keyboard shortcuts are conventional polish; cheap to add, expected by users.

## Alternatives considered
- **Hand-rolled file logger** instead of `tauri-plugin-log` — about half the lines but reinvents rotation, file-naming, multi-target. Rejected.
- **`tauri::AppMenu` for keyboard shortcuts** (giving us a real macOS menu bar with ⌘N etc.) — more polish but a bigger lift. Skipped for v0.1; can graduate later.
- **Skip the `.dmg` target** — could pin `bundle.targets` to `["app"]` only. Rejected; keeping `.dmg` available is the right default for distribution; the flake here is environmental, not a config issue.
- **Squash all of Phase 4 into one commit** — split into 4 instead (4.1+4.2 polish, 4.3 logging, 4.4 README+build, 4.5 close-out) for cleaner history.

## What's deferred (won't be in v0.1.0)
- **Code-signing + notarization** — requires Apple Developer cert ($99/yr) and a `notarytool` workflow on macOS, plus a code-signing cert ($80–200/yr) for Windows. Document the manual steps in README; user can wire when ready.
- **Auto-updater** — `tauri-plugin-updater` exists; needs a hosted update endpoint + signing setup. Phase 5 (post-v0.1) work.
- **Empty-states polish, error toasts, loading skeletons** — current inline error banners work, just not as pretty. Iterate post-ship.
- **Crash reporter / sentry-style telemetry** — explicitly skipped; this is a local app.

## Consequences
- **Logs land in OS-conventional locations** (`~/Library/Logs/app.blackhand.client/blackhand.log` on macOS) and rotate at 5 MB. The "Open log folder" action in About is the user-visible entry point.
- **Distribution is unsigned.** End users will see Gatekeeper warning on macOS ("BlackHand is from an unidentified developer") and SmartScreen warning on Windows. Documented.
- **The `.app` (or its unbundled binary at `target/{debug,release}/blackhand`) is directly runnable** — `.dmg` is purely a distribution wrapper. If `bundle_dmg.sh` flakes, manually drag-install from `target/.../bundle/macos/BlackHand.app` to `/Applications`.
- **Keyboard shortcut suppression in inputs is a global concern** — if a future feature adds custom key handlers, audit interaction with the layout-level handler.
