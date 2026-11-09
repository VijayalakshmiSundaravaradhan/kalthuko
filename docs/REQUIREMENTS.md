# Kalthuko --- Requirements, Architecture, Data Structures, and 8-Hour MVP Plan

## 1. Project Overview

**Kalthuko** is a local-first, conversational language-learning
application designed to help learners develop spoken fluency in a
configurable target language through a configurable known language.

The application should deliver short, practical lessons, build
progressively on previously taught concepts, and use semantic search to
reduce unnecessary repetition while preserving an accurate history of
what the learner has learned.

**Minimal Viable Prpduct scope:** An 8-hour time-box is intended to produce a working
technical prototype / vertical slice, not a polished production-ready or
fully cross-platform application.

## 2. Functional Requirements

### FR-1 Language Configuration and Profiles

-   Let the learner configure a **known language** and a **target
    language**.
-   Support different language pairs, such as Tamil → Kannada.
-   Represent each language pair as a learning profile.
-   Let the learner return to an existing profile when switching between
    language pairs.
-   Avoid accidentally creating duplicate profiles for the same language
    pair. If independent journeys for the same pair are supported later,
    allow a profile name or journey identifier.

### FR-2 Learning Content

-   Generate short, bite-sized micro-lessons.
-   Focus on natural, locally used, real-world conversational language.
-   Prioritize spoken fluency rather than native-script reading or
    writing.
-   Use intuitive explanations and cues from the learner's mother
    tongue.
-   Use previously taught content as context so new lessons build
    progressively on existing knowledge.
-   Keep all teaching content for both languages in **Latin letters
    only**. Do not display native scripts.
-   Validate generated content before showing or saving it.

### FR-3 Lesson History and Provenance

-   Persist every accepted lesson.
-   Let learners reopen a lesson and see the exact content originally
    presented.
-   Do not regenerate a historical lesson when it is opened.
-   Link every taught word, phrase, or concept to the lesson in which it
    was first explicitly taught.
-   Keep the first-teaching lesson reference immutable when a concept is
    reused in later lessons or practice.
-   Separate immutable lesson content from mutable learner-progress
    data.

### FR-4 Semantic Search and Duplicate Prevention

-   Detect previously taught concepts even when their wording differs.
-   Use a hybrid pipeline:
    1.  Normalize text and perform exact-match checks.
    2.  Retrieve semantically similar concepts using vector similarity
        search.
    3.  Use an LLM to judge likely matches only when needed.
    4.  Accept, replace, or reject candidates based on the result.
-   Search concepts using meaning, communicative intent, and
    context---not only the target-language phrase.
-   Distinguish unnecessary repetition from useful variation, such as
    different registers or contexts.
-   Allow known phrases to be reused for practice without treating every
    reuse as new teaching.
-   Exact duplicate constraints can be deterministic; semantic
    deduplication reduces repetition but cannot guarantee that semantic
    duplicates will never occur.

### FR-5 Progress and Daily Lessons

-   Track which concepts have been introduced and learner progress for
    each profile.
-   Keep progress separate from saved lesson snapshots.
-   Provide a **New Lesson** action.
-   Offer a new lesson each day, on demand or through a daily
    notification.
-   For the initial prototype, daily notification scheduling and
    background generation may be deferred.


### FR-6 Boot Time

-   Application must start within 2 seconds
    in source control. Avoid logging secrets or unnecessary sensitive
    request content.

## 3. UI Requirements

### UIR-1 Profile Setup

-   First-run flow asks for the known language and target language.
-   Existing profiles can be selected.
-   A new language pair creates or selects the corresponding profile.

### UIR-2 Profile Dashboard

-   Show the active language-pair profile.
-   Show lesson history.
-   Show learned-content/progress information.
-   Provide a **New Lesson** action.

### UIR-3 Lesson View

-   Display short conversational expressions.
-   Provide an option for user to generate a new lesson
-   Include meanings and mother-tongue explanations/cues.
-   Ensure teaching content uses Latin letters only.
-   Opening a saved lesson displays the stored snapshot, not newly
    generated content.

### UIR-4 Phrase / Concept Detail

-   Let the learner select a learned phrase or concept.
-   Show its meaning and useful context.
-   Link to the lesson where it was first explicitly taught.

### UIR-5 Daily Lesson Experience

-   Offer a new lesson when requested.
-   Notify the user when a new lesson is generated.
-   Avoid silently generating lessons every midnight in the MVP;
    generation can wait until the learner opens the app or requests a
    lesson.

### UIR-6 Authentication (Login)

-   **Login page** with fields for Name, Email, and Password.
-   If the user forgets their password, provide an option to reset it via a link sent to their registered email address.
-   Secure handling of credentials without storing passwords in plain text.

### UIR-7 Profile Creation & Selection

-   **First-run flow**: Prompt the learner to configure a **known language** and a **target language**.
-   Collect both language pairs explicitly during setup.
-   Prevent accidental duplicate profile creation for the same language pair.
-   Allow returning to an existing profile when switching between language pairs.

### UIR-8 Lesson Generation Logic

-   When a profile is selected, display whether the next lesson is newly generated (since first app view) or the last lesson previously viewed.
-   Generate short, bite-sized micro-lessons using the LLM workflow.
-   Validate generated content against the Latin-only teaching policy before saving.

### UIR-9 Lesson History & Navigation

-   Provide an option to view lesson history with clear visual indicators:
    -   Lessons viewed but not yet completed (open icon)
    -   Completed lessons (closed icon)
    -   Unopened lessons (greyed-out icon)
-   Enable navigation to the latest lesson at any time.
-   Show progress per profile (completed lessons, total lessons).

### UIR-10 Profile Management

-   Switch between different language-pair profiles seamlessly.
-   View detailed profile information including:
    -   Known and target languages
    -   List of lessons generated under this profile
    -   Lessons completed vs. those still pending
-   Support for logging out of the application.

### UIR-11 Logout

-   Clear all local data and terminate the session upon logout.
-   Ensure no lingering references to active sessions remain.

## 4. Technical Decisions

-   **Frontend:** React + TypeScript
-   **Desktop/mobile application framework:** Tauri
-   **Application workflow:** Rust commands and explicit workflow
    orchestration
-   **Primary persistence:** Local SQLite database.
-   **Semantic retrieval:** Vector index over canonical learning
    concepts and semantic descriptions.
-   **LLM provider:** OpenRouter, using the learner/developer's existing
    API key.
-   **Provider abstraction:** Keep generation and embedding providers
    replaceable.
-   **Startup target:** Aim for UI launch in under two seconds. Load the
    UI and local data first; do not block startup on network calls, LLM
    initialization, or embedding work.
-   **Development platform:** Windows initially.
-   **Future portability:** Structure the app so Android support through
    Tauri can be evaluated later.
-   **Local-first design:** SQLite is the authoritative record. The
    vector index is a rebuildable search index.
-   **Embedding dependency:** Verify the availability, quality,
    dimensions, and cost of the chosen embedding model. OpenRouter
    chat-model access does not automatically imply free embedding
    access.
-   **Security:** Do not commit API keys or persist them in plain text

## 5. Recommended Data Structures

Use relational tables for authoritative records and a vector index for
semantic retrieval.

### 5.1 `profiles`

Represents a language-pair learning journey.

Fields:

-   `id` --- primary key.
-   `profile_name` --- unique
-   `known_language` --- configured source language.
-   `target_language` --- configured learning language.
-   `created_at` --- creation timestamp.
-   `learning_frequency` --- The pace in terms of lesson generation - once in a day, two days, 3 days,.. a week

Use a uniqueness rule for the language pair if duplicate profiles are
not desired.

### 5.2 `lessons`

Stores the exact content shown to the learner.

Fields:

-   `id` --- primary key.
-   `profile_id` --- foreign key to `profiles`.
-   `created_at`.
-   `objective` --- lesson objective.
-   `content_json` --- immutable snapshot of the complete accepted
    lesson.
-   `introduced_words` --- What new word(s) have been taught in this lesson
-   `viewed` --- partial, complete, not yet opened.

### 5.3 `learner_progress`

Stores mutable learner progress separately from lesson content.

Suggested fields:

-   `profile_id`.
-   `last_lesson_id`.

Add a unique constraint on `(profile_id, lesson_id)`.

### 5.4 Vector Index

-   Store embeddings keyed by `lesson_id`.
-   Include or filter by `profile_id`.
-   Record the embedding model identifier and dimensions.
-   Use one consistent embedding model for indexed descriptions and
    queries.
-   Make the index rebuildable from canonical concept data in SQLite.
-   Evaluate SQLite with a vector extension such as `sqlite-vec`; keep
    the vector implementation behind a Rust interface so it can be
    replaced later.

## 6. Semantic Search and Lesson Generation Workflow

1.  The learner selects a profile and requests a new lesson.
2.  Rust loads the profile, recent lesson context, known concepts, and
    progress from SQLite.
3.  The LLM generates a compact set of candidate expressions in
    structured JSON.
4.  Validate the response schema and Latin-only teaching-content policy.
5.  Normalize candidate text and check exact matches / database
    uniqueness.
6.  Create or use a semantic description for each candidate concept.
7.  Generate an embedding with the selected embedding provider.
8.  Retrieve the nearest concepts from the active profile's vector
    index.
9.  If a candidate appears similar to prior concepts, ask the LLM to
    judge whether it is redundant or meaningfully distinct.
10. Replace or reject redundant candidates; preserve useful variations.
11. Save the accepted lesson snapshot, concepts, lesson items, and
    progress changes transactionally in SQLite.
12. Update the vector index after the authoritative database write. If
    indexing fails, record/retry it and allow rebuilding from SQLite.
13. Show the saved lesson in the UI.

**Important distinction:** Exact matching and database uniqueness can
prevent exact duplicates deterministically. Semantic retrieval plus LLM
judgment can greatly reduce semantic repetition, but cannot guarantee
perfect zero repetition.

## 7. Mermaid Architecture Diagram

``` mermaid
flowchart TB
    Learner([Learner])

    subgraph UI["Presentation Layer"]
        React["React + TypeScript UI"]
        ProfileUI["Profiles and Language Settings"]
        Dashboard["Dashboard and Lesson History"]
        LessonUI["Lesson View and Phrase Links"]
        React --> ProfileUI
        React --> Dashboard
        React --> LessonUI
    end

    subgraph App["Tauri Application Layer"]
        Tauri["Tauri Bridge"]
        Rust["Rust Commands and Application Services"]
        Workflow["Explicit Lesson Workflow"]
        Validator["Schema and Latin-Only Validation"]
        Exact["Normalization and Exact Duplicate Check"]
        Semantic["Semantic Retrieval and Similarity Ranking"]
        Judge["Conditional LLM Duplicate Judgment"]
        Persist["Transactional Persistence"]
        Tauri --> Rust
        Rust --> Workflow
        Workflow --> Validator
        Validator --> Exact
        Exact --> Semantic
        Semantic --> Judge
        Judge --> Persist
    end

    subgraph Local["Local Data Layer"]
        SQLite[("SQLite: Profiles, Lessons, Concepts, Lesson Items, Progress")]
        Vector[("Vector Index: Concept Embeddings")]
    end

    subgraph External["External AI Services"]
        OpenRouter["OpenRouter LLM API"]
        Embeddings["Embedding Provider or Local Embedding Model"]
    end

    Learner --> React
    React <--> Tauri
    Rust <--> SQLite
    Semantic <--> Vector
    Workflow -->|"Generate candidates"| OpenRouter
    Judge -->|"Judge only likely matches"| OpenRouter
    Semantic -->|"Embed query/concept description"| Embeddings
    Persist -->|"Save authoritative records"| SQLite
    Persist -->|"Index accepted concepts after DB commit"| Vector
    SQLite -.->|"Rebuild vector index when needed"| Vector
```

### Architecture Notes

-   React should not access SQLite or external AI APIs directly; use the
    Tauri/Rust application layer.
-   Rust coordinates the workflow and applies validation before content
    is persisted.
-   SQLite owns profiles, lesson snapshots, concepts, lesson items, and
    progress.
-   The vector index supports nearest-neighbor retrieval and can be
    rebuilt from SQLite.
-   The LLM generates candidate content and judges ambiguous duplicates;
    it does not decide what is persisted without application validation.
-   Keep embedding generation and indexing off the critical app-startup
    path.

## 8. Eight-Hour MVP Development Plan

  ------------------------------------------------------------------------
  Time                    Work item               Deliverable
  ----------------------- ----------------------- ------------------------
  0:00--0:45 (45 min)     Bootstrap and           Verify existing
                          architecture            React/Tauri build,
                                                  environment
                                                  configuration, and
                                                  provider interface.

  0:45--2:00 (75 min)     SQLite schema and       Migrations and
                          persistence             repository methods for
                                                  profiles, lessons,
                                                  concepts, lesson items,
                                                  and progress.

  2:00--3:00 (60 min)     OpenRouter lesson       Structured JSON
                          generation              generation, schema
                                                  validation, and
                                                  Latin-only content
                                                  validation.

  3:00--4:15 (75 min)     Semantic embeddings and Verified embedding
                          retrieval               source, vector indexing,
                                                  nearest-concept
                                                  retrieval, and
                                                  exact-match checks.

  4:15--5:15 (60 min)     Duplicate decision and  Conditional LLM
                          save flow               judgment, candidate
                                                  replacement/rejection,
                                                  and transactional
                                                  persistence.

  5:15--6:30 (75 min)     Core React UI           Profile selection,
                                                  dashboard, New Lesson
                                                  action, lesson history,
                                                  and lesson detail view.

  6:30--7:15 (45 min)     Provenance and progress First-teaching lesson
                                                  links and progress
                                                  updates separate from
                                                  lesson snapshots.

  7:15--8:00 (45 min)     End-to-end testing      Test duplicates,
                                                  distinct phrases, API
                                                  errors, restart
                                                  persistence, and startup
                                                  behavior.
  ------------------------------------------------------------------------

**Total: 8 hours.** This estimate assumes an existing project scaffold
and a functioning development environment. If embedding integration or
Tauri setup is not already available, reduce UI polish or extend the
estimate.

## 9. MVP Acceptance Criteria

-   [ ] The learner can create or select a language-pair profile.
-   [ ] A lesson can be generated and saved locally.
-   [ ] Reopening a lesson shows the exact original snapshot.
-   [ ] A candidate retrieves previously taught concepts by semantic
    similarity, not only exact text.
-   [ ] The system can reject a redundant candidate and accept a
    meaningfully distinct candidate.
-   [ ] Each taught concept links to its first teaching lesson.
-   [ ] Teaching content is validated against a Latin-only policy, with
    explicit handling for punctuation, numbers, and other non-letter
    symbols.
-   [ ] Progress persists independently from historical lesson content.
-   [ ] The UI can launch and display local data without waiting for the
    LLM or embedding provider.
-   [ ] Vector-index failure does not destroy authoritative learning
    data.

## 10. Explicitly Out of Scope for the First 8 Hours

-   Android packaging and background notifications.
-   Speech recognition, pronunciation scoring, or text-to-speech.
-   Sophisticated spaced repetition.
-   Multiple embedding providers and automatic failover.
-   A comprehensive lesson-quality evaluation framework.
-   A guarantee of zero semantic repetition.

## 11. Implementation Priorities

1.  **Prove semantic search early.** Validate the embedding source and
    vector extension before investing heavily in UI polish.
2.  **Keep SQLite authoritative.** The vector index must be rebuildable.
3.  **Search canonical concepts, not just phrases.** Use meaning,
    intent, and context to handle wording and transliteration
    differences.
4.  **Keep workflow orchestration explicit in Rust.** No LangGraph in
    the initial implementation.
5.  **Preserve provenance.** The first-teaching lesson reference must
    remain stable even when a concept appears again.
6.  **Optimize startup separately from generation.** Load the UI and
    local records first; perform network/model work only when requested.
