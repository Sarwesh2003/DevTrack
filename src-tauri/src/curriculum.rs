use crate::db::{now_rfc3339, open_connection, DbResult};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
  pub id: String,
  pub key: String,
  pub name: String,
  pub description: Option<String>,
  pub position: i32,
  pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
  pub id: String,
  pub track_id: String,
  pub key: String,
  pub name: String,
  pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
  pub id: String,
  pub module_id: String,
  pub key: String,
  pub title: String,
  pub status: String,
  pub priority: String,
  pub estimate_minutes: Option<i32>,
  pub confidence: Option<i32>,
  pub completion_criteria: Option<String>,
  pub notes: Option<String>,
  pub evidence: Option<String>,
  pub target_week: Option<i32>,
  pub is_custom: bool,
  pub source_ref: Option<String>,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtopic {
  pub id: String,
  pub topic_id: String,
  pub key: String,
  pub title: String,
  pub is_complete: bool,
  pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
  pub id: String,
  pub topic_id: Option<String>,
  pub title: String,
  pub url: Option<String>,
  pub r#type: Option<String>,
  pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicDependency {
  pub id: String,
  pub topic_id: String,
  pub depends_on_topic_id: String,
  pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct UpsertTopicRequest {
  pub module_id: String,
  pub key: String,
  pub title: String,
  pub status: Option<String>,
  pub priority: Option<String>,
  pub estimate_minutes: Option<i32>,
  pub confidence: Option<i32>,
  pub completion_criteria: Option<String>,
  pub notes: Option<String>,
  pub target_week: Option<i32>,
  pub is_custom: Option<bool>,
  pub source_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertSubtopicRequest {
  pub topic_id: String,
  pub key: String,
  pub title: String,
  pub is_complete: Option<bool>,
  pub position: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertResourceRequest {
  pub topic_id: Option<String>,
  pub title: String,
  pub url: Option<String>,
  pub r#type: Option<String>,
  pub position: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertDependencyRequest {
  pub topic_id: String,
  pub depends_on_topic_id: String,
  pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackListResponse {
  pub ok: bool,
  pub tracks: Vec<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleListResponse {
  pub ok: bool,
  pub modules: Vec<Module>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicListResponse {
  pub ok: bool,
  pub topics: Vec<Topic>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicDetailResponse {
  pub ok: bool,
  pub topic: Option<Topic>,
  pub subtopics: Vec<Subtopic>,
  pub resources: Vec<Resource>,
  pub dependencies: Vec<TopicDependency>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpsertResponse {
  pub ok: bool,
  pub id: String,
}

#[tauri::command]
pub async fn get_tracks() -> TrackListResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(_) => return TrackListResponse { ok: false, tracks: vec![] },
  };

  let mut stmt = match conn.prepare("SELECT id, key, name, description, position, created_at FROM tracks ORDER BY position") {
    Ok(s) => s,
    Err(_) => return TrackListResponse { ok: false, tracks: vec![] },
  };

  let rows = match stmt.query_map([], |row| {
    Ok(Track {
      id: row.get(0)?,
      key: row.get(1)?,
      name: row.get(2)?,
      description: row.get(3)?,
      position: row.get(4)?,
      created_at: row.get(5)?,
    })
  }) {
    Ok(r) => r,
    Err(_) => return TrackListResponse { ok: false, tracks: vec![] },
  };

  let mut tracks = Vec::new();
  for row in rows.flatten() {
    tracks.push(row);
  }
  TrackListResponse { ok: true, tracks }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_modules(track_key: String) -> ModuleListResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(_) => return ModuleListResponse { ok: false, modules: vec![] },
  };

  let mut stmt = match conn.prepare(
    "SELECT m.id, m.track_id, m.key, m.name, m.position FROM modules m JOIN tracks t ON m.track_id = t.id WHERE t.key = ?1 ORDER BY m.position"
  ) {
    Ok(s) => s,
    Err(_) => return ModuleListResponse { ok: false, modules: vec![] },
  };

  let rows = match stmt.query_map(params![track_key], |row| {
    Ok(Module {
      id: row.get(0)?,
      track_id: row.get(1)?,
      key: row.get(2)?,
      name: row.get(3)?,
      position: row.get(4)?,
    })
  }) {
    Ok(r) => r,
    Err(_) => return ModuleListResponse { ok: false, modules: vec![] },
  };

  let mut modules = Vec::new();
  for row in rows.flatten() {
    modules.push(row);
  }
  ModuleListResponse { ok: true, modules }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_topics(module_id: String) -> TopicListResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(_) => return TopicListResponse { ok: false, topics: vec![] },
  };

  let mut stmt = match conn.prepare(
    "SELECT id, module_id, key, title, status, priority, estimate_minutes, confidence, completion_criteria, notes, evidence, target_week, is_custom, source_ref, created_at, updated_at FROM topics WHERE module_id = ?1 ORDER BY position"
  ) {
    Ok(s) => s,
    Err(_) => return TopicListResponse { ok: false, topics: vec![] },
  };

  let rows = match stmt.query_map(params![module_id], |row| {
    Ok(Topic {
      id: row.get(0)?,
      module_id: row.get(1)?,
      key: row.get(2)?,
      title: row.get(3)?,
      status: row.get(4)?,
      priority: row.get(5)?,
      estimate_minutes: row.get(6)?,
      confidence: row.get(7)?,
      completion_criteria: row.get(8)?,
      notes: row.get(9)?,
      evidence: row.get(10)?,
      target_week: row.get(11)?,
      is_custom: row.get::<_, i32>(12)? != 0,
      source_ref: row.get(13)?,
      created_at: row.get(14)?,
      updated_at: row.get(15)?,
    })
  }) {
    Ok(r) => r,
    Err(_) => return TopicListResponse { ok: false, topics: vec![] },
  };

  let mut topics = Vec::new();
  for row in rows.flatten() {
    topics.push(row);
  }
  TopicListResponse { ok: true, topics }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_topic_detail(topic_id: String) -> TopicDetailResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(_) => return TopicDetailResponse { ok: false, topic: None, subtopics: vec![], resources: vec![], dependencies: vec![] },
  };

  let topic = conn.query_row(
    "SELECT id, module_id, key, title, status, priority, estimate_minutes, confidence, completion_criteria, notes, evidence, target_week, is_custom, source_ref, created_at, updated_at FROM topics WHERE id = ?1",
    params![topic_id],
    |row| {
      Ok(Topic {
        id: row.get(0)?,
        module_id: row.get(1)?,
        key: row.get(2)?,
        title: row.get(3)?,
        status: row.get(4)?,
        priority: row.get(5)?,
        estimate_minutes: row.get(6)?,
        confidence: row.get(7)?,
        completion_criteria: row.get(8)?,
        notes: row.get(9)?,
        evidence: row.get(10)?,
        target_week: row.get(11)?,
        is_custom: row.get::<_, i32>(12)? != 0,
        source_ref: row.get(13)?,
        created_at: row.get(14)?,
        updated_at: row.get(15)?,
      })
    },
  );

  let topic = match topic {
    Ok(t) => Some(t),
    Err(_) => None,
  };

  let subtopics = {
    let mut stmt = match conn.prepare("SELECT id, topic_id, key, title, is_complete, position FROM subtopics WHERE topic_id = ?1 ORDER BY position") {
      Ok(s) => s,
      Err(_) => return TopicDetailResponse { ok: true, topic, subtopics: vec![], resources: vec![], dependencies: vec![] },
    };
    let rows = match stmt.query_map(params![topic_id], |row| {
      Ok(Subtopic {
        id: row.get(0)?,
        topic_id: row.get(1)?,
        key: row.get(2)?,
        title: row.get(3)?,
        is_complete: row.get::<_, i32>(4)? != 0,
        position: row.get(5)?,
      })
    }) {
      Ok(r) => r,
      Err(_) => return TopicDetailResponse { ok: true, topic, subtopics: vec![], resources: vec![], dependencies: vec![] },
    };
    let mut v = Vec::new();
    for row in rows.flatten() { v.push(row); }
    v
  };

  let resources = {
    let mut stmt = match conn.prepare("SELECT id, topic_id, title, url, type, position FROM resources WHERE topic_id = ?1 ORDER BY position") {
      Ok(s) => s,
      Err(_) => return TopicDetailResponse { ok: true, topic, subtopics, resources: vec![], dependencies: vec![] },
    };
    let rows = match stmt.query_map(params![topic_id], |row| {
      Ok(Resource {
        id: row.get(0)?,
        topic_id: row.get(1)?,
        title: row.get(2)?,
        url: row.get(3)?,
        r#type: row.get(4)?,
        position: row.get(5)?,
      })
    }) {
      Ok(r) => r,
      Err(_) => return TopicDetailResponse { ok: true, topic, subtopics, resources: vec![], dependencies: vec![] },
    };
    let mut v = Vec::new();
    for row in rows.flatten() { v.push(row); }
    v
  };

  let dependencies = {
    let mut stmt = match conn.prepare("SELECT id, topic_id, depends_on_topic_id, type FROM topic_dependencies WHERE topic_id = ?1") {
      Ok(s) => s,
      Err(_) => return TopicDetailResponse { ok: true, topic, subtopics, resources, dependencies: vec![] },
    };
    let rows = match stmt.query_map(params![topic_id], |row| {
      Ok(TopicDependency {
        id: row.get(0)?,
        topic_id: row.get(1)?,
        depends_on_topic_id: row.get(2)?,
        r#type: row.get(3)?,
      })
    }) {
      Ok(r) => r,
      Err(_) => return TopicDetailResponse { ok: true, topic, subtopics, resources, dependencies: vec![] },
    };
    let mut v = Vec::new();
    for row in rows.flatten() { v.push(row); }
    v
  };

  TopicDetailResponse { ok: true, topic, subtopics, resources, dependencies }
}

#[tauri::command]
pub async fn upsert_topic(req: UpsertTopicRequest) -> UpsertResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };

  let status = req.status.clone().unwrap_or_else(|| "Not started".to_string());
  let priority = req.priority.clone().unwrap_or_else(|| "Medium".to_string());
  let is_custom = req.is_custom.unwrap_or(false);

  let id = match conn.query_row("SELECT id FROM topics WHERE key = ?1 AND module_id = ?2", params![req.key, req.module_id], |row| row.get::<_, String>(0)) {
    Ok(existing_id) => existing_id,
    Err(_) => {
      let new_id = Uuid::new_v4().to_string();
      let now = now_rfc3339();
      let _ = conn.execute(
        "INSERT INTO topics (id, module_id, key, title, status, priority, estimate_minutes, confidence, completion_criteria, notes, target_week, is_custom, source_ref, created_at, updated_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)",
        params![
          new_id, req.module_id, req.key, req.title,
          status,
          priority,
          req.estimate_minutes, req.confidence,
          req.completion_criteria, req.notes,
          req.target_week,
          if is_custom { 1 } else { 0 },
          req.source_ref,
          now, now,
        ],
      );
      new_id
    }
  };

  let now = now_rfc3339();
  let _ = conn.execute(
    "UPDATE topics SET title = ?1, status = ?2, priority = ?3, estimate_minutes = ?4, confidence = ?5, completion_criteria = ?6, notes = ?7, target_week = ?8, is_custom = ?9, source_ref = ?10, updated_at = ?11 WHERE id = ?12",
    params![
      req.title,
      status,
      priority,
      req.estimate_minutes, req.confidence,
      req.completion_criteria, req.notes,
      req.target_week,
      if is_custom { 1 } else { 0 },
      req.source_ref,
      now, id,
    ],
  );

  UpsertResponse { ok: true, id }
}

#[tauri::command]
pub async fn upsert_subtopic(req: UpsertSubtopicRequest) -> UpsertResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };

  let is_complete = req.is_complete.unwrap_or(false);
  let position = req.position.unwrap_or(0);

  let id = match conn.query_row("SELECT id FROM subtopics WHERE key = ?1 AND topic_id = ?2", params![req.key, req.topic_id], |row| row.get::<_, String>(0)) {
    Ok(existing_id) => existing_id,
    Err(_) => {
      let new_id = Uuid::new_v4().to_string();
      let _ = conn.execute(
        "INSERT INTO subtopics (id, topic_id, key, title, is_complete, position) VALUES (?1,?2,?3,?4,?5,?6)",
        params![new_id, req.topic_id, req.key, req.title, if is_complete { 1 } else { 0 }, position],
      );
      new_id
    }
  };

  let _ = conn.execute(
    "UPDATE subtopics SET title = ?1, is_complete = ?2, position = ?3 WHERE id = ?4",
    params![req.title, if is_complete { 1 } else { 0 }, position, id],
  );

  UpsertResponse { ok: true, id }
}

#[tauri::command]
pub async fn upsert_resource(req: UpsertResourceRequest) -> UpsertResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };

  let id = Uuid::new_v4().to_string();
  let position = req.position.unwrap_or(0);
  let _ = conn.execute(
    "INSERT INTO resources (id, topic_id, title, url, type, position) VALUES (?1,?2,?3,?4,?5,?6)",
    params![id, req.topic_id, req.title, req.url, req.r#type, position],
  );

  UpsertResponse { ok: true, id }
}

#[tauri::command]
pub async fn upsert_dependency(req: UpsertDependencyRequest) -> UpsertResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };

  let id = Uuid::new_v4().to_string();
  let _ = conn.execute(
    "INSERT OR IGNORE INTO topic_dependencies (id, topic_id, depends_on_topic_id, type) VALUES (?1,?2,?3,?4)",
    params![id, req.topic_id, req.depends_on_topic_id, req.r#type],
  );

  UpsertResponse { ok: true, id }
}
