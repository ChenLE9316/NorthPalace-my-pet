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
- bounded accessibility metadata only when explicitly allowed.

These signals let Lenvu feel aware without continuously capturing the screen.

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
      ├─ local hour
      └─ bounded focused-control accessibility metadata [opt-in]
      ↓
on-demand ScreenContextSnapshot
```

The broker contains **no pixels, screenshots, OCR output, window-title history or accessibility text history**. It is a current-context boundary, not a surveillance log.

Per-app exclusions are user-managed, local and fail-closed. An excluded active app is filtered before its identity, window geometry or accessibility metadata crosses into the broker/Domain Event path.

## Bounded accessibility collector V1

The Windows accessibility worker is implemented with Windows UI Automation and runs at a deliberately low two-second cadence. It remains completely inactive at the UI Automation/COM layer while `accessibilityContextEnabled` is off.

When explicitly enabled and the current app passes the exclusion gate, V1 reads only the currently focused element and verifies that the element belongs to the current foreground process. The broker stores only:

- numeric control-type ID;
- enabled flag;
- keyboard-focusable flag;
- keyboard-focus flag;
- offscreen flag;
- password-element flag;
- bounded element geometry when available.

V1 explicitly does **not** read:

- `Name`;
- `Value`;
- `HelpText`;
- window title;
- raw text;
- accessibility-tree descendants;
- arbitrary UI Automation properties;
- event history.

The result is ephemeral in-memory context and is not automatically written to long-term memory. App changes invalidate old accessibility context, and late results for a previous app are ignored. A per-app exclusion always overrides the global accessibility capability.

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
        -> bounded focused-control accessibility metadata implemented, opt-in

Level 2: on-demand screenshot/region understanding
        -> optional vision worker
        -> not implemented

Level 3: continuous visual perception
        -> explicitly out of scope
```

## When a vision model becomes useful

Add optional vision only for tasks that cannot be answered reliably by structured signals, for example:

- understanding arbitrary visual content in an application;
- describing an image the user explicitly points at;
- recognizing a visual error/dialog when structured accessibility data is insufficient;
- screen-context assistance requested explicitly by the user;
- future computer-use capabilities with separate permissions.

## Privacy contract

Vision must be opt-in and event/on-demand driven.

The following prerequisites are now in place or remain intentionally deferred:

- [x] fail-closed privacy policy service;
- [x] user-managed per-app deny list;
- [x] deny filtering before structured active-app context crosses the privacy boundary;
- [x] Screen Context Broker with no screenshot persistence;
- [x] privacy-gated visible active-window bounds;
- [x] explicit accessibility-context capability, off by default;
- [x] Settings UI showing current structured-context/privacy state;
- [x] bounded accessibility metadata collector;
- [ ] visible indicator while visual capture is active;
- [ ] region/window capture instead of whole desktop where possible;
- [ ] explicit visual-capture capability/permission setting;
- [ ] no screenshot persistence unless explicitly requested;
- [ ] bounded capture frequency;
- [ ] optional vision-worker lifecycle and immediate unload policy.

The same app exclusion service must be reused for future capture. A subsystem must not invent a second independent privacy list.

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
        |      `--> bounded accessibility metadata
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

Keep screenshot/vision work deferred. Structured Windows + bounded accessibility context is now sufficient to begin using current context in behavior and future AI orchestration without adding an always-resident visual model.
