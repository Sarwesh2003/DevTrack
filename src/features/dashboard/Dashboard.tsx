import { useEffect, useState, useCallback } from 'react'
import { api } from '@/services/api'
import type {
  Track,
  AgendaResponse,
  StudyTask,
  ActivitySummaryResponse,
  LearningEntry,
} from '@/types'
import {
  todayISO,
  weekStartISO,
  addDaysISO,
  formatDateHuman,
} from '@/lib/dates'

type Page = 'dashboard' | 'settings' | 'roadmap' | 'review' | 'dsa' | 'workspace' | 'projects' | 'analytics' | 'search'

const isTauri =
  typeof window !== 'undefined' &&
  (typeof window.__TAURI__?.core?.invoke === 'function' ||
    typeof window.__TAURI__?.invoke === 'function')

const TRACK_LABEL: Record<string, string> = {
  a: 'Java',
  b: 'DSA',
  c: 'LLD',
  d: 'HLD',
  e: 'FDE',
  review: 'Review',
  revision: 'Revision',
  custom: 'Custom',
}

const GROUPS: { kind: string; label: string }[] = [
  { kind: 'planned', label: 'Primary learning' },
  { kind: 'revision', label: 'Revision' },
  { kind: 'review', label: 'Weekly review' },
  { kind: 'stretch', label: 'Optional stretch' },
  { kind: 'custom', label: 'Additional learning' },
]

function trackLabel(track: string): string {
  return TRACK_LABEL[track] ?? track
}

export default function Dashboard({ onNavigate }: { onNavigate: (page: Page) => void }) {
  const [today] = useState(todayISO())
  const [viewDate, setViewDate] = useState(today)
  const [overdue, setOverdue] = useState<StudyTask[]>([])
  const [tracks, setTracks] = useState<Track[] | null>(null)
  const [agenda, setAgenda] = useState<AgendaResponse | null>(null)
  const [activity, setActivity] = useState<ActivitySummaryResponse | null>(null)
  const [entries, setEntries] = useState<LearningEntry[]>([])
  const [loading, setLoading] = useState(true)
  const [busy, setBusy] = useState(false)
  const [message, setMessage] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [actuals, setActuals] = useState<Record<string, number>>({})

  const [showEntry, setShowEntry] = useState(false)
  const [entryTitle, setEntryTitle] = useState('')
  const [entryTrack, setEntryTrack] = useState('java')
  const [entryDescription, setEntryDescription] = useState('')
  const [entryMinutes, setEntryMinutes] = useState('30')

  const refresh = useCallback(async () => {
    const from = addDaysISO(today, -90)
    const [agendaRes, overdueRes, activityRes, entriesRes] = await Promise.all([
      api.getAgenda(viewDate).catch(() => null),
      api.listOverdueTasks(today).catch(() => null),
      api.getActivitySummary(from, today, today).catch(() => null),
      api.listLearningEntries(addDaysISO(today, -14), today).catch(() => null),
    ])
    const rememberActuals = (list: StudyTask[]) =>
      setActuals((prev) => {
        const next = { ...prev }
        for (const task of list) {
          if (next[task.id] === undefined) {
            next[task.id] = task.actual_minutes ?? task.estimate_minutes
          }
        }
        return next
      })
    if (agendaRes?.ok) {
      setAgenda(agendaRes)
      rememberActuals(agendaRes.tasks)
    }
    if (overdueRes?.ok) {
      setOverdue(overdueRes.tasks)
      rememberActuals(overdueRes.tasks)
    }
    if (activityRes?.ok) setActivity(activityRes)
    if (entriesRes?.ok) setEntries(entriesRes.entries)
  }, [today, viewDate])

  useEffect(() => {
    Promise.all([api.getTracks().catch(() => null), refresh()])
      .then(([trackRes]) => {
        setTracks(trackRes?.ok ? trackRes.tracks : null)
      })
      .finally(() => setLoading(false))
  }, [refresh])

  const hasCurriculum = (tracks?.length ?? 0) > 0

  const runAction = async (fn: () => Promise<void>) => {
    setBusy(true)
    setError(null)
    setMessage(null)
    try {
      await fn()
      await refresh()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Action failed')
    } finally {
      setBusy(false)
    }
  }

  const handleGenerate = () =>
    runAction(async () => {
      const res = await api.generateWeek(weekStartISO(today), true)
      if (res.ok) {
        const warn = res.warnings.length > 0 ? ` ${res.warnings.join(' ')}` : ''
        setMessage(`Generated ${res.tasks_created} tasks for the week.${warn}`)
      } else {
        setError(res.error ?? 'Failed to generate plan')
      }
    })

  const handleReplan = () =>
    runAction(async () => {
      const res = await api.replanBacklog(today)
      setMessage(
        res.ok
          ? `Rescheduled ${res.moved} missed task(s)${res.unplaced ? `, ${res.unplaced} still unplaced` : ''}.`
          : res.error ?? 'Failed to reschedule',
      )
    })

  const handleComplete = (task: StudyTask) =>
    runAction(async () => {
      const res = await api.updateStudyTask({
        id: task.id,
        status: 'done',
        actual_minutes: actuals[task.id] ?? task.estimate_minutes,
      })
      if (!res.ok) setError(res.error ?? 'Failed to complete task')
    })

  const handleSkip = (task: StudyTask) =>
    runAction(async () => {
      const res = await api.updateStudyTask({ id: task.id, status: 'skipped' })
      if (!res.ok) setError(res.error ?? 'Failed to skip task')
    })

  const handleUndo = (task: StudyTask) =>
    runAction(async () => {
      const res = await api.updateStudyTask({
        id: task.id,
        status: 'planned',
        actual_minutes: undefined,
        output: undefined,
      })
      if (!res.ok) setError(res.error ?? 'Failed to undo task')
    })

  const handleMoveTo = (task: StudyTask, date: string) =>
    runAction(async () => {
      const res = await api.updateStudyTask({
        id: task.id,
        scheduled_date: date,
      })
      if (!res.ok) setError(res.error ?? 'Failed to move task')
    })

  const handleMove = (task: StudyTask) =>
    handleMoveTo(task, addDaysISO(task.scheduled_date, 1))

  const handleAddEntry = () =>
    runAction(async () => {
      const res = await api.createLearningEntry({
        title: entryTitle.trim(),
        entry_date: today,
        track: entryTrack,
        description: entryDescription.trim(),
        duration_minutes: Number(entryMinutes) || 0,
      })
      if (res.ok) {
        setShowEntry(false)
        setEntryTitle('')
        setEntryDescription('')
        setEntryMinutes('30')
        setMessage('Learning entry added.')
      } else {
        setError(res.error ?? 'Failed to add entry')
      }
    })

  if (loading) return <div className="p-4 text-muted-foreground">Loading today...</div>

  const tasks = agenda?.tasks ?? []
  const doneCount = tasks.filter((t) => t.status === 'done').length
  const plannedMinutes = agenda?.planned_minutes ?? 0
  const completedMinutes = agenda?.completed_minutes ?? 0

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <h2 className="text-2xl font-bold">
            {viewDate === today ? 'What to Study Today' : 'What to Study'}
          </h2>
          <div className="mt-1 flex flex-wrap items-center gap-2 text-sm text-muted-foreground">
            <button
              onClick={() => setViewDate(addDaysISO(viewDate, -1))}
              className="rounded-md border border-border px-2 py-0.5 text-xs transition-colors hover:bg-muted"
            >
              ← Prev
            </button>
            <span>
              {formatDateHuman(viewDate)} · Week of {formatDateHuman(weekStartISO(viewDate))}
            </span>
            <button
              onClick={() => setViewDate(addDaysISO(viewDate, 1))}
              className="rounded-md border border-border px-2 py-0.5 text-xs transition-colors hover:bg-muted"
            >
              Next →
            </button>
            {viewDate !== today && (
              <button
                onClick={() => setViewDate(today)}
                className="rounded-md border border-border px-2 py-0.5 text-xs transition-colors hover:bg-muted"
              >
                Back to today
              </button>
            )}
          </div>
        </div>
        <div className="flex flex-wrap items-center gap-2">
          <button
            onClick={handleGenerate}
            disabled={!isTauri || busy}
            className="rounded-lg bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90 disabled:cursor-not-allowed disabled:opacity-50"
          >
            Generate this week's plan
          </button>
          <button
            onClick={handleReplan}
            disabled={!isTauri || busy}
            className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted disabled:cursor-not-allowed disabled:opacity-50"
          >
            Replan missed
          </button>
        </div>
      </div>

      {!isTauri && (
        <div className="rounded-lg border border-border bg-muted/50 p-3 text-sm text-muted-foreground">
          Planner actions require the desktop app. Run `npm run tauri dev`.
        </div>
      )}

      {message && (
        <div className="rounded-lg border border-green-200 bg-green-50 p-3 text-sm text-green-900 dark:border-green-900 dark:bg-green-950 dark:text-green-100">
          {message}
        </div>
      )}
      {error && (
        <div className="rounded-lg border border-destructive/50 bg-destructive/10 p-3 text-sm text-destructive">
          {error}
        </div>
      )}

      {!hasCurriculum && (
        <div className="rounded-lg border border-dashed border-border p-6 text-center">
          <p className="text-sm text-muted-foreground">
            No curriculum imported yet. Import the roadmap to generate a study plan.
          </p>
          <button
            onClick={() => onNavigate('roadmap')}
            className="mt-3 rounded-lg bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90"
          >
            Import Roadmap
          </button>
        </div>
      )}

      <div className="grid grid-cols-2 gap-3 md:grid-cols-4">
        <StatCard
          label={viewDate === today ? 'Completed today' : 'Completed'}
          value={`${doneCount}/${tasks.length}`}
        />
        <StatCard
          label="Minutes"
          value={`${completedMinutes}/${plannedMinutes}`}
          hint="done / planned"
        />
        <StatCard label="Current streak" value={`${activity?.current_streak ?? 0} d`} />
        <StatCard label="Revisions due" value={`${agenda?.due_revisions ?? 0}`} />
      </div>

      {overdue.length > 0 && (
        <section className="rounded-lg border border-amber-300 bg-amber-50/60 p-4 dark:border-amber-900 dark:bg-amber-950/30">
          <div className="mb-2 flex flex-wrap items-center justify-between gap-2">
            <h3 className="font-semibold">Missed from previous days ({overdue.length})</h3>
            <button
              onClick={handleReplan}
              disabled={!isTauri || busy}
              className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted disabled:cursor-not-allowed disabled:opacity-50"
            >
              Replan missed
            </button>
          </div>
          <ul className="divide-y divide-border/60">
            {overdue.map((task) => (
              <li
                key={task.id}
                className="flex flex-col gap-2 py-2 sm:flex-row sm:items-center sm:justify-between"
              >
                <div className="min-w-0">
                  <div className="flex flex-wrap items-center gap-2">
                    <span className="rounded bg-background px-1.5 py-0.5 text-xs text-muted-foreground">
                      {formatDateHuman(task.scheduled_date)}
                    </span>
                    <span className="rounded bg-background px-1.5 py-0.5 text-xs text-muted-foreground">
                      {trackLabel(task.track)}
                    </span>
                    <span className="truncate text-sm">{task.title}</span>
                    {task.status === 'skipped' && (
                      <span className="rounded-full bg-muted px-2 py-0.5 text-xs text-muted-foreground">
                        skipped
                      </span>
                    )}
                  </div>
                  <div className="mt-1 text-xs text-muted-foreground">
                    {task.estimate_minutes} min planned
                  </div>
                </div>
                <div className="flex flex-wrap items-center gap-2">
                  {task.status === 'planned' ? (
                    <>
                      <button
                        onClick={() => handleComplete(task)}
                        disabled={busy}
                        className="rounded-md bg-green-600 px-2 py-1 text-xs font-medium text-white hover:bg-green-700 disabled:opacity-50"
                      >
                        Done
                      </button>
                      <button
                        onClick={() => handleMoveTo(task, today)}
                        disabled={busy}
                        className="rounded-md border border-border px-2 py-1 text-xs hover:bg-muted disabled:opacity-50"
                      >
                        Move to today
                      </button>
                      <button
                        onClick={() => handleSkip(task)}
                        disabled={busy}
                        className="rounded-md border border-border px-2 py-1 text-xs hover:bg-muted disabled:opacity-50"
                      >
                        Skip
                      </button>
                    </>
                  ) : (
                    <>
                      <button
                        onClick={() => handleUndo(task)}
                        disabled={busy}
                        className="rounded-md border border-border px-2 py-1 text-xs hover:bg-muted disabled:opacity-50"
                      >
                        Undo skip
                      </button>
                      <button
                        onClick={() => handleMoveTo(task, today)}
                        disabled={busy}
                        className="rounded-md border border-border px-2 py-1 text-xs hover:bg-muted disabled:opacity-50"
                      >
                        Move to today
                      </button>
                    </>
                  )}
                </div>
              </li>
            ))}
          </ul>
        </section>
      )}

      {hasCurriculum && tasks.length === 0 ? (
        <div className="rounded-lg border border-dashed border-border p-6 text-center">
          <p className="text-sm text-muted-foreground">
            No tasks for {viewDate === today ? 'today' : 'this day'} yet. Use “Generate this
            week's plan” to build an agenda from your curriculum and available time.
          </p>
        </div>
      ) : (
        <div className="space-y-5">
          {GROUPS.map((group) => {
            const groupTasks = tasks.filter((t) => t.kind === group.kind)
            if (groupTasks.length === 0) return null
            return (
              <section key={group.kind}>
                <h3 className="mb-2 text-sm font-semibold uppercase tracking-wide text-muted-foreground">
                  {group.label}
                </h3>
                <ul className="divide-y divide-border overflow-hidden rounded-lg border border-border">
                  {groupTasks.map((task) => (
                    <li key={task.id} className="flex flex-col gap-3 p-3 sm:flex-row sm:items-center sm:justify-between">
                      <div className="min-w-0">
                        <div className="flex items-center gap-2">
                          <span className="rounded bg-muted px-1.5 py-0.5 text-xs text-muted-foreground">
                            {trackLabel(task.track)}
                          </span>
                          <span
                            className={`truncate text-sm ${
                              task.status === 'done' ? 'line-through text-muted-foreground' : ''
                            }`}
                          >
                            {task.title}
                          </span>
                        </div>
                        <div className="mt-1 text-xs text-muted-foreground">
                          {task.estimate_minutes} min planned
                          {task.status === 'done' &&
                            ` · ${task.actual_minutes ?? task.estimate_minutes} min actual`}
                          {task.status === 'skipped' && ' · skipped'}
                        </div>
                      </div>

                      {task.status === 'planned' ? (
                        <div className="flex items-center gap-2">
                          <input
                            type="number"
                            min={0}
                            value={actuals[task.id] ?? task.estimate_minutes}
                            onChange={(e) =>
                              setActuals((prev) => ({ ...prev, [task.id]: Number(e.target.value) }))
                            }
                            className="w-16 rounded-md border border-border bg-background px-2 py-1 text-sm"
                            aria-label="Actual minutes"
                          />
                          <button
                            onClick={() => handleComplete(task)}
                            disabled={busy}
                            className="rounded-md bg-green-600 px-2 py-1 text-xs font-medium text-white hover:bg-green-700 disabled:opacity-50"
                          >
                            Done
                          </button>
                          <button
                            onClick={() => handleMove(task)}
                            disabled={busy}
                            className="rounded-md border border-border px-2 py-1 text-xs hover:bg-muted disabled:opacity-50"
                          >
                            Move +1d
                          </button>
                          <button
                            onClick={() => handleSkip(task)}
                            disabled={busy}
                            className="rounded-md border border-border px-2 py-1 text-xs hover:bg-muted disabled:opacity-50"
                          >
                            Skip
                          </button>
                        </div>
                      ) : task.status === 'done' ? (
                        <div className="flex items-center gap-2">
                          <span
                            className="shrink-0 rounded-full px-2 py-0.5 text-xs font-medium bg-green-100 text-green-900 dark:bg-green-950 dark:text-green-200"
                          >
                            Done
                          </span>
                          <button
                            onClick={() => handleUndo(task)}
                            disabled={busy}
                            className="rounded-md border border-border px-2 py-1 text-xs hover:bg-muted disabled:opacity-50"
                          >
                            Undo
                          </button>
                        </div>
                      ) : (
                        <div className="flex items-center gap-2">
                          <span className="shrink-0 rounded-full bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground">
                            skipped
                          </span>
                          <button
                            onClick={() => handleUndo(task)}
                            disabled={busy}
                            className="rounded-md border border-border px-2 py-1 text-xs hover:bg-muted disabled:opacity-50"
                          >
                            Undo skip
                          </button>
                        </div>
                      )}
                    </li>
                  ))}
                </ul>
              </section>
            )
          })}
        </div>
      )}

      <section className="rounded-lg border border-border p-4">
        <div className="flex items-center justify-between">
          <h3 className="font-semibold">Additional learning</h3>
          <button
            onClick={() => setShowEntry((v) => !v)}
            disabled={!isTauri}
            className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted disabled:opacity-50"
          >
            {showEntry ? 'Cancel' : '+ Add Learning Entry'}
          </button>
        </div>

        {showEntry && (
          <form
            className="mt-3 space-y-2"
            onSubmit={(e) => {
              e.preventDefault()
              if (entryTitle.trim() && entryDescription.trim()) handleAddEntry()
            }}
          >
            <input
              value={entryTitle}
              onChange={(e) => setEntryTitle(e.target.value)}
              placeholder="Title"
              className="w-full rounded-md border border-border bg-background px-3 py-2 text-sm"
            />
            <div className="flex gap-2">
              <select
                value={entryTrack}
                onChange={(e) => setEntryTrack(e.target.value)}
                className="rounded-md border border-border bg-background px-3 py-2 text-sm"
              >
                <option value="java">Java</option>
                <option value="dsa">DSA</option>
                <option value="lld">LLD</option>
                <option value="hld">HLD</option>
                <option value="fde">FDE</option>
                <option value="other">Other</option>
              </select>
              <input
                type="number"
                min={0}
                value={entryMinutes}
                onChange={(e) => setEntryMinutes(e.target.value)}
                className="w-24 rounded-md border border-border bg-background px-3 py-2 text-sm"
                aria-label="Duration minutes"
              />
            </div>
            <textarea
              value={entryDescription}
              onChange={(e) => setEntryDescription(e.target.value)}
              placeholder="What did you learn?"
              rows={2}
              className="w-full rounded-md border border-border bg-background px-3 py-2 text-sm"
            />
            <button
              type="submit"
              disabled={busy}
              className="rounded-lg bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
            >
              Save entry
            </button>
          </form>
        )}

        {entries.length > 0 && (
          <ul className="mt-3 space-y-2">
            {entries.slice(0, 5).map((entry) => (
              <li key={entry.id} className="rounded-md border border-border/60 p-2 text-sm">
                <div className="flex items-center justify-between gap-2">
                  <span className="truncate">{entry.title}</span>
                  <span className="shrink-0 text-xs text-muted-foreground">
                    {trackLabel(entry.track)}
                    {entry.duration_minutes ? ` · ${entry.duration_minutes} min` : ''}
                  </span>
                </div>
                <p className="mt-0.5 line-clamp-2 text-xs text-muted-foreground">
                  {entry.description}
                </p>
              </li>
            ))}
          </ul>
        )}
      </section>

      <div className="flex items-center gap-2 pt-1">
        <button
          onClick={() => onNavigate('review')}
          className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted"
        >
          Weekly review
        </button>
        <button
          onClick={() => onNavigate('settings')}
          className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted"
        >
          Go to Settings
        </button>
      </div>
    </div>
  )
}

function StatCard({ label, value, hint }: { label: string; value: string; hint?: string }) {
  return (
    <div className="rounded-lg border border-border p-3">
      <div className="text-xs text-muted-foreground">{label}</div>
      <div className="mt-1 text-xl font-semibold">{value}</div>
      {hint && <div className="text-[10px] uppercase tracking-wide text-muted-foreground">{hint}</div>}
    </div>
  )
}
