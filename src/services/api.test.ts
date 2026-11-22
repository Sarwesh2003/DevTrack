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

  it('listOverdueTasks calls list_overdue_tasks with today', async () => {
    mockInvoke.mockResolvedValue({ ok: true, tasks: [] })
    await api.listOverdueTasks('2026-01-08')
    expect(mockInvoke).toHaveBeenCalledWith('list_overdue_tasks', {
      today: '2026-01-08',
    })
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

  it('previewNeetcode calls preview_neetcode with content', async () => {
    mockInvoke.mockResolvedValue({ ok: true, total_rows: 250, valid_rows: 250, previously_covered: 109, new_learning: 141, by_category: [], sample: [], error: undefined })
    await api.previewNeetcode('official_order,problem_id,title,url,category,difficulty,category_order,source_name,source_version\n1,a,T,https://x/a,Arrays & Hashing,Easy,1,NeetCode 250,2026-11-13')
    expect(mockInvoke).toHaveBeenCalledWith('preview_neetcode', { req: { content: expect.any(String) } })
  })

  it('importNeetcode calls import_neetcode with content', async () => {
    mockInvoke.mockResolvedValue({ ok: true, imported: 250, skipped: 0, previously_covered: 109, new_learning: 141, warnings: [], error: undefined })
    await api.importNeetcode('content')
    expect(mockInvoke).toHaveBeenCalledWith('import_neetcode', { req: { content: 'content' } })
  })

  it('getProblems nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, problems: [], total: 0, error: undefined })
    await api.getProblems({ category: 'Arrays & Hashing', cover_filter: 'new' })
    expect(mockInvoke).toHaveBeenCalledWith('get_problems', {
      req: { category: 'Arrays & Hashing', cover_filter: 'new' },
    })
  })

  it('recordAttempt nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, id: 'attempt-1', error: undefined })
    await api.recordAttempt({ problem_id: 'two-sum', attempt_date: '2026-01-05', outcome: 'solved', duration_minutes: 20 })
    expect(mockInvoke).toHaveBeenCalledWith('record_attempt', {
      req: { problem_id: 'two-sum', attempt_date: '2026-01-05', outcome: 'solved', duration_minutes: 20 },
    })
  })

  it('listAttempts sends problem_id as search', async () => {
    mockInvoke.mockResolvedValue({ ok: true, attempts: [], error: undefined })
    await api.listAttempts('two-sum')
    expect(mockInvoke).toHaveBeenCalledWith('list_attempts', { req: { search: 'two-sum' } })
  })

  it('recordReview nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, revision_item: null, error: undefined })
    await api.recordReview({ revision_item_id: 'ri-1', reviewed_at: '2026-01-06', recall_result: 'correct_clear', confidence: 4 })
    expect(mockInvoke).toHaveBeenCalledWith('record_review', {
      req: { revision_item_id: 'ri-1', reviewed_at: '2026-01-06', recall_result: 'correct_clear', confidence: 4 },
    })
  })

  it('getRevisionQueue calls get_revision_queue', async () => {
    mockInvoke.mockResolvedValue({ ok: true, items: [], error: undefined })
    await api.getRevisionQueue()
    expect(mockInvoke).toHaveBeenCalledWith('get_revision_queue', { req: {} })
  })

  it('startRevisionBaseline calls start_revision_baseline', async () => {
    mockInvoke.mockResolvedValue({ ok: true, created: 0, error: undefined })
    await api.startRevisionBaseline()
    expect(mockInvoke).toHaveBeenCalledWith('start_revision_baseline', { req: {} })
  })

  it('getDsaProgress calls get_dsa_progress', async () => {
    mockInvoke.mockResolvedValue({ ok: true, categories: [], total_problems: 0, total_attempted: 0, total_solved: 0, error: undefined })
    await api.getDsaProgress()
    expect(mockInvoke).toHaveBeenCalledWith('get_dsa_progress', { req: {} })
  })

  it('toggleFavorite nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, is_favorite: true, error: undefined })
    await api.toggleFavorite({ problem_id: 'p-1', favorite: true })
    expect(mockInvoke).toHaveBeenCalledWith('toggle_favorite', {
      req: { problem_id: 'p-1', favorite: true },
    })
  })

  it('openExternalUrl sends the url', async () => {
    mockInvoke.mockResolvedValue({ ok: true, error: undefined })
    await api.openExternalUrl('https://leetcode.com/problems/two-sum/')
    expect(mockInvoke).toHaveBeenCalledWith('open_external_url', {
      url: 'https://leetcode.com/problems/two-sum/',
    })
  })

  it('upsertCodeExperiment nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, id: 'exp-1' })
    await api.upsertCodeExperiment({ topic_id: 't-1', title: 'Demo', code: 'class A {}', language: 'java' })
    expect(mockInvoke).toHaveBeenCalledWith('upsert_code_experiment', {
      req: { topic_id: 't-1', title: 'Demo', code: 'class A {}', language: 'java' },
    })
  })

  it('listCodeExperiments sends topic_id', async () => {
    mockInvoke.mockResolvedValue({ ok: true, experiments: [], error: undefined })
    await api.listCodeExperiments('t-1')
    expect(mockInvoke).toHaveBeenCalledWith('list_code_experiments', { topic_id: 't-1' })
  })

  it('upsertRecallQuestion nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, id: 'q-1' })
    await api.upsertRecallQuestion({ topic_id: 't-1', question: 'Q?', answer: 'A', difficulty: 'easy' })
    expect(mockInvoke).toHaveBeenCalledWith('upsert_recall_question', {
      req: { topic_id: 't-1', question: 'Q?', answer: 'A', difficulty: 'easy' },
    })
  })

  it('recordRecallReview nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, id: 'q-1' })
    await api.recordRecallReview({ id: 'q-1', recall_result: 'good' })
    expect(mockInvoke).toHaveBeenCalledWith('record_recall_review', {
      req: { id: 'q-1', recall_result: 'good' },
    })
  })

  it('upsertDiagram nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, id: 'd-1' })
    await api.upsertDiagram({ topic_id: 't-1', title: 'Flow', mermaid_content: 'graph TD' })
    expect(mockInvoke).toHaveBeenCalledWith('upsert_diagram', {
      req: { topic_id: 't-1', title: 'Flow', mermaid_content: 'graph TD' },
    })
  })

  it('listDesignArtifacts sends topic_id', async () => {
    mockInvoke.mockResolvedValue({ ok: true, artifacts: [], error: undefined })
    await api.listDesignArtifacts('t-1')
    expect(mockInvoke).toHaveBeenCalledWith('list_design_artifacts', { topic_id: 't-1' })
  })

  it('listCaseStudies sends topic_id', async () => {
    mockInvoke.mockResolvedValue({ ok: true, case_studies: [], error: undefined })
    await api.listCaseStudies('t-1')
    expect(mockInvoke).toHaveBeenCalledWith('list_case_studies', { topic_id: 't-1' })
  })

  it('addTopicLink nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, link: null, error: undefined })
    await api.addTopicLink({ from_topic_id: 't-1', to_topic_id: 't-2', link_type: 'related' })
    expect(mockInvoke).toHaveBeenCalledWith('add_topic_link', {
      req: { from_topic_id: 't-1', to_topic_id: 't-2', link_type: 'related' },
    })
  })

  it('listTopicLinks sends topic_id', async () => {
    mockInvoke.mockResolvedValue({ ok: true, links: [], error: undefined })
    await api.listTopicLinks('t-1')
    expect(mockInvoke).toHaveBeenCalledWith('list_topic_links', { topic_id: 't-1' })
  })

  it('saveWorkspaceFile nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, file_path: '/tmp/x.java', error: undefined })
    await api.saveWorkspaceFile({ subdir: 'experiments', topic_id: 't-1', file_name: 'A.java', content: 'class A {}' })
    expect(mockInvoke).toHaveBeenCalledWith('save_workspace_file', {
      req: { subdir: 'experiments', topic_id: 't-1', file_name: 'A.java', content: 'class A {}' },
    })
  })

  it('listProjects sends optional track', async () => {
    mockInvoke.mockResolvedValue({ ok: true, projects: [], error: undefined })
    await api.listProjects('E')
    expect(mockInvoke).toHaveBeenCalledWith('list_projects', { track: 'E' })
    mockInvoke.mockResolvedValue({ ok: true, projects: [], error: undefined })
    await api.listProjects()
    expect(mockInvoke).toHaveBeenCalledWith('list_projects', {})
  })

  it('getProject sends id', async () => {
    mockInvoke.mockResolvedValue({ ok: true, project: null, error: undefined })
    await api.getProject('p-1')
    expect(mockInvoke).toHaveBeenCalledWith('get_project', { id: 'p-1' })
  })

  it('upsertProject nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, id: 'p-1' })
    await api.upsertProject({ track: 'E', name: 'FDE Capstone', slug: 'fde-capstone', status: 'active' })
    expect(mockInvoke).toHaveBeenCalledWith('upsert_project', {
      req: { track: 'E', name: 'FDE Capstone', slug: 'fde-capstone', status: 'active' },
    })
  })

  it('upsertProjectMilestone nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, id: 'm-1' })
    await api.upsertProjectMilestone({ project_id: 'p-1', name: 'M1 \u2014 Problem', is_complete: false })
    expect(mockInvoke).toHaveBeenCalledWith('upsert_project_milestone', {
      req: { project_id: 'p-1', name: 'M1 \u2014 Problem', is_complete: false },
    })
  })

  it('upsertProjectTask nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, id: 't-1' })
    await api.upsertProjectTask({ project_id: 'p-1', title: 'Setup repo', status: 'todo' })
    expect(mockInvoke).toHaveBeenCalledWith('upsert_project_task', {
      req: { project_id: 'p-1', title: 'Setup repo', status: 'todo' },
    })
  })

  it('upsertProjectLink nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, id: 'l-1' })
    await api.upsertProjectLink({ project_id: 'p-1', title: 'Doc', url: 'https://x', link_type: 'doc' })
    expect(mockInvoke).toHaveBeenCalledWith('upsert_project_link', {
      req: { project_id: 'p-1', title: 'Doc', url: 'https://x', link_type: 'doc' },
    })
  })

  it('upsertProjectNote nests the request', async () => {
    mockInvoke.mockResolvedValue({ ok: true, id: 'n-1' })
    await api.upsertProjectNote({ project_id: 'p-1', kind: 'evaluation', title: 'Review', content: 'Good' })
    expect(mockInvoke).toHaveBeenCalledWith('upsert_project_note', {
      req: { project_id: 'p-1', kind: 'evaluation', title: 'Review', content: 'Good' },
    })
  })

  it('getProjectDetail sends id', async () => {
    mockInvoke.mockResolvedValue({ ok: true, project: null, milestones: [], tasks: [], links: [], notes: [], error: undefined })
    await api.getProjectDetail('p-1')
    expect(mockInvoke).toHaveBeenCalledWith('get_project_detail', { id: 'p-1' })
  })

  it('seedFdeProject sends empty args', async () => {
    mockInvoke.mockResolvedValue({ ok: true, id: 'p-1' })
    await api.seedFdeProject()
    expect(mockInvoke).toHaveBeenCalledWith('seed_fde_project', {})
  })
})
