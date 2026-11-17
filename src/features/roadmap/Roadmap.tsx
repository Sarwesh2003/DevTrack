import { useState, useEffect } from 'react'
import { api } from '@/services/api'
import type { Track } from '@/types'

type Page = 'roadmap' | 'settings' | 'dashboard'

interface RoadmapProps {
  onNavigate: (page: Page) => void
}

export default function Roadmap({ onNavigate }: RoadmapProps) {
  const [tracks, setTracks] = useState<Track[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [importResult, setImportResult] = useState<{
    ok: boolean
    track_count: number
    topic_count: number
    week_count: number
    warnings: string[]
    error?: string
  } | null>(null)
  const [expandedTracks, setExpandedTracks] = useState<Set<string>>(new Set())

  useEffect(() => {
    api.getTracks()
      .then((res) => {
        if (res.ok) setTracks(res.tracks)
        else setError('Failed to load tracks')
      })
      .catch(() => setError('Failed to load tracks'))
      .finally(() => setLoading(false))
  }, [])

  const toggleTrack = (key: string) => {
    setExpandedTracks((prev) => {
      const next = new Set(prev)
      if (next.has(key)) next.delete(key)
      else next.add(key)
      return next
    })
  }

  const handleImport = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (!file) return

    setImportResult(null)
    setError(null)

    const content = await file.text()

    try {
      const res = await api.importRoadmap(content)
      setImportResult(res)
      if (res.ok) {
        api.getTracks().then((r) => {
          if (r.ok) setTracks(r.tracks)
        })
      } else {
        setError(res.error || 'Import failed')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Import failed')
    }
  }

  if (loading) return <div className="p-4">Loading roadmap...</div>

  if (error) {
    return (
      <div className="p-4">
        <h2 className="text-2xl font-bold">Roadmap</h2>
        <p className="text-destructive">{error}</p>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold">Roadmap</h2>
          <p className="text-muted-foreground">
            26-week curriculum across Java, DSA, LLD, HLD and FDE tracks.
          </p>
        </div>
        <div className="flex items-center gap-3">
          <input
            type="file"
            accept=".md"
            onChange={handleImport}
            className="text-sm text-muted-foreground"
          />
          <button
            onClick={() => {
              api.importRoadmap('')
                .then((res) => {
                  setImportResult(res)
                  if (res.ok) {
                    api.getTracks().then((r) => {
                      if (r.ok) setTracks(r.tracks)
                    })
                  } else {
                    setError(res.error || 'Import failed')
                  }
                })
            }}
            className="text-sm border border-border rounded-lg px-3 py-1.5 hover:bg-muted transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Import Empty
          </button>
        </div>
      </div>

      {importResult && (
        <div className={`p-3 rounded-lg text-sm border ${
          importResult.ok
            ? 'bg-green-50 border-green-200 text-green-900'
            : 'bg-red-50 border-red-200 text-red-900'
        }`}>
          <div className="flex items-center gap-2 mb-1">
            <span className="font-semibold">
              {importResult.ok ? 'Import successful' : 'Import failed'}
            </span>
            <span className="text-xs opacity-70">
              {importResult.track_count} tracks • {importResult.topic_count} topics • {importResult.week_count} weeks
            </span>
          </div>
          {importResult.warnings.length > 0 && (
            <div className="mt-1">
              <span className="text-xs opacity-80">Warnings:</span>
              <ul className="list-disc list-inside text-xs mt-1">
                {importResult.warnings.map((w, i) => (
                  <li key={i}>{w}</li>
                ))}
              </ul>
            </div>
          )}
          {importResult.error && (
            <p className="mt-1 text-xs">{importResult.error}</p>
          )}
        </div>
      )}

      {error && !importResult && (
        <div className="p-3 rounded-md bg-destructive/10 border border-destructive/50 text-destructive text-sm">
          {error}
        </div>
      )}

      <div className="space-y-4">
        {tracks.map((track) => (
          <div key={track.key} className="border rounded-lg overflow-hidden">
            <div
              onClick={() => toggleTrack(track.key)}
              className="flex items-center justify-between bg-muted/50 px-4 py-3 cursor-pointer hover:bg-muted/70 transition-colors"
            >
              <span className="font-semibold">{track.name}</span>
              <span className="text-xs">{track.position}. {track.key.toUpperCase()}</span>
              <svg
                className={`w-4 h-4 ${expandedTracks.has(track.key) ? 'rotate-180' : ''} transition-transform`}
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
              </svg>
            </div>

            {expandedTracks.has(track.key) && (
              <div className="px-4 py-3 border-t border-border">
                <div className="text-sm text-muted-foreground">
                  Modules and topics will render here after import.
                </div>
              </div>
            )}
          </div>
        ))}
      </div>

      <div className="mt-6 flex items-center gap-2">
        <button
          onClick={() => onNavigate('dashboard')}
          className="text-sm border border-border rounded-lg px-3 py-1.5 hover:bg-muted transition-colors"
        >
          Back to Dashboard
        </button>
      </div>
    </div>
  )
}
