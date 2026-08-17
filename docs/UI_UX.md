# Lenvu UI/UX and Interaction Model

## Core rule

**Motion and behavior before text.** Lenvu should usually communicate by looking, moving, changing posture, ear/tail motion and short context bubbles. A permanent chat box would make the product feel like an assistant wearing a pet skin; that is not the goal.

## Interaction hierarchy

### Level 1 — Ambient

No conventional UI is required.

States include idle, observe, walk, sit, rest, sleep and explore. The Windows pet overlay uses selective native cursor passthrough: transparent regions do not block the desktop, while semantic Lenvu hit zones and the companion handle remain interactive.

Ambient behavior is intentionally quiet. Weighted personality decisions such as ordinary observe, sit and explore do not automatically create speech bubbles.

### Level 2 — Direct pet interaction

- hover head/body → glance / ear response;
- head short press → petting reaction + bond change;
- body/tail short press → touch/attention reaction;
- hold and move beyond the gesture threshold → pick up and native window drag;
- play action → short playful sequence;
- double-click Lenvu → toggle the independent Companion window.

These actions must feel instant and must not call the LLM.

#### Semantic hit zones

The current V0.2 interaction contract defines forgiving normalized `head`, `body` and `tail` regions in `src/lib/pet/lenvu.manifest.json`. They are mapped into native window coordinates and used by the Windows cursor-passthrough controller. Production frame-aligned masks can replace/augment these regions later without changing Pet Brain commands.

### Level 3 — Context bubble

The Context Bubble V1 is implemented as a low-noise presentation policy rather than a permanent state label.

It can appear for meaningful transitions such as:

- being picked up;
- receiving affection or play;
- Focus Guard start/end;
- sleep/wake transitions;
- Pet Runtime degradation/recovery.

Ordinary ambient stay/observe/sit/explore behavior remains silent. Bubble cues have priority, short display TTL and repeat cooldown, so lower-priority chatter cannot continuously replace an important runtime/focus message.

Current tones:

- `soft` — ordinary companion reaction;
- `focus` — Focus Guard/system-attention state;
- `warning` — degraded/recovering/error runtime state.

The bubble remains pointer-transparent and automatically disappears. Its JavaScript timer controls presentation lifetime only; it does not advance or own Pet Brain simulation time.

Future AI thinking/speaking and actionable reminders can use the same cue boundary, but must obey the same low-noise policy.

### Level 4 — Companion window

The Companion is not “the app”; it is a deeper view of the companion. It is a separate Tauri window from the transparent pet overlay, so closing/hiding it does not stop Lenvu's Rust Pet Runtime or PixiJS pet layer.

Companion V2 now uses a persistent status summary plus three explicit tabs:

```text
Companion
├─ Home
│  ├─ direct pet actions
│  ├─ posture / attention / emotion / cognition
│  ├─ renderer/runtime context
│  └─ concept / product identity
├─ Memory
│  ├─ local FTS5 search
│  ├─ manual remember
│  ├─ edit / forget
│  └─ source-event provenance
└─ Activity
   ├─ meaningful low-frequency events
   ├─ relationship context
   └─ bond delta / timestamp
```

`Memory` and `Activity` are lazy-loaded the first time their tabs are opened. Home does not pre-read both SQLite management views on every Companion launch. A pet/play/focus interaction marks Activity as stale; it refreshes immediately only when the Activity tab is currently visible, otherwise it reloads on the next visit.

This tab boundary is also the reserved extension point for future `Chat` and `Settings` without turning Companion into a single unbounded scrolling page.

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
├─ PickUp / Drag / Drop
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

## Window and tray model

Implemented now:

1. `pet` — transparent, always-on-top, taskbar-hidden desktop-pet overlay; PixiJS renderer + compact Context Bubble + Companion handle.
2. `companion` — independently show/hide-able managed window for Home, Memory and Activity. Native close is converted into hide so it can reopen without restarting Pet Runtime.
3. native system tray — persistent shell control while the pet is running.

Tray V1 behavior:

```text
left click tray
└─ show + focus Companion

tray menu
├─ Open Lenvu Companion
├─ Show / Hide Lenvu
└─ Quit NorthPalace-my-pet
```

The tray is owned by the Rust/Tauri shell, not a new background service. A true Windows launch-at-login/startup preference remains a separate feature and should be explicit/user-controlled.

Planned windows:

4. `settings` — normal managed application window.
5. `debug` — development-only event/state inspector.

The pet window must remain lightweight when deeper UI is closed.

## Desktop movement UX

`walk` and `run` locomotion drive actual native pet-window horizontal movement. The controller:

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
- muted warm warning tone only for runtime degradation/error;
- avoid constant glow/particles when idle to protect Vega 8 resources;
- use animation-specific low-power frame budgets for rest/sleep states.

Reference board: `docs/assets/lenvu-system-overview.webp`.
Character rules: `docs/CHARACTER_BIBLE.md`.
Runtime asset contract: `docs/ASSET_PIPELINE.md`.
