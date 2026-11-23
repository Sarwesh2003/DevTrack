use crate::db::{now_rfc3339, open_connection};
use chrono::{Duration, Local, NaiveDate};
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// CSV parsing (pure, unit tested)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NeetcodeRow {
  pub official_order: i32,
  pub problem_id: String,
  pub title: String,
  pub url: String,
  pub category: String,
  pub difficulty: String,
  pub category_order: i32,
  pub source_name: String,
  pub source_version: String,
}

#[derive(Debug, Serialize)]
pub struct PreviewNeetcodeResponse {
  pub ok: bool,
  pub total_rows: i32,
  pub valid_rows: i32,
  pub previously_covered: i32,
  pub new_learning: i32,
  pub by_category: Vec<CategoryCount>,
  pub sample: Vec<NeetcodeRow>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CategoryCount {
  pub category: String,
  pub count: i32,
  pub previously_covered: bool,
}

fn parse_csv_line(line: &str) -> Vec<String> {
  let mut fields = Vec::new();
  let mut current = String::new();
  let mut in_quotes = false;
  let mut chars = line.chars().peekable();
  while let Some(ch) = chars.next() {
    if in_quotes {
      if ch == '"' {
        if chars.peek() == Some(&'"') {
          current.push('"');
          chars.next();
        } else {
          in_quotes = false;
        }
      } else {
        current.push(ch);
      }
    } else if ch == '"' {
      in_quotes = true;
    } else if ch == ',' {
      fields.push(current.clone());
      current.clear();
    } else {
      current.push(ch);
    }
  }
  fields.push(current);
  fields
}

pub fn parse_neetcode_csv(content: &str) -> Result<Vec<NeetcodeRow>, String> {
  let mut rows: Vec<NeetcodeRow> = Vec::new();
  let mut header_seen = false;
  for (idx, line) in content.lines().enumerate() {
    let trimmed = line.trim();
    if trimmed.is_empty() {
      continue;
    }
    if !header_seen {
      header_seen = true;
      continue;
    }
    let fields = parse_csv_line(trimmed);
    if fields.len() < 9 {
      return Err(format!("Line {idx}: expected 9 fields, got {}", fields.len()));
    }
    let official_order: i32 = fields[0]
      .parse()
      .map_err(|_| format!("Line {idx}: invalid official_order '{}'", fields[0]))?;
    let category_order: i32 = fields[6]
      .parse()
      .map_err(|_| format!("Line {idx}: invalid category_order '{}'", fields[6]))?;
    rows.push(NeetcodeRow {
      official_order,
      problem_id: fields[1].clone(),
      title: fields[2].clone(),
      url: fields[3].clone(),
      category: fields[4].clone(),
      difficulty: fields[5].clone(),
      category_order,
      source_name: fields[7].clone(),
      source_version: fields[8].clone(),
    });
  }
  if rows.is_empty() {
    return Err("No data rows found in CSV".to_string());
  }
  Ok(rows)
}

fn is_previously_covered(category_order: i32) -> bool {
  // Categories through Trees (category_order <= 7) are revision-only.
  // New learning starts at Tries (category_order >= 8).
  category_order <= 7
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Problem {
  pub id: String,
  pub external_id: String,
  pub title: String,
  pub category: String,
  pub difficulty: String,
  pub url: String,
  pub official_order: i32,
  pub is_previously_covered: bool,
  pub tags: Vec<String>,
  pub is_favorite: bool,
  pub attempt_count: i32,
  pub solved: bool,
}

#[derive(Debug, Serialize)]
pub struct ProblemListResponse {
  pub ok: bool,
  pub problems: Vec<Problem>,
  pub total: i32,
  pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetProblemsRequest {
  pub category: Option<String>,
  pub difficulty: Option<String>,
  pub search: Option<String>,
  #[serde(default = "default_cover_filter")]
  pub cover_filter: String,
}

fn default_cover_filter() -> String {
  "all".to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attempt {
  pub id: String,
  pub problem_id: String,
  pub attempt_date: String,
  pub outcome: String,
  pub duration_minutes: Option<i32>,
  pub notes: Option<String>,
  pub code_file: Option<String>,
  pub complexity_note: Option<String>,
  pub edge_cases: Option<String>,
  pub interview_explanation: Option<String>,
  pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct RecordAttemptRequest {
  pub problem_id: String,
  pub attempt_date: String,
  pub outcome: String,
  pub duration_minutes: Option<i32>,
  pub notes: Option<String>,
  pub code_file: Option<String>,
  pub complexity_note: Option<String>,
  pub edge_cases: Option<String>,
  pub interview_explanation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordAttemptResponse {
  pub ok: bool,
  pub id: Option<String>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListAttemptsResponse {
  pub ok: bool,
  pub attempts: Vec<Attempt>,
  pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SaveSolutionRequest {
  pub problem_id: String,
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveSolutionResponse {
  pub ok: bool,
  pub file_path: Option<String>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Solution {
  pub id: String,
  pub problem_id: String,
  pub file_path: String,
  pub content: Option<String>,
  pub language: String,
  pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct GetSolutionResponse {
  pub ok: bool,
  pub solution: Option<Solution>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RevisionItemView {
  pub id: String,
  pub source_type: String,
  pub source_id: String,
  pub initial_completion_date: Option<String>,
  pub last_reviewed_date: Option<String>,
  pub next_due_date: String,
  pub current_interval: i32,
  pub repetition: i32,
  pub state: String,
  pub recall_result: Option<String>,
  pub confidence: Option<i32>,
  pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RecordReviewRequest {
  pub revision_item_id: String,
  pub reviewed_at: String,
  pub recall_result: String,
  pub confidence: Option<i32>,
  pub notes: Option<String>,
  pub evidence: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordReviewResponse {
  pub ok: bool,
  pub revision_item: Option<RevisionItemView>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetRevisionQueueResponse {
  pub ok: bool,
  pub items: Vec<RevisionItemView>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartRevisionBaselineResponse {
  pub ok: bool,
  pub created: i32,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CategoryProgress {
  pub category: String,
  pub total: i32,
  pub attempted: i32,
  pub solved: i32,
  pub previously_covered: bool,
}

#[derive(Debug, Serialize)]
pub struct GetDsaProgressResponse {
  pub ok: bool,
  pub categories: Vec<CategoryProgress>,
  pub total_problems: i32,
  pub total_attempted: i32,
  pub total_solved: i32,
  pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImportNeetcodeRequest {
  pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ImportNeetcodeResponse {
  pub ok: bool,
  pub imported: i32,
  pub skipped: i32,
  pub previously_covered: i32,
  pub new_learning: i32,
  pub warnings: Vec<String>,
  pub error: Option<String>,
}

// ---------------------------------------------------------------------------
// Revision interval logic (ported from revision.ts, unit tested)
// ---------------------------------------------------------------------------

const DEFAULT_INTERVALS: [i32; 6] = [1, 3, 7, 14, 30, 60];

pub fn next_interval(current_interval: i32, result: &str) -> i32 {
  let idx = DEFAULT_INTERVALS.iter().position(|&x| x == current_interval);
  match result {
    "correct_clear" => {
      if let Some(i) = idx {
        if i < DEFAULT_INTERVALS.len() - 1 {
          DEFAULT_INTERVALS[i + 1]
        } else {
          current_interval + 30
        }
      } else {
        current_interval
      }
    }
    "correct_slow" => {
      if let Some(i) = idx {
        if i > 0 {
          DEFAULT_INTERVALS[i - 1]
        } else {
          current_interval
        }
      } else {
        current_interval
      }
    }
    "partial" => 3,
    "incorrect" => 1,
    _ => current_interval,
  }
}

fn revision_item_to_view(item: &rusqlite::Row) -> RusqliteResult<RevisionItemView> {
  let id: String = item.get(0)?;
  let source_type: String = item.get(1)?;
  let source_id: String = item.get(2)?;
  let initial_completion_date: Option<String> = item.get(3)?;
  let last_reviewed_date: Option<String> = item.get(4)?;
  let next_due_date: String = item.get(5)?;
  let current_interval: i32 = item.get(6)?;
  let repetition: i32 = item.get(7)?;
  let state: String = item.get(8)?;
  let recall_result: Option<String> = item.get(9)?;
  let confidence: Option<i32> = item.get(10)?;
  let notes: Option<String> = item.get(11)?;
  Ok(RevisionItemView {
    id,
    source_type,
    source_id,
    initial_completion_date,
    last_reviewed_date,
    next_due_date,
    current_interval,
    repetition,
    state,
    recall_result,
    confidence,
    notes,
  })
}

type RusqliteResult<T> = std::result::Result<T, rusqlite::Error>;

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn preview_neetcode(req: ImportNeetcodeRequest) -> PreviewNeetcodeResponse {
  match parse_neetcode_csv(&req.content) {
    Ok(rows) => {
      let total_rows = rows.len() as i32;
      let mut previously_covered = 0;
      let mut new_learning = 0;
      let mut by_category: HashMap<String, (i32, bool)> = HashMap::new();
      for row in &rows {
        let covered = is_previously_covered(row.category_order);
        if covered {
          previously_covered += 1;
        } else {
          new_learning += 1;
        }
        let entry = by_category.entry(row.category.clone()).or_insert((0, covered));
        entry.0 += 1;
        entry.1 = entry.1 || covered;
      }
      let mut by_category_vec: Vec<CategoryCount> = by_category
        .into_iter()
        .map(|(category, (count, covered))| CategoryCount {
          category,
          count,
          previously_covered: covered,
        })
        .collect();
      by_category_vec.sort_by(|a, b| a.category.cmp(&b.category));
      let sample = rows.into_iter().take(5).collect();
      PreviewNeetcodeResponse {
        ok: true,
        total_rows,
        valid_rows: total_rows,
        previously_covered,
        new_learning,
        by_category: by_category_vec,
        sample,
        error: None,
      }
    }
    Err(e) => PreviewNeetcodeResponse {
      ok: false,
      total_rows: 0,
      valid_rows: 0,
      previously_covered: 0,
      new_learning: 0,
      by_category: vec![],
      sample: vec![],
      error: Some(e),
    },
  }
}

const INSERT_PROBLEM_SQL: &str =
  "INSERT OR IGNORE INTO problems (id, external_id, title, category, difficulty, url, official_order, statement, constraints, tags, is_previously_covered, created_at)
   VALUES (?1,?2,?3,?4,?5,?6,?7,NULL,NULL,NULL,?8,?9)";

fn insert_problem(
  tx: &rusqlite::Transaction,
  row: &NeetcodeRow,
  covered: bool,
  now: &str,
) -> rusqlite::Result<usize> {
  tx.execute(
    INSERT_PROBLEM_SQL,
    params![
      Uuid::new_v4().to_string(),
      row.problem_id,
      row.title,
      row.category,
      row.difficulty,
      row.url,
      row.official_order,
      covered as i32,
      now
    ],
  )
}

// Seed the roadmap DSA track (track B) with one category topic per official
// NeetCode category, derived from the imported dataset. B1-B7 (through Trees)
// are revision-only; B8+ (Tries onward) are new learning. Idempotent: existing
// topics keep their status/progress.
fn ensure_dsa_categories(
  conn: &rusqlite::Connection,
  rows: &[NeetcodeRow],
  now: &str,
) -> rusqlite::Result<i32> {
  conn.execute(
    "INSERT OR IGNORE INTO tracks (id, key, name, description, position, created_at)
     VALUES (?1, 'b', ?2, NULL, 2, ?3)",
    params![
      Uuid::new_v4().to_string(),
      "Data Structures & Algorithms (NeetCode 250 only)",
      now
    ],
  )?;
  let track_id: String = conn.query_row(
    "SELECT id FROM tracks WHERE key = 'b'",
    [],
    |row| row.get(0),
  )?;

  conn.execute(
    "INSERT OR IGNORE INTO modules (id, track_id, key, name, position)
     VALUES (?1, ?2, 'B', 'NeetCode 250', 1)",
    params![Uuid::new_v4().to_string(), track_id],
  )?;
  let module_id: String = conn.query_row(
    "SELECT id FROM modules WHERE track_id = ?1 AND key = 'B'",
    params![track_id],
    |row| row.get(0),
  )?;

  let mut categories: std::collections::BTreeMap<i32, String> = std::collections::BTreeMap::new();
  for row in rows {
    categories
      .entry(row.category_order)
      .or_insert_with(|| row.category.clone());
  }

  let mut inserted = 0;
  for (order, category) in &categories {
    let key = format!("B{order}");
    let status = if *order <= 7 {
      "Needs revision"
    } else {
      "Not started"
    };
    let changed = conn.execute(
      "INSERT OR IGNORE INTO topics (id, module_id, key, title, status, priority, estimate_minutes, confidence, completion_criteria, notes, evidence, target_week, is_custom, source_ref, created_at, updated_at)
       VALUES (?1, ?2, ?3, ?4, ?5, 'Medium', NULL, NULL, NULL, NULL, NULL, NULL, 0, ?6, ?7, ?7)",
      params![
        Uuid::new_v4().to_string(),
        module_id,
        key,
        category,
        status,
        format!("neetcode250:category:{order}"),
        now
      ],
    )?;
    inserted += changed as i32;
  }

  Ok(inserted)
}

#[tauri::command]
pub async fn import_neetcode(req: ImportNeetcodeRequest) -> ImportNeetcodeResponse {
  let rows = match parse_neetcode_csv(&req.content) {
    Ok(r) => r,
    Err(e) => {
      return ImportNeetcodeResponse {
        ok: false,
        imported: 0,
        skipped: 0,
        previously_covered: 0,
        new_learning: 0,
        warnings: vec![],
        error: Some(e),
      }
    }
  };

  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return ImportNeetcodeResponse {
        ok: false,
        imported: 0,
        skipped: 0,
        previously_covered: 0,
        new_learning: 0,
        warnings: vec![],
        error: Some(e.to_string()),
      }
    }
  };
  if let Err(e) = crate::migrations::ensure_schema(&mut conn) {
    return ImportNeetcodeResponse {
      ok: false,
      imported: 0,
      skipped: 0,
      previously_covered: 0,
      new_learning: 0,
      warnings: vec![],
      error: Some(e.to_string()),
    };
  }

  let mut imported = 0;
  let mut skipped = 0;
  let mut previously_covered = 0;
  let mut new_learning = 0;
  let mut warnings: Vec<String> = Vec::new();
  let now = now_rfc3339();

  let tx = match conn.transaction() {
    Ok(t) => t,
    Err(e) => {
      return ImportNeetcodeResponse {
        ok: false,
        imported: 0,
        skipped: 0,
        previously_covered: 0,
        new_learning: 0,
        warnings,
        error: Some(e.to_string()),
      }
    }
  };

  for row in &rows {
    let covered = is_previously_covered(row.category_order);
    let result = insert_problem(&tx, row, covered, &now);
    match result {
      Ok(changed) if changed > 0 => {
        imported += 1;
        if covered {
          previously_covered += 1;
        } else {
          new_learning += 1;
        }
      }
      Ok(_) => {
        skipped += 1;
      }
      Err(e) => {
        warnings.push(format!("Failed to insert {}: {e}", row.problem_id));
        skipped += 1;
      }
    }
  }

  if let Err(e) = ensure_dsa_categories(&tx, &rows, &now) {
    warnings.push(format!("DSA category topics warning: {e}"));
  }

  if let Err(e) = tx.commit() {
    return ImportNeetcodeResponse {
      ok: false,
      imported: 0,
      skipped: 0,
      previously_covered: 0,
      new_learning: 0,
      warnings,
      error: Some(e.to_string()),
    };
  }

  ImportNeetcodeResponse {
    ok: true,
    imported,
    skipped,
    previously_covered,
    new_learning,
    warnings,
    error: None,
  }
}

#[tauri::command]
pub async fn get_problems(req: GetProblemsRequest) -> ProblemListResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return ProblemListResponse {
        ok: false,
        problems: vec![],
        total: 0,
        error: Some(e.to_string()),
      }
    }
  };

  let mut sql = String::from(
    "SELECT p.id, p.external_id, p.title, p.category, p.difficulty, p.url, p.official_order,
            p.is_previously_covered, COALESCE(f.id IS NOT NULL, 0) AS is_favorite,
            COALESCE(a.attempt_count, 0) AS attempt_count,
            COALESCE(a.solved, 0) AS solved
     FROM problems p
     LEFT JOIN favorites f ON f.problem_id = p.id
     LEFT JOIN (
       SELECT problem_id, COUNT(*) AS attempt_count,
              MAX(CASE WHEN outcome = 'solved' THEN 1 ELSE 0 END) AS solved
       FROM problem_attempts GROUP BY problem_id
     ) a ON a.problem_id = p.id
     WHERE 1 = 1",
  );
  let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

  macro_rules! add_filter {
    ($col:expr, $val:expr) => {{
      sql.push_str(format!(" AND {} = ?{}", $col, values.len() + 1).as_str());
      values.push(Box::new($val.to_string()) as Box<dyn rusqlite::ToSql>);
    }};
  }

  if let Some(c) = &req.category {
    add_filter!("p.category", c.clone());
  }
  if let Some(d) = &req.difficulty {
    add_filter!("p.difficulty", d.clone());
  }
  match req.cover_filter.as_str() {
    "new" => {
      sql.push_str(" AND p.is_previously_covered = 0");
    }
    "revision" => {
      sql.push_str(" AND p.is_previously_covered = 1");
    }
    _ => {}
  }
  if let Some(s) = &req.search {
    let like = format!("%{}%", s);
    sql.push_str(&format!(" AND (p.title LIKE ?{} OR p.category LIKE ?{})", values.len() + 1, values.len() + 2));
    values.push(Box::new(like.clone()) as Box<dyn rusqlite::ToSql>);
    values.push(Box::new(like) as Box<dyn rusqlite::ToSql>);
  }
  sql.push_str(" ORDER BY p.official_order");

  let params_array: Vec<&dyn rusqlite::ToSql> = values.iter().map(|v| v.as_ref()).collect();
  let mut stmt = match conn.prepare(&sql) {
    Ok(s) => s,
    Err(e) => {
      return ProblemListResponse {
        ok: false,
        problems: vec![],
        total: 0,
        error: Some(e.to_string()),
      }
    }
  };

  let rows = match stmt.query_map(params_array.as_slice(), |row| {
    Ok(Problem {
      id: row.get(0)?,
      external_id: row.get(1)?,
      title: row.get(2)?,
      category: row.get(3)?,
      difficulty: row.get(4)?,
      url: row.get(5)?,
      official_order: row.get(6)?,
      is_previously_covered: row.get::<_, i32>(7)? != 0,
      tags: vec![],
      is_favorite: row.get::<_, i32>(8)? != 0,
      attempt_count: row.get(9)?,
      solved: row.get(10)?,
    })
  }) {
    Ok(r) => r,
    Err(e) => {
      return ProblemListResponse {
        ok: false,
        problems: vec![],
        total: 0,
        error: Some(e.to_string()),
      }
    }
  };

  let mut problems = Vec::new();
  for row in rows.flatten() {
    problems.push(row);
  }
  let total = problems.len() as i32;

  ProblemListResponse {
    ok: true,
    problems,
    total,
    error: None,
  }
}

#[derive(Debug, Deserialize)]
pub struct GetProblemByIdRequest {
  pub problem_id: String,
}

#[derive(Debug, Serialize)]
pub struct ProblemResponse {
  pub ok: bool,
  pub problem: Option<Problem>,
  pub error: Option<String>,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_problem_by_id(req: GetProblemByIdRequest) -> ProblemResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return ProblemResponse {
        ok: false,
        problem: None,
        error: Some(e.to_string()),
      }
    }
  };

  let problem = conn.query_row(
    "SELECT p.id, p.external_id, p.title, p.category, p.difficulty, p.url, p.official_order,
            p.is_previously_covered, COALESCE(f.id IS NOT NULL, 0) AS is_favorite,
            COALESCE(a.attempt_count, 0) AS attempt_count,
            COALESCE(a.solved, 0) AS solved
     FROM problems p
     LEFT JOIN favorites f ON f.problem_id = p.id
     LEFT JOIN (
       SELECT problem_id, COUNT(*) AS attempt_count,
              MAX(CASE WHEN outcome = 'solved' THEN 1 ELSE 0 END) AS solved
       FROM problem_attempts GROUP BY problem_id
     ) a ON a.problem_id = p.id
     WHERE p.id = ?1",
    params![req.problem_id],
    |row| {
      Ok(Problem {
        id: row.get(0)?,
        external_id: row.get(1)?,
        title: row.get(2)?,
        category: row.get(3)?,
        difficulty: row.get(4)?,
        url: row.get(5)?,
        official_order: row.get(6)?,
        is_previously_covered: row.get::<_, i32>(7)? != 0,
        tags: vec![],
        is_favorite: row.get::<_, i32>(8)? != 0,
        attempt_count: row.get(9)?,
        solved: row.get(10)?,
      })
    },
  );

  match problem {
    Ok(p) => ProblemResponse { ok: true, problem: Some(p), error: None },
    Err(_) => ProblemResponse { ok: true, problem: None, error: None },
  }
}

#[tauri::command]
pub async fn record_attempt(req: RecordAttemptRequest) -> RecordAttemptResponse {
  if req.problem_id.trim().is_empty() {
    return RecordAttemptResponse {
      ok: false,
      id: None,
      error: Some("problem_id is required".to_string()),
    };
  }
  if NaiveDate::parse_from_str(&req.attempt_date, "%Y-%m-%d").is_err() {
    return RecordAttemptResponse {
      ok: false,
      id: None,
      error: Some("Invalid attempt_date (expected YYYY-MM-DD)".to_string()),
    };
  }

  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return RecordAttemptResponse {
        ok: false,
        id: None,
        error: Some(e.to_string()),
      }
    }
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let id = Uuid::new_v4().to_string();
  let now = now_rfc3339();
  let solved = req.outcome == "solved";

  let result = conn.execute(
    "INSERT INTO problem_attempts (id, problem_id, attempt_date, outcome, duration_minutes, notes, code_file, complexity_note, edge_cases, interview_explanation, created_at)
     VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
    params![
      id,
      req.problem_id,
      req.attempt_date,
      req.outcome,
      req.duration_minutes,
      req.notes,
      req.code_file,
      req.complexity_note,
      req.edge_cases,
      req.interview_explanation,
      now
    ],
  );
  if let Err(e) = result {
    return RecordAttemptResponse {
      ok: false,
      id: None,
      error: Some(e.to_string()),
    };
  }

  // When a problem is solved for the first time, seed a revision item.
  if solved {
    let existing: Option<String> = conn
      .query_row(
        "SELECT id FROM revision_items WHERE source_type = 'problem' AND source_id = ?1",
        params![req.problem_id],
        |r| r.get(0),
      )
      .optional()
      .unwrap_or(None);
    if existing.is_none() {
      let next_due = (Local::now().date_naive() + Duration::days(1)).format("%Y-%m-%d").to_string();
      let _ = conn.execute(
        "INSERT INTO revision_items (id, source_type, source_id, initial_completion_date, last_reviewed_date, next_due_date, current_interval, repetition, state, recall_result, confidence, notes, created_at, updated_at)
         VALUES (?1,'problem',?2,?3,NULL,?4,1,1,'reviewing',NULL,NULL,NULL,?5,?5)",
        params![Uuid::new_v4().to_string(), req.problem_id, req.attempt_date, next_due, now],
      );
    }
  }

  RecordAttemptResponse {
    ok: true,
    id: Some(id),
    error: None,
  }
}

#[tauri::command]
pub async fn list_attempts(req: GetProblemsRequest) -> ListAttemptsResponse {
  // Reuse problem_id via a dedicated field is cleaner; fall back to search field.
  let problem_id = req.search.clone().unwrap_or_default();
  if problem_id.trim().is_empty() {
    return ListAttemptsResponse {
      ok: false,
      attempts: vec![],
      error: Some("problem_id is required".to_string()),
    };
  }

  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return ListAttemptsResponse {
        ok: false,
        attempts: vec![],
        error: Some(e.to_string()),
      }
    }
  };

  let mut stmt = match conn.prepare(
    "SELECT id, problem_id, attempt_date, outcome, duration_minutes, notes, code_file, complexity_note, edge_cases, interview_explanation, created_at
     FROM problem_attempts WHERE problem_id = ?1 ORDER BY attempt_date DESC, created_at DESC",
  ) {
    Ok(s) => s,
    Err(e) => {
      return ListAttemptsResponse {
        ok: false,
        attempts: vec![],
        error: Some(e.to_string()),
      }
    }
  };

  let attempts: Vec<Attempt> = stmt
    .query_map(params![problem_id], |row| {
      Ok(Attempt {
        id: row.get(0)?,
        problem_id: row.get(1)?,
        attempt_date: row.get(2)?,
        outcome: row.get(3)?,
        duration_minutes: row.get(4)?,
        notes: row.get(5)?,
        code_file: row.get(6)?,
        complexity_note: row.get(7)?,
        edge_cases: row.get(8)?,
        interview_explanation: row.get(9)?,
        created_at: row.get(10)?,
      })
    })
    .map(|rows| rows.flatten().collect())
    .unwrap_or_default();

  ListAttemptsResponse {
    ok: true,
    attempts,
    error: None,
  }
}

#[tauri::command]
pub async fn save_solution(req: SaveSolutionRequest) -> SaveSolutionResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return SaveSolutionResponse {
        ok: false,
        file_path: None,
        error: Some(e.to_string()),
      }
    }
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let data_dir = match crate::db::app_data_dir() {
    Ok(d) => d,
    Err(e) => {
      return SaveSolutionResponse {
        ok: false,
        file_path: None,
        error: Some(e.to_string()),
      }
    }
  };
  let solutions_dir = data_dir.join("content").join("solutions");
  if let Err(e) = std::fs::create_dir_all(&solutions_dir) {
    return SaveSolutionResponse {
      ok: false,
      file_path: None,
      error: Some(e.to_string()),
    };
  }
  let file_name = format!("{}.java", req.problem_id);
  let file_path = solutions_dir.join(&file_name);
  if let Err(e) = std::fs::write(&file_path, &req.content) {
    return SaveSolutionResponse {
      ok: false,
      file_path: None,
      error: Some(e.to_string()),
    };
  }
  let path_str = file_path.to_string_lossy().to_string();
  let now = now_rfc3339();
  let result = conn.execute(
    "INSERT INTO solutions (id, problem_id, file_path, content, language, updated_at)
     VALUES (?1,?2,?3,?4,'java',?5)
     ON CONFLICT(problem_id) DO UPDATE SET file_path = excluded.file_path,
                                          content = excluded.content,
                                          updated_at = excluded.updated_at",
    params![Uuid::new_v4().to_string(), req.problem_id, path_str, req.content, now],
  );
  if let Err(e) = result {
    return SaveSolutionResponse {
      ok: false,
      file_path: None,
      error: Some(e.to_string()),
    };
  }

  SaveSolutionResponse {
    ok: true,
    file_path: Some(path_str),
    error: None,
  }
}

#[tauri::command]
pub async fn get_solution(req: GetProblemsRequest) -> GetSolutionResponse {
  let problem_id = req.search.clone().unwrap_or_default();
  if problem_id.trim().is_empty() {
    return GetSolutionResponse {
      ok: false,
      solution: None,
      error: Some("problem_id is required".to_string()),
    };
  }

  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return GetSolutionResponse {
        ok: false,
        solution: None,
        error: Some(e.to_string()),
      }
    }
  };

  let solution = conn
    .query_row(
      "SELECT id, problem_id, file_path, content, language, updated_at FROM solutions WHERE problem_id = ?1",
      params![problem_id],
      |row| {
        Ok(Solution {
          id: row.get(0)?,
          problem_id: row.get(1)?,
          file_path: row.get(2)?,
          content: row.get(3)?,
          language: row.get(4)?,
          updated_at: row.get(5)?,
        })
      },
    )
    .optional()
    .ok()
    .flatten();

  GetSolutionResponse {
    ok: true,
    solution,
    error: None,
  }
}

#[tauri::command]
pub async fn record_review(req: RecordReviewRequest) -> RecordReviewResponse {
  if NaiveDate::parse_from_str(&req.reviewed_at, "%Y-%m-%d").is_err() {
    return RecordReviewResponse {
      ok: false,
      revision_item: None,
      error: Some("Invalid reviewed_at (expected YYYY-MM-DD)".to_string()),
    };
  }

  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return RecordReviewResponse {
        ok: false,
        revision_item: None,
        error: Some(e.to_string()),
      }
    }
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let mut item: Option<(String, i32, i32, String)> = None; // id, interval, repetition, state
  if let Ok((id, interval, repetition, state)) = conn.query_row(
    "SELECT id, current_interval, repetition, state FROM revision_items WHERE id = ?1",
    params![req.revision_item_id],
    |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
  ) {
    item = Some((id, interval, repetition, state));
  }
  let Some((_id, current_interval, repetition, _state)) = item else {
    return RecordReviewResponse {
      ok: false,
      revision_item: None,
      error: Some("Revision item not found".to_string()),
    };
  };

  let new_interval = next_interval(current_interval, &req.recall_result);
  let new_repetition = repetition + 1;
  let next_due = (NaiveDate::parse_from_str(&req.reviewed_at, "%Y-%m-%d").unwrap()
    + Duration::days(new_interval as i64))
  .format("%Y-%m-%d")
  .to_string();
  let new_state = if req.recall_result == "incorrect" {
    "learning"
  } else {
    "reviewing"
  };

  let _ = conn.execute(
    "UPDATE revision_items SET next_due_date = ?1, current_interval = ?2, repetition = ?3, state = ?4,
     last_reviewed_date = ?5, recall_result = ?6, confidence = ?7, notes = ?8, updated_at = ?9
     WHERE id = ?10",
    params![
      next_due,
      new_interval,
      new_repetition,
      new_state,
      req.reviewed_at,
      req.recall_result,
      req.confidence,
      req.notes,
      now_rfc3339(),
      req.revision_item_id
    ],
  );

  let _ = conn.execute(
    "INSERT INTO revision_reviews (id, revision_item_id, reviewed_at, interval_before, interval_after, recall_result, confidence, notes, evidence, created_at)
     VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
    params![
      Uuid::new_v4().to_string(),
      req.revision_item_id,
      req.reviewed_at,
      current_interval,
      new_interval,
      req.recall_result,
      req.confidence,
      req.notes,
      req.evidence,
      now_rfc3339()
    ],
  );

  let revision_item = conn
    .query_row(
      "SELECT id, source_type, source_id, initial_completion_date, last_reviewed_date, next_due_date, current_interval, repetition, state, recall_result, confidence, notes
       FROM revision_items WHERE id = ?1",
      params![req.revision_item_id],
      |row| revision_item_to_view(row),
    )
    .ok();

  RecordReviewResponse {
    ok: true,
    revision_item,
    error: None,
  }
}

#[tauri::command]
pub async fn get_revision_queue(req: GetProblemsRequest) -> GetRevisionQueueResponse {
  let due_before = req
    .search
    .clone()
    .unwrap_or_else(|| Local::now().date_naive().format("%Y-%m-%d").to_string());

  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return GetRevisionQueueResponse {
        ok: false,
        items: vec![],
        error: Some(e.to_string()),
      }
    }
  };

  let mut stmt = match conn.prepare(
    "SELECT id, source_type, source_id, initial_completion_date, last_reviewed_date, next_due_date, current_interval, repetition, state, recall_result, confidence, notes
     FROM revision_items WHERE next_due_date <= ?1 ORDER BY next_due_date",
  ) {
    Ok(s) => s,
    Err(e) => {
      return GetRevisionQueueResponse {
        ok: false,
        items: vec![],
        error: Some(e.to_string()),
      }
    }
  };

  let items: Vec<RevisionItemView> = stmt
    .query_map(params![due_before], |row| revision_item_to_view(row))
    .map(|rows| rows.flatten().collect())
    .unwrap_or_default();

  GetRevisionQueueResponse {
    ok: true,
    items,
    error: None,
  }
}

pub fn start_revision_baseline_inner() -> StartRevisionBaselineResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return StartRevisionBaselineResponse {
        ok: false,
        created: 0,
        error: Some(e.to_string()),
      }
    }
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let today = Local::now().date_naive().format("%Y-%m-%d").to_string();
  let now = now_rfc3339();

  let created = conn
    .execute(
      "INSERT INTO revision_items (id, source_type, source_id, initial_completion_date, last_reviewed_date, next_due_date, current_interval, repetition, state, recall_result, confidence, notes, created_at, updated_at)
       SELECT lower(hex(randomblob(16))), 'problem', p.id, ?1, NULL, ?1, 1, 0, 'learning', NULL, NULL, NULL, ?2, ?2
       FROM problems p
       WHERE p.is_previously_covered = 1
         AND NOT EXISTS (SELECT 1 FROM problem_attempts pa WHERE pa.problem_id = p.id)
         AND NOT EXISTS (SELECT 1 FROM revision_items ri WHERE ri.source_type = 'problem' AND ri.source_id = p.id)",
      params![today, now],
    )
    .unwrap_or(0) as i32;

  StartRevisionBaselineResponse {
    ok: true,
    created,
    error: None,
  }
}

#[tauri::command]
pub async fn start_revision_baseline() -> StartRevisionBaselineResponse {
  start_revision_baseline_inner()
}

#[tauri::command]
pub async fn get_dsa_progress() -> GetDsaProgressResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return GetDsaProgressResponse {
        ok: false,
        categories: vec![],
        total_problems: 0,
        total_attempted: 0,
        total_solved: 0,
        error: Some(e.to_string()),
      }
    }
  };

  let mut stmt = match conn.prepare(
    "SELECT p.category,
            COUNT(*) AS total,
            COALESCE(SUM(CASE WHEN a.attempt_count > 0 THEN 1 ELSE 0 END), 0) AS attempted,
            COALESCE(SUM(CASE WHEN a.solved > 0 THEN 1 ELSE 0 END), 0) AS solved,
            p.is_previously_covered
     FROM problems p
     LEFT JOIN (
       SELECT problem_id, COUNT(*) AS attempt_count,
              MAX(CASE WHEN outcome = 'solved' THEN 1 ELSE 0 END) AS solved
       FROM problem_attempts GROUP BY problem_id
     ) a ON a.problem_id = p.id
     GROUP BY p.category, p.is_previously_covered
     ORDER BY MIN(p.official_order)",
  ) {
    Ok(s) => s,
    Err(e) => {
      return GetDsaProgressResponse {
        ok: false,
        categories: vec![],
        total_problems: 0,
        total_attempted: 0,
        total_solved: 0,
        error: Some(e.to_string()),
      }
    }
  };

  let categories: Vec<CategoryProgress> = stmt
    .query_map([], |row| {
      Ok(CategoryProgress {
        category: row.get(0)?,
        total: row.get(1)?,
        attempted: row.get(2)?,
        solved: row.get(3)?,
        previously_covered: row.get::<_, i32>(4)? != 0,
      })
    })
    .map(|rows| rows.flatten().collect())
    .unwrap_or_default();

  let total_problems: i32 = categories.iter().map(|c| c.total).sum();
  let total_attempted: i32 = categories.iter().map(|c| c.attempted).sum();
  let total_solved: i32 = categories.iter().map(|c| c.solved).sum();

  GetDsaProgressResponse {
    ok: true,
    categories,
    total_problems,
    total_attempted,
    total_solved,
    error: None,
  }
}

#[derive(Debug, Deserialize)]
pub struct ToggleFavoriteRequest {
  pub problem_id: String,
  pub favorite: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToggleFavoriteResponse {
  pub ok: bool,
  pub is_favorite: bool,
  pub error: Option<String>,
}

#[tauri::command]
pub async fn toggle_favorite(req: ToggleFavoriteRequest) -> ToggleFavoriteResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return ToggleFavoriteResponse {
        ok: false,
        is_favorite: false,
        error: Some(e.to_string()),
      }
    }
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let result = if req.favorite {
    conn.execute(
      "INSERT INTO favorites (id, problem_id, created_at) VALUES (?1, ?2, ?3)
       ON CONFLICT(problem_id) DO NOTHING",
      params![Uuid::new_v4().to_string(), req.problem_id, now_rfc3339()],
    )
  } else {
    conn.execute(
      "DELETE FROM favorites WHERE problem_id = ?1",
      params![req.problem_id],
    )
  };

  match result {
    Ok(_) => ToggleFavoriteResponse {
      ok: true,
      is_favorite: req.favorite,
      error: None,
    },
    Err(e) => ToggleFavoriteResponse {
      ok: false,
      is_favorite: !req.favorite,
      error: Some(e.to_string()),
    },
  }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_neetcode_csv_with_quoted_fields() {
    let content = "official_order,problem_id,title,url,category,difficulty,category_order,source_name,source_version
1,concatenation-of-array,Concatenation of Array,https://leetcode.com/problems/concatenation-of-array/,Arrays & Hashing,Easy,1,NeetCode 250,2026-11-13
249,powx-n,\"Pow(x, n)\",https://leetcode.com/problems/powx-n/,Math & Geometry,Medium,18,NeetCode 250,2026-11-13";
    let rows = parse_neetcode_csv(content).unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[1].title, "Pow(x, n)");
    assert_eq!(rows[1].category_order, 18);
  }

  #[test]
  fn rejects_malformed_csv() {
    let content = "official_order,problem_id,title,url,category,difficulty,category_order,source_name,source_version
1,concatenation-of-array,Concatenation of Array";
    assert!(parse_neetcode_csv(content).is_err());
  }

  #[test]
  fn previously_covered_boundary() {
    assert!(is_previously_covered(7)); // Trees
    assert!(!is_previously_covered(8)); // Tries
    assert!(is_previously_covered(1)); // Arrays & Hashing
  }

  #[test]
  fn revision_interval_advances_on_clear() {
    assert_eq!(next_interval(1, "correct_clear"), 3);
    assert_eq!(next_interval(7, "correct_clear"), 14);
    assert_eq!(next_interval(60, "correct_clear"), 90);
  }

  #[test]
  fn revision_interval_slow_or_wrong() {
    assert_eq!(next_interval(7, "correct_slow"), 3);
    assert_eq!(next_interval(1, "correct_slow"), 1);
    assert_eq!(next_interval(7, "partial"), 3);
    assert_eq!(next_interval(14, "incorrect"), 1);
  }

  #[test]
  fn preview_counts_covered_vs_new() {
    let content = "official_order,problem_id,title,url,category,difficulty,category_order,source_name,source_version
1,a,Title A,https://x/a,Arrays & Hashing,Easy,1,NeetCode 250,2026-11-13
2,b,Title B,https://x/b,Two Pointers,Easy,2,NeetCode 250,2026-11-13
110,t,Title T,https://x/t,Tries,Medium,8,NeetCode 250,2026-11-13";
    let resp = preview_neetcode(ImportNeetcodeRequest {
      content: content.to_string(),
    });
    assert!(resp.ok);
    assert_eq!(resp.total_rows, 3);
    assert_eq!(resp.previously_covered, 2);
    assert_eq!(resp.new_learning, 1);
  }

  #[test]
  fn import_insert_matches_real_schema_and_is_idempotent() {
    let mut conn = rusqlite::Connection::open_in_memory().unwrap();
    crate::migrations::ensure_schema(&mut conn).unwrap();

    let row = NeetcodeRow {
      official_order: 1,
      problem_id: "concatenation-of-array".to_string(),
      title: "Concatenation of Array".to_string(),
      url: "https://leetcode.com/problems/concatenation-of-array/".to_string(),
      category: "Arrays & Hashing".to_string(),
      difficulty: "Easy".to_string(),
      category_order: 1,
      source_name: "NeetCode 250".to_string(),
      source_version: "2026-11-13".to_string(),
    };

    let tx = conn.transaction().unwrap();
    let now = "2026-01-01T00:00:00Z";
    assert_eq!(insert_problem(&tx, &row, true, now).unwrap(), 1);
    assert_eq!(insert_problem(&tx, &row, true, now).unwrap(), 0);
    tx.commit().unwrap();

    let count: i32 = conn
      .query_row("SELECT COUNT(*) FROM problems", [], |r| r.get(0))
      .unwrap();
    assert_eq!(count, 1);
  }

  #[test]
  fn import_seeds_dsa_category_topics_idempotently() {
    let mut conn = rusqlite::Connection::open_in_memory().unwrap();
    crate::migrations::ensure_schema(&mut conn).unwrap();

    let rows = vec![
      NeetcodeRow {
        official_order: 1,
        problem_id: "a".to_string(),
        title: "A".to_string(),
        url: "u".to_string(),
        category: "Arrays & Hashing".to_string(),
        difficulty: "Easy".to_string(),
        category_order: 1,
        source_name: "NeetCode 250".to_string(),
        source_version: "v".to_string(),
      },
      NeetcodeRow {
        official_order: 110,
        problem_id: "t".to_string(),
        title: "T".to_string(),
        url: "u".to_string(),
        category: "Tries".to_string(),
        difficulty: "Medium".to_string(),
        category_order: 8,
        source_name: "NeetCode 250".to_string(),
        source_version: "v".to_string(),
      },
    ];

    let now = "2026-01-01T00:00:00Z";
    assert_eq!(ensure_dsa_categories(&conn, &rows, now).unwrap(), 2);

    let b1: (String, String) = conn
      .query_row("SELECT title, status FROM topics WHERE key = 'B1'", [], |r| {
        Ok((r.get(0)?, r.get(1)?))
      })
      .unwrap();
    assert_eq!(b1, ("Arrays & Hashing".to_string(), "Needs revision".to_string()));

    let b8_status: String = conn
      .query_row("SELECT status FROM topics WHERE key = 'B8'", [], |r| r.get(0))
      .unwrap();
    assert_eq!(b8_status, "Not started");

    // Re-running must not duplicate or overwrite topics.
    assert_eq!(ensure_dsa_categories(&conn, &rows, now).unwrap(), 0);
    let topic_count: i32 = conn
      .query_row("SELECT COUNT(*) FROM topics WHERE key LIKE 'B%'", [], |r| r.get(0))
      .unwrap();
    assert_eq!(topic_count, 2);
  }
}
