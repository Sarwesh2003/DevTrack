import { useSettings } from '@/services/useSettings'

type Page = 'dashboard' | 'settings'

export default function Settings({ onNavigate }: { onNavigate: (page: Page) => void }) {
  const { settings, loading, error, save } = useSettings()

  if (loading) return <div className="p-4">Loading settings...</div>

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold">Settings</h2>
        <p className="text-muted-foreground">Configure your local learning workspace.</p>
      </div>

      {error && (
        <div className="p-3 rounded-md bg-destructive/10 border border-destructive/50 text-destructive text-sm">
          {error}
        </div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="border rounded-lg p-4 space-y-4">
          <h3 className="font-semibold">Theme</h3>
          <div className="flex items-center gap-2">
            <label className="text-sm cursor-pointer group">
              <input
                type="radio"
                name="theme"
                value="dark"
                defaultChecked={settings.theme !== 'light'}
                className="accent-foreground"
                onChange={() => save({ ...settings, theme: 'dark' })}
              />
              <span className="group-hover:text-muted-foreground transition-colors">Dark</span>
            </label>
            <label className="text-sm cursor-pointer group">
              <input
                type="radio"
                name="theme"
                value="light"
                defaultChecked={settings.theme === 'light'}
                className="accent-foreground"
                onChange={() => save({ ...settings, theme: 'light' })}
              />
              <span className="group-hover:text-muted-foreground transition-colors">Light</span>
            </label>
          </div>
        </div>

        <div className="border rounded-lg p-4 space-y-4">
          <h3 className="font-semibold">Weekly Capacity</h3>
          <div className="flex items-center gap-2">
            <span className="text-sm">{settings.weekly_capacity_hours ?? 7.5}</span>
            <span className="text-xs text-muted-foreground">hours/week</span>
          </div>
          <p className="text-xs text-muted-foreground">
            Adjust your available study time per week.
          </p>
        </div>

        <div className="border rounded-lg p-4 space-y-4">
          <h3 className="font-semibold">Timezone</h3>
          <p className="text-sm text-muted-foreground">
            {settings.timezone ?? 'Asia/Kolkata'}
          </p>
          <p className="text-xs text-muted-foreground">
            Used for date-based scheduling and revision intervals.
          </p>
        </div>

        <div className="border rounded-lg p-4 space-y-4">
          <h3 className="font-semibold">Data Directory</h3>
          <p className="text-sm text-muted-foreground">
            App-managed local directory for SQLite and content files.
          </p>
          <p className="text-xs font-mono text-muted-foreground truncate">
            ~/Documents/DevTrack/
          </p>
        </div>

        <div className="border rounded-lg p-4 space-y-4">
          <h3 className="font-semibold">DSA Cursor</h3>
          <p className="text-sm text-muted-foreground">
            {settings.dsa_cursor_category ?? 'Tries'}
          </p>
          <p className="text-xs text-muted-foreground">
            New DSA learning starts at Tries. Categories through Trees are revision-only.
          </p>
        </div>

        <div className="border rounded-lg p-4 space-y-4">
          <h3 className="font-semibold">LLD Cursor</h3>
          <p className="text-sm text-muted-foreground">
            {settings.lld_cursor_topic ?? 'Bridge Pattern'}
          </p>
          <p className="text-xs text-muted-foreground">
            New LLD learning starts at Bridge Pattern. Content through Composite is revision-only.
          </p>
        </div>
      </div>

      <div className="mt-6 flex items-center gap-2">
        <button
          onClick={() => onNavigate('dashboard')}
          className="text-sm border border-border rounded-lg px-3 py-1.5 hover:bg-muted transition-colors"
        >
          Go to Dashboard
        </button>
      </div>
    </div>
  )
}
