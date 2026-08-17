# Memory System

NorthPalace-my-pet separates **storage** from **memory judgement**.

SQLite can store and retrieve memories, but it does not decide what deserves to become a memory. That decision belongs to a future Memory Evaluator above the persistence layer.

## Design goals

- local-first and offline by default;
- cheap enough for Ryzen 3 2200G + 16 GB DRAM;
- no vector database requirement for the first releases;
- no requirement to keep MiniCPM5-1B loaded just to remember ordinary events;
- bounded event history so background usage does not grow without limit;
- explicit user-editable memory later;
- AI failure must never break Pet Brain or existing stored memories.

## Memory layers

```text
Immediate runtime state
      │
      │ milliseconds -> minutes
      ▼
Behavior / attention context
      │
      │ selected meaningful events
      ▼
Bounded activity journal
      │
      ├───────────────┐
      ▼               ▼
Relationship history  Memory Evaluator [planned]
                      │
                      ▼
                Typed long-term memory
                      │
                      ▼
                  SQLite + FTS5
```

## Typed long-term memory

### Episodic

Specific events tied to a moment or experience.

Examples:

```text
The user took a break after a long focus session.
The user was happy after completing a project milestone.
```

### Semantic

Stable facts or distilled knowledge about the user/world.

Examples:

```text
The user usually works on Windows.
NorthPalace-my-pet is the user's local desktop companion project.
```

### Preference

Explicit or repeatedly demonstrated preferences.

Examples:

```text
The user prefers quiet companion behavior while focusing.
The user prefers local-first tools over cloud-only dependencies.
```

### Relationship

Facts about the bond and recurring interaction pattern between the user and Lenvu.

Examples:

```text
The user often pets Lenvu after returning to the computer.
Lenvu has a strong positive association with evening focus sessions.
```

## What does not automatically become memory

The following should normally remain transient or bounded journal data:

- every cursor hover;
- every runtime tick;
- every active-window change;
- raw notification streams;
- every line of chat;
- raw screenshots;
- temporary AI chain-of-thought or worker diagnostics;
- short-lived animation state.

Keeping everything is not memory; it is uncontrolled logging.

## Storage schema

The current V2 persistence layer provides:

```text
activity_journal
relationship_events
memories
memory_fts
rhythm_hourly
```

`memories` uses a single normalized table with a `kind` discriminator rather than four near-duplicate physical tables. This keeps migrations, FTS indexing and retrieval simple while preserving explicit episodic/semantic/preference/relationship semantics.

## Retrieval V1

The first local retrieval path uses SQLite FTS5:

```text
query
  ↓
memory_fts MATCH
  ↓
BM25 text relevance
  ↓
importance
  ↓
recency
  ↓
bounded result set
```

This is intentionally enough for the first MiniCPM5-1B integration.

A vector database is not justified until real usage demonstrates that lexical + metadata retrieval is insufficient.

## Memory Evaluator [planned]

The evaluator receives normalized candidate information, not raw system noise.

Example contract:

```text
MemoryCandidate
├─ source
│  ├─ conversation
│  ├─ relationship_event
│  ├─ user_explicit
│  └─ significant_activity
├─ content
├─ candidate_kind
├─ confidence
├─ novelty
├─ emotional_weight
├─ repetition
└─ user_visibility
```

Possible decision:

```text
candidate
   ↓
rule prefilter
   ↓
existing-memory lookup
   ↓
novel / repeated / important?
   ├─ no  -> discard or journal only
   └─ yes -> store / merge / update
```

MiniCPM5-1B may later help classify or summarize complex candidates, but simple explicit facts and relationship events should not require an LLM call.

## Relationship history

`bond` remains a fast scalar used by Pet Brain.

It is not enough to explain *why* the bond changed, so `relationship_events` separately records low-frequency meaningful events such as affection, play and reunion.

This gives us two different tools:

```text
bond
= fast current relationship intensity

relationship_events
= explainable life history
```

Long-term personality/bond evolution can later derive features from this history without making Pet Brain query the database every tick.

## Rhythm profile

`rhythm_hourly` stores a deliberately small 24-hour interaction profile.

The first implementation records:

- the local hour when Lenvu is active;
- count of meaningful interactions in each hour bucket;
- last observation timestamp.

Future behavior can use this as one signal for morning/evening routines, but it must not become invasive surveillance.

## Privacy rules

Memory persistence should default to the minimum data needed for companion continuity.

Important boundaries:

- foreground window titles are not stored by default;
- screenshots are never placed in memory automatically;
- raw clipboard contents are not memory inputs by default;
- app identity should not become long-term preference data without a clear reason;
- user must later be able to inspect/delete/edit long-term memories;
- disabling AI must not make stored memory inaccessible to basic management tools.

## Future Memory Browser

The Companion window should eventually provide:

```text
Memory
├─ Search
├─ Episodic
├─ Facts
├─ Preferences
├─ Relationship
└─ Activity History
```

Each long-term memory should expose:

- kind;
- text;
- importance;
- created/updated time;
- source where appropriate;
- edit/delete controls.

The interface should make the difference between **temporary journal history** and **long-term memory** visible to the user.
