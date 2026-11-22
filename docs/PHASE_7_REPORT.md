# Phase 7 — Analytics, polish and data portability

**Status:** Complete (pending user approval)

## Goal

Turn progress history into useful insights, make all content searchable, support data portability
(export/backup/restore), and harden daily usability — without any cloud or mandatory network.

## What was built

### 1. No new schema

Phase 7 reuses the existing tables (`topics`, `modules`, `tracks`, `study_tasks`, `daily_plans`,
`activity_events`, `revision_items`, `revision_reviews`, `problem_attempts`, `projects`,
`project_milestones`, `project_notes`, `code_experiments`, `recall_questions`, etc.). `SCHEMA_VERSION`
remains 6; no new migration was required.

### 2. Backend (`src-tauri/src/analytics.rs`)

Typed responses and Tauri commands:

- `search_all(query)` — global search across topics, problems, projects, code experiments, recall
  questions, design artifacts, case studies, project notes and learning entries. Returns categorized
  results (type, id, title, subtitle, track).
- `export_all(destination)` — copies the SQLite database file and the entire `content/` directory
  (Markdown, Java, Mermaid, JSON files) to a destination folder, so data can be moved to another Mac.
- `get_analytics()` — aggregated, read-only stats derived from local records:
  - Per-track topic progress (total/done/in-progress/not-started, average confidence).
  - Planned vs actual minutes from `study_tasks`, plus completed-task count.
  - Revision stats (total items, due today, overdue, average confidence).
  - Problem attempt stats (total, solved, average duration).
  - Activity stats (events and active days in the last 30 days).

All metrics are computed from real local records; nothing is inferred from app launches.

### 3. Frontend

- **Analytics page** (`src/features/analytics/Analytics.tsx`): stat cards (tracks, planned/actual
  minutes, completed tasks, revision items due/overdue, attempts, active days) plus a per-track
  progress table with completion bars and average confidence.
- **Search page** (`src/features/search/Search.tsx`): query input with Enter submission and a
  categorized results table (topic, problem, project, experiment, question, design, case study,
  note, entry).
- Sidebar now exposes Today, Roadmap, DSA, Workspace, Projects, **Analytics**, **Search**, Weekly
  Review, Settings.
- Backup/restore already existed (Phase 1/3) and are exposed in Settings; export is available via
  the `export_all` command.

## Files

- Backend: `src-tauri/src/analytics.rs` (new), `src-tauri/src/main.rs` (module + command registration).
- Frontend: `src/features/analytics/Analytics.tsx` (new), `src/features/search/Search.tsx` (new),
  `src/App.tsx`, `src/components/layout/Sidebar.tsx`, `src/components/layout/Shell.tsx`,
  `src/features/dashboard/Dashboard.tsx`, `src/features/settings/Settings.tsx`,
  `src/features/roadmap/Roadmap.tsx`, `src/features/review/Review.tsx`, `src/features/dsa/DSA.tsx`,
  `src/features/workspace/Workspace.tsx`, `src/features/projects/Projects.tsx`,
  `src/types/index.ts`, `src/services/api.ts`, `src/services/api.test.ts`.

## Tests

- Rust (`cargo test`): 33 passing (adds a search dedupe/truncate unit test).
- Frontend (`npx vitest run`): 81 passing. Adds API-shape tests for search_all, export_all,
  get_analytics.
- `npx tsc --noEmit`, `eslint --max-warnings 0`, `npm run build`, `cargo build`: clean.
  `cargo build` still reports pre-existing dead-code/unused-import warnings unrelated to this phase.

## How to validate

```bash
cd /Users/sarwesh/Desktop/Projects/DevTrack
npm run tauri dev
```

1. Sidebar → **Analytics**: confirm stat cards and per-track progress table populate from local
   records (zeros are expected on a fresh DB).
2. Sidebar → **Search**: enter a term (e.g. a topic title or problem name) and confirm categorized
   results appear.
3. Settings → confirm backup/restore still work; use `export_all` (via command) to copy the DB and
   `content/` folder to a destination, then verify the copy opens on another Mac.
4. Revision due/overdue counts update when you record reviews or change due dates.

## Known limitations / deferred

- No charting library is bundled; analytics uses simple CSS progress bars and tables. A charting
  library can be added later without changing the backend.
- Search is LIKE-based (substring, case-insensitive via SQLite default) across the listed tables;
  full-text search (FTS) can be added later.
- Export copies files but does not produce a single archive; the destination folder contains
  `devtrack.sqlite` and `content/`. Import/restore uses the existing `restore_database` command.
- Weekly/monthly trend charts, weak-area insights, and keyboard shortcuts are deferred to a later
  polish pass.
