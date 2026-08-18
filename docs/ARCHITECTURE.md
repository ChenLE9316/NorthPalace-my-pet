# NorthPalace-my-pet Architecture

## 1. Scope and invariants

Primary runtime target: **Windows 11 / Ryzen 3 2200G / 16 GB DRAM / Vega 8 iGPU**.

NorthPalace-my-pet is an always-running desktop life/companion system. Idle cost and predictable degradation matter more than peak throughput. Expensive subsystems, especially text and vision AI, must remain optional and independently unloadable.

The core rule is **Pet first, AI second**. Removing every AI worker must not stop Lenvu's movement, sleep/wake rhythm, direct interaction, Focus Guard, persistence, inspectable memory or ordinary structured Windows awareness.

## 2. Layer model

```text
Presentation
PixiJS Pet / Context Bubble / Svelte Companion
─────────────────────────────────────────────
Application
Tauri commands / Screen Context Broker / services / controllers
─────────────────────────────────────────────
Domain
PetBrainV2 / Pet State / Behavior Intent / Personality / Domain Events
─────────────────────────────────────────────
Infrastructure
Windows adapters / SQLite / filesystem / future llama.cpp workers
```

The Domain layer must not depend on Svelte, PixiJS, WebView coordinates, Tauri windows, Win32, SQLite or llama.cpp.

## 3. Process and window topology

```text
NorthPalace-my-pet.exe
│
├─ Rust runtime + application services
│
├─ pet WebView
│  └─ transparent / always-on-top / PixiJS
│
├─ companion WebView
│  └─ Svelte Home / Memory / Activity / Settings
│
└─ future workers
   ├─ northpalace-llm-worker.exe
   └─ optional vision worker
```

`pet` and `companion` share the same Rust application runtime but not UI lifecycle. Hiding/closing the Companion does not stop Lenvu. A close request on the Companion is converted to hide.

The pet window contains only the character renderer, compact context bubble and a small Companion handle. Transparent space is selectively click-through through a native cursor controller. The pet remains reachable inside semantic hit regions.

## 4. Pet Runtime ownership

Rust owns semantic simulation time. `RuntimeHandle` exposes:

- a single `DomainEvent` input channel;
- a background Pet Runtime owner;
- immutable `PetRuntimeSnapshot` reads;
- low-frequency domain-event observers for persistence/history.

The frontend may poll or later subscribe for presentation, but it never advances Pet Brain time.

`PetRuntimeSnapshot` currently contains runtime health, sequence, parallel Pet State and an optional Behavior Intent.

## 5. Parallel Pet State

Pet State dimensions intentionally coexist:

```text
locomotion  stationary / walk / run / jump
facing      left / right
posture     stand / sit / lie / sleep / held
attention   idle / user / cursor / window / object
emotion     calm / curious / happy / shy / concerned / sleepy / focused
mode        ambient / focus_guard / do_not_disturb / play
cognition   idle / listening / thinking / speaking / remembering
```

Long-lived scalar state currently includes energy, curiosity, bond and sleep pressure. `user_idle_ms` and `ai_available` are transient environment/runtime values.

`held` is a domain posture, not a Windows-only flag. While held, ambient logic and autonomous locomotion cannot overwrite it.

## 6. Behavior Intent and personality

Short actions are represented separately from persistent state. `BehaviorIntent` carries kind, priority, remaining lifetime, interruption policy and semantic animation name.

Ordinary ambient variation is produced by a deterministic weighted `PersonalityProfile`, using current energy, curiosity, bond, sleep pressure, user-idle duration, time and decision index. This provides repeatable personality variation without an LLM.

The selector may produce actions such as `Explore`, `Observe`, `Sit` or `Stay`. Focus Guard, held posture, sleep/rest and explicit interactions take precedence over ambient personality choices.

## 7. Domain Event backbone

Current events include:

```text
Tick
UserIdleChanged
UserReturned
CursorEnteredPet
CursorLeftPet
PetTouched
PetPetted
PetPlayRequested
PetPickedUp
PetDropped
FocusModeStarted
FocusModeEnded
ActiveWindowChanged
NotificationReceived
TimeOfDayChanged
LlmWorkerStateChanged
PetFacingChanged
```

Sensors publish facts. Privacy gates decide whether sensitive context can cross a capability boundary. Pet Brain interprets allowed facts. Application/platform controllers perform validated side effects.

## 8. Windows adapters

Current Windows modules include:

- idle/return sensor;
- local-hour sensor;
- foreground executable identity + visible DWM bounds;
- display/monitor/DPI/work-area context;
- bounded optional UI Automation metadata;
- selective cursor passthrough;
- native pet-window motion/drag.

Adapters remain outside the Domain layer.

### 8.1 Native movement

`walk` and `run` locomotion are translated into horizontal window displacement at DPI-scaled logical speed. The controller:

- seeds motion direction from domain `facing` instead of forcing a startup direction;
- clamps Lenvu to Windows work areas;
- reverses at horizontal edges;
- preserves negative desktop coordinates;
- allows autonomous monitor transitions only when the active Behavior Intent is `Explore`;
- selects genuinely adjacent horizontal displays with sufficient vertical overlap;
- rejects large monitor gaps and vertical-stack teleporting;
- keeps non-Explore interactions on the current display;
- does not use native vertical movement for semantic `jump` yet.

Explicit user drag is separate from autonomous motion and maps to domain `PetPickedUp` / `PetDropped`.

### 8.2 Selective click-through

The frontend publishes normalized semantic hit regions. A low-cost native cursor loop decides whether the transparent pet window should ignore cursor events. Empty startup regions intentionally keep the window interactive until the WebView has published valid geometry.

Hit regions are facing-aware. They are not derived from transparent pixels yet; production sprite masks can replace/augment semantic regions later.

## 9. Privacy boundary

`PrivacyPolicyService` starts fail-closed. Until the local rule store is initialized safely, sensitive structured context is blocked.

Foreground-app path:

```text
GetForegroundWindow
      ↓
process executable stem
      ↓
PrivacyPolicyService
      ├─ blocked / fail-closed
      │      └─ no identity, no bounds, no Domain Event
      └─ allowed
             ↓
       visible DWM frame bounds
             ↓
       ScreenContextBroker
             ↓
       ActiveWindowChanged on identity transition
```

Window titles are not collected by default. DWM bounds are not queried until the app has passed the exclusion gate.

Rules live in `privacy-rules.json`, separate from SQLite/Memory/Activity. Updates are serialized to a temporary file, flushed, then installed through a replace-existing/write-through filesystem path on Windows. In-memory policy changes are rolled back when persistence fails.

## 10. Bounded accessibility context

Windows UI Automation is implemented as a separately opt-in structured-context capability behind the same privacy gate.

The collector may expose only bounded metadata for the currently focused control:

- control type ID;
- enabled;
- keyboard focusable;
- has keyboard focus;
- offscreen;
- password flag;
- bounding rectangle.

It does **not** read Name, Value, HelpText, raw control text, window titles or accessibility-tree dumps.

The reader is not initialized while the capability is disabled. Disabling it drops the COM/UI Automation reader. Transient initialization failure marks context unavailable and retries after a bounded backoff instead of remaining broken until restart.

## 11. Screen Context Broker

`ScreenContextBroker` is the application boundary between cheap structured sensors and future context/AI composition.

Current snapshot includes:

```text
activeAppId
activeAppState
activeWindowBounds
accessibilityState
accessibility
userIdleMs
localHour
sequence
```

Excluded apps never expose identity, bounds or accessibility metadata. Switching applications invalidates stale accessibility context, and stale accessibility results for a previous process are ignored.

The broker has no screenshot buffer, OCR data, window-title history or persistence.

Future AI composition should add observation freshness/timestamps before depending on these signals for higher-stakes interpretation.

## 12. Renderer and asset boundary

Pet Brain never knows sprite filenames.

```text
PetRuntimeSnapshot
      ↓
resolveAnimation()
      ↓
src/lib/pet/lenvu.manifest.json
      ↓
PixiJS Renderer
```

`src/lib/pet/lenvu.manifest.json` is the **single authoritative runtime renderer manifest**. A second hand-maintained public manifest is intentionally not kept.

The current renderer is procedural and temporary. It validates state-to-animation mapping, normal/low-power FPS policy, semantic hit regions, focus presentation and window integration.

Lenvu has asymmetric identity features. Whole-root blind horizontal mirroring is forbidden. Placeholder symmetric geometry may mirror, but identity-bearing layers such as heterochromia and the left-side crescent ornament must remain semantically correct. Production rendering should use directional assets or explicit semantic remapping.

Visual identity authority is defined separately by original source evidence and `docs/LENVU_VISUAL_GROUND_TRUTH.md`. Generated candidates and canonical coordinates cannot redefine source identity.

## 13. Persistence

SQLite is bundled through `rusqlite`; no system SQLite installation is required.

Current persistent structures:

```text
pet_state
activity_journal
relationship_events
memories
memory_fts
rhythm_hourly
```

A DB-owning persistence worker keeps SQLite I/O away from Pet Brain ticks. Current policy includes WAL, schema migrations, changed-only autosave, bounded final-save acknowledgement, session-only fallback, a 30-day / 2,000-row activity cap, typed memories and FTS5 retrieval.

Transient runtime state such as posture, locomotion, mode, cognition and current idle time is intentionally reset on restart; long-lived state is restored into fresh runtime defaults.

Known cleanup debt: memory-domain types are still duplicated between persistence/admin layers and should be consolidated before the Memory Evaluator grows the model.

## 14. UI boundary

Svelte owns management UI, not life simulation or per-frame animation.

Current Companion sections:

- Home;
- Memory;
- Activity;
- Settings/Privacy.

Management reads are lazy where practical. Technical runtime fields are still visible in the development UI and should later move behind a dedicated debug/developer surface.

Pet interaction and Companion opening are separate: the `☾` handle/tray opens the panel; double-clicking Lenvu is not used as an overlapping hidden gesture.

Known cleanup debt: `CompanionView.svelte` has grown into a large orchestration component and should be split into section components before more product surfaces are added.

## 15. Runtime health and worker lifecycle

Runtime health exposes `ready`, `degraded`, `recovering` and `error`, but the full recovery state machine is not yet implemented across all workers. This contract should not be treated as complete merely because all enum values exist.

Current background threads are mostly detached. Before adding MiniCPM/vision workers, introduce a common worker lifecycle/supervision model for cancellation, shutdown, join/restart and health reporting.

## 16. Local AI policy

Text AI remains planned, not part of the ordinary pet runtime.

Target topology:

```text
northpalace-my-pet.exe
        │
        │ local IPC
        ↓
northpalace-llm-worker.exe
        │
        ↓
llama.cpp
        │
        ↓
MiniCPM5-1B GGUF
```

The worker must be unloadable. Prompt composition should be selective and use identity + current state + privacy-approved fresh structured context + relevant memory + recent conversation + current request.

No hover, petting, walking, sleeping, basic animation or focus reaction may require an LLM call.

## 17. Vision policy

Continuous visual perception is out of scope. Structured Windows context is the default cheap path.

If pixels become necessary, future vision must be a separately-loadable, opt-in, on-demand worker behind the same per-app privacy policy, with visible capture indication and no screenshot history by default. Pet Brain should receive normalized observations, never raw screenshots as core state.

## 18. Resource policy

The first resource rules are design constraints until measured on the target machine:

- keep Rust runtime/sensors cheap enough for all-day use;
- keep expensive capability workers unloadable;
- cap presentation FPS by animation state;
- keep Companion hidden independently from Pet Runtime;
- avoid adding a polling loop for every new UI feature;
- keep logs/history bounded;
- benchmark before choosing permanent LLM context/KV-cache defaults.

## 19. Failure policy

- text/vision failure → AI unavailable, ordinary pet continues;
- SQLite/local-data failure → session-only life state + diagnostics;
- privacy initialization failure → sensitive context blocked fail-closed;
- DWM bounds failure → identity can remain allowed while geometry is unavailable;
- UI Automation initialization failure → unavailable + bounded retry;
- renderer/production-asset failure → procedural fallback during development;
- one Windows sensor failure → degrade that capability, not Pet Brain;
- runtime channel failure → explicit unhealthy runtime state rather than fake healthy idle.

## 20. Validation and release boundary

Windows CI should guard `main` with frontend asset/build validation, Rust formatting, Clippy and tests. Dependency lockfiles are still required before the build is fully reproducible.

CI source compatibility is not the same as product validation. Before release, run a clean executable/bundle build and measure idle RAM/CPU/GPU on the actual Ryzen 3 2200G + Vega 8 target.

Other release blockers include code/art licensing, `SECURITY.md`, restrictive CSP, production icon/art, dependency locks and a final tracked-data audit.

## 21. Documentation maintenance order

For engineering behavior:

```text
running code + tests
        ↓
ARCHITECTURE.md
        ↓
ROADMAP.md
        ↓
README.md
```

For Lenvu visual identity, original source evidence and `LENVU_VISUAL_GROUND_TRUTH.md` override renderer placeholders, generated candidates and inferred coordinates.
