use crate::db::{load_settings, now_rfc3339, open_connection, Settings};
use chrono::{Datelike, Duration, NaiveDate, Weekday};
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Domain helpers (pure, unit tested)
// ---------------------------------------------------------------------------

pub fn parse_date(value: &str) -> Option<NaiveDate> {
  NaiveDate::parse_from_str(value, "%Y-%m-%d").ok()
}

pub fn iso_weekday(date: NaiveDate) -> i32 {
  date.weekday().number_from_monday() as i32
}

pub fn week_dates(week_start: NaiveDate) -> Vec<NaiveDate> {
  (0..7).map(|i| week_start + Duration::days(i as i64)).collect()
}

pub fn default_cadence(iso: i32) -> Vec<String> {
  match iso {
    1 | 3 | 5 => vec!["dsa".to_string()],
    2 => vec!["java".to_string()],
    4 => vec!["lld".to_string()],
    6 => vec!["fde".to_string()],
    7 => vec!["review".to_string()],
    _ => vec![],
  }
}

pub fn cadence_for(settings: &Settings, iso: i32) -> Vec<String> {
  if let Some(cadence) = &settings.cadence {
    if let Some(tracks) = cadence.get(&iso.to_string()) {
      if !tracks.is_empty() {
        return tracks.clone();
      }
    }
  }
  default_cadence(iso)
}

pub fn track_key_for_handle(handle: &str) -> Option<&'static str> {
  match handle.to_ascii_lowercase().as_str() {
    "java" => Some("a"),
    "dsa" => Some("b"),
    "lld" => Some("c"),
    "hld" => Some("d"),
    "fde" => Some("e"),
    _ => None,
  }
}

pub fn study_day_count(settings: &Settings) -> i32 {
  settings
    .study_days
    .as_ref()
    .map(|days| days.iter().filter(|d| (1..=7).contains(*d)).count() as i32)
    .filter(|count| *count > 0)
    .unwrap_or(6)
}

pub fn per_day_capacity_minutes(settings: &Settings) -> i32 {
  let weekly = settings.weekly_capacity_hours.unwrap_or(7.5).max(0.0);
  let days = study_day_count(settings).max(1);
  ((weekly * 60.0) / days as f64).round() as i32
}

pub fn usable_capacity_minutes(capacity: i32, settings: &Settings) -> i32 {
  let buffer = settings.daily_buffer_ratio.unwrap_or(0.15).clamp(0.0, 0.5);
  ((capacity as f64) * (1.0 - buffer)).round() as i32
}

fn is_available(settings: &Settings, iso: i32) -> bool {
  settings
    .study_days
    .as_ref()
    .map(|days| days.contains(&iso))
    .unwrap_or(iso != 7)
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyPlan {
  pub id: String,
  pub plan_date: String,
  pub capacity_minutes: Option<i32>,
  pub available: bool,
  pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudyTask {
  pub id: String,
  pub daily_plan_id: Option<String>,
  pub kind: String,
  pub title: String,
  pub track: String,
  pub topic_id: Option<String>,
  pub problem_id: Option<String>,
  pub revision_item_id: Option<String>,
  pub estimate_minutes: i32,
  pub actual_minutes: Option<i32>,
  pub status: String,
  pub output: Option<String>,
  pub completed_at: Option<String>,
  pub position: i32,
  pub scheduled_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgendaResponse {
  pub ok: bool,
  pub plan: Option<DailyPlan>,
  pub tasks: Vec<StudyTask>,
  pub due_revisions: i32,
  pub planned_minutes: i32,
  pub completed_minutes: i32,
  pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GenerateWeekRequest {
  pub week_start: String,
  #[serde(default)]
  pub force: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateWeekResponse {
  pub ok: bool,
  pub tasks_created: i32,
  pub days_generated: i32,
  pub warnings: Vec<String>,
  pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SetAvailabilityRequest {
  pub plan_date: String,
  pub available: bool,
  pub capacity_minutes: Option<i32>,
  pub note: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
  pub id: String,
  pub status: Option<String>,
  pub actual_minutes: Option<i32>,
  pub output: Option<String>,
  pub scheduled_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskResponse {
  pub ok: bool,
  pub task: Option<StudyTask>,
  pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
  pub scheduled_date: String,
  pub title: String,
  pub track: String,
  pub kind: Option<String>,
  pub topic_id: Option<String>,
  pub estimate_minutes: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskResponse {
  pub ok: bool,
  pub id: Option<String>,
  pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReplanRequest {
  pub today: String,
  #[serde(default = "default_horizon")]
  pub horizon_days: i64,
}

fn default_horizon() -> i64 {
  14
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplanResponse {
  pub ok: bool,
  pub moved: i32,
  pub unplaced: i32,
  pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SaveWeeklyGoalRequest {
  pub week_start_date: String,
  pub track_targets: Option<String>,
  pub note: Option<String>,
  pub reviewed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeeklyGoal {
  pub id: String,
  pub week_start_date: String,
  pub track_targets: Option<String>,
  pub note: Option<String>,
  pub reviewed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeekReviewResponse {
  pub ok: bool,
  pub week_start: String,
  pub goal: Option<WeeklyGoal>,
  pub planned: i32,
  pub completed: i32,
  pub skipped: i32,
  pub planned_minutes: i32,
  pub completed_minutes: i32,
  pub by_track: HashMap<String, TrackTally>,
  pub tasks: Vec<StudyTask>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TrackTally {
  pub planned: i32,
  pub completed: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverdueTasksResponse {
  pub ok: bool,
  pub tasks: Vec<StudyTask>,
  pub error: Option<String>,
}

// ---------------------------------------------------------------------------
// Repository helpers
// ---------------------------------------------------------------------------

fn row_to_task(row: &rusqlite::Row) -> rusqlite::Result<StudyTask> {
  Ok(StudyTask {
    id: row.get(0)?,
    daily_plan_id: row.get(1)?,
    kind: row.get(2)?,
    title: row.get(3)?,
    track: row.get(4)?,
    topic_id: row.get(5)?,
    problem_id: row.get(6)?,
    revision_item_id: row.get(7)?,
    estimate_minutes: row.get(8)?,
    actual_minutes: row.get(9)?,
    status: row.get(10)?,
    output: row.get(11)?,
    completed_at: row.get(12)?,
    position: row.get(13)?,
    scheduled_date: row.get(14)?,
  })
}

const TASK_COLUMNS: &str = "id, daily_plan_id, kind, title, track, topic_id, problem_id, revision_item_id, estimate_minutes, actual_minutes, status, output, completed_at, position, scheduled_date";

fn tasks_for_date(conn: &rusqlite::Connection, date: &str) -> Vec<StudyTask> {
  let mut stmt = match conn.prepare(&format!(
    "SELECT {TASK_COLUMNS} FROM study_tasks WHERE scheduled_date = ?1 ORDER BY position"
  )) {
    Ok(s) => s,
    Err(_) => return vec![],
  };
  stmt
    .query_map(params![date], row_to_task)
    .map(|rows| rows.flatten().collect())
    .unwrap_or_default()
}

fn load_task(conn: &rusqlite::Connection, id: &str) -> Option<StudyTask> {
  conn
    .query_row(
      &format!("SELECT {TASK_COLUMNS} FROM study_tasks WHERE id = ?1"),
      params![id],
      row_to_task,
    )
    .optional()
    .ok()
    .flatten()
}

fn ensure_daily_plan(
  conn: &rusqlite::Connection,
  date: &str,
  available: bool,
  capacity: i32,
) -> rusqlite::Result<String> {
  if let Some((id, manual)) = conn
    .query_row(
      "SELECT id, COALESCE(manual_override, 0) FROM daily_plans WHERE plan_date = ?1",
      params![date],
      |r| Ok((r.get::<_, String>(0)?, r.get::<_, i32>(1)?)),
    )
    .optional()?
  {
    // Auto-generated plans track current settings; manual overrides are kept.
    if manual == 0 {
      conn.execute(
        "UPDATE daily_plans SET available = ?1, capacity_minutes = ?2 WHERE id = ?3",
        params![available as i32, capacity, id],
      )?;
    }
    return Ok(id);
  }

  let id = Uuid::new_v4().to_string();
  conn.execute(
    "INSERT INTO daily_plans (id, plan_date, capacity_minutes, available, note, created_at) VALUES (?1,?2,?3,?4,NULL,?5)",
    params![id, date, capacity, available as i32, now_rfc3339()],
  )?;
  Ok(id)
}

/// Explicitly set a day's availability (used by the availability editor).
fn upsert_plan(
  conn: &rusqlite::Connection,
  date: &str,
  available: bool,
  capacity: i32,
  note: Option<String>,
) -> rusqlite::Result<String> {
  let existing: Option<String> = conn
    .query_row(
      "SELECT id FROM daily_plans WHERE plan_date = ?1",
      params![date],
      |r| r.get::<_, String>(0),
    )
    .optional()?;

  if let Some(id) = existing {
    conn.execute(
      "UPDATE daily_plans SET available = ?1, capacity_minutes = ?2, note = ?3, manual_override = 1 WHERE id = ?4",
      params![available as i32, capacity, note, id],
    )?;
    return Ok(id);
  }

  let id = Uuid::new_v4().to_string();
  conn.execute(
    "INSERT INTO daily_plans (id, plan_date, capacity_minutes, available, note, manual_override, created_at) VALUES (?1,?2,?3,?4,?5,1,?6)",
    params![id, date, capacity, available as i32, note, now_rfc3339()],
  )?;
  Ok(id)
}

fn load_plan(conn: &rusqlite::Connection, date: &str) -> Option<DailyPlan> {
  conn
    .query_row(
      "SELECT id, plan_date, capacity_minutes, available, note FROM daily_plans WHERE plan_date = ?1",
      params![date],
      |row| {
        Ok(DailyPlan {
          id: row.get(0)?,
          plan_date: row.get(1)?,
          capacity_minutes: row.get(2)?,
          available: row.get::<_, i32>(3)? != 0,
          note: row.get(4)?,
        })
      },
    )
    .optional()
    .ok()
    .flatten()
}

fn next_new_topic(
  conn: &rusqlite::Connection,
  track_key: &str,
) -> Option<(String, String, i32)> {
  conn
    .query_row(
      "SELECT t.id, t.title, COALESCE(t.estimate_minutes, 60)
       FROM topics t
       JOIN modules m ON t.module_id = m.id
       JOIN tracks tr ON m.track_id = tr.id
       WHERE tr.key = ?1
         AND t.status = 'Not started'
         AND t.id NOT IN (SELECT topic_id FROM study_tasks WHERE topic_id IS NOT NULL)
       ORDER BY m.position, LENGTH(t.key), t.key
       LIMIT 1",
      params![track_key],
      |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )
    .optional()
    .ok()
    .flatten()
}

fn day_load(conn: &rusqlite::Connection, date: &str) -> i32 {
  conn
    .query_row(
      "SELECT COALESCE(SUM(estimate_minutes), 0) FROM study_tasks WHERE scheduled_date = ?1 AND status != 'skipped'",
      params![date],
      |r| r.get::<_, i32>(0),
    )
    .unwrap_or(0)
}

fn insert_task(
  conn: &rusqlite::Connection,
  plan_id: &str,
  date: &str,
  kind: &str,
  title: &str,
  track: &str,
  topic_id: Option<&str>,
  revision_item_id: Option<&str>,
  problem_id: Option<&str>,
  estimate: i32,
  position: i32,
) -> rusqlite::Result<String> {
  let id = Uuid::new_v4().to_string();
  conn.execute(
    "INSERT INTO study_tasks (id, daily_plan_id, kind, title, track, topic_id, problem_id, revision_item_id, estimate_minutes, actual_minutes, status, output, completed_at, position, scheduled_date)
     VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,NULL,'planned',NULL,NULL,?10,?11)",
    params![
      id, plan_id, kind, title, track, topic_id, problem_id, revision_item_id, estimate, position, date
    ],
  )?;
  Ok(id)
}

fn difficulty_estimate_minutes(difficulty: &str) -> i32 {
  match difficulty {
    "Easy" => 30,
    "Medium" => 45,
    "Hard" => 60,
    _ => 45,
  }
}

// Schedule individual DSA problems within the current new-learning category
// (Tries onward) instead of one whole category topic per day. Picks as many
// problems as fit in the remaining capacity, always at least one when any fit.
fn schedule_dsa_problems(
  conn: &rusqlite::Connection,
  plan_id: &str,
  date_str: &str,
  position: &mut i32,
  remaining: &mut i32,
) -> rusqlite::Result<i32> {
  // Current new-learning category: lowest ordered category (track B topic key
  // B1..B18) that still has an unsolved problem not already on an open task.
  let category_order: Option<i32> = conn
    .query_row(
      "SELECT MIN(CAST(SUBSTR(t.key, 2) AS INTEGER))
       FROM problems p
       JOIN topics t ON t.title = p.category
       JOIN modules m ON t.module_id = m.id
       JOIN tracks tr ON m.track_id = tr.id AND tr.key = 'b'
       WHERE p.is_previously_covered = 0
         AND NOT EXISTS (
           SELECT 1 FROM problem_attempts a
           WHERE a.problem_id = p.id AND a.outcome = 'solved'
         )
         AND NOT EXISTS (
           SELECT 1 FROM study_tasks st
           WHERE st.problem_id = p.id AND st.status != 'done'
         )",
      [],
      |row| row.get(0),
    )
    .optional()
    .unwrap_or(None);

  let Some(co) = category_order else {
    return Ok(0);
  };

  let category_topic_id: Option<String> = conn
    .query_row(
      "SELECT id FROM topics WHERE key = ?1 LIMIT 1",
      params![format!("B{}", co)],
      |row| row.get(0),
    )
    .optional()
    .unwrap_or(None);

  // Problems already planned for this day count toward "at least one per day";
  // we only force a first problem into a small remaining slot when the day is
  // otherwise empty of DSA work.
  let existing_dsa: i32 = conn
    .query_row(
      "SELECT COUNT(*) FROM study_tasks
       WHERE scheduled_date = ?1 AND problem_id IS NOT NULL AND status != 'done'",
      params![date_str],
      |r| r.get(0),
    )
    .unwrap_or(0);

  let mut stmt = conn.prepare(
    "SELECT p.id, p.title, p.difficulty
     FROM problems p
     JOIN topics t ON t.title = p.category
     JOIN modules m ON t.module_id = m.id
     JOIN tracks tr ON m.track_id = tr.id AND tr.key = 'b'
     WHERE CAST(SUBSTR(t.key, 2) AS INTEGER) = ?1
       AND p.is_previously_covered = 0
       AND NOT EXISTS (
         SELECT 1 FROM problem_attempts a
         WHERE a.problem_id = p.id AND a.outcome = 'solved'
       )
       AND NOT EXISTS (
         SELECT 1 FROM study_tasks st
         WHERE st.problem_id = p.id AND st.status != 'done'
       )
     ORDER BY p.official_order",
  )?;
  let problems: Vec<(String, String, String)> = stmt
    .query_map(params![co], |row| {
      Ok((
        row.get::<_, String>(0)?,
        row.get::<_, String>(1)?,
        row.get::<_, String>(2)?,
      ))
    })?
    .flatten()
    .collect();

  let mut created = 0;
  let mut dsa_count = existing_dsa;
  for (pid, title, difficulty) in problems {
    let estimate = difficulty_estimate_minutes(&difficulty);
    let can_schedule = (dsa_count == 0 && *remaining >= 15) || *remaining >= estimate;
    if !can_schedule {
      break;
    }
    insert_task(
      conn,
      plan_id,
      date_str,
      "planned",
      &title,
      "b",
      category_topic_id.as_deref(),
      None,
      Some(&pid),
      estimate,
      *position,
    )?;
    created += 1;
    dsa_count += 1;
    *position += 1;
    *remaining -= estimate;
  }
  Ok(created)
}

fn record_problem_solved(
  conn: &rusqlite::Connection,
  problem_id: &str,
  attempt_date: &str,
  duration_minutes: i32,
  study_task_id: Option<&str>,
) -> rusqlite::Result<()> {
  let already_solved: i32 = conn
    .query_row(
      "SELECT COUNT(*) FROM problem_attempts
       WHERE problem_id = ?1 AND outcome = 'solved'",
      params![problem_id],
      |r| r.get(0),
    )
    .unwrap_or(0);
  if already_solved > 0 {
    return Ok(());
  }
  let attempt_id = Uuid::new_v4().to_string();
  let now = now_rfc3339();
  conn.execute(
    "INSERT INTO problem_attempts
     (id, problem_id, attempt_date, outcome, duration_minutes, notes, code_file, complexity_note,
      edge_cases, interview_explanation, created_at, study_task_id)
     VALUES (?1,?2,?3,'solved',?4,NULL,NULL,NULL,NULL,NULL,?5,?6)",
    params![attempt_id, problem_id, attempt_date, duration_minutes, now, study_task_id],
  )?;
  let existing: Option<String> = conn
    .query_row(
      "SELECT id FROM revision_items
       WHERE source_type = 'problem' AND source_id = ?1",
      params![problem_id],
      |r| r.get(0),
    )
    .optional()
    .unwrap_or(None);
  if existing.is_none() {
    let next_due = (chrono::Local::now().date_naive() + Duration::days(1))
      .format("%Y-%m-%d")
      .to_string();
    conn.execute(
      "INSERT INTO revision_items
       (id, source_type, source_id, initial_completion_date, last_reviewed_date,
        next_due_date, current_interval, repetition, state, recall_result,
        confidence, notes, created_at, updated_at)
       VALUES (?1,'problem',?2,?3,NULL,?4,1,1,'reviewing',NULL,NULL,NULL,?5,?5)",
      params![
        Uuid::new_v4().to_string(),
        problem_id,
        attempt_date,
        next_due,
        now
      ],
    )?;
  }
  Ok(())
}

fn clear_problem_solved_for_task(
  conn: &rusqlite::Connection,
  problem_id: &str,
  study_task_id: &str,
) -> rusqlite::Result<()> {
  conn.execute(
    "DELETE FROM problem_attempts
     WHERE study_task_id = ?1 AND outcome = 'solved'",
    params![study_task_id],
  )?;
  let remaining: i32 = conn
    .query_row(
      "SELECT COUNT(*) FROM problem_attempts
       WHERE problem_id = ?1 AND outcome = 'solved'",
      params![problem_id],
      |r| r.get(0),
    )
    .unwrap_or(0);
  if remaining == 0 {
    conn.execute(
      "DELETE FROM revision_items
       WHERE source_type = 'problem' AND source_id = ?1",
      params![problem_id],
    )?;
  }
  Ok(())
}

fn generate_day(
  conn: &rusqlite::Connection,
  date: NaiveDate,
  settings: &Settings,
  warnings: &mut Vec<String>,
) -> rusqlite::Result<i32> {
  let date_str = date.format("%Y-%m-%d").to_string();
  let iso = iso_weekday(date);
  let computed_available = is_available(settings, iso);
  let computed_capacity = if computed_available {
    per_day_capacity_minutes(settings)
  } else {
    0
  };
  let plan_id = ensure_daily_plan(conn, &date_str, computed_available, computed_capacity)?;

  // Respect a plan that already exists (e.g. a user-marked rest day).
  let (available, capacity) = match load_plan(conn, &date_str) {
    Some(plan) => (
      plan.available,
      plan.capacity_minutes.unwrap_or(computed_capacity),
    ),
    None => (computed_available, computed_capacity),
  };

  if !available {
    return Ok(0);
  }

  let usable = usable_capacity_minutes(capacity, settings).max(15);
  let tracks = cadence_for(settings, iso);
  let mut created = 0;
  let mut position = 0;
  // Account for tasks that survive regeneration (completed/skipped/custom work).
  let mut remaining = (usable - day_load(conn, &date_str)).max(0);
  if remaining < 15 {
    return Ok(0);
  }

  // Weekly review day.
  if tracks.iter().any(|t| t.eq_ignore_ascii_case("review")) {
    let est = 30.min(remaining).max(15);
    insert_task(
      conn,
      &plan_id,
      &date_str,
      "review",
      "Weekly review",
      "review",
      None,
      None,
      None,
      est,
      position,
    )?;
    created += 1;
    remaining -= est;
    position += 1;
  }

  let learning_tracks: Vec<&String> = tracks
    .iter()
    .filter(|t| !t.eq_ignore_ascii_case("review"))
    .collect();

  let mut primary_added = false;
  for handle in &learning_tracks {
    if remaining < 15 {
      break;
    }
    if let Some(track_key) = track_key_for_handle(handle) {
      // DSA is scheduled problem-by-problem within the current new-learning
      // category instead of as one whole category topic.
      if track_key == "b" {
        let added = schedule_dsa_problems(
          conn,
          &plan_id,
          &date_str,
          &mut position,
          &mut remaining,
        )?;
        if added > 0 {
          created += added;
          primary_added = true;
        }
        continue;
      }
      if let Some((topic_id, title, estimate)) = next_new_topic(conn, track_key) {
        let est = estimate.min(remaining).max(15);
        insert_task(
          conn,
          &plan_id,
          &date_str,
          "planned",
          &title,
          track_key,
          Some(&topic_id),
          None,
          None,
          est,
          position,
        )?;
        created += 1;
        position += 1;
        remaining -= est;
        primary_added = true;
      }
    }
  }

  // If the cadence track has no available content (e.g. DSA not imported yet),
  // fall back to another track that still has unstarted topics.
  if !primary_added && remaining >= 15 {
    for fallback in ["a", "c", "d", "e"] {
      if let Some((topic_id, title, estimate)) = next_new_topic(conn, fallback) {
        let est = estimate.min(remaining).max(15);
        insert_task(
          conn,
          &plan_id,
          &date_str,
          "planned",
          &title,
          fallback,
          Some(&topic_id),
          None,
          None,
          est,
          position,
        )?;
        created += 1;
        position += 1;
        remaining -= est;
        primary_added = true;
        break;
      }
    }
    if !primary_added {
      warnings.push(format!("No available topics to schedule on {date_str}."));
    }
  }

  // Due revisions (skip items already scheduled as a task).
  let mut due_stmt = conn.prepare(
    "SELECT r.id, r.source_id FROM revision_items r
     WHERE r.next_due_date <= ?1
       AND r.id NOT IN (SELECT revision_item_id FROM study_tasks WHERE revision_item_id IS NOT NULL)",
  )?;
  let due_items: Vec<(String, String)> = due_stmt
    .query_map(params![date_str], |row| {
      Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?
    .flatten()
    .collect();

  for (rev_id, source_id) in due_items {
    if remaining < 20 {
      break;
    }
    let est = 25.min(remaining).max(15);
    insert_task(
      conn,
      &plan_id,
      &date_str,
      "revision",
      &format!("Revision: {source_id}"),
      "revision",
      None,
      Some(&rev_id),
      None,
      est,
      position,
    )?;
    created += 1;
    position += 1;
    remaining -= est;
  }

  // Optional stretch only if there is meaningful capacity left.
  if remaining >= 45 {
    let stretch_handle = learning_tracks.first().map(|s| s.as_str()).unwrap_or("java");
    if let Some(track_key) = track_key_for_handle(stretch_handle) {
      if let Some((topic_id, title, _)) = next_new_topic(conn, track_key) {
        insert_task(
          conn,
          &plan_id,
          &date_str,
          "stretch",
          &title,
          track_key,
          Some(&topic_id),
          None,
          None,
          45.min(remaining),
          position,
        )?;
        created += 1;
      }
    }
  }

  Ok(created)
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

#[tauri::command(rename_all = "snake_case")]
pub async fn get_agenda(date: String) -> AgendaResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return AgendaResponse {
        ok: false,
        plan: None,
        tasks: vec![],
        due_revisions: 0,
        planned_minutes: 0,
        completed_minutes: 0,
        error: Some(e.to_string()),
      }
    }
  };

  let plan = load_plan(&conn, &date);
  let tasks = tasks_for_date(&conn, &date);
  let due_revisions: i32 = conn
    .query_row(
      "SELECT COUNT(*) FROM revision_items WHERE next_due_date <= ?1",
      params![date],
      |r| r.get(0),
    )
    .unwrap_or(0);
  let planned_minutes: i32 = tasks
    .iter()
    .filter(|t| t.status != "skipped")
    .map(|t| t.estimate_minutes)
    .sum();
  let completed_minutes: i32 = tasks
    .iter()
    .filter(|t| t.status == "done")
    .map(|t| t.actual_minutes.unwrap_or(t.estimate_minutes))
    .sum();

  AgendaResponse {
    ok: true,
    plan,
    tasks,
    due_revisions,
    planned_minutes,
    completed_minutes,
    error: None,
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn list_overdue_tasks(today: String) -> OverdueTasksResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return OverdueTasksResponse {
        ok: false,
        tasks: vec![],
        error: Some(e.to_string()),
      }
    }
  };
  let mut stmt = match conn.prepare(&format!(
    "SELECT {TASK_COLUMNS} FROM study_tasks
     WHERE scheduled_date < ?1 AND status IN ('planned', 'skipped')
     ORDER BY scheduled_date, position"
  )) {
    Ok(s) => s,
    Err(e) => {
      return OverdueTasksResponse {
        ok: false,
        tasks: vec![],
        error: Some(e.to_string()),
      }
    }
  };
  let tasks: Vec<StudyTask> = stmt
    .query_map(params![today], row_to_task)
    .map(|rows| rows.flatten().collect())
    .unwrap_or_default();
  OverdueTasksResponse {
    ok: true,
    tasks,
    error: None,
  }
}

#[tauri::command]
pub async fn generate_week(req: GenerateWeekRequest) -> GenerateWeekResponse {
  let start = match parse_date(&req.week_start) {
    Some(d) => d,
    None => {
      return GenerateWeekResponse {
        ok: false,
        tasks_created: 0,
        days_generated: 0,
        warnings: vec![],
        error: Some("Invalid week_start (expected YYYY-MM-DD)".to_string()),
      }
    }
  };
  if start.weekday() != Weekday::Mon {
    return GenerateWeekResponse {
      ok: false,
      tasks_created: 0,
      days_generated: 0,
      warnings: vec![],
      error: Some("week_start must be a Monday".to_string()),
    };
  }

  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return GenerateWeekResponse {
        ok: false,
        tasks_created: 0,
        days_generated: 0,
        warnings: vec![],
        error: Some(e.to_string()),
      }
    }
  };
  if let Err(e) = crate::migrations::ensure_schema(&mut conn) {
    return GenerateWeekResponse {
      ok: false,
      tasks_created: 0,
      days_generated: 0,
      warnings: vec![],
      error: Some(e.to_string()),
    };
  }
  let settings = load_settings(&conn);

  let tx = match conn.transaction() {
    Ok(t) => t,
    Err(e) => {
      return GenerateWeekResponse {
        ok: false,
        tasks_created: 0,
        days_generated: 0,
        warnings: vec![],
        error: Some(e.to_string()),
      }
    }
  };

  let mut warnings = Vec::new();
  let mut created = 0;
  let mut days_generated = 0;

  let local_today = chrono::Local::now().date_naive();
  for date in week_dates(start) {
    // Do not schedule work into days that have already passed.
    if date < local_today {
      continue;
    }
    let date_str = date.format("%Y-%m-%d").to_string();
    let existing: i32 = tx
      .query_row(
        "SELECT COUNT(*) FROM study_tasks WHERE scheduled_date = ?1",
        params![date_str],
        |r| r.get(0),
      )
      .unwrap_or(0);

    if existing > 0 {
      if !req.force {
        continue;
      }
      // Regenerate only auto-generated, not-yet-completed tasks.
      tx.execute(
        "DELETE FROM study_tasks WHERE scheduled_date = ?1 AND status = 'planned' AND kind IN ('planned','stretch','review')",
        params![date_str],
      )
      .ok();
    }

    match generate_day(&tx, date, &settings, &mut warnings) {
      Ok(count) => {
        created += count;
        days_generated += 1;
      }
      Err(e) => warnings.push(format!("Failed to generate {date_str}: {e}")),
    }
  }

  if let Err(e) = tx.commit() {
    return GenerateWeekResponse {
      ok: false,
      tasks_created: 0,
      days_generated: 0,
      warnings,
      error: Some(e.to_string()),
    };
  }

  GenerateWeekResponse {
    ok: true,
    tasks_created: created,
    days_generated,
    warnings,
    error: None,
  }
}

#[tauri::command]
pub async fn set_day_availability(req: SetAvailabilityRequest) -> UpdateTaskResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return UpdateTaskResponse {
        ok: false,
        task: None,
        error: Some(e.to_string()),
      }
    }
  };
  let _ = crate::migrations::ensure_schema(&mut conn);
  let settings = load_settings(&conn);
  let capacity = req
    .capacity_minutes
    .unwrap_or_else(|| if req.available { per_day_capacity_minutes(&settings) } else { 0 });

  match upsert_plan(
    &conn,
    &req.plan_date,
    req.available,
    capacity,
    req.note.clone(),
  ) {
    Ok(_) => UpdateTaskResponse {
      ok: true,
      task: None,
      error: None,
    },
    Err(e) => UpdateTaskResponse {
      ok: false,
      task: None,
      error: Some(e.to_string()),
    },
  }
}

#[tauri::command]
pub async fn update_study_task(req: UpdateTaskRequest) -> UpdateTaskResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return UpdateTaskResponse {
        ok: false,
        task: None,
        error: Some(e.to_string()),
      }
    }
  };

  let mut task = match load_task(&conn, &req.id) {
    Some(t) => t,
    None => {
      return UpdateTaskResponse {
        ok: false,
        task: None,
        error: Some("Task not found".to_string()),
      }
    }
  };

  let was_done = task.status == "done";

  if let Some(status) = &req.status {
    task.status = status.clone();
  }
  if let Some(actual) = req.actual_minutes {
    task.actual_minutes = Some(actual);
  }
  if let Some(output) = &req.output {
    task.output = Some(output.clone());
  }
  if let Some(new_date) = &req.scheduled_date {
    if parse_date(new_date).is_none() {
      return UpdateTaskResponse {
        ok: false,
        task: None,
        error: Some("Invalid scheduled_date".to_string()),
      };
    }
    task.scheduled_date = new_date.clone();
    task.daily_plan_id = ensure_daily_plan(&conn, new_date, true, per_day_capacity_minutes(&load_settings(&conn)))
      .ok();
  }

  let new_done = task.status == "done";
  let completed_at = if new_done { Some(now_rfc3339()) } else { None };

  if let Err(e) = conn.execute(
    "UPDATE study_tasks SET status = ?1, actual_minutes = ?2, output = ?3, completed_at = ?4, scheduled_date = ?5, daily_plan_id = ?6 WHERE id = ?7",
    params![
      task.status,
      task.actual_minutes,
      task.output,
      completed_at,
      task.scheduled_date,
      task.daily_plan_id,
      task.id
    ],
  ) {
    return UpdateTaskResponse {
      ok: false,
      task: None,
      error: Some(e.to_string()),
    };
  }
  task.completed_at = completed_at;

  if new_done {
    let minutes = task.actual_minutes.unwrap_or(task.estimate_minutes);
    let _ = conn.execute(
      "INSERT INTO activity_events (id, event_date, event_type, source_id, minutes, created_at) VALUES (?1,?2,'task_done',?3,?4,?5)",
      params![Uuid::new_v4().to_string(), task.scheduled_date, task.id, minutes, now_rfc3339()],
    );
    // Mark the topic as in progress (never auto-complete it).
    if let Some(topic_id) = &task.topic_id {
      let _ = conn.execute(
        "UPDATE topics SET status = 'Learning', updated_at = ?1 WHERE id = ?2 AND status = 'Not started'",
        params![now_rfc3339(), topic_id],
      );
    }

    // Record the completed DSA problem as solved so the scheduler advances to
    // the next problem/category. Never duplicate an existing solved attempt.
    if let Some(problem_id) = &task.problem_id {
      let _ = record_problem_solved(&conn, problem_id, &task.scheduled_date, minutes, Some(&task.id));
    }
  } else if was_done {
    // Undo: drop the completion event and release the topic back to Not started
    // unless another task for it is still done.
    let _ = conn.execute(
      "DELETE FROM activity_events WHERE source_id = ?1 AND event_type = 'task_done'",
      params![task.id],
    );
    // Remove the auto-created solved attempt for this task's problem (if any).
    if let Some(problem_id) = &task.problem_id {
      let _ = clear_problem_solved_for_task(&conn, problem_id, task.id.as_str());
    }
    if let Some(topic_id) = &task.topic_id {
      let remaining_done: i32 = conn
        .query_row(
          "SELECT COUNT(*) FROM study_tasks WHERE topic_id = ?1 AND status = 'done' AND id != ?2",
          params![topic_id, task.id],
          |r| r.get::<_, i32>(0),
        )
        .unwrap_or(0);
      if remaining_done == 0 {
        let _ = conn.execute(
          "UPDATE topics SET status = 'Not started', updated_at = ?1 WHERE id = ?2 AND status = 'Learning'",
          params![now_rfc3339(), topic_id],
        );
      }
    }
  }

  UpdateTaskResponse {
    ok: true,
    task: load_task(&conn, &req.id),
    error: None,
  }
}

#[tauri::command]
pub async fn create_study_task(req: CreateTaskRequest) -> CreateTaskResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return CreateTaskResponse {
        ok: false,
        id: None,
        error: Some(e.to_string()),
      }
    }
  };
  let _ = crate::migrations::ensure_schema(&mut conn);
  if parse_date(&req.scheduled_date).is_none() {
    return CreateTaskResponse {
      ok: false,
      id: None,
      error: Some("Invalid scheduled_date".to_string()),
    };
  }
  let settings = load_settings(&conn);
  let plan_id = match ensure_daily_plan(
    &conn,
    &req.scheduled_date,
    true,
    per_day_capacity_minutes(&settings),
  ) {
    Ok(id) => id,
    Err(e) => {
      return CreateTaskResponse {
        ok: false,
        id: None,
        error: Some(e.to_string()),
      }
    }
  };
  let position = tasks_for_date(&conn, &req.scheduled_date).len() as i32;
  let kind = req.kind.unwrap_or_else(|| "custom".to_string());
    match insert_task(
    &conn,
    &plan_id,
    &req.scheduled_date,
    &kind,
    &req.title,
    &req.track,
    req.topic_id.as_deref(),
    None,
    None,
    req.estimate_minutes.unwrap_or(30),
    position,
  ) {
    Ok(id) => CreateTaskResponse {
      ok: true,
      id: Some(id),
      error: None,
    },
    Err(e) => CreateTaskResponse {
      ok: false,
      id: None,
      error: Some(e.to_string()),
    },
  }
}

/// Redistribute unfinished work from dates before `today` across the next
/// `horizon_days` available days by remaining capacity. Returns
/// `(moved, unplaced)`. Skipped tasks are treated as missed (the user deferred
/// them) and are reset to `planned` when moved.
fn replan_missed(
  conn: &rusqlite::Connection,
  today: chrono::NaiveDate,
  horizon_days: i64,
  settings: &Settings,
) -> rusqlite::Result<(i32, i32)> {
  let capacity = per_day_capacity_minutes(settings);
  let horizon = horizon_days.clamp(1, 60);
  let mut days: Vec<String> = Vec::new();
  let mut load: HashMap<String, i32> = HashMap::new();
  for offset in 0..horizon {
    let date = today + Duration::days(offset);
    let date_str = date.format("%Y-%m-%d").to_string();
    let computed_available = is_available(settings, iso_weekday(date));
    let plan_capacity = if computed_available { capacity } else { 0 };
    let _ = ensure_daily_plan(conn, &date_str, computed_available, plan_capacity);
    // Respect an existing plan (a user may have marked the day unavailable).
    let available = load_plan(conn, &date_str)
      .map(|p| p.available)
      .unwrap_or(computed_available);
    if available {
      load.insert(date_str.clone(), day_load(conn, &date_str));
      days.push(date_str);
    }
  }

  let today_str = today.format("%Y-%m-%d").to_string();
  let overdue: Vec<(String, i32)> = {
    let mut stmt = conn.prepare(
      "SELECT id, estimate_minutes FROM study_tasks
       WHERE status IN ('planned', 'skipped') AND scheduled_date < ?1
       ORDER BY scheduled_date, position",
    )?;
    let rows = stmt.query_map(params![today_str], |row| {
      Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
    })?;
    rows.flatten().collect()
  };

  let mut moved = 0;
  let mut unplaced = 0;
  for (task_id, estimate) in overdue {
    let mut placed = false;
    for day in &days {
      let current = *load.get(day).unwrap_or(&0);
      if current + estimate <= capacity {
        conn.execute(
          "UPDATE study_tasks
           SET scheduled_date = ?1,
               daily_plan_id = (SELECT id FROM daily_plans WHERE plan_date = ?1),
               status = 'planned'
           WHERE id = ?2",
          params![day, task_id],
        )?;
        load.insert(day.clone(), current + estimate);
        moved += 1;
        placed = true;
        break;
      }
    }
    if !placed {
      unplaced += 1;
    }
  }
  Ok((moved, unplaced))
}

#[tauri::command]
pub async fn replan_backlog(req: ReplanRequest) -> ReplanResponse {
  let today = match parse_date(&req.today) {
    Some(d) => d,
    None => {
      return ReplanResponse {
        ok: false,
        moved: 0,
        unplaced: 0,
        error: Some("Invalid today (expected YYYY-MM-DD)".to_string()),
      }
    }
  };

  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return ReplanResponse {
        ok: false,
        moved: 0,
        unplaced: 0,
        error: Some(e.to_string()),
      }
    }
  };
  let _ = crate::migrations::ensure_schema(&mut conn);
  let settings = load_settings(&conn);

  let tx = match conn.transaction() {
    Ok(t) => t,
    Err(e) => {
      return ReplanResponse {
        ok: false,
        moved: 0,
        unplaced: 0,
        error: Some(e.to_string()),
      }
    }
  };

  let (moved, unplaced) = match replan_missed(&tx, today, req.horizon_days, &settings) {
    Ok(v) => v,
    Err(e) => {
      return ReplanResponse {
        ok: false,
        moved: 0,
        unplaced: 0,
        error: Some(e.to_string()),
      }
    }
  };

  if let Err(e) = tx.commit() {
    return ReplanResponse {
      ok: false,
      moved: 0,
      unplaced: 0,
      error: Some(e.to_string()),
    };
  }

  ReplanResponse {
    ok: true,
    moved,
    unplaced,
    error: None,
  }
}

#[tauri::command]
pub async fn save_weekly_goal(req: SaveWeeklyGoalRequest) -> UpdateTaskResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return UpdateTaskResponse {
        ok: false,
        task: None,
        error: Some(e.to_string()),
      }
    }
  };
  let _ = crate::migrations::ensure_schema(&mut conn);
  let reviewed = req.reviewed.unwrap_or(false) as i32;
  let result = conn.execute(
    "INSERT INTO weekly_goals (id, week_start_date, track_targets, note, reviewed, created_at)
     VALUES (?1,?2,?3,?4,?5,?6)
     ON CONFLICT(week_start_date) DO UPDATE SET
       track_targets = excluded.track_targets,
       note = excluded.note,
       reviewed = excluded.reviewed",
    params![
      Uuid::new_v4().to_string(),
      req.week_start_date,
      req.track_targets,
      req.note,
      reviewed,
      now_rfc3339()
    ],
  );
  match result {
    Ok(_) => UpdateTaskResponse {
      ok: true,
      task: None,
      error: None,
    },
    Err(e) => UpdateTaskResponse {
      ok: false,
      task: None,
      error: Some(e.to_string()),
    },
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_week_review(week_start: String) -> WeekReviewResponse {
  let start = match parse_date(&week_start) {
    Some(d) => d,
    None => {
      return WeekReviewResponse {
        ok: false,
        week_start,
        goal: None,
        planned: 0,
        completed: 0,
        skipped: 0,
        planned_minutes: 0,
        completed_minutes: 0,
        by_track: HashMap::new(),
        tasks: vec![],
        error: Some("Invalid week_start".to_string()),
      }
    }
  };
  let end = start + Duration::days(6);

  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return WeekReviewResponse {
        ok: false,
        week_start,
        goal: None,
        planned: 0,
        completed: 0,
        skipped: 0,
        planned_minutes: 0,
        completed_minutes: 0,
        by_track: HashMap::new(),
        tasks: vec![],
        error: Some(e.to_string()),
      }
    }
  };

  let goal = conn
    .query_row(
      "SELECT id, week_start_date, track_targets, note, reviewed FROM weekly_goals WHERE week_start_date = ?1",
      params![week_start],
      |row| {
        Ok(WeeklyGoal {
          id: row.get(0)?,
          week_start_date: row.get(1)?,
          track_targets: row.get(2)?,
          note: row.get(3)?,
          reviewed: row.get::<_, i32>(4)? != 0,
        })
      },
    )
    .optional()
    .ok()
    .flatten();

  let mut stmt = match conn.prepare(&format!(
    "SELECT {TASK_COLUMNS} FROM study_tasks WHERE scheduled_date BETWEEN ?1 AND ?2 ORDER BY scheduled_date, position"
  )) {
    Ok(s) => s,
    Err(e) => {
      return WeekReviewResponse {
        ok: false,
        week_start,
        goal,
        planned: 0,
        completed: 0,
        skipped: 0,
        planned_minutes: 0,
        completed_minutes: 0,
        by_track: HashMap::new(),
        tasks: vec![],
        error: Some(e.to_string()),
      }
    }
  };
  let tasks: Vec<StudyTask> = stmt
    .query_map(
      params![week_start, end.format("%Y-%m-%d").to_string()],
      row_to_task,
    )
    .map(|rows| rows.flatten().collect())
    .unwrap_or_default();

  let mut by_track: HashMap<String, TrackTally> = HashMap::new();
  let mut completed = 0;
  let mut skipped = 0;
  let mut completed_minutes = 0;
  let mut planned_minutes = 0;
  for task in &tasks {
    let tally = by_track.entry(task.track.clone()).or_default();
    tally.planned += 1;
    if task.status != "skipped" {
      planned_minutes += task.estimate_minutes;
    }
    if task.status == "done" {
      completed += 1;
      completed_minutes += task.actual_minutes.unwrap_or(task.estimate_minutes);
      tally.completed += 1;
    } else if task.status == "skipped" {
      skipped += 1;
    }
  }

  WeekReviewResponse {
    ok: true,
    week_start,
    goal,
    planned: tasks.len() as i32,
    completed,
    skipped,
    planned_minutes,
    completed_minutes,
    by_track,
    tasks,
    error: None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn date(s: &str) -> NaiveDate {
    parse_date(s).unwrap()
  }

  #[test]
  fn parses_and_rejects_dates() {
    assert_eq!(parse_date("2026-01-05"), Some(date("2026-01-05")));
    assert_eq!(parse_date("05-01-2026"), None);
  }

  #[test]
  fn week_dates_are_seven_consecutive_days() {
    let days = week_dates(date("2026-01-05"));
    assert_eq!(days.len(), 7);
    assert_eq!(days[0], date("2026-01-05"));
    assert_eq!(days[6], date("2026-01-11"));
  }

  #[test]
  fn default_cadence_matches_plan() {
    assert_eq!(default_cadence(1), vec!["dsa"]);
    assert_eq!(default_cadence(2), vec!["java"]);
    assert_eq!(default_cadence(4), vec!["lld"]);
    assert_eq!(default_cadence(6), vec!["fde"]);
    assert_eq!(default_cadence(7), vec!["review"]);
  }

  #[test]
  fn track_handles_map_to_letters() {
    assert_eq!(track_key_for_handle("java"), Some("a"));
    assert_eq!(track_key_for_handle("dsa"), Some("b"));
    assert_eq!(track_key_for_handle("lld"), Some("c"));
    assert_eq!(track_key_for_handle("hld"), Some("d"));
    assert_eq!(track_key_for_handle("fde"), Some("e"));
    assert_eq!(track_key_for_handle("review"), None);
  }

  #[test]
  fn capacity_is_divided_across_study_days() {
    let settings = Settings::default();
    // 7.5h/week across 6 study days = 75 minutes/day.
    assert_eq!(per_day_capacity_minutes(&settings), 75);
    // 15% buffer leaves ~64 usable minutes.
    assert_eq!(usable_capacity_minutes(75, &settings), 64);
  }

  #[test]
  fn custom_cadence_overrides_default() {
    let mut settings = Settings::default();
    let mut cadence = HashMap::new();
    cadence.insert("1".to_string(), vec!["java".to_string(), "lld".to_string()]);
    settings.cadence = Some(cadence);
    assert_eq!(cadence_for(&settings, 1), vec!["java", "lld"]);
    // Days without an override fall back to the default.
    assert_eq!(cadence_for(&settings, 2), vec!["java"]);
  }

  fn seeded_conn() -> rusqlite::Connection {
    let mut conn = rusqlite::Connection::open_in_memory().unwrap();
    conn.pragma_update(None, "foreign_keys", "ON").unwrap();
    crate::migrations::ensure_schema(&mut conn).unwrap();

    conn.execute(
      "INSERT INTO tracks (id, key, name, description, position, created_at) VALUES ('tr-a','a','Java',NULL,1,'now')",
      [],
    )
    .unwrap();
    conn.execute(
      "INSERT INTO modules (id, track_id, key, name, position) VALUES ('mod-a','tr-a','A','Module A',1)",
      [],
    )
    .unwrap();
    for (key, title) in [
      ("A1", "Topic one"),
      ("A2", "Topic two"),
      ("A10", "Topic ten"),
    ] {
      conn.execute(
        "INSERT INTO topics (id, module_id, key, title, status, priority, estimate_minutes, is_custom, created_at, updated_at)
         VALUES (?1, 'mod-a', ?2, ?3, 'Not started', 'High', 60, 0, 'now', 'now')",
        params![format!("topic-{key}"), key, title],
      )
      .unwrap();
    }
    conn
  }

  #[test]
  fn generation_picks_topics_without_duplicating_and_respects_rest_days() {
    let conn = seeded_conn();
    let settings = Settings::default();
    // 2026-01-05 is a Monday.
    let monday = date("2026-01-05");
    let mut warnings = Vec::new();

    let created = generate_day(&conn, monday, &settings, &mut warnings).unwrap();
    assert!(created >= 1, "expected at least one generated task");
    assert_eq!(tasks_for_date(&conn, "2026-01-05").len(), created as usize);

    // The generated topic must not be picked again.
    let next = next_new_topic(&conn, "a").expect("a second topic should remain");
    assert_eq!(next.0, "topic-A2");

    // A day explicitly marked unavailable gets no tasks.
    upsert_plan(&conn, "2026-01-06", false, 0, None).unwrap();
    let before = tasks_for_date(&conn, "2026-01-06").len();
    let created_rest = generate_day(&conn, date("2026-01-06"), &settings, &mut warnings).unwrap();
    assert_eq!(created_rest, 0);
    assert_eq!(tasks_for_date(&conn, "2026-01-06").len(), before);
  }

  #[test]
  fn regeneration_refreshes_auto_capacity_and_preserves_manual_override() {
    let conn = seeded_conn();
    let mut settings = Settings::default();
    settings.weekly_capacity_hours = Some(7.5);

    let id = ensure_daily_plan(
      &conn,
      "2026-01-05",
      true,
      per_day_capacity_minutes(&settings),
    )
    .unwrap();
    let cap1: i32 = conn
      .query_row(
        "SELECT capacity_minutes FROM daily_plans WHERE id = ?1",
        params![id],
        |r| r.get(0),
      )
      .unwrap();
    assert_eq!(cap1, 75, "7.5h/week over 6 study days = 75 min/day");

    // Changing weekly hours must refresh the auto-generated plan capacity.
    settings.weekly_capacity_hours = Some(12.0);
    ensure_daily_plan(
      &conn,
      "2026-01-05",
      true,
      per_day_capacity_minutes(&settings),
    )
    .unwrap();
    let cap2: i32 = conn
      .query_row(
        "SELECT capacity_minutes FROM daily_plans WHERE id = ?1",
        params![id],
        |r| r.get(0),
      )
      .unwrap();
    assert_eq!(cap2, 120, "12h/week over 6 study days = 120 min/day");

    // A manual override must not be overwritten by regeneration.
    upsert_plan(&conn, "2026-01-05", true, 90, None).unwrap();
    ensure_daily_plan(&conn, "2026-01-05", true, 45).unwrap();
    let cap3: i32 = conn
      .query_row(
        "SELECT capacity_minutes FROM daily_plans WHERE id = ?1",
        params![id],
        |r| r.get(0),
      )
      .unwrap();
    assert_eq!(cap3, 90);
  }

  #[test]
  fn next_new_topic_uses_natural_key_order() {
    let conn = seeded_conn();
    // A1 is first, not A10 (lexicographic would put A10 before A2).
    assert_eq!(next_new_topic(&conn, "a").unwrap().1, "Topic one");

    // Once A1 is already on a task, the next topic is A2 — not A10.
    let plan = ensure_daily_plan(&conn, "2026-01-05", true, 75).unwrap();
    insert_task(
      &conn,
      &plan,
      "2026-01-05",
      "planned",
      "Topic one",
      "a",
      Some("topic-A1"),
      None,
      None,
      60,
      0,
    )
    .unwrap();
    assert_eq!(next_new_topic(&conn, "a").unwrap().1, "Topic two");
  }

  #[test]
  fn replan_moves_overdue_planned_and_skipped_tasks() {
    let conn = seeded_conn();
    let settings = Settings::default();

    let old_plan = ensure_daily_plan(
      &conn,
      "2026-01-05",
      true,
      per_day_capacity_minutes(&settings),
    )
    .unwrap();
    let planned_id = insert_task(
      &conn,
      &old_plan,
      "2026-01-05",
      "planned",
      "Planned overdue",
      "a",
      Some("topic-A1"),
      None,
      None,
      30,
      0,
    )
    .unwrap();
    let skipped_id = insert_task(
      &conn,
      &old_plan,
      "2026-01-05",
      "planned",
      "Skipped overdue",
      "a",
      Some("topic-A2"),
      None,
      None,
      30,
      1,
    )
    .unwrap();
    conn.execute(
      "UPDATE study_tasks SET status = 'skipped' WHERE id = ?1",
      params![skipped_id],
    )
    .unwrap();

    let (moved, unplaced) =
      replan_missed(&conn, date("2026-01-06"), 14, &settings).unwrap();
    assert_eq!(moved, 2, "both overdue planned and skipped tasks should move");
    assert_eq!(unplaced, 0);

    for id in [&planned_id, &skipped_id] {
      let (status, new_date): (String, String) = conn
        .query_row(
          "SELECT status, scheduled_date FROM study_tasks WHERE id = ?1",
          params![id],
          |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
      assert_eq!(status, "planned", "replanned tasks return to planned");
      assert!(new_date.as_str() >= "2026-01-06");
    }
  }

  fn dsa_seeded_conn() -> rusqlite::Connection {
    let mut conn = rusqlite::Connection::open_in_memory().unwrap();
    conn.pragma_update(None, "foreign_keys", "ON").unwrap();
    crate::migrations::ensure_schema(&mut conn).unwrap();
    let now = "now";
    conn.execute(
      "INSERT INTO tracks (id, key, name, description, position, created_at) VALUES (?1,'b','DSA',NULL,2,?2)",
      params!["tr-b", now],
    ).unwrap();
    conn.execute(
      "INSERT INTO modules (id, track_id, key, name, position) VALUES (?1,'tr-b','B','NeetCode 250',1)",
      params!["mod-b"],
    ).unwrap();
    for (key, title) in [("B8", "Tries"), ("B9", "Backtracking")] {
      conn.execute(
        "INSERT INTO topics (id, module_id, key, title, status, priority, estimate_minutes, is_custom, created_at, updated_at)
         VALUES (?1,'mod-b',?2,?3,'Not started','Medium',NULL,0,'now','now')",
        params![format!("topic-{key}"), key, title],
      ).unwrap();
    }
    let problems = [
      ("p-1", "tries-1", "Implement Trie", "Tries", "Easy", 100),
      ("p-2", "tries-2", "Add and Search Word", "Tries", "Medium", 101),
      ("p-3", "tries-3", "Design Prefix", "Tries", "Hard", 102),
      ("p-4", "tries-4", "Longest Word", "Tries", "Easy", 103),
      ("p-5", "bt-1", "N-Queens", "Backtracking", "Easy", 200),
    ];
    for (id, ext, title, category, difficulty, order) in problems {
      conn.execute(
        "INSERT INTO problems (id, external_id, title, category, difficulty, url, official_order, statement, constraints, tags, is_previously_covered, created_at)
         VALUES (?1,?2,?3,?4,?5,?6,?7,NULL,NULL,NULL,0,'now')",
        params![id, ext, title, category, difficulty, format!("https://x/{ext}"), order],
      ).unwrap();
    }
    conn
  }

  #[test]
  fn dsa_schedules_individual_problems_in_category_order() {
    let conn = dsa_seeded_conn();
    let settings = Settings::default();
    let mut warnings = Vec::new();

    // Monday 2026-01-05 is a DSA day (cadence day 1 = dsa).
    let created = generate_day(&conn, date("2026-01-05"), &settings, &mut warnings).unwrap();
    assert!(created >= 1, "expected at least one DSA problem on a DSA day");
    let tasks = tasks_for_date(&conn, "2026-01-05");
    assert_eq!(tasks.len(), 1, "should schedule one problem, not the whole category");
    assert_eq!(tasks[0].track, "b");
    assert_eq!(tasks[0].title, "Implement Trie");
    assert_eq!(tasks[0].problem_id.as_deref(), Some("p-1"));

    // Medium (45 min) does not fit in remaining capacity (64 - 30 = 34) today.
    let created_wed = generate_day(&conn, date("2026-01-07"), &settings, &mut warnings).unwrap();
    let wed_tasks = tasks_for_date(&conn, "2026-01-07");
    assert_eq!(wed_tasks.len(), 1);
    assert_eq!(wed_tasks[0].title, "Add and Search Word");
    assert_eq!(wed_tasks[0].problem_id.as_deref(), Some("p-2"));

    // Re-generating the same day does not duplicate an open task.
    assert_eq!(
      generate_day(&conn, date("2026-01-07"), &settings, &mut warnings).unwrap(),
      0
    );
  }

  #[test]
  fn dsa_completion_advances_to_next_category() {
    let conn = dsa_seeded_conn();
    let settings = Settings::default();
    let mut warnings = Vec::new();

    // Schedule all three Tries problems over successive DSA days.
    generate_day(&conn, date("2026-01-05"), &settings, &mut warnings).unwrap();
    generate_day(&conn, date("2026-01-07"), &settings, &mut warnings).unwrap();
    generate_day(&conn, date("2026-01-09"), &settings, &mut warnings).unwrap();

    // Mark each Tries problem solved (simulates completing its task).
    for pid in ["p-1", "p-2", "p-3", "p-4"] {
      record_problem_solved(&conn, pid, "2026-01-09", 30, None).unwrap();
      // Calling twice must not create a duplicate solved attempt.
      record_problem_solved(&conn, pid, "2026-01-09", 30, None).unwrap();
      let solved: i32 = conn
        .query_row(
          "SELECT COUNT(*) FROM problem_attempts WHERE problem_id = ?1 AND outcome = 'solved'",
          params![pid],
          |r| r.get(0),
        )
        .unwrap();
      assert_eq!(solved, 1, "{pid} should have exactly one solved attempt");
      let rev: Option<String> = conn
        .query_row(
          "SELECT id FROM revision_items WHERE source_type = 'problem' AND source_id = ?1",
          params![pid],
          |r| r.get(0),
        )
        .optional()
        .unwrap();
      assert!(rev.is_some(), "{pid} should seed a revision item");
    }

    // All Tries problems solved → next DSA day moves to Backtracking.
    generate_day(&conn, date("2026-01-12"), &settings, &mut warnings).unwrap();
    let next_tasks = tasks_for_date(&conn, "2026-01-12");
    assert_eq!(next_tasks.len(), 1);
    assert_eq!(next_tasks[0].title, "N-Queens");
    assert_eq!(next_tasks[0].problem_id.as_deref(), Some("p-5"));
  }

  #[test]
  fn dsa_undo_removes_solved_attempt() {
    let conn = dsa_seeded_conn();
    let task_id = "task-p1";

    record_problem_solved(&conn, "p-1", "2026-01-05", 30, Some(task_id)).unwrap();
    let solved: i32 = conn
      .query_row(
        "SELECT COUNT(*) FROM problem_attempts WHERE problem_id = 'p-1' AND outcome = 'solved'",
        [],
        |r| r.get(0),
      )
      .unwrap();
    assert_eq!(solved, 1);
    let rev: Option<String> = conn
      .query_row(
        "SELECT id FROM revision_items WHERE source_type = 'problem' AND source_id = 'p-1'",
        [],
        |r| r.get(0),
      )
      .optional()
      .unwrap();
    assert!(rev.is_some());

    clear_problem_solved_for_task(&conn, "p-1", task_id).unwrap();
    let solved: i32 = conn
      .query_row(
        "SELECT COUNT(*) FROM problem_attempts WHERE problem_id = 'p-1' AND outcome = 'solved'",
        [],
        |r| r.get(0),
      )
      .unwrap();
    assert_eq!(solved, 0);
    let rev: Option<String> = conn
      .query_row(
        "SELECT id FROM revision_items WHERE source_type = 'problem' AND source_id = 'p-1'",
        [],
        |r| r.get(0),
      )
      .optional()
      .unwrap();
    assert!(rev.is_none());
  }
}
