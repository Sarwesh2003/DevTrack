# DevTrack

Private, local-first Mac desktop application for six-month SDE interview and FDE (Full-Stack Dev Engineer / AI engineering) preparation.

## What it is

DevTrack is a personal technical learning system, not a general to-do app. It connects a structured 26-week learning roadmap across five tracks — Core Java, DSA (NeetCode 250), LLD (CodeArmy), HLD and FDE — with daily planning, spaced revision, notes, code artifacts and progress tracking.

Core features:
- Dashboard with a prominent "What to Study Today" agenda
- Daily and weekly planning generated from the imported roadmap
- Spaced-revision engine (1, 3, 7, 14, 30, 60–90 day intervals)
- DSA workspace for the official NeetCode 250 sheet (attempts, Java solutions, notes, revision history)
- Core Java, LLD, HLD and FDE topic workspaces with local Markdown/Mermaid/Java files
- Custom learning entries, activity heatmap and streaks
- Local SQLite database, backup/restore, export

## Constraints

- Single-user, local-first. Core features work offline.
- No cloud hosting, Firebase, Supabase, remote database, hosted backend or mandatory accounts.
- User data stays in an app-managed local data directory (`~/Documents/DevTrack/` by default).
- Structured records in SQLite; learning content as normal Markdown, Java, JSON and Mermaid files.
- No secrets in plaintext files or SQLite.

## Current learning checkpoints (do not reset to zero)

- DSA: categories through Trees are revision-only. New learning starts at **Tries**.
- LLD: through Composite is revision-only. New learning starts at **Bridge Pattern**.
- Core Java: starts at A1 of the supplied roadmap (internals/depth focus, not beginner syntax).
- HLD: starts at D1.
- FDE: starts at E1.

## Stack

- Desktop shell: Tauri 2
- Frontend: React + TypeScript + Vite
- UI: Tailwind CSS
- Backend boundary: Tauri commands in Rust
- Database: SQLite with versioned migrations (rusqlite, bundled)
- Tests: Vitest + TypeScript

## Repository structure

```
DevTrack/
├── AGENTS.md
├── README.md
├── package.json
├── src/                 # React + TypeScript frontend
├── src-tauri/           # Rust backend, Tauri config, migrations
├── docs/                # Architecture, schema, import design, ADRs
│   └── roadmap/
│       └── six_month_engineering_roadmap.md  # place the supplied roadmap here
└── tests/
```

## Development phases

## Development phases

Implemented one approved phase at a time from `docs/DEVTRACK_IMPLEMENTATION_PLAN.md`:

0. Repository discovery, environment check, architecture and product documentation (Phase 0 — complete)
1. Desktop shell, SQLite, local file storage, settings and backup foundation (Phase 1 — complete)
2. Curriculum data model and 26-week roadmap import (Phase 2 — complete)
3. Daily planner, weekly goals, custom learning entries, rescheduling and activity tracking (Phase 3 — complete)
4. NeetCode 250 workspace, attempt/code/notes storage and revision engine (Phase 4 — complete)
5. Core Java, LLD and HLD learning workspaces (Phase 5 — complete)
6. FDE learning workspace and project portfolio (Phase 6 — complete)
7. Analytics, global search, backup/export polish and QA (Phase 7 — complete)
8. Optional GitHub/LeetCode integrations

## Getting started

Prerequisites (Mac): Node.js, npm or pnpm, Rust toolchain, Xcode command-line tools.

### Phase 5 (complete)

1. Frontend: `npm install && npm run dev` (dev) or `npm run build` (production)
2. Desktop app: `cd src-tauri && cargo tauri dev`
3. Typecheck: `npx tsc --noEmit`
4. Tests: `npx vitest run` (frontend) and `cargo test` in `src-tauri/` (backend)
5. Lint: `npx eslint . --ext ts,tsx`
6. Backend build: `cargo build` in `src-tauri/` (requires Xcode license acceptance: `sudo xcodebuild -license accept`)

### Next step

All core phases (0–7) are complete. Proceed to Phase 8 (optional GitHub/LeetCode integrations) only if desired.

## Rules (see AGENTS.md)

- Local-first, no cloud. Never invent user progress or data.
- SQL behind repositories; UI components do not issue ad-hoc SQL.
- Tests before UI wiring; update docs and AGENTS.md with each phase.
- Do not jump phases. Never commit secrets.
