import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { api } from '@/services/api'
import { todayISO } from '@/lib/dates'
import type {
  Problem,
  GetProblemsRequest,
  GetDsaProgressResponse,
  RevisionItemView,
  PreviewNeetcodeResponse,
  ImportNeetcodeResponse,
  Attempt,
} from '@/types'

type Page = 'dashboard' | 'settings' | 'roadmap' | 'review' | 'dsa'
type Tab = 'questions' | 'revision' | 'progress'
type CoverFilter = 'all' | 'new' | 'revision'

const isTauri =
  typeof window !== 'undefined' &&
  (typeof window.__TAURI__?.core?.invoke === 'function' ||
    typeof window.__TAURI__?.invoke === 'function')

const DIFFICULTIES = ['Easy', 'Medium', 'Hard']
const OUTCOMES: { value: string; label: string }[] = [
  { value: 'solved', label: 'Solved' },
  { value: 'partial', label: 'Partial' },
  { value: 'incorrect', label: 'Incorrect' },
]
const RECALLS: { value: string; label: string }[] = [
  { value: 'correct_clear', label: 'Correct (clear)' },
  { value: 'correct_slow', label: 'Correct (slow)' },
  { value: 'partial', label: 'Partial' },
  { value: 'incorrect', label: 'Incorrect' },
]

function difficultyClass(difficulty: string): string {
  switch (difficulty) {
    case 'Easy':
      return 'bg-green-100 text-green-800 dark:bg-green-900/40 dark:text-green-200'
    case 'Medium':
      return 'bg-amber-100 text-amber-800 dark:bg-amber-900/40 dark:text-amber-200'
    case 'Hard':
      return 'bg-red-100 text-red-800 dark:bg-red-900/40 dark:text-red-200'
    default:
      return 'bg-muted text-muted-foreground'
  }
}

export default function DSA({ onNavigate }: { onNavigate: (page: Page) => void }) {
  const [tab, setTab] = useState<Tab>('questions')
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [message, setMessage] = useState<string | null>(null)

  const [allProblems, setAllProblems] = useState<Problem[]>([])
  const [problems, setProblems] = useState<Problem[]>([])
  const [progress, setProgress] = useState<GetDsaProgressResponse | null>(null)
  const [revisionItems, setRevisionItems] = useState<RevisionItemView[]>([])
  const [selectedId, setSelectedId] = useState<string | null>(null)

  const [search, setSearch] = useState('')
  const [category, setCategory] = useState('')
  const [difficulty, setDifficulty] = useState('')
  const [coverFilter, setCoverFilter] = useState<CoverFilter>('all')

  const [importOpen, setImportOpen] = useState(false)
  const [importContent, setImportContent] = useState('')
  const [preview, setPreview] = useState<PreviewNeetcodeResponse | null>(null)
  const [importResult, setImportResult] = useState<ImportNeetcodeResponse | null>(null)
  const [importing, setImporting] = useState(false)
  const fileInputRef = useRef<HTMLInputElement>(null)

  const loadBase = useCallback(async () => {
    const [allRes, progressRes, revRes] = await Promise.all([
      api.getProblems({}).catch(() => null),
      api.getDsaProgress().catch(() => null),
      api.getRevisionQueue().catch(() => null),
    ])
    if (allRes?.ok) setAllProblems(allRes.problems)
    if (progressRes?.ok) setProgress(progressRes)
    if (revRes?.ok) setRevisionItems(revRes.items)
  }, [])

  const loadProblems = useCallback(async () => {
    const req: GetProblemsRequest = { cover_filter: coverFilter }
    if (search.trim()) req.search = search.trim()
    if (category) req.category = category
    if (difficulty) req.difficulty = difficulty
    const res = await api.getProblems(req).catch(() => null)
    if (res?.ok) setProblems(res.problems)
  }, [search, category, difficulty, coverFilter])

  useEffect(() => {
    let active = true
    ;(async () => {
      setLoading(true)
      await Promise.all([loadBase(), loadProblems()])
      if (active) setLoading(false)
    })()
    return () => {
      active = false
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  useEffect(() => {
    loadProblems()
  }, [loadProblems])

  const refreshAll = useCallback(async () => {
    await Promise.all([loadBase(), loadProblems()])
  }, [loadBase, loadProblems])

  const problemById = useMemo(() => {
    const map = new Map<string, Problem>()
    for (const p of allProblems) map.set(p.id, p)
    return map
  }, [allProblems])

  const categories = useMemo(
    () => (progress?.categories ?? []).map((c) => c.category),
    [progress],
  )

  const selectedProblem = selectedId ? problemById.get(selectedId) ?? null : null

  const handlePickFile = () => {
    if (!isTauri) return
    fileInputRef.current?.click()
  }

  const handleFile = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    event.target.value = ''
    if (!file) return
    setImporting(true)
    setError(null)
    setImportResult(null)
    setPreview(null)
    try {
      const content = await file.text()
      setImportContent(content)
      const res = await api.previewNeetcode(content)
      setPreview(res)
      if (!res.ok) setError(res.error ?? 'Could not parse CSV')
      else setImportOpen(true)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Could not read file')
    } finally {
      setImporting(false)
    }
  }

  const handleImport = async () => {
    if (!importContent) return
    setImporting(true)
    setError(null)
    try {
      const res = await api.importNeetcode(importContent)
      setImportResult(res)
      if (res.ok) {
        setMessage(
          `Imported ${res.imported} problems (${res.new_learning} new, ${res.previously_covered} revision-only).`,
        )
        setImportOpen(false)
        setPreview(null)
        setImportContent('')
        await refreshAll()
      } else {
        setError(res.error ?? 'Import failed')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Import failed')
    } finally {
      setImporting(false)
    }
  }

  const handleToggleFavorite = async (problem: Problem) => {
    const next = !problem.is_favorite
    setProblems((prev) =>
      prev.map((p) => (p.id === problem.id ? { ...p, is_favorite: next } : p)),
    )
    setAllProblems((prev) =>
      prev.map((p) => (p.id === problem.id ? { ...p, is_favorite: next } : p)),
    )
    try {
      const res = await api.toggleFavorite({ problem_id: problem.id, favorite: next })
      if (!res.ok) {
        setError(res.error ?? 'Could not update favorite')
        await loadProblems()
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Could not update favorite')
      await loadProblems()
    }
  }

  const handleBaseline = async () => {
    setError(null)
    setMessage(null)
    try {
      const res = await api.startRevisionBaseline()
      if (res.ok) {
        setMessage(
          res.created > 0
            ? `Started revision baseline for ${res.created} previously covered problems.`
            : 'Revision baseline already present — nothing new to add.',
        )
        await refreshAll()
      } else {
        setError(res.error ?? 'Could not start baseline')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Could not start baseline')
    }
  }

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
        <div>
          <h2 className="text-2xl font-bold">DSA</h2>
          <p className="text-muted-foreground">
            NeetCode 250 workspace — attempts, Java solutions, favorites and revision.
          </p>
        </div>
        <div className="flex flex-col items-start gap-1 sm:items-end">
          <input
            ref={fileInputRef}
            type="file"
            accept=".csv"
            onChange={handleFile}
            className="hidden"
          />
          <button
            type="button"
            onClick={handlePickFile}
            disabled={!isTauri || importing}
            className="inline-flex items-center gap-2 rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90 disabled:cursor-not-allowed disabled:opacity-50"
          >
            {importing ? 'Reading...' : 'Import NeetCode CSV'}
          </button>
          {!isTauri && (
            <span className="text-xs text-muted-foreground">
              Requires desktop app (run `npm run tauri dev`)
            </span>
          )}
        </div>
      </div>

      {message && (
        <div className="rounded-lg border border-green-200 bg-green-50 p-3 text-sm text-green-900 dark:border-green-900 dark:bg-green-950 dark:text-green-100">
          {message}
        </div>
      )}
      {error && (
        <div className="rounded-lg border border-destructive/50 bg-destructive/10 p-3 text-sm text-destructive">
          {error}
        </div>
      )}

      {importOpen && preview && (
        <section className="rounded-lg border border-border p-4">
          <div className="flex items-start justify-between">
            <h3 className="font-semibold">Preview import</h3>
            <button
              onClick={() => {
                setImportOpen(false)
                setPreview(null)
              }}
              className="text-sm text-muted-foreground hover:text-foreground"
            >
              Cancel
            </button>
          </div>
          <div className="mt-2 grid grid-cols-2 gap-3 text-sm sm:grid-cols-4">
            <Stat label="Rows" value={`${preview.total_rows}`} />
            <Stat label="New learning" value={`${preview.new_learning}`} />
            <Stat label="Revision-only" value={`${preview.previously_covered}`} />
            <Stat label="Categories" value={`${preview.by_category.length}`} />
          </div>
          {preview.by_category.length > 0 && (
            <div className="mt-3 flex flex-wrap gap-2 text-xs text-muted-foreground">
              {preview.by_category.map((c) => (
                <span key={c.category} className="rounded-full border border-border px-2 py-0.5">
                  {c.category}: {c.count}
                  {c.previously_covered ? ' (revision)' : ''}
                </span>
              ))}
            </div>
          )}
          <button
            onClick={handleImport}
            disabled={importing}
            className="mt-4 rounded-lg bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
          >
            {importing ? 'Importing...' : 'Confirm import'}
          </button>
        </section>
      )}

      {importResult && importResult.warnings.length > 0 && (
        <ul className="space-y-1 text-sm text-amber-700 dark:text-amber-300">
          {importResult.warnings.map((w) => (
            <li key={w}>{w}</li>
          ))}
        </ul>
      )}

      <div className="flex gap-1 border-b border-border">
        {(['questions', 'revision', 'progress'] as Tab[]).map((t) => (
          <button
            key={t}
            onClick={() => setTab(t)}
            className={`-mb-px border-b-2 px-3 py-2 text-sm capitalize transition-colors ${
              tab === t
                ? 'border-primary font-medium text-foreground'
                : 'border-transparent text-muted-foreground hover:text-foreground'
            }`}
          >
            {t}
            {t === 'revision' && revisionItems.length > 0 ? ` (${revisionItems.length})` : ''}
          </button>
        ))}
      </div>

      {loading ? (
        <p className="text-muted-foreground">Loading DSA workspace...</p>
      ) : tab === 'questions' ? (
        selectedProblem ? (
          <QuestionDetail
            problem={selectedProblem}
            onBack={() => setSelectedId(null)}
            onChanged={refreshAll}
            onToggleFavorite={() => handleToggleFavorite(selectedProblem)}
          />
        ) : (
          <div className="space-y-4">
            <div className="flex flex-wrap items-center gap-2">
              <input
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                placeholder="Search problems or categories"
                className="min-w-[12rem] flex-1 rounded-md border border-border bg-background px-3 py-2 text-sm"
              />
              <select
                value={category}
                onChange={(e) => setCategory(e.target.value)}
                className="rounded-md border border-border bg-background px-2 py-2 text-sm"
              >
                <option value="">All categories</option>
                {categories.map((c) => (
                  <option key={c} value={c}>
                    {c}
                  </option>
                ))}
              </select>
              <select
                value={difficulty}
                onChange={(e) => setDifficulty(e.target.value)}
                className="rounded-md border border-border bg-background px-2 py-2 text-sm"
              >
                <option value="">All difficulties</option>
                {DIFFICULTIES.map((d) => (
                  <option key={d} value={d}>
                    {d}
                  </option>
                ))}
              </select>
              <select
                value={coverFilter}
                onChange={(e) => setCoverFilter(e.target.value as CoverFilter)}
                className="rounded-md border border-border bg-background px-2 py-2 text-sm"
              >
                <option value="all">All coverage</option>
                <option value="new">New learning</option>
                <option value="revision">Revision-only</option>
              </select>
            </div>

            {problems.length === 0 ? (
              <p className="text-sm text-muted-foreground">
                {allProblems.length === 0
                  ? 'No problems yet — import the NeetCode 250 CSV to get started.'
                  : 'No problems match these filters.'}
              </p>
            ) : (
              <ul className="divide-y divide-border rounded-lg border border-border">
                {problems.map((p) => (
                  <li key={p.id}>
                    <div className="flex items-center gap-3 px-3 py-2">
                      <button
                        onClick={() => handleToggleFavorite(p)}
                        title={p.is_favorite ? 'Remove favorite' : 'Add favorite'}
                        className={`text-lg leading-none ${
                          p.is_favorite ? 'text-amber-500' : 'text-muted-foreground/40'
                        }`}
                      >
                        ★
                      </button>
                      <button
                        onClick={() => setSelectedId(p.id)}
                        className="flex flex-1 items-center justify-between gap-3 text-left"
                      >
                        <span className="min-w-0">
                          <span className="truncate font-medium">{p.title}</span>
                          <span className="ml-2 text-xs text-muted-foreground">{p.category}</span>
                        </span>
                        <span className="flex shrink-0 items-center gap-2 text-xs">
                          {p.is_previously_covered ? (
                            <span className="rounded-full border border-border px-2 py-0.5 text-muted-foreground">
                              revision
                            </span>
                          ) : (
                            <span className="rounded-full border border-border px-2 py-0.5 text-muted-foreground">
                              new
                            </span>
                          )}
                          <span className={`rounded-full px-2 py-0.5 ${difficultyClass(p.difficulty)}`}>
                            {p.difficulty}
                          </span>
                          {p.solved ? (
                            <span className="text-green-600 dark:text-green-400">solved</span>
                          ) : p.attempt_count > 0 ? (
                            <span className="text-muted-foreground">{p.attempt_count} tries</span>
                          ) : null}
                        </span>
                      </button>
                    </div>
                  </li>
                ))}
              </ul>
            )}
          </div>
        )
      ) : tab === 'revision' ? (
        <RevisionTab
          items={revisionItems}
          problemById={problemById}
          onBaseline={handleBaseline}
          onChanged={refreshAll}
        />
      ) : (
        <ProgressTab progress={progress} />
      )}

      <div className="flex items-center gap-2 pt-1">
        <button
          onClick={() => onNavigate('dashboard')}
          className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted"
        >
          Back to Today
        </button>
        <button
          onClick={() => onNavigate('roadmap')}
          className="rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted"
        >
          Roadmap
        </button>
      </div>
    </div>
  )
}

function Stat({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-lg border border-border p-3">
      <div className="text-xs text-muted-foreground">{label}</div>
      <div className="mt-1 text-xl font-semibold">{value}</div>
    </div>
  )
}

function QuestionDetail({
  problem,
  onBack,
  onChanged,
  onToggleFavorite,
}: {
  problem: Problem
  onBack: () => void
  onChanged: () => Promise<void>
  onToggleFavorite: () => void
}) {
  const [attempts, setAttempts] = useState<Attempt[]>([])
  const [solution, setSolution] = useState('')
  const [solutionPath, setSolutionPath] = useState<string | null>(null)
  const [loading, setLoading] = useState(true)
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [message, setMessage] = useState<string | null>(null)

  const [attemptDate, setAttemptDate] = useState(todayISO())
  const [outcome, setOutcome] = useState('solved')
  const [duration, setDuration] = useState('')
  const [notes, setNotes] = useState('')
  const [complexity, setComplexity] = useState('')
  const [edgeCases, setEdgeCases] = useState('')
  const [explanation, setExplanation] = useState('')

  const load = useCallback(async () => {
    setLoading(true)
    const [attemptRes, solutionRes] = await Promise.all([
      api.listAttempts(problem.id).catch(() => null),
      api.getSolution(problem.id).catch(() => null),
    ])
    if (attemptRes?.ok) setAttempts(attemptRes.attempts)
    if (solutionRes?.ok && solutionRes.solution) {
      setSolution(solutionRes.solution.content ?? '')
      setSolutionPath(solutionRes.solution.file_path)
    } else {
      setSolution('')
      setSolutionPath(null)
    }
    setLoading(false)
  }, [problem.id])

  useEffect(() => {
    load()
  }, [load])

  const handleSaveAttempt = async () => {
    setBusy(true)
    setError(null)
    setMessage(null)
    try {
      const res = await api.recordAttempt({
        problem_id: problem.id,
        attempt_date: attemptDate,
        outcome,
        duration_minutes: duration ? Number(duration) : undefined,
        notes: notes.trim() || undefined,
        complexity_note: complexity.trim() || undefined,
        edge_cases: edgeCases.trim() || undefined,
        interview_explanation: explanation.trim() || undefined,
      })
      if (res.ok) {
        setMessage('Attempt recorded.')
        setNotes('')
        setComplexity('')
        setEdgeCases('')
        setExplanation('')
        setDuration('')
        await load()
        await onChanged()
      } else {
        setError(res.error ?? 'Could not record attempt')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Could not record attempt')
    } finally {
      setBusy(false)
    }
  }

  const handleSaveSolution = async () => {
    setBusy(true)
    setError(null)
    setMessage(null)
    try {
      const res = await api.saveSolution({ problem_id: problem.id, content: solution })
      if (res.ok) {
        setSolutionPath(res.file_path ?? null)
        setMessage('Java solution saved.')
        await onChanged()
      } else {
        setError(res.error ?? 'Could not save solution')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Could not save solution')
    } finally {
      setBusy(false)
    }
  }

  return (
    <div className="space-y-5">
      <div className="flex items-start justify-between gap-3">
        <div>
          <button
            onClick={onBack}
            className="mb-2 text-sm text-muted-foreground hover:text-foreground"
          >
            ← Back to list
          </button>
          <h3 className="text-xl font-semibold">{problem.title}</h3>
          <div className="mt-1 flex flex-wrap items-center gap-2 text-xs">
            <span className="text-muted-foreground">{problem.category}</span>
            <span className={`rounded-full px-2 py-0.5 ${difficultyClass(problem.difficulty)}`}>
              {problem.difficulty}
            </span>
            <span className="rounded-full border border-border px-2 py-0.5 text-muted-foreground">
              {problem.is_previously_covered ? 'revision-only' : 'new learning'}
            </span>
            <a
              href={problem.url}
              onClick={(e) => {
                e.preventDefault()
                api
                  .openExternalUrl(problem.url)
                  .then((res) => {
                    if (!res.ok) setError(res.error ?? 'Could not open link')
                  })
                  .catch(() => setError('Could not open link'))
              }}
              className="text-primary underline-offset-2 hover:underline"
            >
              Open problem
            </a>
          </div>
        </div>
        <button
          onClick={onToggleFavorite}
          className={`rounded-lg border border-border px-3 py-1.5 text-sm ${
            problem.is_favorite ? 'text-amber-500' : 'text-muted-foreground'
          }`}
        >
          {problem.is_favorite ? '★ Favorite' : '☆ Favorite'}
        </button>
      </div>

      {message && (
        <div className="rounded-lg border border-green-200 bg-green-50 p-3 text-sm text-green-900 dark:border-green-900 dark:bg-green-950 dark:text-green-100">
          {message}
        </div>
      )}
      {error && (
        <div className="rounded-lg border border-destructive/50 bg-destructive/10 p-3 text-sm text-destructive">
          {error}
        </div>
      )}

      <section className="rounded-lg border border-border p-4">
        <h4 className="mb-3 font-semibold">Record attempt</h4>
        <div className="grid gap-3 sm:grid-cols-3">
          <Field label="Date">
            <input
              type="date"
              value={attemptDate}
              onChange={(e) => setAttemptDate(e.target.value)}
              className="w-full rounded-md border border-border bg-background px-2 py-1.5 text-sm"
            />
          </Field>
          <Field label="Outcome">
            <select
              value={outcome}
              onChange={(e) => setOutcome(e.target.value)}
              className="w-full rounded-md border border-border bg-background px-2 py-1.5 text-sm"
            >
              {OUTCOMES.map((o) => (
                <option key={o.value} value={o.value}>
                  {o.label}
                </option>
              ))}
            </select>
          </Field>
          <Field label="Duration (min)">
            <input
              type="number"
              min={0}
              value={duration}
              onChange={(e) => setDuration(e.target.value)}
              className="w-full rounded-md border border-border bg-background px-2 py-1.5 text-sm"
            />
          </Field>
        </div>
        <div className="mt-3 grid gap-3 sm:grid-cols-2">
          <Field label="Complexity">
            <input
              value={complexity}
              onChange={(e) => setComplexity(e.target.value)}
              placeholder="e.g. O(n) time, O(1) space"
              className="w-full rounded-md border border-border bg-background px-2 py-1.5 text-sm"
            />
          </Field>
          <Field label="Edge cases">
            <input
              value={edgeCases}
              onChange={(e) => setEdgeCases(e.target.value)}
              placeholder="e.g. empty input, duplicates"
              className="w-full rounded-md border border-border bg-background px-2 py-1.5 text-sm"
            />
          </Field>
        </div>
        <div className="mt-3 grid gap-3 sm:grid-cols-2">
          <Field label="Notes">
            <textarea
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
              rows={2}
              className="w-full rounded-md border border-border bg-background px-2 py-1.5 text-sm"
            />
          </Field>
          <Field label="Interview explanation">
            <textarea
              value={explanation}
              onChange={(e) => setExplanation(e.target.value)}
              rows={2}
              className="w-full rounded-md border border-border bg-background px-2 py-1.5 text-sm"
            />
          </Field>
        </div>
        <button
          onClick={handleSaveAttempt}
          disabled={busy || !isTauri}
          className="mt-3 rounded-lg bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
        >
          Save attempt
        </button>
      </section>

      <section className="rounded-lg border border-border p-4">
        <h4 className="mb-2 font-semibold">Attempt history</h4>
        {loading ? (
          <p className="text-sm text-muted-foreground">Loading attempts...</p>
        ) : attempts.length === 0 ? (
          <p className="text-sm text-muted-foreground">No attempts recorded yet.</p>
        ) : (
          <ul className="space-y-2 text-sm">
            {attempts.map((a) => (
              <li key={a.id} className="rounded-md border border-border p-2">
                <div className="flex items-center justify-between">
                  <span className="font-medium">
                    {a.attempt_date} · {a.outcome}
                  </span>
                  {a.duration_minutes != null && (
                    <span className="text-xs text-muted-foreground">{a.duration_minutes} min</span>
                  )}
                </div>
                {a.complexity_note && (
                  <div className="text-xs text-muted-foreground">Complexity: {a.complexity_note}</div>
                )}
                {a.edge_cases && (
                  <div className="text-xs text-muted-foreground">Edge cases: {a.edge_cases}</div>
                )}
                {a.notes && <div className="mt-1 text-xs">{a.notes}</div>}
                {a.interview_explanation && (
                  <div className="mt-1 text-xs text-muted-foreground">{a.interview_explanation}</div>
                )}
              </li>
            ))}
          </ul>
        )}
      </section>

      <section className="rounded-lg border border-border p-4">
        <div className="mb-2 flex items-center justify-between">
          <h4 className="font-semibold">Java solution</h4>
          {solutionPath && (
            <span className="truncate text-xs text-muted-foreground">{solutionPath}</span>
          )}
        </div>
        <textarea
          value={solution}
          onChange={(e) => setSolution(e.target.value)}
          rows={12}
          spellCheck={false}
          placeholder="// Write your Java solution here"
          className="w-full rounded-md border border-border bg-background px-3 py-2 font-mono text-xs"
        />
        <button
          onClick={handleSaveSolution}
          disabled={busy || !isTauri}
          className="mt-3 rounded-lg bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
        >
          Save solution
        </button>
      </section>
    </div>
  )
}

function Field({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <label className="block text-xs">
      <span className="mb-1 block text-muted-foreground">{label}</span>
      {children}
    </label>
  )
}

function RevisionTab({
  items,
  problemById,
  onBaseline,
  onChanged,
}: {
  items: RevisionItemView[]
  problemById: Map<string, Problem>
  onBaseline: () => void
  onChanged: () => Promise<void>
}) {
  return (
    <div className="space-y-4">
      <section className="rounded-lg border border-border p-4">
        <h3 className="font-semibold">Revision baseline</h3>
        <p className="mt-1 text-sm text-muted-foreground">
          Create a one-time baseline for previously covered problems that have no history. This sets
          a new starting point for revision; it does not recover original dates.
        </p>
        <button
          onClick={onBaseline}
          disabled={!isTauri}
          className="mt-3 rounded-lg border border-border px-3 py-1.5 text-sm transition-colors hover:bg-muted disabled:opacity-50"
        >
          Start revision baseline now
        </button>
      </section>

      <section className="rounded-lg border border-border p-4">
        <h3 className="mb-3 font-semibold">Due for review</h3>
        {items.length === 0 ? (
          <p className="text-sm text-muted-foreground">Nothing due right now.</p>
        ) : (
          <ul className="space-y-3">
            {items.map((item) => (
              <RevisionRow
                key={item.id}
                item={item}
                problem={
                  item.source_type === 'problem' ? problemById.get(item.source_id) ?? null : null
                }
                onChanged={onChanged}
              />
            ))}
          </ul>
        )}
      </section>
    </div>
  )
}

function RevisionRow({
  item,
  problem,
  onChanged,
}: {
  item: RevisionItemView
  problem: Problem | null
  onChanged: () => Promise<void>
}) {
  const [recall, setRecall] = useState('correct_clear')
  const [confidence, setConfidence] = useState('3')
  const [notes, setNotes] = useState('')
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleReview = async () => {
    setBusy(true)
    setError(null)
    try {
      const res = await api.recordReview({
        revision_item_id: item.id,
        reviewed_at: todayISO(),
        recall_result: recall,
        confidence: confidence ? Number(confidence) : undefined,
        notes: notes.trim() || undefined,
      })
      if (res.ok) {
        await onChanged()
      } else {
        setError(res.error ?? 'Could not record review')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Could not record review')
    } finally {
      setBusy(false)
    }
  }

  return (
    <li className="rounded-md border border-border p-3">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <div>
          <div className="font-medium">
            {problem ? problem.title : item.source_id}
            {problem && <span className="ml-2 text-xs text-muted-foreground">{problem.category}</span>}
          </div>
          <div className="text-xs text-muted-foreground">
            Due {item.next_due_date} · interval {item.current_interval}d · rep {item.repetition} ·{' '}
            {item.state}
            {item.last_reviewed_date ? ` · last ${item.last_reviewed_date}` : ''}
          </div>
        </div>
        <div className="flex items-center gap-2">
          <select
            value={recall}
            onChange={(e) => setRecall(e.target.value)}
            className="rounded-md border border-border bg-background px-2 py-1 text-xs"
          >
            {RECALLS.map((r) => (
              <option key={r.value} value={r.value}>
                {r.label}
              </option>
            ))}
          </select>
          <select
            value={confidence}
            onChange={(e) => setConfidence(e.target.value)}
            className="rounded-md border border-border bg-background px-2 py-1 text-xs"
          >
            {[1, 2, 3, 4, 5].map((n) => (
              <option key={n} value={n}>
                {n}/5
              </option>
            ))}
          </select>
          <button
            onClick={handleReview}
            disabled={busy || !isTauri}
            className="rounded-lg bg-primary px-3 py-1 text-xs font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
          >
            {busy ? 'Saving...' : 'Record'}
          </button>
        </div>
      </div>
      <input
        value={notes}
        onChange={(e) => setNotes(e.target.value)}
        placeholder="Optional note"
        className="mt-2 w-full rounded-md border border-border bg-background px-2 py-1 text-xs"
      />
      {error && <div className="mt-1 text-xs text-destructive">{error}</div>}
    </li>
  )
}

function ProgressTab({ progress }: { progress: GetDsaProgressResponse | null }) {
  if (!progress) {
    return <p className="text-sm text-muted-foreground">No progress data yet.</p>
  }
  const pct = (n: number, d: number) => (d > 0 ? Math.round((n / d) * 100) : 0)

  return (
    <div className="space-y-4">
      <div className="grid grid-cols-3 gap-3">
        <Stat label="Problems" value={`${progress.total_problems}`} />
        <Stat label="Attempted" value={`${progress.total_attempted}`} />
        <Stat label="Solved" value={`${progress.total_solved}`} />
      </div>

      {progress.categories.length === 0 ? (
        <p className="text-sm text-muted-foreground">Import the NeetCode 250 CSV to see categories.</p>
      ) : (
        <ul className="space-y-3">
          {progress.categories.map((c) => (
            <li key={`${c.category}-${c.previously_covered}`}>
              <div className="flex items-center justify-between text-sm">
                <span className="flex items-center gap-2">
                  {c.category}
                  {c.previously_covered && (
                    <span className="rounded-full border border-border px-2 py-0.5 text-[10px] text-muted-foreground">
                      revision
                    </span>
                  )}
                </span>
                <span className="text-xs text-muted-foreground">
                  {c.solved}/{c.total} solved · {c.attempted} attempted
                </span>
              </div>
              <div className="mt-1 h-2 w-full overflow-hidden rounded-full bg-muted">
                <div
                  className="h-full rounded-full bg-green-500"
                  style={{ width: `${pct(c.solved, c.total)}%` }}
                />
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  )
}
