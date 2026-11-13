import { useEffect, useState } from 'react'
import { api } from '@/services/api'
import type { HealthResponse } from '@/types'

type Page = 'dashboard' | 'settings'

export default function Dashboard({ onNavigate }: { onNavigate: (page: Page) => void }) {
  const [health, setHealth] = useState<HealthResponse | null>(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    api.healthCheck()
      .then((res) => {
        setHealth(res)
      })
      .catch(() => {
        setHealth(null)
      })
      .finally(() => setLoading(false))
  }, [])

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold">What to Study Today</h2>
        <p className="text-muted-foreground">
          Your daily learning agenda will appear here once the roadmap is imported.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="border rounded-lg p-4">
          <h3 className="font-semibold mb-2">Database Health</h3>
          {loading ? (
            <p className="text-sm text-muted-foreground">Checking...</p>
          ) : health ? (
            <div className="space-y-2">
              <p className="text-sm">
                Connected: <span className={`font-mono ${health.ok ? 'text-green-600 dark:text-green-400' : 'text-destructive'}`}>{String(health.health.connected)}</span>
              </p>
              <p className="text-sm">
                Version: <span className="font-mono">{health.health.version}</span>
              </p>
              <p className="text-xs text-muted-foreground truncate">{health.health.path}</p>
            </div>
          ) : (
            <p className="text-sm text-destructive">Unable to check database</p>
          )}
        </div>

        <div className="border rounded-lg p-4">
          <h3 className="font-semibold mb-2">Status</h3>
          <p className="text-sm text-muted-foreground">Phase 1 foundation</p>
          <p className="text-xs text-muted-foreground mt-2">
            Desktop shell + SQLite ready. Import roadmap next.
          </p>
        </div>

        <div className="border rounded-lg p-4">
          <h3 className="font-semibold mb-2">Next Step</h3>
          <p className="text-sm text-muted-foreground">Phase 2</p>
          <p className="text-xs text-muted-foreground mt-2">
            Curriculum data model and 26-week roadmap import
          </p>
        </div>
      </div>

      <div className="border rounded-lg p-4 bg-muted/50">
        <h3 className="font-semibold mb-2">Empty State</h3>
        <p className="text-sm text-muted-foreground">
          No curriculum data imported yet. Place your roadmap at
          <code className="font-mono text-xs">docs/roadmap/six_month_engineering_roadmap.md</code>
          and proceed to Phase 2 for import.
        </p>
      </div>

      <div className="mt-6 flex items-center gap-2">
        <button
          onClick={() => onNavigate('settings')}
          className="text-sm border border-border rounded-lg px-3 py-1.5 hover:bg-muted transition-colors"
        >
          Go to Settings
        </button>
      </div>
    </div>
  )
}
