use crate::db::{
  check_db_health, get_default_settings, load_settings, open_connection, resolve_db_path,
  store_settings, Settings,
};
use crate::migrations::SCHEMA_VERSION;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
  pub ok: bool,
  pub health: crate::db::DbHealth,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsResponse {
  pub ok: bool,
  pub settings: Settings,
}

#[derive(Debug, Deserialize)]
pub struct SaveSettingsRequest {
  pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveSettingsResponse {
  pub ok: bool,
}

#[derive(Debug, Deserialize)]
pub struct BackupRequest {
  pub destination: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupResponse {
  pub ok: bool,
  pub path: String,
  pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RestoreRequest {
  pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreResponse {
  pub ok: bool,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MigrationsResponse {
  pub ok: bool,
  pub version: i32,
  pub error: Option<String>,
}

#[tauri::command]
pub async fn health_check() -> HealthResponse {
  match check_db_health() {
    Ok(h) => HealthResponse { ok: true, health: h },
    Err(e) => HealthResponse {
      ok: false,
      health: crate::db::DbHealth {
        connected: false,
        path: e.to_string(),
        version: 0,
      },
    },
  }
}

#[tauri::command]
pub async fn get_settings() -> SettingsResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(_) => {
      return SettingsResponse {
        ok: true,
        settings: get_default_settings(),
      }
    }
  };
  let _ = crate::migrations::ensure_schema(&mut conn);
  SettingsResponse {
    ok: true,
    settings: load_settings(&conn),
  }
}

#[tauri::command]
pub async fn save_settings(req: SaveSettingsRequest) -> SaveSettingsResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(_) => return SaveSettingsResponse { ok: false },
  };
  if crate::migrations::ensure_schema(&mut conn).is_err() {
    return SaveSettingsResponse { ok: false };
  }
  match store_settings(&conn, &req.settings) {
    Ok(()) => SaveSettingsResponse { ok: true },
    Err(_) => SaveSettingsResponse { ok: false },
  }
}

#[tauri::command]
pub async fn backup_database(_app: AppHandle, req: BackupRequest) -> BackupResponse {
  let source = match resolve_db_path() {
    Ok(p) => p,
    Err(e) => return BackupResponse { ok: false, path: "".to_string(), error: Some(e.to_string()) },
  };

  if !source.exists() {
    return BackupResponse { ok: false, path: "".to_string(), error: Some("database does not exist".to_string()) };
  }

  let dest = std::path::Path::new(&req.destination);
  if let Some(parent) = dest.parent() {
    if let Err(e) = std::fs::create_dir_all(parent) {
      return BackupResponse { ok: false, path: req.destination.clone(), error: Some(e.to_string()) };
    }
  }

  match std::fs::copy(&source, dest) {
    Ok(_) => BackupResponse { ok: true, path: req.destination.clone(), error: None },
    Err(e) => BackupResponse { ok: false, path: req.destination.clone(), error: Some(e.to_string()) },
  }
}

#[tauri::command]
pub async fn restore_database(_app: AppHandle, req: RestoreRequest) -> RestoreResponse {
  let source = std::path::Path::new(&req.source);
  if !source.exists() {
    return RestoreResponse { ok: false, error: Some("source file does not exist".to_string()) };
  }

  let dest = match resolve_db_path() {
    Ok(p) => p,
    Err(e) => return RestoreResponse { ok: false, error: Some(e.to_string()) },
  };

  if let Some(parent) = dest.parent() {
    if let Err(e) = std::fs::create_dir_all(parent) {
      return RestoreResponse { ok: false, error: Some(e.to_string()) };
    }
  }

  match std::fs::copy(source, &dest) {
    Ok(_) => RestoreResponse { ok: true, error: None },
    Err(e) => RestoreResponse { ok: false, error: Some(e.to_string()) },
  }
}

#[tauri::command]
pub async fn run_migrations(_app: AppHandle) -> MigrationsResponse {
  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return MigrationsResponse { ok: false, version: 0, error: Some(e.to_string()) },
  };

  match crate::migrations::ensure_schema(&mut conn) {
    Ok(()) => MigrationsResponse { ok: true, version: SCHEMA_VERSION, error: None },
    Err(e) => MigrationsResponse { ok: false, version: 0, error: Some(e.to_string()) },
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadmapPhase {
  pub id: String,
  pub key: String,
  pub name: String,
  pub position: i32,
  pub source_ref: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadmapWeek {
  pub id: String,
  pub phase_id: Option<String>,
  pub week_number: i32,
  pub track_allocations: Option<String>,
  pub total_estimate_minutes: Option<i32>,
  pub source_ref: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeeklyPlanItem {
  pub id: String,
  pub week_id: String,
  pub topic_id: Option<String>,
  pub problem_id: Option<String>,
  pub task_type: String,
  pub title: String,
  pub estimate_minutes: i32,
  pub position: i32,
  pub day_of_week: Option<i32>,
  pub source_ref: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadmapPhasesResponse {
  pub ok: bool,
  pub phases: Vec<RoadmapPhase>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadmapWeeksResponse {
  pub ok: bool,
  pub weeks: Vec<RoadmapWeek>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeeklyPlanItemsResponse {
  pub ok: bool,
  pub items: Vec<WeeklyPlanItem>,
}

#[tauri::command]
pub async fn get_roadmap_phases() -> RoadmapPhasesResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(_) => return RoadmapPhasesResponse { ok: false, phases: vec![] },
  };

  let mut stmt = match conn.prepare("SELECT id, key, name, position, source_ref FROM roadmap_phases ORDER BY position") {
    Ok(s) => s,
    Err(_) => return RoadmapPhasesResponse { ok: false, phases: vec![] },
  };

  let rows = match stmt.query_map([], |row| {
    Ok(RoadmapPhase {
      id: row.get(0)?,
      key: row.get(1)?,
      name: row.get(2)?,
      position: row.get(3)?,
      source_ref: row.get(4)?,
    })
  }) {
    Ok(r) => r,
    Err(_) => return RoadmapPhasesResponse { ok: false, phases: vec![] },
  };

  let mut phases = Vec::new();
  for row in rows.flatten() {
    phases.push(row);
  }
  RoadmapPhasesResponse { ok: true, phases }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_roadmap_weeks(phase_key: Option<String>) -> RoadmapWeeksResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(_) => return RoadmapWeeksResponse { ok: false, weeks: vec![] },
  };

  let weeks: Vec<RoadmapWeek> = match phase_key {
    Some(key) => {
      let mut stmt = match conn.prepare("SELECT w.id, w.phase_id, w.week_number, w.track_allocations, w.total_estimate_minutes, w.source_ref FROM roadmap_weeks w JOIN roadmap_phases p ON w.phase_id = p.id WHERE p.key = ?1 ORDER BY w.week_number") {
        Ok(s) => s,
        Err(_) => return RoadmapWeeksResponse { ok: false, weeks: vec![] },
      };
      let rows = match stmt.query_map(params![key], |row| {
        Ok(RoadmapWeek {
          id: row.get(0)?,
          phase_id: row.get(1)?,
          week_number: row.get(2)?,
          track_allocations: row.get(3)?,
          total_estimate_minutes: row.get(4)?,
          source_ref: row.get(5)?,
        })
      }) {
        Ok(r) => r,
        Err(_) => return RoadmapWeeksResponse { ok: false, weeks: vec![] },
      };
      let mut v = Vec::new();
      for row in rows.flatten() { v.push(row); }
      v
    }
    None => {
      let mut stmt = match conn.prepare("SELECT id, phase_id, week_number, track_allocations, total_estimate_minutes, source_ref FROM roadmap_weeks ORDER BY week_number") {
        Ok(s) => s,
        Err(_) => return RoadmapWeeksResponse { ok: false, weeks: vec![] },
      };
      let rows = match stmt.query_map([], |row| {
        Ok(RoadmapWeek {
          id: row.get(0)?,
          phase_id: row.get(1)?,
          week_number: row.get(2)?,
          track_allocations: row.get(3)?,
          total_estimate_minutes: row.get(4)?,
          source_ref: row.get(5)?,
        })
      }) {
        Ok(r) => r,
        Err(_) => return RoadmapWeeksResponse { ok: false, weeks: vec![] },
      };
      let mut v = Vec::new();
      for row in rows.flatten() { v.push(row); }
      v
    }
  };

  RoadmapWeeksResponse { ok: true, weeks }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_weekly_plan_items(week_id: String) -> WeeklyPlanItemsResponse {
  let conn = match open_connection() {
    Ok(c) => c,
    Err(_) => return WeeklyPlanItemsResponse { ok: false, items: vec![] },
  };

  let mut stmt = match conn.prepare("SELECT id, week_id, topic_id, problem_id, task_type, title, estimate_minutes, position, day_of_week, source_ref FROM weekly_plan_items WHERE week_id = ?1 ORDER BY position") {
    Ok(s) => s,
    Err(_) => return WeeklyPlanItemsResponse { ok: false, items: vec![] },
  };

  let rows = match stmt.query_map(params![week_id], |row| {
    Ok(WeeklyPlanItem {
      id: row.get(0)?,
      week_id: row.get(1)?,
      topic_id: row.get(2)?,
      problem_id: row.get(3)?,
      task_type: row.get(4)?,
      title: row.get(5)?,
      estimate_minutes: row.get(6)?,
      position: row.get(7)?,
      day_of_week: row.get(8)?,
      source_ref: row.get(9)?,
    })
  }) {
    Ok(r) => r,
    Err(_) => return WeeklyPlanItemsResponse { ok: false, items: vec![] },
  };

  let mut items = Vec::new();
  for row in rows.flatten() {
    items.push(row);
  }
  WeeklyPlanItemsResponse { ok: true, items }
}

