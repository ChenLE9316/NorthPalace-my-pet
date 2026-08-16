# NorthPalace-my-pet Foundation Review

This document records the first architecture review before the project grows into memory, Windows awareness, animation and local AI.

## Decisions to keep

- Tauri 2 desktop shell.
- Rust runtime/core.
- Svelte + TypeScript for application UI.
- PixiJS for the pet renderer.
- SQLite + FTS5 for local persistence/search.
- MiniCPM5-1B + llama.cpp as an optional local cognition layer.
- Separate LLM worker process.
- Pet-first, AI-second product philosophy.
- Target machine remains Windows 11 / Ryzen 3 2200G / 16 GB DRAM / Vega 8.

## Corrections required before feature growth

### 1. Runtime clock belongs to Rust

The Pet Brain must not depend on a JavaScript `setInterval` to stay alive. Rust owns monotonic time, simulation ticks and recovery after UI throttling/suspend. Frontend code receives snapshots/events and renders them.

```text
Rust Runtime Clock
       |
       v
   Pet Runtime
       |
       v
    Pet Brain
       |
       +--> snapshots/events --> UI
```

### 2. Pet state is parallel, not one activity enum

Movement, posture, attention, emotion, system mode and cognition can coexist.

```text
PetState
|- locomotion: stationary | walk | run | jump
|- posture: stand | sit | lie | sleep
|- attention: idle | user | cursor | window | object
|- emotion: calm | curious | happy | shy | concerned | sleepy
|- mode: ambient | focus_guard | do_not_disturb | play
`- cognition: idle | listening | thinking | speaking | remembering
```

This allows states such as `sit + curious + user attention + focus_guard + cognition idle`.

### 3. Behavior Intent is separate from persistent state

Short interactions must survive longer than one simulation tick. A behavior intent carries priority, duration/TTL, interruptibility and a renderer-facing animation intent.

Example:

```json
{
  "kind": "receive_pet",
  "priority": 60,
  "remaining_ms": 3200,
  "interruptible": true,
  "animation": "pet_receive"
}
```

### 4. Domain events form the backbone

Sensors publish facts; Pet Runtime interprets them.

Initial events:

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
- `LLMWorkerStateChanged`

### 5. UI is not the source of truth

Svelte should issue commands and render state. It must not own simulation time, Pet Brain state, memory rules or platform sensors.

### 6. Window model

Logical windows/layers:

1. `pet` — transparent desktop pet overlay.
2. `bubble` — short-lived context/speech layer.
3. `companion` — status/chat/memory panel.
4. `settings` — model/privacy/performance configuration.
5. `debug` — development-only runtime inspector.

The pet overlay must remain lightweight when deeper UI is closed.

### 7. Transparent does not mean click-through

Windows hit-testing must distinguish the animated Lenvu body from transparent pixels. Transparent areas should pass pointer input to the application/desktop underneath; the pet hit region remains interactive. Hit masks must be compatible with animation frames and DPI scaling.

### 8. Renderer boundary

PixiJS owns high-frequency rendering concerns:

- sprite atlases;
- animation graph;
- anchor/pivot;
- effects/particles;
- hit masks;
- renderer LOD/FPS policy.

Svelte owns panels/settings, not per-frame pet animation.

### 9. Error state must be explicit

Frontend fallbacks must not silently make runtime failures look like a normal idle pet. Runtime status should expose `ready`, `degraded`, `recovering` or `error`, with detailed diagnostics in development mode.

### 10. Avoid one global lock as the system grows

The initial `Mutex<PetBrain>` is acceptable only as scaffolding. The target is a single-owner Pet Runtime actor/task receiving commands/events and publishing immutable snapshots. SQLite, model I/O and Windows sensors must not block the whole Pet Brain lock.

## Asset pipeline

Reference art and runtime assets are different products.

```text
assets/
|- reference/
|  |- anatomy/
|  |- expressions/
|  |- movement/
|  |- behavior/
|  |- abilities/
|  `- ui-concepts/
`- runtime/
   `- lenvu/
      |- sprites/
      |- atlases/
      |- masks/
      |- effects/
      `- manifest.json
```

The current concept sheets define the visual bible. Production animation must normalize anatomy, scale, anchors and silhouette before frames are assembled.

## Public-release checklist

Before changing the repository to public:

- choose code license;
- define a separate Lenvu character/artwork license;
- add dependency lockfiles;
- add Windows CI build/check;
- add `SECURITY.md`;
- restore a restrictive Content Security Policy;
- document asset provenance/licensing;
- record first R3 2200G performance baseline;
- verify no models, private data, local DBs, secrets or logs are tracked.

## Non-negotiable acceptance test

Unload MiniCPM5-1B completely.

Lenvu must still feel alive: movement, sleep/wake, petting, attention, focus behavior, state persistence and ordinary desktop interaction continue without an LLM.
