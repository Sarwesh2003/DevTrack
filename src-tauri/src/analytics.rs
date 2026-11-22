use crate::db::{app_data_dir, open_connection};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// ---------------------------------------------------------------------------
// Search
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
  pub r#type: String,   // topic, problem, project, experiment, question, design, case_study, note, entry
  pub id: String,
  pub title: String,
  pub subtitle: Option<String>,
  pub track: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
  pub ok: bool,
  pub results: Vec<SearchResult>,
  pub error: Option<String>,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn search_all(query: String) -> SearchResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return SearchResponse { ok: false, results: vec![], error: Some(e.to_string()) },
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let keywords: Vec<String> = query
    .split_whitespace()
    .map(|w| w.to_lowercase())
    .filter(|w| !w.is_empty())
    .collect();
  if keywords.is_empty() {
    return SearchResponse { ok: true, results: vec![], error: None };
  }

  let mut found: HashMap<(String, String), SearchResult> = HashMap::new();

  let insert = |found: &mut HashMap<(String, String), SearchResult>, r: SearchResult| {
    found.entry((r.r#type.clone(), r.id.clone())).or_insert(r);
  };

  let run = |conn: &mut rusqlite::Connection,
             pattern: &str,
             sql: &str,
             found: &mut HashMap<(String, String), SearchResult>,
             map: fn(&rusqlite::Row<'_>) -> rusqlite::Result<SearchResult>| {
    if let Ok(mut stmt) = conn.prepare(sql) {
      if let Ok(rows) = stmt.query_map(params![pattern], map) {
        for r in rows.flatten() {
          insert(found, r);
        }
      }
    }
  };

  for kw in &keywords {
    let pattern = format!("%{}%", kw);

    // Topics (joined through modules -> tracks for track name)
    run(&mut conn, &pattern,
        "SELECT t.id, t.title, t.key, tr.name FROM topics t
         JOIN modules m ON t.module_id = m.id
         JOIN tracks tr ON m.track_id = tr.id
         WHERE LOWER(t.title) LIKE LOWER(?1) OR LOWER(t.key) LIKE LOWER(?1)
         ORDER BY tr.position, m.position, t.position LIMIT 50",
        &mut found,
        |row| {
          Ok(SearchResult {
            r#type: "topic".to_string(),
            id: row.get(0)?,
            title: row.get(1)?,
            subtitle: Some(format!("{} · {}", row.get::<_, Option<String>>(2)?.unwrap_or_default(), row.get::<_, String>(3)?)),
            track: Some(row.get(3)?),
          })
        });

    // Problems
    run(&mut conn, &pattern,
        "SELECT id, title, category, difficulty FROM problems WHERE LOWER(title) LIKE LOWER(?1) ORDER BY category_order LIMIT 50",
        &mut found,
        |row| {
          Ok(SearchResult {
            r#type: "problem".to_string(),
            id: row.get(0)?,
            title: row.get(1)?,
            subtitle: Some(format!("{} · {}", row.get::<_, String>(2)?, row.get::<_, String>(3)?)),
            track: None,
          })
        });

    // Projects
    run(&mut conn, &pattern,
        "SELECT id, name, track, slug FROM projects WHERE LOWER(name) LIKE LOWER(?1) ORDER BY updated_at DESC LIMIT 50",
        &mut found,
        |row| {
          Ok(SearchResult {
            r#type: "project".to_string(),
            id: row.get(0)?,
            title: row.get(1)?,
            subtitle: Some(format!("{} · {}", row.get::<_, String>(2)?, row.get::<_, String>(3)?)),
            track: Some(row.get(2)?),
          })
        });

    // Code experiments
    run(&mut conn, &pattern,
        "SELECT id, title, language FROM code_experiments WHERE LOWER(title) LIKE LOWER(?1) OR LOWER(code) LIKE LOWER(?1) ORDER BY updated_at DESC LIMIT 50",
        &mut found,
        |row| {
          Ok(SearchResult {
            r#type: "experiment".to_string(),
            id: row.get(0)?,
            title: row.get(1)?,
            subtitle: Some(row.get::<_, String>(2)?),
            track: None,
          })
        });

    // Recall questions
    run(&mut conn, &pattern,
        "SELECT id, question, difficulty FROM recall_questions WHERE LOWER(question) LIKE LOWER(?1) OR LOWER(answer) LIKE LOWER(?1) ORDER BY updated_at DESC LIMIT 50",
        &mut found,
        |row| {
          Ok(SearchResult {
            r#type: "question".to_string(),
            id: row.get(0)?,
            title: row.get(1)?,
            subtitle: Some(row.get::<_, String>(2)?),
            track: None,
          })
        });

    // Design artifacts
    run(&mut conn, &pattern,
        "SELECT id, title FROM design_artifacts WHERE LOWER(title) LIKE LOWER(?1) ORDER BY updated_at DESC LIMIT 50",
        &mut found,
        |row| {
          Ok(SearchResult {
            r#type: "design".to_string(),
            id: row.get(0)?,
            title: row.get(1)?,
            subtitle: Some("LLD design artifact".to_string()),
            track: None,
          })
        });

    // Case studies
    run(&mut conn, &pattern,
        "SELECT id, title FROM case_studies WHERE LOWER(title) LIKE LOWER(?1) ORDER BY updated_at DESC LIMIT 50",
        &mut found,
        |row| {
          Ok(SearchResult {
            r#type: "case_study".to_string(),
            id: row.get(0)?,
            title: row.get(1)?,
            subtitle: Some("HLD case study".to_string()),
            track: None,
          })
        });

    // Project notes
    run(&mut conn, &pattern,
        "SELECT id, title, kind, content FROM project_notes WHERE LOWER(title) LIKE LOWER(?1) OR LOWER(content) LIKE LOWER(?1) ORDER BY updated_at DESC LIMIT 50",
        &mut found,
        |row| {
          Ok(SearchResult {
            r#type: "note".to_string(),
            id: row.get(0)?,
            title: row.get(1)?,
            subtitle: Some(format!("{} · {}", row.get::<_, String>(2)?, "project note")),
            track: None,
          })
        });

    // Learning entries
    run(&mut conn, &pattern,
        "SELECT id, title, description FROM learning_entries WHERE LOWER(title) LIKE LOWER(?1) OR LOWER(description) LIKE LOWER(?1) ORDER BY entry_date DESC LIMIT 50",
        &mut found,
        |row| {
          Ok(SearchResult {
            r#type: "entry".to_string(),
            id: row.get(0)?,
            title: row.get(1)?,
            subtitle: Some(row.get::<_, Option<String>>(2)?.unwrap_or_default()),
            track: None,
          })
        });
  }

  let mut results: Vec<SearchResult> = found.into_values().collect();
  results.sort_by(|a, b| a.title.cmp(&b.title));
  results.truncate(100);

  SearchResponse { ok: true, results, error: None }
}

// ---------------------------------------------------------------------------
// Export
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportResponse {
  pub ok: bool,
  pub destination: String,
  pub error: Option<String>,
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
  fs::create_dir_all(dst)?;
  for entry in fs::read_dir(src)? {
    let entry = entry?;
    let src_path = entry.path();
    let dst_path = dst.join(entry.file_name());
    if src_path.is_dir() {
      copy_dir_all(&src_path, &dst_path)?;
    } else {
      fs::copy(&src_path, &dst_path)?;
    }
  }
  Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn export_all(destination: String) -> ExportResponse {
  let db_path = match crate::db::resolve_db_path() {
    Ok(p) => p,
    Err(e) => return ExportResponse { ok: false, destination, error: Some(e.to_string()) },
  };
  if !db_path.exists() {
    return ExportResponse { ok: false, destination, error: Some("database does not exist".to_string()) };
  }

  let dest = Path::new(&destination);
  if let Some(parent) = dest.parent() {
    if let Err(e) = fs::create_dir_all(parent) {
      return ExportResponse { ok: false, destination, error: Some(e.to_string()) };
    }
  }

  // Copy the SQLite database file.
  if let Err(e) = fs::copy(&db_path, dest.join("devtrack.sqlite")) {
    return ExportResponse { ok: false, destination, error: Some(e.to_string()) };
  }

  // Copy the entire content directory (Markdown, Java, Mermaid, JSON files).
  let data_dir = match app_data_dir() {
    Ok(d) => d,
    Err(e) => return ExportResponse { ok: false, destination, error: Some(e.to_string()) },
  };
  let content_dir = data_dir.join("content");
  if content_dir.exists() {
    if let Err(e) = copy_dir_all(&content_dir, &dest.join("content")) {
      return ExportResponse { ok: false, destination, error: Some(e.to_string()) };
    }
  }

  ExportResponse { ok: true, destination, error: None }
}

// ---------------------------------------------------------------------------
// Analytics
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackProgress {
  pub track: String,
  pub total_topics: i64,
  pub completed: i64,
  pub in_progress: i64,
  pub not_started: i64,
  pub avg_confidence: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedVsActual {
  pub planned_minutes: i64,
  pub actual_minutes: i64,
  pub task_count: i64,
  pub completed_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevisionStats {
  pub total_items: i64,
  pub due_count: i64,
  pub overdue_count: i64,
  pub avg_confidence: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttemptStats {
  pub total: i64,
  pub solved: i64,
  pub avg_duration_minutes: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityStats {
  pub total_events: i64,
  pub active_days_last30: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsResponse {
  pub ok: bool,
  pub track_progress: Vec<TrackProgress>,
  pub planned_vs_actual: PlannedVsActual,
  pub revision: RevisionStats,
  pub attempts: AttemptStats,
  pub activity: ActivityStats,
  pub error: Option<String>,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_analytics() -> AnalyticsResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return empty_analytics(Some(e.to_string())),
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  // Track progress via topics -> modules -> tracks.
  let track_progress = match conn.prepare(
    "SELECT tr.name,
            COUNT(t.id),
            SUM(CASE WHEN t.status = 'Done' THEN 1 ELSE 0 END),
            SUM(CASE WHEN t.status = 'In Progress' THEN 1 ELSE 0 END),
            SUM(CASE WHEN t.status = 'Not Started' THEN 1 ELSE 0 END),
            AVG(CASE WHEN t.confidence IS NOT NULL THEN t.confidence END)
     FROM topics t
     JOIN modules m ON t.module_id = m.id
     JOIN tracks tr ON m.track_id = tr.id
     GROUP BY tr.id, tr.name
     ORDER BY tr.position",
  ) {
    Ok(mut stmt) => stmt
      .query_map([], |row| {
        Ok(TrackProgress {
          track: row.get(0)?,
          total_topics: row.get(1)?,
          completed: row.get::<_, Option<i64>>(2)?.unwrap_or(0),
          in_progress: row.get::<_, Option<i64>>(3)?.unwrap_or(0),
          not_started: row.get::<_, Option<i64>>(4)?.unwrap_or(0),
          avg_confidence: row.get(5)?,
        })
      })
      .map(|rows| rows.flatten().collect())
      .unwrap_or_default(),
    Err(_) => vec![],
  };

  // Planned vs actual from study_tasks.
  let planned_vs_actual = match conn.prepare(
    "SELECT COALESCE(SUM(estimate_minutes), 0),
            COALESCE(SUM(actual_minutes), 0),
            COUNT(*),
            SUM(CASE WHEN status = 'completed' THEN 1 ELSE 0 END)
     FROM study_tasks",
  ) {
    Ok(mut stmt) => match stmt.query_row([], |row| {
      Ok(PlannedVsActual {
        planned_minutes: row.get(0)?,
        actual_minutes: row.get(1)?,
        task_count: row.get(2)?,
        completed_count: row.get::<_, Option<i64>>(3)?.unwrap_or(0),
      })
    }) {
      Ok(v) => v,
      Err(_) => PlannedVsActual { planned_minutes: 0, actual_minutes: 0, task_count: 0, completed_count: 0 },
    },
    Err(_) => PlannedVsActual { planned_minutes: 0, actual_minutes: 0, task_count: 0, completed_count: 0 },
  };

  // Revision stats.
  let today = chrono::Utc::now().date_naive().format("%Y-%m-%d").to_string();
  let revision = match conn.prepare(
    "SELECT COUNT(*),
            SUM(CASE WHEN date(next_due_date) <= date(?1) THEN 1 ELSE 0 END),
            SUM(CASE WHEN date(next_due_date) < date(?1) THEN 1 ELSE 0 END),
            AVG(CASE WHEN confidence IS NOT NULL THEN confidence END)
     FROM revision_items",
  ) {
    Ok(mut stmt) => match stmt.query_row(params![&today], |row| {
      Ok(RevisionStats {
        total_items: row.get(0)?,
        due_count: row.get::<_, Option<i64>>(1)?.unwrap_or(0),
        overdue_count: row.get::<_, Option<i64>>(2)?.unwrap_or(0),
        avg_confidence: row.get(3)?,
      })
    }) {
      Ok(v) => v,
      Err(_) => RevisionStats { total_items: 0, due_count: 0, overdue_count: 0, avg_confidence: None },
    },
    Err(_) => RevisionStats { total_items: 0, due_count: 0, overdue_count: 0, avg_confidence: None },
  };

  // Attempt stats.
  let attempts = match conn.prepare(
    "SELECT COUNT(*),
            SUM(CASE WHEN outcome = 'solved' THEN 1 ELSE 0 END),
            AVG(CASE WHEN duration_minutes IS NOT NULL THEN duration_minutes END)
     FROM problem_attempts",
  ) {
    Ok(mut stmt) => match stmt.query_row([], |row| {
      Ok(AttemptStats {
        total: row.get(0)?,
        solved: row.get::<_, Option<i64>>(1)?.unwrap_or(0),
        avg_duration_minutes: row.get(2)?,
      })
    }) {
      Ok(v) => v,
      Err(_) => AttemptStats { total: 0, solved: 0, avg_duration_minutes: None },
    },
    Err(_) => AttemptStats { total: 0, solved: 0, avg_duration_minutes: None },
  };

  // Activity stats: last 30 days.
  let thirty_days_ago = chrono::Utc::now()
    .date_naive()
    .checked_sub_signed(chrono::Duration::days(30))
    .map(|d| d.format("%Y-%m-%d").to_string())
    .unwrap_or_default();
  let activity = match conn.prepare(
    "SELECT COUNT(*),
            COUNT(DISTINCT event_date)
     FROM activity_events
     WHERE event_date >= ?1",
  ) {
    Ok(mut stmt) => match stmt.query_row(params![&thirty_days_ago], |row| {
      Ok(ActivityStats {
        total_events: row.get(0)?,
        active_days_last30: row.get(1)?,
      })
    }) {
      Ok(v) => v,
      Err(_) => ActivityStats { total_events: 0, active_days_last30: 0 },
    },
    Err(_) => ActivityStats { total_events: 0, active_days_last30: 0 },
  };

  AnalyticsResponse {
    ok: true,
    track_progress,
    planned_vs_actual,
    revision,
    attempts,
    activity,
    error: None,
  }
}

fn empty_analytics(error: Option<String>) -> AnalyticsResponse {
  AnalyticsResponse {
    ok: false,
    track_progress: vec![],
    planned_vs_actual: PlannedVsActual { planned_minutes: 0, actual_minutes: 0, task_count: 0, completed_count: 0 },
    revision: RevisionStats { total_items: 0, due_count: 0, overdue_count: 0, avg_confidence: None },
    attempts: AttemptStats { total: 0, solved: 0, avg_duration_minutes: None },
    activity: ActivityStats { total_events: 0, active_days_last30: 0 },
    error,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn search_results_are_deduped_and_truncated() {
    let mut results: Vec<SearchResult> = vec![
      SearchResult { r#type: "topic".into(), id: "a".into(), title: "Zebra".into(), subtitle: None, track: None },
      SearchResult { r#type: "topic".into(), id: "a".into(), title: "Zebra".into(), subtitle: None, track: None },
      SearchResult { r#type: "topic".into(), id: "b".into(), title: "Alpha".into(), subtitle: None, track: None },
    ];
    results.sort_by(|a, b| a.title.cmp(&b.title));
    results.dedup_by(|a, b| a.id == b.id && a.r#type == b.r#type);
    results.truncate(100);
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].title, "Alpha");
    assert_eq!(results[1].title, "Zebra");
  }
}
