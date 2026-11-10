# Kalthuko Phase 0 Design

Phase 0 focuses on UI scaffold, storage schema, and profile language entry. The scope is intentionally limited to the local storage design and the frontend-backend integration via Tauri.

## System overview
- Frontend: React + Vite UI (src/App.jsx, src/main.jsx, src/styles.css)
- Backend bridge: Tauri commands (Rust) that orchestrate data access
- Persistence: Local SQLite database (phase 0) for profiles, languages, and lessons
- Language model integration: Backend calls to the LLM provider (Gemma/OpenRouter) to generate lesson content as in Phase 0

## Architecture diagram

```mermaid
graph TD
  UI[Frontend (React + Vite)] --> TAURI[Tauri Bridge]
  TAURI --> DB[(SQLite DB)]
  TAURI --> LM[LLM Backend (Gemma/OpenRouter)]
  LM --> TAURI
  TAURI --> UI
```

## Data model (Phase 0)
Entities and their core attributes are sufficient to support Phase 0 flows: create language, create a profile, and store generated lessons.

### Tables
```sql
-- Languages supported by the UI
CREATE TABLE languages (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  code TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- User profiles with an associated language
CREATE TABLE profiles (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  language_id INTEGER,
  settings_json TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(language_id) REFERENCES languages(id)
);

-- Lessons generated for a given profile
CREATE TABLE lessons (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  profile_id INTEGER NOT NULL,
  topic TEXT,
  kannada TEXT,
  transliteration TEXT,
  english TEXT,
  explanation TEXT,
  practice TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(profile_id) REFERENCES profiles(id)
);

-- Per-profile key-value settings
CREATE TABLE settings (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  profile_id INTEGER NOT NULL,
  key TEXT NOT NULL,
  value TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(profile_id) REFERENCES profiles(id)
);
```

### Views
```sql
-- Lightweight overview of profiles with language context and lesson counts
CREATE VIEW vw_profiles_overview AS
SELECT p.id AS profile_id, p.name AS profile_name,
       l.code AS language_code, l.name AS language_name,
       (SELECT COUNT(*) FROM lessons WHERE profile_id = p.id) AS lesson_count
FROM profiles p LEFT JOIN languages l ON p.language_id = l.id;

-- All lessons with their parent profile
CREATE VIEW vw_profile_lessons AS
SELECT l.id AS lesson_id, l.profile_id, l.topic, l.kannada, l.transliteration, l.english, l.explanation, l.practice, l.created_at
FROM lessons l;
```

## Indexes and constraints
- Indexes on commonly queried columns (to be created by setup script):
  - languages(code)
  - profiles(language_id)
  - lessons(profile_id)

## Tauri API mapping (Phase 0)
The frontend will call the following Taura commands exposed by the Rust backend:
- db_init_schema: initialize the schema (optional on first run)
- db_insert_language(code, name)
- db_insert_profile(name, language_id, settings_json)
- db_insert_lesson(profile_id, topic, kannada, transliteration, english, explanation, practice)
- db_get_profiles()
- db_get_lessons(profile_id)
- db_update_profile_settings(profile_id, key, value)

Note: Phase 1 will introduce a richer API surface; Phase 0 keeps surface small and targeted.

## Migration plan (Phase 0 only)
- Create the initial SQLite DB at data/kalthuko_phase0.db and apply the schema above.
- The app should create the database on first run if missing; otherwise use the existing DB.

## Non-functional considerations
- Data integrity via FK constraints; indexes for read-heavy access patterns.
- Simple, local storage design to allow offline use.
- App-side updates to updated_at are recommended when updating rows.

## Implementation notes
- Data path: data/kalthuko_phase0.db within the project directory during development; in production, adapt to a user data directory.
- This design does not implement user authentication; profiles are simple user contexts.
