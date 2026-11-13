# DevTrack — Roadmap and Dataset Import

Deterministic, validated import of the supplied 26-week roadmap and the NeetCode 250 problem dataset. No free-form LLM parsing is used as the normal import mechanism.

## 1. Six-month roadmap import

### 1.1 Source location

The user places a copy of the supplied Markdown roadmap at:

`docs/roadmap/six_month_engineering_roadmap.md`

The file is kept unchanged; a source reference/version is preserved in `import_runs`.

### 1.2 Import steps

1. Read the complete Markdown file.
2. Convert its content into a structured, versioned curriculum representation without losing: descriptions, resources, dependencies, practical work, completion criteria, durations, weekly sequencing, milestones or revision rules.
3. Produce an import preview: track counts, topic counts, subtopic counts, week count and warnings.
4. Validate the imported curriculum before inserting into SQLite.
5. Make re-imports idempotent: no duplicated topics, no overwritten user notes or progress.
6. Support a diff preview when a newer roadmap is imported.
7. Record the run in `import_runs` with record count, warnings (missing/duplicate/malformed records) and status.

### 1.3 What must not happen

- Do not invent topic titles, descriptions or counts.
- Do not use runtime LLM parsing as the normal import mechanism (deterministic parser or curated structured seed derived from the Markdown).
- Do not overwrite user-set status, confidence, notes or evidence on re-import.

## 2. NeetCode 250 import

### 2.1 Source

The user has supplied the official NeetCode 250 dataset at:

`/Users/sarwesh/Downloads/NEETCODE_250.csv`

Validated: 250 problems, all required fields present (official_order, problem_id/slug, title, url, category, difficulty, category_order, source_name, source_version). Category counts match the roadmap exactly (109 through Trees, 141 from Tries onward).

Import into SQLite is deferred to Phase 2/4 per the incremental plan — not imported during Phase 0.

### 2.2 Import template and validation

- Build a CSV/JSON import template and an import preview/validator.
- Required fields: stable identifier/slug, title, URL, official order, category, difficulty.
- Do not make up missing titles, slugs, links, difficulty or official order.
- If no dataset is available, show a clear empty/import-required state rather than fake seeded questions.
- Enforce duplicate checks; display count mismatches against roadmap planning counts (109 through Trees, 141 from Tries onward).

### 2.3 Versioning

- Keep the imported dataset versioned; record import source and date in `import_runs`.

## 3. DSA progress overrides applied at import

- Categories Arrays & Hashing through Trees: `is_previously_covered = 1`, available for revision only.
- Next new category: Tries.
- Individual problem statuses and solve dates are unknown until imported or confirmed by the user.
- Offer a one-time "Start revision baseline now" action for previously covered problems with no history. This explicitly establishes a new baseline; it does not pretend to recover original solve dates.

## 4. LLD progress overrides applied at import

- OOP/foundations, SOLID, creational patterns, structural patterns through Composite: revision only.
- Next new topic: Bridge Pattern; then remaining structural patterns, behavioral patterns, projects, machine-coding readiness.
- Preserve verified CodeArmy lecture order where available.

## 5. Java, HLD, FDE overrides

- Core Java starts at A1; HLD at D1; FDE at E1, unless the user changes this.

## 6. Initial progress wizard (run once after import)

Ask for confirmation of:
- DSA: previously covered through Trees; start new work at Tries.
- LLD: previously covered through Composite; start new work at Bridge.
- Java/HLD/FDE: start from the beginning of the supplied plan unless changed.

Then allow per-problem/per-topic status import or manual confirmation. Missing historical dates remain unknown.

## 7. Import output

The importer reports: missing records, duplicate records, malformed records, count mismatches, and a preview of tracks/topics/weeks before committing to SQLite. The user approves before data is written.
