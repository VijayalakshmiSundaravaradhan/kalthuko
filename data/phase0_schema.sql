-- Phase 0 SQLite schema for Kalthuko
-- Languages
CREATE TABLE languages (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  code TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Profiles
CREATE TABLE profiles (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  language_id INTEGER,
  settings_json TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(language_id) REFERENCES languages(id)
);

-- Lessons
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

-- Settings
CREATE TABLE settings (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  profile_id INTEGER NOT NULL,
  key TEXT NOT NULL,
  value TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(profile_id) REFERENCES profiles(id)
);

-- Views for quick reporting
CREATE VIEW vw_profiles_overview AS
SELECT p.id AS profile_id,
       p.name AS profile_name,
       l.code AS language_code,
       l.name AS language_name,
       (SELECT COUNT(*) FROM lessons WHERE profile_id = p.id) AS lesson_count
FROM profiles p
LEFT JOIN languages l ON p.language_id = l.id;

CREATE VIEW vw_profile_lessons AS
SELECT l.id AS lesson_id,
       l.profile_id,
       l.topic,
       l.kannada,
       l.transliteration,
       l.english,
       l.explanation,
       l.practice,
       l.created_at
FROM lessons l;
