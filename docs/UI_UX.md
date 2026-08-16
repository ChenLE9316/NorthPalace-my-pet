# Lenvu UI/UX and Interaction Model

## Core rule

**Motion and behavior before text.** Lenvu should usually communicate by looking, moving, changing posture, ear/tail motion and short context bubbles. A permanent chat box would make the product feel like an assistant wearing a pet skin; that is not the goal.

## Interaction hierarchy

### Level 1 — Ambient

No conventional UI is required.

States include idle, observe, walk, sit, rest, sleep and explore. The Windows pet overlay uses selective native cursor passthrough: transparent regions do not block the desktop, while semantic Lenvu hit zones and the companion handle remain interactive.

### Level 2 — Direct pet interaction

- hover head/body → glance / ear response;
- head pointer-down → petting reaction + bond change;
- body/tail pointer-down → touch/attention reaction;
- future drag body → reposition/pick-up behavior;
- play action → short playful sequence;
- double-click Lenvu → toggle the independent Companion window.

These actions must feel instant and must not call the LLM.

#### Semantic hit zones

The current V0.2 interaction contract defines forgiving normalized `head`, `body` and `tail` regions in `src/lib/pet/lenvu.manifest.json`. They are mapped into native window coordinates and used by the Windows cursor-passthrough controller. Production frame-aligned masks can replace/augment these regions later without changing Pet Brain commands.

### Level 3 — Context bubble

A compact temporary bubble near Lenvu is used for:

- tiny reactions;
- reminders;
- Focus Guard state;
- AI thinking/speaking state;
- short answers;
- actionable notifications.

The bubble is part of the lightweight pet overlay. It should disappear automatically unless a future interaction explicitly pins or expands it.

### Level 4 — Companion window

The Companion is not 'the app'; it is a deeper view of the companion. It is now a separate Tauri window from the transparent pet overlay, so closing/hiding it does not stop Lenvu's Rust Pet Runtime or PixiJS pet layer.

Current sections:

- runtime/pet status — energy, curiosity, bond, sleep pressure;
- state — posture, attention, emotion, cognition;
- interact — pet/play/Focus Guard actions;
- renderer/debug context — animation id, sequence and DPI/display information.

Planned sections:

- conversation — optional AI dialogue;
- memories — meaningful moments and editable memory;
- activity — what Lenvu has noticed and done.

### Level 5 — Deep management

Dedicated settings for model/runtime, privacy, memory, performance, display, startup, animation and debug remain planned as deeper management surfaces.

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

Implemented now:

1. `pet` —  transparent, always-on-top, taskbar-hidden desktop-pet overlay; PixiJS renderer + compact bubble + Companion handle.
2. `companion` — independently show/hide-able managed window for status and deeper interaction. Native close is converted into hide so it can reopen without restarting Pet Runtime.

Planned:

3. `settings` — normal managed application window.
4. `debug` — development-only event/state inspector.

The pet window must remain lightweight when deeper UI is closed.

## Desktop movement UX

`walk` and `run` locomotion now drive actual native pet-window horizontal movement. The controller:

- stays within the current monitor work area;
- keeps Lenvu above the taskbar/work-area boundary;
- reverses at horizontal edges;
- converts logical movement speed through the current DPI scale factor;
- deliberately avoids autonomous monitor crossing until multi-monitor policy is designed.

Jump currently remains an in-character renderer action instead of moving the native window vertically. This prevents a premature physics/window-position coupling.

## Visual language

- deep blue/near-black UI surfaces;
- cyan Lumen-Code highlights;
- restrained violet for emotional/AI accents;
- gold only for Lenvu's horn-ring identity details;
- translucent holographic rings for focus/protection/system states;
- avoid constant glow/particles when idle to protect Vega 8 resources;
- use animation-specific low-power frame budgets for rest/sleep states.

Reference board: `docs/assets/lenvu-system-overview.webp`.
Character rules: `docs/CHARACTER_BIBLE.md`.
Runtime asset contract: `docs/ASSET_PIPELINE.md`.
