use crate::curriculum::UpsertResponse;
use crate::db::{now_rfc3339, open_connection};
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
  pub id: String,
  pub track: String,
  pub name: String,
  pub slug: String,
  pub status: String,
  pub repo_url: Option<String>,
  pub local_path: Option<String>,
  pub readme: Option<String>,
  pub milestone: Option<String>,
  pub notes: Option<String>,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertProjectRequest {
  pub id: Option<String>,
  pub track: String,
  pub name: String,
  pub slug: Option<String>,
  pub status: Option<String>,
  pub repo_url: Option<String>,
  pub local_path: Option<String>,
  pub readme: Option<String>,
  pub milestone: Option<String>,
  pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectResponse {
  pub ok: bool,
  pub project: Option<Project>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListProjectsResponse {
  pub ok: bool,
  pub projects: Vec<Project>,
  pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMilestone {
  pub id: String,
  pub project_id: String,
  pub name: String,
  pub is_complete: bool,
  pub due_date: Option<String>,
  pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertProjectMilestoneRequest {
  pub id: Option<String>,
  pub project_id: String,
  pub name: String,
  pub is_complete: Option<bool>,
  pub due_date: Option<String>,
  pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTask {
  pub id: String,
  pub project_id: String,
  pub title: String,
  pub status: String,
  pub due_date: Option<String>,
  pub notes: Option<String>,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertProjectTaskRequest {
  pub id: Option<String>,
  pub project_id: String,
  pub title: String,
  pub status: Option<String>,
  pub due_date: Option<String>,
  pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectLink {
  pub id: String,
  pub project_id: String,
  pub title: String,
  pub url: String,
  pub link_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertProjectLinkRequest {
  pub id: Option<String>,
  pub project_id: String,
  pub title: String,
  pub url: String,
  pub link_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectNote {
  pub id: String,
  pub project_id: String,
  pub kind: String,
  pub title: String,
  pub content: String,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertProjectNoteRequest {
  pub id: Option<String>,
  pub project_id: String,
  pub kind: Option<String>,
  pub title: String,
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDetailResponse {
  pub ok: bool,
  pub project: Option<Project>,
  pub milestones: Vec<ProjectMilestone>,
  pub tasks: Vec<ProjectTask>,
  pub links: Vec<ProjectLink>,
  pub notes: Vec<ProjectNote>,
  pub error: Option<String>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn slugify(name: &str) -> String {
  let mut out = String::new();
  let mut prev_dash = false;
  for ch in name.chars() {
    if ch.is_ascii_alphanumeric() {
      out.push(ch.to_ascii_lowercase());
      prev_dash = false;
    } else if !prev_dash && !out.is_empty() {
      out.push('-');
      prev_dash = true;
    }
  }
  while out.ends_with('-') {
    out.pop();
  }
  if out.is_empty() {
    "project".to_string()
  } else {
    out
  }
}

fn row_to_project(row: &rusqlite::Row<'_>) -> rusqlite::Result<Project> {
  Ok(Project {
    id: row.get(0)?,
    track: row.get(1)?,
    name: row.get(2)?,
    slug: row.get(3)?,
    status: row.get(4)?,
    repo_url: row.get(5)?,
    local_path: row.get(6)?,
    readme: row.get(7)?,
    milestone: row.get(8)?,
    notes: row.get(9)?,
    created_at: row.get(10)?,
    updated_at: row.get(11)?,
  })
}

const PROJECT_COLUMNS: &str =
  "id, track, name, slug, status, repo_url, local_path, readme, milestone, notes, created_at, updated_at";

// ---------------------------------------------------------------------------
// Projects
// ---------------------------------------------------------------------------

#[tauri::command(rename_all = "snake_case")]
pub async fn list_projects(track: Option<String>) -> ListProjectsResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return ListProjectsResponse { ok: false, projects: vec![], error: Some(e.to_string()) },
  };
  let (sql, filter) = match track.as_deref() {
    Some(t) if !t.is_empty() => (
      format!("SELECT {PROJECT_COLUMNS} FROM projects WHERE track = ?1 ORDER BY updated_at DESC"),
      Some(t.to_string()),
    ),
    _ => (
      format!("SELECT {PROJECT_COLUMNS} FROM projects ORDER BY updated_at DESC"),
      None,
    ),
  };
  let mut stmt = match conn.prepare(&sql) {
    Ok(s) => s,
    Err(e) => return ListProjectsResponse { ok: false, projects: vec![], error: Some(e.to_string()) },
  };
  let mapped = match &filter {
    Some(t) => stmt.query_map(params![t], row_to_project),
    None => stmt.query_map([], row_to_project),
  };
  let rows = match mapped {
    Ok(r) => r,
    Err(e) => return ListProjectsResponse { ok: false, projects: vec![], error: Some(e.to_string()) },
  };
  let projects: Vec<Project> = rows.flatten().collect();
  ListProjectsResponse { ok: true, projects, error: None }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_project(id: String) -> ProjectResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return ProjectResponse { ok: false, project: None, error: Some(e.to_string()) },
  };
  let project = conn
    .query_row(
      &format!("SELECT {PROJECT_COLUMNS} FROM projects WHERE id = ?1"),
      params![id],
      row_to_project,
    )
    .optional()
    .unwrap_or(None);
  ProjectResponse { ok: true, project, error: None }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn upsert_project(req: UpsertProjectRequest) -> UpsertResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let id = req.id.unwrap_or_else(|| Uuid::new_v4().to_string());
  let now = now_rfc3339();
  let slug = req
    .slug
    .filter(|s| !s.trim().is_empty())
    .unwrap_or_else(|| slugify(&req.name));
  let status = req.status.unwrap_or_else(|| "active".to_string());

  let result = conn.execute(
    "INSERT INTO projects (id, track, name, slug, status, repo_url, local_path, readme, milestone, notes, created_at, updated_at)
     VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)
     ON CONFLICT(id) DO UPDATE SET
       track = excluded.track,
       name = excluded.name,
       slug = excluded.slug,
       status = excluded.status,
       repo_url = excluded.repo_url,
       local_path = excluded.local_path,
       readme = excluded.readme,
       milestone = excluded.milestone,
       notes = excluded.notes,
       updated_at = excluded.updated_at",
    params![
      id,
      req.track,
      req.name,
      slug,
      status,
      req.repo_url,
      req.local_path,
      req.readme,
      req.milestone,
      req.notes,
      now,
      now
    ],
  );

  if let Err(e) = result {
    return UpsertResponse { ok: false, id: e.to_string() };
  }
  UpsertResponse { ok: true, id }
}

// ---------------------------------------------------------------------------
// Milestones
// ---------------------------------------------------------------------------

#[tauri::command(rename_all = "snake_case")]
pub async fn upsert_project_milestone(req: UpsertProjectMilestoneRequest) -> UpsertResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let id = req.id.unwrap_or_else(|| Uuid::new_v4().to_string());
  let is_complete = req.is_complete.unwrap_or(false);

  let result = conn.execute(
    "INSERT INTO project_milestones (id, project_id, name, is_complete, due_date, notes)
     VALUES (?1,?2,?3,?4,?5,?6)
     ON CONFLICT(id) DO UPDATE SET
       project_id = excluded.project_id,
       name = excluded.name,
       is_complete = excluded.is_complete,
       due_date = excluded.due_date,
       notes = excluded.notes",
    params![id, req.project_id, req.name, is_complete as i32, req.due_date, req.notes],
  );

  if let Err(e) = result {
    return UpsertResponse { ok: false, id: e.to_string() };
  }
  UpsertResponse { ok: true, id }
}

// ---------------------------------------------------------------------------
// Tasks
// ---------------------------------------------------------------------------

#[tauri::command(rename_all = "snake_case")]
pub async fn upsert_project_task(req: UpsertProjectTaskRequest) -> UpsertResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let id = req.id.unwrap_or_else(|| Uuid::new_v4().to_string());
  let now = now_rfc3339();
  let status = req.status.unwrap_or_else(|| "todo".to_string());

  let result = conn.execute(
    "INSERT INTO project_tasks (id, project_id, title, status, due_date, notes, created_at, updated_at)
     VALUES (?1,?2,?3,?4,?5,?6,?7,?8)
     ON CONFLICT(id) DO UPDATE SET
       project_id = excluded.project_id,
       title = excluded.title,
       status = excluded.status,
       due_date = excluded.due_date,
       notes = excluded.notes,
       updated_at = excluded.updated_at",
    params![id, req.project_id, req.title, status, req.due_date, req.notes, now, now],
  );

  if let Err(e) = result {
    return UpsertResponse { ok: false, id: e.to_string() };
  }
  UpsertResponse { ok: true, id }
}

// ---------------------------------------------------------------------------
// Links
// ---------------------------------------------------------------------------

#[tauri::command(rename_all = "snake_case")]
pub async fn upsert_project_link(req: UpsertProjectLinkRequest) -> UpsertResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let id = req.id.unwrap_or_else(|| Uuid::new_v4().to_string());

  let result = conn.execute(
    "INSERT INTO project_links (id, project_id, title, url, type)
     VALUES (?1,?2,?3,?4,?5)
     ON CONFLICT(id) DO UPDATE SET
       project_id = excluded.project_id,
       title = excluded.title,
       url = excluded.url,
       type = excluded.type",
    params![id, req.project_id, req.title, req.url, req.link_type],
  );

  if let Err(e) = result {
    return UpsertResponse { ok: false, id: e.to_string() };
  }
  UpsertResponse { ok: true, id }
}

// ---------------------------------------------------------------------------
// Notes journal
// ---------------------------------------------------------------------------

#[tauri::command(rename_all = "snake_case")]
pub async fn upsert_project_note(req: UpsertProjectNoteRequest) -> UpsertResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let id = req.id.unwrap_or_else(|| Uuid::new_v4().to_string());
  let now = now_rfc3339();
  let kind = req.kind.unwrap_or_else(|| "note".to_string());

  let result = conn.execute(
    "INSERT INTO project_notes (id, project_id, kind, title, content, created_at, updated_at)
     VALUES (?1,?2,?3,?4,?5,?6,?7)
     ON CONFLICT(id) DO UPDATE SET
       project_id = excluded.project_id,
       kind = excluded.kind,
       title = excluded.title,
       content = excluded.content,
       updated_at = excluded.updated_at",
    params![id, req.project_id, kind, req.title, req.content, now, now],
  );

  if let Err(e) = result {
    return UpsertResponse { ok: false, id: e.to_string() };
  }
  UpsertResponse { ok: true, id }
}

// ---------------------------------------------------------------------------
// Aggregated detail
// ---------------------------------------------------------------------------

#[tauri::command(rename_all = "snake_case")]
pub async fn get_project_detail(id: String) -> ProjectDetailResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(e) => {
      return ProjectDetailResponse {
        ok: false,
        project: None,
        milestones: vec![],
        tasks: vec![],
        links: vec![],
        notes: vec![],
        error: Some(e.to_string()),
      }
    }
  };

  let project = match conn
    .query_row(
      &format!("SELECT {PROJECT_COLUMNS} FROM projects WHERE id = ?1"),
      params![id],
      row_to_project,
    )
    .optional()
  {
    Ok(p) => p,
    Err(e) => {
      return ProjectDetailResponse {
        ok: false,
        project: None,
        milestones: vec![],
        tasks: vec![],
        links: vec![],
        notes: vec![],
        error: Some(e.to_string()),
      }
    }
  };

  let milestones = match conn.prepare(
    "SELECT id, project_id, name, is_complete, due_date, notes FROM project_milestones WHERE project_id = ?1 ORDER BY name",
  ) {
    Ok(mut stmt) => stmt
      .query_map(params![id], |row| {
        Ok(ProjectMilestone {
          id: row.get(0)?,
          project_id: row.get(1)?,
          name: row.get(2)?,
          is_complete: row.get::<_, i64>(3)? != 0,
          due_date: row.get(4)?,
          notes: row.get(5)?,
        })
      })
      .map(|rows| rows.flatten().collect())
      .unwrap_or_default(),
    Err(_) => vec![],
  };

  let tasks = match conn.prepare(
    "SELECT id, project_id, title, status, due_date, notes, created_at, updated_at FROM project_tasks WHERE project_id = ?1 ORDER BY created_at",
  ) {
    Ok(mut stmt) => stmt
      .query_map(params![id], |row| {
        Ok(ProjectTask {
          id: row.get(0)?,
          project_id: row.get(1)?,
          title: row.get(2)?,
          status: row.get(3)?,
          due_date: row.get(4)?,
          notes: row.get(5)?,
          created_at: row.get(6)?,
          updated_at: row.get(7)?,
        })
      })
      .map(|rows| rows.flatten().collect())
      .unwrap_or_default(),
    Err(_) => vec![],
  };

  let links = match conn.prepare(
    "SELECT id, project_id, title, url, type FROM project_links WHERE project_id = ?1 ORDER BY title",
  ) {
    Ok(mut stmt) => stmt
      .query_map(params![id], |row| {
        Ok(ProjectLink {
          id: row.get(0)?,
          project_id: row.get(1)?,
          title: row.get(2)?,
          url: row.get(3)?,
          link_type: row.get(4)?,
        })
      })
      .map(|rows| rows.flatten().collect())
      .unwrap_or_default(),
    Err(_) => vec![],
  };

  let notes = match conn.prepare(
    "SELECT id, project_id, kind, title, content, created_at, updated_at FROM project_notes WHERE project_id = ?1 ORDER BY updated_at DESC",
  ) {
    Ok(mut stmt) => stmt
      .query_map(params![id], |row| {
        Ok(ProjectNote {
          id: row.get(0)?,
          project_id: row.get(1)?,
          kind: row.get(2)?,
          title: row.get(3)?,
          content: row.get(4)?,
          created_at: row.get(5)?,
          updated_at: row.get(6)?,
        })
      })
      .map(|rows| rows.flatten().collect())
      .unwrap_or_default(),
    Err(_) => vec![],
  };

  ProjectDetailResponse {
    ok: true,
    project,
    milestones,
    tasks,
    links,
    notes,
    error: None,
  }
}

// ---------------------------------------------------------------------------
// FDE project bootstrap (idempotent)
// ---------------------------------------------------------------------------

const FDE_MILESTONES: [(&str, &str); 6] = [
  (
    "M1 \u{2014} Problem definition",
    "Weeks 1\u{2013}4 \u{00b7} One-page problem statement, user journey, success metric, architecture sketch",
  ),
  (
    "M2 \u{2014} Basic model integration",
    "Weeks 5\u{2013}8 \u{00b7} Working API call, structured output, validation, logs",
  ),
  (
    "M3 \u{2014} Useful context/tools",
    "Weeks 9\u{2013}13 \u{00b7} RAG or tool-calling vertical slice (only add both if the use case needs both)",
  ),
  (
    "M4 \u{2014} Reliability and evaluation",
    "Weeks 14\u{2013}18 \u{00b7} Golden dataset, regression evaluation, error handling, authorization, telemetry",
  ),
  (
    "M5 \u{2014} Production hardening",
    "Weeks 19\u{2013}22 \u{00b7} Cost/latency controls, deployment/reproducibility, documentation",
  ),
  (
    "M6 \u{2014} Technical walkthrough",
    "Weeks 23\u{2013}26 \u{00B7} Architecture explanation, demo, trade-offs, known limitations, next steps",
  ),
];

#[tauri::command(rename_all = "snake_case")]
pub async fn seed_fde_project() -> UpsertResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return UpsertResponse { ok: false, id: e.to_string() },
  };
  let _ = crate::migrations::ensure_schema(&mut conn);

  let existing: Option<String> = conn
    .query_row(
      "SELECT id FROM projects WHERE track = 'E' ORDER BY created_at LIMIT 1",
      [],
      |row| row.get(0),
    )
    .optional()
    .unwrap_or(None);

  let project_id = match existing {
    Some(id) => id,
    None => {
      let id = Uuid::new_v4().to_string();
      let now = now_rfc3339();
      let result = conn.execute(
        "INSERT INTO projects (id, track, name, slug, status, repo_url, local_path, readme, milestone, notes, created_at, updated_at)
         VALUES (?1, 'E', ?2, ?3, 'active', NULL, NULL, NULL, 'M1', NULL, ?4, ?4)",
        params![id, "FDE Capstone Project", "fde-capstone", now],
      );
      if let Err(e) = result {
        return UpsertResponse { ok: false, id: e.to_string() };
      }
      id
    }
  };

  // Seed M1-M6 only when the project has no milestones yet, so user edits and
  // completions are never overwritten on repeat calls.
  let existing_count: i64 = conn
    .query_row(
      "SELECT COUNT(*) FROM project_milestones WHERE project_id = ?1",
      params![project_id],
      |row| row.get(0),
    )
    .unwrap_or(0);

  if existing_count == 0 {
    for (name, notes) in FDE_MILESTONES.iter() {
      let mid = Uuid::new_v4().to_string();
      let _ = conn.execute(
        "INSERT INTO project_milestones (id, project_id, name, is_complete, due_date, notes)
         VALUES (?1, ?2, ?3, 0, NULL, ?4)",
        params![mid, project_id, name, notes],
      );
    }
  }

  UpsertResponse { ok: true, id: project_id }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn slugify_produces_url_safe_slugs() {
    assert_eq!(slugify("FDE Capstone Project"), "fde-capstone-project");
    assert_eq!(slugify("  Multi---Agent!! "), "multi-agent");
    assert_eq!(slugify(""), "project");
  }
}
