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
        <button
          onClick={() => {
            api.importRoadmap('')
              .then((res) => {
                if (res.ok) {
                  api.getTracks().then((r) => {
                    if (r.ok) setTracks(r.tracks)
                  })
                } else {
                  setError(res.error || 'Import failed')
                }
              })
          }}
          className="text-sm border border-border rounded-lg px-3 py-1.5 hover:bg-muted transition-colors"
        >
          Import Roadmap
        </button>
      </div>

      {error && (
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
                {tracks.filter((t) => t.key === track.key).flatMap(() => {
                  // Modules are loaded separately; this is a placeholder structure
                  return [
                    <div key="module-placeholder" className="ml-4 border-l border-border pl-4 my-2">
                      <div className="flex items-center justify-between">
                        <span className="text-sm font-medium">Loading modules...</span>
                      </div>
                    </div>,
                  ]
                })}
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
