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
|   +-- PixiJS 8 Pet Overlay
|   +-- Context Bubble
|   `-- Svelte 5 + TypeScript Companion/Settings UI
|
+-- Rust Runtime
|   +-- Runtime Clock
|   +-- Event Bus
|   +-- Pet Brain
|   |   +-- Parallel Pet State
|   |   `-- Behavior Intent
|   +-- Memory System
|   +-- Windows Adapter
|   `-- AI Orchestrator
|
+-- Optional AI Workers
|   +-- Text: llama.cpp -> MiniCPM5-1B GGUF
|   `-- Vision: deferred, separately-loadable, on-demand only
|
`-- Local Data
    +-- SQLite + FTS5
    +-- TOML/JSON configuration
    `-- bounded rolling logs
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
+-- docs/                  living architecture/product specifications
+-- public/                current UI-served assets
+-- src/                   Svelte presentation layer
+-- src-tauri/             Rust runtime and Tauri shell
+-- README.md
+-- package.json
`-- vite.config.ts
```

Reference art and production runtime animation assets will be separated as the visual pipeline is normalized.

## Current foundation status

The repository contains a V0 Pet Brain prototype plus the first V0.2 contracts for:

- domain events;
- parallel pet state;
- behavior intents with duration/priority;
- deferred/on-demand vision architecture.

The next implementation step is to move simulation time into a Rust-owned runtime and wire the V2 state/behavior contracts into it before adding memory or MiniCPM5-1B.

## Living design documents

- `docs/ARCHITECTURE.md`
- `docs/FOUNDATION_REVIEW.md`
- `docs/UI_UX.md`
- `docs/PET_BRAIN.md`
- `docs/MODEL_RUNTIME.md`
- `docs/VISION_SYSTEM.md`
- `docs/ROADMAP.md`
