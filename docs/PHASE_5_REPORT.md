# Phase 5 — Core Java, LLD and HLD learning workspaces

**Status:** Complete (pending user approval)

## Goal

Support deeper notes, exercises and design practice beyond the daily plan: a code/experiment
workspace for Java topics, an LLD design workspace (requirements, API, Java implementation,
tests, trade-offs), an HLD case-study template with architecture diagrams, Mermaid diagrams,
recall questions with a spaced-review ladder, and cross-track topic links.

## What was built

### 1. Schema (migration v5)

`SCHEMA_VERSION = 5`. v5 (`learning_workspaces`) adds six tables on top of the existing
schema, each with a topic foreign key (`ON DELETE CASCADE`) and a per-topic index:

- `code_experiments` — `title`, `code`, `language`, optional `file_path`.
- `recall_questions` — `question`, `answer`, `difficulty`, `last_reviewed`, `next_review`,
  `interval`, `repetition`, `state`.
- `diagrams` — `title`, `mermaid_content`, optional `file_path`.
- `design_artifacts` — `requirements`, `api_spec`, `java_implementation`, `tests`, `trade_offs`.
- `case_studies` — `context`, `diagram_content`, `trade_offs`.
- `topic_links` — `from_topic_id`, `to_topic_id`, `link_type`, unique on
  `(from_topic_id, to_topic_id, link_type)`.

The migration loop applies every pending version sequentially.

### 2. Backend (`src-tauri/src/workspace.rs`)

Typed request/response structs and Tauri commands:

- Code experiments: `upsert_code_experiment`, `get_code_experiment`, `list_code_experiments`.
- Recall questions: `upsert_recall_question`, `record_recall_review`, `get_recall_question`,
  `list_recall_questions`.
- Diagrams: `upsert_diagram`, `get_diagram`, `list_diagrams`.
- LLD design: `upsert_design_artifact`, `get_design_artifact`, `list_design_artifacts`.
- HLD case studies: `upsert_case_study`, `get_case_study`, `list_case_studies`.
- Cross-track links: `add_topic_link`, `list_topic_links`.
- Local source files: `save_workspace_file` writes under
  `content/workspace/{experiments,diagrams}/{topic_id}/{name}` with filename sanitisation
  (rejects path separators / traversal) and atomic write.

Recall review ladder: `again` resets the interval to 1 day, `hard` keeps the current interval,
`good` doubles it, `easy` triples it; `next_review` and `state` are updated accordingly.

### 3. Frontend

- New **Workspace** page (`src/features/workspace/Workspace.tsx`): pick a track, then a topic
  (modules are loaded per track and their topics flattened), then work through tabs:
  - **Experiment** — title, language and a Java/code editor; save, reload saved version.
  - **Recall** — add question/answer/difficulty, list questions, grade each with
    again/hard/good/easy.
  - **Diagram** — Mermaid source editor with a note to preview on mermaid.live.
  - **Design (LLD)** — requirements, API spec, Java implementation, tests and trade-offs.
  - **Case Study (HLD)** — context, architecture diagram (Mermaid source) and trade-offs.
  - **Links** — add a related/prerequisite/follows link to another topic id and list links.
- Sidebar now exposes Today, Roadmap, DSA, **Workspace**, Weekly Review, Settings.
- Browser dev mode returns empty shapes for every workspace command so the page renders
  without the Tauri backend.

## Files

- Backend: `src-tauri/src/workspace.rs` (new), `src-tauri/src/migrations.rs` (v5),
  `src-tauri/src/main.rs` (module + command registration).
- Frontend: `src/features/workspace/Workspace.tsx` (new), `src/App.tsx`,
  `src/components/layout/Sidebar.tsx`, `src/components/layout/Shell.tsx`,
  `src/types/index.ts`, `src/services/api.ts`, `src/services/api.test.ts`.

## Tests

- Rust (`cargo test`): 31 passing (unchanged count; Phase 5 adds commands exercised through the
  migration/build rather than a dedicated workspace DB test yet).
- Frontend (`npx vitest run`): 69 passing. Adds API-shape tests for the workspace commands
  (experiment upsert/list, recall upsert/review, diagram upsert, design and case-study list,
  topic link add/list, workspace file save).
- `npx tsc --noEmit`, `eslint --max-warnings 0`, `npm run build`, `cargo build`: clean.
  `cargo build` still reports pre-existing dead-code/unused-import warnings unrelated to this
  phase.

## How to validate

```bash
cd /Users/sarwesh/Desktop/Projects/DevTrack
npm run tauri dev
```

1. Sidebar → **Workspace**. Choose the Core Java track (or LLD/HLD) and a topic.
2. Experiment → write Java, **Save experiment**, reload to confirm persistence.
3. Recall → add a question and grade it `good`; the state/interval advances.
4. Diagram → enter Mermaid source and save.
5. Design (LLD) → fill requirements/API/impl/tests/trade-offs and save.
6. Case Study (HLD) → fill context/diagram/trade-offs and save.
7. Links → add a link to another topic id and confirm it lists.

## Known limitations / deferred

- There is no Mermaid renderer bundled; diagrams are stored as Mermaid source and previewed
  externally (mermaid.live). A renderer can be added without changing the schema.
- Diagrams/experiments are keyed as lists per topic; the UI edits the most recently updated
  entry. Multiple named variants are supported by the backend but not yet surfaced as a picker.
- Topic links are entered by target topic id (no cross-track topic picker yet).
- Diagram/experiment source files are written to disk only via the explicit
  `save_workspace_file` command; the editor currently saves to SQLite.
- LLD/HLD scheduling in the planner (revision queue through Composite; new learning at
  Bridge) remains roadmap-driven from the existing curriculum/planner; the dedicated per-topic
  design/revision scheduling UI is a later-phase concern.
