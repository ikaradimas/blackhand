# Phase 0 scaffold: small decisions made during initial setup

**Date:** 2026-05-02
**Status:** Decided

## Decision
Project scaffolded via `pnpm create tauri-app` with: SvelteKit (in SPA mode via `adapter-static`), TypeScript, Tauri 2, identifier `app.blackhand.client`, window 1280×800 with min 960×600, native decorations, `tauri-plugin-opener` for "open in player." Scaffolded into a temp dir then merged via rsync to preserve the existing `CLAUDE.md`, `tasks/`, `memories/`.

## Why
- **SvelteKit over plain Svelte+Vite:** the official `create-tauri-app` `svelte-ts` template ships SvelteKit with `adapter-static`. Saved a manual setup step; SPA mode (`ssr: false` in `+layout.ts`) keeps things webview-friendly. The implementation plan noted SvelteKit as recommended anyway.
- **`tauri-plugin-opener` over the `open` crate:** the scaffold ships `tauri-plugin-opener` by default. It's the Tauri-native way to open files/URLs and is exposed straight to the frontend — cleaner than calling out to a Rust crate via a custom command. Plan referenced `open`; superseded.
- **`app.blackhand.client` bundle ID:** reverse-DNS-ish, three components (some validators/code-signing tooling want this shape). Easy to change later in `tauri.conf.json` if a real domain shows up.
- **Native decorations:** matches plan §9 recommendation. Frameless custom chrome remains an option for a later milestone.
- **Window 1280×800 / min 960×600:** comfortable default; min size keeps the dense list view legible.
- **Bundle from temp + rsync:** create-tauri-app refuses non-empty target dirs without `-f`, and `-f` was unverified for whether it overwrites. Temp + rsync is provably non-destructive.

## Alternatives considered
- **`-f` flag on create-tauri-app** — would have been one step instead of two. Rejected for the safety argument above; behavior of `-f` against existing files wasn't documented clearly.
- **Plain Svelte + Vite** — slightly lighter than SvelteKit, but choosing it would have meant building the template ourselves. SvelteKit's overhead is negligible for our purposes.
- **Use the Rust `open` crate** — works fine but duplicates what `tauri-plugin-opener` already does, and forces a custom Tauri command for every file-open call. Rejected.

## Consequences
- Frontend lives at `src/routes/` (SvelteKit conventions). New pages = new `+page.svelte` files. Routing comes for free.
- "Open in player" / "Reveal in finder" should be implemented through `tauri-plugin-opener` rather than custom Rust commands.
- The `tauri-plugin-opener` permission model is governed by `src-tauri/capabilities/` — to be reviewed when wiring up file opening.
- @types/node added as devDependency to satisfy SvelteKit's auto-generated tsconfig; vite.config.js had a stale `@ts-expect-error` directive that became invalid once node types loaded — removed.
- `pnpm` 10's safer postinstall behavior ignored esbuild's build script. Currently a non-issue (Vite still runs). Revisit with `pnpm approve-builds` if Vite acts up.
