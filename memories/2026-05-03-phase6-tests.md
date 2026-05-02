# Phase 6: post-ship tests for backend + frontend

**Date:** 2026-05-03
**Status:** Decided

## Decision
Established test infrastructure for both halves of the codebase, scoped to high-ROI pure logic:
- **Backend:** Rust `#[cfg(test)]` unit tests in `categories.rs`, `settings.rs`, `types.rs`. 24 new tests.
- **Frontend:** Vitest + `@testing-library/svelte` + jsdom. 55 tests across pure logic (`api.ts`, `ui` store, `categories` store) and components (`PixelMark`, `ProgressBar`, `Modal`, `AddTorrentModal`, `RemoveConfirmModal`).

## Why
- Two recent regressions (`Speed.mbps` MiB/s confusion, the 10× kbps→bps bug) were caught manually. A test for `mibps_to_bps` and `kbps_to_nz_bps` would have flagged either pre-merge.
- Categories logic and the inline-error UX are the parts of v0.2 most likely to drift quietly. Lock them in now.
- Out of scope: anything requiring a live `librqbit::Api` or full Tauri runtime — too heavy for the marginal coverage.

## Architecture / decisions worth remembering

### Backend testability
- `CategoryStore` is a thin lock+persistence wrapper around `CategoryData`. Pure logic (`set_category`, `category_for`, `list_with_counts`) lives on `CategoryData` and is unit-tested directly without filesystem.
- `paths.rs` is hardcoded to `dirs::data_dir()`. Anything that goes through it (settings load/save, CategoryStore::load) is **not** unit-testable without polluting the user's data dir. We tested the pure helpers and skipped the I/O wrappers — sufficient.
- `TorrentSnapshot::from_response` takes `Option<&CategoryStore>`; testing it would mean constructing a `librqbit::api::TorrentListResponse` (lots of nested types). Skipped — category threading is implicitly covered by `CategoryData` tests + the trivial wrapper.

### Frontend test infrastructure
- `vitest.config.ts` uses `svelte()` (not `sveltekit()`) plus `svelteTesting()` from `@testing-library/svelte/vite`. The svelte plugin compiles `.svelte` and `.svelte.ts` files; the testing-library plugin sets up the `browser` resolve condition and noExternal so runes work in jsdom.
- `$lib` alias is configured manually in `resolve.alias` (SvelteKit normally provides this; standalone vitest doesn't pick it up from svelte.config.js).
- jsdom 29 does **not** implement `HTMLDialogElement.showModal` / `close`. `src/test-setup.ts` polyfills both to toggle the `open` attribute. Without this, every Modal-backed component test fails with `showModal is not a function`.
- Test file naming: vitest's `include` is `src/**/*.{test,spec}.{ts,js}`. **Don't** use `*.test.svelte.ts` — vitest won't match it. The test files don't need the `.svelte.ts` extension themselves (they don't use runes); the runed modules they import do.

### Bindings mocking
- `vi.mock("$lib/bindings", () => ({ commands: { ... } }))` at the top of each component/store test file. The mock returns the same `{ status: "ok" | "error" }` discriminated union the real Tauri-Specta bindings produce, so `$lib/api::unwrap` works against it.
- We mock per-test by reassigning `mockFn.mockResolvedValueOnce(...)` in each test, with `mockReset()` in `beforeEach` to keep tests independent.

### Singleton store testability
- `ui` and `toasts` and `categories` are exported as singleton instances, not constructors. Tests reset state in `beforeEach` (`ui.cancelRemove()`, `toasts.list = []`, `categories.list = []` etc.) rather than constructing fresh instances. Works because vitest runs each test file in a fresh module scope by default.

## Out of scope (and reasons)
- **`commands.rs`, `session.rs`, `stats.rs` Rust tests** — would need a real `librqbit::Api` harness with a live session. The cost-of-test-setup-vs-bug-caught ratio is wrong.
- **`SettingsModal.svelte` component test** — heavy form (multiple sections, conditional banners). Defer until a bug warrants it.
- **Route tests (`/+page.svelte`, `/torrent/[id]/+page.svelte`)** — would need to mock the full Tauri event stream + commands surface. Skip.
- **E2E (`tauri-driver`, Playwright)** — heavyweight infra; not worth it for a personal project at v0.2 scope.

## Consequences
- `pnpm test` (watch) and `pnpm test:run` (one-shot) are now part of the workflow alongside `pnpm check` and `cargo test`. Run all three before declaring a change done.
- The test-setup file (`src/test-setup.ts`) is the place to add any further test-environment polyfills (e.g. `IntersectionObserver`, `matchMedia`) if future component tests need them.
- Future component tests that use `<dialog>` work transparently thanks to the polyfill — no per-test boilerplate.
- The CategoryStore refactor preserves its public API exactly; existing call sites are unchanged.
