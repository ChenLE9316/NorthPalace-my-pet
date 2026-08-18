# NorthPalace-my-pet Foundation Review

> **Historical review record.** This document captures the first foundation review and the corrections that originally shaped the project. Many items below have since been implemented or superseded. For current behavior and architecture, use `docs/ARCHITECTURE.md` and `docs/ROADMAP.md`.

## Decisions that survived the review

- Tauri 2 desktop shell.
- Rust runtime/core.
- Svelte + TypeScript for application UI.
- PixiJS for the pet renderer.
- SQLite + FTS5 for local persistence/search.
- MiniCPM5-1B + llama.cpp as a future optional local cognition layer.
- Separate LLM worker process.
- Pet-first, AI-second product philosophy.
- Primary target: Windows 11 / Ryzen 3 2200G / 16 GB DRAM / Vega 8.

## Original corrections

### 1. Runtime clock belongs to Rust

The Pet Brain must not depend on a JavaScript timer to stay alive. Rust owns monotonic time and semantic simulation ticks. Frontend code renders snapshots and issues commands.

**Current status:** implemented.

### 2. Pet state is parallel, not one activity enum

Movement, facing, posture, attention, emotion, system mode and cognition can coexist.

**Current status:** implemented as Pet State V2.

### 3. Behavior Intent is separate from persistent state

Short reactions require priority, duration/TTL, interruption semantics and renderer-facing animation intent.

**Current status:** implemented.

### 4. Domain events form the backbone

Sensors publish facts and Pet Runtime interprets them.

**Current status:** implemented and expanded with held/drop/facing and Windows context events.

### 5. UI is not the source of truth

Svelte issues commands and renders state. It does not own simulation time, persistence policy or Windows sensors.

**Current status:** implemented; frontend polling remains presentation-only.

### 6. Window boundaries

The review proposed separate logical responsibilities for pet, bubble, companion, settings and debug surfaces.

**Current status:** `pet` and `companion` are separate native windows. Bubble remains inside the pet overlay. Settings currently lives inside the Companion. A dedicated debug surface is still future work.

### 7. Transparent does not mean click-through

Transparent space must pass pointer input through while semantic pet regions remain interactive.

**Current status:** implemented through native selective cursor passthrough with normalized semantic hit regions. Production per-frame masks remain future asset work.

### 8. Renderer boundary

PixiJS owns high-frequency character presentation; Svelte owns management UI.

**Current status:** implemented as a renderer/manifest boundary, but the actual character remains a procedural placeholder until production assets are authored.

### 9. Error state must be explicit

Runtime failure must not silently imitate a healthy idle pet.

**Current status:** runtime-health contract exists, but full recovering/error transition semantics remain consolidation work.

### 10. Avoid one global Pet Brain lock

The target was a single-owner Pet Runtime receiving events and publishing immutable snapshots, with SQLite/model/platform work outside that owner.

**Current status:** implemented for Pet Runtime; background worker lifecycle/supervision still needs consolidation before AI workers are introduced.

## Asset-pipeline decision

Reference art and runtime assets are different products. Original source evidence defines identity; canonical engineering coordinates normalize production; runtime sprites/atlases are downstream outputs.

The current repository has source provenance, canonical measurement/remap contracts and validation gates. That governance layer is now intentionally frozen unless new source evidence exposes a concrete missing requirement. The next visual work should produce the canonical master and runtime assets rather than additional policy documents.

## Public-release items that remain open

- code license;
- separate character/artwork license;
- dependency lockfiles;
- `SECURITY.md`;
- restrictive CSP;
- production icon/art;
- clean Windows bundle validation;
- Ryzen 3 2200G performance baseline;
- final tracked-data/privacy audit.

## Non-negotiable acceptance test

Unload MiniCPM5-1B and any future vision worker completely. Lenvu still moves, sleeps/wakes, reacts, focuses, persists state and supports ordinary desktop interaction.
