import { useEffect, useState } from 'react'
import { api } from '@/services/api'
import type { HealthResponse, Track } from '@/types'

type Page = 'dashboard' | 'settings' | 'roadmap'

export default function Dashboard({ onNavigate }: { onNavigate: (page: Page) => void }) {
  const [health, setHealth] = useState<HealthResponse | null>(null)
  const [tracks, setTracks] = useState<Track[] | null>(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    Promise.all([
      api.healthCheck().catch(() => null),
      api.getTracks().catch(() => null),
    ])
      .then(([healthRes, tracksRes]) => {
        setHealth(healthRes)
        setTracks(tracksRes?.ok ? tracksRes.tracks : null)
      })
      .finally(() => setLoading(false))
  }, [])

  const hasCurriculum = (tracks?.length ?? 0) > 0

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold">What to Study Today</h2>
        <p className="text-muted-foreground">
          {hasCurriculum
            ? 'Curriculum imported. The daily agenda and scheduling arrive in Phase 3.'
            : 'Your daily learning agenda will appear here once the roadmap is imported.'}
        </p>
      </div>

      <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
        <div className="rounded-lg border border-border p-4">
          <h3 className="mb-2 font-semibold">Database Health</h3>
          {loading ? (
            <p className="text-sm text-muted-foreground">Checking...</p>
          ) : health ? (
            <div className="space-y-2">
              <p className="text-sm">
                Connected:{' '}
                <span
                  className={`font-mono ${
                    health.ok ? 'text-green-600 dark:text-green-400' : 'text-destructive'
                  }`}
                >
                  {String(health.health.connected)}
                </span>
              </p>
              <p className="text-sm">
                Version: <span className="font-mono">{health.health.version}</span>
              </p>
              <p className="truncate text-xs text-muted-foreground">{health.health.path}</p>
            </div>
          ) : (
            <p className="text-sm text-destructive">Unable to check database</p>
          )}
        </div>

        <div className="rounded-lg border border-border p-4">
          <h3 className="mb-2 font-semibold">Curriculum</h3>
          {loading ? (
            <p className="text-sm text-muted-foreground">Loading...</p>
          ) : hasCurriculum ? (
            <div className="space-y-1">
              <p className="text-sm">
                <span className="font-mono text-lg">{tracks!.length}</span> tracks imported
              </p>
              <p className="text-xs text-muted-foreground">
                Java, DSA, LLD, HLD and FDE curriculum loaded.
              </p>
            </div>
          ) : (
            <div className="space-y-2">
              <p className="text-sm text-muted-foreground">Not imported yet</p>
              <button
                onClick={() => onNavigate('roadmap')}
                className="rounded-lg bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90"
              >
                Import Roadmap
              </button>
            </div>
          )}
        </div>

        <div className="rounded-lg border border-border p-4">
          <h3 className="mb-2 font-semibold">Next Step</h3>
          <p className="text-sm text-muted-foreground">Phase 3</p>
          <p className="mt-2 text-xs text-muted-foreground">
            Daily planner, weekly goals, revision scheduling and activity tracking.
          </p>
        </div>
      </div>

      <div className="flex items-center gap-2 pt-2">
        <button
          onClick={() => onNavigate('roadmap')}
          className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted"
        >
          View Roadmap
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
