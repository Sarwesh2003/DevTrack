use crate::db::{load_settings, now_rfc3339, open_connection};
use chrono::{Duration, NaiveDate};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Pure helpers (unit tested)
// ---------------------------------------------------------------------------

/// Current streak ending today (or yesterday, so an unfinished today does not
/// immediately break the streak).
pub fn compute_current_streak(active: &BTreeSet<NaiveDate>, today: NaiveDate) -> i32 {
  let mut cursor = today;
  if !active.contains(&cursor) {
    cursor = today - Duration::days(1);
    if !active.contains(&cursor) {
      return 0;
    }
  }
  let mut streak = 0;
  while active.contains(&cursor) {
    streak += 1;
    cursor -= Duration::days(1);
  }
  streak
}

pub fn compute_longest_streak(active: &BTreeSet<NaiveDate>) -> i32 {
  let mut longest = 0;
  let mut run = 0;
  let mut previous: Option<NaiveDate> = None;
  for day in active {
    run = match previous {
      Some(prev) if *day == prev + Duration::days(1) => run + 1,
      _ => 1,
    };
    longest = longest.max(run);
    previous = Some(*day);
  }
  longest
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LearningEntry {
  pub id: String,
  pub title: String,
  pub entry_date: String,
  pub track: String,
  pub description: String,
  pub notes: Option<String>,
  pub resource_url: Option<String>,
  pub duration_minutes: Option<i32>,
  pub output_file: Option<String>,
  pub tags: Option<String>,
  pub related_topic_id: Option<String>,
  pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateLearningEntryRequest {
  pub title: String,
  pub entry_date: String,
  pub track: String,
  pub description: String,
  pub notes: Option<String>,
  pub resource_url: Option<String>,
  pub duration_minutes: Option<i32>,
  pub output_file: Option<String>,
  pub tags: Option<String>,
  pub related_topic_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLearningEntryResponse {
  pub ok: bool,
  pub id: Option<String>,
  pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListEntriesRequest {
  pub from: String,
  pub to: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListEntriesResponse {
  pub ok: bool,
  pub entries: Vec<LearningEntry>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityDay {
  pub date: String,
  pub minutes: i32,
  pub events: i32,
}

#[derive(Debug, Deserialize)]
pub struct ActivitySummaryRequest {
  pub from: String,
  pub to: String,
  pub today: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivitySummaryResponse {
  pub ok: bool,
  pub days: Vec<ActivityDay>,
  pub current_streak: i32,
  pub longest_streak: i32,
  pub total_minutes: i32,
  pub active_days: i32,
  pub min_meaningful_minutes: i32,
  pub error: Option<String>,
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn create_learning_entry(req: CreateLearningEntryRequest) -> CreateLearningEntryResponse {
  if req.title.trim().is_empty() {
    return CreateLearningEntryResponse {
      ok: false,
      id: None,
      error: Some("Title is required".to_string()),
    };
  }
  if NaiveDate::parse_from_str(&req.entry_date, "%Y-%m-%d").is_err() {
    return CreateLearningEntryResponse {
      ok: false,
      id: None,
      error: Some("Invalid entry_date (expected YYYY-MM-DD)".to_string()),
    };
  }

  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return CreateLearningEntryResponse {
        ok: false,
        id: None,
        error: Some(e.to_string()),
      }
    }
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let id = Uuid::new_v4().to_string();
  let now = now_rfc3339();
  let insert = conn.execute(
    "INSERT INTO learning_entries (id, title, entry_date, track, description, notes, resource_url, duration_minutes, output_file, tags, related_topic_id, related_problem_id, links_to_topic_id, created_at)
     VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,NULL,NULL,?12)",
    params![
      id,
      req.title,
      req.entry_date,
      req.track,
      req.description,
      req.notes,
      req.resource_url,
      req.duration_minutes,
      req.output_file,
      req.tags,
      req.related_topic_id,
      now,
    ],
  );
  if let Err(e) = insert {
    return CreateLearningEntryResponse {
      ok: false,
      id: None,
      error: Some(e.to_string()),
    };
  }

  let minutes = req.duration_minutes.unwrap_or(0);
  let _ = conn.execute(
    "INSERT INTO activity_events (id, event_date, event_type, source_id, minutes, created_at) VALUES (?1,?2,'learning_entry',?3,?4,?5)",
    params![Uuid::new_v4().to_string(), req.entry_date, id, minutes, now_rfc3339()],
  );

  CreateLearningEntryResponse {
    ok: true,
    id: Some(id),
    error: None,
  }
}

#[tauri::command]
pub async fn list_learning_entries(req: ListEntriesRequest) -> ListEntriesResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return ListEntriesResponse {
        ok: false,
        entries: vec![],
        error: Some(e.to_string()),
      }
    }
  };

  let mut stmt = match conn.prepare(
    "SELECT id, title, entry_date, track, description, notes, resource_url, duration_minutes, output_file, tags, related_topic_id, created_at
     FROM learning_entries WHERE entry_date BETWEEN ?1 AND ?2 ORDER BY entry_date DESC, created_at DESC",
  ) {
    Ok(s) => s,
    Err(e) => {
      return ListEntriesResponse {
        ok: false,
        entries: vec![],
        error: Some(e.to_string()),
      }
    }
  };

  let entries: Vec<LearningEntry> = stmt
    .query_map(params![req.from, req.to], |row| {
      Ok(LearningEntry {
        id: row.get(0)?,
        title: row.get(1)?,
        entry_date: row.get(2)?,
        track: row.get(3)?,
        description: row.get(4)?,
        notes: row.get(5)?,
        resource_url: row.get(6)?,
        duration_minutes: row.get(7)?,
        output_file: row.get(8)?,
        tags: row.get(9)?,
        related_topic_id: row.get(10)?,
        created_at: row.get(11)?,
      })
    })
    .map(|rows| rows.flatten().collect())
    .unwrap_or_default();

  ListEntriesResponse {
    ok: true,
    entries,
    error: None,
  }
}

#[tauri::command]
pub async fn get_activity_summary(req: ActivitySummaryRequest) -> ActivitySummaryResponse {
  let today = match NaiveDate::parse_from_str(&req.today, "%Y-%m-%d") {
    Ok(d) => d,
    Err(_) => {
      return ActivitySummaryResponse {
        ok: false,
        days: vec![],
        current_streak: 0,
        longest_streak: 0,
        total_minutes: 0,
        active_days: 0,
        min_meaningful_minutes: 0,
        error: Some("Invalid today".to_string()),
      }
    }
  };

  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return ActivitySummaryResponse {
        ok: false,
        days: vec![],
        current_streak: 0,
        longest_streak: 0,
        total_minutes: 0,
        active_days: 0,
        min_meaningful_minutes: 0,
        error: Some(e.to_string()),
      }
    }
  };
  let settings = load_settings(&conn);
  let min_meaningful = settings.min_meaningful_minutes.unwrap_or(15).max(0);

  let mut stmt = match conn.prepare(
    "SELECT event_date, COALESCE(SUM(minutes), 0), COUNT(*)
     FROM activity_events WHERE event_date BETWEEN ?1 AND ?2 GROUP BY event_date ORDER BY event_date",
  ) {
    Ok(s) => s,
    Err(e) => {
      return ActivitySummaryResponse {
        ok: false,
        days: vec![],
        current_streak: 0,
        longest_streak: 0,
        total_minutes: 0,
        active_days: 0,
        min_meaningful_minutes: min_meaningful,
        error: Some(e.to_string()),
      }
    }
  };

  let days: Vec<ActivityDay> = stmt
    .query_map(params![req.from, req.to], |row| {
      Ok(ActivityDay {
        date: row.get(0)?,
        minutes: row.get(1)?,
        events: row.get(2)?,
      })
    })
    .map(|rows| rows.flatten().collect())
    .unwrap_or_default();

  let mut active: BTreeSet<NaiveDate> = BTreeSet::new();
  let mut total_minutes = 0;
  for day in &days {
    total_minutes += day.minutes;
    if day.minutes >= min_meaningful {
      if let Ok(date) = NaiveDate::parse_from_str(&day.date, "%Y-%m-%d") {
        active.insert(date);
      }
    }
  }

  ActivitySummaryResponse {
    ok: true,
    current_streak: compute_current_streak(&active, today),
    longest_streak: compute_longest_streak(&active),
    total_minutes,
    active_days: active.len() as i32,
    days,
    min_meaningful_minutes: min_meaningful,
    error: None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn d(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
  }

  fn set(dates: &[&str]) -> BTreeSet<NaiveDate> {
    dates.iter().map(|s| d(s)).collect()
  }

  #[test]
  fn streak_counts_consecutive_days_ending_today() {
    let active = set(&["2026-01-05", "2026-01-06", "2026-01-07"]);
    assert_eq!(compute_current_streak(&active, d("2026-01-07")), 3);
  }

  #[test]
  fn streak_survives_an_unfinished_today_if_yesterday_active() {
    let active = set(&["2026-01-05", "2026-01-06"]);
    assert_eq!(compute_current_streak(&active, d("2026-01-07")), 2);
  }

  #[test]
  fn streak_is_zero_when_recent_days_inactive() {
    let active = set(&["2026-01-01"]);
    assert_eq!(compute_current_streak(&active, d("2026-01-07")), 0);
  }

  #[test]
  fn longest_streak_finds_best_run() {
    let active = set(&["2026-01-01", "2026-01-02", "2026-01-05", "2026-01-06", "2026-01-07"]);
    assert_eq!(compute_longest_streak(&active), 3);
  }

  #[test]
  fn empty_activity_has_zero_streaks() {
    let active = set(&[]);
    assert_eq!(compute_current_streak(&active, d("2026-01-07")), 0);
    assert_eq!(compute_longest_streak(&active), 0);
  }
}
