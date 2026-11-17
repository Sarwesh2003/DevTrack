#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod curriculum;
mod db;
mod learning;
mod migrations;
mod planner;
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
      planner::get_agenda,
      planner::generate_week,
      planner::set_day_availability,
      planner::update_study_task,
      planner::create_study_task,
      planner::replan_backlog,
      planner::save_weekly_goal,
      planner::get_week_review,
      learning::create_learning_entry,
      learning::list_learning_entries,
      learning::get_activity_summary,
    ])
    .setup(|_app| {
      let _ = db::init_data_dir()?;
      if let Ok(mut conn) = db::open_connection() {
        let _ = migrations::ensure_schema(&mut conn);
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
