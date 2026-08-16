# NorthPalace-my-pet Architecture

## 1. Target

Primary target: **Windows 11 / Ryzen 3 2200G / 16 GB DRAM / Vega 8 iGPU**.

The product is an always-running digital companion, so idle cost matters more than peak benchmark performance. The architecture must allow every expensive subsystem — especially the LLM — to sleep or unload independently.

## 2. Layer model

```text
Presentation
├─ Pet Overlay (PixiJS)
├─ Context Bubble
├─ Companion Panel (Svelte)
└─ Settings / Debug
        │
Application
├─ Commands
├─ Orchestrators
└─ Event routing
        │
Domain
├─ Pet Brain
├─ Behavior policy
├─ Emotion / needs
├─ Relationship / bond
└─ Memory policy
        │
Infrastructure
├─ Windows adapter
├─ SQLite / FTS5
├─ file/config/log storage
└─ isolated LLM worker → llama.cpp → MiniCPM5-1B GGUF
```

The **Domain layer must not depend on Svelte, PixiJS, Win32 or llama.cpp**.

## 3. Runtime boundaries

### Main desktop process

Owns the Tauri shell, Pet Brain, event bus, Windows integration and persistence coordination.

### WebView presentation

Owns rendering and presentation only. It sends typed commands/events into Rust and renders the resulting domain state.

### LLM worker process

Planned as a separate executable. The worker can crash, reload, trim context or unload without killing Lenvu. Initial IPC will be stdio or a Windows Named Pipe with JSON messages; protobuf is deliberately deferred.

## 4. Event-driven backbone

Planned domain events include:

- `UserIdleChanged`
- `UserReturned`
- `CursorEnteredPet`
- `PetTouched`
- `PetPetted`
- `ActiveWindowChanged`
- `FocusModeStarted`
- `FocusModeEnded`
- `NotificationReceived`
- `TimeOfDayChanged`
- `PetStateChanged`
- `LLMResponseReady`
- `MemoryCreated`

Sensors publish facts. Pet Brain interprets facts. Presentation renders decisions.

## 5. Memory

SQLite is the only database planned for V1.

Memory categories:

- episodic — events worth remembering;
- semantic — stable learned facts;
- preference — user preferences;
- relationship — bond/history between user and Lenvu;
- system — pet/runtime state.

V1 retrieval: FTS5 + metadata + recency + importance. A vector database is explicitly out of scope until evidence shows it is necessary.

## 6. LLM policy

MiniCPM5-1B is the planned local cognition layer. Its long context is a capability ceiling, not an instruction to keep a 128K KV cache resident all day.

Prompt composition should be selective:

```text
Lenvu identity
+ current pet state
+ current environment
+ relevant memories
+ recent conversation
+ current request
```

No mouse movement, animation frame, hover event or basic pet reaction is allowed to require an LLM call.

## 7. Resource policy

The first performance budgets are design targets, not measured guarantees:

- shell/core + ordinary UI should remain small enough for all-day use;
- the pet renderer should reduce work when static or occluded;
- model memory is separately budgeted and unloadable;
- animation quality can scale down under pressure;
- telemetry/debug tracing must be bounded and rotate.

## 8. Failure policy

If any subsystem fails:

- LLM failure → AI becomes unavailable, pet continues;
- database failure → fall back to temporary session state and surface diagnostics;
- renderer failure → runtime remains recoverable;
- asset failure → render fallback placeholder state;
- Windows sensor failure → disable that sensor rather than blocking Pet Brain.
