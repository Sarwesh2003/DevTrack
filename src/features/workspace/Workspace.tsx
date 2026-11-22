import { useEffect, useState } from 'react'
import { api } from '@/services/api'
import type {
  Track,
  Topic,
  CodeExperiment,
  UpsertCodeExperimentRequest,
  RecallQuestion,
  UpsertRecallQuestionRequest,
  Diagram,
  UpsertDiagramRequest,
  DesignArtifact,
  UpsertDesignArtifactRequest,
  CaseStudy,
  UpsertCaseStudyRequest,
  TopicLink,
  AddTopicLinkRequest,
} from '@/types'

type Tab = 'experiment' | 'recall' | 'diagram' | 'design' | 'case' | 'links'

type Page = 'dashboard' | 'settings' | 'roadmap' | 'review' | 'dsa' | 'workspace' | 'projects' | 'analytics' | 'search'

const TAB_LABELS: Record<Tab, string> = {
  experiment: 'Experiment',
  recall: 'Recall',
  diagram: 'Diagram',
  design: 'Design (LLD)',
  case: 'Case Study (HLD)',
  links: 'Links',
}

export default function Workspace(_props: { onNavigate: (page: Page) => void }) {
  const [tracks, setTracks] = useState<Track[]>([])
  const [topics, setTopics] = useState<Topic[]>([])
  const [selectedTrack, setSelectedTrack] = useState('')
  const [selectedTopic, setSelectedTopic] = useState<Topic | null>(null)
  const [tab, setTab] = useState<Tab>('experiment')
  const [loading, setLoading] = useState(false)
  const [message, setMessage] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)

  const [experiment, setExperiment] = useState<CodeExperiment | null>(null)
  const [expTitle, setExpTitle] = useState('')
  const [expCode, setExpCode] = useState('')
  const [expLang, setExpLang] = useState('java')

  const [questions, setQuestions] = useState<RecallQuestion[]>([])
  const [newQ, setNewQ] = useState('')
  const [newA, setNewA] = useState('')
  const [newQDiff, setNewQDiff] = useState('medium')

  const [diagram, setDiagram] = useState<Diagram | null>(null)
  const [diaTitle, setDiaTitle] = useState('')
  const [diaContent, setDiaContent] = useState('')

  const [design, setDesign] = useState<DesignArtifact | null>(null)
  const [desTitle, setDesTitle] = useState('')
  const [desReq, setDesReq] = useState('')
  const [desApi, setDesApi] = useState('')
  const [desJava, setDesJava] = useState('')
  const [desTests, setDesTests] = useState('')
  const [desTrade, setDesTrade] = useState('')

  const [study, setStudy] = useState<CaseStudy | null>(null)
  const [stuTitle, setStuTitle] = useState('')
  const [stuContext, setStuContext] = useState('')
  const [stuDiagram, setStuDiagram] = useState('')
  const [stuTrade, setStuTrade] = useState('')

  const [links, setLinks] = useState<TopicLink[]>([])
  const [linkTarget, setLinkTarget] = useState('')
  const [linkType, setLinkType] = useState('related')

  const run = async (fn: () => Promise<void>) => {
    setLoading(true)
    setError(null)
    setMessage(null)
    try {
      await fn()
      if (selectedTopic) await loadTopicData(selectedTopic.id)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Action failed')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    api
      .getTracks()
      .then((res) => {
        if (res.ok) setTracks(res.tracks)
      })
      .catch(() => setError('Failed to load tracks'))
  }, [])

  useEffect(() => {
    if (!selectedTrack) {
      setTopics([])
      return
    }
    let cancelled = false
    api
      .getModules(selectedTrack)
      .then(async (modRes) => {
        if (!modRes.ok) return
        const lists = await Promise.all(modRes.modules.map((m) => api.getTopics(m.id)))
        if (cancelled) return
        setTopics(lists.flatMap((r) => (r.ok ? r.topics : [])))
      })
      .catch(() => setError('Failed to load topics'))
    return () => {
      cancelled = true
    }
  }, [selectedTrack])

  useEffect(() => {
    if (!selectedTopic) return
    loadTopicData(selectedTopic.id).catch(() => setError('Failed to load workspace data'))
  }, [selectedTopic])

  const loadTopicData = async (topicId: string) => {
    const [expRes, qRes, diaRes, desRes, stuRes, linkRes] = await Promise.all([
      api.listCodeExperiments(topicId).catch(() => null),
      api.listRecallQuestions(topicId).catch(() => null),
      api.listDiagrams(topicId).catch(() => null),
      api.listDesignArtifacts(topicId).catch(() => null),
      api.listCaseStudies(topicId).catch(() => null),
      api.listTopicLinks(topicId).catch(() => null),
    ])
    const exp = expRes?.ok ? expRes.experiments[0] ?? null : null
    setExperiment(exp)
    setExpTitle(exp?.title ?? '')
    setExpCode(exp?.code ?? '')
    setExpLang(exp?.language ?? 'java')
    setQuestions(qRes?.ok ? qRes.questions : [])
    const dia = diaRes?.ok ? diaRes.diagrams[0] ?? null : null
    setDiagram(dia)
    setDiaTitle(dia?.title ?? '')
    setDiaContent(dia?.mermaid_content ?? '')
    const des = desRes?.ok ? desRes.artifacts[0] ?? null : null
    setDesign(des)
    setDesTitle(des?.title ?? '')
    setDesReq(des?.requirements ?? '')
    setDesApi(des?.api_spec ?? '')
    setDesJava(des?.java_implementation ?? '')
    setDesTests(des?.tests ?? '')
    setDesTrade(des?.trade_offs ?? '')
    const stu = stuRes?.ok ? stuRes.case_studies[0] ?? null : null
    setStudy(stu)
    setStuTitle(stu?.title ?? '')
    setStuContext(stu?.context ?? '')
    setStuDiagram(stu?.diagram_content ?? '')
    setStuTrade(stu?.trade_offs ?? '')
    setLinks(linkRes?.ok ? linkRes.links : [])
  }

  const saveExperiment = async () => {
    if (!selectedTopic) return
    const req: UpsertCodeExperimentRequest = {
      id: experiment?.id,
      topic_id: selectedTopic.id,
      title: expTitle || 'Experiment',
      code: expCode,
      language: expLang,
    }
    const res = await api.upsertCodeExperiment(req)
    if (!res.ok) throw new Error('Failed to save experiment')
    setMessage('Experiment saved')
  }

  const saveRecall = async () => {
    if (!selectedTopic || !newQ || !newA) return
    const req: UpsertRecallQuestionRequest = {
      topic_id: selectedTopic.id,
      question: newQ,
      answer: newA,
      difficulty: newQDiff,
    }
    const res = await api.upsertRecallQuestion(req)
    if (!res.ok) throw new Error('Failed to save question')
    setNewQ('')
    setNewA('')
    setMessage('Question saved')
  }

  const reviewRecall = async (id: string, result: string) => {
    const res = await api.recordRecallReview({ id, recall_result: result })
    if (!res.ok) throw new Error('Failed to record review')
  }

  const saveDiagram = async () => {
    if (!selectedTopic) return
    const req: UpsertDiagramRequest = {
      id: diagram?.id,
      topic_id: selectedTopic.id,
      title: diaTitle || 'Diagram',
      mermaid_content: diaContent,
    }
    const res = await api.upsertDiagram(req)
    if (!res.ok) throw new Error('Failed to save diagram')
    setMessage('Diagram saved')
  }

  const saveDesign = async () => {
    if (!selectedTopic) return
    const req: UpsertDesignArtifactRequest = {
      id: design?.id,
      topic_id: selectedTopic.id,
      title: desTitle || 'Design',
      requirements: desReq,
      api_spec: desApi,
      java_implementation: desJava,
      tests: desTests,
      trade_offs: desTrade,
    }
    const res = await api.upsertDesignArtifact(req)
    if (!res.ok) throw new Error('Failed to save design')
    setMessage('Design saved')
  }

  const saveCaseStudy = async () => {
    if (!selectedTopic) return
    const req: UpsertCaseStudyRequest = {
      id: study?.id,
      topic_id: selectedTopic.id,
      title: stuTitle || 'Case Study',
      context: stuContext,
      diagram_content: stuDiagram,
      trade_offs: stuTrade,
    }
    const res = await api.upsertCaseStudy(req)
    if (!res.ok) throw new Error('Failed to save case study')
    setMessage('Case study saved')
  }

  const addLink = async () => {
    if (!selectedTopic || !linkTarget) return
    const req: AddTopicLinkRequest = {
      from_topic_id: selectedTopic.id,
      to_topic_id: linkTarget,
      link_type: linkType,
    }
    const res = await api.addTopicLink(req)
    if (!res.ok) throw new Error(res.error ?? 'Failed to add link')
    setLinkTarget('')
    setMessage('Link added')
  }

  const btn =
    'rounded-md bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50'
  const ghost = 'rounded-md border border-border px-3 py-1.5 text-sm hover:bg-muted'
  const input = 'rounded-md border border-border bg-background px-3 py-2 text-sm'

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <h2 className="text-2xl font-bold">Learning Workspace</h2>
          <p className="text-muted-foreground">
            Notes, experiments, diagrams, designs and case studies for curriculum topics.
          </p>
        </div>
        <div className="flex flex-wrap items-center gap-2">
          <select
            value={selectedTrack}
            onChange={(e) => {
              setSelectedTrack(e.target.value)
              setSelectedTopic(null)
            }}
            className={input}
          >
            <option value="">Select track</option>
            {tracks.map((t) => (
              <option key={t.id} value={t.key}>
                {t.name}
              </option>
            ))}
          </select>
          <select
            disabled={!selectedTrack}
            value={selectedTopic?.id ?? ''}
            onChange={(e) => setSelectedTopic(topics.find((t) => t.id === e.target.value) ?? null)}
            className={input}
          >
            <option value="">Select topic</option>
            {topics.map((t) => (
              <option key={t.id} value={t.id}>
                {t.key} — {t.title}
              </option>
            ))}
          </select>
        </div>
      </div>

      {!selectedTopic && (
        <div className="rounded-lg border border-border bg-muted/50 p-4 text-center text-sm text-muted-foreground">
          Select a track and topic to start working.
        </div>
      )}

      {selectedTopic && (
        <>
          <div className="flex flex-wrap items-center gap-1">
            {(Object.keys(TAB_LABELS) as Tab[]).map((t) => (
              <button
                key={t}
                onClick={() => setTab(t)}
                className={`rounded-md border px-3 py-1.5 text-sm transition-colors ${
                  tab === t
                    ? 'bg-primary text-primary-foreground'
                    : 'border-border text-muted-foreground hover:bg-muted'
                }`}
              >
                {TAB_LABELS[t]}
              </button>
            ))}
          </div>

          {tab === 'experiment' && (
            <section className="rounded-lg border border-border p-4">
              <h3 className="mb-2 font-semibold">Code Experiment</h3>
              <div className="mb-2 flex gap-2">
                <input
                  value={expTitle}
                  onChange={(e) => setExpTitle(e.target.value)}
                  placeholder="Experiment title"
                  className={`${input} flex-1`}
                />
                <select value={expLang} onChange={(e) => setExpLang(e.target.value)} className={input}>
                  <option value="java">Java</option>
                  <option value="txt">Text</option>
                </select>
              </div>
              <textarea
                value={expCode}
                onChange={(e) => setExpCode(e.target.value)}
                placeholder="Write Java code here..."
                className="min-h-[300px] w-full rounded-md border border-border bg-background px-3 py-2 font-mono text-sm"
              />
              <div className="mt-2 flex gap-2">
                <button onClick={() => run(saveExperiment)} disabled={loading} className={btn}>
                  Save experiment
                </button>
                {experiment && (
                  <button onClick={() => setExpCode(experiment.code)} className={ghost}>
                    Reload saved
                  </button>
                )}
              </div>
              {experiment && (
                <div className="mt-3 text-xs text-muted-foreground">
                  Saved {new Date(experiment.updated_at).toLocaleString()}
                </div>
              )}
            </section>
          )}

          {tab === 'recall' && (
            <section className="rounded-lg border border-border p-4">
              <h3 className="mb-2 font-semibold">Recall Questions</h3>
              <div className="mb-3 flex flex-wrap gap-2">
                <input
                  value={newQ}
                  onChange={(e) => setNewQ(e.target.value)}
                  placeholder="Question"
                  className={`${input} flex-1`}
                />
                <input
                  value={newA}
                  onChange={(e) => setNewA(e.target.value)}
                  placeholder="Answer"
                  className={`${input} flex-1`}
                />
                <select value={newQDiff} onChange={(e) => setNewQDiff(e.target.value)} className={input}>
                  <option value="easy">Easy</option>
                  <option value="medium">Medium</option>
                  <option value="hard">Hard</option>
                </select>
                <button onClick={() => run(saveRecall)} disabled={loading || !newQ || !newA} className={btn}>
                  Add
                </button>
              </div>
              {questions.length === 0 ? (
                <p className="text-sm text-muted-foreground">No questions yet. Add one above.</p>
              ) : (
                <ul className="divide-y divide-border">
                  {questions.map((q) => (
                    <li key={q.id} className="py-3">
                      <div className="mb-1 flex justify-between gap-2">
                        <span className="text-sm font-medium">{q.question}</span>
                        <span className="rounded bg-muted px-1.5 py-0.5 text-xs text-muted-foreground">
                          {q.difficulty}
                        </span>
                      </div>
                      <div className="mb-2 text-sm text-muted-foreground">{q.answer}</div>
                      <div className="flex flex-wrap items-center gap-1">
                        {['again', 'hard', 'good', 'easy'].map((r) => (
                          <button
                            key={r}
                            onClick={() => run(() => reviewRecall(q.id, r))}
                            disabled={loading}
                            className="rounded-md border border-border px-2 py-1 text-xs hover:bg-muted disabled:opacity-50"
                          >
                            {r}
                          </button>
                        ))}
                        <span className="ml-auto text-xs text-muted-foreground">{q.state}</span>
                      </div>
                    </li>
                  ))}
                </ul>
              )}
            </section>
          )}

          {tab === 'diagram' && (
            <section className="rounded-lg border border-border p-4">
              <h3 className="mb-2 font-semibold">Diagram (Mermaid)</h3>
              <input
                value={diaTitle}
                onChange={(e) => setDiaTitle(e.target.value)}
                placeholder="Diagram title"
                className={`${input} mb-2 w-full`}
              />
              <textarea
                value={diaContent}
                onChange={(e) => setDiaContent(e.target.value)}
                placeholder={'graph TD\n  A[Start] --> B[End]'}
                className="min-h-[250px] w-full rounded-md border border-border bg-background px-3 py-2 font-mono text-sm"
              />
              <div className="mt-2">
                <button onClick={() => run(saveDiagram)} disabled={loading} className={btn}>
                  Save diagram
                </button>
              </div>
              <p className="mt-3 text-xs text-muted-foreground">
                Paste the Mermaid source into mermaid.live to preview.
              </p>
            </section>
          )}

          {tab === 'design' && (
            <section className="rounded-lg border border-border p-4">
              <h3 className="mb-2 font-semibold">Design Artifact (LLD)</h3>
              <input
                value={desTitle}
                onChange={(e) => setDesTitle(e.target.value)}
                placeholder="Design title"
                className={`${input} mb-3 w-full`}
              />
              <div className="grid gap-3 sm:grid-cols-2">
                <label className="flex flex-col gap-1 text-xs font-semibold">
                  Requirements
                  <textarea
                    value={desReq}
                    onChange={(e) => setDesReq(e.target.value)}
                    className="min-h-[120px] rounded-md border border-border bg-background px-3 py-2 text-sm font-normal"
                  />
                </label>
                <label className="flex flex-col gap-1 text-xs font-semibold">
                  API Spec
                  <textarea
                    value={desApi}
                    onChange={(e) => setDesApi(e.target.value)}
                    className="min-h-[120px] rounded-md border border-border bg-background px-3 py-2 text-sm font-normal"
                  />
                </label>
              </div>
              <label className="mt-3 flex flex-col gap-1 text-xs font-semibold">
                Java Implementation
                <textarea
                  value={desJava}
                  onChange={(e) => setDesJava(e.target.value)}
                  className="min-h-[150px] rounded-md border border-border bg-background px-3 py-2 font-mono text-sm font-normal"
                />
              </label>
              <div className="mt-3 grid gap-3 sm:grid-cols-2">
                <label className="flex flex-col gap-1 text-xs font-semibold">
                  Tests
                  <textarea
                    value={desTests}
                    onChange={(e) => setDesTests(e.target.value)}
                    className="min-h-[100px] rounded-md border border-border bg-background px-3 py-2 font-mono text-sm font-normal"
                  />
                </label>
                <label className="flex flex-col gap-1 text-xs font-semibold">
                  Trade-offs
                  <textarea
                    value={desTrade}
                    onChange={(e) => setDesTrade(e.target.value)}
                    className="min-h-[100px] rounded-md border border-border bg-background px-3 py-2 text-sm font-normal"
                  />
                </label>
              </div>
              <div className="mt-3">
                <button onClick={() => run(saveDesign)} disabled={loading} className={btn}>
                  Save design
                </button>
              </div>
            </section>
          )}

          {tab === 'case' && (
            <section className="rounded-lg border border-border p-4">
              <h3 className="mb-2 font-semibold">Case Study (HLD)</h3>
              <input
                value={stuTitle}
                onChange={(e) => setStuTitle(e.target.value)}
                placeholder="Case study title"
                className={`${input} mb-3 w-full`}
              />
              <label className="flex flex-col gap-1 text-xs font-semibold">
                Context
                <textarea
                  value={stuContext}
                  onChange={(e) => setStuContext(e.target.value)}
                  placeholder="Context and problem statement..."
                  className="min-h-[120px] rounded-md border border-border bg-background px-3 py-2 text-sm font-normal"
                />
              </label>
              <label className="mt-3 flex flex-col gap-1 text-xs font-semibold">
                Architecture Diagram (Mermaid)
                <textarea
                  value={stuDiagram}
                  onChange={(e) => setStuDiagram(e.target.value)}
                  placeholder={'graph TD\n  A[Client] --> B[API]\n  B --> C[DB]'}
                  className="min-h-[150px] rounded-md border border-border bg-background px-3 py-2 font-mono text-sm font-normal"
                />
              </label>
              <label className="mt-3 flex flex-col gap-1 text-xs font-semibold">
                Trade-offs
                <textarea
                  value={stuTrade}
                  onChange={(e) => setStuTrade(e.target.value)}
                  placeholder="Trade-offs and decisions..."
                  className="min-h-[100px] rounded-md border border-border bg-background px-3 py-2 text-sm font-normal"
                />
              </label>
              <div className="mt-3">
                <button onClick={() => run(saveCaseStudy)} disabled={loading} className={btn}>
                  Save case study
                </button>
              </div>
            </section>
          )}

          {tab === 'links' && (
            <section className="rounded-lg border border-border p-4">
              <h3 className="mb-2 font-semibold">Related Topics</h3>
              <div className="mb-4 flex flex-wrap gap-2">
                <input
                  value={linkTarget}
                  onChange={(e) => setLinkTarget(e.target.value)}
                  placeholder="Target topic id"
                  className={`${input} flex-1`}
                />
                <select value={linkType} onChange={(e) => setLinkType(e.target.value)} className={input}>
                  <option value="related">Related</option>
                  <option value="prerequisite">Prerequisite</option>
                  <option value="follows">Follows</option>
                </select>
                <button onClick={() => run(addLink)} disabled={loading || !linkTarget} className={btn}>
                  Add link
                </button>
              </div>
              {links.length === 0 ? (
                <p className="text-sm text-muted-foreground">No links yet.</p>
              ) : (
                <ul className="divide-y divide-border">
                  {links.map((l) => (
                    <li key={l.id} className="flex items-center justify-between py-2">
                      <span className="text-sm">{l.link_type}</span>
                      <span className="rounded bg-muted px-1.5 py-0.5 text-xs text-muted-foreground">
                        {l.from_topic_id} → {l.to_topic_id}
                      </span>
                    </li>
                  ))}
                </ul>
              )}
            </section>
          )}
        </>
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
