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

export interface ResetResponse {
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

export interface OverdueTasksResponse {
  ok: boolean
  tasks: StudyTask[]
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

// ---------------------------------------------------------------------------
// Phase 4: DSA / NeetCode 250 workspace, attempts and revision engine
// ---------------------------------------------------------------------------

export interface NeetcodeRow {
  official_order: number
  problem_id: string
  title: string
  url: string
  category: string
  difficulty: string
  category_order: number
  source_name: string
  source_version: string
}

export interface ImportNeetcodeRequest {
  content: string
}

export interface ImportNeetcodeResponse {
  ok: boolean
  imported: number
  skipped: number
  previously_covered: number
  new_learning: number
  warnings: string[]
  error?: string
}

export interface PreviewNeetcodeResponse {
  ok: boolean
  total_rows: number
  valid_rows: number
  previously_covered: number
  new_learning: number
  by_category: CategoryCount[]
  sample: NeetcodeRow[]
  error?: string
}

export interface CategoryCount {
  category: string
  count: number
  previously_covered: boolean
}

export interface Problem {
  id: string
  external_id: string
  title: string
  category: string
  difficulty: string
  url: string
  official_order: number
  is_previously_covered: boolean
  tags: string[]
  is_favorite: boolean
  attempt_count: number
  solved: boolean
}

export interface GetProblemsRequest {
  category?: string
  difficulty?: string
  search?: string
  cover_filter?: 'all' | 'new' | 'revision'
}

export interface ProblemListResponse {
  ok: boolean
  problems: Problem[]
  total: number
  error?: string
}

export interface GetProblemByIdRequest {
  problem_id: string
}

export interface ProblemResponse {
  ok: boolean
  problem: Problem | null
  error?: string
}

export interface Attempt {
  id: string
  problem_id: string
  attempt_date: string
  outcome: string
  duration_minutes?: number
  notes?: string
  code_file?: string
  complexity_note?: string
  edge_cases?: string
  interview_explanation?: string
  created_at: string
}

export interface RecordAttemptRequest {
  problem_id: string
  attempt_date: string
  outcome: string
  duration_minutes?: number
  notes?: string
  code_file?: string
  complexity_note?: string
  edge_cases?: string
  interview_explanation?: string
}

export interface RecordAttemptResponse {
  ok: boolean
  id: string | null
  error?: string
}

export interface ListAttemptsResponse {
  ok: boolean
  attempts: Attempt[]
  error?: string
}

export interface Solution {
  id: string
  problem_id: string
  file_path: string
  content?: string
  language: string
  updated_at: string
}

export interface SaveSolutionRequest {
  problem_id: string
  content: string
}

export interface SaveSolutionResponse {
  ok: boolean
  file_path: string | null
  error?: string
}

export interface GetSolutionResponse {
  ok: boolean
  solution: Solution | null
  error?: string
}

export interface RevisionItemView {
  id: string
  source_type: string
  source_id: string
  initial_completion_date?: string
  last_reviewed_date?: string
  next_due_date: string
  current_interval: number
  repetition: number
  state: string
  recall_result?: string
  confidence?: number
  notes?: string
}

export interface RecordReviewRequest {
  revision_item_id: string
  reviewed_at: string
  recall_result: string
  confidence?: number
  notes?: string
  evidence?: string
}

export interface RecordReviewResponse {
  ok: boolean
  revision_item: RevisionItemView | null
  error?: string
}

export interface GetRevisionQueueResponse {
  ok: boolean
  items: RevisionItemView[]
  error?: string
}

export interface StartRevisionBaselineResponse {
  ok: boolean
  created: number
  error?: string
}

export interface CategoryProgress {
  category: string
  total: number
  attempted: number
  solved: number
  previously_covered: boolean
}

export interface GetDsaProgressResponse {
  ok: boolean
  categories: CategoryProgress[]
  total_problems: number
  total_attempted: number
  total_solved: number
  error?: string
}

export interface ToggleFavoriteRequest {
  problem_id: string
  favorite: boolean
}

export interface ToggleFavoriteResponse {
  ok: boolean
  is_favorite: boolean
  error?: string
}

export interface OpenUrlResponse {
  ok: boolean
  error?: string
}

// ---------------------------------------------------------------------------
// Phase 5: learning workspaces (Java/LLD/HLD)
// ---------------------------------------------------------------------------

export interface CodeExperiment {
  id: string
  topic_id: string
  title: string
  code: string
  language: string
  file_path?: string
  created_at: string
  updated_at: string
}

export interface UpsertCodeExperimentRequest {
  topic_id: string
  title: string
  code: string
  language?: string
  id?: string
}

export interface CodeExperimentResponse {
  ok: boolean
  experiment: CodeExperiment | null
  error?: string
}

export interface ListCodeExperimentsResponse {
  ok: boolean
  experiments: CodeExperiment[]
  error?: string
}

export interface RecallQuestion {
  id: string
  topic_id: string
  question: string
  answer: string
  difficulty: string
  last_reviewed?: string
  next_review?: string
  interval: number
  repetition: number
  state: string
  created_at: string
  updated_at: string
}

export interface UpsertRecallQuestionRequest {
  topic_id: string
  question: string
  answer: string
  difficulty?: string
  id?: string
}

export interface RecordRecallReviewRequest {
  id: string
  recall_result: string
  confidence?: number
}

export interface RecallQuestionResponse {
  ok: boolean
  question: RecallQuestion | null
  error?: string
}

export interface ListRecallQuestionsResponse {
  ok: boolean
  questions: RecallQuestion[]
  error?: string
}

export interface Diagram {
  id: string
  topic_id: string
  title: string
  mermaid_content: string
  file_path?: string
  created_at: string
  updated_at: string
}

export interface UpsertDiagramRequest {
  topic_id: string
  title: string
  mermaid_content: string
  id?: string
}

export interface DiagramResponse {
  ok: boolean
  diagram: Diagram | null
  error?: string
}

export interface ListDiagramsResponse {
  ok: boolean
  diagrams: Diagram[]
  error?: string
}

export interface DesignArtifact {
  id: string
  topic_id: string
  title: string
  requirements?: string
  api_spec?: string
  java_implementation?: string
  tests?: string
  trade_offs?: string
  created_at: string
  updated_at: string
}

export interface UpsertDesignArtifactRequest {
  topic_id: string
  title: string
  requirements?: string
  api_spec?: string
  java_implementation?: string
  tests?: string
  trade_offs?: string
  id?: string
}

export interface DesignArtifactResponse {
  ok: boolean
  artifact: DesignArtifact | null
  error?: string
}

export interface ListDesignArtifactsResponse {
  ok: boolean
  artifacts: DesignArtifact[]
  error?: string
}

export interface CaseStudy {
  id: string
  topic_id: string
  title: string
  context?: string
  diagram_content?: string
  trade_offs?: string
  created_at: string
  updated_at: string
}

export interface UpsertCaseStudyRequest {
  topic_id: string
  title: string
  context?: string
  diagram_content?: string
  trade_offs?: string
  id?: string
}

export interface CaseStudyResponse {
  ok: boolean
  case_study: CaseStudy | null
  error?: string
}

export interface ListCaseStudiesResponse {
  ok: boolean
  case_studies: CaseStudy[]
  error?: string
}

export interface TopicLink {
  id: string
  from_topic_id: string
  to_topic_id: string
  link_type: string
  created_at: string
}

export interface AddTopicLinkRequest {
  from_topic_id: string
  to_topic_id: string
  link_type?: string
}

export interface TopicLinkResponse {
  ok: boolean
  link: TopicLink | null
  error?: string
}

export interface ListTopicLinksResponse {
  ok: boolean
  links: TopicLink[]
  error?: string
}

export interface SaveWorkspaceFileRequest {
  subdir: string
  topic_id: string
  file_name: string
  content: string
}

export interface SaveWorkspaceFileResponse {
  ok: boolean
  file_path?: string
  error?: string
}

// ---------------------------------------------------------------------------
// Projects / FDE portfolio (Phase 6)
// ---------------------------------------------------------------------------

export interface Project {
  id: string
  track: string
  name: string
  slug: string
  status: string
  repo_url?: string
  local_path?: string
  readme?: string
  milestone?: string
  notes?: string
  created_at: string
  updated_at: string
}

export interface UpsertProjectRequest {
  id?: string
  track: string
  name: string
  slug?: string
  status?: string
  repo_url?: string
  local_path?: string
  readme?: string
  milestone?: string
  notes?: string
}

export interface ProjectResponse {
  ok: boolean
  project: Project | null
  error?: string
}

export interface ListProjectsResponse {
  ok: boolean
  projects: Project[]
  error?: string
}

export interface ProjectMilestone {
  id: string
  project_id: string
  name: string
  is_complete: boolean
  due_date?: string
  notes?: string
}

export interface UpsertProjectMilestoneRequest {
  id?: string
  project_id: string
  name: string
  is_complete?: boolean
  due_date?: string
  notes?: string
}

export interface ProjectTask {
  id: string
  project_id: string
  title: string
  status: string
  due_date?: string
  notes?: string
  created_at: string
  updated_at: string
}

export interface UpsertProjectTaskRequest {
  id?: string
  project_id: string
  title: string
  status?: string
  due_date?: string
  notes?: string
}

export interface ProjectLink {
  id: string
  project_id: string
  title: string
  url: string
  link_type?: string
}

export interface UpsertProjectLinkRequest {
  id?: string
  project_id: string
  title: string
  url: string
  link_type?: string
}

export interface ProjectNote {
  id: string
  project_id: string
  kind: string
  title: string
  content: string
  created_at: string
  updated_at: string
}

export interface UpsertProjectNoteRequest {
  id?: string
  project_id: string
  kind?: string
  title: string
  content: string
}

export interface ProjectDetailResponse {
  ok: boolean
  project: Project | null
  milestones: ProjectMilestone[]
  tasks: ProjectTask[]
  links: ProjectLink[]
  notes: ProjectNote[]
  error?: string
}

// ---------------------------------------------------------------------------
// Analytics / search / export (Phase 7)
// ---------------------------------------------------------------------------

export interface SearchResult {
  type: string
  id: string
  title: string
  subtitle?: string
  track?: string
}

export interface SearchResponse {
  ok: boolean
  results: SearchResult[]
  error?: string
}

export interface ExportResponse {
  ok: boolean
  destination: string
  error?: string
}

export interface TrackProgress {
  track: string
  total_topics: number
  completed: number
  in_progress: number
  not_started: number
  avg_confidence: number | null
}

export interface PlannedVsActual {
  planned_minutes: number
  actual_minutes: number
  task_count: number
  completed_count: number
}

export interface RevisionStats {
  total_items: number
  due_count: number
  overdue_count: number
  avg_confidence: number | null
}

export interface AttemptStats {
  total: number
  solved: number
  avg_duration_minutes: number | null
}

export interface ActivityStats {
  total_events: number
  active_days_last30: number
}

export interface AnalyticsResponse {
  ok: boolean
  track_progress: TrackProgress[]
  planned_vs_actual: PlannedVsActual
  revision: RevisionStats
  attempts: AttemptStats
  activity: ActivityStats
  error?: string
}
