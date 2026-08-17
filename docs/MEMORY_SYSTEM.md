# Memory System

NorthPalace-my-pet separates **storage**, **manual memory management** and **automatic memory judgement**.

SQLite can already store, search and edit long-term memories. The user can explicitly tell Lenvu what to remember through the Companion window without loading MiniCPM5-1B. Automatic decisions about what deserves to become a long-term memory still belong to a future Memory Evaluator above the persistence layer.

## Design goals

- local-first and offline by default;
- cheap enough for Ryzen 3 2200G + 16 GB DRAM;
- no vector database requirement for the first releases;
- no requirement to keep MiniCPM5-1B loaded just to inspect or edit memory;
- bounded event history so background usage does not grow without limit;
- explicit user-editable memory;
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
                      ▲
                      │
              Memory Browser V1
          CRUD / search / provenance
```

## Typed long-term memory

### Episodic
Specific events tied to a moment or experience.

### Semantic
Stable facts or distilled knowledge about the user/world.

### Preference
Explicit or repeatedly demonstrated preferences.

### Relationship
Facts about the bond and recurring interaction pattern between the user and Lenvu.

## What does not automatically become memory

The following normally remain transient or bounded journal data:

- every cursor hover;
- every runtime tick;
- every active-window change;
- raw notification streams;
- every line of chat;
- raw screenshots;
- temporary AI reasoning/worker diagnostics;
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

`memories` uses one normalized table with a `kind` discriminator rather than four near-duplicate physical tables. This keeps migrations, FTS indexing and retrieval simple while preserving explicit episodic/semantic/preference/relationship semantics.

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

A vector database is not justified until real use demonstrates that lexical + metadata retrieval is insufficient.

## Memory Browser V1 — implemented

The Companion window exposes explicit local memory management:

```text
Memory
├─ recent list
├─ FTS5 search
├─ kind filter
├─ manual “remember this” creation
├─ edit content
├─ edit kind
├─ edit importance
├─ source-event provenance
└─ delete / forget
```

The four editable kinds are:

- episodic / 事件;
- semantic / 事實;
- preference / 偏好;
- relationship / 關係.

Manual memory management is deliberately independent of AI. If MiniCPM5-1B is unloaded, the user can still inspect, create, edit, search and delete memories.

### I/O boundary

Continuous runtime persistence still belongs to the dedicated persistence worker. User-explicit Memory Browser CRUD and Activity History reads open short-lived SQLite admin connections only when a management action occurs.

```text
Pet Runtime snapshots/events
        ↓
Persistence worker         Companion user action
        ↓                          ↓
long-lived connection       short-lived admin connection
        └──────────────┬───────────┘
                       ↓
                 WAL SQLite DB
```

This keeps SQLite management calls outside the Pet Brain owner loop while allowing direct UI operations. `busy_timeout` is used so occasional writer overlap degrades into a bounded wait rather than uncontrolled blocking.

## Activity History and provenance — implemented

The Companion window now exposes a bounded Activity History based on the journal that persistence already stores. It does **not** introduce a new telemetry stream.

Current journal-visible events are intentionally low frequency:

```text
user_returned
pet_petted
pet_play
focus_started
focus_ended
```

Where a journal row also created a relationship event, the UI can show relationship context such as `reunion`, `affection` or `play`, together with the associated bond delta.

Long-term memories already have an optional `source_event_id`. The UI resolves this ID back to the journal when present:

```text
memory
  ├─ source_event_id = null
  │      └─ provenance: manually created / explicit
  │
  └─ source_event_id = journal row
         ↓
   Activity History
         ├─ event type
         ├─ category
         ├─ relationship kind
         ├─ bond delta
         └─ timestamp
```

This means a future Memory Evaluator can produce explainable automatic memories without changing the storage model. If an old journal row is removed by retention, the foreign key uses `ON DELETE SET NULL`, so long-term memory survives while stale provenance is safely detached.

Activity History remains bounded by the persistence retention policy (currently 30 days / 2,000 rows) and excludes high-frequency cursor/runtime/window noise.

## Memory Evaluator [planned]

The evaluator receives normalized candidates, not raw system noise.

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

MiniCPM5-1B may later help classify or summarize complex candidates, but explicit user memories and simple relationship events should not require an LLM call.

## Relationship history

`bond` remains a fast scalar used by Pet Brain, while `relationship_events` records explainable low-frequency history such as affection, play and reunion.

```text
bond
= fast current relationship intensity

relationship_events
= explainable life history
```

## Rhythm profile

`rhythm_hourly` stores a deliberately small 24-hour interaction profile: local hour, meaningful interaction count and latest observation time. It is a behavior signal, not a desktop-surveillance log.

## Privacy rules

Memory persistence defaults to the minimum data needed for companion continuity:

- foreground window titles are not stored by default;
- screenshots are never placed in memory automatically;
- raw clipboard contents are not memory inputs by default;
- app identity should not become long-term preference data without a clear reason;
- Activity History only exposes the already-filtered low-frequency journal;
- long-term memories are user-inspectable and editable;
- disabling AI does not disable basic memory/history management.
