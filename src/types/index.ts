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
  study_days?: number[]
  cadence?: Record<string, string[]>
  min_meaningful_minutes?: number
  daily_buffer_ratio?: number
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

// ---------------------------------------------------------------------------
// Phase 3: planner, learning entries and activity
// ---------------------------------------------------------------------------

export type TaskKind = 'planned' | 'revision' | 'review' | 'stretch' | 'custom'
export type TaskStatus = 'planned' | 'done' | 'skipped'

export interface DailyPlan {
  id: string
  plan_date: string
  capacity_minutes?: number
  available: boolean
  note?: string
}

export interface StudyTask {
  id: string
  daily_plan_id?: string
  kind: string
  title: string
  track: string
  topic_id?: string
  problem_id?: string
  revision_item_id?: string
  estimate_minutes: number
  actual_minutes?: number
  status: string
  output?: string
  completed_at?: string
  position: number
  scheduled_date: string
}

export interface AgendaResponse {
  ok: boolean
  plan: DailyPlan | null
  tasks: StudyTask[]
  due_revisions: number
  planned_minutes: number
  completed_minutes: number
  error?: string
}

export interface GenerateWeekResponse {
  ok: boolean
  tasks_created: number
  days_generated: number
  warnings: string[]
  error?: string
}

export interface UpdateTaskRequest {
  id: string
  status?: string
  actual_minutes?: number
  output?: string
  scheduled_date?: string
}

export interface UpdateTaskResponse {
  ok: boolean
  task: StudyTask | null
  error?: string
}

export interface CreateTaskRequest {
  scheduled_date: string
  title: string
  track: string
  kind?: string
  topic_id?: string
  estimate_minutes?: number
}

export interface CreateTaskResponse {
  ok: boolean
  id: string | null
  error?: string
}

export interface SetAvailabilityRequest {
  plan_date: string
  available: boolean
  capacity_minutes?: number
  note?: string
}

export interface ReplanResponse {
  ok: boolean
  moved: number
  unplaced: number
  error?: string
}

export interface WeeklyGoal {
  id: string
  week_start_date: string
  track_targets?: string
  note?: string
  reviewed: boolean
}

export interface TrackTally {
  planned: number
  completed: number
}

export interface WeekReviewResponse {
  ok: boolean
  week_start: string
  goal: WeeklyGoal | null
  planned: number
  completed: number
  skipped: number
  planned_minutes: number
  completed_minutes: number
  by_track: Record<string, TrackTally>
  tasks: StudyTask[]
  error?: string
}

export interface LearningEntry {
  id: string
  title: string
  entry_date: string
  track: string
  description: string
  notes?: string
  resource_url?: string
  duration_minutes?: number
  output_file?: string
  tags?: string
  related_topic_id?: string
  created_at: string
}

export interface CreateLearningEntryRequest {
  title: string
  entry_date: string
  track: string
  description: string
  notes?: string
  resource_url?: string
  duration_minutes?: number
  output_file?: string
  tags?: string
  related_topic_id?: string
}

export interface CreateLearningEntryResponse {
  ok: boolean
  id: string | null
  error?: string
}

export interface ListEntriesResponse {
  ok: boolean
  entries: LearningEntry[]
  error?: string
}

export interface ActivityDay {
  date: string
  minutes: number
  events: number
}

export interface ActivitySummaryResponse {
  ok: boolean
  days: ActivityDay[]
  current_streak: number
  longest_streak: number
  total_minutes: number
  active_days: number
  min_meaningful_minutes: number
  error?: string
}
