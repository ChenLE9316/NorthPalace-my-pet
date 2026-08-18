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
├─ pet WebView       → transparent / always-on-top / PixiJS
├─ companion WebView → Svelte Home / Memory / Activity / Settings
└─ future workers
   ├─ northpalace-llm-worker.exe
   └─ optional vision worker
```

`pet` and `companion` share the same Rust application runtime but not UI lifecycle. Hiding the Companion does not stop Lenvu.

## 4. Pet Runtime ownership

Rust owns semantic simulation time. `RuntimeHandle` exposes one `DomainEvent` input channel, a background Pet Runtime owner, immutable snapshots and low-frequency event observers.

Runtime health is intentionally truthful:

```text
ready     normal runtime loop
degraded  event input channel disconnected; final semantic snapshot retained
error     Pet Runtime thread panicked; last published snapshot retained
```

There is no speculative `recovering` state. Restart/recovery semantics belong with a future real worker supervisor.

## 5. Pet state, behavior and personality

Pet State keeps locomotion, facing, posture, attention, emotion, mode and cognition as parallel dimensions. Long-lived scalar state includes energy, curiosity, bond and sleep pressure. `held` is a domain posture, not a Windows-only flag.

Short actions use `BehaviorIntent` priority, remaining lifetime, interruption policy and semantic animation name. Ordinary ambient variation comes from deterministic weighted personality selection without an LLM. Explicit interactions, Focus Guard, held posture and sleep/rest override ambient choice.

## 6. Windows adapters

Current Windows modules include idle/return, local hour, foreground executable identity + privacy-approved DWM bounds, display/DPI/work area, optional bounded UI Automation metadata, selective cursor passthrough and native pet-window motion/drag.

Movement seeds direction from domain `facing`, respects work areas, reverses at edges and only allows autonomous monitor transitions for `Explore` across genuinely adjacent horizontal displays.

## 7. Privacy boundary

`PrivacyPolicyService` starts fail-closed. Foreground identity is limited to the executable stem; window titles are not collected by default. DWM bounds are not queried until the process identity has passed the app exclusion gate.

Rules live in `privacy-rules.json`, separate from memory/history. Updates use a flushed temporary file followed by Windows replace-existing/write-through replacement; in-memory policy mutations roll back when persistence fails.

Windows UI Automation is separately opt-in. It exposes only bounded structural metadata for the focused control and never reads Name, Value, HelpText, raw text or tree dumps. Disabling the capability drops its COM/UIA reader, and transient initialization failures retry after a bounded backoff.

## 8. Screen Context Broker and freshness

`ScreenContextBroker` is the structured-environment boundary used by the UI today and future AI composition later.

Current snapshot contains:

```text
activeAppId / activeAppState / activeWindowBounds
activeAppObservedAtMs
accessibilityState / accessibility
accessibilityObservedAtMs
userIdleMs / userIdleObservedAtMs
localHour / localHourObservedAtMs
sequence
```

The `*ObservedAtMs` fields record the last successful sensor observation, not the time a snapshot happened to be requested. Foreground-app and local-hour sensors refresh freshness on every successful poll even when the semantic value is unchanged. Heartbeats therefore do **not** advance `sequence`; sequence advances only when exposed semantic state changes.

This lets a future context composer reject stale context explicitly instead of inferring freshness from sequence numbers. Switching active applications invalidates previous accessibility data and its freshness timestamp. Stale accessibility results for a previous app are ignored.

The broker has no screenshot buffer, OCR data, window-title history or persistence.

## 9. Renderer and asset boundary

Pet Brain never knows sprite filenames:

```text
PetRuntimeSnapshot → resolveAnimation() → src/lib/pet/lenvu.manifest.json → PixiJS
```

`src/lib/pet/lenvu.manifest.json` is the single authoritative renderer manifest. The current renderer remains procedural. Lenvu's asymmetric identity forbids blind whole-root mirroring; only symmetric placeholder geometry may mirror while heterochromia and the left-side crescent remain semantically correct.

Original source evidence and `docs/LENVU_VISUAL_GROUND_TRUTH.md` outrank renderer placeholders, generated candidates and inferred coordinates.

## 10. Persistence

SQLite is bundled through `rusqlite`. Current structures cover pet state, bounded activity/relationship history, typed memories, FTS5 and hourly interaction rhythm. A DB-owning worker keeps SQLite I/O away from Pet Brain ticks.

Known cleanup debt: memory-domain types are still duplicated between persistence/admin layers and should be consolidated before Memory Evaluator growth.

## 11. UI boundary

Svelte owns management UI, not life simulation or per-frame animation. Current Companion sections are Home, Memory, Activity and Settings/Privacy. Pet interaction and Companion opening are separate; the `☾` handle/tray opens the panel.

Known cleanup debt: split the large `CompanionView.svelte` before adding more surfaces.

## 12. Worker lifecycle

Current background threads are mostly detached. Before MiniCPM/vision workers are added, introduce common cancellation, shutdown, join/restart and health reporting. Runtime `recovering` should only be introduced with that real mechanism.

## 13. Local AI policy

Text AI remains planned:

```text
northpalace-my-pet.exe → local IPC → northpalace-llm-worker.exe → llama.cpp → MiniCPM5-1B GGUF
```

No hover, petting, walking, sleeping, basic animation or focus reaction may require an LLM call. Future context composition must check structured-context freshness before using observations.

## 14. Vision policy

Continuous visual perception is out of scope. Future visual understanding must be separately loadable, opt-in and on-demand behind the same privacy policy.

## 15. Validation and release boundary

Windows CI guards `main` with frontend asset/build validation, Rust formatting, Clippy and tests. Dependency lockfiles are still required before builds are fully reproducible. CI does not replace a clean bundle build and target-machine RAM/CPU/GPU measurements.

## 16. Documentation authority

For engineering behavior: running code/tests → `ARCHITECTURE.md` → `ROADMAP.md` → README. For visual identity, original source evidence and `LENVU_VISUAL_GROUND_TRUTH.md` remain authoritative.
