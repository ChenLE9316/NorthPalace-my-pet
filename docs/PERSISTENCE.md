# Persistent Life

NorthPalace-my-pet treats persistence as part of Lenvu's life system, not as a reason to block the Pet Runtime.

## Current scope

The long-lived Pet State still stores only values that make sense across restarts:

- `facing`
- `energy`
- `curiosity`
- `bond`
- `sleep_pressure`

Transient runtime state is deliberately reset on startup:

- locomotion and current Behavior Intent;
- posture / current animation phase;
- attention and short-lived emotion;
- Focus Guard / Do Not Disturb / Play mode;
- cognition / AI availability;
- Windows user-idle timing.

The database now also contains separate structures for life history and future memory instead of turning the singleton `pet_state` row into a generic dump.

## Storage location

The runtime asks Tauri for its app-local-data directory and stores:

```text
<lenvu app-local-data>/lenvu.sqlite3
```

The database is never stored inside the Git repository or project workspace during normal application use.

## SQLite policy

Schema version is currently `PRAGMA user_version = 2`.

Connection policy:

```text
foreign_keys = ON
journal_mode = WAL
synchronous = NORMAL
busy_timeout = 2500 ms
```

Rust links bundled SQLite through `rusqlite`, so the Windows target does not need a separate system SQLite installation. The bundled build also gives the project FTS5 support for local memory search.

## Runtime boundary

Database I/O does not run inside Pet Brain's owner loop.

```text
Domain Events ────────────────┐
                              │ filtered, low-frequency
Rust Pet Runtime              ▼
      │                Event-journal bridge
      │ snapshot              │
      ▼                       │
Autosave sampler              │
      │                       │
      └──────────┬────────────┘
                 ▼
         Persistence worker
                 │
                 ▼
              SQLite
```

The persistence worker owns the SQLite connection. Pet Runtime publishes immutable snapshots and domain events; ordinary simulation ticks never wait on SQLite.

## Graceful degradation

`PersistenceService` is always available to the application, but it can internally be disabled when the local-data path or database cannot be initialized.

That means:

```text
SQLite available   -> persistent life + journal + memory infrastructure
SQLite unavailable -> session-only Lenvu, ordinary Pet Runtime continues
```

A missing database must never make petting, movement, sleeping, Focus Guard or UI commands fail.

## Startup order

```text
resolve app-local-data path
        ↓
open / migrate SQLite
        ↓
load long-lived Pet State
        ↓
combine with fresh transient defaults
        ↓
spawn PetBrainV2
        ↓
install PersistenceService
        ↓
subscribe journal bridge
        ↓
start Windows sensors
```

Starting sensors after persistence initialization prevents the first local-time/presence events from being needlessly lost.

## Autosave and shutdown

The runtime samples persistent Pet State every 30 seconds and queues a write only when the persistent subset changed.

On graceful process exit, the application also performs a final `save_and_flush` with a bounded acknowledgement wait. This closes the previous gap where the final interaction could be newer than the most recent autosave.

## Schema V2

### `pet_state`

Singleton long-lived physical/relationship state.

### `activity_journal`

Stores only low-frequency events with real life-history value. Current examples include:

- user returned;
- petting;
- play;
- Focus Guard start/end.

High-frequency/noisy events such as runtime ticks, cursor hover and facing changes are intentionally filtered out.

Retention is bounded by both:

- 30 days;
- maximum 2,000 journal rows.

Whichever removes old rows first wins.

### `relationship_events`

Stores relationship-relevant history separately from the scalar `bond` value, including an event kind and bond delta. Journal retention may detach the original journal row, but relationship history can remain.

### `memories`

Typed local memory records with four categories:

- episodic;
- semantic;
- preference;
- relationship.

Each memory has content, importance, timestamps and an optional source activity event.

### `memory_fts`

FTS5 external-content index over memory text. Insert/update/delete triggers keep the index synchronized with `memories`.

The first retrieval policy ranks by FTS5/BM25 relevance and then uses memory importance and recency as secondary ordering signals.

### `rhythm_hourly`

Stores a lightweight 24-hour interaction profile. A Windows local-time sensor emits `TimeOfDayChanged` only on startup/hour transition; relationship/focus interactions increment the current hour bucket.

This is deliberately much cheaper and less invasive than continuously sampling desktop activity into the database.

## Memory API boundary

Persistence already exposes worker-backed primitives for:

- queueing a typed `MemoryDraft`;
- FTS5 searching with a bounded result count and timeout.

Pet Brain does not decide what becomes a long-term memory. A future Memory Evaluator will sit above this storage boundary and decide whether an event/conversation deserves episodic, semantic, preference or relationship storage.

See `docs/MEMORY_SYSTEM.md`.

## Acceptance tests

Rust tests cover:

- clean schema creation at V2;
- V1 -> V2 migration while retaining `pet_state`;
- persistent Pet State save/load round trip;
- transient state reset on load;
- graceful-shutdown save acknowledgement;
- filtering high-frequency events out of the journal;
- relationship-event insertion;
- memory insertion + FTS5 retrieval;
- hourly rhythm accumulation;
- Pet Runtime startup from supplied persisted state;
- runtime domain-event subscription used by the journal bridge.

All changes must still pass the complete Windows CI before they are considered part of the validated feature-branch baseline.
