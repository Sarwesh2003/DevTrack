import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'

// Mock window.__TAURI__.invoke
const mockInvoke = vi.fn()
Object.defineProperty(window, '__TAURI__', {
  value: { invoke: mockInvoke },
  configurable: true,
})

import { api } from './api'

describe('api service', () => {
  beforeEach(() => {
    mockInvoke.mockReset()
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it('healthCheck calls health_check command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, health: { connected: true, path: '/tmp/test.sqlite', version: 1 } })
    const result = await api.healthCheck()
    expect(mockInvoke).toHaveBeenCalledWith('health_check', undefined)
    expect(result.ok).toBe(true)
    expect(result.health.connected).toBe(true)
  })

  it('getSettings calls get_settings command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, settings: { theme: 'dark' } })
    const result = await api.getSettings()
    expect(mockInvoke).toHaveBeenCalledWith('get_settings', undefined)
    expect(result.ok).toBe(true)
  })

  it('saveSettings calls save_settings command', async () => {
    mockInvoke.mockResolvedValue({ ok: true })
    const result = await api.saveSettings({ theme: 'light' })
    expect(mockInvoke).toHaveBeenCalledWith('save_settings', { req: { settings: { theme: 'light' } } })
    expect(result.ok).toBe(true)
  })

  it('backupDatabase calls backup_database command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, path: '/backup/devtrack.sqlite', error: undefined })
    const result = await api.backupDatabase('/backup/devtrack.sqlite')
    expect(mockInvoke).toHaveBeenCalledWith('backup_database', { req: { destination: '/backup/devtrack.sqlite' } })
    expect(result.ok).toBe(true)
  })

  it('restoreDatabase calls restore_database command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, error: undefined })
    const result = await api.restoreDatabase('/backup/devtrack.sqlite')
    expect(mockInvoke).toHaveBeenCalledWith('restore_database', { req: { source: '/backup/devtrack.sqlite' } })
    expect(result.ok).toBe(true)
  })

  it('runMigrations calls run_migrations command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, version: 1, error: undefined })
    const result = await api.runMigrations()
    expect(mockInvoke).toHaveBeenCalledWith('run_migrations', undefined)
    expect(result.ok).toBe(true)
  })

  it('getTracks calls get_tracks command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, tracks: [] })
    const result = await api.getTracks()
    expect(mockInvoke).toHaveBeenCalledWith('get_tracks', undefined)
    expect(result.ok).toBe(true)
  })

  it('getModules calls get_modules command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, modules: [] })
    const result = await api.getModules('track-dsa')
    expect(mockInvoke).toHaveBeenCalledWith('get_modules', { track_key: 'track-dsa' })
    expect(result.ok).toBe(true)
  })

  it('getTopics calls get_topics command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, topics: [] })
    const result = await api.getTopics('module-id')
    expect(mockInvoke).toHaveBeenCalledWith('get_topics', { module_id: 'module-id' })
    expect(result.ok).toBe(true)
  })

  it('getTopicDetail calls get_topic_detail command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, topic: null, subtopics: [], resources: [], dependencies: [] })
    const result = await api.getTopicDetail('topic-id')
    expect(mockInvoke).toHaveBeenCalledWith('get_topic_detail', { topic_id: 'topic-id' })
    expect(result.ok).toBe(true)
  })

  it('upsertTopic calls upsert_topic command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, id: 'topic-id' })
    const result = await api.upsertTopic({
      module_id: 'module-id',
      key: 'A1',
      title: 'Test Topic',
    })
    expect(mockInvoke).toHaveBeenCalledWith('upsert_topic', {
      req: {
        module_id: 'module-id',
        key: 'A1',
        title: 'Test Topic',
      },
    })
    expect(result.ok).toBe(true)
  })

  it('getRoadmapPhases calls get_roadmap_phases command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, phases: [] })
    const result = await api.getRoadmapPhases()
    expect(mockInvoke).toHaveBeenCalledWith('get_roadmap_phases', undefined)
    expect(result.ok).toBe(true)
  })

  it('getRoadmapWeeks calls get_roadmap_weeks command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, weeks: [] })
    const result = await api.getRoadmapWeeks('phase1')
    expect(mockInvoke).toHaveBeenCalledWith('get_roadmap_weeks', { phase_key: 'phase1' })
    expect(result.ok).toBe(true)
  })

  it('getWeeklyPlanItems calls get_weekly_plan_items command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, items: [] })
    const result = await api.getWeeklyPlanItems('week-id')
    expect(mockInvoke).toHaveBeenCalledWith('get_weekly_plan_items', { week_id: 'week-id' })
    expect(result.ok).toBe(true)
  })

  it('importRoadmap calls import_roadmap command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, track_count: 5, topic_count: 44, week_count: 26, warnings: [], error: undefined })
    const result = await api.importRoadmap('# Track content')
    expect(mockInvoke).toHaveBeenCalledWith('import_roadmap', { req: { content: '# Track content' } })
    expect(result.ok).toBe(true)
    expect(result.track_count).toBe(5)
  })

  it('getAgenda calls get_agenda with date', async () => {
    mockInvoke.mockResolvedValue({ ok: true, plan: null, tasks: [], due_revisions: 0, planned_minutes: 0, completed_minutes: 0 })
    await api.getAgenda('2026-01-05')
    expect(mockInvoke).toHaveBeenCalledWith('get_agenda', { date: '2026-01-05' })
  })

  it('generateWeek calls generate_week with snake_case request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, tasks_created: 3, days_generated: 3, warnings: [] })
    await api.generateWeek('2026-01-05', true)
    expect(mockInvoke).toHaveBeenCalledWith('generate_week', {
      req: { week_start: '2026-01-05', force: true },
    })
  })

  it('updateStudyTask nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, task: null })
    await api.updateStudyTask({ id: 'task-1', status: 'done', actual_minutes: 40 })
    expect(mockInvoke).toHaveBeenCalledWith('update_study_task', {
      req: { id: 'task-1', status: 'done', actual_minutes: 40 },
    })
  })

  it('undoing a task sends planned with cleared actual minutes', async () => {
    mockInvoke.mockResolvedValue({ ok: true, task: null })
    await api.updateStudyTask({ id: 'task-2', status: 'planned', actual_minutes: undefined, output: undefined })
    expect(mockInvoke).toHaveBeenCalledWith('update_study_task', expect.objectContaining({ req: { id: 'task-2', status: 'planned' } }))
  })

  it('replanBacklog sends today', async () => {
    mockInvoke.mockResolvedValue({ ok: true, moved: 2, unplaced: 0 })
    await api.replanBacklog('2026-01-08')
    expect(mockInvoke).toHaveBeenCalledWith('replan_backlog', { req: { today: '2026-01-08' } })
  })

  it('getWeekReview sends week_start', async () => {
    mockInvoke.mockResolvedValue({ ok: true, week_start: '2026-01-05', goal: null, planned: 0, completed: 0, skipped: 0, planned_minutes: 0, completed_minutes: 0, by_track: {}, tasks: [] })
    await api.getWeekReview('2026-01-05')
    expect(mockInvoke).toHaveBeenCalledWith('get_week_review', { week_start: '2026-01-05' })
  })

  it('createLearningEntry nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, id: 'entry-1' })
    await api.createLearningEntry({
      title: 'Read JVM notes',
      entry_date: '2026-01-05',
      track: 'java',
      description: 'Studied class loading',
    })
    expect(mockInvoke).toHaveBeenCalledWith('create_learning_entry', {
      req: {
        title: 'Read JVM notes',
        entry_date: '2026-01-05',
        track: 'java',
        description: 'Studied class loading',
      },
    })
  })

  it('getActivitySummary nests from/to/today', async () => {
    mockInvoke.mockResolvedValue({ ok: true, days: [], current_streak: 0, longest_streak: 0, total_minutes: 0, active_days: 0, min_meaningful_minutes: 15 })
    await api.getActivitySummary('2026-01-01', '2026-01-31', '2026-01-05')
    expect(mockInvoke).toHaveBeenCalledWith('get_activity_summary', {
      req: { from: '2026-01-01', to: '2026-01-31', today: '2026-01-05' },
    })
  })
})
