<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { page } from "$app/state";
  import { openPath, revealItemInDir } from "@tauri-apps/plugin-opener";
  import { join } from "@tauri-apps/api/path";

  import { commands, type TorrentDetail, type TorrentFile } from "$lib/bindings";
  import { unwrap } from "$lib/api";
  import ProgressBar from "$lib/components/ProgressBar.svelte";
  import PixelMark from "$lib/components/PixelMark.svelte";
  import { categories } from "$lib/stores/categories.svelte";
  import { toasts } from "$lib/stores/toasts.svelte";
  import { torrents } from "$lib/stores/torrents.svelte";

  let id = $derived(Number(page.params.id));
  let detail = $state<TorrentDetail | null>(null);
  let files = $state<TorrentFile[]>([]);
  let trackers = $state<string[]>([]);
  let loadFailed = $state(false);
  let savingFiles = $state(false);
  let categoryInput = $state("");
  let savingCategory = $state(false);
  let timer: ReturnType<typeof setInterval> | null = null;

  // Live snapshot of the torrent (gives us the current category).
  const summary = $derived(torrents.list.find((t) => t.id === id));

  // Seed the category input from the torrent's current category.
  let lastSyncedCategory: string | null = $state(null);
  $effect(() => {
    const current = summary?.category ?? "";
    if (current !== lastSyncedCategory) {
      categoryInput = current;
      lastSyncedCategory = current;
    }
  });

  const categoryDirty = $derived(
    detail !== null && categoryInput.trim() !== (summary?.category ?? "").trim(),
  );

  async function refresh() {
    try {
      const d = await unwrap(commands.getTorrentDetail(id));
      detail = d;
      // Only seed `files` once — keep user toggles intact across refreshes.
      if (files.length !== d.files.length) {
        files = d.files.map((f) => ({ ...f }));
      } else {
        // Update length/name from server without clobbering `included` choices.
        for (let i = 0; i < d.files.length; i++) {
          files[i] = { ...files[i], name: d.files[i].name, length: d.files[i].length };
        }
      }
    } catch (e) {
      // Only toast once — polling errors otherwise spam.
      if (!loadFailed) {
        loadFailed = true;
        toasts.error(`couldn't load torrent: ${e}`);
      }
    }
  }

  onMount(() => {
    refresh();
    loadTrackers();
    timer = setInterval(refresh, 1000);
  });

  async function loadTrackers() {
    try {
      trackers = await unwrap(commands.getTrackers(id));
    } catch {
      // tolerable; just leaves the list empty
    }
  }

  async function openFile(f: TorrentFile) {
    if (!detail) return;
    try {
      const full = await join(detail.output_folder, f.name);
      await openPath(full);
    } catch (e) {
      toasts.error(`couldn't open file: ${e}`);
    }
  }

  async function revealFile(f: TorrentFile) {
    if (!detail) return;
    try {
      const full = await join(detail.output_folder, f.name);
      await revealItemInDir(full);
    } catch (e) {
      toasts.error(`couldn't reveal file: ${e}`);
    }
  }

  async function applyCategory() {
    if (!detail) return;
    savingCategory = true;
    try {
      const trimmed = categoryInput.trim();
      await categories.assign(detail.info_hash, trimmed === "" ? null : trimmed);
      toasts.ok(trimmed === "" ? "category cleared" : `category set to "${trimmed}"`);
    } catch (e) {
      toasts.error(`couldn't set category: ${e}`);
    } finally {
      savingCategory = false;
    }
  }
  onDestroy(() => {
    if (timer) clearInterval(timer);
  });

  function fmtBytes(n: number): string {
    if (!Number.isFinite(n)) return "—";
    const u = ["B", "KB", "MB", "GB", "TB"];
    let i = 0;
    let v = n;
    while (v >= 1024 && i < u.length - 1) {
      v /= 1024;
      i++;
    }
    return `${v.toFixed(v < 10 ? 2 : 1)} ${u[i]}`;
  }

  const filesDirty = $derived.by(() => {
    if (!detail) return false;
    if (files.length !== detail.files.length) return true;
    return files.some((f, i) => f.included !== detail!.files[i].included);
  });

  async function saveFiles() {
    if (!detail || !filesDirty) return;
    savingFiles = true;
    try {
      const idxs = files.filter((f) => f.included).map((f) => f.idx);
      await unwrap(commands.setOnlyFiles(detail.id, idxs));
      await refresh();
      toasts.ok("file selection applied");
    } catch (e) {
      toasts.error(`apply failed: ${e}`);
    } finally {
      savingFiles = false;
    }
  }

  function selectAll(value: boolean) {
    files = files.map((f) => ({ ...f, included: value }));
  }

  async function reveal() {
    if (!detail) return;
    try {
      await openPath(detail.output_folder);
    } catch {}
  }
</script>

{#if !detail}
  <p class="loading">{loadFailed ? "couldn't load torrent" : "loading…"}</p>
{:else}
  <header class="hd">
    <div class="hd-left">
      <span class="dot state-{detail.state}" class:done={detail.finished}>
        <PixelMark hash={detail.info_hash} />
      </span>
      <div class="title-stack">
        <a class="back" href="/">← back</a>
        <h1 class="name">{detail.name ?? detail.info_hash}</h1>
      </div>
    </div>
    <div class="hd-right">
      <button type="button" onclick={reveal}>Open folder</button>
    </div>
  </header>

  <section class="overview">
    <div class="bar-wrap">
      <ProgressBar
        pct={detail.progress_pct}
        finished={detail.finished}
        paused={detail.state === "paused"}
      />
    </div>
    <div class="overview-row tnum">
      <span>{detail.progress_pct.toFixed(1)}%</span>
      <span>{fmtBytes(detail.progress_bytes)} / {fmtBytes(detail.total_bytes)}</span>
      <span>uploaded {fmtBytes(detail.uploaded_bytes)}</span>
      <span class="state state-{detail.state}">{detail.finished ? "finished" : detail.state}</span>
    </div>
    {#if detail.error}
      <p class="err tnum">{detail.error}</p>
    {/if}
  </section>

  <section class="card">
    <header class="card-hd">
      <h2>Info</h2>
    </header>
    <dl class="kv">
      <dt>Hash</dt>
      <dd class="tnum mono">{detail.info_hash}</dd>
      <dt>Output folder</dt>
      <dd class="tnum mono">{detail.output_folder}</dd>
      <dt>Total size</dt>
      <dd class="tnum">{fmtBytes(detail.total_bytes)}</dd>
      <dt>Files</dt>
      <dd class="tnum">{files.length}</dd>
      <dt>Category</dt>
      <dd class="cat-row">
        <input
          type="text"
          list="categories-options"
          placeholder="(none)"
          bind:value={categoryInput}
          disabled={savingCategory}
          onkeydown={(e) => {
            if (e.key === "Enter" && categoryDirty) {
              e.preventDefault();
              applyCategory();
            }
          }}
        />
        <datalist id="categories-options">
          {#each categories.list as c (c.name)}
            <option value={c.name}></option>
          {/each}
        </datalist>
        <button
          type="button"
          class="cat-apply"
          onclick={applyCategory}
          disabled={!categoryDirty || savingCategory}
        >
          {savingCategory ? "…" : "Apply"}
        </button>
      </dd>
    </dl>
  </section>

  <section class="card">
    <header class="card-hd">
      <h2>Files</h2>
      <div class="card-actions">
        <button type="button" onclick={() => selectAll(true)} disabled={savingFiles}>All</button>
        <button type="button" onclick={() => selectAll(false)} disabled={savingFiles}>None</button>
        <button
          type="button"
          class="primary"
          onclick={saveFiles}
          disabled={!filesDirty || savingFiles}
        >
          {savingFiles ? "saving…" : "Apply"}
        </button>
      </div>
    </header>
    <div class="files">
      <header class="file-headers tnum">
        <span></span>
        <span class="hd-name">PATH</span>
        <span class="hd-num">SIZE</span>
        <span></span>
      </header>
      {#each files as f (f.idx)}
        <div class="file-row">
          <label class="file-pick">
            <input type="checkbox" bind:checked={f.included} disabled={savingFiles} />
          </label>
          <span class="file-name">{f.name}</span>
          <span class="tnum file-size">{fmtBytes(f.length)}</span>
          <span class="file-actions">
            <button
              type="button"
              class="action"
              title="Open in default app"
              aria-label="Open in default app"
              onclick={() => openFile(f)}
            >▷</button>
            <button
              type="button"
              class="action"
              title="Reveal in file manager"
              aria-label="Reveal in file manager"
              onclick={() => revealFile(f)}
            >
              <svg viewBox="0 0 16 16" width="11" height="11" fill="currentColor" aria-hidden="true">
                <path d="M1.5 3a.5.5 0 0 1 .5-.5h4l1.5 1.5h6.5a.5.5 0 0 1 .5.5v8.5a.5.5 0 0 1-.5.5H2a.5.5 0 0 1-.5-.5V3Zm1 .5v9h11v-7H7.293L5.793 4H2.5Z"/>
              </svg>
            </button>
          </span>
        </div>
      {/each}
    </div>
  </section>

  {#if trackers.length > 0}
    <section class="card">
      <header class="card-hd">
        <h2>Trackers</h2>
        <span class="dim tnum">{trackers.length}</span>
      </header>
      <ul class="tracker-list">
        {#each trackers as url (url)}
          <li class="tracker tnum">{url}</li>
        {/each}
      </ul>
    </section>
  {/if}
{/if}

<style>
  .loading {
    text-align: center;
    color: var(--fg-2);
    padding: var(--sp-7) 0;
  }

  .err {
    color: var(--err);
    font-size: var(--fs-xs);
    background: rgba(255, 63, 63, 0.08);
    border: 1px solid rgba(255, 63, 63, 0.3);
    padding: var(--sp-2) var(--sp-3);
    border-radius: var(--radius-md);
    margin: 0 0 var(--sp-3);
  }

  .hd {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: var(--sp-4);
    margin-bottom: var(--sp-4);
  }
  .hd-left {
    display: flex;
    gap: var(--sp-3);
    align-items: flex-start;
    min-width: 0;
  }
  .dot {
    display: inline-flex;
    width: 28px;
    height: 28px;
    align-items: center;
    justify-content: center;
    color: var(--fg-2);
    flex-shrink: 0;
  }
  .dot.state-live { color: var(--accent-cyan); }
  .dot.state-paused { color: var(--fg-2); }
  .dot.state-initializing { color: var(--warn); }
  .dot.state-error { color: var(--err); }
  .dot.done { color: var(--ok); }

  .title-stack {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }
  .back {
    color: var(--fg-2);
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
    letter-spacing: var(--tracking-wider);
    text-decoration: none;
  }
  .back:hover {
    color: var(--accent-cyan);
  }
  .name {
    margin: 0;
    font-size: var(--fs-xl);
    font-weight: 600;
    color: var(--fg-0);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  button {
    background: var(--bg-2);
    border: 1px solid var(--bg-3);
    color: var(--fg-0);
    border-radius: var(--radius-md);
    padding: var(--sp-2) var(--sp-3);
    font-size: var(--fs-sm);
    font-family: inherit;
    cursor: pointer;
    transition: border-color var(--motion-fast), color var(--motion-fast);
  }
  button:hover:not(:disabled) {
    border-color: var(--accent-cyan);
    color: var(--accent-cyan);
  }
  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  button.primary {
    background: var(--accent-magenta);
    border-color: var(--accent-magenta);
    color: var(--fg-0);
    box-shadow: var(--glow-magenta-sm);
  }
  button.primary:hover:not(:disabled) {
    background: var(--accent-magenta-hover);
    border-color: var(--accent-magenta-hover);
    color: var(--fg-0);
  }

  .overview {
    margin-bottom: var(--sp-4);
  }
  .bar-wrap {
    margin-bottom: var(--sp-2);
  }
  .overview-row {
    display: flex;
    flex-wrap: wrap;
    gap: var(--sp-2) var(--sp-4);
    align-items: center;
    font-size: var(--fs-xs);
    color: var(--fg-1);
  }
  .state {
    text-transform: uppercase;
    letter-spacing: var(--tracking-wider);
    font-family: var(--font-mono);
    margin-left: auto;
  }
  .state-live { color: var(--accent-cyan); }
  .state-paused { color: var(--fg-2); }
  .state-initializing { color: var(--warn); }
  .state-error { color: var(--err); }

  .card {
    background: var(--bg-1);
    border: 1px solid var(--bg-3);
    border-radius: var(--radius-lg);
    padding: var(--sp-3) var(--sp-4);
    margin-bottom: var(--sp-3);
  }
  .card-hd {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--sp-3);
    margin-bottom: var(--sp-3);
  }
  .card-hd h2 {
    margin: 0;
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
    font-weight: 500;
    letter-spacing: var(--tracking-widest);
    text-transform: uppercase;
    color: var(--accent-magenta);
  }
  .card-actions {
    display: flex;
    gap: var(--sp-1);
  }
  .card-actions button {
    padding: 4px var(--sp-2);
    font-size: var(--fs-xs);
  }

  .kv {
    display: grid;
    grid-template-columns: 140px 1fr;
    gap: var(--sp-2) var(--sp-4);
    margin: 0;
  }
  .kv dt {
    color: var(--fg-2);
    font-family: var(--font-mono);
    font-size: var(--fs-xs);
    letter-spacing: var(--tracking-wider);
    text-transform: uppercase;
  }
  .kv dd {
    color: var(--fg-0);
    font-size: var(--fs-sm);
    margin: 0;
    overflow-wrap: anywhere;
  }
  .kv .mono {
    font-family: var(--font-mono);
    color: var(--fg-1);
  }

  .cat-row {
    display: flex;
    gap: var(--sp-2);
    align-items: center;
  }
  .cat-row input[type="text"] {
    flex: 1;
    padding: 4px var(--sp-2);
    background: var(--bg-1);
    border: 1px solid var(--bg-3);
    color: var(--fg-0);
    border-radius: var(--radius-sm);
    font-family: var(--font-mono);
    font-size: var(--fs-sm);
    transition: border-color var(--motion-fast);
  }
  .cat-row input[type="text"]:focus {
    outline: none;
    border-color: var(--accent-cyan);
    box-shadow: 0 0 0 1px var(--accent-cyan);
  }
  .cat-apply {
    background: var(--bg-2);
    border: 1px solid var(--bg-3);
    color: var(--fg-0);
    border-radius: var(--radius-sm);
    padding: 4px var(--sp-3);
    font-family: inherit;
    font-size: var(--fs-xs);
    cursor: pointer;
    transition: border-color var(--motion-fast), color var(--motion-fast);
  }
  .cat-apply:hover:not(:disabled) {
    border-color: var(--accent-magenta);
    color: var(--accent-magenta);
  }
  .cat-apply:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .files {
    max-height: 50vh;
    overflow-y: auto;
  }
  .file-headers {
    display: grid;
    grid-template-columns: 22px minmax(0, 1fr) 100px 64px;
    gap: var(--sp-3);
    padding: 0 var(--sp-2) var(--sp-2);
    font-size: var(--fs-xs);
    color: var(--fg-2);
    letter-spacing: var(--tracking-wider);
    border-bottom: 1px solid var(--bg-3);
  }
  .hd-name { text-align: left; }
  .hd-num { text-align: right; }

  .file-row {
    display: grid;
    grid-template-columns: 22px minmax(0, 1fr) 100px 64px;
    gap: var(--sp-3);
    align-items: center;
    padding: 6px var(--sp-2);
    border-bottom: 1px solid rgba(35, 35, 54, 0.4);
    font-size: var(--fs-sm);
  }
  .file-row:hover {
    background: rgba(8, 247, 254, 0.04);
  }
  .file-pick {
    display: flex;
    align-items: center;
    cursor: pointer;
  }
  .file-pick input[type="checkbox"] {
    accent-color: var(--accent-magenta);
    cursor: pointer;
    margin: 0;
  }
  .file-name {
    color: var(--fg-0);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-family: var(--font-mono);
  }
  .file-size {
    color: var(--fg-1);
    text-align: right;
    font-size: var(--fs-xs);
  }
  .file-actions {
    display: flex;
    gap: 2px;
    justify-content: flex-end;
  }
  .action {
    background: transparent;
    border: 1px solid var(--bg-3);
    color: var(--fg-1);
    border-radius: var(--radius-sm);
    width: 26px;
    height: 22px;
    padding: 0;
    cursor: pointer;
    font-family: inherit;
    font-size: var(--fs-xs);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    transition: border-color var(--motion-fast), color var(--motion-fast);
  }
  .action:hover {
    border-color: var(--accent-cyan);
    color: var(--accent-cyan);
  }

  .tracker-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: var(--sp-1);
    max-height: 30vh;
    overflow-y: auto;
  }
  .tracker {
    color: var(--fg-1);
    font-size: var(--fs-xs);
    padding: 4px var(--sp-2);
    background: var(--bg-1);
    border: 1px solid rgba(35, 35, 54, 0.6);
    border-radius: var(--radius-sm);
    overflow-wrap: anywhere;
  }
</style>
