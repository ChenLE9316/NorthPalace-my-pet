# Vision / Screen Understanding Strategy

## Decision

A dedicated vision model is **not required for the first living-desktop-pet releases**.

NorthPalace-my-pet should first obtain useful context from low-cost deterministic Windows signals:

- active window/app identity;
- window bounds and monitor layout;
- user idle/return state;
- cursor position and pet hit-testing;
- focus session state;
- notification metadata where permission allows;
- time of day, power/session state and explicit user actions.

These signals are enough for Lenvu to feel aware without continuously capturing the screen.

## Why vision is deferred

The target Ryzen 3 2200G + 16 GB DRAM machine has a strict all-day resource budget. A continuously resident image-language model would add memory pressure, CPU/iGPU contention, latency and privacy complexity while duplicating information often available from Windows APIs.

The project should therefore use this hierarchy:

```text
Level 0: Windows/system events
        -> cheap, continuous

Level 1: structured app/context adapters
        -> window title, accessibility/UI metadata when explicitly enabled

Level 2: on-demand screenshot/region understanding
        -> optional vision worker

Level 3: continuous visual perception
        -> explicitly out of scope unless future measurements justify it
```

## When a vision model becomes useful

Add optional vision only for tasks that cannot be answered reliably by structured signals, for example:

- understanding arbitrary visual content in an application;
- describing an image the user points at;
- recognizing a visual error/dialog when no structured accessibility data is available;
- screen-context assistance requested explicitly by the user;
- future computer-use capabilities.

## Privacy contract

Vision must be opt-in and event/on-demand driven.

Required controls before implementation:

- visible indicator while screen capture is active;
- per-app deny list and protected/sensitive app policy;
- region/window capture instead of whole-desktop capture when possible;
- no background screenshot history by default;
- no screenshot persistence unless explicitly requested;
- local inference by default;
- bounded capture frequency;
- immediate worker unload when unused on constrained hardware.

## Architecture

Vision must not be embedded inside Pet Brain.

```text
Windows / User Request
        |
        v
Screen Context Broker
        |
        +--> structured Windows context
        |
        `--> optional Vision Worker
                  |
                  v
          validated ScreenObservation
                  |
                  v
              Event Bus
                  |
                  v
              Pet Brain / AI Orchestrator
```

Pet Brain receives normalized observations, never raw screenshots.

## Worker boundary

If vision is added later, keep it in a separately loadable process just like the text LLM worker. It must have lifecycle states such as `unloaded`, `loading`, `ready`, `busy`, `sleeping`, `error`.

This lets NorthPalace-my-pet run with:

- no AI workers;
- text LLM only;
- vision only for a short request;
- text + vision temporarily when a multimodal task genuinely requires both.

## Current recommendation

For V0.2 through the first usable desktop-pet milestone: **do not add a vision model**. Build Windows Awareness + Pet Brain + renderer first. Design an interface for future `ScreenObservation` events now, but leave the expensive worker optional and unloaded.
