# Vision / Screen Understanding Strategy

## Decision

A dedicated vision model is **not required for the first living-desktop-pet releases**.

NorthPalace-my-pet first obtains useful context from low-cost deterministic Windows signals:

- active app identity;
- privacy-approved visible active-window bounds;
- user idle/return state;
- local hour;
- monitor / DPI / work-area context;
- cursor and semantic pet hit-testing;
- focus session state;
- future bounded accessibility metadata only when explicitly allowed.

These signals are enough for Lenvu to feel aware without continuously capturing the screen.

## Current implementation

The structured path is implemented through a fail-closed `PrivacyPolicyService` and an in-memory `ScreenContextBroker`.

```text
Windows signals
      ↓
PrivacyPolicyService
      ↓
ScreenContextBroker
      ├─ active app identity or privacy-blocked state
      ├─ visible active-window bounds when allowed
      ├─ user idle milliseconds
      └─ local hour
      ↓
on-demand ScreenContextSnapshot
```

The current broker contains **no pixels, screenshots, OCR output or window-title history**. It is an in-memory current-context boundary, not a surveillance log.

Per-app exclusions are implemented. The deny list is user-managed, local and fail-closed. Excluded active apps are filtered before their identity or window geometry reaches the broker/Domain Event path.

An explicit `accessibilityContextEnabled` privacy capability is also implemented and exposed in Settings. It is **off by default** and uses the same per-app exclusion service. The capability currently grants permission only; the bounded Windows accessibility collector is still intentionally unimplemented.

## Why vision is deferred

The target Ryzen 3 2200G + 16 GB DRAM machine has a strict all-day resource budget. A continuously resident image-language model would add memory pressure, CPU/iGPU contention, latency and privacy complexity while duplicating information often available from Windows APIs.

The project therefore uses this hierarchy:

```text
Level 0: Windows/system events
        -> cheap, continuous
        -> implemented core

Level 1: structured app/context adapters
        -> brokered + privacy-gated
        -> app identity / bounds / idle / time implemented
        -> accessibility capability implemented
        -> bounded accessibility collector next

Level 2: on-demand screenshot/region understanding
        -> optional vision worker
        -> not implemented

Level 3: continuous visual perception
        -> explicitly out of scope
```

## Bounded accessibility contract

The future accessibility collector must remain deliberately narrow. The approved capability does **not** grant a license to dump an accessibility tree.

Allowed first-step metadata should be bounded and structural, for example:

- focused element control type;
- whether the focused element is enabled/focusable/focused;
- whether it is offscreen;
- whether it is a password element;
- bounded geometry if useful.

The first collector should explicitly avoid:

- element names or window titles by default;
- raw text/value content;
- help text;
- full accessibility-tree enumeration;
- continuous event history;
- persisting accessibility metadata into long-term memory automatically.

A per-app exclusion must override the global accessibility capability.

## When a vision model becomes useful

Add optional vision only for tasks that cannot be answered reliably by structured signals, for example:

- understanding arbitrary visual content in an application;
- describing an image the user explicitly points at;
- recognizing a visual error/dialog when no structured accessibility data is available;
- screen-context assistance requested explicitly by the user;
- future computer-use capabilities with separate permissions.

## Privacy contract

Vision must be opt-in and event/on-demand driven.

The following prerequisites are now partially in place:

- [x] fail-closed privacy policy service;
- [x] user-managed per-app deny list;
- [x] deny filtering before structured active-app context crosses the privacy boundary;
- [x] Screen Context Broker with no screenshot persistence;
- [x] privacy-gated visible active-window bounds;
- [x] explicit accessibility-context capability, off by default;
- [x] Settings UI showing current structured-context/privacy state;
- [ ] bounded accessibility metadata collector;
- [ ] visible indicator while capture is active;
- [ ] region/window capture instead of whole desktop where possible;
- [ ] explicit capture capability/permission setting;
- [ ] no screenshot persistence unless explicitly requested;
- [ ] bounded capture frequency;
- [ ] optional vision-worker lifecycle and immediate unload policy.

The same app exclusion service must be reused for future accessibility metadata and capture. A subsystem must not invent a second independent privacy list.

## Future architecture

```text
Windows / explicit user request
        |
        v
PrivacyPolicyService
        |
        v
Screen Context Broker
        |
        +--> structured Windows context
        |      `--> bounded accessibility metadata [next]
        |
        `--> optional on-demand capture
                  |
                  v
           Vision Worker [later]
                  |
                  v
          validated ScreenObservation
                  |
                  v
             AI Orchestrator
```

Pet Brain should receive semantic facts/actions, never raw screenshots.

## `ScreenObservation` future contract

A later visual observation should be bounded and explainable, for example:

```text
ScreenObservation
├─ source
│  ├─ app id
│  ├─ window / region
│  └─ requested-at timestamp
├─ privacy-approved capability
├─ summary / detected UI facts
├─ confidence
├─ expires-after
└─ no raw image persistence by default
```

The raw frame is an ephemeral worker input, not long-term pet memory.

## Worker boundary

If vision is added later, keep it in a separately loadable process just like the text LLM worker. It must have lifecycle states such as `unloaded`, `loading`, `ready`, `busy`, `sleeping`, `error`.

This lets NorthPalace-my-pet run with:

- no AI workers;
- text LLM only;
- vision only for a short explicit request;
- text + vision temporarily when a multimodal task genuinely requires both.

## Current recommendation

Implement the **bounded accessibility metadata collector** behind the already-approved capability and per-app deny gate. Do not collect element names/raw text in the first version. Screenshot capture should remain unimplemented until there is a clear user-facing capability, separate permission UI and visible capture indicator.
