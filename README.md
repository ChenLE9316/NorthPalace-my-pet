# NorthPalace-my-pet

**Lenvu — local-first Neralune digital companion for Windows.**

NorthPalace-my-pet is a lightweight desktop-pet and local AI companion designed around **AMD Ryzen 3 2200G + 16 GB DRAM + Vega 8**. Lenvu remains a complete interactive desktop creature when every AI worker is unloaded; local AI is an optional cognition layer, not the animation controller and not the source of life.

![NorthPalace-my-pet system overview](docs/assets/lenvu-system-overview.webp)

> Current development status: **V0.2 — living desktop pet + Persistent Life V2 foundation**. Foundation-level changes are validated on a clean Windows CI runner before they are treated as stable feature-branch baseline.

## Product principles

- **Pet first, chat second** — animation, expression and behavior communicate before text.
- **Always-lightweight** — the ambient desktop pet must remain cheap enough to run all day.
- **Event driven** — Windows, pointer, focus, time and user interactions become domain events.
- **Rust owns life-time** — Pet Runtime does not depend on JavaScript simulation timers or an open panel.
- **Parallel state** — movement, facing, posture, attention, emotion, mode and cognition coexist.
- **LLM is not the reflex layer** — ordinary pet behavior is instant and fully offline.
- **Vision is optional** — structured Windows context comes first; screen vision is future, opt-in and on-demand.
- **Local first** — pet state, relationship, memories, logs and model execution stay on-device by default.
- **Graceful degradation** — AI, database or sensor failures degrade capabilities without killing Lenvu's ordinary life loop.

## Current architecture

```text
Windows 11 / Ryzen 3 2200G / 16 GB DRAM / Vega 8
|
+-- Tauri 2 desktop process
|   +-- Rust Pet Runtime
|   |   +-- monotonic 250 ms clock
|   |   +-- Domain Event channel + low-frequency observers
|   |   +-- PetBrainV2 / parallel Pet State / Behavior Intent
|   |   `-- immutable Runtime Snapshot
|   +-- Windows / desktop-space adapters
|   |   +-- idle + user-return sensor
|   |   +-- foreground-app identity sensor
|   |   +-- local-hour sensor
|   |   +-- monitor / DPI / work-area context
|   |   +-- selective native cursor passthrough
|   |   `-- native autonomous + explicit drag window motion
|   +-- Persistent Life
|   |   +-- app-local-data / lenvu.sqlite3
|   |   +-- schema V2 migrations
|   |   +-- separate DB-owning worker
|   |   +-- autosave + graceful final flush
|   |   +-- bounded activity journal + relationship history
|   |   +-- typed memories + FTS5
|   |   `-- hourly interaction rhythm
|   `-- WebView windows
|       +-- pet       -> transparent / always-on-top / PixiJS
|       `-- companion -> independent Svelte window
|
+-- Lenvu presentation contract
|   +-- versioned animation manifest
|   +-- semantic animation resolver
|   +-- facing-aware semantic hit zones
|   `-- PixiJS low-power renderer
|
`-- Optional AI Workers [planned]
    +-- Text: llama.cpp -> MiniCPM5-1B GGUF
    `-- Vision: separate on-demand worker only if pixels are actually needed
```

## Living behavior implemented now

The fully offline path includes:

- idle / observe / sit / lie / sleep / wake;
- energy and sleep-pressure changes with recovery while sleeping;
- hover, touch, petting and play;
- Focus Guard mode;
- Windows idle/return, foreground-app identity and local-time awareness;
- deterministic ambient exploration without an LLM;
- native `walk` / `run` pet-window movement with work-area clamping and edge reversal;
- domain-level facing synchronized with renderer and semantic hit regions;
- native transparent click-through outside Lenvu;
- **pick-up / drag / drop**: a short pointer gesture remains pet/touch, while movement beyond the drag threshold enters a domain-level `held` posture and native window drag;
- window-moved / scale-factor observation with debounced display/hit-region refresh;
- SQLite persistence of facing, energy, curiosity, bond and sleep pressure across restarts;
- relationship history, bounded meaningful-event journal and hourly interaction rhythm.

`held` is a Pet Brain posture, not a Windows-specific state. While held, autonomous locomotion is stopped and ambient behavior cannot overwrite the posture; dropping returns Lenvu to a stable ambient pose or resumes Focus Guard presentation.

Transient environment state is intentionally not restored from SQLite. A new session starts with fresh locomotion/posture/mode/cognition/user-idle state, then applies saved long-lived values.

The current Lenvu render is still a lightweight procedural placeholder. Production character art will replace it only after anatomy, scale, anchors, atlas bounds and masks are normalized from the reference sheets.

## UI/UX interaction layers

1. **Ambient** — Lenvu exists on the desktop without conventional UI.
2. **Direct pet interaction** — hover, touch, pet, play and pick-up are reflex-layer actions with no AI call.
3. **Context bubble** — compact status/reaction layer beside Lenvu.
4. **Companion window** — independent status/interaction surface; hiding it does not stop Pet Runtime.
5. **Deep management** — model/privacy/memory/performance/settings surfaces remain separate.

## Persistent Life and memory

`lenvu.sqlite3` lives under the application's local-data directory. Database work happens on a separate persistence worker; ordinary Pet Brain ticks never wait on SQLite.

Schema V2 separates:

```text
pet_state
activity_journal
relationship_events
memories
memory_fts
rhythm_hourly
```

The journal ignores noisy/high-frequency events and is capped at **30 days / 2,000 rows**. Long-term memory supports episodic, semantic, preference and relationship types. FTS5 is the first retrieval path using BM25 relevance plus importance and recency; no vector database is required for the first releases.

Storage and judgement are separate. A future Memory Evaluator decides whether a candidate should be stored, merged or discarded. If persistence cannot initialize, Lenvu continues session-only.

See `docs/PERSISTENCE.md` and `docs/MEMORY_SYSTEM.md`.

## Vision policy

A vision model is **not required for the first usable releases**. Lenvu first understands the computer through cheap structured signals such as active app identity, idle/return state, pointer interaction, monitor/work-area geometry, focus mode and time.

Future screen/image understanding sits behind a separate Screen Context Broker and optional worker. Pet Brain receives normalized observations, never raw screenshots as core state.

See `docs/VISION_SYSTEM.md`.

## Repository layout

```text
NorthPalace-my-pet/
+-- .github/workflows/      Windows CI
+-- assets/
|   +-- reference/          source/reference character material
|   `-- runtime/            optimized production runtime assets
+-- docs/                   living architecture/product specifications
+-- public/                 UI-served resources
+-- src/                    Svelte + PixiJS presentation
+-- src-tauri/              Rust runtime + persistence + Tauri/Windows integration
+-- README.md
+-- package.json
`-- vite.config.ts
```

## Validation rule

Feature-branch foundation changes must pass:

- Svelte + PixiJS production build;
- Rust/Tauri compilation and unit tests;
- Windows platform/domain tests;
- SQLite migration/state/journal/FTS5/rhythm tests.

CI proves clean-runner source compatibility. It does **not** replace executable/performance validation on the actual Ryzen 3 2200G + Vega 8 target machine.

## Next milestones

- normalize the canonical production Lenvu master and import high-resolution reference sheets;
- replace the procedural renderer with a production PixiJS sprite/atlas graph;
- define deliberate multi-monitor autonomous movement behavior;
- add tray/startup controls;
- build the user-facing Memory Browser/editor and Memory Evaluator;
- add low-noise context bubbles and privacy exclusions;
- benchmark the target machine before committing to MiniCPM5-1B runtime/context defaults.

## Living design documents

- `docs/ARCHITECTURE.md`
- `docs/FOUNDATION_REVIEW.md`
- `docs/UI_UX.md`
- `docs/PET_BRAIN.md`
- `docs/PERSISTENCE.md`
- `docs/MEMORY_SYSTEM.md`
- `docs/CHARACTER_BIBLE.md`
- `docs/ASSET_PIPELINE.md`
- `docs/DESKTOP_WINDOW.md`
- `docs/MODEL_RUNTIME.md`
- `docs/VISION_SYSTEM.md`
- `docs/ROADMAP.md`
