import { useCallback, useEffect, useState } from 'react'
import { api } from '@/services/api'
import type {
  Project,
  UpsertProjectRequest,
  UpsertProjectMilestoneRequest,
  UpsertProjectTaskRequest,
  UpsertProjectLinkRequest,
  UpsertProjectNoteRequest,
  ProjectMilestone,
  ProjectTask,
  ProjectDetailResponse,
} from '@/types'

type Page = 'dashboard' | 'settings' | 'roadmap' | 'review' | 'dsa' | 'workspace' | 'projects'

const TRACKS = [
  { key: '', name: 'All tracks' },
  { key: 'A', name: 'Core Java' },
  { key: 'C', name: 'LLD' },
  { key: 'D', name: 'HLD' },
  { key: 'E', name: 'FDE' },
] as const

const NOTE_KINDS = [
  'note',
  'evaluation',
  'security',
  'reliability',
  'cost',
  'latency',
  'walkthrough',
  'evidence',
  'doc',
] as const

export default function Projects(_props: { onNavigate: (page: Page) => void }) {
  const [projects, setProjects] = useState<Project[]>([])
  const [selectedId, setSelectedId] = useState<string | null>(null)
  const [detail, setDetail] = useState<ProjectDetailResponse | null>(null)
  const [trackFilter, setTrackFilter] = useState('')
  const [loading, setLoading] = useState(false)
  const [message, setMessage] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [showAddProject, setShowAddProject] = useState(false)

  // add project form
  const [npTrack, setNpTrack] = useState('E')
  const [npName, setNpName] = useState('')
  const [npSlug, setNpSlug] = useState('')
  const [npStatus, setNpStatus] = useState('active')
  const [npRepo, setNpRepo] = useState('')
  const [npLocalPath, setNpLocalPath] = useState('')

  // detail edit / add item forms
  const [editName, setEditName] = useState('')
  const [editStatus, setEditStatus] = useState('')
  const [editRepo, setEditRepo] = useState('')
  const [editLocalPath, setEditLocalPath] = useState('')
  const [editReadme, setEditReadme] = useState('')
  const [editMilestone, setEditMilestone] = useState('')
  const [editNotes, setEditNotes] = useState('')

  const [nmName, setNmName] = useState('')
  const [nmDue, setNmDue] = useState('')

  const [ntTitle, setNtTitle] = useState('')
  const [ntStatus, setNtStatus] = useState('todo')
  const [ntDue, setNtDue] = useState('')

  const [nlTitle, setNlTitle] = useState('')
  const [nlUrl, setNlUrl] = useState('')
  const [nlType, setNlType] = useState('doc')

  const [nnKind, setNnKind] = useState('note')
  const [nnTitle, setNnTitle] = useState('')
  const [nnContent, setNnContent] = useState('')

  const run = async (fn: () => Promise<void>) => {
    setLoading(true)
    setError(null)
    setMessage(null)
    try {
      await fn()
      await refresh()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Action failed')
    } finally {
      setLoading(false)
    }
  }

  const refresh = useCallback(async () => {
    const res = await api.listProjects(trackFilter || undefined).catch(() => null)
    if (res?.ok) setProjects(res.projects)
  }, [trackFilter])

  useEffect(() => {
    refresh()
  }, [refresh])

  useEffect(() => {
    if (!selectedId) {
      setDetail(null)
      return
    }
    let cancelled = false
    api
      .getProjectDetail(selectedId)
      .then((res) => {
        if (!cancelled && res?.ok) setDetail(res)
      })
      .catch(() => setDetail(null))
    return () => {
      cancelled = true
    }
  }, [selectedId])

  const project = detail?.project ?? null

  useEffect(() => {
    if (!project) {
      setEditName('')
      setEditStatus('')
      setEditRepo('')
      setEditLocalPath('')
      setEditReadme('')
      setEditMilestone('')
      setEditNotes('')
      return
    }
    setEditName(project.name)
    setEditStatus(project.status)
    setEditRepo(project.repo_url ?? '')
    setEditLocalPath(project.local_path ?? '')
    setEditReadme(project.readme ?? '')
    setEditMilestone(project.milestone ?? '')
    setEditNotes(project.notes ?? '')
  }, [project])


  const toggleMilestone = async (m: ProjectMilestone) => {
    await run(async () => {
      const res = await api.upsertProjectMilestone({
        id: m.id,
        project_id: m.project_id,
        name: m.name,
        is_complete: !m.is_complete,
        due_date: m.due_date,
        notes: m.notes,
      })
      if (!res.ok) throw new Error('Failed to toggle milestone')
    })
  }

  const toggleTask = async (t: ProjectTask) => {
    await run(async () => {
      const res = await api.upsertProjectTask({
        id: t.id,
        project_id: t.project_id,
        title: t.title,
        status: t.status === 'done' ? 'todo' : 'done',
        due_date: t.due_date,
        notes: t.notes,
      })
      if (!res.ok) throw new Error('Failed to toggle task')
    })
  }

  const saveProject = async () => {
    if (!npName) return
    await run(async () => {
      const req: UpsertProjectRequest = {
        track: npTrack,
        name: npName,
        slug: npSlug || npName,
        status: npStatus,
        repo_url: npRepo || undefined,
        local_path: npLocalPath || undefined,
      }
      const res = await api.upsertProject(req)
      if (!res.ok) throw new Error(res?.toString?.() ?? 'Failed to create project')
      setNpName('')
      setNpSlug('')
      setNpRepo('')
      setNpLocalPath('')
      setShowAddProject(false)
      setMessage('Project created')
    })
  }

  const saveSelectedEdit = async () => {
    if (!project) return
    await run(async () => {
      const req: UpsertProjectRequest = {
        id: project.id,
        track: project.track,
        name: editName,
        slug: project.slug,
        status: editStatus,
        repo_url: editRepo || undefined,
        local_path: editLocalPath || undefined,
        readme: editReadme || undefined,
        milestone: editMilestone || undefined,
        notes: editNotes || undefined,
      }
      const res = await api.upsertProject(req)
      if (!res.ok) throw new Error('Failed to save project')
      setMessage('Project saved')
    })
  }

  const addMilestone = async () => {
    if (!project || !nmName) return
    await run(async () => {
      const req: UpsertProjectMilestoneRequest = {
        project_id: project.id,
        name: nmName,
        due_date: nmDue || undefined,
      }
      const res = await api.upsertProjectMilestone(req)
      if (!res.ok) throw new Error('Failed to add milestone')
      setNmName('')
      setNmDue('')
    })
  }

  const addTask = async () => {
    if (!project || !ntTitle) return
    await run(async () => {
      const req: UpsertProjectTaskRequest = {
        project_id: project.id,
        title: ntTitle,
        status: ntStatus,
        due_date: ntDue || undefined,
      }
      const res = await api.upsertProjectTask(req)
      if (!res.ok) throw new Error('Failed to add task')
      setNtTitle('')
      setNtStatus('todo')
      setNtDue('')
    })
  }

  const addLink = async () => {
    if (!project || !nlTitle || !nlUrl) return
    await run(async () => {
      const req: UpsertProjectLinkRequest = {
        project_id: project.id,
        title: nlTitle,
        url: nlUrl,
        link_type: nlType,
      }
      const res = await api.upsertProjectLink(req)
      if (!res.ok) throw new Error('Failed to add link')
      setNlTitle('')
      setNlUrl('')
    })
  }

  const addNote = async () => {
    if (!project || !nnTitle || !nnContent) return
    await run(async () => {
      const req: UpsertProjectNoteRequest = {
        project_id: project.id,
        kind: nnKind,
        title: nnTitle,
        content: nnContent,
      }
      const res = await api.upsertProjectNote(req)
      if (!res.ok) throw new Error('Failed to add note')
      setNnTitle('')
      setNnContent('')
    })
  }

  const btn =
    'rounded-md bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50'
  const ghost = 'rounded-md border border-border px-3 py-1.5 text-sm hover:bg-muted'
  const input = 'rounded-md border border-border bg-background px-3 py-2 text-sm'

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <h2 className="text-2xl font-bold">Projects & FDE Portfolio</h2>
          <p className="text-muted-foreground">
            Track FDE projects, milestones, tasks, links and a typed notes journal (evaluations,
            security, reliability, cost/latency, walkthroughs, evidence).
          </p>
        </div>
        <div className="flex flex-wrap items-center gap-2">
          <select value={trackFilter} onChange={(e) => setTrackFilter(e.target.value)} className={input}>
            {TRACKS.map((t) => (
              <option key={t.key} value={t.key}>
                {t.name}
              </option>
            ))}
          </select>
          <button
            onClick={() => run(async () => { await api.seedFdeProject() })}
            disabled={loading}
            className={btn}
          >
            Seed FDE project
          </button>
          <button
            onClick={() => setShowAddProject(true)}
            disabled={loading}
            className={btn}
          >
            Add project
          </button>
        </div>
      </div>

      {showAddProject && (
        <section className="rounded-lg border border-border p-4">
          <h3 className="mb-3 font-semibold">New project</h3>
          <div className="grid gap-2 sm:grid-cols-4">
            <div className="flex flex-col gap-1">
              <label className="text-xs font-semibold">Track</label>
              <select
                value={npTrack}
                onChange={(e) => setNpTrack(e.target.value)}
                className={input}
              >
                {TRACKS.map((t) => (
                  <option key={t.key} value={t.key}>
                    {t.name}
                  </option>
                ))}
              </select>
            </div>
            <div className="flex flex-col gap-1 sm:col-span-2">
              <label className="text-xs font-semibold">Name</label>
              <input value={npName} onChange={(e) => setNpName(e.target.value)} placeholder="e.g. RAG support agent" className={input} />
            </div>
            <div className="flex flex-col gap-1">
              <label className="text-xs font-semibold">Slug</label>
              <input value={npSlug} onChange={(e) => setNpSlug(e.target.value)} placeholder="auto" className={input} />
            </div>
            <div className="flex flex-col gap-1">
              <label className="text-xs font-semibold">Status</label>
              <select value={npStatus} onChange={(e) => setNpStatus(e.target.value)} className={input}>
                <option value="active">Active</option>
                <option value="planned">Planned</option>
                <option value="on_hold">On hold</option>
                <option value="done">Done</option>
              </select>
            </div>
            <div className="flex flex-col gap-1 sm:col-span-2">
              <label className="text-xs font-semibold">Repo URL</label>
              <input value={npRepo} onChange={(e) => setNpRepo(e.target.value)} placeholder="https://github.com/user/repo" className={input} />
            </div>
            <div className="flex flex-col gap-1 sm:col-span-2">
              <label className="text-xs font-semibold">Local path</label>
              <input value={npLocalPath} onChange={(e) => setNpLocalPath(e.target.value)} placeholder="content/projects/..." className={input} />
            </div>
          </div>
          <div className="mt-3 flex gap-2">
            <button onClick={() => run(saveProject)} disabled={loading || !npName} className={btn}>
              Create
            </button>
            <button onClick={() => setShowAddProject(false)} className={ghost}>
              Cancel
            </button>
          </div>
        </section>
      )}

      <div className="rounded-lg border border-border overflow-hidden">
        <table className="w-full text-left">
          <thead>
            <tr className="border-b bg-muted/50">
              <th className="px-4 py-2 text-sm font-medium">Name</th>
              <th className="px-4 py-2 text-sm font-medium">Track</th>
              <th className="px-4 py-2 text-sm font-medium">Status</th>
              <th className="px-4 py-2 text-sm font-medium">Slug</th>
              <th className="px-4 py-2 text-sm font-medium">Repo</th>
              <th className="px-4 py-2 text-sm font-medium">Actions</th>
            </tr>
          </thead>
          <tbody className="divide-y">
            {projects.length === 0 ? (
              <tr>
                <td colSpan={6} className="px-4 py-6 text-center text-sm text-muted-foreground">
                  No projects. Create one or seed the FDE project.
                </td>
              </tr>
            ) : (
              projects.map((p) => (
                <tr key={p.id} className="hover:bg-muted/30">
                  <td className="px-4 py-3">
                    <button
                      onClick={() => setSelectedId(p.id === selectedId ? null : p.id)}
                      className={`text-sm font-medium ${selectedId === p.id ? 'text-primary' : 'text-foreground hover:underline'}`}
                    >
                      {p.name}
                    </button>
                  </td>
                  <td className="px-4 py-3 text-sm text-muted-foreground">{TRACKS.find((t) => t.key === p.track)?.name ?? p.track}</td>
                  <td className="px-4 py-3 text-sm">
                    <span className="rounded bg-muted px-2 py-0.5 text-xs text-muted-foreground">{p.status}</span>
                  </td>
                  <td className="px-4 py-3 text-sm text-muted-foreground truncate max-w-[120px]">{p.slug}</td>
                  <td className="px-4 py-3 text-sm text-muted-foreground truncate max-w-[140px]">{p.repo_url ?? '—'}</td>
                  <td className="px-4 py-3 text-sm">
                    {p.local_path && (
                      <button
                        onClick={() => {
                          if (p.local_path) {
                            void api.openExternalUrl(p.local_path)
                          }
                        }}
                        className="text-xs text-muted-foreground hover:underline"
                      >
                        Open path
                      </button>
                    )}
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {project && (
        <section className="rounded-lg border border-border p-4">
          <div className="flex flex-wrap items-center justify-between gap-3 mb-4">
            <div>
              <h3 className="font-semibold">{project.name}</h3>
              <p className="text-xs text-muted-foreground">
                {project.track} · {project.slug} · {project.status}
              </p>
            </div>
            <div className="flex gap-2">
              <button onClick={() => setShowAddProject(true)} className={ghost}>Add another</button>
            </div>
          </div>

          <div className="grid gap-3 sm:grid-cols-4 mb-4">
            <div className="flex flex-col gap-1">
              <label className="text-xs font-semibold">Name</label>
              <input value={editName} onChange={(e) => setEditName(e.target.value)} className={input} />
            </div>
            <div className="flex flex-col gap-1">
              <label className="text-xs font-semibold">Status</label>
              <select value={editStatus} onChange={(e) => setEditStatus(e.target.value)} className={input}>
                <option value="active">Active</option>
                <option value="planned">Planned</option>
                <option value="on_hold">On hold</option>
                <option value="done">Done</option>
              </select>
            </div>
            <div className="flex flex-col gap-1 sm:col-span-2">
              <label className="text-xs font-semibold">Repo URL</label>
              <input value={editRepo} onChange={(e) => setEditRepo(e.target.value)} className={input} />
            </div>
            <div className="flex flex-col gap-1 sm:col-span-2">
              <label className="text-xs font-semibold">Local path</label>
              <input value={editLocalPath} onChange={(e) => setEditLocalPath(e.target.value)} className={input} />
            </div>
            <div className="flex flex-col gap-1 sm:col-span-2">
              <label className="text-xs font-semibold">Readme</label>
              <input value={editReadme} onChange={(e) => setEditReadme(e.target.value)} className={input} />
            </div>
            <div className="flex flex-col gap-1">
              <label className="text-xs font-semibold">Milestone</label>
              <input value={editMilestone} onChange={(e) => setEditMilestone(e.target.value)} placeholder="e.g. M4" className={input} />
            </div>
            <div className="flex flex-col gap-1 sm:col-span-2">
              <label className="text-xs font-semibold">Notes</label>
              <input value={editNotes} onChange={(e) => setEditNotes(e.target.value)} className={input} />
            </div>
          </div>
          <div className="mb-4">
            <button onClick={() => run(saveSelectedEdit)} disabled={loading} className={btn}>
              Save project
            </button>
          </div>

          <h4 className="font-semibold mb-2">Milestones</h4>
          <div className="flex flex-wrap gap-2 mb-3">
            <input value={nmName} onChange={(e) => setNmName(e.target.value)} placeholder="Milestone name" className={`${input} flex-1 min-w-[150px]`} />
            <input value={nmDue} onChange={(e) => setNmDue(e.target.value)} type="date" className={input} />
            <button onClick={() => run(addMilestone)} disabled={loading || !nmName} className={btn}>Add milestone</button>
          </div>
          <table className="w-full text-left divide-y mb-6">
            <thead>
              <tr className="border-b text-xs text-muted-foreground">
                <th className="px-4 py-2 font-medium">Name</th>
                <th className="px-4 py-2 font-medium">Done</th>
                <th className="px-4 py-2 font-medium">Due</th>
                <th className="px-4 py-2 font-medium">Notes</th>
              </tr>
            </thead>
            <tbody>
              {(detail?.milestones ?? []).map((m) => (
                <tr key={m.id} className="hover:bg-muted/30">
                  <td className="px-4 py-2 text-sm">{m.name}</td>
                  <td className="px-4 py-2">
                    <input
                      type="checkbox"
                      checked={m.is_complete}
                      onChange={() => run(() => toggleMilestone(m))}
                      disabled={loading}
                      className="rounded border border-border"
                    />
                  </td>
                  <td className="px-4 py-2 text-sm text-muted-foreground">{m.due_date ?? '—'}</td>
                  <td className="px-4 py-2 text-sm text-muted-foreground truncate max-w-[200px]">{m.notes ?? '—'}</td>
                </tr>
              ))}
            </tbody>
          </table>

          <h4 className="font-semibold mb-2">Tasks</h4>
          <div className="flex flex-wrap gap-2 mb-3">
            <input value={ntTitle} onChange={(e) => setNtTitle(e.target.value)} placeholder="Task title" className={`${input} flex-1 min-w-[150px]`} />
            <select value={ntStatus} onChange={(e) => setNtStatus(e.target.value)} className={input}>
              <option value="todo">Todo</option>
              <option value="in_progress">In progress</option>
              <option value="done">Done</option>
            </select>
            <input value={ntDue} onChange={(e) => setNtDue(e.target.value)} type="date" className={input} />
            <button onClick={() => run(addTask)} disabled={loading || !ntTitle} className={btn}>Add task</button>
          </div>
          <table className="w-full text-left divide-y mb-6">
            <thead>
              <tr className="border-b text-xs text-muted-foreground">
                <th className="px-4 py-2 font-medium">Title</th>
                <th className="px-4 py-2 font-medium">Status</th>
                <th className="px-4 py-2 font-medium">Due</th>
              </tr>
            </thead>
            <tbody>
              {(detail?.tasks ?? []).map((t) => (
                <tr key={t.id} className="hover:bg-muted/30">
                  <td className="px-4 py-2 text-sm">{t.title}</td>
                  <td className="px-4 py-2">
                    <input
                      type="checkbox"
                      checked={t.status === 'done'}
                      onChange={() => run(() => toggleTask(t))}
                      disabled={loading}
                      className="rounded border border-border"
                    />
                    <span className="ml-1 text-xs text-muted-foreground">{t.status}</span>
                  </td>
                  <td className="px-4 py-2 text-sm text-muted-foreground">{t.due_date ?? '—'}</td>
                </tr>
              ))}
            </tbody>
          </table>

          <h4 className="font-semibold mb-2">Links</h4>
          <div className="flex flex-wrap gap-2 mb-3">
            <input value={nlTitle} onChange={(e) => setNlTitle(e.target.value)} placeholder="Title" className={`${input} min-w-[140px]`} />
            <input value={nlUrl} onChange={(e) => setNlUrl(e.target.value)} placeholder="URL" className={`${input} flex-1 min-w-[200px]`} />
            <select value={nlType} onChange={(e) => setNlType(e.target.value)} className={input}>
              <option value="doc">Doc</option>
              <option value="diagram">Diagram</option>
              <option value="code">Code</option>
              <option value="test">Test</option>
            </select>
            <button onClick={() => run(addLink)} disabled={loading || !nlTitle || !nlUrl} className={btn}>Add link</button>
          </div>
          <table className="w-full text-left divide-y mb-6">
            <thead>
              <tr className="border-b text-xs text-muted-foreground">
                <th className="px-4 py-2 font-medium">Title</th>
                <th className="px-4 py-2 font-medium">URL</th>
                <th className="px-4 py-2 font-medium">Type</th>
              </tr>
            </thead>
            <tbody>
              {(detail?.links ?? []).map((l) => (
                <tr key={l.id} className="hover:bg-muted/30">
                  <td className="px-4 py-2 text-sm">{l.title}</td>
                  <td className="px-4 py-2 text-sm text-muted-foreground truncate max-w-[220px]">{l.url}</td>
                  <td className="px-4 py-2 text-sm text-muted-foreground">{l.link_type ?? 'doc'}</td>
                </tr>
              ))}
            </tbody>
          </table>

          <h4 className="font-semibold mb-2">Notes journal</h4>
          <div className="flex flex-wrap gap-2 mb-3">
            <select value={nnKind} onChange={(e) => setNnKind(e.target.value)} className={input}>
              {NOTE_KINDS.map((k) => (
                <option key={k} value={k}>
                  {k[0].toUpperCase() + k.slice(1)}
                </option>
              ))}
            </select>
            <input value={nnTitle} onChange={(e) => setNnTitle(e.target.value)} placeholder="Title" className={`${input} min-w-[140px]`} />
            <textarea value={nnContent} onChange={(e) => setNnContent(e.target.value)} placeholder="Content..." rows={2} className={`${input} flex-1 min-w-[200px]`} />
            <button onClick={() => run(addNote)} disabled={loading || !nnTitle || !nnContent} className={btn}>
              Add note
            </button>
          </div>
          <div className="space-y-2">
            {(detail?.notes ?? []).map((n) => (
              <div key={n.id} className="p-3 rounded border border-border bg-muted/40">
                <div className="flex justify-between gap-2 mb-1">
                  <span className="text-xs font-semibold">{n.kind}</span>
                  <span className="text-xs text-muted-foreground">{n.title}</span>
                </div>
                <div className="text-sm text-foreground whitespace-pre-wrap break-words">{n.content}</div>
              </div>
            ))}
          </div>
        </section>
      )}

      {message && (
        <div className="rounded-lg border border-green-500/40 bg-green-500/10 p-3 text-sm text-green-700 dark:text-green-300">
          {message}
        </div>
      )}
      {error && (
        <div className="rounded-lg border border-destructive/50 bg-destructive/10 p-3 text-sm text-destructive">
          {error}
        </div>
      )}
    </div>
  )
}
