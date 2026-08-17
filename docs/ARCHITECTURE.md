# NorthPalace-my-pet Architecture

## 1. Target

Primary target: **Windows 11 / Ryzen 3 2200G / 16 GB DRAM / Vega 8 iGPU**.

NorthPalace-my-pet is an always-running desktop life/companion system. Idle cost matters more than peak benchmark performance, so expensive subsystems — especially text/vision AI — remain optional and independently unloadable.

The product rule is **Pet first, AI second**: unloading every AI worker must not stop Lenvu's movement, sleep/wake rhythm, direct interaction, focus behavior, persistence, memory management or ordinary Windows awareness.

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
│  │  ├─ active-app identity + visible-bounds sensor
│  │  ├─ local-hour sensor
│  │  ├─ display / DPI / work-area context
│  │  ├─ native cursor click-through controller
│  │  └─ pet-window motion controller
│  │
│  ├─ PrivacyPolicyService
│  │  ├─ fail-closed startup state
│  │  ├─ local privacy-rules.json
│  │  └─ per-app context gate
│  │
│  ├─ ScreenContextBroker
│  │  ├─ active app identity or privacy-blocked state
│  │  ├─ privacy-approved visible window bounds
│  │  ├─ user idle milliseconds
│  │  ├─ local hour
│  │  └─ on-demand immutable snapshot
│  │
│  ├─ Local persistence
│  │  ├─ SQLite + WAL
│  │  ├─ activity / relationship history
│  │  ├─ typed long-term memory
│  │  └─ FTS5 retrieval
│  │
│  ├─ Native shell UX
│  │  ├─ System Tray
│  │  └─ opt-in Windows launch-at-login
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
│     ├─ Home
│     ├─ Memory
│     ├─ Activity
│     └─ Settings
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

Ambient behavior uses a deterministic weighted personality selector driven by current curiosity, energy, sleep pressure, bond, user idle time and local hour. This creates variation without making ordinary movement depend on an LLM.

### Windows platform controllers

Platform controllers consume snapshots and/or produce domain events:

- idle sensor → `UserIdleChanged` / `UserReturned` and current idle value into Screen Context Broker;
- active-app sensor → process identity → privacy gate → allowed-only visible DWM frame bounds → Screen Context Broker; `ActiveWindowChanged` is emitted only for allowed identity transitions;
- local-hour sensor → `TimeOfDayChanged` and broker hour;
- motion controller → turns `walk/run` locomotion into native pet-window displacement while clamping to the current work area;
- cursor hit-test controller → toggles native cursor passthrough using normalized semantic interaction regions.

Moving/resizing the same allowed foreground window updates broker geometry without repeatedly emitting `ActiveWindowChanged`, so Pet Brain is not spammed by geometry-only changes.

Native window coordinates and Win32 cursor details stay outside Pet Brain.

## 4. Privacy boundary

Foreground app identity is intentionally limited to the process executable stem. The active-window adapter does **not** collect window titles by default.

The privacy path is:

```text
GetForegroundWindow
      ↓
process app id only
      ↓
PrivacyPolicyService
      ├─ excluded / fail-closed
      │      ↓
      │   identity blocked
      │   window bounds not queried
      │   broker id/bounds cleared
      │   no ActiveWindowChanged
      │
      └─ allowed
             ↓
      DWM visible frame bounds
             ↓
      ScreenContextBroker
             ↓
      ActiveWindowChanged (identity transition only)
```

`PrivacyPolicyService` starts fail-closed. Until its local rule file has been safely initialized, all active-app identity is treated as blocked. A corrupt rule file therefore reduces awareness instead of silently disabling privacy.

The deny list is stored separately in `privacy-rules.json`; it is not copied into SQLite, Memory or Activity History. The Settings UI only shows rules the user explicitly created and does not maintain a recent-app inventory.

Window geometry is treated as context data under the same gate: the sensor does not ask DWM for visible frame bounds until the foreground process identity has passed the exclusion policy.

This service is also the intended common gate for future structured accessibility context and optional capture.

## 5. Screen Context Broker

`ScreenContextBroker` is the application-level boundary between low-cost environment sensors and future AI/context composition.

V1 contains only structured values available without visual capture:

```text
ScreenContextSnapshot
├─ activeAppId: string | null
├─ activeAppState
│  ├─ unknown
│  ├─ available
│  └─ privacy_blocked
├─ activeWindowBounds: WindowBounds | null
│  ├─ x
│  ├─ y
│  ├─ width
│  └─ height
├─ userIdleMs
├─ localHour
└─ sequence
```

An excluded app never appears as an app id or geometry in the snapshot. When privacy rules change while that application remains in the foreground, the sensor re-evaluates the in-memory gate on its next normal one-second tick and clears stale identity and bounds.

For allowed applications, the active-window adapter uses the visible DWM frame rectangle and preserves negative desktop coordinates so monitors positioned left/up of the primary display remain representable.

The broker has no screenshot buffer, no OCR data, no window-title history and no persistence. `screen_context_get` is an on-demand snapshot command; the normal Companion UI does not continuously poll it.

Future context can extend the broker with bounded accessibility metadata, but that capability must be independently opt-in and pass the same app privacy boundary first.

## 6. Window boundaries

### `pet`

The pet window is a small transparent always-on-top overlay. It contains only:

- PixiJS Lenvu renderer;
- compact Context Bubble;
- small Companion toggle handle.

Transparent/non-interactive regions are native click-through; semantic Lenvu/handle regions remain interactive. The native cursor sensor can restore interaction when the cursor re-enters a hit zone, so click-through does not permanently make the pet unreachable.

### `companion`

The Companion is a separate managed window. It can be shown/hidden independently. A close request is converted to hide so Pet Runtime continues and the panel can reopen without rebuilding Lenvu's life state.

Current lazy sections are `Home`, `Memory`, `Activity` and `Settings`. Memory/Activity/Settings management I/O is loaded when the relevant surface is opened rather than becoming an idle background workload.

### Native shell

The tray provides open Companion, show/hide Lenvu and explicit quit actions. Windows launch-at-login is opt-in and controlled through the official Tauri autostart integration; the operating-system registration is treated as source of truth.

### Future windows

- dedicated deeper settings surface if the current Settings tab becomes too dense;
- `debug` — development-only event/state diagnostics.

## 7. Renderer / asset boundary

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

## 8. Event-driven backbone

Current/defined events include:

- `Tick`
- `UserIdleChanged`
- `UserReturned`
- `CursorEnteredPet`
- `CursorLeftPet`
- `PetTouched`
- `PetPetted`
- `PetPlayRequested`
- `PetPickedUp`
- `PetDropped`
- `FocusModeStarted`
- `FocusModeEnded`
- `ActiveWindowChanged`
- `NotificationReceived`
- `TimeOfDayChanged`
- `LlmWorkerStateChanged`
- `PetFacingChanged`

Sensors publish facts. Privacy gates decide whether sensitive context may cross a capability boundary. Pet Brain interprets allowed facts. Platform/application controllers perform validated side effects. Presentation renders snapshots.

## 9. Movement boundary

Window movement is an infrastructure/application concern rather than an animation trick. `walk` and `run` locomotion drive horizontal native movement at DPI-scaled speed. The current controller deliberately:

- stays on the current monitor;
- respects the monitor work area/taskbar boundary;
- reverses at horizontal edges;
- does not yet autonomously cross monitors;
- does not move the native window vertically for `jump`.

A deliberate multi-monitor movement policy remains future work.

## 10. Persistence and memory

SQLite is the only database required for V1. It is bundled through `rusqlite`, so the target machine does not require a system SQLite installation.

Current persistent structures include pet state, bounded activity/relationship history, typed long-term memories, FTS5 search and a small hourly interaction rhythm profile.

Long-term memory categories are:

- episodic — events worth remembering;
- semantic — stable learned facts;
- preference — user preferences;
- relationship — bond/history between user and Lenvu.

V1 retrieval uses FTS5 + metadata + recency + importance. A vector database remains out of scope until evidence shows it is necessary.

Privacy rules deliberately remain outside this memory database.

## 11. AI policy

MiniCPM5-1B is the planned local text cognition layer. Long context is a capability ceiling, not an instruction to keep a large KV cache resident all day.

Future prompt composition should be selective:

```text
Lenvu identity
+ current pet state
+ privacy-approved ScreenContextSnapshot
+ relevant memories
+ recent conversation
+ current request
```

No cursor movement, animation frame, hover event, walking, sleeping or basic pet reaction may require an LLM call. Vision remains a separately-loadable future worker behind the Screen Context Broker and the same privacy gate.

## 12. Resource policy

The first performance budgets are design targets, not measured guarantees:

- Rust runtime and sensors should remain inexpensive enough for all-day use;
- no new polling loop was added for Screen Context Broker — it consumes existing sensor observations;
- active-window bounds reuse the same one-second foreground observation loop;
- PixiJS uses animation-specific normal/low-power FPS caps;
- Companion UI can be hidden independently from the pet overlay;
- Memory/Activity/Settings management reads are lazy;
- AI model memory is separately budgeted and unloadable;
- glow/particles can scale down under pressure;
- telemetry/debug tracing must be bounded and rotate.

## 13. Failure policy

- text/vision AI failure → AI unavailable, pet continues;
- database failure → temporary session state + diagnostics;
- privacy-rule initialization failure → active-app identity and bounds are blocked fail-closed;
- DWM bounds lookup failure → identity may remain available, bounds become unavailable; no screenshot fallback is attempted;
- renderer/asset failure → fallback procedural/placeholder presentation;
- one Windows sensor failure → disable/degrade that sensor, not Pet Brain;
- runtime command/channel failure → surface `degraded/recovering/error`, do not silently imitate healthy idle state.
