import { useEffect, useState } from 'react'
import { api } from '@/services/api'
import type { AnalyticsResponse } from '@/types'

type Page = 'dashboard' | 'settings' | 'roadmap' | 'review' | 'dsa' | 'workspace' | 'projects' | 'analytics' | 'search'

export default function Analytics(_props: { onNavigate: (page: Page) => void }) {
  const [analytics, setAnalytics] = useState<AnalyticsResponse | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    api
      .getAnalytics()
      .then((res) => {
        if (res.ok) setAnalytics(res)
        else setError(res.error ?? 'Failed to load analytics')
      })
      .catch(() => setError('Failed to load analytics'))
      .finally(() => setLoading(false))
  }, [])

  if (loading) return <div className="p-4 text-muted-foreground">Loading analytics...</div>

  const pct = (value: number, total: number) => (total > 0 ? Math.round((value / total) * 100) : 0)

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <h2 className="text-2xl font-bold">Analytics</h2>
          <p className="text-muted-foreground">
            All figures are derived from your local SQLite records — nothing is inferred from
            app launches alone.
          </p>
        </div>
      </div>

      {error && (
        <div className="rounded-lg border border-destructive/50 bg-destructive/10 p-3 text-sm text-destructive">
          {error}
        </div>
      )}

      {!analytics && (
        <div className="rounded-lg border border-border bg-muted/50 p-4 text-center text-sm text-muted-foreground">
          No analytics available.
        </div>
      )}

      {analytics && (
        <>
          <div className="grid gap-4 sm:grid-cols-3 lg:grid-cols-4">
            <div className="rounded-lg border border-border p-4">
              <div className="text-xs text-muted-foreground">Tracks</div>
              <div className="text-2xl font-bold">{analytics.track_progress.length}</div>
            </div>
            <div className="rounded-lg border border-border p-4">
              <div className="text-xs text-muted-foreground">Planned minutes</div>
              <div className="text-2xl font-bold">{analytics.planned_vs_actual.planned_minutes}</div>
            </div>
            <div className="rounded-lg border border-border p-4">
              <div className="text-xs text-muted-foreground">Actual minutes</div>
              <div className="text-2xl font-bold">{analytics.planned_vs_actual.actual_minutes}</div>
            </div>
            <div className="rounded-lg border border-border p-4">
              <div className="text-xs text-muted-foreground">Tasks completed</div>
              <div className="text-2xl font-bold">{analytics.planned_vs_actual.completed_count}</div>
            </div>
            <div className="rounded-lg border border-border p-4">
              <div className="text-xs text-muted-foreground">Revision items</div>
              <div className="text-2xl font-bold">{analytics.revision.total_items}</div>
            </div>
            <div className="rounded-lg border border-border p-4">
              <div className="text-xs text-muted-foreground">Revisions due</div>
              <div className="text-2xl font-bold">{analytics.revision.due_count}</div>
            </div>
            <div className="rounded-lg border border-border p-4">
              <div className="text-xs text-muted-foreground">Overdue revisions</div>
              <div className="text-2xl font-bold text-destructive">{analytics.revision.overdue_count}</div>
            </div>
            <div className="rounded-lg border border-border p-4">
              <div className="text-xs text-muted-foreground">Problem attempts</div>
              <div className="text-2xl font-bold">{analytics.attempts.total}</div>
            </div>
            <div className="rounded-lg border border-border p-4">
              <div className="text-xs text-muted-foreground">Active days (30d)</div>
              <div className="text-2xl font-bold">{analytics.activity.active_days_last30}</div>
            </div>
          </div>

          <div className="grid gap-6">
            <section className="rounded-lg border border-border p-4 lg:col-span-2">
              <h3 className="mb-4 font-semibold">Track progress</h3>
              <div className="overflow-x-auto">
                <table className="w-full text-left text-sm">
                  <thead>
                    <tr className="border-b bg-muted/50">
                      <th className="px-4 py-2 font-medium">Track</th>
                      <th className="px-4 py-2 font-medium">Total</th>
                      <th className="px-4 py-2 font-medium">Done</th>
                      <th className="px-4 py-2 font-medium">In progress</th>
                      <th className="px-4 py-2 font-medium">Not started</th>
                      <th className="px-4 py-2 font-medium">Completion</th>
                      <th className="px-4 py-2 font-medium">Avg confidence</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y">
                    {analytics.track_progress.map((tp) => {
                      const total = tp.total_topics
                      const done = tp.completed
                      const completion = pct(done, total)
                      return (
                        <tr key={tp.track} className="hover:bg-muted/30">
                          <td className="px-4 py-3 font-medium">{tp.track}</td>
                          <td className="px-4 py-3 text-muted-foreground">{total}</td>
                          <td className="px-4 py-3">{done}</td>
                          <td className="px-4 py-3">{tp.in_progress}</td>
                          <td className="px-4 py-3">{tp.not_started}</td>
                          <td className="px-4 py-3">
                            <div className="flex items-center gap-2">
                              <span className="text-sm font-medium">{completion}%</span>
                              <div className="flex-1 h-2 rounded-full border border-border bg-muted">
                                <div
                                  className="h-full bg-primary rounded-full"
                                  style={{ width: `${Math.min(completion, 100)}%` }}
                                ></div>
                              </div>
                            </div>
                          </td>
                          <td className="px-4 py-3 text-sm text-muted-foreground">
                            {tp.avg_confidence == null ? '—' : tp.avg_confidence.toFixed(1)}
                          </td>
                        </tr>
                      )
                    })}
                    {analytics.track_progress.length === 0 && (
                      <tr>
                        <td colSpan={7} className="px-4 py-6 text-center text-sm text-muted-foreground">
                          No topic data yet.
                        </td>
                      </tr>
                    )}
                  </tbody>
                </table>
              </div>
            </section>

            <section className="rounded-lg border border-border p-4 lg:col-span-2">
              <h3 className="mb-4 font-semibold">Planned vs actual time</h3>
              <div className="flex flex-col gap-3">
                <div className="flex items-center justify-between">
                  <span className="text-sm">Planned</span>
                  <span className="text-sm font-medium text-muted-foreground">{analytics.planned_vs_actual.planned_minutes} min</span>
                </div>
                <div className="h-2 rounded-full border border-border bg-muted">
                  <div
                    className="h-full bg-blue-600 rounded-full"
                    style={{ width: `${Math.min(pct(analytics.planned_vs_actual.planned_minutes, 1000), 100)}%` }}
                  ></div>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-sm">Actual</span>
                  <span className="text-sm font-medium text-muted-foreground">{analytics.planned_vs_actual.actual_minutes} min</span>
                </div>
                <div className="h-2 rounded-full border border-border bg-muted">
                  <div
                    className="h-full bg-emerald-600 rounded-full"
                    style={{ width: `${Math.min(pct(analytics.planned_vs_actual.actual_minutes, 1000), 100)}%` }}
                  ></div>
                </div>
                <div className="pt-2 text-xs text-muted-foreground">
                  {analytics.planned_vs_actual.task_count} tasks · {analytics.planned_vs_actual.completed_count} completed
                </div>
              </div>
            </section>

            <section className="rounded-lg border border-border p-4 lg:col-span-2">
              <h3 className="mb-4 font-semibold">Revision</h3>
              <div className="grid gap-3 sm:grid-cols-2 text-sm">
                <div className="p-3 rounded border border-border bg-muted/40">
                  <div className="text-xs text-muted-foreground">Total items</div>
                  <div className="text-lg font-bold">{analytics.revision.total_items}</div>
                </div>
                <div className="p-3 rounded border border-border bg-muted/40">
                  <div className="text-xs text-muted-foreground">Due today</div>
                  <div className="text-lg font-bold">{analytics.revision.due_count}</div>
                </div>
                <div className="p-3 rounded border border-border bg-destructive/10">
                  <div className="text-xs text-destructive">Overdue</div>
                  <div className="text-lg font-bold text-destructive">{analytics.revision.overdue_count}</div>
                </div>
                <div className="p-3 rounded border border-border bg-muted/40">
                  <div className="text-xs text-muted-foreground">Avg confidence</div>
                  <div className="text-lg font-bold">
                    {analytics.revision.avg_confidence == null ? '—' : analytics.revision.avg_confidence.toFixed(1)}
                  </div>
                </div>
              </div>
            </section>

            <section className="rounded-lg border border-border p-4 lg:col-span-2">
              <h3 className="mb-4 font-semibold">Problem attempts</h3>
              <div className="grid gap-3 sm:grid-cols-2 text-sm">
                <div className="p-3 rounded border border-border bg-muted/40">
                  <div className="text-xs text-muted-foreground">Total</div>
                  <div className="text-lg font-bold">{analytics.attempts.total}</div>
                </div>
                <div className="p-3 rounded border border-border bg-muted/40">
                  <div className="text-xs text-muted-foreground">Solved</div>
                  <div className="text-lg font-bold">{analytics.attempts.solved}</div>
                </div>
                <div className="p-3 rounded border border-border bg-muted/40">
                  <div className="text-xs text-muted-foreground">Avg duration</div>
                  <div className="text-lg font-bold">
                    {analytics.attempts.avg_duration_minutes == null ? '—' : analytics.attempts.avg_duration_minutes.toFixed(0) + ' min'}
                  </div>
                </div>
              </div>
            </section>

            <section className="rounded-lg border border-border p-4 lg:col-span-2">
              <h3 className="mb-4 font-semibold">Activity (last 30 days)</h3>
              <div className="grid gap-3 sm:grid-cols-2 text-sm">
                <div className="p-3 rounded border border-border bg-muted/40">
                  <div className="text-xs text-muted-foreground">Events</div>
                  <div className="text-lg font-bold">{analytics.activity.total_events}</div>
                </div>
                <div className="p-3 rounded border border-border bg-muted/40">
                  <div className="text-xs text-muted-foreground">Active days</div>
                  <div className="text-lg font-bold">{analytics.activity.active_days_last30}</div>
                </div>
              </div>
            </section>
          </div>
        </>
      )}
    </div>
  )
}
