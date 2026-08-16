# NorthPalace-my-pet Architecture

## 1. Target

Primary target: **Windows 11 / Ryzen 3 2200G / 16 GB DRAM / Vega 8 iGPU**.

NorthPalace-my-pet is an always-running desktop life/companion system. Idle cost matters more than peak benchmark performance, so expensive subsystems — especially text/vision AI — must be optional and independently unloadable.

## 2. Current layer model

```text
Windows 11
│
├─ Tauri 2 Desktop Process
│  ├─ Rust Pet Runtime (single owner)
│  │  ├─ monotonic runtime clock
│  │  ├─ Domain Event channel
│  │  ├─ PetBrainV2
│  │  └─ immutable RuntimeSnapshot
│  │
│  ├─ Windows adapters
│  │  ├─ idle / return sensor
│  │  ├─ active-app identity sensor
│  │  ├─ display / DPI / work-area context
│  │  ├─ native cursor click-through controller
│  │  └─ pet-window motion controller
│  │
│  └─ Managed WebView windows
│     ├─ pet       → transparent, always-on-top, PixiJS
│     └─ companion → independent Svelte window
│
├─ Presentation
│  ├─ Lenvu manifest / semantic animation resolver
│  ├─ PixiJS pet renderer
│  ├─ Context Bubble
│  └─ Svelte Companion UI
│
├─ Local Data (planned next)
│  └─ SQLite + FTS5
│
└─ Optional AI Workers (later)
   ├─ text: llama.cpp → MiniCPM5-1B GGUF
   └─ vision: on-demand separate worker only if needed
```

The **Domain layer must not depend on Svelte, PixiJS, Win32, Tauri window coordinates, SQLite or llama.cpp**. Platform modules translate operating-system facts/actions around the domain rather than becoming the domain.

## 3. Runtime ownership

### Rust Pet Runtime

Rust owns Lenvu's simulation clock and mutable Pet Brain. Other threads/components send `DomainEvent`s through a channel and read immutable snapshots. WebView timers may poll snapshots for presentation, but they do not advance simulation time.

### Pet Brain

`PetBrainV2` owns semantic state such as locomotion, posture, attention, emotion, mode and cognition. Short reactions are represented with `BehaviorIntent` rather than being collapsed into one exclusive activity enum.

### Windows platform controllers

Platform controllers consume snapshots and/or produce domain events:

- idle sensor → `UserIdleChanged` / `UserReturned`;
- active-app sensor → `ActiveWindowChanged`;
- motion controller → turns `walk/run` locomotion into native pet-window displacement while clamping to the current work area;
- cursor hit-test controller → toggles native cursor passthrough using normalized semantic interaction regions.

Native window coordinates and Win32 cursor details stay outside Pet Brain.

## 4. Window boundaries

### `pet`

The pet window is a small transparent always-on-top overlay. It contains only:

- PixiJS Lenvu renderer;
- compact context bubble;
- small Companion toggle handle.

Transparent/non-interactive regions are native click-through; semantic Lenvu/handle regions remain interactive. The native cursor sensor can restore interaction when the cursor re-enters a hit zone, so click-through does not permanently make the pet unreachable.

### `companion`

The Companion is a separate managed window. It can be shown/hidden independently. A close request is converted to hide so Pet Runtime continues and the panel can reopen without rebuilding Lenvu's life state.

### Future windows

- `settings` — persistent model/privacy/performance configuration;
- `debug` — development-only event/state diagnostics.

## 5. Renderer / asset boundary

Pet Brain emits semantic state and Behavior Intents. It never knows sprite filenames.

```text
PetRuntimeSnapshot
      │
      ▼
resolveAnimation()
      │
      ▼
lenvu.manifest.json
      │
      ├─ animation profile / FPS budget
      ├─ semantic hit zones
      ├─ anchor / nominal size
      └─ future atlas asset path
      │
      ▼
PixiJS Renderer
```

The current renderer is deliberately procedural/vector-based. Production assets will be normalized through `docs/CHARACTER_BIBLE.md` and `docs/ASSET_PIPELINE.md` before replacing the placeholder.

## 6. Event-driven backbone

Current/defined events include:

- `Tick`
- `UserIdleChanged`
- `UserReturned`
- `CursorEnteredPet`
- `CursorLeftPet`
- `PetTouched`
- `PetPetted`
- `PetPlayRequested`
- `FocusModeStarted`
- `FocusModeEnded`
- `ActiveWindowChanged`
- `NotificationReceived`
- `TimeOfDayChanged`
- `LlmWorkerStateChanged`

Sensors publish facts. Pet Brain interprets facts. Platform/application controllers perform validated side effects. Presentation renders snapshots.

## 7. Movement boundary

Window movement is currently an infrastructure/application concern rather than an animation trick. `walk` and `run` locomotion drive horizontal native movement at DPI-scaled speed. The current controller deliberately:

- stays on the current monitor;
- respects the monitor work area/taskbar boundary;
- reverses at horizontal edges;
- does not yet autonomously cross monitors;
- does not move the native window vertically for `jump`.

Future work will expose facing direction in domain state and add a deliberate multi-monitor policy.

## 8. Memory

SQLite is the only database planned for V1.

Memory categories:

- episodic — events worth remembering;
- semantic — stable learned facts;
- preference — user preferences;
- relationship — bond/history between user and Lenvu;
- system — pet/runtime state.

V1 retrieval: FTS5 + metadata + recency + importance. A vector database is explicitly out of scope until evidence shows it is necessary.

## 9. AI policy

MiniCPM5-1B is the planned local text cognition layer. Long context is a capability ceiling, not an instruction to keep a large KV cache resident all day.

Prompt composition should be selective:

```text
Lenvu identity
+ current pet state
+ current environment
+ relevant memories
+ recent conversation
+ current request
```

No cursor movement, animation frame, hover event, walking, sleeping or basic pet reaction may require an LLM call. Vision remains a separately-loadable future worker behind a Screen Context Broker.

## 10. Resource policy

The first performance budgets are design targets, not measured guarantees:

- Rust runtime and sensors should remain inexpensive enough for all-day use;
- PixiJS uses animation-specific normal/low-power FPS caps;
- Companion UI can be hidden independently from the pet overlay;
- AI model memory is separately budgeted and unloadable;
- glow/particles can scale down under pressure;
- telemetry/debug tracing must be bounded and rotate.

## 11. Failure policy

- text/vision AI failure → AI unavailable, pet continues;
- database failure → temporary session state + diagnostics;
- renderer/asset failure → fallback procedural/placeholder presentation;
- one Windows sensor failure → disable/degrade that sensor, not Pet Brain;
- runtime command/channel failure → surface `degraded/recovering/error`, do not silently imitate healthy idle state.
