# NorthPalace-my-pet

**Lenvu — local-first Neralune digital companion for Windows.**

NorthPalace-my-pet is a lightweight desktop-pet and local AI companion designed around **AMD Ryzen 3 2200G + 16 GB DRAM + Vega 8**. Lenvu remains a complete interactive desktop creature when every AI worker is unloaded; local AI is an optional cognition layer, not the animation controller and not the source of life.

![NorthPalace-my-pet system overview](docs/assets/lenvu-system-overview.webp)

> Current development status: **V0.2 / Phase 1 living-desktop-pet foundation**. The feature branch has passed a complete Windows CI run covering the Svelte/PixiJS frontend and Rust/Tauri tests.

## Product principles

- **Pet first, chat second** — animation, expression and behavior communicate before text.
- **Always-lightweight** — the ambient desktop pet must remain cheap enough to run all day.
- **Event driven** — Windows, pointer, focus, time and user interactions become domain events.
- **Rust owns life-time** — the Pet Runtime clock/state does not depend on JavaScript timers or an open panel.
- **Parallel state** — movement, facing, posture, attention, emotion, mode and cognition coexist.
- **LLM is not the reflex layer** — ordinary pet behavior is instant and fully offline.
- **Vision is optional** — structured Windows context comes first; screen vision is future, opt-in and on-demand.
- **Local first** — pet state, relationship, memories, logs and model execution stay on-device by default.
- **Graceful degradation** — AI/sensor failures degrade capabilities without killing Lenvu's ordinary life loop.

## Current architecture

```text
Windows 11 / Ryzen 3 2200G / 16 GB DRAM / Vega 8
|
+-- Tauri 2 desktop process
|   |
|   +-- Rust Pet Runtime
|   |   +-- monotonic 250 ms runtime clock
|   |   +-- Domain Event channel
|   |   +-- PetBrainV2
|   |   |   +-- parallel Pet State
|   |   |   `-- Behavior Intent
|   |   `-- immutable Runtime Snapshot
|   |
|   +-- Windows adapters/controllers
|   |   +-- idle / user-return sensor
|   |   +-- foreground-app identity sensor
|   |   +-- monitor / DPI / work-area context
|   |   +-- selective native cursor passthrough
|   |   `-- native pet-window motion controller
|   |
|   `-- WebView windows
|       +-- pet       -> transparent / always-on-top / PixiJS
|       `-- companion -> independent Svelte management window
|
+-- Lenvu presentation contract
|   +-- versioned animation manifest
|   +-- semantic animation resolver
|   +-- facing-aware semantic hit zones
|   `-- PixiJS low-power renderer
|
+-- Local Data [planned]
|   `-- SQLite + FTS5
|
`-- Optional AI Workers [planned]
    +-- Text: llama.cpp -> MiniCPM5-1B GGUF
    `-- Vision: separate on-demand worker only if pixels are actually needed
```

## Living behavior implemented now

Lenvu already has a fully offline path for:

- idle / observe / sit / lie / sleep / wake;
- energy and sleep-pressure changes, including recovery while sleeping;
- hover, touch, petting and play;
- Focus Guard mode;
- Windows user idle/return awareness;
- foreground-application identity awareness without collecting titles by default;
- short-lived Behavior Intents that survive multiple runtime ticks;
- deterministic ambient exploration without calling an LLM;
- `walk` / `run` locomotion that moves the native pet window;
- current-monitor work-area clamping so Lenvu stays above the taskbar;
- automatic left/right boundary reversal;
- domain-level facing state synchronized with the renderer;
- facing-aware head/body/tail interaction regions;
- native transparent click-through outside Lenvu while retaining cursor re-entry into interactive regions.

The current Lenvu render is deliberately a lightweight procedural placeholder. Production character art will replace it only after anatomy, scale, anchors, atlas bounds and masks are normalized from the reference sheets.

## UI/UX interaction layers

1. **Ambient** — Lenvu exists on the desktop without requiring conventional UI.
2. **Direct pet interaction** — hover, touch, pet and play are reflex-layer actions with no AI call.
3. **Context bubble** — compact status/reaction layer beside Lenvu.
4. **Companion window** — independent status and interaction surface; hiding it does not stop Pet Runtime.
5. **Deep management** — model/privacy/memory/performance/settings surfaces are planned separately.

## Vision policy

A vision model is **not required for the first usable releases**. Lenvu first understands the computer through cheap structured signals such as active app identity, idle/return state, cursor/pet interaction, monitor/work-area geometry, focus mode and time.

Future screen/image understanding must sit behind a separate Screen Context Broker and optional worker. Pet Brain receives normalized observations, never raw screenshots as its core state.

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
+-- src-tauri/              Rust Pet Runtime + Tauri/Windows integration
+-- README.md
+-- package.json
`-- vite.config.ts
```

## Validation status

The first complete Windows CI has passed:

- frontend dependency installation;
- Svelte + PixiJS production build;
- stable Rust setup;
- Rust/Tauri compilation;
- Pet Runtime/domain/platform unit tests.

This proves the feature-branch source compiles in a clean GitHub Windows runner. It does **not** replace the required performance/build validation on the actual Ryzen 3 2200G + Vega 8 target machine.

## Next milestones

- normalize the canonical production Lenvu master and import the high-resolution reference sheets;
- replace the procedural character with a production PixiJS sprite/atlas graph;
- add drag/pick-up and monitor/DPI change observation;
- define a deliberate multi-monitor movement policy;
- add SQLite persistence before MiniCPM5-1B;
- benchmark the real target machine before committing to AI context/runtime defaults.

## Living design documents

- `docs/ARCHITECTURE.md`
- `docs/FOUNDATION_REVIEW.md`
- `docs/UI_UX.md`
- `docs/PET_BRAIN.md`
- `docs/CHARACTER_BIBLE.md`
- `docs/ASSET_PIPELINE.md`
- `docs/DESKTOP_WINDOW.md`
- `docs/MODEL_RUNTIME.md`
- `docs/VISION_SYSTEM.md`
- `docs/ROADMAP.md`
