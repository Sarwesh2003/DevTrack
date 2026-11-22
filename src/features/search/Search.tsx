import { useState } from 'react'
import { api } from '@/services/api'
import type { SearchResponse } from '@/types'

type Page = 'dashboard' | 'settings' | 'roadmap' | 'review' | 'dsa' | 'workspace' | 'projects' | 'analytics' | 'search'

const TYPE_LABEL: Record<string, string> = {
  topic: 'Topic',
  problem: 'Problem',
  project: 'Project',
  experiment: 'Experiment',
  question: 'Recall question',
  design: 'Design (LLD)',
  case_study: 'Case study (HLD)',
  note: 'Project note',
  entry: 'Learning entry',
}

const TYPE_PAGE: Record<string, Page> = {
  topic: 'roadmap',
  problem: 'dsa',
  project: 'projects',
  experiment: 'workspace',
  question: 'workspace',
  design: 'workspace',
  case_study: 'workspace',
  note: 'projects',
  entry: 'dashboard',
}

export default function Search({ onNavigate }: { onNavigate: (page: Page) => void }) {
  const [query, setQuery] = useState('')
  const [results, setResults] = useState<SearchResponse['results']>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const search = async () => {
    const q = query.trim()
    if (!q) {
      setResults([])
      return
    }
    setLoading(true)
    setError(null)
    try {
      const res = await api.searchAll(q)
      if (res.ok) setResults(res.results)
      else setError(res.error ?? 'Search failed')
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Search failed')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <h2 className="text-2xl font-bold">Search</h2>
          <p className="text-muted-foreground">
            Search across topics, problems, projects, experiments, recall questions, designs,
            case studies, notes and learning entries.
          </p>
        </div>
      </div>

      <div className="rounded-lg border border-border p-4">
        <div className="flex flex-col sm:flex-row gap-3 max-w-xl">
          <input
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={(e) => { if (e.key === 'Enter') search() }}
            placeholder="Search everything..."
            className="flex-1 rounded-md border border-border bg-background px-3 py-2 text-sm"
          />
          <button onClick={search} disabled={loading || !query.trim()} className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50">
            Search
          </button>
        </div>
      </div>

      {error && (
        <div className="rounded-lg border border-destructive/50 bg-destructive/10 p-3 text-sm text-destructive">
          {error}
        </div>
      )}

      {loading && <div className="p-4 text-muted-foreground">Searching...</div>}

      {!loading && results.length === 0 && query.trim() && (
        <div className="rounded-lg border border-border bg-muted/50 p-6 text-center text-sm text-muted-foreground">
          No results found.
        </div>
      )}

      {!loading && results.length > 0 && (
        <div className="rounded-lg border border-border overflow-hidden">
          <table className="w-full text-left text-sm">
            <thead>
              <tr className="border-b bg-muted/50">
                <th className="px-4 py-2 font-medium">Type</th>
                <th className="px-4 py-2 font-medium">Title</th>
                <th className="px-4 py-2 font-medium">Subtitle</th>
              </tr>
            </thead>
            <tbody className="divide-y">
              {results.map((r) => {
                const targetPage = TYPE_PAGE[r.type] ?? 'dashboard'
                return (
                  <tr key={`${r.type}-${r.id}`} className="hover:bg-muted/30 cursor-pointer">
                    <td className="px-4 py-3">
                      <span className="rounded bg-muted px-2 py-0.5 text-xs text-muted-foreground">
                        {TYPE_LABEL[r.type] ?? r.type}
                      </span>
                    </td>
                    <td
                      className="px-4 py-3 font-medium text-primary hover:underline"
                      onClick={() => onNavigate(targetPage)}
                    >
                      {r.title}
                    </td>
                    <td className="px-4 py-3 text-sm text-muted-foreground truncate max-w-[260px]">
                      {r.subtitle ?? '—'}
                    </td>
                  </tr>
                )
              })}
            </tbody>
          </table>
        </div>
      )}
    </div>
  )
}
