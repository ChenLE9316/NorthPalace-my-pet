# Lenvu Pet Brain

Pet Brain is the core of NorthPalace-my-pet. It must produce believable behavior without an LLM or a vision model.

## V0 status

The repository still contains the original V0 `PetActivity` prototype for compatibility while V0.2 is introduced. V0.2 adds separate contracts for parallel state, domain events and behavior intents. The next migration step is to move simulation ownership into the Rust runtime clock and retire UI-owned ticking.

## Parallel state model (V0.2 target)

Lenvu does not have one exclusive activity. Several dimensions coexist:

```text
PetState
|- locomotion: stationary | walk | run | jump
|- posture: stand | sit | lie | sleep
|- attention: idle | user | cursor | window | object
|- emotion: calm | curious | happy | shy | concerned | sleepy
|- mode: ambient | focus_guard | do_not_disturb | play
`- cognition: idle | listening | thinking | speaking | remembering
```

Example valid state:

```text
posture     = sit
attention   = user
emotion     = curious
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

Short-lived actions are represented independently from long-lived state. They carry duration/TTL, priority and interruption policy so reactions survive more than one tick.

```json
{
  "kind": "receive_pet",
  "priority": 60,
  "remainingMs": 3200,
  "interruptible": true,
  "animation": "pet_receive"
}
```

Examples:

- receive pet;
- play;
- wake;
- settle to rest;
- focus guard;
- observe user.

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
Weighted behavior selection
          |
          v
Behavior Intent
          |
          +--> Animation intent
          +--> Context bubble policy
          `--> Optional AI request
```

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

V0 used a fixed idle threshold. The target system should select rest/sleep based on utility from multiple signals such as energy, sleep pressure, time of day, recent interaction and user idle state. Sleeping should recover energy rather than continue draining it.

## User focus is not pet focus

Avoid a single ambiguous `focus: f32`. Keep these concepts separate:

- pet `attention` / `alertness`;
- user focus-session/context state;
- `focus_guard` system mode.

## Runtime ownership

Rust owns simulation time. Svelte issues commands and renders immutable snapshots; the UI is not allowed to keep Lenvu alive by repeatedly calling a tick function.

## Acceptance test

With the text model and any future vision worker fully unloaded, Lenvu must still have a complete behavior path for ambient life, sleep/wake, direct interaction, focus mode and persistence.
