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
- [x] Pass complete Windows CI: Svelte/PixiJS build + Rust/Tauri tests.
- [ ] Run first clean executable build on the target Windows 11 / R3 2200G machine.
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
- [x] Mirror semantic/native hit regions when Lenvu changes facing direction.
- [x] Implement desktop movement controller with current work-area clamping.
- [x] Synchronize `walk` / `run` locomotion with native pet-window repositioning.
- [x] Expose facing direction through Pet State and renderer.
- [x] Add deterministic offline ambient explore/wander behavior selection.
- [ ] Replace deterministic explore timing with weighted personality/curiosity behavior selection.
- [ ] Multi-monitor autonomous movement policy.
- [x] Idle/rest/sleep policy can consume Windows idle state through Behavior Intents.
- [x] Hover enter/leave, touch, pet and play event loop.
- [ ] Drag/pick-up interaction.
- [ ] Tray control and startup policy.

## Phase 2 — Persistent life

- [x] Add bundled SQLite persistence without requiring a system SQLite install.
- [x] Add schema versioning/migration with `PRAGMA user_version`.
- [x] Store core long-lived pet state: facing, energy, curiosity, bond and sleep pressure.
- [x] Load long-lived values into fresh transient Pet State defaults on startup.
- [x] Move database writes to a separate persistence worker.
- [x] Add changed-only 30-second autosave.
- [x] Fall back to session-only state if local data / SQLite initialization fails.
- [x] Pass Windows CI with SQLite migration/state round-trip tests.
- [ ] Add graceful-shutdown final save.
- [ ] Add relationship-event history beyond the current `bond` scalar.
- [ ] Add episodic / semantic / preference / relationship memory tables.
- [ ] Add FTS5 indexes and memory retrieval policy.
- [ ] Add time-of-day rhythm persistence where it represents long-lived state.
- [ ] Add bounded activity/event journal with retention policy.

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
- [x] Add and pass Windows CI checks.
- [x] Keep only the latest Windows CI run per branch to avoid stale runner queues.
- [ ] Add SECURITY.md.
- [ ] Restore a restrictive Content Security Policy.
- [ ] Replace the low-resolution concept-board copy with the original high-resolution source.
- [ ] Import selected original Lenvu reference sheets with provenance/resolution records.
- [ ] Replace temporary Windows placeholder icon with canonical Lenvu application icon.
- [ ] Verify no model weights, local databases, secrets, logs or private user data are tracked.

## Definition of success

Unload MiniCPM5-1B completely. Lenvu should still feel alive and useful: movement, sleep/wake, petting, attention, focus behavior, persistence and ordinary desktop interaction must continue without an LLM or a vision model.
