import { useState, useEffect, useRef } from 'react'
import { api } from '@/services/api'
import type { Track, ImportRoadmapResponse } from '@/types'

type Page = 'roadmap' | 'settings' | 'dashboard'

interface RoadmapProps {
  onNavigate: (page: Page) => void
}

const isTauri =
  typeof window !== 'undefined' &&
  (typeof window.__TAURI__?.core?.invoke === 'function' ||
    typeof window.__TAURI__?.invoke === 'function')

export default function Roadmap({ onNavigate }: RoadmapProps) {
  const [tracks, setTracks] = useState<Track[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [importing, setImporting] = useState(false)
  const [importResult, setImportResult] = useState<ImportRoadmapResponse | null>(null)
  const [expandedTracks, setExpandedTracks] = useState<Set<string>>(new Set())
  const fileInputRef = useRef<HTMLInputElement>(null)

  const loadTracks = () => {
    api.getTracks()
      .then((res) => {
        if (res.ok) setTracks(res.tracks)
        else setError('Failed to load tracks')
      })
      .catch(() => setError('Failed to load tracks'))
      .finally(() => setLoading(false))
  }

  useEffect(() => {
    loadTracks()
  }, [])

  const toggleTrack = (key: string) => {
    setExpandedTracks((prev) => {
      const next = new Set(prev)
      if (next.has(key)) next.delete(key)
      else next.add(key)
      return next
    })
  }

  const handleImportClick = () => {
    if (!isTauri) return
    fileInputRef.current?.click()
  }

  const handleFileSelected = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    event.target.value = ''
    if (!file) return

    setImporting(true)
    setImportResult(null)
    setError(null)

    try {
      const content = await file.text()
      const res = await api.importRoadmap(content)
      setImportResult(res)
      if (res.ok) loadTracks()
      else setError(res.error || 'Import failed')
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Import failed')
    } finally {
      setImporting(false)
    }
  }

  if (loading) return <div className="p-4 text-muted-foreground">Loading roadmap...</div>

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
        <div>
          <h2 className="text-2xl font-bold">Roadmap</h2>
          <p className="text-muted-foreground">
            26-week curriculum across Java, DSA, LLD, HLD and FDE tracks.
          </p>
        </div>
        <div className="flex flex-col items-start gap-1 sm:items-end">
          <input
            ref={fileInputRef}
            type="file"
            accept=".md"
            onChange={handleFileSelected}
            className="hidden"
          />
          <button
            type="button"
            onClick={handleImportClick}
            disabled={!isTauri || importing}
            className="inline-flex items-center gap-2 rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90 disabled:cursor-not-allowed disabled:opacity-50"
          >
            {importing ? 'Importing...' : 'Import Roadmap'}
          </button>
          {!isTauri && (
            <span className="text-xs text-muted-foreground">
              Requires desktop app (run `npm run tauri dev`)
            </span>
          )}
        </div>
      </div>

      {importResult && (
        <div
          className={`rounded-lg border p-3 text-sm ${
            importResult.ok
              ? 'border-green-200 bg-green-50 text-green-900 dark:border-green-900 dark:bg-green-950 dark:text-green-100'
              : 'border-destructive/50 bg-destructive/10 text-destructive'
          }`}
        >
          <div className="flex flex-wrap items-center gap-x-2 gap-y-1">
            <span className="font-semibold">
              {importResult.ok ? 'Import successful' : 'Import failed'}
            </span>
            <span className="text-xs opacity-70">
              {importResult.track_count} tracks · {importResult.topic_count} topics ·{' '}
              {importResult.week_count} weeks
            </span>
          </div>
          {importResult.error && <p className="mt-1 text-xs">{importResult.error}</p>}
          {importResult.warnings.length > 0 && (
            <ul className="mt-2 list-inside list-disc text-xs opacity-80">
              {importResult.warnings.map((w, i) => (
                <li key={i}>{w}</li>
              ))}
            </ul>
          )}
        </div>
      )}

      {error && (
        <div className="rounded-lg border border-destructive/50 bg-destructive/10 p-3 text-sm text-destructive">
          {error}
        </div>
      )}

      {tracks.length === 0 ? (
        <div className="rounded-lg border border-dashed border-border p-8 text-center">
          <p className="text-sm text-muted-foreground">
            No tracks yet. Import the roadmap to populate your curriculum.
          </p>
        </div>
      ) : (
        <div className="space-y-3">
          {tracks.map((track) => (
            <div key={track.key} className="overflow-hidden rounded-lg border border-border">
              <button
                type="button"
                onClick={() => toggleTrack(track.key)}
                className="flex w-full items-center justify-between bg-muted/50 px-4 py-3 text-left transition-colors hover:bg-muted/70"
              >
                <span className="font-semibold">{track.name}</span>
                <span className="flex items-center gap-3">
                  <span className="rounded-full bg-background px-2 py-0.5 text-xs uppercase tracking-wide text-muted-foreground">
                    {track.key}
                  </span>
                  <svg
                    className={`h-4 w-4 transition-transform ${
                      expandedTracks.has(track.key) ? 'rotate-180' : ''
                    }`}
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M19 9l-7 7-7-7"
                    />
                  </svg>
                </span>
              </button>

              {expandedTracks.has(track.key) && (
                <div className="border-t border-border px-4 py-3 text-sm text-muted-foreground">
                  Modules and topics will render here after import.
                </div>
              )}
            </div>
          ))}
        </div>
      )}

      <div>
        <button
          type="button"
          onClick={() => onNavigate('dashboard')}
          className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted"
        >
          Back to Dashboard
        </button>
      </div>
    </div>
  )
}
