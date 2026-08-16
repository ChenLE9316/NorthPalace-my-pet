# Lenvu Pet Brain

Pet Brain is the core of NorthPalace-my-pet. It must produce believable behavior without an LLM.

## State dimensions

Initial normalized dimensions:

- `energy`
- `curiosity`
- `bond`
- `focus`

Planned later dimensions:

- comfort
- social need
- confidence
- stimulation
- sleep pressure
- trust

Not every dimension should be shown directly to the user. Some are internal causes of behavior rather than game meters.

## Current V0 behavior

The first code commit intentionally starts with a tiny deterministic system:

```text
0–14 s idle   → Idle
15–59 s       → Observe
60–179 s      → Sit
180–599 s     → Rest
600+ s        → Sleep
```

Interactions reset idle time. Petting raises bond; play spends some energy; Focus Guard holds a focused state.

This is scaffolding, not the final behavioral model.

## Target decision pipeline

```text
Observation
    ↓
Perception / normalization
    ↓
Internal state update
    ↓
Candidate behaviors
    ↓
Constraints + priorities
    ↓
Weighted behavior selection
    ↓
Action intent
    ↓
Animation / bubble / AI request
```

## Reflex vs cognition

### Reflex layer — no LLM

- cursor proximity;
- petting;
- sleeping/waking;
- walking/sitting;
- notification ear twitch;
- focus shield animation;
- idle exploration.

### Cognition layer — LLM may help

- interpreting user language;
- summarizing a cluster of events;
- deciding whether a memory is meaningful;
- generating a nuanced response;
- connecting current context with relevant past memories.

The LLM produces suggestions/intent, not raw animation commands.

## Behavior contract

A future action intent should resemble:

```json
{
  "behavior": "approach_and_check_in",
  "urgency": 0.25,
  "emotion": "gentle_concern",
  "speech_policy": "silent_first",
  "animation": "walk_then_sit",
  "ttl_ms": 12000
}
```

This keeps model output behind a validated domain contract.
