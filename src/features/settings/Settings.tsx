import { useEffect, useState } from 'react'
import { api } from '@/services/api'
import { useSettings } from '@/services/useSettings'
import { WEEKDAY_LABELS } from '@/lib/dates'

type Page = 'dashboard' | 'settings' | 'roadmap' | 'review' | 'dsa' | 'workspace' | 'projects' | 'analytics' | 'search'
type Theme = 'dark' | 'light' | 'system'

const TRACK_OPTIONS: { handle: string; label: string }[] = [
  { handle: 'dsa', label: 'DSA' },
  { handle: 'java', label: 'Java' },
  { handle: 'lld', label: 'LLD' },
  { handle: 'hld', label: 'HLD' },
  { handle: 'fde', label: 'FDE' },
  { handle: 'review', label: 'Review' },
]

const DEFAULT_CADENCE: Record<string, string[]> = {
  '1': ['dsa'],
  '2': ['java'],
  '3': ['dsa'],
  '4': ['lld'],
  '5': ['dsa'],
  '6': ['fde'],
  '7': ['review'],
}

export default function Settings({
  onNavigate,
  theme,
  setTheme,
}: {
  onNavigate: (page: Page) => void
  theme: Theme
  setTheme: (t: Theme) => void
}) {
  const { settings, loading, error, save } = useSettings()

  const [studyDays, setStudyDays] = useState<number[]>([1, 2, 3, 4, 5, 6])
  const [capacityHours, setCapacityHours] = useState('7.5')
  const [minMeaningful, setMinMeaningful] = useState('15')
  const [cadence, setCadence] = useState<Record<string, string[]>>(DEFAULT_CADENCE)
  const [saved, setSaved] = useState(false)
  const [resetConfirm, setResetConfirm] = useState(false)
  const [resetError, setResetError] = useState<string | null>(null)

  useEffect(() => {
    setStudyDays(settings.study_days ?? [1, 2, 3, 4, 5, 6])
    setCapacityHours(String(settings.weekly_capacity_hours ?? 7.5))
    setMinMeaningful(String(settings.min_meaningful_minutes ?? 15))
    setCadence(settings.cadence ?? DEFAULT_CADENCE)
  }, [settings])

  const toggleDay = (iso: number) => {
    setStudyDays((prev) =>
      prev.includes(iso) ? prev.filter((d) => d !== iso) : [...prev, iso].sort((a, b) => a - b),
    )
  }

  const toggleCadenceTrack = (iso: number, handle: string) => {
    const key = String(iso)
    setCadence((prev) => {
      const current = prev[key] ?? []
      const next = current.includes(handle)
        ? current.filter((h) => h !== handle)
        : [...current, handle]
      return { ...prev, [key]: next }
    })
  }

  const handleSave = async () => {
    setSaved(false)
    const result = await save({
      ...settings,
      weekly_capacity_hours: Number(capacityHours) || 0,
      study_days: studyDays,
      min_meaningful_minutes: Number(minMeaningful) || 0,
      cadence,
    })
    setSaved(result)
  }

  const handleReset = async () => {
    setResetConfirm(false)
    setResetError(null)
    const result = await api.resetAllData()
    if (result.ok) {
      window.location.reload()
    } else {
      setResetError(result.error ?? 'Reset failed')
    }
  }

  if (loading) return <div className="p-4">Loading settings...</div>

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold">Settings</h2>
        <p className="text-muted-foreground">Configure your local learning workspace.</p>
      </div>

      {(error || resetError) && (
        <div className="rounded-md border border-destructive/50 bg-destructive/10 p-3 text-sm text-destructive">
          {error || resetError}
        </div>
      )}

      <div className="grid grid-cols-1 gap-6 md:grid-cols-2">
        <div className="space-y-4 rounded-lg border border-border p-4">
          <h3 className="font-semibold">Theme</h3>
          <div className="flex items-center gap-4">
            <label className="group flex cursor-pointer items-center gap-2 text-sm">
              <input
                type="radio"
                name="theme"
                value="dark"
                checked={theme === 'dark' || theme === 'system'}
                onChange={() => {
                  setTheme('dark')
                  save({ ...settings, theme: 'dark' })
                }}
              />
              <span className="transition-colors group-hover:text-muted-foreground">Dark</span>
            </label>
            <label className="group flex cursor-pointer items-center gap-2 text-sm">
              <input
                type="radio"
                name="theme"
                value="light"
                checked={theme === 'light'}
                onChange={() => {
                  setTheme('light')
                  save({ ...settings, theme: 'light' })
                }}
              />
              <span className="transition-colors group-hover:text-muted-foreground">Light</span>
            </label>
          </div>
        </div>

        <div className="space-y-3 rounded-lg border border-border p-4">
          <h3 className="font-semibold">Weekly capacity</h3>
          <div className="flex items-center gap-2">
            <input
              type="number"
              min={0}
              step={0.5}
              value={capacityHours}
              onChange={(e) => setCapacityHours(e.target.value)}
              className="w-24 rounded-md border border-border bg-background px-3 py-1.5 text-sm"
            />
            <span className="text-xs text-muted-foreground">hours / week</span>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-sm text-muted-foreground">Minimum meaningful activity</span>
            <input
              type="number"
              min={0}
              value={minMeaningful}
              onChange={(e) => setMinMeaningful(e.target.value)}
              className="w-20 rounded-md border border-border bg-background px-3 py-1.5 text-sm"
            />
            <span className="text-xs text-muted-foreground">min</span>
          </div>
        </div>

        <div className="space-y-3 rounded-lg border border-border p-4 md:col-span-2">
          <h3 className="font-semibold">Study days</h3>
          <p className="text-xs text-muted-foreground">
            Unchecked days are treated as rest / unavailable and do not count as missed.
          </p>
          <div className="flex flex-wrap gap-2">
            {[1, 2, 3, 4, 5, 6, 7].map((iso) => (
              <button
                key={iso}
                type="button"
                onClick={() => toggleDay(iso)}
                className={`rounded-full px-3 py-1 text-sm transition-colors ${
                  studyDays.includes(iso)
                    ? 'bg-primary text-primary-foreground'
                    : 'border border-border text-muted-foreground hover:bg-muted'
                }`}
              >
                {WEEKDAY_LABELS[iso]}
              </button>
            ))}
          </div>
        </div>

        <div className="space-y-3 rounded-lg border border-border p-4 md:col-span-2">
          <h3 className="font-semibold">Weekly cadence</h3>
          <p className="text-xs text-muted-foreground">
            Choose which tracks are scheduled on each day. Days without a track fall back to the
            default cadence.
          </p>
          <div className="space-y-2">
            {[1, 2, 3, 4, 5, 6, 7].map((iso) => (
              <div key={iso} className="flex flex-wrap items-center gap-3">
                <span className="w-10 text-sm font-medium text-muted-foreground">
                  {WEEKDAY_LABELS[iso]}
                </span>
                {TRACK_OPTIONS.map((opt) => {
                  const active = (cadence[String(iso)] ?? []).includes(opt.handle)
                  return (
                    <label key={opt.handle} className="flex items-center gap-1 text-sm">
                      <input
                        type="checkbox"
                        checked={active}
                        onChange={() => toggleCadenceTrack(iso, opt.handle)}
                      />
                      {opt.label}
                    </label>
                  )
                })}
              </div>
            ))}
          </div>
        </div>

        <div className="space-y-2 rounded-lg border border-border p-4">
          <h3 className="font-semibold">Timezone</h3>
          <p className="text-sm text-muted-foreground">{settings.timezone ?? 'Asia/Kolkata'}</p>
          <p className="text-xs text-muted-foreground">
            Used for date-based scheduling and revision intervals.
          </p>
        </div>

        <div className="space-y-2 rounded-lg border border-border p-4">
          <h3 className="font-semibold">Data directory</h3>
          <p className="text-xs text-muted-foreground">
            App-managed local directory for SQLite and content files.
          </p>
          <p className="truncate font-mono text-xs text-muted-foreground">~/DevTrack/</p>
        </div>
      </div>

      <div className="flex items-center gap-2">
        <button
          onClick={handleSave}
          className="rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90"
        >
          Save planner settings
        </button>
        {saved && <span className="text-sm text-green-600 dark:text-green-400">Saved.</span>}
      </div>

      <div className="flex items-center gap-2">
        <button
          onClick={() => onNavigate('dashboard')}
          className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted"
        >
          Go to Today
        </button>
        <button
          onClick={() => onNavigate('review')}
          className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted"
        >
          Weekly Review
        </button>
      </div>

      {resetConfirm ? (
        <div className="rounded-lg border border-destructive bg-destructive/10 p-4">
          <h3 className="font-semibold text-destructive">Reset all data</h3>
          <p className="text-sm text-destructive/80">
            This will permanently delete your SQLite database and all content files
            (Markdown, Java, Mermaid, JSON). This cannot be undone.
          </p>
          <div className="flex items-center gap-2 mt-3">
            <button
              onClick={handleReset}
              className="rounded-lg bg-destructive px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-destructive/90"
            >
              Delete everything
            </button>
            <button
              onClick={() => setResetConfirm(false)}
              className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted"
            >
              Cancel
            </button>
          </div>
        </div>
      ) : (
        <div className="flex items-center gap-2">
          <button
            onClick={() => setResetConfirm(true)}
            className="rounded-lg border border-destructive bg-destructive/10 px-4 py-2 text-sm font-medium text-destructive transition-colors hover:bg-destructive/20"
          >
            Reset all data
          </button>
        </div>
      )}
    </div>
  )
}
