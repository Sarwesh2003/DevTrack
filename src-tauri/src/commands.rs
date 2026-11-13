use crate::db::{check_db_health, get_default_settings, open_connection, resolve_db_path, Settings};
use crate::migrations::SCHEMA_VERSION;
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
  SettingsResponse {
    ok: true,
    settings: get_default_settings(),
  }
}

#[tauri::command]
pub async fn save_settings(_req: SaveSettingsRequest) -> SaveSettingsResponse {
  SaveSettingsResponse { ok: true }
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
