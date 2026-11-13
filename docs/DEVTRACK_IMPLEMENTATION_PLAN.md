# DevTrack — Product & Incremental Implementation Plan

**Purpose:** A private, local-first engineering learning workspace for a six-month SDE interview and FDE preparation plan.

**Primary user:** One backend engineer using a Mac.  
**Planning source:** `six_month_engineering_roadmap.md` supplied by the user.  
**Planning horizon:** 26 weeks.  
**Target study capacity:** 7–8 focused hours per week, configurable by the user.

---

## 1. Product vision

DevTrack is not a general-purpose ToDo application. It is a personal technical learning system that connects:

- A structured learning roadmap across Core Java, DSA, LLD, HLD and FDE.
- Daily and weekly learning goals generated from that roadmap.
- Spaced revision of previously learned problems and concepts.
- Personal notes, code, attempts, diagrams and practical outputs.
- Progress, confidence, consistency, streaks and weak-area analytics.

The dashboard must answer:

1. What should I study today?
2. Which revisions are due today, and why?
3. What is the next unfinished topic in each track?
4. Am I on track for this week's plan without accumulating missed-task debt?
5. What have I actually learned, implemented or independently recalled?

### Product principles

1. **Local-first:** Core features work without an internet connection. External integrations are optional.
2. **Roadmap-driven:** The supplied 26-week roadmap is the default curriculum, not a set of hardcoded UI labels.
3. **Progress-aware:** Never assume all tracks start from zero. Respect the user's current DSA and LLD checkpoints.
4. **Evidence over consumption:** Watching a video or reading an article alone does not make a topic complete.
5. **Revision is first-class:** Revision tasks are generated from actual completion/review dates and recall outcomes.
6. **Flexible, not punitive:** Missed tasks are intelligently replanned. A missed day must not double the next day's workload.
7. **Portable data:** Notes and code are normal local files with export and backup support.
8. **No fake data:** Do not invent historical solve dates, attempts, completion states, metrics or external integration results.

## 2. Scope and non-goals

### Included

- Five learning tracks and the 26-week roadmap.
- Daily agenda, weekly goals, learning sessions and custom learning entries.
- DSA question library for the official NeetCode 250 sheet.
- Per-problem attempts, Java solutions, notes, complexity and revision history.
- Topic workspaces for Java, LLD, HLD and FDE.
- LLD/HLD design notes, diagrams, code links and project records.
- Revision engine, completion tracking, activity heatmap, streaks and analytics.
- Local database, filesystem storage, backup, restore and export.
- Optional future GitHub and LeetCode integrations.

### Not in the initial release

- Cloud database, Firebase, Supabase, hosted backend or mandatory deployment.
- Multi-user accounts, social features, leaderboards or collaboration.
- An autonomous AI tutor or automatic solution generator.
- Automatic scraping or login automation against LeetCode.
- A large plugin marketplace or generic productivity/task manager.
- Automatic access to the whole Mac filesystem.

AI assistance may be added later as an optional helper. It must not be required for normal tracking or scheduling.

## 3. Learning plan and current-progress overrides

The attached roadmap contains five tracks, learning units, subtopics, priorities, estimates, dependencies, resources, practical exercises, completion criteria, weekly allocation, 26-week sequencing and revision rules. Preserve that information during import.

### 3.1 DSA — NeetCode 250

**Current checkpoint:** The learner has already covered the official NeetCode categories through Trees. New DSA learning must start at **Tries**, not Arrays & Hashing.

The uploaded roadmap's category counts give the following baseline:

| Official order | Category | Problems | DevTrack treatment |
|---:|---|---:|---|
| 1 | Arrays & Hashing | 22 | Previously covered; revision only |
| 2 | Two Pointers | 13 | Previously covered; revision only |
| 3 | Sliding Window | 9 | Previously covered; revision only |
| 4 | Binary Search | 14 | Previously covered; revision only |
| 5 | Linked List | 14 | Previously covered; revision only |
| 6 | Stack | 14 | Previously covered; revision only |
| 7 | Trees | 23 | Previously covered; revision only |
| 8 | Tries | 4 | **Next new category** |
| 9 | Backtracking | 17 | New learning |
| 10 | Heap / Priority Queue | 12 | New learning |
| 11 | Graphs | 21 | New learning |
| 12 | Advanced Graphs | 10 | New learning |
| 13 | 1-D Dynamic Programming | 17 | New learning |
| 14 | 2-D Dynamic Programming | 16 | New learning |
| 15 | Greedy | 14 | New learning |
| 16 | Intervals | 7 | New learning |
| 17 | Bit Manipulation | 10 | New learning |
| 18 | Math & Geometry | 13 | New learning |

The counts in the uploaded roadmap sum to 250: the first seven categories account for 109 questions and the remaining categories account for 141. Treat these as roadmap planning counts and validate the imported problem dataset before displaying confirmed counts.

**Important status/data rule:** The category checkpoint is known, but individual problem histories and solve dates may not be available. On first setup:

- Mark the first seven categories as `previously_covered` for sequencing purposes.
- Do not fabricate individual `solved_at` dates, attempts, code, or confidence.
- Import exact per-problem statuses when the user supplies an export or marks them in the app.
- Offer a one-time “Start revision baseline now” action for previously covered problems with no historical date. This action should explicitly tell the user that it establishes a new revision baseline; it must not pretend to recover the original solve date.
- Never assign new DSA questions from categories 1–7 unless the user explicitly changes the starting checkpoint.
- Revisions of old questions are allowed and should appear in the revision queue.

**Problem source rules:**

- The official NeetCode 250 sheet is the only source of new questions in this track.
- Preserve exact title, URL, official order, category, difficulty and stable external identifier/slug where available.
- Do not append random LeetCode questions to the NeetCode 250 curriculum.
- The app must support importing a validated CSV/JSON dataset. Do not depend on undocumented scraping to create the initial dataset.
- Keep the imported dataset versioned and record import source/date. The import must report missing, duplicate or malformed records.

**Per-problem workflow:** understand constraints → derive brute force → identify pattern/invariant → independently implement Java solution → analyze complexity → compare/review → reproduce later.

**Solved criteria:** genuine independent attempt, accepted/correct implementation, understood complexity, explainable invariant/recurrence and at least one identified edge case. Reading the solution alone is not completion.

**DSA revision intervals:** next day; day 3–4; day 7; day 14; day 30; day 60–90. The engine may use 60 days as the default final interval and expose 90 days as a configurable long-retention option.

### 3.2 LLD — CodeArmy

**Current checkpoint:** The learner has completed LLD through the **Composite Design Pattern**.

**Previously studied / revision pool:**

- C1: LLD approach, OOP foundations, UML and modeling.
- C2: SOLID and code-quality principles.
- C3: Creational patterns.
- C4: Structural patterns through Composite, including the previously studied patterns in the learner's curriculum.

**Next new topic:** **Bridge Pattern**, followed by the remaining structural-pattern material (including Flyweight where it remains in the course sequence), then behavioral patterns, independent end-to-end LLD projects and machine-coding readiness.

Rules:

- CodeArmy remains the primary LLD video curriculum.
- Preserve the playlist's published order when lecture metadata is available.
- Do not replay C1–C4 from the beginning as new learning. Use active-recall revision tasks for previously studied content.
- The user should attempt each practical design problem before watching its solution.
- The app should support requirements, use cases, entities, class/sequence diagrams, APIs, Java code, tests, trade-offs and retrospective notes.
- Target at least four substantial independently implemented projects and three timed mocks, consistent with the source roadmap.

### 3.3 Core Java

Start from the uploaded roadmap's Java track (A1–A10). It is not a beginner syntax course. Focus on interview depth, runtime behavior, edge cases, trade-offs, code experiments and production Spring Boot context.

Topics include language/object model, collections, Java 8+ features, concurrency/JMM, locks/executors/asynchronous programming, JVM/memory, diagnostics/performance, Spring internals, SQL/transactions/persistence, testing/API engineering and production concerns.

### 3.4 HLD

Start from the beginning (D1–D10): interview method and estimation; networking/request lifecycle; data modeling; caching; messaging; distributed systems; scalability/resilience; observability/security/operations; case studies; senior-level architecture judgment.

### 3.5 FDE / AI Engineering

Start from the beginning (E1–E10): practical Python; LLM APIs; prompting/structured outputs/context; tool calling/workflows; embeddings/RAG; agent orchestration/state; evaluation; guardrails/security; production operations; one coherent AI-enabled project.

Do not begin with a multi-agent system. The roadmap's progression is direct model calls → structured outputs → retrieval or tool use → deterministic workflow → bounded orchestration only when justified.

### 3.6 Cross-track relationships

Import the roadmap's dependencies and cross-track connections. Examples include Java collections with DSA/LLD, concurrency with LLD/HLD/FDE, SQL with HLD/FDE, APIs with HLD/FDE, queues with HLD/Java/FDE, and testing/observability across all tracks. Related topics should link to one another rather than create duplicate curriculum items.

### 3.7 Required adjustment to the original 26-week matrix

The source document's weekly matrix was written from scratch. Its Java, HLD and FDE columns should remain the baseline. Its DSA and LLD columns must be recalculated using the current-progress checkpoints above; do not import early-week DSA/LLD assignments literally where they conflict with those checkpoints.

**DSA weekly remapping:**

- Week 1 starts at Tries. Continue the official NeetCode 250 order from Tries onward, using the exact imported problem sequence.
- Plan roughly 6–8 new problems per week as the starting target, plus 3–5 revisions from previously studied or newly solved items. Lower the new-problem target when the difficulty or understanding warrants it.
- The new-learning cursor progresses through Backtracking, Heap/Priority Queue, Graphs, Advanced Graphs, 1-D DP, 2-D DP, Greedy, Intervals, Bit Manipulation and Math & Geometry.
- The scheduler should determine the actual next problem from problem-level status, not hardcode a category to a calendar week. Category order is a guardrail; the weekly target is a pacing estimate.
- Aim to finish new problems by approximately weeks 20–24 when pace and retention allow. Weeks 25–26 prioritize weak-problem re-solves, timed NeetCode-only sessions and final status review. If the user needs longer, preserve understanding rather than rush.
- Categories 1–7 remain available for scheduled revision, especially weak/failed items, but must not receive new-problem assignments by default.

**LLD weekly remapping:**

- Weeks 1–2: revise OOP/SOLID/creational/structural patterns through Composite; learn Bridge and remaining structural-pattern material (including Flyweight where it remains in the CodeArmy sequence).
- Weeks 3–5: complete the remaining behavioral patterns with independent examples and pattern-selection drills.
- Weeks 6–14: work through CodeArmy end-to-end projects in published order. Target at least four substantial independent implementations; attempt before watching the solution. Use project size and actual availability to spread work across weeks.
- Weeks 15–17: machine-coding readiness, testing, concurrency, extensibility and timed 60–90 minute exercises.
- Weeks 18–26: rebuild the weakest project from blank, practice unfamiliar requirements, run at least three timed LLD mocks and complete final design review.
- This is a planning overlay, not a new curriculum: if the CodeArmy playlist's verified lecture order differs, preserve its actual sequence while keeping Composite as the last previously completed checkpoint.

**Java, HLD and FDE:** Keep their existing week-by-week progression from the source document unless the user explicitly changes it. The application may move tasks within the week based on capacity, but must preserve dependencies and phase outcomes.

## 4. Scheduling rules from the roadmap

### 4.1 Study capacity and default allocation

The initial schedule must use the roadmap's sustainable baseline of 7–8 focused hours per week. These values are editable in Settings.

| Track | Weekly average |
|---|---:|
| DSA — NeetCode 250 | 2.5–3 hours |
| Java interview preparation | 1–1.25 hours |
| LLD — CodeArmy | 1 hour |
| HLD | 1–1.25 hours |
| FDE / AI engineering | 1.5–2 hours |

### 4.2 Default weekly cadence

| Day | Default session | Duration | Expected output |
|---|---|---:|---|
| Monday | NeetCode 250 | 45–60 min | One new problem attempt or deliberate revision |
| Tuesday | Java | 45–60 min | One concept explained plus one code experiment |
| Wednesday | NeetCode 250 | 45–60 min | One problem plus pattern note |
| Thursday | LLD or HLD, alternating weekly | 45–60 min | Diagram, class model or design explanation |
| Friday | NeetCode review + Java recall | 30–45 min | Re-solve/review and answer recall questions |
| Saturday | FDE project + design practice | 2–2.5 hours | Working project increment and/or design exercise |
| Sunday | Weekly review | 30–45 min | Revision queue review, progress update and next-week plan |

The weekly cadence is a default template, not a locked calendar. The learner can change availability, move sessions and set rest days.

### 4.3 Daily agenda rules

A normal study day should contain:

- One primary task (new topic, problem, design exercise or project milestone).
- One revision task due from the revision queue, when due and capacity allows.
- One small tangible output (code, diagram, explanation, test or design decision).
- An optional stretch task only after planned work is done.

Do not schedule all five tracks every day. Normally schedule one or two tracks per day. Typical task durations from the source roadmap:

| Task type | Default duration |
|---|---:|
| Java concept + code experiment | 45–60 min |
| One NeetCode problem | 35–60 min |
| NeetCode revision | 15–30 min |
| LLD lecture + notes | 30–45 min |
| LLD implementation | 45–90 min |
| HLD concept | 30–45 min |
| HLD design exercise | 45–60 min |
| FDE concept + implementation | 45–90 min |
| Weekly review | 30 min |

### 4.4 Capacity and missed-task policy

- Default weekdays: 45–75 minutes; one weekend day: 2–2.5 hours; configurable.
- Keep 10–20% of available time unallocated as a buffer.
- Do not fill the full capacity by default.
- Limit new heavy concepts to one per day.
- Keep revision tasks shorter than new-learning tasks.
- Preserve at least one low-intensity/rest day per week.
- If a day is marked unavailable, move tasks without duplicating them.
- If a task is missed, reschedule based on priority, dependencies, age and capacity; do not automatically move everything to tomorrow.
- A missed day must not create debt or double the next day's workload.
- Weekly review is the point at which the user can accept or modify next week's generated plan.

### 4.5 Task priority heuristic

The source roadmap suggests:

`task_score = priority_weight + overdue_weight + weakness_weight + dependency_weight + track_balance_weight`

Initial weights:

- High priority: +5; Medium: +3; Low: +1.
- Overdue revision: +1 to +4 depending on lateness.
- Weak recall / failed previous attempt: +3.
- Unblocks dependent topic: +2.
- Track balance: +1 when a track has been neglected too long.

This score is a ranking aid only. Prerequisites, explicit user overrides and available daily capacity take precedence. Show the reason for each suggested task in the UI (e.g. `due for revision`, `next in roadmap`, `high priority`, `unblocks topic`).

## 5. Functional requirements

### 5.1 Dashboard

The dashboard is the daily command centre. It must show:

- Date and today's planned study capacity.
- “What to Study Today” agenda grouped by primary learning, revision and optional stretch.
- Track/topic label, expected duration, priority and completion state for each task.
- Today completion progress and weekly-goal progress.
- Revision items due today, overdue count and weak items.
- Current 26-week phase and current week.
- Progress across Java, DSA, LLD, HLD and FDE.
- Current streak and learning activity heatmap.
- A visible `+ Add Learning Entry` action.
- A clear route to the weekly review.

The dashboard must not show fabricated stats or fake examples in production. Empty states should explain how to import a roadmap or add progress.

### 5.2 Roadmap / curriculum

- Import the 26-week roadmap with weeks, phases, track allocations, learning units, subtopics, priorities, estimated hours, dependencies, resources, practical tasks and completion criteria.
- Display an expandable Track → Module → Topic → Subtopic hierarchy.
- Display status using the source roadmap's states: Not started, Learning, Practicing, Needs revision, Interview ready, Mastered.
- Support notes, links, completion evidence, confidence (1–5), estimated time and actual time.
- Support marking subtopics individually; topic completion must be derived from its completion criteria/subtopics, not from opening a resource.
- Support custom topics and editing the imported roadmap without destroying original source metadata.
- Show previous/next dependencies and related cross-track topics.
- Let the user override a topic's priority, target week, estimate and status.
- Changes to the roadmap must not silently erase completion history.

### 5.3 Daily and weekly planner

- Generate a daily agenda from the current week of the roadmap, availability and due revisions.
- Generate a weekly plan with track allocation, daily task estimates and expected outputs.
- Let the user drag/reorder, move, split, postpone, skip or add tasks.
- Mark a task complete with an optional/required output depending on task type.
- Track planned vs actual minutes separately.
- Provide daily and weekly completion percentages.
- Preserve planned tasks separately from revisions and additional learning.
- Replan missed work without duplicating it or increasing the next day's load indiscriminately.
- Allow the user to freeze a week or regenerate the remaining week after a review.

### 5.4 Custom learning entries

The user can record learning that was not assigned by the system. Fields:

- Title and date/time.
- Track or `Unplanned / Other` category.
- Description / what I learned.
- Optional notes or Markdown content.
- Optional resource URL.
- Optional duration.
- Optional output/code/diagram/file link.
- Tags and related roadmap topics/problems/projects.

Custom entries must appear in the activity history and heatmap. They must be visually distinguished from planned roadmap tasks and must not automatically mark roadmap topics complete. The user may explicitly link an entry as evidence for a topic.

### 5.5 Progress and streaks

- Track planned tasks completed, revisions completed, custom learning entries and study sessions separately.
- Show daily, weekly and track-level progress.
- Streaks should be based on meaningful learning activity, not merely opening the application or running a timer.
- Allow the user to configure the minimum meaningful activity rule (e.g. a completed task or saved learning entry with content).
- Keep rest/unavailable days visible without treating them as missed goals.
- Show learning heatmap intensity based on meaningful activity, not raw app launches.
- Do not equate time spent with mastery.

### 5.6 Revision engine

Support revisions for DSA questions and conceptual/design topics.

**Default intervals:** 1, 3, 7, 14, 30, 60 days. For NeetCode, expose the roadmap's final 60–90 day interval as a configurable option.

Each revision record includes:

- Source item (problem/topic/subtopic/design/project).
- Initial completion date if known.
- Last reviewed date.
- Next due date.
- Current interval / repetition number.
- Recall result and confidence.
- Revision notes and optional evidence.
- Full immutable review history.

Recall outcomes:

| Recall result | Scheduling response |
|---|---|
| Correct, clear, no notes | Move to the next interval |
| Correct but slow/uncertain | Repeat at the same or next shorter interval |
| Partially correct | Schedule in 1–3 days |
| Incorrect / cannot explain | Return item to Learning or weak state; schedule in 1 day |
| Repeatedly failed | Break into smaller subtopics and add a practical exercise |

Revision task type depends on the item:

- DSA: re-solve the exact official problem or write pseudocode under a time limit.
- Java: explain from memory and reproduce a small code experiment.
- LLD: draw the class model / sequence and defend pattern selection.
- HLD: recreate an architecture diagram and explain trade-offs/failure modes.
- FDE: reproduce a working slice, explain request/data flow or re-run an evaluation.

A failed revision does not reset an entire track or category. It updates only the relevant item and its due date. Manual due-date overrides must be supported and audited.

### 5.7 DSA workspace

- Problem list with official order, category, difficulty, URL and status.
- Filters by category, difficulty, status, pattern, due revision and starred/favourite state.
- Search by title and tags.
- Problem details workspace with statement/constraints (if imported), personal intuition, brute-force approach, optimized approach, invariant, time/space complexity, edge cases, mistakes, interview explanation and references.
- Multiple attempts per problem, each with date, outcome, duration, notes and code file.
- Java solution storage in normal `.java` files.
- Support code editor with syntax highlighting. Monaco may be used if it works reliably in Tauri; evaluate bundle size and save behavior first.
- Distinguish accepted/solved from interview-ready and mastered.
- Star/bookmark a problem.
- Revision history and next due date.
- Do not treat external LeetCode accepted status as proof that personal explanation/recall criteria are satisfied.

### 5.8 Java workspace

- Topic hierarchy following A1–A10.
- Subtopic checklist, resource references, priority and estimates.
- Markdown notes, interview questions, code experiments and linked files.
- Confidence score and status.
- Revision schedule and actual learning evidence.

### 5.9 LLD workspace

- Topic and design-pattern library following the CodeArmy sequence.
- Current new-learning cursor begins at Bridge after Composite.
- Previously covered OOP/SOLID/creational/structural-through-Composite content is in revision mode.
- Each design supports requirements, actors/use cases, entities/value objects, class relationships, class/sequence diagrams, interfaces/APIs, Java code, tests, patterns, trade-offs, extension exercise and retrospective.
- Store diagrams as Mermaid and/or image files in the local content directory.
- Track independent attempt before viewing the course solution.
- Track at least four substantial independent projects and timed mock outcomes.

### 5.10 HLD workspace

- Topic library following D1–D10.
- Each case study supports scope, functional/non-functional requirements, estimates, APIs, data model, architecture diagram, read/write flows, bottlenecks, scaling plan, consistency, failure handling, retries/idempotency, observability, security and trade-offs.
- Store diagrams and Markdown locally.
- Track at least five complete designs and timed mock outcomes as the roadmap's success evidence.

### 5.11 FDE workspace and projects

- Topic library following E1–E10.
- Project milestones M1–M6 from the source roadmap.
- Track one coherent AI-enabled project from problem definition through model integration, retrieval/tooling where justified, evaluation, security, observability, cost/latency and technical walkthrough.
- Store repository URL, local project path, README, diagrams, test/evaluation data and milestone notes.
- The app tracks the FDE learning project; it is not itself required to contain an AI agent.

### 5.12 Global search and navigation

Search across topics, subtopics, problems, attempts, notes, code metadata, design exercises, custom entries and projects. Navigation should be keyboard-friendly, fast and consistent.

### 5.13 Backups, restore and export

- One-click backup of SQLite database and the entire DevTrack content directory.
- Restore wizard with validation and confirmation before overwrite.
- Export notes as Markdown and optionally export all records/metadata as JSON/CSV.
- Use SQLite-safe backup or checkpoint methods; do not copy a live database file in an unsafe way.
- Preserve relative file paths so a backup can be moved to another Mac.
- Show last backup date and backup health.

### 5.14 Optional integrations (later phase)

**GitHub:** Read-only repository linking, repository metadata, recent commit activity and opening linked code/repositories. Store PAT in macOS Keychain or an OS secure credential store, never plaintext SQLite/config files.

**LeetCode:** Treat as best-effort optional integration. There may be no stable public API for all desired stats. Do not rely on undocumented endpoints as a core dependency. Provide manual CSV/JSON import and user-managed status as permanent fallback. Never store the LeetCode password. Do not automate authenticated scraping without explicit approval.

Integrations must not overwrite personal notes, attempts, completion evidence or revision history.

## 6. Information architecture and UI

Recommended primary navigation:

1. Dashboard
2. Today
3. Roadmap
4. DSA
5. Core Java
6. LLD
7. HLD
8. FDE
9. Projects
10. Revision
11. Analytics
12. Settings

### UI expectations

- Premium, calm developer-oriented interface; dark theme initially, with light/system theme support.
- Persistent left sidebar and clear top-level page titles.
- Dense enough for technical content but not visually cluttered.
- Dashboard emphasizes today's plan and due revisions.
- Topic/problem details should support split-pane navigation where useful.
- Clear empty, loading, error, import and backup states.
- Responsive enough for common Mac window sizes; prioritize desktop use.
- Avoid decorative cards for every item. Optimize for frequent use and fast capture.
- Provide keyboard shortcuts for search, quick-add and navigation where practical.

## 7. Local-first technical architecture

### Recommended baseline

- **Desktop shell:** Tauri 2.
- **Frontend:** React + TypeScript + Vite.
- **UI:** Tailwind CSS and a restrained component library such as shadcn/ui.
- **Backend boundary:** Tauri commands in Rust.
- **Database:** SQLite, with versioned migrations and foreign-key constraints.
- **Content:** Normal Markdown, Java, JSON and Mermaid files in an app-managed local data directory.
- **Charts:** A lightweight React chart library only where analytics needs it.
- **Diagrams:** Mermaid rendering with source saved as `.mmd` or Markdown files.

OpenCode must verify compatibility against current stable documentation before choosing exact package versions. Do not blindly install a stack from an outdated tutorial. Keep the UI independent of the persistence implementation through typed service/repository interfaces.

### Storage layout

Use an app-managed data root, configurable in Settings. Suggested default:

```text
~/Documents/DevTrack/
├── devtrack.sqlite
├── content/
│   ├── dsa/
│   │   └── <problem-slug>/
│   │       ├── notes.md
│   │       ├── intuition.md
│   │       ├── attempts/
│   │       │   └── <attempt-id>/solution.java
│   │       └── diagrams/
│   ├── core-java/<topic-slug>/
│   ├── lld/<design-or-pattern-slug>/
│   ├── hld/<case-study-slug>/
│   ├── fde/<topic-or-project-slug>/
│   └── projects/<project-slug>/
├── imports/
├── exports/
└── backups/
```

Use relative paths in the database, never hardcode machine-specific absolute paths in records. Prevent path traversal. Write files atomically where practical. The app must not scan or modify arbitrary directories without an explicit user action.

### Data model — logical entities

OpenCode should produce a normalized schema and ER diagram. At minimum, account for:

- `tracks`, `modules`, `topics`, `subtopics`.
- `resources`, `topic_dependencies`, `related_topics`.
- `roadmap_phases`, `roadmap_weeks`, `weekly_plan_items`.
- `problems`, `problem_attempts`, `problem_tags`, `problem_topic_links`.
- `study_tasks`, `daily_plans`, `weekly_goals`, `study_sessions`.
- `revision_items`, `revision_reviews`.
- `learning_entries` (custom/unplanned learning).
- `projects`, `project_milestones`, `project_links`.
- `content_files` or file-path metadata.
- `activity_events` for heatmap/streak calculation.
- `settings`, `import_runs`, `schema_migrations`.

Important modeling rules:

- Use stable UUIDs for internal IDs and stable source IDs/slugs for external curriculum records.
- Store planned tasks separately from completed activity and custom learning entries.
- Store each DSA attempt and revision review as a separate historical record; never overwrite history.
- Store completion state and mastery/readiness state separately.
- Keep file contents in files, with relative path, checksum and metadata in SQLite where useful.
- Store date-only fields as local calendar dates and instants with timezone-aware timestamps. Default to the user's Mac timezone (currently Asia/Kolkata), but allow the timezone to follow system settings or be changed.
- Migrations must be reversible where practical and backed up before destructive migration.

## 8. Roadmap and dataset import design

### 8.1 Six-month roadmap import

The user will place a copy of the source Markdown file in:

`docs/roadmap/six_month_engineering_roadmap.md`

OpenCode should:

1. Read the complete Markdown file.
2. Convert its content into a structured, versioned curriculum representation (JSON/YAML or seed data) without losing descriptions, resources, dependencies, practical work, completion criteria, durations, weekly sequencing, milestones or revision rules.
3. Produce an import preview showing track counts, topic counts, subtopic counts, week count and warnings.
4. Validate the imported curriculum before inserting it into SQLite.
5. Keep the original Markdown file unchanged and preserve a source reference/version.
6. Make future re-imports idempotent: do not duplicate topics or overwrite user notes/progress.
7. Support a diff preview when a newer roadmap is imported.

Do not use free-form runtime LLM parsing as the normal import mechanism. Use a deterministic, validated importer or a curated structured seed file derived from the Markdown.

### 8.2 NeetCode 250 import

The roadmap document contains category names/counts and the official URL, not the full 250-problem record set. Therefore:

- Build a CSV/JSON import template and an import preview/validator.
- Do not make up missing question titles, slugs, links, difficulty or official order.
- If the user later supplies the dataset/export, validate it and import it.
- If a reliable dataset is not yet available, show a clear empty/import-required state rather than fake seeded questions.
- Enforce duplicate checks and display count mismatches.

### 8.3 Initial progress wizard

Ask once for confirmation of the known checkpoints:

- DSA: previously covered through Trees; start new work at Tries.
- LLD: previously covered through Composite; start new work at Bridge.
- Java/HLD/FDE: start from the beginning of the supplied plan unless the user changes this.

Then allow per-problem/per-topic status import or manual confirmation. Missing historical dates remain unknown. Provide the explicit baseline action for old items without history.

## 9. Incremental development phases

Every phase must end with a working app, tests, updated documentation and a short acceptance checklist. Do not implement all phases in one massive generation.

### Phase 0 — Repository discovery and architecture lock

**Goal:** Make sound decisions before generating the codebase.

Tasks:

- Inspect the Mac development environment and the current repository/workspace.
- Read the complete learning roadmap.
- Confirm runtime architecture (Tauri desktop default; local browser is acceptable only if Tauri has a concrete blocker and the user approves).
- Create the PRD, architecture decision record, data-model draft, implementation backlog and project `AGENTS.md`.
- Identify required setup and missing inputs (especially the actual NeetCode 250 problem dataset and historical per-problem state).
- Do not invent a dataset or progress data.

Deliverables:

- `README.md`
- `AGENTS.md`
- `docs/PRODUCT_REQUIREMENTS.md`
- `docs/ARCHITECTURE.md`
- `docs/DATABASE_SCHEMA.md` (initial ERD + logical schema)
- `docs/IMPLEMENTATION_PLAN.md`
- `docs/ROADMAP_IMPORT.md`
- `docs/decisions/ADR-001-local-first-architecture.md`

**Acceptance:** The user can review architecture, phase boundaries, environment checks and unresolved data inputs before code is generated.

### Phase 1 — Desktop shell and safe local persistence

**Goal:** Launch a real local Mac application with reliable persistence.

Tasks:

- Bootstrap Tauri + React + TypeScript using current stable compatible versions.
- Create navigation shell, sidebar, page layout, settings page and theme support.
- Implement SQLite initialization and migrations behind a typed backend/repository layer.
- Create the app-managed data directory and content directory.
- Add application settings, database health, startup error handling and structured logging.
- Add one-click backup/restore of an empty or sample database and content directory.
- Include a sample empty state, not fake learning data.

**Acceptance:** App opens as a desktop app; creates/reopens local database; data survives restart; migrations run; backup can be restored; no network/hosted backend is required.

### Phase 2 — Curriculum model and roadmap import

**Goal:** Turn the supplied roadmap into real editable curriculum data.

Tasks:

- Implement tracks/modules/topics/subtopics/resources/dependencies/weekly-plan entities.
- Build deterministic roadmap import, preview, validation, idempotency and diff handling.
- Import the 26-week plan and current-progress overrides.
- Build track overview pages, topic tree, topic detail page and progress states.
- Add topic notes, resources, completion criteria, confidence and evidence fields.
- Include initial progress confirmation wizard.

**Acceptance:** Five tracks appear with roadmap content; DSA's next new topic is Tries; LLD's next new topic is Bridge; older DSA/LLD content is revision-only; HLD/Java/FDE begin at the roadmap start; re-import does not duplicate or erase user data.

### Phase 3 — Daily planner and weekly planning

**Goal:** Make the dashboard useful every day.

Tasks:

- Build availability settings and weekly cadence editor.
- Generate daily/weekly tasks from the current week of the imported plan.
- Add task categories: planned learning, revision, weekly review, optional stretch.
- Add daily task completion, output/evidence, planned vs actual duration.
- Add custom learning entries.
- Add weekly review and rescheduling rules.
- Add meaningful activity events and first version of streak/heatmap.
- Add daily/weekly progress summaries.

**Acceptance:** Dashboard shows a believable agenda based on the roadmap and available time; user can complete planned tasks, add unplanned learning, move tasks, mark unavailable days and complete a weekly review; missed work does not pile up or duplicate.

### Phase 4 — DSA / NeetCode 250 workspace and revision engine

**Goal:** Deliver the most-used technical learning workflow.

Tasks:

- Build NeetCode CSV/JSON import template, validator and preview.
- Import official question order, category, difficulty, title, URL and stable identifiers.
- Build question list, search, filters, favorites and question details.
- Build attempt history, Java solution file management, notes and complexity fields.
- Implement revision engine and review history.
- Add DSA category progress and old-category revision queue.
- Start new-problem assignment from Tries and enforce no new assignments from categories 1–7 by default.

**Acceptance:** The user can import the actual sheet, open any problem, save multiple attempts and Java solutions, capture intuition/mistakes/complexity, mark solved only after independent attempt criteria, and see revisions scheduled from known dates or explicitly established baseline dates.

**First usable release:** At the end of Phase 4, DevTrack should support the user's daily learning routine and DSA progress end-to-end.

### Phase 5 — Core Java, LLD and HLD learning workspaces

**Goal:** Support deeper notes, exercises and design practice.

Tasks:

- Java A1–A10 topic workspace, code experiments and recall questions.
- LLD topic workspace; revision queue through Composite; new learning begins at Bridge.
- LLD design workspace with diagrams, requirements, APIs, Java implementation, tests and trade-offs.
- HLD D1–D10 workspace and case-study templates.
- Mermaid diagram editor/preview and local source-file storage.
- Related topic links across tracks.

**Acceptance:** The learner can maintain a complete Java note/experiment, independently design and document an LLD problem, and create an HLD case study with an architecture diagram and trade-off record.

### Phase 6 — FDE and project portfolio

**Goal:** Track applied AI learning and one coherent project.

Tasks:

- Import E1–E10 and M1–M6 project milestones.
- Build project workspace with repo/local path, tasks, milestones, docs, diagrams and evidence.
- Link topics across FDE, Java and HLD.
- Track evaluations, security reviews, reliability/cost/latency findings and technical walkthrough notes.

**Acceptance:** A project can move through problem definition, model integration, retrieval/tool use where justified, evaluation, security, production hardening and demo readiness.

### Phase 7 — Analytics, polish and data portability

**Goal:** Turn progress history into useful insights and harden daily usability.

Tasks:

- Weekly/monthly reports and track progress charts.
- Topic confidence and weak-area insights.
- Planned vs actual time and learning output analytics.
- Revision retention statistics and overdue analysis.
- Search across all content.
- Export all metadata and Markdown/code files.
- Validate backup/restore and recovery cases.
- Add keyboard shortcuts, loading/error/empty states, accessibility and UI polish.

**Acceptance:** All analytics are derived from real local records; export/import can move data to another Mac; no metric is invented or inferred from app launches alone.

### Phase 8 — Optional integrations

**Goal:** Enrich the local workspace without making it dependent on external services.

Tasks:

- GitHub read-only integration, secure credential storage, repo linking and recent activity.
- LeetCode integration only if a reliable, permitted method exists; otherwise retain CSV/manual import.
- Integration errors never break core app functionality.

**Acceptance:** Disconnecting the network or revoking a token does not affect notes, tracking, scheduling or local records.

## 10. Testing and quality requirements

### Automated tests

- Unit tests for scheduling, revision intervals, scoring and streak calculations.
- Unit tests for roadmap and problem import validation, duplicate detection and idempotency.
- Database migration and repository tests.
- File storage tests: atomic writes, path traversal rejection, missing file and recovery handling.
- UI tests for primary dashboard, marking a task complete, adding a custom entry, saving a DSA attempt and completing a revision.
- Integration tests for backup/restore and re-import behavior.

### Critical scheduling test cases

1. User has 45 minutes today: don't schedule 90 minutes of work.
2. User marks today unavailable: move eligible tasks without duplicates.
3. A planned task was missed yesterday: don't automatically stack all missed work onto today.
4. Three DSA revisions are due: rank by lateness/weakness/priority and fit within capacity.
5. Revision failed: schedule that item in one day, not the whole topic or category.
6. User adds unplanned Java learning: count it as activity but do not mark an unrelated roadmap item complete.
7. User completes only a custom learning entry: include activity in heatmap; roadmap completion remains unchanged.
8. User changes weekly availability: regenerate only uncompleted future tasks and preserve history.
9. User re-imports the same roadmap: no duplicate curriculum records or overwritten personal progress.
10. User restores backup: all records and linked content files resolve.

### Non-functional requirements

- Fast startup and responsive navigation on a Mac.
- Local persistence and graceful recovery after abnormal exit.
- No mandatory network for core features.
- No plaintext API tokens.
- Clear errors; no silent data loss.
- Typed interfaces and strict TypeScript where applicable.
- Database queries isolated from UI components.
- No secrets committed to Git.
- Avoid heavyweight dependencies unless justified.

## 11. Suggested repository structure

OpenCode may refine this after architecture review, but should maintain equivalent separation:

```text
DevTrack/
├── AGENTS.md
├── README.md
├── package.json
├── src/
│   ├── app/
│   ├── components/
│   ├── features/
│   │   ├── dashboard/
│   │   ├── planner/
│   │   ├── roadmap/
│   │   ├── dsa/
│   │   ├── java/
│   │   ├── lld/
│   │   ├── hld/
│   │   ├── fde/
│   │   ├── projects/
│   │   ├── revision/
│   │   └── analytics/
│   ├── services/
│   ├── types/
│   └── styles/
├── src-tauri/
│   ├── src/
│   ├── migrations/
│   └── tauri.conf.json
├── docs/
│   ├── roadmap/
│   │   └── six_month_engineering_roadmap.md
│   ├── PRODUCT_REQUIREMENTS.md
│   ├── ARCHITECTURE.md
│   ├── DATABASE_SCHEMA.md
│   ├── ROADMAP_IMPORT.md
│   ├── IMPLEMENTATION_PLAN.md
│   └── decisions/
└── tests/
```

## 12. Definition of done for each phase

A phase is complete only when:

1. Its agreed user stories work in the running app.
2. Acceptance criteria pass.
3. Automated tests for the phase pass.
4. Build, typecheck and lint pass.
5. Data migrations and backup behavior are safe.
6. README/AGENTS/docs are updated.
7. The developer can demonstrate the functionality with real data or an explicitly labeled test fixture.
8. OpenCode reports files changed, commands run, test results, known gaps and any decisions requiring user approval.

Do not begin the next phase automatically. Present the completed increment and wait for the user's go-ahead.
