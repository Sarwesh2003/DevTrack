import { useState, useEffect, useRef } from 'react'
import { api } from '@/services/api'
import type { Track, Module, Topic, ImportRoadmapResponse } from '@/types'

type Page = 'roadmap' | 'settings' | 'dashboard' | 'review' | 'dsa' | 'workspace' | 'projects' | 'analytics' | 'search'

interface RoadmapProps {
  onNavigate: (page: Page) => void
}

const isTauri =
  typeof window !== 'undefined' &&
  (typeof window.__TAURI__?.core?.invoke === 'function' ||
    typeof window.__TAURI__?.invoke === 'function')

function statusClass(status: string): string {
  switch (status) {
    case 'Needs revision':
      return 'bg-amber-100 text-amber-900 dark:bg-amber-950 dark:text-amber-200'
    case 'In progress':
      return 'bg-blue-100 text-blue-900 dark:bg-blue-950 dark:text-blue-200'
    case 'Complete':
      return 'bg-green-100 text-green-900 dark:bg-green-950 dark:text-green-200'
    default:
      return 'bg-muted text-muted-foreground'
  }
}

export default function Roadmap({ onNavigate }: RoadmapProps) {
  const [tracks, setTracks] = useState<Track[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [importing, setImporting] = useState(false)
  const [importResult, setImportResult] = useState<ImportRoadmapResponse | null>(null)
  const [expandedTracks, setExpandedTracks] = useState<Set<string>>(new Set())
  const [modulesByTrack, setModulesByTrack] = useState<Record<string, Module[]>>({})
  const [topicsByModule, setTopicsByModule] = useState<Record<string, Topic[]>>({})
  const [loadingTrack, setLoadingTrack] = useState<string | null>(null)
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

  const loadTrackContent = async (trackKey: string) => {
    setLoadingTrack(trackKey)
    try {
      const modRes = await api.getModules(trackKey)
      const mods = modRes.ok ? modRes.modules : []
      setModulesByTrack((prev) => ({ ...prev, [trackKey]: mods }))

      const topicEntries = await Promise.all(
        mods.map(async (mod) => {
          const res = await api.getTopics(mod.id)
          return [mod.id, res.ok ? res.topics : []] as const
        }),
      )
      setTopicsByModule((prev) => {
        const next = { ...prev }
        for (const [id, list] of topicEntries) next[id] = list
        return next
      })
    } catch {
      setError('Failed to load track content')
    } finally {
      setLoadingTrack(null)
    }
  }

  const toggleTrack = (track: Track) => {
    const key = track.key
    setExpandedTracks((prev) => {
      const next = new Set(prev)
      if (next.has(key)) {
        next.delete(key)
        return next
      }
      next.add(key)
      if (!modulesByTrack[key]) loadTrackContent(key)
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
      if (res.ok) {
        setModulesByTrack({})
        setTopicsByModule({})
        setExpandedTracks(new Set())
        loadTracks()
      } else {
        setError(res.error || 'Import failed')
      }
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
            26-week curriculum across Java, DSA, LLD, HLD and AI Upskilling tracks.
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
          {tracks.map((track) => {
            const expanded = expandedTracks.has(track.key)
            const modules = modulesByTrack[track.key]
            return (
              <div key={track.key} className="overflow-hidden rounded-lg border border-border">
                <button
                  type="button"
                  onClick={() => toggleTrack(track)}
                  className="flex w-full items-center justify-between bg-muted/50 px-4 py-3 text-left transition-colors hover:bg-muted/70"
                >
                  <span className="font-semibold">{track.name}</span>
                  <span className="flex items-center gap-3">
                    <span className="rounded-full bg-background px-2 py-0.5 text-xs uppercase tracking-wide text-muted-foreground">
                      {track.key}
                    </span>
                    <svg
                      className={`h-4 w-4 transition-transform ${expanded ? 'rotate-180' : ''}`}
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

                {expanded && (
                  <div className="space-y-4 border-t border-border bg-background px-4 py-4">
                    {loadingTrack === track.key ? (
                      <p className="text-sm text-muted-foreground">Loading topics...</p>
                    ) : !modules ? (
                      <p className="text-sm text-muted-foreground">No modules loaded.</p>
                    ) : modules.length === 0 ? (
                      <p className="text-sm text-muted-foreground">
                        No topics in this track yet.
                      </p>
                    ) : (
                      modules.map((mod) => {
                        const topics = topicsByModule[mod.id] ?? []
                        return (
                          <div key={mod.id} className="space-y-2">
                            <h4 className="text-sm font-semibold text-foreground">{mod.name}</h4>
                            {topics.length === 0 ? (
                              <p className="text-xs text-muted-foreground">No topics.</p>
                            ) : (
                              <ul className="divide-y divide-border rounded-md border border-border">
                                {topics.map((topic) => (
                                  <li
                                    key={topic.id}
                                    className="flex items-center justify-between gap-3 px-3 py-2"
                                  >
                                    <div className="min-w-0">
                                      <div className="flex items-center gap-2">
                                        <span className="rounded bg-muted px-1.5 py-0.5 text-xs font-mono text-muted-foreground">
                                          {topic.key}
                                        </span>
                                        <span className="truncate text-sm">{topic.title}</span>
                                      </div>
                                      <div className="mt-0.5 flex flex-wrap gap-x-3 text-xs text-muted-foreground">
                                        <span>{topic.priority} priority</span>
                                        {topic.estimate_minutes != null && (
                                          <span>
                                            {Math.round((topic.estimate_minutes / 60) * 10) / 10} h
                                          </span>
                                        )}
                                      </div>
                                    </div>
                                    <span
                                      className={`shrink-0 rounded-full px-2 py-0.5 text-xs font-medium ${statusClass(
                                        topic.status,
                                      )}`}
                                    >
                                      {topic.status}
                                    </span>
                                  </li>
                                ))}
                              </ul>
                            )}
                          </div>
                        )
                      })
                    )}
                  </div>
                )}
              </div>
            )
          })}
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
