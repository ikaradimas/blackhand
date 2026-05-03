# BlackHand — Claude Instructions

## Project context

BlackHand is a cross-platform desktop BitTorrent client. Personal/hobby project.

- **Stack:** Tauri 2 + Rust + Svelte 5 (Vite, TypeScript)
- **Torrent engine:** `librqbit` (embedded as a crate)
- **Platforms:** Windows / macOS / Linux desktop only
- **Aesthetic:** Neon Noir cyberpunk (magenta/cyan on near-black) — clarity over spectacle
- **Implementation plan:** `tasks/todo.md` — read this before proposing architecture or scope changes

Out of scope for v0.1: RSS, multi-tracker search, Tor/I2P, web remote UI, mobile, embedded media player, BT v2, MSE/PE encryption. Don't quietly expand scope.

## Workflow Orchestration

### 1. Plan Node Default
- Enter plan mode for ANY non-trivial task (3+ steps or architectural decisions)
- If something goes sideways, STOP and re-plan immediately – don't keep pushing
- Use plan mode for verification steps, not just building
- Write detailed specs upfront to reduce ambiguity

### 2. Subagent Strategy
- Use subagents liberally to keep main context window clean
- Offload research, exploration, and parallel analysis to subagents
- For complex problems, throw more compute at it via subagents
- One task per subagent for focused execution

### 3. Self-Improvement Loop
- After ANY correction from the user: update `tasks/lessons.md` with the pattern
- Write rules for yourself that prevent the same mistake
- Ruthlessly iterate on these lessons until mistake rate drops
- Review lessons at session start

### 4. Verification Before Done
- Never mark a task complete without proving it works
- Diff behavior between main and your changes when relevant
- Ask yourself: "Would a staff engineer approve this?"
- Run tests, check logs, demonstrate correctness
- For UI: actually run `pnpm tauri dev` and exercise the feature, don't just trust the type checker

### 5. Demand Elegance (Balanced)
- For non-trivial changes: pause and ask "is there a more elegant way?"
- If a fix feels hacky: "Knowing everything I know now, implement the elegant solution"
- Skip this for simple, obvious fixes – don't over-engineer
- Challenge your own work before presenting it

### 6. Autonomous Bug Fixing
- When given a bug report: just fix it. Don't ask for hand-holding
- Point at logs, errors, failing tests – then resolve them
- Zero context switching required from the user
- Go fix failing CI tests without being told how

## Task Management

1. **Plan First:** Write plan to `tasks/todo.md` with checkable items
2. **Verify Plan:** Check in before starting implementation
3. **Track Progress:** Mark items complete as you go
4. **Explain Changes:** High-level summary at each step
5. **Document Results:** Add review section to `tasks/todo.md`
6. **Capture Lessons:** Update `tasks/lessons.md` after corrections

## Decision Memories

After every **major decision**, write a short memory file to `memories/` in this repo. These are project-internal Architecture Decision Records — they answer "why is it like this?" for anyone (including future-Claude) reading the code six months later.

**A "major decision" is one of these:**
- A library / framework / engine choice (or swap)
- An architectural choice with non-trivial alternatives (sync vs async, in-proc vs daemon, IPC shape)
- A scope decision (adding or removing a feature from a milestone)
- A visual-system change (palette, typography scale, motion language)
- A public contract change (Tauri command signatures, event payloads, persisted file formats)
- A trade-off resolution where alternatives were seriously weighed

**NOT a major decision** (don't write memos for these):
- Bug fixes, refactors that don't change interfaces, dependency bumps, typo corrections, icon swaps, copy edits

### File format

One file per decision. Filename: `YYYY-MM-DD-short-slug.md`, lowercase, hyphenated. Chronological filenames make `ls memories/` a usable timeline.

```markdown
# [Decision title]

**Date:** YYYY-MM-DD
**Status:** Decided | Superseded by [filename]

## Decision
One or two sentences. State what we're doing, not what we're considering.

## Why
2–4 lines. Lead with the constraint or goal that forced the choice.

## Alternatives considered
- **Option A** — rejected because [concrete reason]
- **Option B** — rejected because [concrete reason]

## Consequences
What this enables. What it locks us out of. What needs to be revisited if circumstances change.
```

### Rules

- Write the memory **the moment the decision is made**, not at the end of the session — it's how we avoid retroactive rationalization.
- If a later decision overturns an earlier one, **don't delete the old file.** Set its `Status:` to `Superseded by YYYY-MM-DD-new-decision.md` and write a fresh file. The history matters.
- Reference the relevant `tasks/todo.md` section if applicable.
- Keep them short. A memory is a sticky note, not a design doc.
- Read recent memories at session start when picking up context — they tell you what's load-bearing.

## Core Principles

- **Simplicity First:** Make every change as simple as possible. Impact minimal code.
- **No Laziness:** Find root causes. No temporary fixes. Senior developer standards.
- **Minimal Impact:** Changes should only touch what's necessary. Avoid introducing bugs.
- **Clarity over spectacle:** When the cyberpunk aesthetic and readability conflict, readability wins. See `tasks/todo.md` §5.4.

## Committing changes

After completing any task that modifies files, commit all changed files before finishing.
Do not leave work uncommitted at the end of a session.

**Authorship:** Every commit must be authored and committed by Claude. Always set both
the author and committer identity by prefixing the `git commit` call with the environment
variables below. Never rely on the ambient git config for identity.

```bash
GIT_AUTHOR_NAME="Claude" \
GIT_AUTHOR_EMAIL="noreply@anthropic.com" \
GIT_COMMITTER_NAME="Claude" \
GIT_COMMITTER_EMAIL="noreply@anthropic.com" \
git commit -m "..."
```

**Version bump per commit.** Every commit must bump the version. Update **all three**
locations together — they must stay in lockstep:

- `package.json` → `"version"`
- `src-tauri/Cargo.toml` → `version`
- `src-tauri/tauri.conf.json` → `"version"`

The `commands.appVersion()` Tauri command reads `CARGO_PKG_VERSION` at compile time, so
the AppHeader badge and About modal pick up the new version automatically — no UI edits
needed.

**Which level to bump (semver):**

- **Patch** (`0.2.1` → `0.2.2`) — default for every commit: bug fixes, refactors,
  small UX/UI changes, doc updates, test additions, dependency bumps.
- **Minor** (`0.2.x` → `0.3.0`) — when shipping a new user-facing feature
  (e.g. tray popup, status bar, hide-to-tray). Resets patch to 0.
- **Major** (`0.x.y` → `1.0.0`) — only on a deliberate stability promise or
  breaking change to a public contract (Tauri command signature, persisted file
  format, etc.). The user will normally make this call.

If a single commit mixes a feature and a fix, use the **higher** of the two bumps. If
unsure between patch and minor, bump patch — minor bumps should feel like a small
release note, not just "I added some code today."
