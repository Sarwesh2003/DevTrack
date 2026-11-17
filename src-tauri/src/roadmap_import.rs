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

const FIELD_LABELS: [&str; 8] = [
  "Priority",
  "Time estimate",
  "Topics",
  "Subtopics",
  "Learning resources",
  "Practical work",
  "Completion criteria",
  "Dependencies",
];

fn is_separator_line(trimmed: &str) -> bool {
  !trimmed.is_empty() && trimmed.chars().all(|c| c == '-' || c == ' ' || c == '=')
}

fn parse_label_fields<'a>(lines: &[&'a str]) -> std::collections::HashMap<String, String> {
  let mut fields: std::collections::HashMap<String, String> = std::collections::HashMap::new();
  let mut current: Option<String> = None;

  for line in lines {
    let trimmed = line.trim();
    if trimmed.is_empty() || is_separator_line(trimmed) {
      continue;
    }
    if trimmed.starts_with("Field") {
      current = None;
      continue;
    }

    let mut matched: Option<(String, String)> = None;
    for label in FIELD_LABELS.iter() {
      if let Some(rest) = trimmed.strip_prefix(label) {
        if rest.is_empty() || rest.starts_with(char::is_whitespace) {
          matched = Some((label.to_string(), rest.trim().to_string()));
          break;
        }
      }
    }

    match matched {
      Some((label, value)) => {
        current = Some(label.clone());
        fields.insert(label, value);
      }
      None => {
        if let Some(ref label) = current {
          if let Some(entry) = fields.get_mut(label) {
            if !entry.is_empty() {
              entry.push(' ');
            }
            entry.push_str(trimmed);
          }
        }
      }
    }
  }

  fields
}

fn split_semicolon_list(value: &str) -> Vec<String> {
  value
    .split(';')
    .map(|s| s.trim().replace('`', "").trim().to_string())
    .filter(|s| !s.is_empty())
    .collect()
}

fn extract_markdown_links(value: &str) -> Vec<(String, String)> {
  let chars: Vec<char> = value.chars().collect();
  let mut links = Vec::new();
  let mut i = 0;
  while i < chars.len() {
    if chars[i] == '[' {
      if let Some(close_bracket) = chars[i + 1..].iter().position(|&c| c == ']') {
        let title_end = i + 1 + close_bracket;
        let title: String = chars[i + 1..title_end].iter().collect();
        if title_end + 1 < chars.len() && chars[title_end + 1] == '(' {
          if let Some(close_paren) = chars[title_end + 2..].iter().position(|&c| c == ')') {
            let url_end = title_end + 2 + close_paren;
            let url: String = chars[title_end + 2..url_end].iter().collect();
            links.push((title.trim().to_string(), url.trim().to_string()));
            i = url_end + 1;
            continue;
          }
        }
      }
    }
    i += 1;
  }
  links
}

fn parse_estimate_minutes(value: &str) -> Option<i32> {
  let first_num: String = value
    .chars()
    .skip_while(|c| !c.is_ascii_digit())
    .take_while(|c| c.is_ascii_digit())
    .collect();
  let num: i32 = first_num.parse().ok()?;
  if value.to_lowercase().contains("min") {
    Some(num)
  } else {
    Some(num * 60)
  }
}

fn parse_dependencies(value: &str) -> Vec<ParsedDependency> {
  let trimmed = value.trim();
  if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("none") {
    return vec![];
  }
  trimmed
    .split(|c| c == ',' || c == ';')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty() && !s.eq_ignore_ascii_case("none"))
    .map(|s| ParsedDependency {
      depends_on_topic_key: s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
        .to_uppercase(),
      r#type: "prerequisite".to_string(),
    })
    .collect()
}

fn parse_track_header(line: &str) -> Option<(String, String)> {
  let trimmed = line.trim();
  let without = trimmed.strip_prefix("# ")?;
  let parts: Vec<&str> = without.splitn(2, " --- ").collect();
  if parts.len() != 2 {
    return None;
  }
  let left = parts[0].trim();
  let name = parts[1].trim().to_string();
  let idx = left.find("Track ")?;
  let key_part = left[idx + "Track ".len()..].trim();
  let mut chars = key_part.chars();
  let first = chars.next()?;
  if !first.is_ascii_uppercase() || chars.next().is_some() {
    return None;
  }
  Some((first.to_ascii_lowercase().to_string(), name))
}

fn parse_topic_header(line: &str) -> Option<(String, String)> {
  let trimmed = line.trim();
  let rest = trimmed.strip_prefix("### ")?;
  let dot = rest.find(". ")?;
  let key_part = rest[..dot].trim();
  let title = rest[dot + 2..].trim().to_string();
  let mut chars = key_part.chars();
  let first = chars.next()?;
  if !first.is_ascii_uppercase() {
    return None;
  }
  if key_part.len() < 2 || !chars.all(|c| c.is_ascii_digit()) {
    return None;
  }
  Some((key_part.to_string(), title))
}

fn parse_roadmap(content: &str) -> Result<(Vec<ParsedTrack>, Vec<ParsedModule>, Vec<ParsedTopic>), String> {
  let lines: Vec<&str> = content.lines().collect();

  let mut track_headers: Vec<(usize, String, String)> = Vec::new();
  for (i, line) in lines.iter().enumerate() {
    if let Some((key, name)) = parse_track_header(line) {
      track_headers.push((i, key, name));
    }
  }

  if track_headers.is_empty() {
    return Err(
      "No track headers found. Expected headings like '# 3. Track A --- Java Interview Preparation'."
        .to_string(),
    );
  }

  let mut tracks = Vec::new();
  let mut modules = Vec::new();
  let mut topics = Vec::new();
  let mut track_position = 0;

  for (ti, (start, track_key, track_name)) in track_headers.iter().enumerate() {
    let end = track_headers.get(ti + 1).map(|h| h.0).unwrap_or(lines.len());
    let section = &lines[start + 1..end];

    track_position += 1;
    tracks.push(ParsedTrack {
      key: track_key.clone(),
      name: track_name.clone(),
      position: track_position,
    });

    // DSA (track b) problems are imported from the NeetCode 250 dataset, not this document.
    if track_key.eq_ignore_ascii_case("b") {
      continue;
    }

    let mut module_position = 0;
    let mut current_module: Option<String> = None;

    let mut idx = 0;
    while idx < section.len() {
      if !section[idx].trim().starts_with("### ") {
        idx += 1;
        continue;
      }

      let mut next = idx + 1;
      while next < section.len() && !section[next].trim().starts_with("### ") {
        next += 1;
      }
      let block = &section[idx + 1..next];

      if let Some((topic_key, topic_title)) = parse_topic_header(section[idx]) {
        let fields = parse_label_fields(block);
        let module_key: String = topic_key
          .chars()
          .take_while(|c| c.is_ascii_alphabetic())
          .collect();

        if current_module.as_ref() != Some(&module_key) {
          module_position += 1;
          current_module = Some(module_key.clone());
          modules.push(ParsedModule {
            track_key: track_key.clone(),
            key: module_key.clone(),
            name: format!("Module {module_key}"),
            position: module_position,
          });
        }

        let priority = fields
          .get("Priority")
          .cloned()
          .unwrap_or_else(|| "Medium".to_string());
        let estimate_minutes = fields.get("Time estimate").and_then(|s| parse_estimate_minutes(s));
        let completion_criteria = fields.get("Completion criteria").cloned();

        let resources: Vec<ParsedResource> = fields
          .get("Learning resources")
          .map(|v| extract_markdown_links(v))
          .unwrap_or_default()
          .into_iter()
          .enumerate()
          .map(|(i, (title, url))| ParsedResource {
            title,
            url: Some(url),
            r#type: Some("link".to_string()),
            position: i as i32,
          })
          .collect();

        let subtopics: Vec<ParsedSubtopic> = fields
          .get("Subtopics")
          .map(|v| split_semicolon_list(v))
          .unwrap_or_default()
          .into_iter()
          .enumerate()
          .map(|(i, title)| ParsedSubtopic {
            key: format!("sub{i}"),
            title,
            position: i as i32,
          })
          .collect();

        let dependencies = fields
          .get("Dependencies")
          .map(|v| parse_dependencies(v))
          .unwrap_or_default();

        topics.push(ParsedTopic {
          module_key: module_key.clone(),
          key: topic_key.to_uppercase(),
          title: topic_title,
          status: "Not started".to_string(),
          priority,
          estimate_minutes,
          confidence: None,
          completion_criteria,
          notes: None,
          target_week: None,
          is_custom: false,
          source_ref: Some(format!("roadmap:{track_key}:{}", topic_key.to_uppercase())),
          subtopics,
          resources,
          dependencies,
        });
      }

      idx = next;
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

  let inserted = conn.execute(
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

  if inserted == 0 {
    // Topic already exists (re-import): keep existing rows and user progress untouched.
    let existing_id: String = conn.query_row(
      "SELECT id FROM topics WHERE key = ?1",
      params![topic.key],
      |row| row.get(0),
    )?;
    return Ok(existing_id);
  }

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

  if tracks.iter().any(|t| t.key.eq_ignore_ascii_case("b")) {
    warnings.push(
      "DSA (track B) problems are imported from the NeetCode 250 dataset in a later phase; the roadmap contributes guidance only, so track B has no problem topics yet."
        .to_string(),
    );
  }

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

#[cfg(test)]
mod tests {
  use super::*;

  const ROADMAP: &str = include_str!("../../docs/roadmap/six_month_engineering_roadmap.md");

  #[test]
  fn parses_track_headers() {
    assert_eq!(
      parse_track_header("# 3. Track A --- Java Interview Preparation (Senior SDE)"),
      Some(("a".to_string(), "Java Interview Preparation (Senior SDE)".to_string()))
    );
    assert_eq!(parse_track_header("# 8. Cross-track dependency map"), None);
  }

  #[test]
  fn parses_topic_headers() {
    assert_eq!(
      parse_topic_header("### A1. Language, object model and type system"),
      Some(("A1".to_string(), "Language, object model and type system".to_string()))
    );
    assert_eq!(parse_topic_header("### Java track completion standard"), None);
    assert_eq!(parse_topic_header("### Completion states"), None);
  }

  #[test]
  fn parses_real_roadmap() {
    let (tracks, modules, topics) = parse_roadmap(ROADMAP).expect("roadmap should parse");

    assert_eq!(tracks.len(), 5, "expected 5 tracks");
    let keys: Vec<&str> = tracks.iter().map(|t| t.key.as_str()).collect();
    assert_eq!(keys, vec!["a", "b", "c", "d", "e"]);

    // Track B is guidance only; real DSA problems come from NeetCode.
    assert_eq!(topics.len(), 37, "A(10) + C(7) + D(10) + E(10)");
    assert!(topics.iter().all(|t| !t.key.starts_with('B')));

    let a1 = topics.iter().find(|t| t.key == "A1").expect("A1 present");
    assert_eq!(a1.title, "Language, object model and type system");
    assert_eq!(a1.priority, "High");
    assert!(!a1.subtopics.is_empty(), "A1 should have subtopics");
    assert!(!a1.resources.is_empty(), "A1 should have resources");

    let c1 = topics.iter().find(|t| t.key == "C1").expect("C1 present");
    assert!(c1.module_key == "C");
  }
}
