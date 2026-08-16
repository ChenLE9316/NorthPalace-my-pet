# NorthPalace-my-pet

**Lenvu — local-first Neralune digital companion for Windows.**

NorthPalace-my-pet is a lightweight desktop-pet and local AI companion designed for the target machine **AMD Ryzen 3 2200G + 16 GB DRAM**. Lenvu remains a complete interactive pet even when the LLM is unloaded; local AI is an optional high-level cognition layer rather than the animation controller.

![NorthPalace-my-pet system overview](docs/assets/lenvu-system-overview.webp)

## Product principles

- **Pet first, chat second** — animation, expression and behavior communicate before text.
- **Always-lightweight** — the ambient desktop pet must remain cheap to keep running all day.
- **Event driven** — Windows, pointer, focus, time and user interactions become domain events.
- **LLM is not the reflex layer** — deterministic/probabilistic Pet Brain behavior stays instant and offline.
- **Local first** — pet state, relationship, memories, logs and model execution stay on-device by default.
- **Graceful degradation** — if the model is unloaded or crashes, Lenvu still walks, sleeps, reacts and remembers state.

## Initial architecture

```text
Windows 11
│
├─ Tauri 2 desktop shell
│  ├─ Rust runtime/core
│  └─ WebView2
│
├─ Presentation
│  ├─ Svelte 5 + TypeScript — companion/settings UI
│  └─ PixiJS 8 — pet overlay renderer
│
├─ Runtime Core (Rust)
│  ├─ Event Bus
│  ├─ Pet Brain
│  ├─ AI Orchestrator
│  ├─ Memory System
│  └─ Windows Adapter
│
├─ AI Worker
│  ├─ llama.cpp
│  └─ MiniCPM5-1B GGUF
│
└─ Local Data
   ├─ SQLite + FTS5
   ├─ TOML/JSON configuration
   └─ rolling JSONL logs
```

## Interaction layers

1. **Ambient** — idle, observe, walk, sit, sleep, explore.
2. **Direct pet interaction** — hover, touch, pet, drag, play.
3. **Context bubble** — brief reactions, reminders, focus status and compact AI replies.
4. **Companion panel** — conversation, mood, energy, bond, memory, focus and activity.
5. **Deep management** — model, privacy, memory, performance, display and debug settings.

## Repository layout

```text
NorthPalace-my-pet/
├─ docs/                  architecture and product specifications
├─ src/                   Svelte/PixiJS presentation layer
├─ src-tauri/             Rust runtime and Tauri shell
├─ README.md
├─ package.json
└─ vite.config.ts
```

## Phase 0 / Phase 1 goals

- Boot a transparent Windows pet-overlay shell.
- Establish the Rust event/domain model before adding AI.
- Render a placeholder Lenvu body and expose Pet Brain state to the UI.
- Implement idle → sit → rest/sleep behavior transitions.
- Add direct pointer interaction events.
- Build companion/settings panels without making chat the center of the product.
- Add SQLite persistence and memory types.
- Add MiniCPM5-1B as an isolated worker only after the pet works fully offline.

See `docs/ARCHITECTURE.md`, `docs/UI_UX.md`, `docs/PET_BRAIN.md`, and `docs/ROADMAP.md` for the living design documents.
