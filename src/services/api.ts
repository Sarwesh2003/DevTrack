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
} from '@/types'

declare global {
  interface Window {
    __TAURI__?: {
      invoke: (cmd: string, args?: Record<string, unknown>) => Promise<unknown>
    }
  }
}

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (typeof window !== 'undefined' && window.__TAURI__?.invoke) {
    return window.__TAURI__.invoke(cmd, args) as Promise<T>
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
    default:
      throw new Error(`Unknown command: ${cmd}`)
  }
}

export const api = {
  healthCheck: () => invoke<HealthResponse>('health_check'),
  getSettings: () => invoke<SettingsResponse>('get_settings'),
  saveSettings: (settings: Settings) => invoke<SaveSettingsResponse>('save_settings', { settings }),
  backupDatabase: (destination: string) => invoke<BackupResponse>('backup_database', { destination }),
  restoreDatabase: (source: string) => invoke<RestoreResponse>('restore_database', { source }),
  runMigrations: () => invoke<MigrationsResponse>('run_migrations'),

  // Curriculum
  getTracks: () => invoke<TrackListResponse>('get_tracks'),
  getModules: (trackKey: string) => invoke<ModuleListResponse>('get_modules', { track_key: trackKey }),
  getTopics: (moduleId: string) => invoke<TopicListResponse>('get_topics', { module_id: moduleId }),
  getTopicDetail: (topicId: string) => invoke<TopicDetailResponse>('get_topic_detail', { topic_id: topicId }),
  upsertTopic: (req: UpsertTopicRequest) =>
    invoke<UpsertResponse>('upsert_topic', req as unknown as Record<string, unknown>),
  upsertSubtopic: (req: UpsertSubtopicRequest) =>
    invoke<UpsertResponse>('upsert_subtopic', req as unknown as Record<string, unknown>),
  upsertResource: (req: UpsertResourceRequest) =>
    invoke<UpsertResponse>('upsert_resource', req as unknown as Record<string, unknown>),
  upsertDependency: (req: UpsertDependencyRequest) =>
    invoke<UpsertResponse>('upsert_dependency', req as unknown as Record<string, unknown>),

  // Roadmap
  getRoadmapPhases: () => invoke<RoadmapPhasesResponse>('get_roadmap_phases'),
  getRoadmapWeeks: (phaseKey?: string) =>
    invoke<RoadmapWeeksResponse>('get_roadmap_weeks', phaseKey ? { phase_key: phaseKey } : {}),
  getWeeklyPlanItems: (weekId: string) =>
    invoke<WeeklyPlanItemsResponse>('get_weekly_plan_items', { week_id: weekId }),
  importRoadmap: (content: string) =>
    invoke<ImportRoadmapResponse>('import_roadmap', { content }),
}
