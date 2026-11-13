# DevTrack — Product Requirements

Source: `docs/DEVTRACK_IMPLEMENTATION_PLAN.md`. This PRD distills the product vision, scope, learning-plan overrides, scheduling rules and functional requirements for implementation.

## 1. Product vision

DevTrack is a personal technical learning system for a six-month SDE interview and FDE preparation plan. It connects:

- A structured learning roadmap across Core Java, DSA, LLD, HLD and FDE
- Daily and weekly learning goals generated from that roadmap
- Spaced revision of previously learned problems and concepts
- Personal notes, code, attempts, diagrams and practical outputs
- Progress, confidence, consistency, streaks and weak-area analytics

The dashboard must answer:
1. What should I study today?
2. Which revisions are due today, and why?
3. What is the next unfinished topic in each track?
4. Am I on track for this week's plan without accumulating missed-task debt?
5. What have I actually learned, implemented or independently recalled?

## 2. Product principles

1. Local-first: core features work offline. External integrations are optional.
2. Roadmap-driven: the supplied 26-week roadmap is the default curriculum, not hardcoded UI labels.
3. Progress-aware: never assume all tracks start from zero. Respect current DSA and LLD checkpoints.
4. Evidence over consumption: watching a video or reading an article alone does not make a topic complete.
5. Revision is first-class: revision tasks are generated from actual completion/review dates and recall outcomes.
6. Flexible, not punitive: missed tasks are intelligently replanned. A missed day must not double the next day's workload.
7. Portable data: notes and code are normal local files with export and backup support.
8. No fake data: do not invent historical solve dates, attempts, completion states, metrics or external integration results.

## 3. Scope

### Included

- Five learning tracks and the 26-week roadmap
- Daily agenda, weekly goals, learning sessions and custom learning entries
- DSA question library for the official NeetCode 250 sheet
- Per-problem attempts, Java solutions, notes, complexity and revision history
- Topic workspaces for Java, LLD, HLD and FDE
- LLD/HLD design notes, diagrams, code links and project records
- Revision engine, completion tracking, activity heatmap, streaks and analytics
- Local database, filesystem storage, backup, restore and export
- Optional future GitHub and LeetCode integrations

### Not in the initial release

- Cloud database, Firebase, Supabase, hosted backend or mandatory deployment
- Multi-user accounts, social features, leaderboards or collaboration
- An autonomous AI tutor or automatic solution generator
- Automatic scraping or login automation against LeetCode
- A large plugin marketplace or generic productivity/task manager
- Automatic access to the whole Mac filesystem

AI assistance may be added later as an optional helper. It must not be required for normal tracking or scheduling.

## 4. Learning plan and current-progress overrides

### 4.1 DSA — NeetCode 250

Current checkpoint: previously covered through **Trees**. New DSA learning starts at **Tries**.

| Category | Treatment |
|---|---|
| Arrays & Hashing, Two Pointers, Sliding Window, Binary Search, Linked List, Stack, Trees | Previously covered; revision only |
| Tries through Math & Geometry | New learning, in official NeetCode 250 order |

Rules:
- Mark the first seven categories as `previously_covered` for sequencing.
- Do not fabricate individual `solved_at` dates, attempts, code or confidence.
- Offer a one-time "Start revision baseline now" action for previously covered problems with no history.
- Never assign new DSA questions from categories 1–7 unless the user changes the checkpoint.
- Official NeetCode 250 sheet is the only source of new questions. Preserve exact title, URL, order, category, difficulty and stable identifier.
- DSA revision intervals: next day; day 3–4; day 7; day 14; day 30; day 60–90.

### 4.2 LLD — CodeArmy

Current checkpoint: completed through **Composite Design Pattern**. New LLD learning starts at **Bridge Pattern**.

Previously covered (revision pool): OOP/LLD foundations, UML/modeling, SOLID, creational patterns, structural patterns through Composite.

Sequence after Bridge: remaining structural patterns (incl. Flyweight where in course order), behavioral patterns, end-to-end projects, machine-coding readiness, timed mocks.

Rules:
- CodeArmy remains the primary LLD video curriculum; preserve published lecture order.
- Attempt each practical design before watching its solution.
- Target at least four substantial independent projects and three timed mocks.

### 4.3 Core Java

Start from A1 of the supplied roadmap. Interview depth and internals focus — not beginner syntax (user uses Java/Spring Boot professionally).

### 4.4 HLD

Start from D1 of the supplied roadmap.

### 4.5 FDE

Start from E1 of the supplied roadmap. Direct model calls → structured outputs → retrieval/tool use → deterministic workflow → bounded orchestration only when justified. No multi-agent system at the start.

### 4.6 Weekly remapping (DSA/LLD)

- Week 1 starts at Tries. Roughly 6–8 new problems per week plus 3–5 revisions.
- Finish new problems by approximately weeks 20–24; weeks 25–26 for consolidation/mocks.
- Java, HLD and FDE keep their existing week-by-week progression.

## 5. Scheduling rules

- Baseline: 7–8 focused hours per week, editable in Settings.
- Default weekly track allocation: DSA 2.5–3h, Java 1–1.25h, LLD 1h, HLD 1–1.25h, FDE 1.5–2h.
- Default cadence: Mon/Wed/Fri DSA, Tue Java, Thu LLD or HLD alternating, Sat FDE, Sun weekly review.
- Normal day: one primary task, one revision task (when due), one small tangible output, optional stretch only if capacity remains.
- Usually one or two tracks per day, not all five.
- Keep 10–20% time buffer; one rest/low-intensity day per week.
- Missed tasks are replanned by priority, dependencies, age and capacity — never duplicated or doubled onto the next day.
- Task score heuristic: `priority_weight + overdue_weight + weakness_weight + dependency_weight + track_balance_weight`.

## 6. Functional requirements

### 6.1 Dashboard

- Date and today's planned capacity
- "What to Study Today" agenda grouped by primary learning, revision and optional stretch
- Track/topic label, duration, priority, completion state per task
- Today and weekly completion progress
- Revisions due today, overdue count, weak items
- Current 26-week phase and week
- Progress across Java, DSA, LLD, HLD, FDE
- Current streak and activity heatmap
- `+ Add Learning Entry` action; route to weekly review
- No fabricated stats; empty states explain how to import or add progress

### 6.2 Roadmap / curriculum

- Import 26-week plan: weeks, phases, track allocations, units, subtopics, priorities, estimates, dependencies, resources, practical tasks, completion criteria
- Expandable Track → Module → Topic → Subtopic hierarchy
- Status states: Not started, Learning, Practicing, Needs revision, Interview ready, Mastered
- Notes, links, completion evidence, confidence (1–5), estimated/actual time
- Mark subtopics individually; topic completion derived from criteria/subtopics
- Support custom topics and editing without destroying source metadata
- Show previous/next dependencies and related cross-track topics
- Override priority, target week, estimate and status; changes must not erase history

### 6.3 Daily and weekly planner

- Generate daily/weekly agenda from current week, availability and due revisions
- Drag/reorder, move, split, postpone, skip or add tasks
- Mark complete with optional/required output
- Planned vs actual minutes tracked separately
- Daily/weekly completion percentages
- Planned tasks kept separate from revisions and additional learning
- Replan missed work without duplication; freeze or regenerate a week after review

### 6.4 Custom learning entries

Fields: title, date/time, track or "Unplanned / Other", description, notes/Markdown, resource URL, duration, output/code/diagram/file link, tags, related roadmap topics/problems/projects.
- Appear in activity history and heatmap
- Visually distinguished from planned tasks; do not auto-complete roadmap topics
- User may explicitly link an entry as evidence for a topic

### 6.5 Progress and streaks

- Track planned tasks completed, revisions completed, custom entries and study sessions separately
- Daily, weekly and track-level progress
- Streaks based on meaningful activity, not app opens
- Configurable minimum meaningful activity rule
- Rest/unavailable days visible without counting as missed goals
- Heatmap intensity based on meaningful activity

### 6.6 Revision engine

Default intervals: 1, 3, 7, 14, 30, 60 days. NeetCode final interval 60–90 configurable.

Revision record: source item, initial completion date (if known), last reviewed, next due, current interval/repetition, recall result and confidence, revision notes/evidence, immutable review history.

Recall outcomes:
- Correct and clear: advance to next interval
- Correct but slow/uncertain: same or shorter interval
- Partially correct: due in 1–3 days
- Incorrect/cannot explain: return to Learning/weak state, due in 1 day
- Repeated failures: break into subtopics, add practical exercise

Revision task types: DSA re-solve; Java explain + code experiment; LLD draw/defend design; HLD recreate diagram + trade-offs; FDE reproduce slice.

A failed revision updates only the relevant item. Manual due-date overrides supported and audited.

### 6.7 DSA workspace

- Problem list with official order, category, difficulty, URL, status
- Filters by category, difficulty, status, pattern, due revision, starred
- Search by title and tags
- Problem details: statement/constraints, intuition, brute force, optimized approach, invariant, complexity, edge cases, mistakes, interview explanation, references
- Multiple attempts per problem with date, outcome, duration, notes, code file
- Java solution storage in `.java` files
- Accepted ≠ solved ≠ interview-ready ≠ mastered
- Star/bookmark; revision history and next due date

### 6.8 Java workspace

- Topic hierarchy A1–A10; subtopic checklist, resources, priority, estimates
- Markdown notes, interview questions, code experiments, linked files
- Confidence score, status, revision schedule, learning evidence

### 6.9 LLD workspace

- Topic/design-pattern library in CodeArmy sequence; cursor begins at Bridge
- Previously covered content in revision mode
- Each design: requirements, actors/use cases, entities, class relationships, diagrams, APIs, Java code, tests, patterns, trade-offs, extension exercise, retrospective
- Diagrams as Mermaid and/or image files in local content dir
- Track independent attempt before viewing course solution
- Track four independent projects and timed mock outcomes

### 6.10 HLD workspace

- Topic library D1–D10
- Case studies: scope, requirements, estimates, APIs, data model, architecture diagram, flows, bottlenecks, scaling, consistency, failure handling, observability, security, trade-offs
- At least five complete designs and timed mock outcomes

### 6.11 FDE workspace

- Topic library E1–E10; project milestones M1–M6
- One coherent AI-enabled project through problem definition, model integration, retrieval/tooling where justified, evaluation, security, observability, cost/latency, walkthrough
- Repo URL, local path, README, diagrams, test/evaluation data, milestone notes

### 6.12 Global search and navigation

Search across topics, subtopics, problems, attempts, notes, code metadata, designs, custom entries, projects. Keyboard-friendly navigation.

### 6.13 Backups, restore and export

- One-click backup of SQLite DB and content directory
- Restore wizard with validation and confirmation before overwrite
- Export notes as Markdown; metadata as JSON/CSV
- SQLite-safe backup; relative paths preserved; last backup date and health shown

### 6.14 Optional integrations (later phase)

- GitHub: read-only repo linking, metadata, commit activity. PAT in macOS Keychain, never plaintext.
- LeetCode: best-effort only; manual CSV/JSON import as permanent fallback; never store password; no automated scraping without approval.
- Integrations must not overwrite personal notes, attempts, evidence or revision history.

## 7. Information architecture and UI

Primary navigation: Dashboard, Today, Roadmap, DSA, Core Java, LLD, HLD, FDE, Projects, Revision, Analytics, Settings.

UI expectations: premium calm developer interface; dark theme default with light/system support; persistent left sidebar; dense but uncluttered; dashboard emphasizes today's plan and due revisions; split-pane where useful; clear empty/loading/error/import/backup states; desktop-first; keyboard shortcuts for search, quick-add, navigation.
