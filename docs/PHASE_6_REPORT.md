# Phase 6 — FDE learning workspace and project portfolio

**Status:** Complete (pending user approval)

## Goal

Track the Forward Deployed / AI Application Engineering (FDE) track and one coherent
AI-enabled project end-to-end: problem definition, model integration, retrieval/tooling
where justified, evaluation, security, observability, cost/latency, and a technical
walkthrough.

## What was built

### 1. Schema (migration v6)

`SCHEMA_VERSION = 6`. v6 (`fde_projects`) adds two tables on top of the existing
`projects`, `project_milestones` and `project_links` tables (which already existed from
the initial schema):

- `project_tasks` — per-project tasks with `title`, `status` (todo/in_progress/done),
  `due_date`, `notes`.
- `project_notes` — a typed notes journal with `kind`
  (note/evaluation/security/reliability/cost/latency/walkthrough/evidence/doc), `title`,
  `content`.

Plus indexes on `project_id` for milestones, links, tasks and notes. The migration loop
applies every pending version sequentially.

### 2. Backend (`src-tauri/src/projects.rs`)

Typed request/response structs and Tauri commands:

- Projects: `list_projects(track?)`, `get_project(id)`, `upsert_project(req)`.
- Milestones: `upsert_project_milestone(req)` (toggle/update completion).
- Tasks: `upsert_project_task(req)` (add/update, toggle status via upsert).
- Links: `upsert_project_link(req)` (doc/diagram/code/test).
- Notes journal: `upsert_project_note(req)` (typed entries).
- Aggregated detail: `get_project_detail(id)` returns project + milestones + tasks +
  links + notes in one call.
- FDE bootstrap: `seed_fde_project()` idempotently ensures an `E`/FDE project row exists
  and seeds milestones M1–M6 (Problem definition, Basic model integration, Useful
  context/tools, Reliability and evaluation, Production hardening, Technical walkthrough)
  from the roadmap, without overwriting user edits on repeat calls.

### 3. Frontend

- New **Projects** page (`src/features/projects/Projects.tsx`):
  - Track filter (All / Core Java / LLD / HLD / FDE) and project list.
  - Create a project (track, name, slug, status, repo URL, local path).
  - Seed the FDE project with M1–M6 milestones.
  - Project detail: edit fields, milestone list with done toggle, task list with status
    toggle, link list, and a typed notes journal (evaluations, security, reliability,
    cost/latency, walkthroughs, evidence, docs).
- Sidebar now exposes Today, Roadmap, DSA, Workspace, **Projects**, Weekly Review, Settings.
- Browser dev mode returns empty shapes for every projects command so the page renders
  without the Tauri backend.

## Files

- Backend: `src-tauri/src/projects.rs` (new), `src-tauri/src/migrations.rs` (v6),
  `src-tauri/src/main.rs` (module + command registration).
- Frontend: `src/features/projects/Projects.tsx` (new), `src/App.tsx`,
  `src/components/layout/Sidebar.tsx`, `src/components/layout/Shell.tsx`,
  `src/types/index.ts`, `src/services/api.ts`, `src/services/api.test.ts`.

## Tests

- Rust (`cargo test`): 32 passing (adds a `slugify` unit test).
- Frontend (`npx vitest run`): 78 passing. Adds API-shape tests for the projects commands
  (list/get/upsert for project, milestone, task, link, note, detail, seed FDE).
- `npx tsc --noEmit`, `eslint --max-warnings 0`, `npm run build`, `cargo build`: clean.
  `cargo build` still reports pre-existing dead-code/unused-import warnings unrelated to
  this phase.

## How to validate

```bash
cd /Users/sarwesh/Desktop/Projects/DevTrack
npm run tauri dev
```

1. Sidebar → **Projects**.
2. **Seed FDE project** → confirms an `E` track "FDE Capstone Project" is created with
   milestones M1–M6.
3. Create another project (e.g. Core Java), set repo URL / local path / status.
4. Open a project detail: toggle a milestone done, add a task, add a link, add a typed
   note (e.g. `evaluation`), confirm it persists on reload.
5. Filter by track to see only FDE or only Java projects.

## Known limitations / deferred

- No persistent local file watcher: repo/local path is stored as text; "Open path"
  launches the OS handler for the stored path only.
- Milestone due dates and deliverable text are stored but not surfaced in the roadmap
  calendar or weekly plan.
- Cross-topic linking (FDE↔Java↔HLD) is handled by the Phase 5 `topic_links` table; the
  Projects page uses `project_links` (project-scoped URLs) rather than a topic picker.
- Deleting projects/milestones/tasks/notes is not yet supported; records can be updated
  but not removed.
