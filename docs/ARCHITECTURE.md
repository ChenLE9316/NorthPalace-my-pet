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

`RuntimeHandle` also owns an input gate used only for ordered shutdown. Normal `dispatch()` holds a read guard through both the runtime-channel send and domain-event subscriber sends. `close_event_input()` takes the write guard and flips the gate closed, which means it waits for in-flight accepted dispatches to finish and prevents any later WebView or adapter dispatch from entering the final drain window.

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

SQLite is bundled through `rusqlite`. Current structures cover pet state, bounded activity/relationship history, typed memories, FTS5 and hourly interaction rhythm. The production application has one long-lived SQLite owner: the supervised `persistence-db` worker.

Pet-state saves, autosave, domain-event journaling, hourly rhythm writes, future memory-candidate storage, Memory Browser CRUD/search and Activity History list/get all serialize through the same `PersistenceCommand` queue. `memory_admin.rs` and `history_admin.rs` contain input validation, record types and SQL helpers only; they no longer own database paths or open per-command SQLite connections. This keeps SQLite off Pet Brain ticks and avoids a second application-level connection competing with the persistence owner.

The single queue also provides an ordering guarantee for management UI reads: a Memory or Activity query sent after earlier queued writes cannot overtake those writes. Session-only fallback remains explicit; admin commands report persistence/history unavailable when the DB owner was not installed rather than silently opening another connection.

The DB worker, changed-only autosave and domain-event journal are supervised as `persistence-db`, `persistence-autosave` and `persistence-event-journal`, but they belong to different shutdown phases. Autosave is a producer; Pet Runtime has its own runtime phase; the event journal is the journal phase; the DB owner is the persistence phase.

Application exit no longer relies on quiet-time ordering. Platform sensors/controllers and autosave producers are cancelled and joined first so no supervised upstream worker continues generating runtime or persistence work. The application then closes the Pet Runtime input gate; that operation waits for any in-flight dispatch to finish its runtime-channel and journal-subscriber sends and rejects every later dispatch. Pet Runtime is then cancelled in its own phase; before returning, it drains every already accepted event remaining in its input queue and publishes that final semantic snapshot. Only after the runtime join barrier completes does the application read the frozen snapshot.

The journal phase is cancelled and joined next, allowing its already accepted domain events to reach the still-running persistence worker. After that barrier, the frozen Pet State is sent through `SaveAndFlush`; because the final save command is queued after all journal sends have completed, its acknowledgement also proves that earlier queued persistence work has been processed. The DB phase is cancelled and joined last.

The SQLite worker uses a 2500 ms busy timeout; final-save acknowledgement and management-command acknowledgements use bounded application waits so a wedged DB owner does not block the application indefinitely.

Memory category and transport contracts have one domain source of truth in `src-tauri/src/domain/memory.rs`. `MemoryKind`, `MemoryDraft` and `MemorySearchHit` are shared by persistence and the memory-admin/application boundary rather than being redefined by individual adapters. This removes category drift before the future Memory Evaluator is added.

## 11. UI boundary

Svelte owns management UI, not life simulation or per-frame animation. Current Companion sections are Home, Memory, Activity and Settings/Privacy. Pet interaction and Companion opening are separate; the `☾` handle/tray opens the panel.

The Companion shell delegates section-specific state/UI to `src/lib/ui/companion/*`; new product surfaces should remain isolated instead of growing the shell back into a monolith.

Runtime state synchronization is event-driven: both WebViews listen to the Rust-owned `pet-runtime-snapshot` stream after an initial cold-start read. Pet-window move/scale changes are already observed natively by the pet WebView; after its debounced display-context refresh, it publishes the resulting `pet-display-context` event so Companion does not maintain a second periodic display-context timer. These streams carry small structured snapshots only and do not move animation frames, screenshots or other high-throughput payloads through the event bus.

## 12. Application composition boundary

`lib.rs` is intentionally limited to application composition: managed-service registration, Tauri command registration, Windows adapter wiring, Companion close behavior and ordered worker shutdown/final persistence. `PersistenceService` is the only Tauri-managed SQLite application service; Memory/Activity commands use it rather than registering parallel DB services. Local-data/privacy/persistence/Pet Runtime construction lives in `bootstrap.rs`, lifecycle primitives live in `worker.rs`, native tray behavior lives in `shell.rs` and command handlers live in `commands.rs`.

`bootstrap.rs` owns the Tauri snapshot-event bridge by supplying the Pet Runtime with an application-level observer callback. It assigns Pet Runtime, event journal and DB ownership to phase-scoped supervisor views while leaving Windows adapters and autosave on the default producer phase; persistence code itself therefore does not need to know the whole application shutdown topology.

The supervisor itself is Tauri-managed application state. `worker_status_get` exposes a small structured health snapshot, including each worker's shutdown phase, for future deep-management/debug surfaces without pushing worker administration into the ambient pet UI.

## 13. Worker lifecycle

`WorkerSupervisor` is the common lifecycle boundary for long-running in-process workers. It owns one shared registry and independent cancellation tokens for four ordered phases:

```text
producers
runtime
journal
persistence
```

Phase-scoped supervisor views share the same registry/health state but assign newly spawned workers to the selected phase. The default view is `producers`.

Current production-managed workers are classified as:

```text
producers
├─ persistence-autosave
├─ windows-local-time
├─ windows-idle
├─ windows-active-window
├─ windows-accessibility
├─ windows-cursor-passthrough
└─ windows-pet-motion

runtime
└─ pet-runtime

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

Blocking loops must either use `CancellationToken::wait_timeout` or a bounded channel receive timeout and re-check cancellation. Receive timeouts are cancellation-responsiveness mechanisms only; correctness does not depend on one worker guessing that an upstream worker has been quiet for long enough.

Shutdown ordering is deliberate:

```text
cancel + join producers
        ↓
close Runtime input gate; wait for in-flight dispatches
        ↓
cancel runtime; drain accepted runtime events; join runtime
        ↓
read frozen Pet Runtime snapshot
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

Each phase has a bounded join deadline. If an upstream producer or the runtime itself cannot stop before its deadline, final persistence is best-effort rather than being represented as a guaranteed frozen-state save.

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

`package-lock.json` and `src-tauri/Cargo.lock` are committed from clean `windows-latest` resolution/validation runs. Frontend diagnostics are also locked: the ordinary Svelte checker runs against TypeScript 6.0.3, while `svelte-check --tsgo` uses a TypeScript 7.0.2 npm alias. Both are configured with `--fail-on-warnings`; `vite/client` and Node types are explicit, while `skipLibCheck` isolates third-party Pixi/WebGPU declaration-merge conflicts instead of suppressing project-source diagnostics.

Normal Windows CI installs frontend dependencies with `npm ci`, runs both zero-warning Svelte gates, builds the Svelte/PixiJS frontend, verifies the Cargo graph with `cargo metadata --locked`, checks Rust formatting, runs Clippy with `-D warnings`, and runs Rust tests with Cargo `--locked`. `docs/SVELTE_DIAGNOSTIC_BASELINE.md` records a clean Windows run for both Svelte/TypeScript paths; `docs/VALIDATION_BASELINE.md` records a clean Windows run for frontend build plus Rust formatting/Clippy/tests.

The manual Windows Bundle workflow consumes the same committed locks, verifies Cargo metadata under `--locked`, builds the NSIS/executable artifact, and does not generate an independent dependency graph.

Dependency resolution and source-level validation are therefore reproducible at the repository/CI boundary. This does **not** yet establish a successful clean Windows NSIS run, production CSP verification, or target-machine RAM/CPU/GPU performance; those remain separate acceptance gates.

## 17. Documentation authority

For engineering behavior: running code/tests → `ARCHITECTURE.md` → `ROADMAP.md` → README. For visual identity, original source evidence and `docs/LENVU_VISUAL_GROUND_TRUTH.md` remain authoritative.
