#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod curriculum;
mod db;
mod dsa;
mod learning;
mod migrations;
mod planner;
mod projects;
mod roadmap_import;
mod analytics;
mod state;
mod workspace;

use state::AppState;

fn main() {
  tauri::Builder::default()
    .manage(AppState::default())
    .invoke_handler(tauri::generate_handler![
      commands::health_check,
      commands::get_settings,
      commands::save_settings,
      commands::open_external_url,
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
      dsa::preview_neetcode,
      dsa::import_neetcode,
      dsa::get_problems,
      dsa::record_attempt,
      dsa::list_attempts,
      dsa::save_solution,
      dsa::get_solution,
      dsa::record_review,
      dsa::get_revision_queue,
      dsa::start_revision_baseline,
      dsa::get_dsa_progress,
      dsa::toggle_favorite,
      planner::get_agenda,
      planner::list_overdue_tasks,
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
      workspace::upsert_code_experiment,
      workspace::get_code_experiment,
      workspace::list_code_experiments,
      workspace::upsert_recall_question,
      workspace::record_recall_review,
      workspace::get_recall_question,
      workspace::list_recall_questions,
      workspace::upsert_diagram,
      workspace::get_diagram,
      workspace::list_diagrams,
      workspace::upsert_design_artifact,
      workspace::get_design_artifact,
      workspace::list_design_artifacts,
      workspace::upsert_case_study,
      workspace::get_case_study,
      workspace::list_case_studies,
      workspace::add_topic_link,
      workspace::list_topic_links,
      workspace::save_workspace_file,
      projects::list_projects,
      projects::get_project,
      projects::upsert_project,
      projects::upsert_project_milestone,
      projects::upsert_project_task,
      projects::upsert_project_link,
      projects::upsert_project_note,
      projects::get_project_detail,
      projects::seed_fde_project,
      analytics::search_all,
      analytics::export_all,
      analytics::get_analytics,
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
