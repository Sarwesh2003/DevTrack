# DevTrack — Database Schema

Logical schema for the SQLite database. Versioned migrations live in `src-tauri/migrations/`. All internal IDs are UUIDs; external curriculum records use stable source IDs/slugs.

## 1. Core curriculum

```sql
CREATE TABLE tracks (
  id TEXT PRIMARY KEY,          -- uuid
  key TEXT NOT NULL UNIQUE,     -- dsa, core_java, lld, hld, fde
  name TEXT NOT NULL,
  description TEXT,
  position INTEGER NOT NULL,
  created_at TEXT NOT NULL
);

CREATE TABLE modules (
  id TEXT PRIMARY KEY,
  track_id TEXT NOT NULL REFERENCES tracks(id),
  key TEXT NOT NULL,
  name TEXT NOT NULL,
  position INTEGER NOT NULL,
  UNIQUE(track_id, key)
);

CREATE TABLE topics (
  id TEXT PRIMARY KEY,
  module_id TEXT NOT NULL REFERENCES modules(id),
  key TEXT NOT NULL UNIQUE,     -- stable source id/slug
  title TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'Not started',  -- Not started, Learning, Practicing, Needs revision, Interview ready, Mastered
  priority TEXT NOT NULL,       -- High, Medium, Low
  estimate_minutes INTEGER,
  confidence INTEGER CHECK (confidence BETWEEN 1 AND 5),
  completion_criteria TEXT,
  notes TEXT,
  evidence TEXT,                -- reference to output files
  target_week INTEGER,
  is_custom INTEGER NOT NULL DEFAULT 0,
  source_ref TEXT,              -- reference to imported roadmark source
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  UNIQUE(module_id, key)
);

CREATE TABLE subtopics (
  id TEXT PRIMARY KEY,
  topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
  key TEXT NOT NULL,
  title TEXT NOT NULL,
  is_complete INTEGER NOT NULL DEFAULT 0,
  position INTEGER NOT NULL,
  UNIQUE(topic_id, key)
);

CREATE TABLE resources (
  id TEXT PRIMARY KEY,
  topic_id TEXT REFERENCES topics(id),
  title TEXT NOT NULL,
  url TEXT,
  type TEXT,                    -- link, file, video, article
  position INTEGER NOT NULL
);

CREATE TABLE topic_dependencies (
  id TEXT PRIMARY KEY,
  topic_id TEXT NOT NULL REFERENCES topics(id),
  depends_on_topic_id TEXT NOT NULL REFERENCES topics(id),
  type TEXT NOT NULL,           -- prerequisite, related
  UNIQUE(topic_id, depends_on_topic_id)
);

CREATE TABLE related_topics (
  id TEXT PRIMARY KEY,
  topic_id TEXT NOT NULL REFERENCES topics(id),
  related_topic_id TEXT NOT NULL REFERENCES topics(id),
  UNIQUE(topic_id, related_topic_id)
);
```

## 2. Roadmap / weekly plan

```sql
CREATE TABLE roadmap_phases (
  id TEXT PRIMARY KEY,
  key TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  position INTEGER NOT NULL,
  source_ref TEXT
);

CREATE TABLE roadmap_weeks (
  id TEXT PRIMARY KEY,
  phase_id TEXT REFERENCES roadmap_phases(id),
  week_number INTEGER NOT NULL,
  track_allocations TEXT,       -- JSON: { dsa: 2.5, java: 1.0, ... }
  total_estimate_minutes INTEGER,
  source_ref TEXT
);

CREATE TABLE weekly_plan_items (
  id TEXT PRIMARY KEY,
  week_id TEXT NOT NULL REFERENCES roadmap_weeks(id),
  topic_id TEXT REFERENCES topics(id),
  problem_id TEXT REFERENCES problems(id),
  task_type TEXT NOT NULL,      -- learning, revision, review, custom, stretch
  title TEXT NOT NULL,
  estimate_minutes INTEGER NOT NULL,
  position INTEGER NOT NULL,
  day_of_week INTEGER,          -- 1=Mon ... 7=Sun
  source_ref TEXT
);
```

## 3. DSA / NeetCode

```sql
CREATE TABLE problems (
  id TEXT PRIMARY KEY,
  external_id TEXT NOT NULL UNIQUE,   -- stable NeetCode identifier/slug
  title TEXT NOT NULL,
  category TEXT NOT NULL,             -- official category
  difficulty TEXT NOT NULL,           -- Easy, Medium, Hard
  url TEXT NOT NULL,
  official_order INTEGER NOT NULL,
  statement TEXT,
  constraints TEXT,
  tags TEXT,                          -- JSON array
  is_previously_covered INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL
);

CREATE TABLE problem_tags (
  id TEXT PRIMARY KEY,
  problem_id TEXT NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
  tag TEXT NOT NULL
);

CREATE TABLE problem_topic_links (
  id TEXT PRIMARY KEY,
  problem_id TEXT NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
  topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE
);

CREATE TABLE problem_attempts (
  id TEXT PRIMARY KEY,
  problem_id TEXT NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
  attempt_date TEXT NOT NULL,         -- local date
  outcome TEXT NOT NULL,              -- accepted, partial, incorrect, cannot_explain
  duration_minutes INTEGER,
  notes TEXT,
  code_file TEXT,                     -- relative path to solution.java
  complexity_note TEXT,
  edge_cases TEXT,
  interview_explanation TEXT,
  created_at TEXT NOT NULL
);
```

## 4. Tasks, plans, sessions

```sql
CREATE TABLE study_tasks (
  id TEXT PRIMARY KEY,
  daily_plan_id TEXT REFERENCES daily_plans(id),
  kind TEXT NOT NULL,                 -- planned_learning, revision, stretch, custom_entry_link
  title TEXT NOT NULL,
  track TEXT NOT NULL,
  topic_id TEXT REFERENCES topics(id),
  problem_id TEXT REFERENCES problems(id),
  revision_item_id TEXT REFERENCES revision_items(id),
  estimate_minutes INTEGER NOT NULL,
  actual_minutes INTEGER,
  status TEXT NOT NULL DEFAULT 'planned', -- planned, in_progress, completed, missed, postponed
  output TEXT,                        -- evidence/reference
  completed_at TEXT,
  position INTEGER NOT NULL,
  scheduled_date TEXT NOT NULL        -- local date
);

CREATE TABLE daily_plans (
  id TEXT PRIMARY KEY,
  plan_date TEXT NOT NULL UNIQUE,     -- local date
  capacity_minutes INTEGER,
  available INTEGER NOT NULL DEFAULT 1,
  note TEXT,
  created_at TEXT NOT NULL
);

CREATE TABLE weekly_goals (
  id TEXT PRIMARY KEY,
  week_start_date TEXT NOT NULL UNIQUE,
  track_targets TEXT,                 -- JSON
  note TEXT,
  reviewed INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL
);

CREATE TABLE study_sessions (
  id TEXT PRIMARY KEY,
  started_at TEXT NOT NULL,           -- timestamp
  ended_at TEXT,
  track TEXT,
  minutes INTEGER,
  note TEXT
);
```

## 5. Revision engine

```sql
CREATE TABLE revision_items (
  id TEXT PRIMARY KEY,
  source_type TEXT NOT NULL,          -- problem, topic, subtopic, design, project
  source_id TEXT NOT NULL,
  initial_completion_date TEXT,       -- local date, may be NULL
  last_reviewed_date TEXT,
  next_due_date TEXT NOT NULL,
  current_interval INTEGER NOT NULL,  -- days
  repetition INTEGER NOT NULL DEFAULT 0,
  state TEXT NOT NULL DEFAULT 'learning', -- learning, weak, mastered
  recall_result TEXT,                 -- last outcome
  confidence INTEGER CHECK (confidence BETWEEN 1 AND 5),
  notes TEXT,
  manual_override INTEGER NOT NULL DEFAULT 0,
  override_reason TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE revision_reviews (
  id TEXT PRIMARY KEY,
  revision_item_id TEXT NOT NULL REFERENCES revision_items(id) ON DELETE CASCADE,
  reviewed_at TEXT NOT NULL,
  interval_before INTEGER NOT NULL,
  interval_after INTEGER NOT NULL,
  recall_result TEXT NOT NULL,        -- correct_clear, correct_slow, partial, incorrect
  confidence INTEGER CHECK (confidence BETWEEN 1 AND 5),
  notes TEXT,
  evidence TEXT,
  created_at TEXT NOT NULL
);
```

## 6. Custom learning entries

```sql
CREATE TABLE learning_entries (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  entry_date TEXT NOT NULL,           -- local date/time
  track TEXT NOT NULL,                -- or 'Unplanned / Other'
  description TEXT NOT NULL,
  notes TEXT,                         -- markdown
  resource_url TEXT,
  duration_minutes INTEGER,
  output_file TEXT,                   -- relative path
  tags TEXT,                          -- JSON array
  related_topic_id TEXT REFERENCES topics(id),
  related_problem_id TEXT REFERENCES problems(id),
  links_to_topic_id TEXT REFERENCES topics(id),  -- explicit evidence link
  created_at TEXT NOT NULL
);
```

## 7. Projects (LLD/HLD/FDE)

```sql
CREATE TABLE projects (
  id TEXT PRIMARY KEY,
  track TEXT NOT NULL,
  name TEXT NOT NULL,
  slug TEXT NOT NULL,
  status TEXT NOT NULL,
  repo_url TEXT,
  local_path TEXT,                    -- relative path inside content dir
  readme TEXT,
  milestone TEXT,                     -- M1..M6 for FDE; project phase for LLD/HLD
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE project_milestones (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  is_complete INTEGER NOT NULL DEFAULT 0,
  due_date TEXT,
  notes TEXT
);

CREATE TABLE project_links (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  title TEXT NOT NULL,
  url TEXT NOT NULL,
  type TEXT                           -- diagram, code, test, doc
);
```

## 7b. Learning workspaces (Phase 5, migration v5)

Added by `SCHEMA_VERSION = 5` (`learning_workspaces`). Each table references `topics(id)` with
`ON DELETE CASCADE` and has a per-topic index.

```sql
CREATE TABLE code_experiments (
  id TEXT PRIMARY KEY,
  topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
  title TEXT NOT NULL,
  code TEXT NOT NULL,
  language TEXT NOT NULL DEFAULT 'java',
  file_path TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE recall_questions (
  id TEXT PRIMARY KEY,
  topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
  question TEXT NOT NULL,
  answer TEXT NOT NULL,
  difficulty TEXT NOT NULL DEFAULT 'medium',
  last_reviewed TEXT,
  next_review TEXT,
  interval INTEGER NOT NULL DEFAULT 1,
  repetition INTEGER NOT NULL DEFAULT 1,
  state TEXT NOT NULL DEFAULT 'learning',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE diagrams (
  id TEXT PRIMARY KEY,
  topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
  title TEXT NOT NULL,
  mermaid_content TEXT NOT NULL,
  file_path TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE design_artifacts (
  id TEXT PRIMARY KEY,
  topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
  title TEXT NOT NULL,
  requirements TEXT,
  api_spec TEXT,
  java_implementation TEXT,
  tests TEXT,
  trade_offs TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE case_studies (
  id TEXT PRIMARY KEY,
  topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
  title TEXT NOT NULL,
  context TEXT,
  diagram_content TEXT,
  trade_offs TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE topic_links (
  id TEXT PRIMARY KEY,
  from_topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
  to_topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
  link_type TEXT NOT NULL DEFAULT 'related',
  created_at TEXT NOT NULL,
  UNIQUE(from_topic_id, to_topic_id, link_type)
);
```

## 7c. Project portfolio extensions (Phase 6, migration v6)

Added by `SCHEMA_VERSION = 6` (`fde_projects`). The `projects`,
`project_milestones` and `project_links` tables already exist from the initial schema;
v6 adds task tracking and a typed notes journal.

```sql
CREATE TABLE IF NOT EXISTS project_tasks (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  title TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'todo',
  due_date TEXT,
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS project_notes (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  kind TEXT NOT NULL DEFAULT 'note',
  title TEXT NOT NULL,
  content TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

## 8. Content files, activity, settings, import, migrations
```sql
CREATE TABLE content_files (
  id TEXT PRIMARY KEY,
  relative_path TEXT NOT NULL UNIQUE,
  checksum TEXT NOT NULL,
  mime_type TEXT,
  size_bytes INTEGER,
  updated_at TEXT NOT NULL
);

CREATE TABLE activity_events (
  id TEXT PRIMARY KEY,
  event_date TEXT NOT NULL,           -- local date
  event_type TEXT NOT NULL,           -- task_completed, revision_completed, custom_entry, session
  source_id TEXT,
  minutes INTEGER,
  created_at TEXT NOT NULL
);

CREATE TABLE settings (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE import_runs (
  id TEXT PRIMARY KEY,
  source TEXT NOT NULL,               -- roadmap, neetcode
  file_path TEXT NOT NULL,
  imported_at TEXT NOT NULL,
  record_count INTEGER NOT NULL,
  warnings TEXT,                      -- JSON
  status TEXT NOT NULL                -- completed, partial, failed
);

CREATE TABLE schema_migrations (
  version INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  applied_at TEXT NOT NULL
);
```

## 9. Indexes

```sql
CREATE INDEX idx_topics_status ON topics(status);
CREATE INDEX idx_problems_category ON problems(category);
CREATE INDEX idx_problems_order ON problems(official_order);
CREATE INDEX idx_study_tasks_date ON study_tasks(scheduled_date, status);
CREATE INDEX idx_daily_plans_date ON daily_plans(plan_date);
CREATE INDEX idx_revision_items_due ON revision_items(next_due_date);
CREATE INDEX idx_revision_reviews_item ON revision_reviews(revision_item_id);
CREATE INDEX idx_activity_date ON activity_events(event_date);
CREATE INDEX idx_learning_entries_date ON learning_entries(entry_date);
```

## 10. Key decisions

- Planned tasks (`study_tasks`) kept separate from completed activity (`activity_events`) and custom entries (`learning_entries`).
- Each attempt and revision review is immutable history; updates create new rows, never overwrite.
- Completion state and mastery state are separate columns on `topics` and `revision_items`.
- File contents live on disk; `content_files` stores relative path, checksum and metadata only.
- Dates stored as local calendar date strings (`YYYY-MM-DD`); timestamps as ISO-8601 with timezone.
