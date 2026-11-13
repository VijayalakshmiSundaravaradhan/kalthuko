# Kalthuko Phase 0 Implementation

This document summarizes what has been implemented for Phase 0 and outlines the path forward.

## Goals of Phase 0
- Provide a runnable UI scaffold (React + Vite) with a Taura back-end bridge for data access
- Establish a local persistence layer (SQLite) to store languages, profiles, and lessons
- Wire up a minimal data model and views to support Phase 0 flows (language selection, profile, lesson generation)

## System Architecture
- Frontend: React UI in src/ (App.jsx, main.jsx, styles.css)
- Backend Bridge: Tauri commands orchestrating data access
- Persistence: Local SQLite DB (kalthuko_phase0.db) under data/
- Data schema and views defined in phase0_schema.sql

## DB Schema (Phase 0)
- Languages (languages)
- Profiles (profiles)
- Lessons (lessons)
- Settings (settings)
- Views: vw_profiles_overview, vw_profile_lessons

SQL artifacts located at:
- phase0_schema.sql: e:\\Projects\\Kalthuko\\data\\phase0_schema.sql
- bootstrap script: e:\\Projects\\Kalthuko\\setup_phase0_db.ps1
- optional SQL bootstrap loader: e:\\Projects\\Kalthuko\\scripts\\setup_phase0_db_from_sql.ps1

## Current Progress (Phase 0)
- Design and data model defined in DESIGN.md
- Phase 0 DB schema created (SQL) and wired for local bootstrapping
- README updated with DB bootstrap instructions
- Windows bootstrap script available to create the schema

## How to Run (Phase 0)
- Bootstrap inline: powershell -ExecutionPolicy Bypass -File "e:\\\\Projects\\\\Kalthuko\\setup_phase0_db.ps1"
- Bootstrap from SQL: powershell -ExecutionPolicy Bypass -File "e:\\\\Projects\\\\Kalthuko\\\\scripts\\\\setup_phase0_db_from_sql.ps1"
- DB file: e:\\\\Projects\\\\Kalthuko\\\\data\\\\kalthuko_phase0.db

## Next Steps (Phase 1)
- Implement a Rust-based or JS-exposed data access layer via Taura commands
- Extend UI with Profile page and language entry persistence
- Add unit tests for storage and API surface
- Prepare CI/workflow for automated builds and tests
