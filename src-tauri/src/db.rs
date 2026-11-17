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
  #[serde(default)]
  pub study_days: Option<Vec<i32>>,
  #[serde(default)]
  pub cadence: Option<std::collections::HashMap<String, Vec<String>>>,
  #[serde(default)]
  pub min_meaningful_minutes: Option<i32>,
  #[serde(default)]
  pub daily_buffer_ratio: Option<f64>,
}

impl Default for Settings {
  fn default() -> Self {
    let mut cadence = std::collections::HashMap::new();
    cadence.insert("1".to_string(), vec!["dsa".to_string()]);
    cadence.insert("2".to_string(), vec!["java".to_string()]);
    cadence.insert("3".to_string(), vec!["dsa".to_string()]);
    cadence.insert("4".to_string(), vec!["lld".to_string()]);
    cadence.insert("5".to_string(), vec!["dsa".to_string()]);
    cadence.insert("6".to_string(), vec!["fde".to_string()]);
    cadence.insert("7".to_string(), vec!["review".to_string()]);

    Settings {
      data_root: None,
      weekly_capacity_hours: Some(7.5),
      timezone: Some("Asia/Kolkata".to_string()),
      theme: Some("dark".to_string()),
      dsa_cursor_category: Some("Tries".to_string()),
      lld_cursor_topic: Some("Bridge Pattern".to_string()),
      study_days: Some(vec![1, 2, 3, 4, 5, 6]),
      cadence: Some(cadence),
      min_meaningful_minutes: Some(15),
      daily_buffer_ratio: Some(0.15),
    }
  }
}

pub fn get_default_settings() -> Settings {
  Settings::default()
}

/// Load persisted settings, falling back to defaults when absent or unreadable.
pub fn load_settings(conn: &rusqlite::Connection) -> Settings {
  let stored: Option<String> = conn
    .query_row("SELECT value FROM settings WHERE key = 'app'", [], |row| row.get(0))
    .ok();
  match stored.and_then(|raw| serde_json::from_str::<Settings>(&raw).ok()) {
    Some(settings) => settings,
    None => Settings::default(),
  }
}

pub fn store_settings(conn: &rusqlite::Connection, settings: &Settings) -> DbResult<()> {
  let json = serde_json::to_string(settings)
    .map_err(|e| DbError::Serialization(e.to_string()))?;
  conn.execute(
    "INSERT INTO settings (key, value, updated_at) VALUES ('app', ?1, ?2)
     ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
    rusqlite::params![json, now_rfc3339()],
  )?;
  Ok(())
}

pub fn now_rfc3339() -> String {
  Utc::now().to_rfc3339()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn settings_round_trip_persists_planner_fields() {
    let mut conn = rusqlite::Connection::open_in_memory().unwrap();
    crate::migrations::ensure_schema(&mut conn).unwrap();

    // Defaults are returned before anything is stored.
    let loaded = load_settings(&conn);
    assert_eq!(loaded.weekly_capacity_hours, Some(7.5));
    assert_eq!(loaded.study_days, Some(vec![1, 2, 3, 4, 5, 6]));

    let mut settings = Settings::default();
    settings.weekly_capacity_hours = Some(10.0);
    settings.study_days = Some(vec![1, 3, 5]);
    settings.min_meaningful_minutes = Some(20);

    store_settings(&conn, &settings).unwrap();
    let reloaded = load_settings(&conn);

    assert_eq!(reloaded.weekly_capacity_hours, Some(10.0));
    assert_eq!(reloaded.study_days, Some(vec![1, 3, 5]));
    assert_eq!(reloaded.min_meaningful_minutes, Some(20));
    // Cadence should survive the round trip too.
    assert!(reloaded.cadence.unwrap().contains_key("1"));
  }
}
