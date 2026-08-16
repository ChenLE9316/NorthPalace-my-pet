# NorthPalace-my-pet

**Lenvu — local-first Neralune digital companion for Windows.**

NorthPalace-my-pet is a lightweight desktop-pet and local AI companion designed for the target machine **AMD Ryzen 3 2200G + 16 GB DRAM**. Lenvu remains a complete interactive pet even when AI workers are unloaded; local AI is an optional cognition layer rather than the animation controller or the source of life.

![NorthPalace-my-pet system overview](docs/assets/lenvu-system-overview.webp)

## Product principles

- **Pet first, chat second** — animation, expression and behavior communicate before text.
- **Always-lightweight** — the ambient desktop pet must remain cheap to keep running all day.
- **Event driven** — Windows, pointer, focus, time and user interactions become domain events.
- **Rust owns life-time** — the Pet Runtime clock/state must not depend on a JavaScript timer or an open UI panel.
- **Parallel state** — movement, posture, attention, emotion, mode and cognition can coexist.
- **LLM is not the reflex layer** — deterministic/probabilistic Pet Brain behavior stays instant and offline.
- **Vision is optional** — Windows/system signals are preferred; screen vision is future, opt-in and on-demand.
- **Local first** — pet state, relationship, memories, logs and model execution stay on-device by default.
- **Graceful degradation** — if text AI or future vision workers are unloaded/crashed, Lenvu still walks, sleeps, reacts and remembers state.

## Target architecture

```text
Windows 11 / Ryzen 3 2200G / 16 GB DRAM / Vega 8
|
+-- Tauri 2 Desktop Shell
|   +-- Rust Runtime/Core
|   `-- WebView2
|
+-- Presentation
|   +-- PixiJS 8 Pet Renderer
|   |   +-- animation resolver
|   |   `-- runtime asset manifest
|   +-- Context Bubble
|   `-- Svelte 5 + TypeScript Companion/Settings UI
|
+-- Rust Runtime
|   +-- Monotonic Runtime Clock
|   +-- Domain Event channel
|   +-- PetBrainV2
|   |   +-- Parallel Pet State
|   |   `-- Behavior Intent
|   +-- Immutable Runtime Snapshot
|   +-- Windows Adapter
|   |   +-- idle / return
|   |   +-- foreground app identity
|   |   `-- monitor / DPI / work area context
|   +-- Memory System [planned]
|   `-- AI Orchestrator [planned]
|
+-- Optional AI Workers
|   +-- Text: llama.cpp -> MiniCPM5-1B GGUF [planned]
|   `-- Vision: deferred, separately-loadable, on-demand only
|
`-- Local Data
    +-- SQLite + FTS5 [planned]
    +-- TOML/JSON configuration [planned]
    `-- bounded rolling logs [planned]
```

## Interaction layers

1. **Ambient** — idle, observe, walk, sit, rest, sleep, explore.
2. **Direct pet interaction** — hover, touch, pet, drag, play.
3. **Context bubble** — brief reactions, reminders, focus status and compact AI replies.
4. **Companion panel** — conversation, mood, energy, bond, memory, focus and activity.
5. **Deep management** — model, privacy, memory, performance, display and debug settings.

## Vision policy

A vision model is **not required for the first usable releases**. Lenvu should first understand the desktop through cheap structured context: active app/window, idle/return state, cursor, monitor/window geometry, focus mode, time and explicit interaction. Optional image/screen understanding can be added later as a separately loadable worker for tasks that truly require pixels.

See `docs/VISION_SYSTEM.md`.

## Repository layout

```text
NorthPalace-my-pet/
+-- .github/workflows/      Windows CI definition
+-- assets/                 source/reference and runtime-asset boundaries
+-- docs/                   living architecture/product specifications
+-- public/                 UI-served runtime assets/manifests
+-- src/                    Svelte + PixiJS presentation
+-- src-tauri/              Rust Pet Runtime + Tauri shell
+-- README.md
+-- package.json
`-- vite.config.ts
```

## Current foundation status — V0.2

Implemented now:

- Rust-owned 250 ms Pet Runtime clock;
- Domain Event channel;
- V2 parallel state model for locomotion/posture/attention/emotion/mode/cognition;
- Behavior Intents with priority, TTL and interruption policy;
- runtime health snapshots (`ready`, `degraded`, `recovering`, `error`);
- Svelte snapshot-only presentation path — no JavaScript simulation ticking;
- low-cost Windows user idle/return sensor through Win32 input timing;
- foreground-app identity awareness without collecting window titles by default;
- current monitor, DPI scale, work-area and pet-window geometry contract;
- hover enter/leave, touch, pet, play and Focus Guard interactions;
- simple multi-signal rest/sleep policy with energy recovery during sleep;
- PixiJS renderer boundary with a lightweight vector placeholder Lenvu;
- renderer-facing animation resolver plus runtime animation manifest;
- reference-art/runtime-asset separation and asset-pipeline specification;
- Windows CI workflow definition for frontend build + Rust tests.

Still intentionally deferred:

- canonical production Lenvu sprite/atlas assets;
- normalized animated hit zones and native selective click-through;
- autonomous desktop movement and multi-monitor movement policy;
- SQLite memory/persistence;
- MiniCPM5-1B worker;
- any dedicated vision model.

The next engineering milestone is the **desktop-space layer**: normalized hit zones, native selective hit testing, work-area-safe movement, walk/run synchronization and monitor/DPI change observation.

## Living design documents

- `docs/ARCHITECTURE.md`
- `docs/FOUNDATION_REVIEW.md`
- `docs/UI_UX.md`
- `docs/PET_BRAIN.md`
- `docs/ASSET_PIPELINE.md`
- `docs/DESKTOP_WINDOW.md`
- `docs/MODEL_RUNTIME.md`
- `docs/VISION_SYSTEM.md`
- `docs/ROADMAP.md`
