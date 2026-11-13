use crate::db::{now_rfc3339, DbResult};
use rusqlite::Connection;

pub const SCHEMA_VERSION: i32 = 1;

pub fn ensure_schema(conn: &mut Connection) -> DbResult<()> {
  conn.execute_batch(
    "
    CREATE TABLE IF NOT EXISTS schema_migrations (
      version INTEGER PRIMARY KEY,
      name TEXT NOT NULL,
      applied_at TEXT NOT NULL
    );
    ",
  )?;

  let existing: i32 = conn.query_row(
    "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
    [],
    |row| row.get(0),
  )?;

  if existing < SCHEMA_VERSION {
    apply_migration(&mut *conn, SCHEMA_VERSION)?;
  }

  Ok(())
}

fn apply_migration(conn: &mut Connection, version: i32) -> DbResult<()> {
  let tx = conn.transaction()?;

  match version {
    1 => {
      tx.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS tracks (
          id TEXT PRIMARY KEY,
          key TEXT NOT NULL UNIQUE,
          name TEXT NOT NULL,
          description TEXT,
          position INTEGER NOT NULL,
          created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS modules (
          id TEXT PRIMARY KEY,
          track_id TEXT NOT NULL REFERENCES tracks(id),
          key TEXT NOT NULL,
          name TEXT NOT NULL,
          position INTEGER NOT NULL,
          UNIQUE(track_id, key)
        );

        CREATE TABLE IF NOT EXISTS topics (
          id TEXT PRIMARY KEY,
          module_id TEXT NOT NULL REFERENCES modules(id),
          key TEXT NOT NULL UNIQUE,
          title TEXT NOT NULL,
          status TEXT NOT NULL DEFAULT 'Not started',
          priority TEXT NOT NULL,
          estimate_minutes INTEGER,
          confidence INTEGER CHECK (confidence BETWEEN 1 AND 5),
          completion_criteria TEXT,
          notes TEXT,
          evidence TEXT,
          target_week INTEGER,
          is_custom INTEGER NOT NULL DEFAULT 0,
          source_ref TEXT,
          created_at TEXT NOT NULL,
          updated_at TEXT NOT NULL,
          UNIQUE(module_id, key)
        );

        CREATE TABLE IF NOT EXISTS subtopics (
          id TEXT PRIMARY KEY,
          topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
          key TEXT NOT NULL,
          title TEXT NOT NULL,
          is_complete INTEGER NOT NULL DEFAULT 0,
          position INTEGER NOT NULL,
          UNIQUE(topic_id, key)
        );

        CREATE TABLE IF NOT EXISTS resources (
          id TEXT PRIMARY KEY,
          topic_id TEXT REFERENCES topics(id),
          title TEXT NOT NULL,
          url TEXT,
          type TEXT,
          position INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS topic_dependencies (
          id TEXT PRIMARY KEY,
          topic_id TEXT NOT NULL REFERENCES topics(id),
          depends_on_topic_id TEXT NOT NULL REFERENCES topics(id),
          type TEXT NOT NULL,
          UNIQUE(topic_id, depends_on_topic_id)
        );

        CREATE TABLE IF NOT EXISTS related_topics (
          id TEXT PRIMARY KEY,
          topic_id TEXT NOT NULL REFERENCES topics(id),
          related_topic_id TEXT NOT NULL REFERENCES topics(id),
          UNIQUE(topic_id, related_topic_id)
        );

        CREATE TABLE IF NOT EXISTS roadmap_phases (
          id TEXT PRIMARY KEY,
          key TEXT NOT NULL UNIQUE,
          name TEXT NOT NULL,
          position INTEGER NOT NULL,
          source_ref TEXT
        );

        CREATE TABLE IF NOT EXISTS roadmap_weeks (
          id TEXT PRIMARY KEY,
          phase_id TEXT REFERENCES roadmap_phases(id),
          week_number INTEGER NOT NULL,
          track_allocations TEXT,
          total_estimate_minutes INTEGER,
          source_ref TEXT
        );

        CREATE TABLE IF NOT EXISTS weekly_plan_items (
          id TEXT PRIMARY KEY,
          week_id TEXT NOT NULL REFERENCES roadmap_weeks(id),
          topic_id TEXT REFERENCES topics(id),
          problem_id TEXT REFERENCES problems(id),
          task_type TEXT NOT NULL,
          title TEXT NOT NULL,
          estimate_minutes INTEGER NOT NULL,
          position INTEGER NOT NULL,
          day_of_week INTEGER,
          source_ref TEXT
        );

        CREATE TABLE IF NOT EXISTS problems (
          id TEXT PRIMARY KEY,
          external_id TEXT NOT NULL UNIQUE,
          title TEXT NOT NULL,
          category TEXT NOT NULL,
          difficulty TEXT NOT NULL,
          url TEXT NOT NULL,
          official_order INTEGER NOT NULL,
          statement TEXT,
          constraints TEXT,
          tags TEXT,
          is_previously_covered INTEGER NOT NULL DEFAULT 0,
          created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS problem_tags (
          id TEXT PRIMARY KEY,
          problem_id TEXT NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
          tag TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS problem_topic_links (
          id TEXT PRIMARY KEY,
          problem_id TEXT NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
          topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS problem_attempts (
          id TEXT PRIMARY KEY,
          problem_id TEXT NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
          attempt_date TEXT NOT NULL,
          outcome TEXT NOT NULL,
          duration_minutes INTEGER,
          notes TEXT,
          code_file TEXT,
          complexity_note TEXT,
          edge_cases TEXT,
          interview_explanation TEXT,
          created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS study_tasks (
          id TEXT PRIMARY KEY,
          daily_plan_id TEXT REFERENCES daily_plans(id),
          kind TEXT NOT NULL,
          title TEXT NOT NULL,
          track TEXT NOT NULL,
          topic_id TEXT REFERENCES topics(id),
          problem_id TEXT REFERENCES problems(id),
          revision_item_id TEXT REFERENCES revision_items(id),
          estimate_minutes INTEGER NOT NULL,
          actual_minutes INTEGER,
          status TEXT NOT NULL DEFAULT 'planned',
          output TEXT,
          completed_at TEXT,
          position INTEGER NOT NULL,
          scheduled_date TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS daily_plans (
          id TEXT PRIMARY KEY,
          plan_date TEXT NOT NULL UNIQUE,
          capacity_minutes INTEGER,
          available INTEGER NOT NULL DEFAULT 1,
          note TEXT,
          created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS weekly_goals (
          id TEXT PRIMARY KEY,
          week_start_date TEXT NOT NULL UNIQUE,
          track_targets TEXT,
          note TEXT,
          reviewed INTEGER NOT NULL DEFAULT 0,
          created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS study_sessions (
          id TEXT PRIMARY KEY,
          started_at TEXT NOT NULL,
          ended_at TEXT,
          track TEXT,
          minutes INTEGER,
          note TEXT
        );

        CREATE TABLE IF NOT EXISTS revision_items (
          id TEXT PRIMARY KEY,
          source_type TEXT NOT NULL,
          source_id TEXT NOT NULL,
          initial_completion_date TEXT,
          last_reviewed_date TEXT,
          next_due_date TEXT NOT NULL,
          current_interval INTEGER NOT NULL,
          repetition INTEGER NOT NULL DEFAULT 0,
          state TEXT NOT NULL DEFAULT 'learning',
          recall_result TEXT,
          confidence INTEGER CHECK (confidence BETWEEN 1 AND 5),
          notes TEXT,
          manual_override INTEGER NOT NULL DEFAULT 0,
          override_reason TEXT,
          created_at TEXT NOT NULL,
          updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS revision_reviews (
          id TEXT PRIMARY KEY,
          revision_item_id TEXT NOT NULL REFERENCES revision_items(id) ON DELETE CASCADE,
          reviewed_at TEXT NOT NULL,
          interval_before INTEGER NOT NULL,
          interval_after INTEGER NOT NULL,
          recall_result TEXT NOT NULL,
          confidence INTEGER CHECK (confidence BETWEEN 1 AND 5),
          notes TEXT,
          evidence TEXT,
          created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS learning_entries (
          id TEXT PRIMARY KEY,
          title TEXT NOT NULL,
          entry_date TEXT NOT NULL,
          track TEXT NOT NULL,
          description TEXT NOT NULL,
          notes TEXT,
          resource_url TEXT,
          duration_minutes INTEGER,
          output_file TEXT,
          tags TEXT,
          related_topic_id TEXT REFERENCES topics(id),
          related_problem_id TEXT REFERENCES problems(id),
          links_to_topic_id TEXT REFERENCES topics(id),
          created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS projects (
          id TEXT PRIMARY KEY,
          track TEXT NOT NULL,
          name TEXT NOT NULL,
          slug TEXT NOT NULL,
          status TEXT NOT NULL,
          repo_url TEXT,
          local_path TEXT,
          readme TEXT,
          milestone TEXT,
          notes TEXT,
          created_at TEXT NOT NULL,
          updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS project_milestones (
          id TEXT PRIMARY KEY,
          project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
          name TEXT NOT NULL,
          is_complete INTEGER NOT NULL DEFAULT 0,
          due_date TEXT,
          notes TEXT
        );

        CREATE TABLE IF NOT EXISTS project_links (
          id TEXT PRIMARY KEY,
          project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
          title TEXT NOT NULL,
          url TEXT NOT NULL,
          type TEXT
        );

        CREATE TABLE IF NOT EXISTS content_files (
          id TEXT PRIMARY KEY,
          relative_path TEXT NOT NULL UNIQUE,
          checksum TEXT NOT NULL,
          mime_type TEXT,
          size_bytes INTEGER,
          updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS activity_events (
          id TEXT PRIMARY KEY,
          event_date TEXT NOT NULL,
          event_type TEXT NOT NULL,
          source_id TEXT,
          minutes INTEGER,
          created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS settings (
          key TEXT PRIMARY KEY,
          value TEXT NOT NULL,
          updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS import_runs (
          id TEXT PRIMARY KEY,
          source TEXT NOT NULL,
          file_path TEXT NOT NULL,
          imported_at TEXT NOT NULL,
          record_count INTEGER NOT NULL,
          warnings TEXT,
          status TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_topics_status ON topics(status);
        CREATE INDEX IF NOT EXISTS idx_problems_category ON problems(category);
        CREATE INDEX IF NOT EXISTS idx_problems_order ON problems(official_order);
        CREATE INDEX IF NOT EXISTS idx_study_tasks_date ON study_tasks(scheduled_date, status);
        CREATE INDEX IF NOT EXISTS idx_daily_plans_date ON daily_plans(plan_date);
        CREATE INDEX IF NOT EXISTS idx_revision_items_due ON revision_items(next_due_date);
        CREATE INDEX IF NOT EXISTS idx_revision_reviews_item ON revision_reviews(revision_item_id);
        CREATE INDEX IF NOT EXISTS idx_activity_date ON activity_events(event_date);
        CREATE INDEX IF NOT EXISTS idx_learning_entries_date ON learning_entries(entry_date);
        ",
      )?;

      let now = now_rfc3339();
      tx.execute(
        "INSERT INTO schema_migrations (version, name, applied_at) VALUES (?1, ?2, ?3)",
        rusqlite::params![version, "initial_schema", now],
      )?;
    }
    _ => {
      return Err(crate::db::DbError::Serialization(format!("unknown migration version: {}", version)));
    }
  }

  tx.commit()?;
  Ok(())
}
