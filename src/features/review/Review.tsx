import { useEffect, useState, useCallback } from 'react'
import { api } from '@/services/api'
import type { WeekReviewResponse, ActivitySummaryResponse } from '@/types'
import {
  todayISO,
  weekStartISO,
  addDaysISO,
  formatDateHuman,
  parseISODate,
  toISODate,
} from '@/lib/dates'

type Page = 'dashboard' | 'settings' | 'roadmap' | 'review' | 'dsa'

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

function trackLabel(track: string): string {
  return TRACK_LABEL[track] ?? track
}

export default function Review({ onNavigate }: { onNavigate: (page: Page) => void }) {
  const [today] = useState(todayISO())
  const [weekStart, setWeekStart] = useState(weekStartISO(today))
  const [review, setReview] = useState<WeekReviewResponse | null>(null)
  const [activity, setActivity] = useState<ActivitySummaryResponse | null>(null)
  const [note, setNote] = useState('')
  const [reviewed, setReviewed] = useState(false)
  const [loading, setLoading] = useState(true)
  const [busy, setBusy] = useState(false)
  const [message, setMessage] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)

  const load = useCallback(async () => {
    setLoading(true)
    const [reviewRes, activityRes] = await Promise.all([
      api.getWeekReview(weekStart).catch(() => null),
      api
        .getActivitySummary(addDaysISO(today, -84), today, today)
        .catch(() => null),
    ])
    if (reviewRes?.ok) {
      setReview(reviewRes)
      setNote(reviewRes.goal?.note ?? '')
      setReviewed(reviewRes.goal?.reviewed ?? false)
    } else {
      setReview(reviewRes)
    }
    if (activityRes?.ok) setActivity(activityRes)
    setLoading(false)
  }, [weekStart, today])

  useEffect(() => {
    load()
  }, [load])

  const handleSave = async () => {
    setBusy(true)
    setError(null)
    setMessage(null)
    try {
      const res = await api.saveWeeklyGoal({
        week_start_date: weekStart,
        note,
        reviewed,
      })
      if (res.ok) {
        setMessage('Weekly review saved.')
        await load()
      } else {
        setError(res.error ?? 'Failed to save review')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to save review')
    } finally {
      setBusy(false)
    }
  }

  const weekEnd = addDaysISO(weekStart, 6)
  const completionPct =
    review && review.planned > 0 ? Math.round((review.completed / review.planned) * 100) : 0

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <h2 className="text-2xl font-bold">Weekly Review</h2>
          <p className="text-muted-foreground">
            {formatDateHuman(weekStart)} – {formatDateHuman(weekEnd)}
          </p>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={() => setWeekStart(addDaysISO(weekStart, -7))}
            className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted"
          >
            Previous
          </button>
          <button
            onClick={() => setWeekStart(weekStartISO(today))}
            className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted"
          >
            This week
          </button>
          <button
            onClick={() => setWeekStart(addDaysISO(weekStart, 7))}
            className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted"
          >
            Next
          </button>
        </div>
      </div>

      {!isTauri && (
        <div className="rounded-lg border border-border bg-muted/50 p-3 text-sm text-muted-foreground">
          Weekly review requires the desktop app. Run `npm run tauri dev`.
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

      {loading ? (
        <p className="text-muted-foreground">Loading review...</p>
      ) : (
        <>
          <div className="grid grid-cols-2 gap-3 md:grid-cols-4">
            <Stat label="Planned" value={`${review?.planned ?? 0}`} />
            <Stat label="Completed" value={`${review?.completed ?? 0}`} />
            <Stat label="Completion" value={`${completionPct}%`} />
            <Stat label="Skipped" value={`${review?.skipped ?? 0}`} />
          </div>

          <div className="grid gap-4 md:grid-cols-2">
            <section className="rounded-lg border border-border p-4">
              <h3 className="mb-2 font-semibold">By track</h3>
              {review && Object.keys(review.by_track).length > 0 ? (
                <ul className="space-y-1 text-sm">
                  {Object.entries(review.by_track).map(([track, tally]) => (
                    <li key={track} className="flex items-center justify-between">
                      <span>{trackLabel(track)}</span>
                      <span className="text-muted-foreground">
                        {tally.completed}/{tally.planned}
                      </span>
                    </li>
                  ))}
                </ul>
              ) : (
                <p className="text-sm text-muted-foreground">No tasks planned this week.</p>
              )}
              <div className="mt-3 text-sm text-muted-foreground">
                Minutes: {review?.completed_minutes ?? 0}/{review?.planned_minutes ?? 0}
              </div>
            </section>

            <section className="rounded-lg border border-border p-4">
              <h3 className="mb-2 font-semibold">Weekly goal</h3>
              <textarea
                value={note}
                onChange={(e) => setNote(e.target.value)}
                rows={3}
                placeholder="What did you learn? What will you adjust next week?"
                className="w-full rounded-md border border-border bg-background px-3 py-2 text-sm"
                disabled={!isTauri}
              />
              <label className="mt-2 flex items-center gap-2 text-sm">
                <input
                  type="checkbox"
                  checked={reviewed}
                  onChange={(e) => setReviewed(e.target.checked)}
                  disabled={!isTauri}
                />
                Mark week as reviewed
              </label>
              <button
                onClick={handleSave}
                disabled={!isTauri || busy}
                className="mt-3 rounded-lg bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
              >
                Save review
              </button>
            </section>
          </div>

          <section className="rounded-lg border border-border p-4">
            <h3 className="mb-2 font-semibold">Tasks</h3>
            {review && review.tasks.length > 0 ? (
              <div className="space-y-3">
                {Object.entries(
                  review.tasks.reduce<Record<string, typeof review.tasks>>((acc, task) => {
                    ;(acc[task.scheduled_date] ??= []).push(task)
                    return acc
                  }, {}),
                ).map(([date, dayTasks]) => (
                  <div key={date}>
                    <div className="mb-1 text-xs font-semibold uppercase tracking-wide text-muted-foreground">
                      {formatDateHuman(date)}
                    </div>
                    <ul className="divide-y divide-border/60 overflow-hidden rounded-md border border-border/60">
                      {dayTasks.map((task) => {
                        const missed = task.status === 'planned' && date < today
                        return (
                          <li
                            key={task.id}
                            className="flex items-center justify-between gap-2 p-2 text-sm"
                          >
                            <div className="flex min-w-0 items-center gap-2">
                              <span className="rounded bg-muted px-1.5 py-0.5 text-xs text-muted-foreground">
                                {trackLabel(task.track)}
                              </span>
                              <span
                                className={`truncate ${
                                  task.status === 'done' ? 'text-muted-foreground' : ''
                                }`}
                              >
                                {task.title}
                              </span>
                            </div>
                            <span
                              className={`shrink-0 rounded-full px-2 py-0.5 text-xs font-medium ${statusClass(
                                missed ? 'missed' : task.status,
                              )}`}
                            >
                              {missed ? 'missed' : task.status}
                            </span>
                          </li>
                        )
                      })}
                    </ul>
                  </div>
                ))}
              </div>
            ) : (
              <p className="text-sm text-muted-foreground">No tasks planned this week.</p>
            )}
          </section>

          <section className="rounded-lg border border-border p-4">
            <div className="mb-3 flex items-center justify-between">
              <h3 className="font-semibold">Activity</h3>
              <div className="text-xs text-muted-foreground">
                Streak {activity?.current_streak ?? 0} d · Best {activity?.longest_streak ?? 0} d ·{' '}
                {activity?.total_minutes ?? 0} min
              </div>
            </div>
            <Heatmap activity={activity} today={today} />
          </section>
        </>
      )}

      <div className="flex items-center gap-2 pt-1">
        <button
          onClick={() => onNavigate('dashboard')}
          className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted"
        >
          Back to Today
        </button>
        <button
          onClick={() => onNavigate('settings')}
          className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted"
        >
          Settings
        </button>
      </div>
    </div>
  )
}

function Stat({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-lg border border-border p-3">
      <div className="text-xs text-muted-foreground">{label}</div>
      <div className="mt-1 text-xl font-semibold">{value}</div>
    </div>
  )
}

function statusClass(status: string): string {
  if (status === 'done') return 'bg-green-100 text-green-900 dark:bg-green-950 dark:text-green-200'
  if (status === 'skipped') return 'bg-muted text-muted-foreground'
  return 'bg-amber-100 text-amber-900 dark:bg-amber-950 dark:text-amber-200'
}

function heatColor(minutes: number, min: number): string {
  if (minutes <= 0) return 'bg-muted'
  if (minutes < min) return 'bg-green-100 dark:bg-green-900/40'
  if (minutes < min * 2) return 'bg-green-200 dark:bg-green-800/60'
  if (minutes < min * 4) return 'bg-green-400 dark:bg-green-700'
  return 'bg-green-600 dark:bg-green-500'
}

function Heatmap({
  activity,
  today,
}: {
  activity: ActivitySummaryResponse | null
  today: string
}) {
  const minutesByDate = new Map<string, number>()
  for (const day of activity?.days ?? []) minutesByDate.set(day.date, day.minutes)
  const min = Math.max(1, activity?.min_meaningful_minutes ?? 15)

  // 12 weeks ending this week, Monday-aligned.
  const lastMonday = weekStartISO(today)
  const firstMonday = addDaysISO(lastMonday, -77)
  const weeks: string[][] = []
  for (let w = 0; w < 12; w++) {
    const week: string[] = []
    for (let d = 0; d < 7; d++) {
      week.push(addDaysISO(firstMonday, w * 7 + d))
    }
    weeks.push(week)
  }

  return (
    <div className="overflow-x-auto">
      <div className="flex gap-1">
        {weeks.map((week, wi) => (
          <div key={wi} className="flex flex-col gap-1">
            {week.map((date) => {
              const isFuture = parseISODate(date) > parseISODate(today)
              const minutes = minutesByDate.get(date) ?? 0
              return (
                <div
                  key={date}
                  title={`${toISODate(parseISODate(date))}: ${minutes} min`}
                  className={`h-3 w-3 rounded-sm ${
                    isFuture ? 'bg-transparent border border-border/40' : heatColor(minutes, min)
                  }`}
                />
              )
            })}
          </div>
        ))}
      </div>
      <div className="mt-2 flex items-center gap-2 text-[10px] text-muted-foreground">
        <span>Less</span>
        <span className="h-3 w-3 rounded-sm bg-muted" />
        <span className="h-3 w-3 rounded-sm bg-green-200 dark:bg-green-800/60" />
        <span className="h-3 w-3 rounded-sm bg-green-400 dark:bg-green-700" />
        <span className="h-3 w-3 rounded-sm bg-green-600 dark:bg-green-500" />
        <span>More</span>
      </div>
    </div>
  )
}
