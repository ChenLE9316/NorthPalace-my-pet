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
│  ├─ lib.rs       → builder + command registration + adapter wiring + phased shutdown
│  ├─ bootstrap.rs → local-data / privacy / persistence / Pet Runtime bootstrap
│  ├─ worker.rs    → phase cancellation / named registry / health / bounded join
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

The production Pet Runtime is a named managed worker (`pet-runtime`). Its 250 ms semantic clock remains owned by the runtime itself; supervision only supplies cancellation, health and shutdown ownership and does not move semantic timing into a generic scheduler.

Runtime health remains a domain/runtime contract:

```text
ready     normal runtime loop
degraded  event input channel disconnected; final semantic snapshot retained
error     Pet Runtime loop failed/panicked; last published snapshot retained
```

Worker health is a separate infrastructure contract. There is still no speculative runtime `recovering` state because no worker-specific restart policy has been implemented.

## 5. Pet state, behavior and personality

Pet State keeps locomotion, facing, posture, attention, emotion, mode and cognition as parallel dimensions. Long-lived scalar state includes energy, curiosity, bond and sleep pressure. `held` is a domain posture, not a Windows-only flag.

Short actions use `BehaviorIntent` priority, remaining lifetime, interruption policy and semantic animation name. Ordinary ambient variation comes from deterministic weighted personality selection without an LLM. Explicit interactions, Focus Guard, held posture and sleep/rest override ambient choice.

## 6. Windows adapters

Current Windows modules include idle/return, local hour, foreground executable identity + privacy-approved DWM bounds, display/DPI/work area, optional bounded UI Automation metadata, selective cursor passthrough and native pet-window motion/drag.

The long-running Windows adapters are registered as named supervised producer workers. Their previous `thread::sleep` loops use cancellation-aware waits so application shutdown does not wait for a 30-second clock poll, a 2-second accessibility poll or a high-frequency motion/cursor loop to wake naturally.

Movement seeds direction from domain `facing`, respects work areas, reverses at edges and only allows autonomous monitor transitions for `Explore` across genuinely adjacent horizontal displays.

## 7. Privacy boundary

`PrivacyPolicyService` starts fail-closed. Foreground identity is limited to the executable stem; window titles are not collected by default. DWM bounds are not queried until the process identity has passed the app exclusion gate.

Rules live in `privacy-rules.json`, separate from memory/history. Updates use a flushed temporary file followed by Windows replace-existing/write-through replacement; in-memory policy mutations roll back when persistence fails.

Windows UI Automation is separately opt-in. It exposes only bounded structural metadata for the focused control and never reads Name, Value, HelpText, raw text or tree dumps. Disabling the capability drops its COM/UIA reader, and transient initialization failures retry after a bounded backoff. Application cancellation also releases the reader through normal worker teardown.

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

The DB worker, changed-only autosave and domain-event journal are supervised as `persistence-db`, `persistence-autosave` and `persistence-event-journal`, but they belong to different shutdown phases. Autosave is a producer; the event journal is the journal phase; the DB owner is the persistence phase.

Application exit no longer relies on quiet-time ordering between those workers. Producer workers are cancelled and joined first, which freezes Pet Runtime state and prevents new successful domain dispatches. The application then reads that frozen snapshot. The journal phase is cancelled and joined next, allowing its already queued events to reach the still-running persistence worker. Only after the journal barrier completes is the frozen Pet State sent through `SaveAndFlush`. Its acknowledgement therefore sits behind previously queued journal/autosave writes. The DB phase is cancelled and joined last.

The SQLite worker uses a 2500 ms busy timeout; final-save acknowledgement allows 3 seconds so the application does not declare timeout before SQLite's own bounded lock wait can complete.

Memory category and transport contracts have one domain source of truth in `src-tauri/src/domain/memory.rs`. `MemoryKind`, `MemoryDraft` and `MemorySearchHit` are shared by persistence and the memory-admin/application boundary rather than being redefined by individual adapters. This removes category drift before the future Memory Evaluator is added.

## 11. UI boundary

Svelte owns management UI, not life simulation or per-frame animation. Current Companion sections are Home, Memory, Activity and Settings/Privacy. Pet interaction and Companion opening are separate; the `☾` handle/tray opens the panel.

The Companion shell delegates section-specific state/UI to `src/lib/ui/companion/*`; new product surfaces should remain isolated instead of growing the shell back into a monolith.

Runtime state synchronization is event-driven: both WebViews listen to the Rust-owned `pet-runtime-snapshot` stream after an initial cold-start read. Pet-window move/scale changes are already observed natively by the pet WebView; after its debounced display-context refresh, it publishes the resulting `pet-display-context` event so Companion does not maintain a second periodic display-context timer. These streams carry small structured snapshots only and do not move animation frames, screenshots or other high-throughput payloads through the event bus.

## 12. Application composition boundary

`lib.rs` is intentionally limited to application composition: managed-service registration, Tauri command registration, Windows adapter wiring, Companion close behavior and ordered worker shutdown/final persistence. Local-data/privacy/persistence/Pet Runtime construction lives in `bootstrap.rs`, lifecycle primitives live in `worker.rs`, native tray behavior lives in `shell.rs` and command handlers live in `commands.rs`.

`bootstrap.rs` owns the Tauri snapshot-event bridge by supplying the Pet Runtime with an application-level observer callback. It also assigns persistence-related workers to phase-scoped supervisor views; persistence code itself does not need to know the whole application shutdown topology.

The supervisor itself is Tauri-managed application state. `worker_status_get` exposes a small structured health snapshot, including each worker's shutdown phase, for future deep-management/debug surfaces without pushing worker administration into the ambient pet UI.

## 13. Worker lifecycle

`WorkerSupervisor` is the common lifecycle boundary for long-running in-process workers. It owns one shared registry and independent cancellation tokens for three ordered phases:

```text
producers
journal
persistence
```

Phase-scoped supervisor views share the same registry/health state but assign newly spawned workers to the selected phase. The default view is `producers`.

Current production-managed workers are classified as:

```text
producers
├─ pet-runtime
├─ persistence-autosave
├─ windows-local-time
├─ windows-idle
├─ windows-active-window
├─ windows-accessibility
├─ windows-cursor-passthrough
└─ windows-pet-motion

journal
└─ persistence-event-journal

persistence
└─ persistence-db
```

Worker health is explicit:

```text
starting
running
stopped
error
panicked
detached
```

Blocking loops must either use `CancellationToken::wait_timeout` or a bounded channel receive timeout and re-check cancellation. The receive timeout is only a cancellation-responsiveness mechanism; correctness no longer depends on one worker guessing that an upstream worker has been quiet for long enough.

Shutdown ordering is deliberate:

```text
cancel + join producers
        ↓
Pet Runtime is frozen; read final snapshot
        ↓
cancel + join journal
        ↓
all accepted journal work has been handed to persistence
        ↓
SaveAndFlush frozen Pet State + wait for acknowledgement
        ↓
cancel + join persistence DB
        ↓
unfinished workers marked detached + named by phase in stderr
```

Each phase has a bounded join deadline. If a producer cannot stop before its deadline, the application reports it as detached and final persistence becomes best-effort rather than pretending the snapshot is guaranteed frozen.

The supervisor catches unexpected worker panics and records them. It does **not** apply a generic automatic-restart loop. SQLite ownership, COM/UI Automation, WebView controllers and future external model processes have different idempotency and resource-recreation requirements; restart must therefore be an explicit worker-specific policy. Runtime `recovering` should only exist when such a real policy is implemented.

## 14. Local AI policy

Text AI remains planned:

```text
northpalace-my-pet.exe → local IPC → northpalace-llm-worker.exe → llama.cpp → MiniCPM5-1B GGUF
```

No hover, petting, walking, sleeping, basic animation or focus reaction may require an LLM call. Future context composition must check structured-context freshness before using observations.

Future external AI workers should reuse the lifecycle semantics established here—named ownership, health, bounded stop/join and explicit restart policy—without making ordinary Pet Runtime behavior depend on them.

## 15. Vision policy

Continuous visual perception is out of scope. Future visual understanding must be separately loadable, opt-in and on-demand behind the same privacy policy.

## 16. Security, validation and reproducibility boundary

The production Tauri bundle uses a restrictive CSP limited to local resources and Tauri IPC. Vite development explicitly uses `devCsp: null`; the production CSP still needs verification in the first successful Windows bundle/run.

Windows CI guards `main` with tracked-data validation, frontend asset/build validation, Rust formatting, Clippy and tests. A separate manual Windows Bundle workflow builds the NSIS/executable artifact. That clean runner also resolves and uploads `package-lock.json` and `src-tauri/Cargo.lock` candidates so dependency locks can be reviewed before committing them.

Builds are not considered fully reproducible until those lockfiles are committed and normal CI switches to locked installs. CI source compatibility also does not replace a clean target-machine run and RAM/CPU/GPU measurements.

## 17. Documentation authority

For engineering behavior: running code/tests → `ARCHITECTURE.md` → `ROADMAP.md` → README. For visual identity, original source evidence and `LENVU_VISUAL_GROUND_TRUTH.md` remain authoritative.
