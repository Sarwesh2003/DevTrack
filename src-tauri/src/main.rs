#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod curriculum;
mod db;
mod migrations;
mod roadmap_import;
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
      commands::get_roadmap_phases,
      commands::get_roadmap_weeks,
      commands::get_weekly_plan_items,
      curriculum::get_tracks,
      curriculum::get_modules,
      curriculum::get_topics,
      curriculum::get_topic_detail,
      curriculum::upsert_topic,
      curriculum::upsert_subtopic,
      curriculum::upsert_resource,
      curriculum::upsert_dependency,
      roadmap_import::import_roadmap,
    ])
    .setup(|_app| {
      let _ = db::init_data_dir()?;
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
