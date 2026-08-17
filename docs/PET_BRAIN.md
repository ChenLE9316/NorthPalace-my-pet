# Lenvu Pet Brain

Pet Brain is the core of NorthPalace-my-pet. It must produce believable behavior without an LLM or a vision model.

## V0.2 status

The original single `PetActivity` Pet Brain and JavaScript-owned `tick_pet` loop have been retired. V0.2 now runs through a Rust-owned Pet Runtime with:

- monotonic runtime ticks;
- parallel pet state dimensions;
- Domain Events;
- Behavior Intents with priority, duration and interruption policy;
- immutable runtime snapshots for presentation;
- explicit runtime health;
- a low-cost Windows idle/return sensor;
- PixiJS as the high-frequency visual renderer boundary;
- a deterministic weighted personality selector for ambient behavior.

Svelte polls snapshots for presentation only. It no longer advances Lenvu's simulation clock.

## Parallel state model

Lenvu does not have one exclusive activity. Several dimensions coexist:

```text
PetState
|- locomotion: stationary | walk | run | jump
|- posture: stand | sit | lie | sleep
|- attention: idle | user | cursor | window | object
|- emotion: calm | curious | happy | shy | concerned | sleepy | focused
|- mode: ambient | focus_guard | do_not_disturb | play
`- cognition: idle | listening | thinking | speaking | remembering
```

Example valid state:

```text
posture     = sit
attention   = user
emotion     = focused
mode        = focus_guard
cognition   = idle
```

This avoids conflating movement, pose, emotion and AI state into one enum.

## Internal dimensions

Initial normalized dimensions:

- `energy`
- `curiosity`
- `bond`
- `sleep_pressure`

Planned later dimensions:

- comfort
- social need
- confidence
- stimulation
- trust
- alertness

Not every dimension should be exposed as a visible game meter. Some are internal causes of behavior.

## Behavior Intent

Short-lived actions are represented independently from long-lived state. They carry duration/TTL, priority and interruption policy so reactions survive more than one simulation tick.

```json
{
  "kind": "receive_pet",
  "priority": 60,
  "remainingMs": 3200,
  "interruptible": true,
  "animation": "pet_receive"
}
```

Current intent families include:

- ambient sitting;
- explore;
- observe user;
- receive pet;
- play;
- wake;
- settle to rest;
- sleep entry;
- focus guard entry.

Persistent modes such as Focus Guard remain in `PetState`; a short Behavior Intent represents their entry/reaction animation rather than the whole mode lifetime.

## Decision pipeline

```text
Observation / Domain Event
          |
          v
Perception / normalization
          |
          v
Internal state update
          |
          v
Candidate behaviors
          |
          v
Constraints + priorities + needs
          |
          v
Behavior selection
          |
          v
Behavior Intent
          |
          +--> Animation intent
          +--> Context bubble policy
          `--> Optional AI request
```

## Weighted ambient personality selector

Ambient behavior is no longer tied to a single fixed explore timer. A low-frequency pure Rust selector evaluates candidate actions while the user is active and Lenvu is in `ambient` mode.

Current candidate set:

```text
Stay
Observe
Sit
Explore
```

The selector consumes normalized context:

- energy;
- curiosity;
- bond;
- sleep pressure;
- user idle time;
- local hour;
- elapsed ambient decision time.

The canonical Lenvu baseline currently uses four internal tuning traits:

```text
curiosity_drive
calmness
sociability
independence
```

These are behavior tuning constants, not user-facing game meters. They provide Lenvu with a recognizable baseline temperament while keeping all ordinary ambient behavior independent from the LLM.

Selection is weighted but deterministic for the same state and decision index. This gives variation without introducing an opaque global RNG into Pet Brain tests. Higher curiosity and wakefulness increase explore pressure; stronger bond/sociability increase observing behavior; lower energy and higher sleep pressure increase calm sitting; night hours suppress exploration before the separate rest/sleep policy takes over.

Future long-term personality evolution may adjust bounded traits from relationship history, but it must not let an LLM directly command animation or bypass behavior constraints.

## Current deterministic reflexes

No AI is required for:

- cursor enter/leave;
- touch and petting;
- play requests;
- sleeping/waking;
- Focus Guard entry/exit;
- Windows user idle/return state;
- weighted ambient stay/observe/sit/explore selection;
- simple rest/sleep utility selection;
- energy drain and sleep recovery;
- AI worker availability state.

## Reflex vs cognition

### Reflex layer — no AI required

- cursor proximity;
- petting;
- sleeping/waking;
- walking/sitting;
- notification ear twitch;
- focus shield animation;
- idle exploration;
- user return greeting motion;
- time-of-day resting tendencies.

### Cognition layer — text LLM may help

- interpreting user language;
- summarizing a cluster of events;
- evaluating whether an event is worth remembering;
- generating a nuanced reply;
- relating current context to stored memories.

### Visual cognition — optional, later

Pet Brain must never depend on raw screenshots. Future screen vision produces a validated `ScreenObservation` through a separate Screen Context Broker. Structured Windows/accessibility context is preferred whenever possible.

## Sleep and recovery

Rest and sleep are selected from multiple signals rather than a single hard-coded inactivity enum transition. Current V0.2 inputs include user idle time, energy, sleep pressure and time-of-day bias. Sleeping recovers energy and reduces sleep pressure.

The policy remains deliberately simple until it can be tuned against real usage on the target Ryzen 3 2200G machine.

## User focus is not pet focus

These concepts remain separate:

- pet `attention` / future `alertness`;
- user idle/focus-session context;
- `focus_guard` system mode.

There is no ambiguous global `focus: f32` field in V0.2.

## Runtime ownership

```text
Windows sensor / UI command
          |
          v
      Domain Event
          |
          v
 Rust Pet Runtime actor
          |
          +--> monotonic Tick
          +--> PetBrainV2
          +--> RuntimeSnapshot
                    |
              +-----+------+
              |            |
           PixiJS        Svelte
          renderer        panels
```

Rust owns simulation time. Svelte issues commands and renders immutable snapshots; the UI is not allowed to keep Lenvu alive by repeatedly calling a tick function.

## Acceptance test

With the text model and any future vision worker fully unloaded, Lenvu must still have a complete behavior path for ambient life, sleep/wake, direct interaction, focus mode and persistence.
