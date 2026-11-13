#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod migrations;
mod state;

use state::AppState;

fn main() {
  tauri::Builder::default()
    .manage(AppState::default())
    .invoke_handler(tauri::generate_handler![
      commands::health_check,
      commands::get_settings,
      commands::save_settings,
      commands::backup_database,
      commands::restore_database,
      commands::run_migrations,
    ])
    .setup(|_app| {
      let _ = db::init_data_dir()?;
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
