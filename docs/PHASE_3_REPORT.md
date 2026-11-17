# Phase 3 — Daily planner and weekly planning

**Status:** Complete (pending user approval)

## Goal

Make the dashboard useful every day: generate a believable agenda from the imported
curriculum and available time, complete planned work with planned-vs-actual time, add
unplanned learning, mark rest days, reschedule missed work without duplication, and
complete a weekly review.

## What was built

### 1. Persisted planner settings

- `settings` are now stored in the local SQLite `settings` table (previously `get_settings`
  returned defaults and `save_settings` was a no-op).
- New planner fields: `study_days` (ISO 1–7), `cadence` (weekday → track handles),
  `min_meaningful_minutes`, `daily_buffer_ratio`.
- Migrations now also run at app startup so a fresh database has its schema immediately.

### 2. Planner engine (`src-tauri/src/planner.rs`)

Deterministic, testable scheduling:

- Weekly generation for a Monday-aligned week, one primary task per study day from the
  next unstarted topic in the day's cadence track. If a cadence track has no content yet
  (e.g. DSA problems are imported in Phase 4), it falls back to another track with
  unstarted topics so the agenda stays believable.
- Task categories: `planned`, `revision`, `review`, `stretch`, `custom`.
- Per-day capacity = `weekly_capacity_hours × 60 / study_days`, with a configurable
  buffer (default 15%). A weekly-review task is added on the review day.
- Topic de-duplication: a topic already referenced by a task is never scheduled again.
- Rest days respected: generation and replanning never overwrite a day the user marked
  unavailable, and never schedule into past days.
- Due revisions become revision tasks when a `revision_items` row is due (none until
  Phase 4 seeds them).
- Task operations: complete (with actual minutes and optional output), skip, move +1 day.
- Rescheduling: `replan_backlog` redistributes incomplete tasks dated before today across
  the next N available days by remaining capacity — it moves rows (never copies), so
  missed work does not pile up or duplicate.
- Weekly review persistence through `weekly_goals` (note, reviewed flag).

Completing a planned task moves its topic from `Not started` to `Learning`; it never
auto-completes a topic.

### 3. Learning entries, activity and streaks (`src-tauri/src/learning.rs`)

- Custom learning entries with the documented fields (title, date, track, description,
  notes, resource URL, duration, output file, tags, related topic).
- Every completed task and learning entry records an `activity_events` row.
- Activity summary aggregates minutes/events per day, computes current and longest streaks
  using the configurable "meaningful activity" threshold, and powers the heatmap.

### 4. Frontend

- **Today (Dashboard):** date/week header, stats (completed today, planned vs actual
  minutes, streak, revisions due), "Generate this week's plan" and "Replan missed"
  actions, agenda grouped by primary learning / revision / weekly review / optional
  stretch / additional learning, per-task actual-minutes input, Done/Move/Skip, and an
  inline "Add Learning Entry" form with recent entries.
- **Weekly Review:** week navigation, planned/completed/skipped/completion stats,
  per-track tally, editable weekly goal + "reviewed" flag, and a 12-week activity heatmap.
- **Settings:** editable study days, weekly capacity, minimum meaningful activity, and a
  per-weekday cadence editor (DSA/Java/LLD/HLD/FDE/Review).
- Sidebar now exposes real pages only: Today, Roadmap, Weekly Review, Settings.

## Files

- Backend: `src-tauri/src/planner.rs` (new), `src-tauri/src/learning.rs` (new),
  `src-tauri/src/db.rs` (settings persistence), `src-tauri/src/commands.rs`
  (real get/save settings), `src-tauri/src/main.rs` (modules, command registration,
  startup migrations).
- Frontend: `src/features/dashboard/Dashboard.tsx`, `src/features/review/Review.tsx` (new),
  `src/features/settings/Settings.tsx`, `src/components/layout/Sidebar.tsx`,
  `src/components/layout/Shell.tsx`, `src/App.tsx`, `src/lib/dates.ts` (new),
  `src/types/index.ts`, `src/services/api.ts`, `src/services/useSettings.ts`.

## Tests

- Rust (`cargo test`): 17 passing.
  - Roadmap import: real-file parse, header parsing, re-import idempotency/FK (4).
  - Planner: date/week helpers, default + custom cadence, capacity math, track mapping,
    and a DB-backed test that generation avoids duplicate topics and respects rest days (7).
  - Learning: current/longest streak edge cases (5).
  - Persistence: settings round-trip with planner fields (1).
- Frontend (`npx vitest run`): 46 passing (adds API-shape tests for all new commands).
- `npx tsc --noEmit`, `eslint --max-warnings 0`, `npm run build`, `cargo build`: clean.

## How to validate

```bash
cd /Users/sarwesh/Desktop/Projects/DevTrack
npm run tauri dev
```

1. Settings → set study days, weekly capacity and cadence → Save.
2. Roadmap → (re)confirm the curriculum is imported if needed.
3. Today → **Generate this week's plan** → an agenda appears for the remaining days.
4. Complete a task with actual minutes; add an unplanned learning entry.
5. Mark a day unavailable in Settings or move tasks, then **Replan missed** to confirm no
   duplication/pile-up.
6. Weekly Review → add a weekly goal, mark reviewed, check the activity heatmap/streak.

## Known limitations / deferred

- DSA problem-level scheduling, attempts and the spaced-revision seeding belong to Phase 4;
  until the NeetCode dataset is imported, DSA cadence days fall back to other tracks.
- The 26-week phase/week index is not shown yet because the app has no roadmap start date;
  the current week is shown by its Monday date.
- Drag-and-drop reordering is deferred; tasks can be moved by +1 day or completed/skipped.
- Streaks count days whose activity minutes meet the configurable threshold.
