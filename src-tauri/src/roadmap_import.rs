use crate::db::{now_rfc3339, open_connection, DbResult};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ImportRoadmapRequest {
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportRoadmapResponse {
  pub ok: bool,
  pub track_count: i32,
  pub topic_count: i32,
  pub week_count: i32,
  pub warnings: Vec<String>,
  pub error: Option<String>,
}

#[derive(Debug)]
struct ParsedTrack {
  key: String,
  name: String,
  position: i32,
}

#[derive(Debug)]
struct ParsedModule {
  track_key: String,
  key: String,
  name: String,
  position: i32,
}

#[derive(Debug)]
struct ParsedTopic {
  module_key: String,
  key: String,
  title: String,
  status: String,
  priority: String,
  estimate_minutes: Option<i32>,
  confidence: Option<i32>,
  completion_criteria: Option<String>,
  notes: Option<String>,
  target_week: Option<i32>,
  is_custom: bool,
  source_ref: Option<String>,
  subtopics: Vec<ParsedSubtopic>,
  resources: Vec<ParsedResource>,
  dependencies: Vec<ParsedDependency>,
}

#[derive(Debug)]
struct ParsedSubtopic {
  key: String,
  title: String,
  position: i32,
}

#[derive(Debug)]
struct ParsedResource {
  title: String,
  url: Option<String>,
  r#type: Option<String>,
  position: i32,
}

#[derive(Debug)]
struct ParsedDependency {
  depends_on_topic_key: String,
  r#type: String,
}

fn parse_field(lines: &[&str], field_name: &str) -> Option<String> {
  for line in lines {
    let trimmed = line.trim();
    if trimmed.starts_with("| ") && trimmed.contains(field_name) {
      // Table row: | Field | Value |
      let parts: Vec<&str> = trimmed.split('|').map(|s| s.trim()).collect();
      if let Some(idx) = parts.iter().position(|p| *p == field_name) {
        if idx + 1 < parts.len() {
          return Some(parts[idx + 1].to_string());
        }
      }
    }
  }
  None
}

fn parse_list_items(lines: &[&str], list_header: &str) -> Vec<String> {
  let mut items = Vec::new();
  let mut in_list = false;
  for line in lines {
    let trimmed = line.trim();
    if trimmed == list_header || trimmed.starts_with(&format!("| {list_header} |")) {
      in_list = true;
      continue;
    }
    if in_list {
      if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
        items.push(trimmed[2..].trim().to_string());
      } else if trimmed.starts_with("|") || trimmed.is_empty() || trimmed.starts_with("#") {
        // End of list or new section
        if !trimmed.starts_with("- ") && !trimmed.starts_with("* ") {
          in_list = false;
        }
      }
    }
  }
  items
}

fn parse_subtopics(lines: &[&str]) -> Vec<ParsedSubtopic> {
  let mut subtopics = Vec::new();
  let mut in_subtopics = false;
  let mut position = 0;

  for line in lines {
    let trimmed = line.trim();
    if trimmed == "Subtopics" || trimmed == "`Subtopics`" {
      in_subtopics = true;
      continue;
    }
    if in_subtopics {
      if trimmed.starts_with("`") && trimmed.contains("`") {
        // Extract code-formatted subtopic like `==` vs `equals`
        let backtick_content: Vec<&str> = trimmed.split('`').filter(|s| !s.is_empty()).collect();
        if !backtick_content.is_empty() {
          position += 1;
          subtopics.push(ParsedSubtopic {
            key: backtick_content[0].replace(' ', "_").to_lowercase(),
            title: trimmed.trim_matches('`').trim().to_string(),
            position,
          });
        }
      } else if trimmed.starts_with("- ") {
        position += 1;
        subtopics.push(ParsedSubtopic {
          key: trimmed[2..].trim().replace(' ', "_").to_lowercase(),
          title: trimmed[2..].trim().to_string(),
          position,
        });
      } else if trimmed.starts_with("|") || trimmed.starts_with("#") || trimmed.is_empty() {
        if !trimmed.starts_with("- ") && !trimmed.starts_with("`") {
          in_subtopics = false;
        }
      }
    }
  }
  subtopics
}

fn parse_roadmap(content: &str) -> Result<(Vec<ParsedTrack>, Vec<ParsedModule>, Vec<ParsedTopic>), String> {
  let mut tracks = Vec::new();
  let mut modules = Vec::new();
  let mut topics = Vec::new();
  let mut track_position = 0;

  // Split by track headers
  let track_sections: Vec<&str> = content.split("# Track ").collect();
  
  for (ti, section) in track_sections.iter().enumerate() {
    if ti == 0 { continue; } // Skip content before first track

    let lines: Vec<&str> = section.lines().collect();
    if lines.is_empty() { continue; }

    // First line is the track header: "A --- Java Interview Preparation (Senior SDE)"
    let header_line = lines[0].trim();
    let track_parts: Vec<&str> = header_line.splitn(2, " --- ").collect();
    if track_parts.len() < 2 {
      continue;
    }
    let track_key = track_parts[0].trim().to_lowercase();
    let track_name = track_parts[1].trim().to_string();
    track_position += 1;

    tracks.push(ParsedTrack {
      key: track_key.clone(),
      name: track_name,
      position: track_position,
    });

    // Parse modules and topics within this track
    let mut module_position = 0;
    let mut current_module_key: Option<String> = None;

    // Split by topic headers (### A1. Name)
    let topic_sections: Vec<&str> = section.split("### ").collect();
    
    for (si, topic_section) in topic_sections.iter().enumerate() {
      if si == 0 { continue; }
      
      let topic_lines: Vec<&str> = topic_section.lines().collect();
      if topic_lines.is_empty() { continue; }

      // First line is the topic header: "A1. Language, object model and type system"
      let topic_header = topic_lines[0].trim();
      let topic_parts: Vec<&str> = topic_header.splitn(2, ". ").collect();
      if topic_parts.len() < 2 {
        continue;
      }
      let topic_key_part = topic_parts[0].trim();
      let topic_title = topic_parts[1].trim().to_string();
      
      // Determine module key (e.g., "A" from "A1")
      let module_key = topic_key_part.chars().take_while(|c| c.is_alphabetic()).collect::<String>();
      
      // Create module if new
      if current_module_key.as_ref() != Some(&module_key) {
        module_position += 1;
        current_module_key = Some(module_key.clone());
        modules.push(ParsedModule {
          track_key: track_key.clone(),
          key: module_key.clone(),
          name: format!("Module {}", module_key),
          position: module_position,
        });
      }

      let topic_key = topic_key_part.to_uppercase();
      
      // Parse fields from the topic section
      let status = "Not started".to_string();
      let priority = parse_field(&topic_lines, "Priority")
        .unwrap_or_else(|| "Medium".to_string());
      
      let estimate_str = parse_field(&topic_lines, "Time estimate");
      let estimate_minutes = estimate_str.and_then(|s| {
        // Extract number from "3 hours" or "5--7 h"
        let num_str: String = s.chars().filter(|c| c.is_ascii_digit() || *c == '.').collect();
        num_str.parse::<f64>().ok().map(|f| (f * 60.0) as i32)
      });

      let completion_criteria = parse_field(&topic_lines, "Completion criteria");
      let dependencies_str = parse_field(&topic_lines, "Dependencies");
      
      let dependencies: Vec<ParsedDependency> = if let Some(deps) = &dependencies_str {
        if deps == "None" {
          vec![]
        } else {
          deps.split(',').map(|d| {
            let dep_key = d.trim().to_uppercase();
            ParsedDependency {
              depends_on_topic_key: dep_key,
              r#type: "prerequisite".to_string(),
            }
          }).collect()
        }
      } else {
        vec![]
      };

      let subtopics = parse_subtopics(&topic_lines);
      let resources_str = parse_field(&topic_lines, "Learning resources");
      
      let resources: Vec<ParsedResource> = if let Some(res) = &resources_str {
        res.lines().enumerate().filter_map(|(i, line)| {
          let trimmed = line.trim();
          if trimmed.starts_with("[") && trimmed.contains("](") {
            // Markdown link: [Title](URL)
            let start = trimmed.find('[').unwrap_or(0) + 1;
            let end = trimmed.find(']').unwrap_or(start);
            let title = trimmed[start..end].to_string();
            let url_start = trimmed.find('(').unwrap_or(end) + 1;
            let url_end = trimmed.find(')').unwrap_or(url_start);
            let url = trimmed[url_start..url_end].to_string();
            Some(ParsedResource {
              title,
              url: Some(url),
              r#type: Some("link".to_string()),
              position: i as i32,
            })
          } else {
            None
          }
        }).collect()
      } else {
        vec![]
      };

      topics.push(ParsedTopic {
        module_key: module_key.clone(),
        key: topic_key.clone(),
        title: topic_title,
        status,
        priority,
        estimate_minutes,
        confidence: None,
        completion_criteria,
        notes: None,
        target_week: None,
        is_custom: false,
        source_ref: Some(format!("roadmap:{track_key}:{topic_key}")),
        subtopics,
        resources,
        dependencies,
      });
    }
  }

  Ok((tracks, modules, topics))
}

fn insert_track(conn: &rusqlite::Connection, track: &ParsedTrack) -> DbResult<()> {
  let id = Uuid::new_v4().to_string();
  let now = now_rfc3339();
  conn.execute(
    "INSERT OR IGNORE INTO tracks (id, key, name, description, position, created_at) VALUES (?1,?2,?3,?4,?5,?6)",
    params![id, track.key, track.name, None::<String>, track.position, now],
  )?;
  Ok(())
}

fn insert_module(conn: &rusqlite::Connection, module: &ParsedModule) -> DbResult<()> {
  // Get track id
  let track_id: String = conn.query_row(
    "SELECT id FROM tracks WHERE key = ?1",
    params![module.track_key],
    |row| row.get(0),
  )?;
  
  let id = Uuid::new_v4().to_string();
  let now = now_rfc3339();
  conn.execute(
    "INSERT OR IGNORE INTO modules (id, track_id, key, name, position) VALUES (?1,?2,?3,?4,?5)",
    params![id, track_id, module.key, module.name, module.position],
  )?;
  let _ = now;
  Ok(())
}

fn insert_topic(conn: &rusqlite::Connection, topic: &ParsedTopic) -> DbResult<String> {
  // Get module id
  let track_key: String = conn.query_row(
    "SELECT t.key FROM modules m JOIN tracks t ON m.track_id = t.id WHERE m.key = ?1",
    params![topic.module_key],
    |row| row.get::<_, String>(0),
  )?;
  
  let module_id: String = conn.query_row(
    "SELECT id FROM modules WHERE key = ?1 AND track_id = (SELECT id FROM tracks WHERE key = ?2)",
    params![topic.module_key, track_key],
    |row| row.get(0),
  )?;

  let id = Uuid::new_v4().to_string();
  let now = now_rfc3339();
  
  // Determine if previously covered based on progress overrides
  let (is_previously_covered, status) = if topic.key.starts_with('A') || topic.key.starts_with('D') || topic.key.starts_with('E') {
    // Java, HLD, FDE start fresh
    (0, "Not started".to_string())
  } else if topic.key.starts_with('B') {
    // DSA: categories through Trees (B1-B7) are previously covered
    let topic_num: Option<i32> = topic.key.trim_start_matches('B').parse().ok();
    if let Some(n) = topic_num {
      if n <= 7 {
        // Arrays & Hashing through Trees - previously covered, revision only
        (1, "Needs revision".to_string())
      } else {
        // Tries and beyond - new learning
        (0, "Not started".to_string())
      }
    } else {
      (0, "Not started".to_string())
    }
  } else if topic.key.starts_with('C') {
    // LLD: C1-C4 previously covered, C5+ new learning
    let topic_num: Option<i32> = topic.key.trim_start_matches('C').parse().ok();
    if let Some(n) = topic_num {
      if n <= 4 {
        (1, "Needs revision".to_string())
      } else {
        (0, "Not started".to_string())
      }
    } else {
      (0, "Not started".to_string())
    }
  } else {
    (0, "Not started".to_string())
  };

  let _ = is_previously_covered; // Stored in problems table for DSA, topics use status

  conn.execute(
    "INSERT OR IGNORE INTO topics (id, module_id, key, title, status, priority, estimate_minutes, confidence, completion_criteria, notes, target_week, is_custom, source_ref, created_at, updated_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)",
    params![
      id, module_id, topic.key, topic.title, status,
      topic.priority,
      topic.estimate_minutes, topic.confidence,
      topic.completion_criteria, topic.notes,
      topic.target_week, 0, topic.source_ref,
      now, now,
    ],
  )?;

  // Insert subtopics
  for (i, subtopic) in topic.subtopics.iter().enumerate() {
    let sub_id = Uuid::new_v4().to_string();
    let _ = conn.execute(
      "INSERT OR IGNORE INTO subtopics (id, topic_id, key, title, is_complete, position) VALUES (?1,?2,?3,?4,?5,?6)",
      params![sub_id, id, subtopic.key, subtopic.title, 0, i as i32],
    );
  }

  // Insert resources
  for (i, resource) in topic.resources.iter().enumerate() {
    let res_id = Uuid::new_v4().to_string();
    let _ = conn.execute(
      "INSERT OR IGNORE INTO resources (id, topic_id, title, url, type, position) VALUES (?1,?2,?3,?4,?5,?6)",
      params![res_id, id, resource.title, resource.url, resource.r#type, i as i32],
    );
  }

  // Insert dependencies
  for dep in topic.dependencies.iter() {
    let dep_id = Uuid::new_v4().to_string();
    // Find the depends_on topic id
    let depends_on_id: Option<String> = conn.query_row(
      "SELECT id FROM topics WHERE key = ?1 AND module_id = ?2",
      params![dep.depends_on_topic_key, module_id],
      |row| row.get(0),
    ).ok();
    
    if let Some(dep_id_val) = depends_on_id {
      let _ = conn.execute(
        "INSERT OR IGNORE INTO topic_dependencies (id, topic_id, depends_on_topic_id, type) VALUES (?1,?2,?3,?4)",
        params![dep_id, id, dep_id_val, dep.r#type],
      );
    }
  }

  Ok(id)
}

fn insert_roadmap_week(conn: &rusqlite::Connection, phase_id: &str, week_number: i32) -> DbResult<String> {
  let id = Uuid::new_v4().to_string();
  let _ = conn.execute(
    "INSERT OR IGNORE INTO roadmap_weeks (id, phase_id, week_number, track_allocations, total_estimate_minutes, source_ref) VALUES (?1,?2,?3,?4,?5,?6)",
    params![id, phase_id, week_number, None::<String>, None::<i32>, "roadmap:weekly_sequence"],
  )?;
  Ok(id)
}

fn insert_roadmap_phase(conn: &rusqlite::Connection, key: &str, name: &str, position: i32) -> DbResult<String> {
  let id = Uuid::new_v4().to_string();
  let _now = now_rfc3339();
  conn.execute(
    "INSERT OR IGNORE INTO roadmap_phases (id, key, name, position, source_ref) VALUES (?1,?2,?3,?4,?5)",
    params![id, key, name, position, "roadmap:phases"],
  )?;
  Ok(id)
}

#[tauri::command]
pub async fn import_roadmap(req: ImportRoadmapRequest) -> ImportRoadmapResponse {
  let content = &req.content;
  
  let (tracks, modules, topics) = match parse_roadmap(content) {
    Ok(data) => data,
    Err(e) => return ImportRoadmapResponse {
      ok: false,
      track_count: 0,
      topic_count: 0,
      week_count: 0,
      warnings: vec![],
      error: Some(format!("Failed to parse roadmap: {e}")),
    },
  };

  let mut conn = match open_connection() {
    Ok(c) => c,
    Err(e) => return ImportRoadmapResponse {
      ok: false,
      track_count: 0,
      topic_count: 0,
      week_count: 0,
      warnings: vec![],
      error: Some(format!("Failed to open database: {e}")),
    },
  };

  let mut warnings = Vec::new();

  // Run migrations first
  if let Err(e) = crate::migrations::ensure_schema(&mut conn) {
    warnings.push(format!("Migration warning: {e}"));
  }

  // Insert roadmap phases (6 phases)
  let phase_names = [
    ("phase1", "Phase 1 - Foundations and baseline", 1),
    ("phase2", "Phase 2 - Core interview depth", 2),
    ("phase3", "Phase 3 - Applied engineering", 3),
    ("phase4", "Phase 4 - Production readiness", 4),
    ("phase5", "Phase 5 - Interview execution", 5),
    ("phase6", "Phase 6 - Consolidation", 6),
  ];

  let mut phase_ids = std::collections::HashMap::new();
  for (key, name, position) in phase_names.iter() {
    match insert_roadmap_phase(&conn, key, name, *position) {
      Ok(id) => { phase_ids.insert(key.to_string(), id); },
      Err(e) => warnings.push(format!("Phase insert warning: {e}")),
    }
  }

  // Insert tracks
  for track in &tracks {
    if let Err(e) = insert_track(&conn, track) {
      warnings.push(format!("Track insert warning ({}): {e}", track.key));
    }
  }

  // Insert modules
  for module in &modules {
    if let Err(e) = insert_module(&conn, module) {
      warnings.push(format!("Module insert warning ({}): {e}", module.key));
    }
  }

  // Insert topics
  let mut topic_count = 0;
  for topic in &topics {
    match insert_topic(&conn, topic) {
      Ok(_) => topic_count += 1,
      Err(e) => warnings.push(format!("Topic insert warning ({}): {e}", topic.key)),
    }
  }

  // Insert roadmap weeks (26 weeks)
  let mut week_count = 0;
  for week in 1..=26 {
    let phase_idx = ((week - 1) / 4).min(5);
    let phase_key = phase_names[phase_idx].0;
    if let Some(phase_id) = phase_ids.get(phase_key) {
      match insert_roadmap_week(&conn, phase_id, week as i32) {
        Ok(_) => week_count += 1,
        Err(e) => warnings.push(format!("Week insert warning ({week}): {e}")),
      }
    }
  }

  ImportRoadmapResponse {
    ok: true,
    track_count: tracks.len() as i32,
    topic_count: topic_count as i32,
    week_count: week_count as i32,
    warnings,
    error: None,
  }
}
