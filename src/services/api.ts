import {
  HealthResponse,
  SettingsResponse,
  SaveSettingsResponse,
  BackupResponse,
  RestoreResponse,
  MigrationsResponse,
  Settings,
  TrackListResponse,
  ModuleListResponse,
  TopicListResponse,
  TopicDetailResponse,
  UpsertResponse,
  UpsertTopicRequest,
  UpsertSubtopicRequest,
  UpsertResourceRequest,
  UpsertDependencyRequest,
  RoadmapPhasesResponse,
  RoadmapWeeksResponse,
  WeeklyPlanItemsResponse,
  ImportRoadmapResponse,
  AgendaResponse,
  OverdueTasksResponse,
  GenerateWeekResponse,
  UpdateTaskRequest,
  UpdateTaskResponse,
  CreateTaskRequest,
  CreateTaskResponse,
  SetAvailabilityRequest,
  ReplanResponse,
  WeekReviewResponse,
  CreateLearningEntryRequest,
  CreateLearningEntryResponse,
  ListEntriesResponse,
  ActivitySummaryResponse,
  PreviewNeetcodeResponse,
  ImportNeetcodeResponse,
  ProblemListResponse,
  GetProblemsRequest,
  RecordAttemptRequest,
  RecordAttemptResponse,
  ListAttemptsResponse,
  SaveSolutionRequest,
  SaveSolutionResponse,
  GetSolutionResponse,
  RecordReviewRequest,
  RecordReviewResponse,
  GetRevisionQueueResponse,
  StartRevisionBaselineResponse,
  GetDsaProgressResponse,
  ToggleFavoriteRequest,
  ToggleFavoriteResponse,
  OpenUrlResponse,
  // Workspace (Phase 5)
  UpsertCodeExperimentRequest,
  CodeExperimentResponse,
  ListCodeExperimentsResponse,
  UpsertRecallQuestionRequest,
  RecordRecallReviewRequest,
  RecallQuestionResponse,
  ListRecallQuestionsResponse,
  UpsertDiagramRequest,
  DiagramResponse,
  ListDiagramsResponse,
  UpsertDesignArtifactRequest,
  DesignArtifactResponse,
  ListDesignArtifactsResponse,
  UpsertCaseStudyRequest,
  CaseStudyResponse,
  ListCaseStudiesResponse,
  AddTopicLinkRequest,
  TopicLinkResponse,
  ListTopicLinksResponse,
  SaveWorkspaceFileRequest,
  SaveWorkspaceFileResponse,
} from '@/types'

declare global {
  interface Window {
    __TAURI__?: {
      invoke?: (cmd: string, args?: Record<string, unknown>) => Promise<unknown>
      core?: {
        invoke: (cmd: string, args?: Record<string, unknown>) => Promise<unknown>
      }
    }
  }
}

function getInvoke() {
  if (typeof window === 'undefined') return null
  const tauri = window.__TAURI__
  if (!tauri) return null
  if (typeof tauri.core?.invoke === 'function') return tauri.core.invoke
  if (typeof tauri.invoke === 'function') return tauri.invoke
  return null
}

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const tauriInvoke = getInvoke()
  if (tauriInvoke) {
    return tauriInvoke(cmd, args) as Promise<T>
  }
  // Dev fallback: return sensible defaults so the UI works without Tauri
  switch (cmd) {
    case 'health_check':
      return { ok: true, health: { connected: false, path: '', version: 0 } } as T
    case 'get_settings':
      return { ok: true, settings: {} as Settings } as T
    case 'save_settings':
      return { ok: true } as T
    case 'backup_database':
      return { ok: true, path: '', error: undefined } as T
    case 'restore_database':
      return { ok: true, error: undefined } as T
    case 'run_migrations':
      return { ok: true, version: 1, error: undefined } as T
    case 'get_tracks':
      return { ok: true, tracks: [] } as T
    case 'get_modules':
      return { ok: true, modules: [] } as T
    case 'get_topics':
      return { ok: true, topics: [] } as T
    case 'get_topic_detail':
      return { ok: true, topic: null, subtopics: [], resources: [], dependencies: [] } as T
    case 'upsert_topic':
      return { ok: true, id: '' } as T
    case 'upsert_subtopic':
      return { ok: true, id: '' } as T
    case 'upsert_resource':
      return { ok: true, id: '' } as T
    case 'upsert_dependency':
      return { ok: true, id: '' } as T
    case 'get_roadmap_phases':
      return { ok: true, phases: [] } as T
    case 'get_roadmap_weeks':
      return { ok: true, weeks: [] } as T
    case 'get_weekly_plan_items':
      return { ok: true, items: [] } as T
    case 'import_roadmap':
      return {
        ok: true,
        track_count: 0,
        topic_count: 0,
        week_count: 0,
        warnings: [],
        error: undefined,
      } as T
    case 'get_agenda':
      return {
        ok: true,
        plan: null,
        tasks: [],
        due_revisions: 0,
        planned_minutes: 0,
        completed_minutes: 0,
        error: undefined,
      } as T
    case 'generate_week':
      return {
        ok: true,
        tasks_created: 0,
        days_generated: 0,
        warnings: [],
        error: undefined,
      } as T
    case 'set_day_availability':
    case 'save_weekly_goal':
      return { ok: true, task: null, error: undefined } as T
    case 'update_study_task':
      return { ok: true, task: null, error: undefined } as T
    case 'create_study_task':
      return { ok: true, id: null, error: undefined } as T
    case 'replan_backlog':
      return { ok: true, moved: 0, unplaced: 0, error: undefined } as T
    case 'list_overdue_tasks':
      return { ok: true, tasks: [], error: undefined } as T
    case 'get_week_review':
      return {
        ok: true,
        week_start: '',
        goal: null,
        planned: 0,
        completed: 0,
        skipped: 0,
        planned_minutes: 0,
        completed_minutes: 0,
        by_track: {},
        tasks: [],
        error: undefined,
      } as T
    case 'create_learning_entry':
      return { ok: true, id: null, error: undefined } as T
    case 'list_learning_entries':
      return { ok: true, entries: [], error: undefined } as T
    case 'get_activity_summary':
      return {
        ok: true,
        days: [],
        current_streak: 0,
        longest_streak: 0,
        total_minutes: 0,
        active_days: 0,
        min_meaningful_minutes: 15,
        error: undefined,
      } as T
    case 'preview_neetcode':
    case 'import_neetcode':
      return {
        ok: true,
        imported: 0,
        skipped: 0,
        previously_covered: 0,
        new_learning: 0,
        warnings: [],
        error: undefined,
      } as T
    case 'get_problems':
      return { ok: true, problems: [], total: 0, error: undefined } as T
    case 'record_attempt':
      return { ok: true, id: '', error: undefined } as T
    case 'list_attempts':
      return { ok: true, attempts: [], error: undefined } as T
    case 'save_solution':
      return { ok: true, file_path: '', error: undefined } as T
    case 'get_solution':
      return { ok: true, solution: null, error: undefined } as T
    case 'record_review':
      return { ok: true, revision_item: null, error: undefined } as T
    case 'get_revision_queue':
      return { ok: true, items: [], error: undefined } as T
    case 'start_revision_baseline':
      return { ok: true, created: 0, error: undefined } as T
    case 'get_dsa_progress':
      return {
        ok: true,
        categories: [],
        total_problems: 0,
        total_attempted: 0,
        total_solved: 0,
        error: undefined,
      } as T
    case 'toggle_favorite':
      return { ok: true, is_favorite: false, error: undefined } as T
    case 'open_external_url':
      return { ok: true, error: undefined } as T
    // Workspace (Phase 5)
    case 'upsert_code_experiment':
      return { ok: true, id: '', error: undefined } as T
    case 'get_code_experiment':
      return { ok: true, experiment: null, error: undefined } as T
    case 'list_code_experiments':
      return { ok: true, experiments: [], error: undefined } as T
    case 'upsert_recall_question':
      return { ok: true, id: '', error: undefined } as T
    case 'get_recall_question':
      return { ok: true, question: null, error: undefined } as T
    case 'list_recall_questions':
      return { ok: true, questions: [], error: undefined } as T
    case 'record_recall_review':
      return { ok: true, id: '', error: undefined } as T
    case 'upsert_diagram':
      return { ok: true, id: '', error: undefined } as T
    case 'get_diagram':
      return { ok: true, diagram: null, error: undefined } as T
    case 'list_diagrams':
      return { ok: true, diagrams: [], error: undefined } as T
    case 'upsert_design_artifact':
      return { ok: true, id: '', error: undefined } as T
    case 'get_design_artifact':
      return { ok: true, artifact: null, error: undefined } as T
    case 'list_design_artifacts':
      return { ok: true, artifacts: [], error: undefined } as T
    case 'upsert_case_study':
      return { ok: true, id: '', error: undefined } as T
    case 'get_case_study':
      return { ok: true, case_study: null, error: undefined } as T
    case 'list_case_studies':
      return { ok: true, case_studies: [], error: undefined } as T
    case 'add_topic_link':
      return { ok: true, link: null, error: undefined } as T
    case 'list_topic_links':
      return { ok: true, links: [], error: undefined } as T
    case 'save_workspace_file':
      return { ok: true, file_path: '', error: undefined } as T
    default:
      throw new Error(`Unknown command: ${cmd}`)
  }
}

export const api = {
  healthCheck: () => invoke<HealthResponse>('health_check'),
  getSettings: () => invoke<SettingsResponse>('get_settings'),
  saveSettings: (settings: Settings) =>
    invoke<SaveSettingsResponse>('save_settings', { req: { settings } }),
  backupDatabase: (destination: string) =>
    invoke<BackupResponse>('backup_database', { req: { destination } }),
  restoreDatabase: (source: string) => invoke<RestoreResponse>('restore_database', { req: { source } }),
  runMigrations: () => invoke<MigrationsResponse>('run_migrations'),

  // Curriculum
  getTracks: () => invoke<TrackListResponse>('get_tracks'),
  getModules: (trackKey: string) => invoke<ModuleListResponse>('get_modules', { track_key: trackKey }),
  getTopics: (moduleId: string) => invoke<TopicListResponse>('get_topics', { module_id: moduleId }),
  getTopicDetail: (topicId: string) => invoke<TopicDetailResponse>('get_topic_detail', { topic_id: topicId }),
  upsertTopic: (req: UpsertTopicRequest) =>
    invoke<UpsertResponse>('upsert_topic', { req }),
  upsertSubtopic: (req: UpsertSubtopicRequest) =>
    invoke<UpsertResponse>('upsert_subtopic', { req }),
  upsertResource: (req: UpsertResourceRequest) =>
    invoke<UpsertResponse>('upsert_resource', { req }),
  upsertDependency: (req: UpsertDependencyRequest) =>
    invoke<UpsertResponse>('upsert_dependency', { req }),

  // Roadmap
  getRoadmapPhases: () => invoke<RoadmapPhasesResponse>('get_roadmap_phases'),
  getRoadmapWeeks: (phaseKey?: string) =>
    invoke<RoadmapWeeksResponse>('get_roadmap_weeks', phaseKey ? { phase_key: phaseKey } : {}),
  getWeeklyPlanItems: (weekId: string) =>
    invoke<WeeklyPlanItemsResponse>('get_weekly_plan_items', { week_id: weekId }),
  importRoadmap: (content: string) =>
    invoke<ImportRoadmapResponse>('import_roadmap', { req: { content } }),

  // Planner
  getAgenda: (date: string) => invoke<AgendaResponse>('get_agenda', { date }),
  listOverdueTasks: (today: string) =>
    invoke<OverdueTasksResponse>('list_overdue_tasks', { today }),
  generateWeek: (weekStart: string, force = false) =>
    invoke<GenerateWeekResponse>('generate_week', { req: { week_start: weekStart, force } }),
  setDayAvailability: (req: SetAvailabilityRequest) =>
    invoke<UpdateTaskResponse>('set_day_availability', { req }),
  updateStudyTask: (req: UpdateTaskRequest) =>
    invoke<UpdateTaskResponse>('update_study_task', { req }),
  createStudyTask: (req: CreateTaskRequest) =>
    invoke<CreateTaskResponse>('create_study_task', { req }),
  replanBacklog: (today: string) =>
    invoke<ReplanResponse>('replan_backlog', { req: { today } }),
  saveWeeklyGoal: (req: {
    week_start_date: string
    track_targets?: string
    note?: string
    reviewed?: boolean
  }) => invoke<UpdateTaskResponse>('save_weekly_goal', { req }),
  getWeekReview: (weekStart: string) =>
    invoke<WeekReviewResponse>('get_week_review', { week_start: weekStart }),

  // Learning entries + activity
  createLearningEntry: (req: CreateLearningEntryRequest) =>
    invoke<CreateLearningEntryResponse>('create_learning_entry', { req }),
  listLearningEntries: (from: string, to: string) =>
    invoke<ListEntriesResponse>('list_learning_entries', { req: { from, to } }),
  getActivitySummary: (from: string, to: string, today: string) =>
    invoke<ActivitySummaryResponse>('get_activity_summary', { req: { from, to, today } }),

  // DSA / NeetCode 250
  previewNeetcode: (content: string) =>
    invoke<PreviewNeetcodeResponse>('preview_neetcode', { req: { content } }),
  importNeetcode: (content: string) =>
    invoke<ImportNeetcodeResponse>('import_neetcode', { req: { content } }),
  getProblems: (req: GetProblemsRequest) =>
    invoke<ProblemListResponse>('get_problems', { req }),
  recordAttempt: (req: RecordAttemptRequest) =>
    invoke<RecordAttemptResponse>('record_attempt', { req }),
  listAttempts: (problemId: string) =>
    invoke<ListAttemptsResponse>('list_attempts', { req: { search: problemId } }),
  saveSolution: (req: SaveSolutionRequest) =>
    invoke<SaveSolutionResponse>('save_solution', { req }),
  getSolution: (problemId: string) =>
    invoke<GetSolutionResponse>('get_solution', { req: { search: problemId } }),
  recordReview: (req: RecordReviewRequest) =>
    invoke<RecordReviewResponse>('record_review', { req }),
  getRevisionQueue: () =>
    invoke<GetRevisionQueueResponse>('get_revision_queue', { req: {} }),
  startRevisionBaseline: () =>
    invoke<StartRevisionBaselineResponse>('start_revision_baseline', { req: {} }),
  getDsaProgress: () =>
    invoke<GetDsaProgressResponse>('get_dsa_progress', { req: {} }),
  toggleFavorite: (req: ToggleFavoriteRequest) =>
    invoke<ToggleFavoriteResponse>('toggle_favorite', { req }),
  openExternalUrl: (url: string) =>
    invoke<OpenUrlResponse>('open_external_url', { url }),

  // Workspace (Phase 5)
  upsertCodeExperiment: (req: UpsertCodeExperimentRequest) =>
    invoke<UpsertResponse>('upsert_code_experiment', { req }),
  getCodeExperiment: (id: string) =>
    invoke<CodeExperimentResponse>('get_code_experiment', { id }),
  listCodeExperiments: (topicId: string) =>
    invoke<ListCodeExperimentsResponse>('list_code_experiments', { topic_id: topicId }),
  upsertRecallQuestion: (req: UpsertRecallQuestionRequest) =>
    invoke<UpsertResponse>('upsert_recall_question', { req }),
  recordRecallReview: (req: RecordRecallReviewRequest) =>
    invoke<UpsertResponse>('record_recall_review', { req }),
  getRecallQuestion: (id: string) =>
    invoke<RecallQuestionResponse>('get_recall_question', { id }),
  listRecallQuestions: (topicId: string) =>
    invoke<ListRecallQuestionsResponse>('list_recall_questions', { topic_id: topicId }),
  upsertDiagram: (req: UpsertDiagramRequest) =>
    invoke<UpsertResponse>('upsert_diagram', { req }),
  getDiagram: (id: string) =>
    invoke<DiagramResponse>('get_diagram', { id }),
  listDiagrams: (topicId: string) =>
    invoke<ListDiagramsResponse>('list_diagrams', { topic_id: topicId }),
  upsertDesignArtifact: (req: UpsertDesignArtifactRequest) =>
    invoke<UpsertResponse>('upsert_design_artifact', { req }),
  getDesignArtifact: (id: string) =>
    invoke<DesignArtifactResponse>('get_design_artifact', { id }),
  listDesignArtifacts: (topicId: string) =>
    invoke<ListDesignArtifactsResponse>('list_design_artifacts', { topic_id: topicId }),
  upsertCaseStudy: (req: UpsertCaseStudyRequest) =>
    invoke<UpsertResponse>('upsert_case_study', { req }),
  getCaseStudy: (id: string) =>
    invoke<CaseStudyResponse>('get_case_study', { id }),
  listCaseStudies: (topicId: string) =>
    invoke<ListCaseStudiesResponse>('list_case_studies', { topic_id: topicId }),
  addTopicLink: (req: AddTopicLinkRequest) =>
    invoke<TopicLinkResponse>('add_topic_link', { req }),
  listTopicLinks: (topicId: string) =>
    invoke<ListTopicLinksResponse>('list_topic_links', { topic_id: topicId }),
  saveWorkspaceFile: (req: SaveWorkspaceFileRequest) =>
    invoke<SaveWorkspaceFileResponse>('save_workspace_file', { req }),
}
