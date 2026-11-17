export interface DbHealth {
  connected: boolean
  path: string
  version: number
}

export interface Settings {
  data_root?: string
  weekly_capacity_hours?: number
  timezone?: string
  theme?: string
  dsa_cursor_category?: string
  lld_cursor_topic?: string
}

export interface HealthResponse {
  ok: boolean
  health: DbHealth
}

export interface SettingsResponse {
  ok: boolean
  settings: Settings
}

export interface SaveSettingsResponse {
  ok: boolean
}

export interface BackupResponse {
  ok: boolean
  path: string
  error?: string
}

export interface RestoreResponse {
  ok: boolean
  error?: string
}

export interface MigrationsResponse {
  ok: boolean
  version: number
  error?: string
}

export interface BackupRequest {
  destination: string
}

export interface RestoreRequest {
  source: string
}

export interface Track {
  id: string
  key: string
  name: string
  description?: string
  position: number
  created_at: string
}

export interface Module {
  id: string
  track_id: string
  key: string
  name: string
  position: number
}

export interface Topic {
  id: string
  module_id: string
  key: string
  title: string
  status: string
  priority: string
  estimate_minutes?: number
  confidence?: number
  completion_criteria?: string
  notes?: string
  evidence?: string
  target_week?: number
  is_custom: boolean
  source_ref?: string
  created_at: string
  updated_at: string
}

export interface Subtopic {
  id: string
  topic_id: string
  key: string
  title: string
  is_complete: boolean
  position: number
}

export interface Resource {
  id: string
  topic_id?: string
  title: string
  url?: string
  type?: string
  position: number
}

export interface TopicDependency {
  id: string
  topic_id: string
  depends_on_topic_id: string
  type: string
}

export interface TrackListResponse {
  ok: boolean
  tracks: Track[]
}

export interface ModuleListResponse {
  ok: boolean
  modules: Module[]
}

export interface TopicListResponse {
  ok: boolean
  topics: Topic[]
}

export interface TopicDetailResponse {
  ok: boolean
  topic: Topic | null
  subtopics: Subtopic[]
  resources: Resource[]
  dependencies: TopicDependency[]
}

export interface UpsertResponse {
  ok: boolean
  id: string
}

export interface UpsertTopicRequest {
  module_id: string
  key: string
  title: string
  status?: string
  priority?: string
  estimate_minutes?: number
  confidence?: number
  completion_criteria?: string
  notes?: string
  target_week?: number
  is_custom?: boolean
  source_ref?: string
}

export interface UpsertSubtopicRequest {
  topic_id: string
  key: string
  title: string
  is_complete?: boolean
  position?: number
}

export interface UpsertResourceRequest {
  topic_id?: string
  title: string
  url?: string
  type?: string
  position?: number
}

export interface UpsertDependencyRequest {
  topic_id: string
  depends_on_topic_id: string
  type: string
}

export interface RoadmapPhase {
  id: string
  key: string
  name: string
  position: number
  source_ref?: string
}

export interface RoadmapWeek {
  id: string
  phase_id?: string
  week_number: number
  track_allocations?: string
  total_estimate_minutes?: number
  source_ref?: string
}

export interface WeeklyPlanItem {
  id: string
  week_id: string
  topic_id?: string
  problem_id?: string
  task_type: string
  title: string
  estimate_minutes: number
  position: number
  day_of_week?: number
  source_ref?: string
}

export interface RoadmapPhasesResponse {
  ok: boolean
  phases: RoadmapPhase[]
}

export interface RoadmapWeeksResponse {
  ok: boolean
  weeks: RoadmapWeek[]
}

export interface WeeklyPlanItemsResponse {
  ok: boolean
  items: WeeklyPlanItem[]
}

export interface ImportRoadmapRequest {
  content: string
}

export interface ImportRoadmapResponse {
  ok: boolean
  track_count: number
  topic_count: number
  week_count: number
  warnings: string[]
  error?: string
}
