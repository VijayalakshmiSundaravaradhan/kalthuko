Phase 0 Progress Update

Current implementation highlights:
- Frontend: React UI present in src (App.jsx, main.jsx, styles.css) with Tauri integration.
- Backend integration: The UI calls a Tauri command teach_kannada which routes to the OpenRouter gemma backend (gemma-4-31b-it:free).
- Build/run: package.json includes scripts for tauri and vite; dev uses `tauri dev` and `vite` for web.
- Data storage: No local DB implemented yet; storage layer to be introduced in Phase 1.
- Documentation: PLAN.md exists outlining phased milestones.

Planned vs implemented:
- Phase 0 implemented: Core UI scaffold and model invocation.
- Phase 0 storage: Planned for Phase 1.
- Phase 1+: DB schema and persistence layer, API bridge, and lesson scheduling.

Next steps:
- Define and implement a local storage schema (SQLite/IndexedDB) for lessons, profiles, languages, and settings.
- Create a data access layer and expose a clean API to the frontend (via Tauri commands).
- Add basic tests for the new storage layer and API surface.
- Document progress in PROGRESS.md and ensure README has run instructions.
