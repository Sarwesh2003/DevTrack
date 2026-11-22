use crate::db::{now_rfc3339, DbResult};
use rusqlite::Connection;

pub const SCHEMA_VERSION: i32 = 5;

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

  for v in (existing + 1)..=SCHEMA_VERSION {
    apply_migration(&mut *conn, v)?;
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
    2 => {
      tx.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS favorites (
          id TEXT PRIMARY KEY,
          problem_id TEXT NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
          created_at TEXT NOT NULL,
          UNIQUE(problem_id)
        );

        CREATE TABLE IF NOT EXISTS solutions (
          id TEXT PRIMARY KEY,
          problem_id TEXT NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
          file_path TEXT NOT NULL,
          content TEXT,
          language TEXT NOT NULL DEFAULT 'java',
          updated_at TEXT NOT NULL,
          UNIQUE(problem_id)
        );

        CREATE INDEX IF NOT EXISTS idx_favorites_problem ON favorites(problem_id);
        CREATE INDEX IF NOT EXISTS idx_solutions_problem ON solutions(problem_id);
        ",
      )?;

      let now = now_rfc3339();
      tx.execute(
        "INSERT INTO schema_migrations (version, name, applied_at) VALUES (?1, ?2, ?3)",
        rusqlite::params![version, "dsa_workspace", now],
      )?;
    }
    3 => {
      // Distinguish manually edited daily plans from auto-generated ones so
      // regeneration can refresh capacity/availability from current settings
      // without overwriting explicit user overrides.
      tx.execute_batch(
        "ALTER TABLE daily_plans ADD COLUMN manual_override INTEGER NOT NULL DEFAULT 0;",
      )?;

      let now = now_rfc3339();
      tx.execute(
        "INSERT INTO schema_migrations (version, name, applied_at) VALUES (?1, ?2, ?3)",
        rusqlite::params![version, "daily_plan_manual_override", now],
      )?;
    }
    4 => {
      // Link problem attempts to the study task that produced them so undoing a
      // completed DSA task can remove the auto-created solved attempt without
      // touching attempts the user logged manually.
      tx.execute_batch(
        "ALTER TABLE problem_attempts ADD COLUMN study_task_id TEXT;",
      )?;
      tx.execute_batch(
        "CREATE INDEX IF NOT EXISTS idx_problem_attempts_task ON problem_attempts(study_task_id);",
      )?;

      let now = now_rfc3339();
      tx.execute(
        "INSERT INTO schema_migrations (version, name, applied_at) VALUES (?1, ?2, ?3)",
        rusqlite::params![version, "dsa_problem_task_link", now],
      )?;
    }
    5 => {
      // Phase 5: deeper learning workspaces (Java notes/experiments, LLD design,
      // HLD case studies) plus Mermaid diagrams, local source-file storage,
      // recall questions and cross-track topic links.
      tx.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS code_experiments (
          id TEXT PRIMARY KEY,
          topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
          title TEXT NOT NULL,
          code TEXT NOT NULL,
          language TEXT NOT NULL DEFAULT 'java',
          file_path TEXT,
          created_at TEXT NOT NULL,
          updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS recall_questions (
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
        CREATE TABLE IF NOT EXISTS diagrams (
          id TEXT PRIMARY KEY,
          topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
          title TEXT NOT NULL,
          mermaid_content TEXT NOT NULL,
          file_path TEXT,
          created_at TEXT NOT NULL,
          updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS design_artifacts (
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
        CREATE TABLE IF NOT EXISTS case_studies (
          id TEXT PRIMARY KEY,
          topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
          title TEXT NOT NULL,
          context TEXT,
          diagram_content TEXT,
          trade_offs TEXT,
          created_at TEXT NOT NULL,
          updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS topic_links (
          id TEXT PRIMARY KEY,
          from_topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
          to_topic_id TEXT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
          link_type TEXT NOT NULL DEFAULT 'related',
          created_at TEXT NOT NULL,
          UNIQUE(from_topic_id, to_topic_id, link_type)
        );
        CREATE INDEX IF NOT EXISTS idx_code_experiments_topic ON code_experiments(topic_id);
        CREATE INDEX IF NOT EXISTS idx_recall_questions_topic ON recall_questions(topic_id);
        CREATE INDEX IF NOT EXISTS idx_diagrams_topic ON diagrams(topic_id);
        CREATE INDEX IF NOT EXISTS idx_design_artifacts_topic ON design_artifacts(topic_id);
        CREATE INDEX IF NOT EXISTS idx_case_studies_topic ON case_studies(topic_id);
        CREATE INDEX IF NOT EXISTS idx_topic_links_from ON topic_links(from_topic_id);
        CREATE INDEX IF NOT EXISTS idx_topic_links_to ON topic_links(to_topic_id);
        ",
      )?;

      let now = now_rfc3339();
      tx.execute(
        "INSERT INTO schema_migrations (version, name, applied_at) VALUES (?1, ?2, ?3)",
        rusqlite::params![version, "learning_workspaces", now],
      )?;
    }
    _ => {
      return Err(crate::db::DbError::Serialization(format!("unknown migration version: {}", version)));
    }
  }

  tx.commit()?;
  Ok(())
}
