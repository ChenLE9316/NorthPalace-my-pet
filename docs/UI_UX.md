# Lenvu UI/UX and Interaction Model

## Core rule

**Motion and behavior before text.** Lenvu should usually communicate by looking, moving, changing posture, ear/tail motion and short context bubbles. A permanent chat box would make the product feel like an assistant wearing a pet skin; that is not the goal.

## Interaction hierarchy

### Level 1 — Ambient

No conventional UI is required.

States include idle, observe, walk, sit, rest, sleep and explore. Transparent desktop regions should eventually be click-through while Lenvu's hit area remains interactive.

### Level 2 — Direct pet interaction

- hover head/body → glance / ear response;
- click → attention;
- pet gesture → affection animation + bond change;
- drag body → reposition/pick-up behavior;
- play action → short playful sequence;
- double-click → companion panel.

These actions must feel instant and must not call the LLM.

### Level 3 — Context bubble

A compact temporary bubble near Lenvu is used for:

- tiny reactions;
- reminders;
- Focus Guard state;
- AI thinking/speaking state;
- short answers;
- actionable notifications.

The bubble disappears automatically unless pinned or expanded.

### Level 4 — Companion panel

The panel is not 'the app'; it is a deeper view of the companion.

Initial sections:

- status — mood, energy, bond, focus;
- interact — pet/play/focus actions;
- conversation — optional AI dialogue;
- memories — recent meaningful moments;
- activity — what Lenvu has noticed and done.

### Level 5 — Deep management

Dedicated settings for model/runtime, privacy, memory, performance, display, startup, animation and debug.

## Interaction state families

```text
Ambient
├─ Idle
├─ Observe
├─ Walk
├─ Sit
├─ Rest
└─ Sleep

Direct
├─ Hover
├─ Touch
├─ Pet
├─ Drag
└─ Play

Awareness
├─ UserReturned
├─ UserIdle
├─ ActiveWindowChanged
├─ Notification
└─ TimeOfDay

AI
├─ Listening
├─ Thinking
├─ Speaking
├─ Remembering
└─ Suggesting

Special
├─ FocusGuard
├─ DoNotDisturb
├─ LowEnergy
├─ Sleep
└─ OfflineBrain
```

## Window model

Planned windows:

1. `pet` — transparent always-on-top overlay.
2. `companion` — compact side/popover panel.
3. `settings` — normal managed application window.
4. `debug` — development-only state/event inspector.

The pet window should never need to carry the full settings/chat interface in memory if it is not open.

## Visual language

- deep blue/near-black UI surfaces;
- cyan Lumen-Code highlights;
- restrained violet for emotional/AI accents;
- gold only for Lenvu's horn-ring identity details;
- translucent holographic rings for focus/protection/system states;
- avoid constant glow/particles when idle to protect Vega 8 resources.

Reference board: `docs/assets/lenvu-system-overview.webp`.
