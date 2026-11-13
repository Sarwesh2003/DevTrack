use chrono::Utc;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
  #[error("SQL error: {0}")]
  Sql(#[from] rusqlite::Error),
  #[error("IO error: {0}")]
  Io(#[from] std::io::Error),
  #[error("Serialization error: {0}")]
  Serialization(String),
}

pub type DbResult<T> = Result<T, DbError>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DbHealth {
  pub connected: bool,
  pub path: String,
  pub version: i32,
}

pub fn app_data_dir() -> DbResult<PathBuf> {
  dirs::data_dir()
    .ok_or_else(|| {
      DbError::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "could not resolve app data dir",
      ))
    })
    .map(|p| p.join("DevTrack"))
}

pub fn init_data_dir() -> DbResult<PathBuf> {
  let data_dir = app_data_dir()?;
  std::fs::create_dir_all(&data_dir).map_err(DbError::Io)?;

  let content_dir = data_dir.join("content");
  std::fs::create_dir_all(&content_dir).map_err(DbError::Io)?;

  Ok(data_dir)
}

pub fn resolve_db_path() -> DbResult<PathBuf> {
  let data_dir = init_data_dir()?;
  Ok(data_dir.join("devtrack.sqlite"))
}

pub fn check_db_health() -> DbResult<DbHealth> {
  let path = resolve_db_path()?;
  let exists = path.exists();
  let version = if exists { crate::migrations::SCHEMA_VERSION } else { 0 };

  Ok(DbHealth {
    connected: exists,
    path: path.to_string_lossy().to_string(),
    version,
  })
}

pub fn open_connection() -> DbResult<rusqlite::Connection> {
  let path = resolve_db_path()?;
  let conn = rusqlite::Connection::open(&path).map_err(DbError::Sql)?;
  conn.pragma_update(None, "journal_mode", "WAL").map_err(DbError::Sql)?;
  conn.pragma_update(None, "foreign_keys", "ON").map_err(DbError::Sql)?;
  Ok(conn)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
  pub data_root: Option<String>,
  pub weekly_capacity_hours: Option<f64>,
  pub timezone: Option<String>,
  pub theme: Option<String>,
  pub dsa_cursor_category: Option<String>,
  pub lld_cursor_topic: Option<String>,
}

impl Default for Settings {
  fn default() -> Self {
    Settings {
      data_root: None,
      weekly_capacity_hours: Some(7.5),
      timezone: Some("Asia/Kolkata".to_string()),
      theme: Some("dark".to_string()),
      dsa_cursor_category: Some("Tries".to_string()),
      lld_cursor_topic: Some("Bridge Pattern".to_string()),
    }
  }
}

pub fn get_default_settings() -> Settings {
  Settings::default()
}

pub fn now_rfc3339() -> String {
  Utc::now().to_rfc3339()
}
