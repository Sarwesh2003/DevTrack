# DevTrack Project Instructions

## Product constraints
- This is a single-user, local-first Mac desktop application. Do not add cloud hosting, Firebase, Supabase, remote databases, mandatory accounts or a deployed backend.
- Core functionality must work offline.
- Keep user data in an explicit app-managed local data directory. Never scan or modify arbitrary filesystem locations without explicit user selection.
- Store structured records in SQLite and learning content as normal local Markdown, Java, JSON and Mermaid files.
- Never commit secrets or store API tokens in plaintext files/SQLite.

## Learning progress integrity
- DSA new-learning cursor starts at Tries. Categories through Trees are revision-only by default.
- LLD new-learning cursor starts at Bridge Pattern. Previous content through Composite is revision-only.
- Core Java begins at A1, HLD at D1 and FDE at E1 unless the user changes the roadmap.
- Never fabricate historical solve dates, problem attempts, accepted statuses, code, confidence or learning activity.
- Do not add non-NeetCode questions to the official DSA track.
- Preserve original source data and user progress during re-imports/migrations.
- Offer a one-time "Start revision baseline now" action for previously covered items with no history; it establishes a new baseline and must not pretend to recover original dates.

## Architecture and code quality
- Keep UI, domain/business logic, repositories and Tauri commands separated.
- UI components must not issue ad-hoc SQL or perform arbitrary filesystem operations.
- Use typed interfaces, database migrations, input validation and safe error handling.
- Protect file operations against path traversal and use atomic writes where practical.
- Keep external integrations optional and isolated from core learning features.
- Prefer simple, maintainable solutions; avoid unnecessary services, Docker, cloud infrastructure and framework sprawl.

## Incremental delivery
- Implement one approved phase at a time from `docs/DEVTRACK_IMPLEMENTATION_PLAN.md`.
- Do not begin a later phase automatically.
- Each phase requires tests, typecheck/lint/build verification, updated documentation and a completion report.
- Add tests for scheduling, revision, import and persistence logic before relying on them in the UI.
- Do not create fake user-facing dashboard data. Keep test fixtures clearly labeled and separate from user data.

## Documentation
- Keep `README.md`, architecture decisions, database schema, implementation plan and this file updated when behavior or conventions change.
- Explain material architecture decisions and record them in `docs/decisions/`.

## Phase 0 deliverables (created)
- `README.md`
- `AGENTS.md`
- `docs/PRODUCT_REQUIREMENTS.md`
- `docs/ARCHITECTURE.md`
- `docs/DATABASE_SCHEMA.md`
- `docs/DEVTRACK_IMPLEMENTATION_PLAN.md` (pre-existing)
- `docs/ROADMAP_IMPORT.md`
- `docs/decisions/ADR-001-local-first-architecture.md`
- `docs/PHASE_0_REPORT.md`

## Phase 1 deliverables (complete)
- `src-tauri/` — Tauri 2 Rust backend with SQLite, migrations, settings, backup/restore
- `src/` — React + TypeScript + Vite frontend with Dashboard and Settings pages
- `docs/PHASE_1_REPORT.md`

## Phase 2 deliverables (complete)
- `src-tauri/src/curriculum.rs` — curriculum CRUD commands (tracks, modules, topics, subtopics, resources, dependencies)
- `src-tauri/src/roadmap_import.rs` — deterministic Markdown roadmap import with progress overrides
- `src-tauri/src/commands.rs` — roadmap phase/week/plan-item commands
- `src/features/roadmap/Roadmap.tsx` — roadmap page with track list
- `src/types/index.ts` — curriculum, roadmap types
- `src/services/api.ts` — API service for new commands
- `docs/PHASE_2_REPORT.md`

## Phase 3 deliverables (complete)
- `src-tauri/src/planner.rs` — weekly generation, daily agenda, task completion/move/skip, availability, backlog replanning, weekly review (with unit + DB tests)
- `src-tauri/src/learning.rs` — custom learning entries, activity events, streak/heatmap summary (with tests)
- `src-tauri/src/db.rs` — persisted settings (study days, cadence, capacity, meaningful-activity threshold) with round-trip test
- `src-tauri/src/commands.rs` — real get/save settings; migrations run at startup (`main.rs`)
- `src/features/dashboard/Dashboard.tsx` — Today agenda, generate/replan, complete tasks, add learning entries, stats
- `src/features/review/Review.tsx` — weekly review, per-track summary, activity heatmap
- `src/features/settings/Settings.tsx` — study days, capacity and weekly cadence editor
- `src/lib/dates.ts` — local date/week helpers
- `src/types/index.ts`, `src/services/api.ts` — planner/learning types and commands
- `docs/PHASE_3_REPORT.md`

## Phase 4 deliverables (complete)
- `src-tauri/src/migrations.rs` — `SCHEMA_VERSION = 4`; v2 adds `favorites` and `solutions`; v3 adds `daily_plans.manual_override`; v4 adds `problem_attempts.study_task_id`; sequential pending-migration loop
- `src-tauri/src/dsa.rs` — quote-aware NeetCode CSV preview/import, problem list + filters (category/difficulty/search/coverage), favorite toggle, attempts + history, Java solution file save/load under `content/solutions/`, spaced revision engine (1/3/7/14/30/60) + review history, due queue, one-time revision baseline, per-category progress; CSV import also seeds the roadmap DSA track (B1–B18 category topics, B1–B7 revision-only, B8+ new) (with unit tests)
- `src-tauri/src/main.rs` — `dsa` module and all DSA command registrations
- `src/features/dsa/DSA.tsx` — DSA page: CSV import preview/confirm, questions list + detail (attempts, solution editor, favorite), revision queue + baseline, category progress
- `src/App.tsx`, `src/components/layout/Sidebar.tsx`, `src/components/layout/Shell.tsx` — `dsa` page route and nav
- `src/types/index.ts`, `src/services/api.ts`, `src/services/api.test.ts` — DSA types, command wrappers and API-shape tests
- `docs/PHASE_4_REPORT.md`

## Roadmap source (present)
- `docs/roadmap/six_month_engineering_roadmap.md` — complete 26-week roadmap imported and validated as a planning reference.
- `docs/roadmap/CURRENT_PROGRESS.yaml` — per-track progress checkpoints (DSA through Trees, LLD through Composite).
- `/Users/sarwesh/Downloads/NEETCODE_250.csv` — official NeetCode 250 dataset (250 problems, validated). Imported via the DSA page's CSV import.

## Git and delivery
- Remote: https://github.com/Sarwesh2003/DevTrack.git (master branch).
- After each phase is approved by the user, push that phase's changes to master.
- Do not commit until explicitly asked; do not push until phase approval.
