# Persistent Life

NorthPalace-my-pet treats persistence as part of Lenvu's life system, not as a reason to block the Pet Runtime.

## Current scope

V0.2 stores only values that are meaningful across application restarts:

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

This avoids reopening the application in a stale environmental state such as "still walking", "still focused", or "AI worker ready" when those facts are no longer true.

## Storage location

The runtime asks Tauri for its app-local-data directory and stores:

```text
<lenvu app-local-data>/lenvu.sqlite3
```

The database is never stored inside the Git repository or project workspace during normal application use.

## SQLite policy

The first schema uses `PRAGMA user_version = 1` and a singleton `pet_state` table.

Connection policy:

```text
foreign_keys = ON
journal_mode = WAL
synchronous = NORMAL
busy_timeout = 2500 ms
```

The Rust application links a bundled SQLite through `rusqlite`, so the Windows target does not need a separate system SQLite installation.

## Runtime boundary

Database I/O does not run inside Pet Brain's owner loop.

```text
Rust Pet Runtime
      │
      │ immutable snapshot
      ▼
Autosave sampler
      │
      │ state message
      ▼
Persistence worker thread
      │
      ▼
SQLite connection
```

The persistence worker owns the SQLite connection. Pet Runtime only publishes snapshots and never waits for a database query/write during ordinary simulation ticks.

## Startup

```text
resolve app-local-data path
        ↓
open / migrate SQLite
        ↓
load persistent fields
        ↓
combine with fresh transient defaults
        ↓
spawn PetBrainV2
```

If the database or local-data path is unavailable, startup continues with default session-only state and emits diagnostics. Persistence failure is a degraded capability, not a fatal Pet Runtime failure.

## Autosave

The first implementation samples the runtime every 30 seconds and queues a write only when the persistent subset changed.

A fresh database also receives an initial default state. The next persistence iteration should add an explicit final save during graceful application shutdown so the maximum unsaved window is not limited only by the autosave interval.

## Schema V1

```sql
CREATE TABLE pet_state (
  singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
  facing TEXT NOT NULL CHECK (facing IN ('left', 'right')),
  energy REAL NOT NULL CHECK (energy >= 0.0 AND energy <= 1.0),
  curiosity REAL NOT NULL CHECK (curiosity >= 0.0 AND curiosity <= 1.0),
  bond REAL NOT NULL CHECK (bond >= 0.0 AND bond <= 1.0),
  sleep_pressure REAL NOT NULL CHECK (sleep_pressure >= 0.0 AND sleep_pressure <= 1.0),
  updated_at_ms INTEGER NOT NULL
);
```

## What comes next

The same database can later grow separate tables for:

- episodic memory;
- semantic/user facts;
- preferences;
- relationship events/history;
- bounded activity journal;
- configuration metadata;
- FTS5 search indexes.

Those tables must not turn the singleton `pet_state` row into a generic dumping ground.

## Acceptance tests

The current Rust tests verify:

- schema creation and `user_version` migration;
- persistent-state save/load round trip;
- loaded values are combined with fresh transient defaults;
- Pet Runtime can start from a supplied persisted state.

The feature branch also passes the complete Windows CI after adding bundled SQLite.
