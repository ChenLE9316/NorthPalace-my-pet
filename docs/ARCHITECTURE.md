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
PetBrainV2 / Pet State / Behavior Intent / Personality / Memory / Domain Events
─────────────────────────────────────────────
Infrastructure
Windows adapters / SQLite / filesystem / future llama.cpp workers
```

The Domain layer must not depend on Svelte, PixiJS, WebView coordinates, Tauri windows, Win32, SQLite or llama.cpp.

## 3. Process and window topology

```text
NorthPalace-my-pet.exe
│
├─ Rust application composition
│  ├─ lib.rs       → builder + command registration + adapter wiring + shutdown
│  ├─ bootstrap.rs → local-data / privacy / persistence / Pet Runtime bootstrap
│  ├─ shell.rs     → tray-native shell behavior
│  └─ commands.rs  → Tauri application command boundary
├─ pet WebView       → transparent / always-on-top / PixiJS
├─ companion WebView → Svelte Home / Memory / Activity / Settings
└─ future workers
   ├─ northpalace-llm-worker.exe
   └─ optional vision worker
```

`pet` and `companion` share the same Rust application runtime but not UI lifecycle. Hiding the Companion does not stop Lenvu.

## 4. Pet Runtime ownership

Rust owns semantic simulation time. `RuntimeHandle` exposes one `DomainEvent` input channel, a background Pet Runtime owner, immutable snapshots, low-frequency domain-event observers and an optional snapshot observer used by the application boundary.

The Tauri bootstrap installs one snapshot observer that emits `pet-runtime-snapshot` to the WebViews. Each WebView performs one command-based cold-start read, then follows pushed snapshots instead of maintaining its own periodic snapshot timer. Direct interactions may still request one explicit refresh as a bounded fallback; there is no background frontend snapshot polling loop.

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

## 10. Persistence and memory domain

SQLite is bundled through `rusqlite`. Current structures cover pet state, bounded activity/relationship history, typed memories, FTS5 and hourly interaction rhythm. A DB-owning worker keeps SQLite I/O away from Pet Brain ticks.

Memory category and transport contracts have one domain source of truth in `src-tauri/src/domain/memory.rs`. `MemoryKind`, `MemoryDraft` and `MemorySearchHit` are shared by persistence and the memory-admin/application boundary rather than being redefined by individual adapters. This removes category drift before the future Memory Evaluator is added.

## 11. UI boundary

Svelte owns management UI, not life simulation or per-frame animation. Current Companion sections are Home, Memory, Activity and Settings/Privacy. Pet interaction and Companion opening are separate; the `☾` handle/tray opens the panel.

The Companion shell delegates section-specific state/UI to `src/lib/ui/companion/*`; new product surfaces should remain isolated instead of growing the shell back into a monolith.

Runtime state synchronization is event-driven: both WebViews listen to the Rust-owned `pet-runtime-snapshot` stream after an initial cold-start read. Pet-window move/scale changes are already observed natively by the pet WebView; after its debounced display-context refresh, it publishes the resulting `pet-display-context` event so Companion does not maintain a second periodic display-context timer. These streams carry small structured snapshots only and do not move animation frames, screenshots or other high-throughput payloads through the event bus.

## 12. Application composition boundary

`lib.rs` is intentionally limited to application composition: managed-service registration, Tauri command registration, Windows adapter wiring, Companion close behavior and bounded final persistence flush. Local-data/privacy/persistence/Pet Runtime construction lives in `bootstrap.rs`, while native tray behavior lives in `shell.rs` and command handlers live in `commands.rs`.

`bootstrap.rs` also owns the Tauri snapshot-event bridge by supplying the Pet Runtime with an application-level observer callback. The Pet Runtime remains unaware of WebView labels and Tauri event types.

This split keeps future worker supervision from being bolted into one oversized startup function and gives the next consolidation step a stable place for a lifecycle supervisor.

## 13. Worker lifecycle

Current background threads are mostly detached. Before MiniCPM/vision workers are added, introduce common cancellation, shutdown, join/restart and health reporting. Runtime `recovering` should only be introduced with that real mechanism.

## 14. Local AI policy

Text AI remains planned:

```text
northpalace-my-pet.exe → local IPC → northpalace-llm-worker.exe → llama.cpp → MiniCPM5-1B GGUF
```

No hover, petting, walking, sleeping, basic animation or focus reaction may require an LLM call. Future context composition must check structured-context freshness before using observations.

## 15. Vision policy

Continuous visual perception is out of scope. Future visual understanding must be separately loadable, opt-in and on-demand behind the same privacy policy.

## 16. Security, validation and reproducibility boundary

The production Tauri bundle uses a restrictive CSP limited to local resources and Tauri IPC. Vite development explicitly uses `devCsp: null`; the production CSP still needs verification in the first successful Windows bundle/run.

Windows CI guards `main` with tracked-data validation, frontend asset/build validation, Rust formatting, Clippy and tests. A separate manual Windows Bundle workflow builds the NSIS/executable artifact. That clean runner also resolves and uploads `package-lock.json` and `src-tauri/Cargo.lock` candidates so dependency locks can be reviewed before committing them.

Builds are not considered fully reproducible until those lockfiles are committed and normal CI switches to locked installs. CI source compatibility also does not replace a clean target-machine run and RAM/CPU/GPU measurements.

## 17. Documentation authority

For engineering behavior: running code/tests → `ARCHITECTURE.md` → `ROADMAP.md` → README. For visual identity, original source evidence and `LENVU_VISUAL_GROUND_TRUTH.md` remain authoritative.
