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

Rust owns semantic simulation time. `RuntimeHandle` exposes a single `DomainEvent` input channel, a background Pet Runtime owner, immutable `PetRuntimeSnapshot` reads and low-frequency event observers for persistence/history.

The frontend may poll or later subscribe for presentation, but it never advances Pet Brain time.

Runtime health is intentionally small and truthful:

```text
ready     normal runtime loop

degraded  event input channel disconnected; final semantic snapshot retained

error     Pet Runtime thread panicked; last published snapshot retained and marked unhealthy
```

There is no speculative `recovering` state. Recovery/restart semantics should be added together with a real worker supervisor rather than represented by an enum value that cannot occur.

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

Focus Guard, held posture, sleep/rest and explicit interactions take precedence over ambient personality choices.

## 7. Domain Event backbone

Current events include `Tick`, idle/return, pointer/pet interactions, pick-up/drop, Focus Guard transitions, active-window changes, notifications, local time, LLM availability and facing changes.

Sensors publish facts. Privacy gates decide whether sensitive context can cross a capability boundary. Pet Brain interprets allowed facts. Application/platform controllers perform validated side effects.

## 8. Windows adapters

Current Windows modules include idle/return, local hour, foreground executable identity + visible DWM bounds, display/DPI/work-area context, bounded optional UI Automation metadata, selective cursor passthrough and native pet-window motion/drag.

Adapters remain outside the Domain layer.

### 8.1 Native movement

`walk` and `run` locomotion are translated into horizontal window displacement at DPI-scaled logical speed. The controller seeds motion direction from domain `facing`, clamps to Windows work areas, reverses at edges, preserves negative desktop coordinates, and allows autonomous monitor transitions only for `Explore` across genuinely adjacent horizontal displays.

Explicit user drag is separate from autonomous motion and maps to domain `PetPickedUp` / `PetDropped`.

### 8.2 Selective click-through

The frontend publishes normalized semantic hit regions. A low-cost native cursor loop decides whether the transparent pet window should ignore cursor events. Empty startup regions intentionally keep the window interactive until valid geometry exists.

Hit regions are facing-aware. Production sprite masks can replace/augment semantic regions later.

## 9. Privacy boundary

`PrivacyPolicyService` starts fail-closed. Until the local rule store is initialized safely, sensitive structured context is blocked.

Foreground-app path:

```text
GetForegroundWindow
      ↓
process executable stem
      ↓
PrivacyPolicyService
      ├─ blocked / fail-closed → no identity, bounds or Domain Event
      └─ allowed             → DWM bounds → ScreenContextBroker
```

Window titles are not collected by default. DWM bounds are not queried until the app has passed the exclusion gate.

Rules live in `privacy-rules.json`, separate from SQLite/Memory/Activity. Updates are serialized to a temporary file, flushed, then installed through a replace-existing/write-through path on Windows. In-memory policy changes are rolled back when persistence fails.

## 10. Bounded accessibility context

Windows UI Automation is a separately opt-in structured-context capability behind the same privacy gate.

The collector may expose only control type ID, enabled/focus/offscreen/password flags and bounding rectangle for the currently focused control. It does not read Name, Value, HelpText, raw text, window titles or tree dumps.

Disabling the capability drops the COM/UI Automation reader. Transient initialization failure marks context unavailable and retries after a bounded backoff.

## 11. Screen Context Broker

Current snapshot includes active app identity/state, allowed window bounds, accessibility state/metadata, user idle, local hour and sequence. Excluded apps never expose identity, bounds or accessibility metadata. Switching apps invalidates stale accessibility context.

The broker has no screenshot buffer, OCR data, window-title history or persistence. Future AI composition should add observation freshness/timestamps before depending on these signals.

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

`src/lib/pet/lenvu.manifest.json` is the single authoritative runtime renderer manifest. A second hand-maintained public manifest is intentionally not kept.

The current renderer is procedural and temporary. It validates state-to-animation mapping, FPS policy, semantic hit regions, focus presentation and window integration.

Lenvu has asymmetric identity features. Whole-root blind mirroring is forbidden. Placeholder symmetric geometry may mirror, while identity-bearing layers such as heterochromia and the left-side crescent remain semantically correct. Production rendering should use directional assets or explicit semantic remapping.

Visual identity authority is defined separately by original source evidence and `docs/LENVU_VISUAL_GROUND_TRUTH.md`.

## 13. Persistence

SQLite is bundled through `rusqlite`. Current persistent structures include pet state, bounded activity/relationship history, typed long-term memories, FTS5 and hourly interaction rhythm.

A DB-owning worker keeps SQLite I/O away from Pet Brain ticks. Transient runtime state is intentionally reset on restart; long-lived values are restored into fresh defaults.

Known cleanup debt: memory-domain types are still duplicated between persistence/admin layers and should be consolidated before Memory Evaluator growth.

## 14. UI boundary

Svelte owns management UI, not life simulation or per-frame animation. Current Companion sections are Home, Memory, Activity and Settings/Privacy.

Pet interaction and Companion opening are separate: the `☾` handle/tray opens the panel; double-clicking Lenvu is not an overlapping hidden gesture.

Known cleanup debt: `CompanionView.svelte` should be split into section components before more product surfaces are added.

## 15. Worker lifecycle

Current background threads are mostly detached. Before MiniCPM/vision workers are added, introduce a common worker lifecycle/supervision model for cancellation, shutdown, join/restart and health reporting. Runtime `recovering` should only be introduced together with that real restart mechanism.

## 16. Local AI policy

Text AI remains planned, not part of the ordinary pet runtime.

```text
northpalace-my-pet.exe
        │ local IPC
        ↓
northpalace-llm-worker.exe
        ↓
llama.cpp
        ↓
MiniCPM5-1B GGUF
```

No hover, petting, walking, sleeping, basic animation or focus reaction may require an LLM call.

## 17. Vision policy

Continuous visual perception is out of scope. Structured Windows context is the default cheap path. Future visual understanding must be separately loadable, opt-in and on-demand behind the same privacy policy.

## 18. Resource policy

Keep runtime/sensors cheap enough for all-day use, expensive workers unloadable, presentation FPS bounded, Companion hidden independently from Pet Runtime, logs/history bounded, and LLM context/KV-cache defaults benchmark-driven rather than permanently maximal.

## 19. Failure policy

- AI failure → AI unavailable, ordinary pet continues.
- SQLite/local-data failure → session-only life state + diagnostics.
- privacy initialization failure → sensitive context blocked.
- DWM bounds failure → identity may remain available while geometry is absent.
- UI Automation failure → unavailable + retry.
- runtime event-channel disconnect → degraded final snapshot.
- Pet Runtime panic → last snapshot marked error.

## 20. Validation and release boundary

Windows CI guards `main` with frontend asset/build validation, Rust formatting, Clippy and tests. Dependency lockfiles are still required before the build is fully reproducible.

CI source compatibility is not the same as product validation. Before release, run a clean executable/bundle build and measure idle RAM/CPU/GPU on the actual Ryzen 3 2200G + Vega 8 target.

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

For visual identity, original source evidence and `LENVU_VISUAL_GROUND_TRUTH.md` override renderer placeholders, generated candidates and inferred coordinates.
