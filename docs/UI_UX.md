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

### Level 3 — Context Bubble

The Context Bubble V1 is a low-noise presentation policy rather than a permanent state label. Meaningful interactions or system/focus transitions can produce short cues; ordinary ambient stay/observe/sit/explore behavior remains silent.

The bubble is pointer-transparent and its presentation timer never owns Pet Brain simulation time.

### Level 4 — Companion window

The Companion is a deeper view of Lenvu, not the creature itself. It is a separate Tauri window from the transparent pet overlay.

```text
Companion
├─ Home
├─ Memory
├─ Activity
└─ Settings
```

Memory, Activity and deeper Settings I/O are lazy/on-demand where practical.

#### Home

Home keeps ordinary interaction lightweight: pet/play/Focus Guard actions, life-state summary, posture/attention/emotion/cognition and renderer/runtime diagnostics.

#### Memory

Memory provides local long-term memory CRUD/search/provenance without requiring MiniCPM5-1B.

#### Activity

Activity shows bounded meaningful relationship/focus events. It is not foreground-app, cursor or screen history.

#### Settings

Current Settings controls include:

1. **Windows 開機啟動** — opt-in, OS registration is source of truth.
2. **App Privacy Exclusions** — local fail-closed deny list applied before context crosses the sensor boundary.
3. **Accessibility context** — off by default; when enabled, initializes the bounded Windows UI Automation collector only for non-excluded apps.
4. **Structured Screen Context status** — while Settings is mounted, shows the current broker state rather than building a history.

Accessibility UI is intentionally explicit about what is and is not collected. V1 may display only structural focused-control facts: control-type ID, enabled/focusable/focused/offscreen/password flags and geometry. It never displays element names, values, help text or an accessibility-tree dump because V1 never reads them.

### Level 5 — Deep management

Model/runtime, performance, display, animation and debug controls can grow here or later move to a dedicated managed Settings window without changing Rust services.

## Privacy-aware awareness UX

```text
Windows sensors
    ↓
PrivacyPolicyService
    ├─ excluded/fail-closed → identity, bounds, accessibility discarded
    └─ allowed
          ↓
ScreenContextBroker
    ├─ active app identity
    ├─ visible window bounds
    ├─ idle + local hour
    └─ focused-control structural metadata [explicit opt-in]
```

The accessibility worker runs at low frequency and does not initialize UI Automation while permission is off. It validates the focused element against the current foreground process. Switching apps invalidates previous accessibility data; stale results for a previous app are ignored.

No structured-context surface creates foreground-app or accessibility history by default.

## Window model

1. `pet` — transparent, always-on-top, taskbar-hidden Lenvu overlay.
2. `companion` — Home / Memory / Activity / Settings, independently show/hide-able.
3. native system tray — open Companion, show/hide Lenvu, explicit quit.

## Launch-at-login UX

Enabling Windows startup restores the normal pet/tray presence after login; it must not automatically pop the Companion window in front of the user.

## Desktop movement UX

`walk`/`run` can move the native pet window inside the current monitor work area, with edge reversal and DPI-aware speed. Autonomous cross-monitor travel remains a separate policy item.

## Visual language

- deep blue/near-black surfaces;
- cyan Lumen-Code highlights;
- restrained violet emotional/AI accents;
- gold only for Lenvu's horn-ring identity details;
- restrained effects and low-power FPS budgets for the Vega 8 target;
- warm warning accents only for meaningful runtime/privacy degradation.

Reference board: `docs/assets/lenvu-system-overview.webp`.
Character rules: `docs/CHARACTER_BIBLE.md`.
Runtime asset contract: `docs/ASSET_PIPELINE.md`.
Privacy/screen strategy: `docs/VISION_SYSTEM.md`.
