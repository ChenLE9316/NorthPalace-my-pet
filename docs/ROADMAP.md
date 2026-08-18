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

- [x] Define the Lenvu Character Bible.
- [x] Freeze original-image visual ground truth and authority order so generated candidates cannot redefine Lenvu.
- [x] Import the first provenance-tracked high-resolution-canvas anatomy reference preview and preserve original-source checksum/resolution metadata.
- [x] Verify the original 1536×1024 anatomy source dimensions, exact byte length and SHA-256.
- [x] Freeze the Canonical Lenvu production coordinate/identity contract while keeping normalization coordinates subordinate to source evidence.
- [x] Establish provisional front/profile/back normalization landmarks and QA tolerances with explicit source-image precedence.
- [x] Measure front/profile/back production landmarks directly from the original high-resolution anatomy source.
- [x] Remap measured source pixels into canonical 1024×1024 coordinates using aspect-preserving ground/root alignment.
- [x] Revise canonical safe bounds and source-derived pivots so the long canine profile is not compressed by earlier inferred/chibi-biased coordinates.
- [x] Add CI rules that recompute canonical landmarks from source measurements and reject drift.
- [x] Add a candidate-promotion gate that rejects source-divergent/chibi/cat-like master artwork.
- [x] Require future master candidates to match measured source landmarks as well as visual identity.
- [ ] Replace the transport-optimized anatomy preview with a high-quality source-faithful repository derivative.
- [ ] Replace the low-resolution README concept-board copy with the original high-resolution source-faithful derivative.
- [ ] Normalize canonical Lenvu anatomy/scale/anchors from the original sheets into a production master artwork.
- [x] Define reference-art vs runtime-asset separation.
- [x] Define runtime animation manifest and renderer-facing animation resolver.
- [x] Add PixiJS vector placeholder renderer for state/behavior validation.
- [x] Add animation-specific normal/low-power FPS budgets.
- [ ] Replace placeholder with production sprite/atlas animation graph.
- [x] Split the lightweight `pet` overlay from the independent `companion` window.
- [x] Add current monitor / DPI / work-area display context contract.
- [x] Add window-moved / scale-factor observation and debounced display-context refresh.
- [x] Define normalized semantic pet hit zones.
- [x] Implement native selective transparent hit testing / click-through with native cursor re-entry.
- [x] Mirror semantic/native hit regions when Lenvu changes facing direction.
- [x] Implement desktop movement controller with current work-area clamping.
- [x] Synchronize `walk` / `run` locomotion with native pet-window repositioning.
- [x] Expose facing direction through Pet State and renderer.
- [x] Add deterministic offline ambient explore/wander behavior selection.
- [x] Replace deterministic explore timing with weighted personality/curiosity behavior selection.
- [x] Add safe autonomous multi-monitor exploration across genuinely adjacent horizontal displays.
- [x] Keep non-Explore interactions on the current display and reject disconnected/vertical monitor teleporting.
- [x] Idle/rest/sleep policy can consume Windows idle state through Behavior Intents.
- [x] Hover enter/leave, touch, pet and play event loop.
- [x] Add drag/pick-up gesture with a domain-level `held` posture and native pet-window dragging.
- [x] Add native system-tray controls for Companion, pet visibility and quit.
- [x] Add opt-in Windows launch-at-login policy with OS registration as the source of truth.
- [x] Add lazy Settings UI for Windows startup state/control.

## Phase 2 — Persistent life

- [x] Add bundled SQLite persistence without requiring a system SQLite install.
- [x] Add schema versioning/migration with `PRAGMA user_version`.
- [x] Store core long-lived pet state: facing, energy, curiosity, bond and sleep pressure.
- [x] Load long-lived values into fresh transient Pet State defaults on startup.
- [x] Move continuous database writes to a separate persistence worker.
- [x] Add changed-only 30-second autosave.
- [x] Add graceful-shutdown final save with bounded acknowledgement wait.
- [x] Fall back to session-only state if local data / SQLite initialization fails.
- [x] Add an always-present no-op-capable `PersistenceService` boundary.
- [x] Add runtime Domain Event subscription for low-frequency persistence observers.
- [x] Add bounded activity/event journal with 30-day / 2,000-row retention.
- [x] Add relationship-event history beyond the current `bond` scalar.
- [x] Add typed episodic / semantic / preference / relationship memory storage.
- [x] Add FTS5 memory index and first BM25/importance/recency retrieval path.
- [x] Add Windows local-hour sensor independent of the WebView.
- [x] Add lightweight hourly interaction-rhythm persistence.
- [x] Add Memory Browser V1 in the Companion window.
- [x] Add explicit manual memory create/search/list/update/delete APIs.
- [x] Add editable memory kind/content/importance controls without requiring the LLM.
- [x] Show source-event provenance and activity history in the Memory Browser.
- [ ] Add Memory Evaluator deciding store/merge/discard for automatic long-term candidates.
- [ ] Add stronger long-term relationship/personality evolution using history.

## Phase 3 — Windows awareness and focus companion

- [x] User idle / return sensor using low-cost Win32 input timing.
- [x] Active-window app identity sensor without collecting window titles by default.
- [x] Current monitor / work-area / DPI context.
- [x] Observe pet-window movement and DPI scale-factor changes.
- [x] Local time-of-day sensor using Win32 rather than a JavaScript clock.
- [x] Add privacy-gated visible active-window bounds using DWM frame geometry.
- [x] Focus Guard domain mode and UI control.
- [x] Context bubbles and low-noise reminder policy.
- [x] Add fail-closed per-app privacy exclusions before active-app identity reaches Domain Events.
- [x] Add user-managed local `privacy-rules.json` deny list without recent-app history.
- [x] Add structured Screen Context Broker V1 for active app / bounds / idle / local hour, with no pixels or screenshots.
- [x] Define opt-in structured accessibility-context capability behind the same privacy gate.
- [x] Expose the accessibility capability and current structured-context status in Settings.
- [x] Implement bounded accessibility metadata with a low-frequency, privacy-gated Windows UI Automation worker.
- [x] Keep accessibility context free of element names, values, help text, window titles and tree enumeration.

## Phase 4 — Local AI brain

- [ ] Isolated LLM worker executable.
- [ ] llama.cpp integration.
- [ ] MiniCPM5-1B GGUF benchmark on R3 2200G + 16 GB.
- [ ] Dynamic context composer.
- [ ] Structured AI intent contract.
- [ ] Connect Memory Evaluator/retrieval without making Pet Brain depend on the LLM.
- [ ] AI load/unload and memory-pressure policy.

## Phase 5 — Optional visual understanding

- [ ] Define `ScreenObservation` domain contract.
- [ ] On-demand window/region capture with visible privacy indicator.
- [ ] Reuse per-app privacy exclusions for capture deny policy.
- [ ] Optional separately-loadable vision worker.
- [ ] Benchmark vision only if structured Windows/accessibility signals are insufficient.
- [ ] Do not implement continuous screen capture by default.

## Phase 6 — Companion depth

- [x] Split Companion V2 into lazy-loaded Home / Memory / Activity / Settings tabs.
- [x] Add Windows startup control in Settings.
- [x] Add per-app privacy exclusion management in Settings.
- [x] Add explicit accessibility-context capability control and structured Screen Context status in Settings.
- [ ] Conversation panel.
- [x] Memory browser/editor V1.
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
- [ ] Replace the low-resolution concept-board copy with the original high-resolution source-faithful derivative.
- [ ] Import additional selected original Lenvu reference sheets with provenance/resolution records.
- [ ] Replace temporary Windows placeholder icon with canonical Lenvu application icon.
- [ ] Verify no model weights, local databases, secrets, logs or private user data are tracked.

## Definition of success

Unload MiniCPM5-1B completely. Lenvu should still feel alive and useful: movement, sleep/wake, petting, attention, focus behavior, persistence, inspectable memory and ordinary desktop interaction must continue without an LLM or a vision model.
