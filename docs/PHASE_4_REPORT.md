# Phase 4 — DSA / NeetCode 250 workspace and revision engine

**Status:** Complete (pending user approval)

## Goal

Deliver the most-used technical learning workflow: import the official NeetCode 250
dataset, browse/search/filter/favorite problems, record multiple attempts with complexity
and interview notes, keep a local Java solution per problem, and schedule spaced revision
with an explicit baseline for previously covered categories.

## What was built

### 1. Schema (migrations v2 + v3 + v4)

`SCHEMA_VERSION = 4`. v2 adds `favorites` and `solutions` on top of the existing
`problems`, `problem_attempts`, `revision_items` and `revision_reviews` tables. v3 adds
`daily_plans.manual_override` so regeneration can refresh auto-generated capacity from current
settings without overwriting explicit user overrides. v4 adds `problem_attempts.study_task_id`
so a completed DSA task's auto-created solved attempt can be removed on undo without touching
attempts the user logged manually. The migration loop applies every pending version
sequentially.

### 2. NeetCode import (`src-tauri/src/dsa.rs`)

- Quote-aware CSV parser (handles the quoted `"Pow(x, n)"` title) with validation.
- `preview_neetcode` returns row counts, previously-covered vs new-learning split and a
  per-category breakdown before anything is written.
- `import_neetcode` inserts deterministically and idempotently (`INSERT OR IGNORE` on the
  stable external problem id), preserving existing progress on re-import.
- The same import seeds the roadmap **DSA track** (track B) with one module and one category
  topic per official NeetCode category (B1–B18), derived from the dataset. B1–B7 (through
  Trees) are `Needs revision`; B8+ (Tries onward) are `Not started`, so the roadmap tab shows
  the category sequence and the planner's next new DSA topic is Tries. Seeding is idempotent:
  existing topics keep their status/progress.
- Coverage boundary: categories with `category_order <= 7` (through Trees) are
  revision-only; new learning starts at Tries (`>= 8`).

### 3. Question workspace

- `get_problems` with category / difficulty / free-text / coverage filters, plus per-problem
  favorite, attempt count and solved flags.
- `toggle_favorite` for the local favorites list.
- `record_attempt` stores date, outcome, duration, notes, complexity, edge cases and
  interview explanation; the first `solved` attempt seeds a revision item due next day.
- `list_attempts` returns full attempt history for a problem.
- `save_solution` / `get_solution` write and read a per-problem Java file under the
  app-managed `content/solutions/` directory and mirror it in the `solutions` table.

### 4. Revision engine

- Ported interval ladder (1, 3, 7, 14, 30, 60 days) with `correct_clear` / `correct_slow` /
  `partial` / `incorrect` outcomes; `record_review` advances the schedule, records a
  `revision_reviews` row and updates the item.
- `get_revision_queue` lists items due today or earlier.
- `start_revision_baseline` establishes a one-time baseline for previously covered problems
  with no attempts and no existing revision item — it creates fresh due dates and does not
  pretend to recover original history. Ids are generated per row and `source_id` is the
  internal problem id, consistent with attempt-seeded items.

### 5. DSA progress

- `get_dsa_progress` aggregates per-category total / attempted / solved with a
  previously-covered marker, plus overall totals.

### 6. Frontend

- New **DSA** page (`src/features/dsa/DSA.tsx`) with three tabs:
  - **Questions:** search, category/difficulty/coverage filters, favorite toggles and a
    detail view with attempt history, an attempt form (date, outcome, duration, complexity,
    edge cases, notes, interview explanation) and a Java solution editor that loads/saves the
    local file.
  - **Revision:** one-time "Start revision baseline now" action and a due queue with
    recall-result, confidence and note capture.
  - **Progress:** overall totals and per-category solved/attempted bars.
- CSV import flow: pick the file, preview the split, then confirm import.
- Sidebar now exposes Today, Roadmap, DSA, Weekly Review, Settings.
- Browser dev mode shows empty state and disables desktop-only actions.

## Files

- Backend: `src-tauri/src/dsa.rs` (new), `src-tauri/src/migrations.rs` (v2),
  `src-tauri/src/main.rs` (module + command registration).
- Frontend: `src/features/dsa/DSA.tsx` (new), `src/App.tsx`,
  `src/components/layout/Sidebar.tsx`, `src/components/layout/Shell.tsx`,
  `src/features/dashboard/Dashboard.tsx`, `src/features/settings/Settings.tsx`,
  `src/features/roadmap/Roadmap.tsx`, `src/features/review/Review.tsx`,
  `src/types/index.ts`, `src/services/api.ts`, `src/services/api.test.ts`.

## Tests

- Rust (`cargo test`): 26 passing. Adds CSV parsing (quoted fields, malformed rejection),
  coverage boundary, the revision interval ladder, a DB-backed import test that runs the
  real `problems` insert against the migrated schema (guards against column mismatches),
  a test that seeding DSA category topics is correct and idempotent, and a test that daily
  plan regeneration refreshes auto capacity while preserving manual overrides.
- Frontend (`npx vitest run`): 58 passing. Adds API-shape tests for every DSA command
  (preview/import, problems, attempts, solutions, reviews, queue, baseline, progress,
  favorite toggle, external URL open).
- `npx tsc --noEmit`, `eslint --max-warnings 0`, `npm run build`, `cargo build`: clean.
  `cargo build` still reports pre-existing dead-code/unused-import warnings unrelated to
  this phase.

## How to validate

```bash
cd /Users/sarwesh/Desktop/Projects/DevTrack
npm run tauri dev
```

1. DSA → **Import NeetCode CSV** → choose `/Users/sarwesh/Downloads/NEETCODE_250.csv`.
2. Confirm the preview shows 250 rows split into revision-only (through Trees) and new
   learning (Tries onward), then **Confirm import**.
3. Filter to a category, favorite a problem, open it and record an attempt with complexity
   and edge cases; save a Java solution and reload to confirm it persists.
4. Revision → **Start revision baseline now**; the previously covered unsolved problems
   become due items. Record a review and confirm the next due date advances.
5. Progress → per-category solved/attempted bars update.

## Post-review fixes

- Plan regeneration refreshes per-day capacity from the latest settings for auto-generated
  plans (manual overrides preserved via `daily_plans.manual_override`). The dashboard "Generate
  this week's plan" button now forces regeneration so planned tasks rebuild from current
  settings, and surviving completed/custom tasks are counted against the day's capacity.
- DSA "Open problem" opens in the system browser through a validated `open_external_url`
  command (http/https only) instead of a raw link the webview blocks.
- `generate_week` warnings are surfaced in the dashboard message.
- "Replan missed" now also reschedules `skipped` tasks (treated as deferred) and resets every
  moved task to `planned`; previously only `planned` overdue tasks were selected, so a skipped
  goal was silently ignored. Replanning logic is extracted into a testable helper.
- Past work is now visible: the Dashboard has prev/next day navigation over `get_agenda`, an
  overdue list (`list_overdue_tasks`) of pre-today planned/skipped tasks with Done / Move to
  today / Skip / Undo skip, and Weekly Review renders the week's task list grouped by day
  (showing planned tasks from past days as "missed").
- Fixed topic ordering: keys were sorted as raw text, so `A10` came before `A2`; planner and
  roadmap now sort naturally (`LENGTH(key), key`), i.e. A1, A2, …, A10.

## Known limitations / deferred

- New-problem *assignment* into the daily agenda is **problem-level** for DSA: on DSA cadence
  days the planner schedules individual unsolved problems from the current new-learning
  category (Tries onward, `category_order >= 8`), one at a time, sized by difficulty
  (Easy 30 / Medium 45 / Hard 60 minutes) and limited to the day's remaining capacity.
  Completing a problem task records a solved attempt (and seeds its revision item); once all
  problems in a category are solved, the planner moves to the next category. Roadmap category
  topics (B1–B18) remain for progress display but are no longer scheduled as whole-category
  tasks.
- "Solved" is recorded either from the chosen attempt outcome or automatically when a DSA
  problem task is completed; there is no automatic enforcement of an "independent attempt"
  criterion beyond the user's own selection.
- Revision queue lists `source_type = problem` entries; other source types (LLD topics) are
  handled in later phases.
- Import reads the CSV chosen via the file picker; there is no bundled default dataset.
