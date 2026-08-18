# NorthPalace-my-pet Foundation Review

> **Historical review record.** This document captures the first foundation review. Current engineering behavior lives in `docs/ARCHITECTURE.md` and `docs/ROADMAP.md`.

## Decisions that survived

- Tauri 2 desktop shell.
- Rust runtime/core.
- Svelte + TypeScript application UI.
- PixiJS pet renderer.
- SQLite + FTS5 local persistence/search.
- Future optional MiniCPM5-1B + llama.cpp cognition layer.
- Separate LLM worker process.
- Pet-first, AI-second philosophy.
- Windows 11 / Ryzen 3 2200G / 16 GB / Vega 8 primary target.

## Original corrections and current status

### Runtime clock belongs to Rust

Implemented. Pet Brain no longer depends on a JavaScript timer.

### Pet state is parallel

Implemented as locomotion, facing, posture, attention, emotion, mode and cognition.

### Behavior Intent is separate from persistent state

Implemented with priority, TTL, interruption and semantic animation intent.

### Domain events form the backbone

Implemented and expanded with held/drop/facing and structured Windows-context events.

### UI is not the source of truth

Implemented. Frontend timers are presentation polling only and do not advance simulation time.

### Window boundaries

`pet` and `companion` are separate native windows. Bubble remains inside the pet overlay; Settings currently lives inside Companion; dedicated debug remains future work.

### Transparent does not mean click-through

Implemented through native selective cursor passthrough and semantic hit zones. Production per-frame masks remain future asset work.

### Renderer boundary

Implemented as manifest + semantic resolver + PixiJS. Production character art is still pending.

### Error state must be explicit

Implemented as a small truthful runtime-health contract: `ready`, `degraded`, `error`. Event-channel disconnect preserves a degraded snapshot; a Pet Runtime panic preserves the last published snapshot and marks it error. A `recovering` state is intentionally deferred until a real supervisor/restart mechanism exists.

### Avoid one global Pet Brain lock

Implemented as a single-owner runtime channel/snapshot model. Background worker lifecycle/supervision remains consolidation work before AI workers.

## Asset-pipeline decision

Reference art and runtime assets are different products. Original source evidence defines identity; canonical coordinates normalize production; runtime sprites/atlases are downstream outputs.

The governance layer is now intentionally frozen unless new source evidence exposes a concrete missing requirement. The next visual work should produce the canonical master and runtime assets rather than more policy documents.

## Public-release items still open

- code and artwork licenses;
- dependency lockfiles;
- `SECURITY.md`;
- restrictive CSP;
- production icon/art;
- clean Windows bundle validation;
- target-hardware performance baseline;
- tracked-data/privacy audit.

## Non-negotiable acceptance test

Unload MiniCPM5-1B and any future vision worker. Lenvu still moves, sleeps/wakes, reacts, focuses, persists state and supports ordinary desktop interaction.
