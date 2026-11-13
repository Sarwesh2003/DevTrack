# ADR-001 — Local-first architecture and SQLite access

## Status

Proposed (Phase 0). To be confirmed before Phase 1 implementation.

## Context

DevTrack is a single-user, local-first Mac desktop app. We need:
- A desktop shell with a web frontend
- Local structured persistence (SQLite)
- Local file storage for Markdown/Java/Mermaid content
- Typed boundary between UI and local operations

## Decision

Use Tauri 2 (Rust) as the desktop shell, React + TypeScript + Vite for the frontend, and a **Rust-native SQLite crate** accessed through Tauri commands for all database operations.

### SQLite access: Rust-native crate vs Tauri SQL plugin

| Option | Pros | Cons |
|---|---|---|
| Rust-native SQLite crate (e.g. `rusqlite` or `better-sqlite3` equivalent) | Full control, simplest for single-user app, no extra abstraction, migrations straightforward, well documented | Rust FFI/unsafe boundary managed by the crate; must pin compatible versions |
| Tauri SQL plugin | Official Tauri integration, command-generated API | Adds an abstraction layer; still evolving; version compatibility to verify |

For this single-user app, the Rust-native SQLite crate is the simplest maintainable option. The Tauri SQL plugin will be re-evaluated if it offers clear migration, backup or security benefits at Phase 1.

## Consequences

- All SQL lives in Rust repository layer; frontend/UI never issues SQL.
- Database file lives in the app-managed data directory (`~/Documents/DevTrack/devtrack.sqlite` by default).
- Versioned migrations in `src-tauri/migrations/`; backed up before destructive migration.
- No separate server process; no network required for core features.
- Backup/restore uses SQLite-safe checkpoint methods, not live file copies.

## Verification required (Phase 1)

- Confirm current stable Tauri 2 version and Rust-native SQLite crate version against official documentation.
- Verify Tauri 2 works with macOS webview and code signing requirements for local distribution.

## Revisiting

Re-evaluate the Tauri SQL plugin choice if migration or backup complexity proves higher than expected during Phase 1.
