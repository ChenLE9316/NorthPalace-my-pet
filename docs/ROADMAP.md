# NorthPalace-my-pet Roadmap

## Phase 0 — Foundation

- [x] Initialize private repository.
- [x] Commit first Lenvu architecture/UI concept board.
- [x] Create Tauri + Svelte + Rust scaffold.
- [x] Create minimal offline Pet Brain prototype.
- [x] Add architecture, UI/UX and model-runtime specifications.
- [x] Complete first foundation review.
- [x] Define parallel Pet State V2 types.
- [x] Define Domain Event and Behavior Intent contracts.
- [x] Define vision/screen-understanding policy.
- [x] Replace JavaScript-owned simulation ticking with a Rust-owned runtime clock.
- [x] Replace the V0 single `PetActivity` state with the V2 parallel state model.
- [x] Add explicit runtime health state: ready/degraded/recovering/error.
- [x] Make Svelte consume immutable Pet Runtime snapshots instead of driving simulation time.
- [x] Add Windows idle/return and foreground-app awareness.
- [x] Add Windows CI workflow definition.
- [ ] Confirm first successful Windows CI run.
- [ ] Run first clean build on the target Windows 11 machine.
- [ ] Record baseline RAM / idle CPU / GPU usage on Ryzen 3 2200G + 16 GB.

## Phase 1 — Living desktop pet

- [x] Define the Lenvu Character Bible and canonical visual identity rules.
- [ ] Normalize canonical Lenvu anatomy/scale/anchors from the concept sheets into a production master.
- [x] Define reference-art vs runtime-asset separation.
- [x] Define runtime animation manifest and renderer-facing animation resolver.
- [x] Add PixiJS vector placeholder renderer for state/behavior validation.
- [x] Add animation-specific normal/low-power FPS budgets.
- [ ] Replace placeholder with production sprite/atlas animation graph.
- [x] Split the lightweight `pet` overlay from the independent `companion` window.
- [x] Add current monitor / DPI / work-area display context contract.
- [ ] Add monitor/DPI change observation.
- [x] Define normalized semantic pet hit zones.
- [x] Implement native selective transparent hit testing / click-through with native cursor re-entry.
- [x] Implement desktop movement controller with current work-area clamping.
- [x] Synchronize `walk` / `run` locomotion with native pet-window repositioning.
- [ ] Expose facing direction to Pet State / renderer so production sprites flip consistently.
- [ ] Add autonomous ambient explore/wander behavior selection.
- [ ] Multi-monitor autonomous movement policy.
- [x] Idle/rest/sleep policy can consume Windows idle state through Behavior Intents.
- [x] Hover enter/leave, touch, pet and play event loop.
- [ ] Drag/pick-up interaction.
- [ ] Tray control and startup policy.

## Phase 2 — Persistent life

- [ ] SQLite schema and migrations.
- [ ] Save pet state and relationship.
- [ ] Episodic/semantic/preference/relationship memory types.
- [ ] Time-of-day rhythm persistence.
- [ ] Activity/event journal with bounded retention.

## Phase 3 — Windows awareness and focus companion

- [x] User idle / return sensor using low-cost Win32 input timing.
- [x] Active-window app identity sensor without collecting window titles by default.
- [x] Current monitor / work-area / DPI context.
- [ ] Window bounds context for observed apps.
- [x] Focus Guard domain mode and UI control.
- [ ] Context bubbles and low-noise reminder policy.
- [ ] Per-app privacy exclusions.
- [ ] Screen Context Broker using structured Windows signals first.

## Phase 4 — Local AI brain

- [ ] Isolated LLM worker executable.
- [ ] llama.cpp integration.
- [ ] MiniCPM5-1B GGUF benchmark on R3 2200G + 16 GB.
- [ ] Dynamic context composer.
- [ ] Structured AI intent contract.
- [ ] AI load/unload and memory-pressure policy.

## Phase 5 — Optional visual understanding

- [ ] Define `ScreenObservation` domain contract.
- [ ] On-demand window/region capture with visible privacy indicator.
- [ ] Per-app capture deny list.
- [ ] Optional separately-loadable vision worker.
- [ ] Benchmark vision only if structured Windows/accessibility signals are insufficient.
- [ ] Do not implement continuous screen capture by default.

## Phase 6 — Companion depth

- [ ] Conversation panel.
- [ ] Memory browser/editor.
- [ ] Personality and bond evolution.
- [ ] Long-term behavior adaptation.
- [ ] Optional voice layer.
- [ ] Optional phone companion bridge.

## Before public release

- [ ] Choose a code license.
- [ ] Define separate Lenvu character/artwork licensing.
- [ ] Add dependency lockfiles.
- [x] Add Windows CI workflow definition.
- [ ] Execute and pass the first Windows CI run.
- [ ] Add SECURITY.md.
- [ ] Restore a restrictive Content Security Policy.
- [ ] Replace the low-resolution concept-board copy with the original high-resolution source.
- [ ] Verify no model weights, local databases, secrets, logs or private user data are tracked.

## Definition of success

Unload MiniCPM5-1B completely. Lenvu should still feel alive and useful: movement, sleep/wake, petting, attention, focus behavior, persistence and ordinary desktop interaction must continue without an LLM or a vision model.
