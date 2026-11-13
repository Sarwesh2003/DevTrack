# DevTrack — Architecture

Baseline architecture evaluated per the kickoff prompt. All version choices must be verified against current official documentation before installation (Phase 1).

## 1. Recommended baseline

| Layer | Choice | Rationale |
|---|---|---|
| Desktop shell | Tauri 2 | Native Mac desktop app, small binary, webview frontend, Rust backend, no bundled Chromium |
| Frontend | React + TypeScript + Vite | Type-safe UI, fast dev/build, standard tooling |
| Styling / UI | Tailwind CSS + shadcn/ui | Dark theme first, developer-oriented, component primitives copied into the repo |
| Backend boundary | Tauri commands in Rust | Typed command boundary between UI and local operations |
| Database | SQLite | Single file, zero-config, local-first, migrations, ACID |
| Database access | Rust-native SQLite crate (vs Tauri SQL plugin) | Simplest maintainable option for a single-user app; see ADR-001 |
| Content files | Markdown, Java, JSON, Mermaid in app-managed data dir | Normal files the user can back up and inspect |
| Charts | Lightweight React chart library (only where analytics needs it) | Added in Phase 7, not Phase 1 |
| Diagrams | Mermaid source (`.mmd`/Markdown) rendered locally | Source files stored next to content |

## 2. Storage layout

Default data root: `~/Documents/DevTrack/` (configurable in Settings).

```
~/Documents/DevTrack/
├── devtrack.sqlite
├── content/
│   ├── dsa/<problem-slug>/
│   │   ├── notes.md
│   │   ├── intuition.md
│   │   ├── attempts/<attempt-id>/solution.java
│   │   └── diagrams/
│   ├── core-java/<topic-slug>/
│   ├── lld/<design-or-pattern-slug>/
│   ├── hld/<case-study-slug>/
│   ├── fde/<topic-or-project-slug>/
│   └── projects/<project-slug>/
├── imports/
├── exports/
└── backups/
```

- SQLite stores relative paths only; never hardcode machine-specific absolute paths.
- Protect against path traversal; write files atomically where practical.
- App must not scan or modify arbitrary directories without explicit user action.

## 3. Logical data model (entities)

- `tracks`, `modules`, `topics`, `subtopics`
- `resources`, `topic_dependencies`, `related_topics`
- `roadmap_phases`, `roadmap_weeks`, `weekly_plan_items`
- `problems`, `problem_attempts`, `problem_tags`, `problem_topic_links`
- `study_tasks`, `daily_plans`, `weekly_goals`, `study_sessions`
- `revision_items`, `revision_reviews`
- `learning_entries` (custom/unplanned learning)
- `projects`, `project_milestones`, `project_links`
- `content_files` (relative path, checksum, metadata)
- `activity_events` (heatmap/streak calculation)
- `settings`, `import_runs`, `schema_migrations`

Modeling rules:
- Stable UUIDs for internal IDs; stable source IDs/slugs for external curriculum records.
- Planned tasks stored separately from completed activity and custom entries.
- Each attempt and revision review is a separate historical record; never overwrite history.
- Completion state and mastery/readiness state stored separately.
- File contents live in files; SQLite stores relative path, checksum and metadata.
- Date-only fields as local calendar dates; instants as timezone-aware timestamps. Default timezone Asia/Kolkata, follow system or user setting.
- Migrations reversible where practical; back up before destructive migration.

## 4. Layering

UI components → feature services → typed repository interfaces → Tauri commands → SQLite/migrations/file storage.

UI components must not issue ad-hoc SQL or directly manipulate arbitrary filesystem paths. Persistence is behind typed service/repository interfaces.

## 5. Alternatives considered and rejected

- Electron: larger binary, bundled Chromium, higher memory — rejected for a simple local-first desktop tool.
- Tauri SQL plugin: convenient but adds an abstraction layer; a Rust-native SQLite crate is simpler and fully controlled for single-user local access. Re-evaluate if plugin offers clear migration/security benefits (see ADR-001).
- Spring Boot / Node server: rejected — no hosted backend, single-user local app.
- Cloud database / Firebase / Supabase: rejected — violates local-first and offline constraints.

## 6. Version verification (Phase 1 task)

Before installing, verify current stable compatible versions:
- Tauri 2 (`tauri`, `@tauri-apps/api`, `tauri-cli`)
- React, TypeScript, Vite
- Tailwind CSS, shadcn/ui
- Rust SQLite crate
- SQLite CLI for migrations/inspection

Do not copy stale setup commands from tutorials.
