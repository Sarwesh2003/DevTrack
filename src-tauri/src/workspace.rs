use crate::curriculum::UpsertResponse;
use crate::db::{app_data_dir, now_rfc3339, open_connection, DbError, DbResult};
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExperiment {
  pub id: String,
  pub topic_id: String,
  pub title: String,
  pub code: String,
  pub language: String,
  pub file_path: Option<String>,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertCodeExperimentRequest {
  pub topic_id: String,
  pub title: String,
  pub code: String,
  pub language: Option<String>,
  pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeExperimentResponse {
  pub ok: bool,
  pub experiment: Option<CodeExperiment>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListCodeExperimentsResponse {
  pub ok: bool,
  pub experiments: Vec<CodeExperiment>,
  pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallQuestion {
  pub id: String,
  pub topic_id: String,
  pub question: String,
  pub answer: String,
  pub difficulty: String,
  pub last_reviewed: Option<String>,
  pub next_review: Option<String>,
  pub interval: i32,
  pub repetition: i32,
  pub state: String,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertRecallQuestionRequest {
  pub topic_id: String,
  pub question: String,
  pub answer: String,
  pub difficulty: Option<String>,
  pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordRecallReviewRequest {
  pub id: String,
  pub recall_result: String,
  pub confidence: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecallQuestionResponse {
  pub ok: bool,
  pub question: Option<RecallQuestion>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListRecallQuestionsResponse {
  pub ok: bool,
  pub questions: Vec<RecallQuestion>,
  pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagram {
  pub id: String,
  pub topic_id: String,
  pub title: String,
  pub mermaid_content: String,
  pub file_path: Option<String>,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertDiagramRequest {
  pub topic_id: String,
  pub title: String,
  pub mermaid_content: String,
  pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagramResponse {
  pub ok: bool,
  pub diagram: Option<Diagram>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListDiagramsResponse {
  pub ok: bool,
  pub diagrams: Vec<Diagram>,
  pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignArtifact {
  pub id: String,
  pub topic_id: String,
  pub title: String,
  pub requirements: Option<String>,
  pub api_spec: Option<String>,
  pub java_implementation: Option<String>,
  pub tests: Option<String>,
  pub trade_offs: Option<String>,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertDesignArtifactRequest {
  pub topic_id: String,
  pub title: String,
  pub requirements: Option<String>,
  pub api_spec: Option<String>,
  pub java_implementation: Option<String>,
  pub tests: Option<String>,
  pub trade_offs: Option<String>,
  pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DesignArtifactResponse {
  pub ok: bool,
  pub artifact: Option<DesignArtifact>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListDesignArtifactsResponse {
  pub ok: bool,
  pub artifacts: Vec<DesignArtifact>,
  pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseStudy {
  pub id: String,
  pub topic_id: String,
  pub title: String,
  pub context: Option<String>,
  pub diagram_content: Option<String>,
  pub trade_offs: Option<String>,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertCaseStudyRequest {
  pub topic_id: String,
  pub title: String,
  pub context: Option<String>,
  pub diagram_content: Option<String>,
  pub trade_offs: Option<String>,
  pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseStudyResponse {
  pub ok: bool,
  pub case_study: Option<CaseStudy>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListCaseStudiesResponse {
  pub ok: bool,
  pub case_studies: Vec<CaseStudy>,
  pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicLink {
  pub id: String,
  pub from_topic_id: String,
  pub to_topic_id: String,
  pub link_type: String,
  pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTopicLinkRequest {
  pub from_topic_id: String,
  pub to_topic_id: String,
  pub link_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicLinkResponse {
  pub ok: bool,
  pub link: Option<TopicLink>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTopicLinksResponse {
  pub ok: bool,
  pub links: Vec<TopicLink>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveWorkspaceFileResponse {
  pub ok: bool,
  pub file_path: Option<String>,
  pub error: Option<String>,
}

// ---------------------------------------------------------------------------
// File storage helpers
// ---------------------------------------------------------------------------

fn workspace_dir() -> DbResult<PathBuf> {
  let data_dir = app_data_dir()?;
  Ok(data_dir.join("content").join("workspace"))
}

// Reject any path component that could escape the workspace root.
fn safe_file_name(name: &str) -> DbResult<String> {
  if name.is_empty()
    || name == "."
    || name == ".."
    || name.contains('/')
    || name.contains('\\')
    || name.contains('\0')
  {
    return Err(DbError::Serialization(format!("unsafe file name: {name}")));
  }
  Ok(name.to_string())
}

fn write_workspace_file(subdir: &str, topic_id: &str, file_name: &str, content: &str) -> DbResult<PathBuf> {
  let dir = workspace_dir()?.join(subdir).join(topic_id);
  std::fs::create_dir_all(&dir).map_err(DbError::Io)?;
  let path = dir.join(safe_file_name(file_name)?);
  std::fs::write(&path, content).map_err(DbError::Io)?;
  Ok(path)
}

// ---------------------------------------------------------------------------
// Code experiments
// ---------------------------------------------------------------------------

#[tauri::command(rename_all = "snake_case")]
pub async fn upsert_code_experiment(req: UpsertCodeExperimentRequest) -> UpsertResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let id = req.id.unwrap_or_else(|| Uuid::new_v4().to_string());
  let now = now_rfc3339();
  let language = req.language.unwrap_or_else(|| "java".to_string());

  let tx = match conn.transaction() {
    Ok(t) => t,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };

  let result = tx.execute(
    "INSERT INTO code_experiments (id, topic_id, title, code, language, file_path, created_at, updated_at)
     VALUES (?1,?2,?3,?4,?5,NULL,?6,?6)
     ON CONFLICT(id) DO UPDATE SET
       title = excluded.title,
       code = excluded.code,
       language = excluded.language,
       updated_at = excluded.updated_at",
    params![id, req.topic_id, req.title, req.code, language, now],
  );

  if let Err(e) = result {
    return UpsertResponse { ok: false, id: e.to_string() };
  }

  // Persist the experiment as a local source file.
  let ext = if language.eq_ignore_ascii_case("java") { "java" } else { "txt" };
  let file_name = format!("{}.{}", safe_file_name(&req.title).unwrap_or_else(|_| "experiment".to_string()), ext);
  if let Err(e) = write_workspace_file("experiments", &req.topic_id, &file_name, &req.code) {
    return UpsertResponse { ok: false, id: e.to_string() };
  }
  let path_str = workspace_dir()
    .and_then(|d| Ok(d.join("experiments").join(req.topic_id).join(&file_name).to_string_lossy().to_string()))
    .unwrap_or_default();

  let _ = tx.execute(
    "UPDATE code_experiments SET file_path = ?1 WHERE id = ?2",
    params![path_str, id],
  );

  if let Err(e) = tx.commit() {
    return UpsertResponse { ok: false, id: e.to_string() };
  }

  UpsertResponse { ok: true, id }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_code_experiment(id: String) -> CodeExperimentResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return CodeExperimentResponse { ok: false, experiment: None, error: Some(e.to_string()) },
  };
  let experiment = conn
    .query_row(
      "SELECT id, topic_id, title, code, language, file_path, created_at, updated_at FROM code_experiments WHERE id = ?1",
      params![id],
      |row| {
        Ok(CodeExperiment {
          id: row.get(0)?,
          topic_id: row.get(1)?,
          title: row.get(2)?,
          code: row.get(3)?,
          language: row.get(4)?,
          file_path: row.get(5)?,
          created_at: row.get(6)?,
          updated_at: row.get(7)?,
        })
      },
    )
    .optional()
    .unwrap_or(None);
  CodeExperimentResponse { ok: true, experiment, error: None }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn list_code_experiments(topic_id: String) -> ListCodeExperimentsResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return ListCodeExperimentsResponse { ok: false, experiments: vec![], error: Some(e.to_string()) },
  };
  let mut stmt = match conn.prepare(
    "SELECT id, topic_id, title, code, language, file_path, created_at, updated_at FROM code_experiments WHERE topic_id = ?1 ORDER BY updated_at DESC",
  ) {
    Ok(s) => s,
    Err(e) => return ListCodeExperimentsResponse { ok: false, experiments: vec![], error: Some(e.to_string()) },
  };
  let rows = match stmt.query_map(params![topic_id], |row| {
      Ok(CodeExperiment {
        id: row.get(0)?,
        topic_id: row.get(1)?,
        title: row.get(2)?,
        code: row.get(3)?,
        language: row.get(4)?,
        file_path: row.get(5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
      })
    }) {
      Ok(r) => r,
      Err(e) => return ListCodeExperimentsResponse { ok: false, experiments: vec![], error: Some(e.to_string()) },
    };
  let experiments: Vec<CodeExperiment> = rows.flatten().collect();
  ListCodeExperimentsResponse { ok: true, experiments, error: None }
}

// ---------------------------------------------------------------------------
// Recall questions
// ---------------------------------------------------------------------------

#[tauri::command(rename_all = "snake_case")]
pub async fn upsert_recall_question(req: UpsertRecallQuestionRequest) -> UpsertResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let id = req.id.unwrap_or_else(|| Uuid::new_v4().to_string());
  let now = now_rfc3339();
  let difficulty = req.difficulty.unwrap_or_else(|| "medium".to_string());

  let result = conn.execute(
    "INSERT INTO recall_questions (id, topic_id, question, answer, difficulty, last_reviewed, next_review, interval, repetition, state, created_at, updated_at)
     VALUES (?1,?2,?3,?4,?5,NULL,NULL,1,1,'learning',?6,?6)
     ON CONFLICT(id) DO UPDATE SET
       question = excluded.question,
       answer = excluded.answer,
       difficulty = excluded.difficulty,
       updated_at = excluded.updated_at",
    params![id, req.topic_id, req.question, req.answer, difficulty, now],
  );

  if let Err(e) = result {
    return UpsertResponse { ok: false, id: e.to_string() };
  }
  UpsertResponse { ok: true, id }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn record_recall_review(req: RecordRecallReviewRequest) -> UpsertResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let now = now_rfc3339();
  let tx = match conn.transaction() {
    Ok(t) => t,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };

  let current: Option<(i32, i32, String)> = tx
    .query_row(
      "SELECT interval, repetition, state FROM recall_questions WHERE id = ?1",
      params![req.id],
      |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )
    .optional()
    .unwrap_or(None);

  let Some((interval, repetition, state)) = current else {
    return UpsertResponse { ok: false, id: "question not found".to_string() };
  };

  let (next_interval, next_repetition, next_state) = match req.recall_result.as_str() {
    "again" => (1, 1, "learning".to_string()),
    "hard" => (interval.max(1), repetition + 1, "learning".to_string()),
    "good" => (interval * 2, repetition + 1, "reviewing".to_string()),
    "easy" => (interval * 3, repetition + 1, "known".to_string()),
    _ => (interval, repetition, state),
  };

  let next_review = chrono::Local::now().date_naive()
    .checked_add_signed(chrono::Duration::days(next_interval as i64))
    .map(|d| d.format("%Y-%m-%d").to_string())
    .unwrap_or_else(|| now.clone());

  let result = tx.execute(
    "UPDATE recall_questions
     SET last_reviewed = ?1, next_review = ?2, interval = ?3, repetition = ?4, state = ?5, updated_at = ?6
     WHERE id = ?7",
    params![now, next_review, next_interval, next_repetition, next_state, now, req.id],
  );

  if let Err(e) = result {
    return UpsertResponse { ok: false, id: e.to_string() };
  }
  if let Err(e) = tx.commit() {
    return UpsertResponse { ok: false, id: e.to_string() };
  }
  UpsertResponse { ok: true, id: req.id.clone() }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_recall_question(id: String) -> RecallQuestionResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return RecallQuestionResponse { ok: false, question: None, error: Some(e.to_string()) },
  };
  let question = conn
    .query_row(
      "SELECT id, topic_id, question, answer, difficulty, last_reviewed, next_review, interval, repetition, state, created_at, updated_at FROM recall_questions WHERE id = ?1",
      params![id],
      |row| {
        Ok(RecallQuestion {
          id: row.get(0)?,
          topic_id: row.get(1)?,
          question: row.get(2)?,
          answer: row.get(3)?,
          difficulty: row.get(4)?,
          last_reviewed: row.get(5)?,
          next_review: row.get(6)?,
          interval: row.get(7)?,
          repetition: row.get(8)?,
          state: row.get(9)?,
          created_at: row.get(10)?,
          updated_at: row.get(11)?,
        })
      },
    )
    .optional()
    .unwrap_or(None);
  RecallQuestionResponse { ok: true, question, error: None }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn list_recall_questions(topic_id: String) -> ListRecallQuestionsResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return ListRecallQuestionsResponse { ok: false, questions: vec![], error: Some(e.to_string()) },
  };
  let mut stmt = match conn.prepare(
    "SELECT id, topic_id, question, answer, difficulty, last_reviewed, next_review, interval, repetition, state, created_at, updated_at FROM recall_questions WHERE topic_id = ?1 ORDER BY next_review IS NULL, next_review, created_at DESC",
  ) {
    Ok(s) => s,
    Err(e) => return ListRecallQuestionsResponse { ok: false, questions: vec![], error: Some(e.to_string()) },
  };
  let rows = match stmt.query_map(params![topic_id], |row| {
      Ok(RecallQuestion {
        id: row.get(0)?,
        topic_id: row.get(1)?,
        question: row.get(2)?,
        answer: row.get(3)?,
        difficulty: row.get(4)?,
        last_reviewed: row.get(5)?,
        next_review: row.get(6)?,
        interval: row.get(7)?,
        repetition: row.get(8)?,
        state: row.get(9)?,
        created_at: row.get(10)?,
        updated_at: row.get(11)?,
      })
    }) {
      Ok(r) => r,
      Err(e) => return ListRecallQuestionsResponse { ok: false, questions: vec![], error: Some(e.to_string()) },
    };
  let questions: Vec<RecallQuestion> = rows.flatten().collect();
  ListRecallQuestionsResponse { ok: true, questions, error: None }
}

// ---------------------------------------------------------------------------
// Diagrams (Mermaid)
// ---------------------------------------------------------------------------

#[tauri::command(rename_all = "snake_case")]
pub async fn upsert_diagram(req: UpsertDiagramRequest) -> UpsertResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let id = req.id.unwrap_or_else(|| Uuid::new_v4().to_string());
  let now = now_rfc3339();

  let tx = match conn.transaction() {
    Ok(t) => t,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };

  let result = tx.execute(
    "INSERT INTO diagrams (id, topic_id, title, mermaid_content, file_path, created_at, updated_at)
     VALUES (?1,?2,?3,?4,NULL,?5,?5)
     ON CONFLICT(id) DO UPDATE SET
       title = excluded.title,
       mermaid_content = excluded.mermaid_content,
       updated_at = excluded.updated_at",
    params![id, req.topic_id, req.title, req.mermaid_content, now],
  );

  if let Err(e) = result {
    return UpsertResponse { ok: false, id: e.to_string() };
  }

  let file_name = format!("{}.mmd", safe_file_name(&req.title).unwrap_or_else(|_| "diagram".to_string()));
  if let Err(e) = write_workspace_file("diagrams", &req.topic_id, &file_name, &req.mermaid_content) {
    return UpsertResponse { ok: false, id: e.to_string() };
  }
  let path_str = workspace_dir()
    .and_then(|d| Ok(d.join("diagrams").join(req.topic_id).join(&file_name).to_string_lossy().to_string()))
    .unwrap_or_default();

  let _ = tx.execute("UPDATE diagrams SET file_path = ?1 WHERE id = ?2", params![path_str, id]);

  if let Err(e) = tx.commit() {
    return UpsertResponse { ok: false, id: e.to_string() };
  }
  UpsertResponse { ok: true, id }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_diagram(id: String) -> DiagramResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return DiagramResponse { ok: false, diagram: None, error: Some(e.to_string()) },
  };
  let diagram = conn
    .query_row(
      "SELECT id, topic_id, title, mermaid_content, file_path, created_at, updated_at FROM diagrams WHERE id = ?1",
      params![id],
      |row| {
        Ok(Diagram {
          id: row.get(0)?,
          topic_id: row.get(1)?,
          title: row.get(2)?,
          mermaid_content: row.get(3)?,
          file_path: row.get(4)?,
          created_at: row.get(5)?,
          updated_at: row.get(6)?,
        })
      },
    )
    .optional()
    .unwrap_or(None);
  DiagramResponse { ok: true, diagram, error: None }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn list_diagrams(topic_id: String) -> ListDiagramsResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return ListDiagramsResponse { ok: false, diagrams: vec![], error: Some(e.to_string()) },
  };
  let mut stmt = match conn.prepare(
    "SELECT id, topic_id, title, mermaid_content, file_path, created_at, updated_at FROM diagrams WHERE topic_id = ?1 ORDER BY updated_at DESC",
  ) {
    Ok(s) => s,
    Err(e) => return ListDiagramsResponse { ok: false, diagrams: vec![], error: Some(e.to_string()) },
  };
  let rows = match stmt.query_map(params![topic_id], |row| {
      Ok(Diagram {
        id: row.get(0)?,
        topic_id: row.get(1)?,
        title: row.get(2)?,
        mermaid_content: row.get(3)?,
        file_path: row.get(4)?,
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
      })
    }) {
      Ok(r) => r,
      Err(e) => return ListDiagramsResponse { ok: false, diagrams: vec![], error: Some(e.to_string()) },
    };
  let diagrams: Vec<Diagram> = rows.flatten().collect();
  ListDiagramsResponse { ok: true, diagrams, error: None }
}

// ---------------------------------------------------------------------------
// Design artifacts (LLD)
// ---------------------------------------------------------------------------

#[tauri::command(rename_all = "snake_case")]
pub async fn upsert_design_artifact(req: UpsertDesignArtifactRequest) -> UpsertResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let id = req.id.unwrap_or_else(|| Uuid::new_v4().to_string());
  let now = now_rfc3339();

  let result = conn.execute(
    "INSERT INTO design_artifacts (id, topic_id, title, requirements, api_spec, java_implementation, tests, trade_offs, created_at, updated_at)
     VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)
     ON CONFLICT(id) DO UPDATE SET
       title = excluded.title,
       requirements = excluded.requirements,
       api_spec = excluded.api_spec,
       java_implementation = excluded.java_implementation,
       tests = excluded.tests,
       trade_offs = excluded.trade_offs,
       updated_at = excluded.updated_at",
    params![
      id,
      req.topic_id,
      req.title,
      req.requirements,
      req.api_spec,
      req.java_implementation,
      req.tests,
      req.trade_offs,
      now,
      now
    ],
  );

  if let Err(e) = result {
    return UpsertResponse { ok: false, id: e.to_string() };
  }
  UpsertResponse { ok: true, id }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_design_artifact(id: String) -> DesignArtifactResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return DesignArtifactResponse { ok: false, artifact: None, error: Some(e.to_string()) },
  };
  let artifact = conn
    .query_row(
      "SELECT id, topic_id, title, requirements, api_spec, java_implementation, tests, trade_offs, created_at, updated_at FROM design_artifacts WHERE id = ?1",
      params![id],
      |row| {
        Ok(DesignArtifact {
          id: row.get(0)?,
          topic_id: row.get(1)?,
          title: row.get(2)?,
          requirements: row.get(3)?,
          api_spec: row.get(4)?,
          java_implementation: row.get(5)?,
          tests: row.get(6)?,
          trade_offs: row.get(7)?,
          created_at: row.get(8)?,
          updated_at: row.get(9)?,
        })
      },
    )
    .optional()
    .unwrap_or(None);
  DesignArtifactResponse { ok: true, artifact, error: None }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn list_design_artifacts(topic_id: String) -> ListDesignArtifactsResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return ListDesignArtifactsResponse { ok: false, artifacts: vec![], error: Some(e.to_string()) }
    }
  };
  let mut stmt = match conn.prepare(
    "SELECT id, topic_id, title, requirements, api_spec, java_implementation, tests, trade_offs, created_at, updated_at FROM design_artifacts WHERE topic_id = ?1 ORDER BY updated_at DESC",
  ) {
    Ok(s) => s,
    Err(e) => {
      return ListDesignArtifactsResponse { ok: false, artifacts: vec![], error: Some(e.to_string()) }
    }
  };
  let rows = match stmt.query_map(params![topic_id], |row| {
    Ok(DesignArtifact {
      id: row.get(0)?,
      topic_id: row.get(1)?,
      title: row.get(2)?,
      requirements: row.get(3)?,
      api_spec: row.get(4)?,
      java_implementation: row.get(5)?,
      tests: row.get(6)?,
      trade_offs: row.get(7)?,
      created_at: row.get(8)?,
      updated_at: row.get(9)?,
    })
  }) {
    Ok(r) => r,
    Err(e) => {
      return ListDesignArtifactsResponse { ok: false, artifacts: vec![], error: Some(e.to_string()) }
    }
  };
  let artifacts: Vec<DesignArtifact> = rows.flatten().collect();
  ListDesignArtifactsResponse { ok: true, artifacts, error: None }
}

// ---------------------------------------------------------------------------
// Case studies (HLD)
// ---------------------------------------------------------------------------

#[tauri::command(rename_all = "snake_case")]
pub async fn upsert_case_study(req: UpsertCaseStudyRequest) -> UpsertResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let id = req.id.unwrap_or_else(|| Uuid::new_v4().to_string());
  let now = now_rfc3339();

  let result = conn.execute(
    "INSERT INTO case_studies (id, topic_id, title, context, diagram_content, trade_offs, created_at, updated_at)
     VALUES (?1,?2,?3,?4,?5,?6,?7,?8)
     ON CONFLICT(id) DO UPDATE SET
       title = excluded.title,
       context = excluded.context,
       diagram_content = excluded.diagram_content,
       trade_offs = excluded.trade_offs,
       updated_at = excluded.updated_at",
    params![id, req.topic_id, req.title, req.context, req.diagram_content, req.trade_offs, now, now],
  );

  if let Err(e) = result {
    return UpsertResponse { ok: false, id: e.to_string() };
  }
  UpsertResponse { ok: true, id }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_case_study(id: String) -> CaseStudyResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return CaseStudyResponse { ok: false, case_study: None, error: Some(e.to_string()) },
  };
  let case_study = conn
    .query_row(
      "SELECT id, topic_id, title, context, diagram_content, trade_offs, created_at, updated_at FROM case_studies WHERE id = ?1",
      params![id],
      |row| {
        Ok(CaseStudy {
          id: row.get(0)?,
          topic_id: row.get(1)?,
          title: row.get(2)?,
          context: row.get(3)?,
          diagram_content: row.get(4)?,
          trade_offs: row.get(5)?,
          created_at: row.get(6)?,
          updated_at: row.get(7)?,
        })
      },
    )
    .optional()
    .unwrap_or(None);
  CaseStudyResponse { ok: true, case_study, error: None }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn list_case_studies(topic_id: String) -> ListCaseStudiesResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return ListCaseStudiesResponse { ok: false, case_studies: vec![], error: Some(e.to_string()) }
    }
  };
  let mut stmt = match conn.prepare(
    "SELECT id, topic_id, title, context, diagram_content, trade_offs, created_at, updated_at FROM case_studies WHERE topic_id = ?1 ORDER BY updated_at DESC",
  ) {
    Ok(s) => s,
    Err(e) => {
      return ListCaseStudiesResponse { ok: false, case_studies: vec![], error: Some(e.to_string()) }
    }
  };
  let rows = match stmt.query_map(params![topic_id], |row| {
    Ok(CaseStudy {
      id: row.get(0)?,
      topic_id: row.get(1)?,
      title: row.get(2)?,
      context: row.get(3)?,
      diagram_content: row.get(4)?,
      trade_offs: row.get(5)?,
      created_at: row.get(6)?,
      updated_at: row.get(7)?,
    })
  }) {
    Ok(r) => r,
    Err(e) => {
      return ListCaseStudiesResponse { ok: false, case_studies: vec![], error: Some(e.to_string()) }
    }
  };
  let case_studies: Vec<CaseStudy> = rows.flatten().collect();
  ListCaseStudiesResponse { ok: true, case_studies, error: None }
}

// ---------------------------------------------------------------------------
// Topic links (cross-track)
// ---------------------------------------------------------------------------

#[tauri::command(rename_all = "snake_case")]
pub async fn add_topic_link(req: AddTopicLinkRequest) -> TopicLinkResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return TopicLinkResponse { ok: false, link: None, error: Some(e.to_string()) },
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let id = Uuid::new_v4().to_string();
  let now = now_rfc3339();
  let link_type = req.link_type.unwrap_or_else(|| "related".to_string());

  let result = conn.execute(
    "INSERT INTO topic_links (id, from_topic_id, to_topic_id, link_type, created_at)
     VALUES (?1,?2,?3,?4,?5)
     ON CONFLICT(from_topic_id, to_topic_id, link_type) DO UPDATE SET
       created_at = excluded.created_at",
    params![id, req.from_topic_id, req.to_topic_id, link_type, now],
  );

  if let Err(e) = result {
    let error = Some(e.to_string());
    return TopicLinkResponse { ok: false, link: None, error };
  }

  let link = TopicLink {
    id,
    from_topic_id: req.from_topic_id,
    to_topic_id: req.to_topic_id,
    link_type,
    created_at: now,
  };
  TopicLinkResponse { ok: true, link: Some(link), error: None }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn list_topic_links(topic_id: String) -> ListTopicLinksResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return ListTopicLinksResponse { ok: false, links: vec![], error: Some(e.to_string()) },
  };
  let mut stmt = match conn.prepare(
    "SELECT id, from_topic_id, to_topic_id, link_type, created_at FROM topic_links WHERE from_topic_id = ?1 OR to_topic_id = ?1 ORDER BY created_at DESC",
  ) {
    Ok(s) => s,
    Err(e) => return ListTopicLinksResponse { ok: false, links: vec![], error: Some(e.to_string()) },
  };
  let rows = match stmt.query_map(params![topic_id, topic_id], |row| {
      Ok(TopicLink {
        id: row.get(0)?,
        from_topic_id: row.get(1)?,
        to_topic_id: row.get(2)?,
        link_type: row.get(3)?,
        created_at: row.get(4)?,
      })
    }) {
      Ok(r) => r,
      Err(e) => return ListTopicLinksResponse { ok: false, links: vec![], error: Some(e.to_string()) },
    };
  let links: Vec<TopicLink> = rows.flatten().collect();
  ListTopicLinksResponse { ok: true, links, error: None }
}

// ---------------------------------------------------------------------------
// Generic file save
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct SaveWorkspaceFileRequest {
  pub subdir: String,
  pub topic_id: String,
  pub file_name: String,
  pub content: String,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn save_workspace_file(req: SaveWorkspaceFileRequest) -> SaveWorkspaceFileResponse {
  match write_workspace_file(&req.subdir, &req.topic_id, &req.file_name, &req.content) {
    Ok(path) => SaveWorkspaceFileResponse {
      ok: true,
      file_path: Some(path.to_string_lossy().to_string()),
      error: None,
    },
    Err(e) => SaveWorkspaceFileResponse {
      ok: false,
      file_path: None,
      error: Some(e.to_string()),
    },
  }
}
