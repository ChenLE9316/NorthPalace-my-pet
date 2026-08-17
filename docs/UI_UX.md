# Lenvu UI/UX and Interaction Model

## Core rule

**Motion and behavior before text.** Lenvu should usually communicate by looking, moving, changing posture, ear/tail motion and short context bubbles. A permanent chat box would make the product feel like an assistant wearing a pet skin; that is not the goal.

The second rule is **quiet by default**. Ambient behavior, system awareness and local context should not continuously interrupt the user.

## Interaction hierarchy

### Level 1 — Ambient

No conventional UI is required.

States include idle, observe, walk, sit, rest, sleep and explore. The Windows pet overlay uses selective native cursor passthrough: transparent regions do not block the desktop, while semantic Lenvu hit zones and the Companion handle remain interactive.

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

The interaction contract defines forgiving normalized `head`, `body` and `tail` regions in `src/lib/pet/lenvu.manifest.json`. They are mapped into native window coordinates and used by the Windows cursor-passthrough controller. Production frame-aligned masks can replace/augment these regions later without changing Pet Brain commands.

### Level 3 — Context Bubble

The Context Bubble V1 is a low-noise presentation policy rather than a permanent state label.

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

The Companion is not "the app"; it is a deeper view of the companion. It is a separate Tauri window from the transparent pet overlay, so closing/hiding it does not stop Lenvu's Rust Pet Runtime or PixiJS pet layer.

The current navigation is:

```text
Companion
├─ Home
├─ Memory
├─ Activity
└─ Settings
```

The tabs are deliberately lazy where deeper management I/O is involved.

#### Home

Home stays lightweight and shows:

- energy / curiosity / bond / sleep pressure summary;
- pet/play/Focus Guard controls;
- posture / attention / emotion / cognition;
- renderer/runtime diagnostics such as animation id, sequence and DPI/display context.

#### Memory

Memory is loaded when opened and provides:

- recent local long-term memories;
- FTS5 search;
- kind filtering;
- explicit “remember this” creation;
- content/kind/importance editing;
- source-event provenance;
- explicit delete / forget.

It does not require MiniCPM5-1B to be loaded.

#### Activity

Activity is loaded when opened and shows only bounded low-frequency meaningful events already written by the persistence layer, such as reunion, affection, play and Focus Guard transitions.

It is explicitly **not** a foreground-app history, cursor log or screen-history surface.

#### Settings

Settings is mounted/read only when opened.

Current controls:

1. **Windows 開機啟動**
   - off by default;
   - opt-in only;
   - reads the actual OS launch-at-login registration;
   - enable/disable is performed by the Rust shell through the official Tauri autostart integration;
   - the checkbox is never treated as source of truth before the backend confirms the actual state.

2. **App Privacy Exclusions**
   - user manually enters a process app-id such as `discord` or `keepassxc`;
   - case and a trailing `.exe` are normalized;
   - the UI does not automatically build a list of recently observed applications;
   - rules are local and removable;
   - an excluded app is blocked before active-app identity reaches Domain Events or Screen Context Broker;
   - fail-closed state is shown explicitly rather than presented as an empty rule list.

The Settings UX should always explain what a control changes. Privacy controls are capability boundaries, not cosmetic switches.

### Level 5 — Deep management

The current Settings tab can grow to include model/runtime, memory, performance, display, animation quality and debug controls. If it becomes too dense, those surfaces may move into a dedicated managed Settings window without changing the underlying Rust services.

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
├─ TimeOfDay
└─ future structured context

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

## Privacy-aware awareness UX

Lenvu's awareness must be useful without feeling invasive.

Current structured awareness uses process app identity, user idle state and local hour. The Screen Context Broker contains no pixels, screenshot history or window-title history.

```text
Windows sensor
    ↓
Privacy gate
    ├─ blocked → identity discarded / broker marked privacy_blocked
    └─ allowed → structured context only
```

When the current foreground application is added to the deny list, the normal one-second active-window sensor tick re-evaluates privacy and clears the broker's previous app identity. The user does not have to switch windows for the new rule to take effect.

Future window-title/accessibility/capture features must be separately permissioned and pass the same exclusion policy. Privacy should not be reimplemented independently by each subsystem.

## Window model

Implemented now:

1. `pet` — transparent, always-on-top, taskbar-hidden desktop-pet overlay; PixiJS renderer + compact Context Bubble + Companion handle.
2. `companion` — independently show/hide-able managed window for Home / Memory / Activity / Settings. Native close is converted into hide so it can reopen without restarting Pet Runtime.
3. native system tray — open Companion, show/hide Lenvu, explicit quit.

The pet window must remain lightweight when deeper UI is closed.

## Launch-at-login UX

Enabling Windows startup must not turn launch into an interruption. On login, Lenvu should restore the normal desktop-pet/tray presence while the Companion window remains hidden until the user requests it.

Launch-at-login registration is an OS setting. It is not duplicated as a second boolean in SQLite.

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
- muted warm warning tone only for runtime degradation/error or privacy fail-closed warnings;
- avoid constant glow/particles when idle to protect Vega 8 resources;
- use animation-specific low-power frame budgets for rest/sleep states.

Reference board: `docs/assets/lenvu-system-overview.webp`.
Character rules: `docs/CHARACTER_BIBLE.md`.
Runtime asset contract: `docs/ASSET_PIPELINE.md`.
Privacy/screen strategy: `docs/VISION_SYSTEM.md`.
