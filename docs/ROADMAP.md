# NorthPalace-my-pet Roadmap

This roadmap tracks the current implementation baseline. A checked item means the capability exists in the repository; it does **not** automatically mean the latest `main` commit has been observed passing Windows CI or has been performance-validated on the Ryzen 3 2200G target machine.

## Phase 0 — Foundation

- [x] Initialize repository and Tauri + Svelte + Rust scaffold.
- [x] Keep the original Lenvu system-overview board as the product north star.
- [x] Establish Pet-first / AI-second acceptance rule.
- [x] Rust-owned monotonic Pet Runtime clock.
- [x] Domain Event channel and immutable runtime snapshots.
- [x] Parallel Pet State V2.
- [x] Behavior Intent priority / TTL / interruption model.
- [x] Explicit runtime-health contract.
- [x] Svelte consumes snapshots instead of driving simulation time.
- [x] Windows CI workflow on `main`.
- [x] Add Rust formatting and Clippy gates to Windows CI.
- [ ] Add dependency lockfiles and switch CI to locked/reproducible installs.
- [ ] Run first clean executable/bundle build on the target Windows 11 / R3 2200G machine.
- [ ] Record baseline RAM / idle CPU / GPU usage on Ryzen 3 2200G + 16 GB + Vega 8.

## Phase 1 — Living desktop pet

### Character / visual ground truth

- [x] Define Lenvu Character Bible.
- [x] Freeze original-image visual ground truth and authority order.
- [x] Preserve original-source checksum, dimensions and provenance metadata.
- [x] Measure required front/profile/back landmarks from the original high-resolution source.
- [x] Remap measured source pixels into canonical coordinates with aspect-preserving root/ground alignment.
- [x] Correct canonical safe bounds/pivots so the long canine profile is not compressed.
- [x] Add asset validator rules that recompute canonical landmarks from source measurements.
- [x] Add source-faithful candidate promotion gate.
- [x] Freeze current visual contracts; do not expand policy/measurement layers without concrete source evidence.
- [ ] Replace the transport-optimized anatomy preview with a high-quality source-faithful derivative.
- [ ] Replace the low-resolution README concept-board copy with a high-resolution source-faithful derivative.
- [ ] Produce the canonical Lenvu production master artwork.
- [ ] Import additional selected original reference sheets with provenance records.

### Renderer / animation

- [x] Separate reference art from runtime assets.
- [x] Keep one authoritative runtime manifest: `src/lib/pet/lenvu.manifest.json`.
- [x] Remove the conflicting second hand-maintained public runtime manifest.
- [x] Define semantic animation resolver and normal/low-power FPS budgets.
- [x] Add PixiJS procedural placeholder renderer.
- [x] Enforce asymmetric identity semantics instead of blind whole-root horizontal mirroring.
- [x] Keep heterochromia and left-side crescent identity semantically correct when facing changes.
- [ ] Replace placeholder with production sprite/atlas animation graph.
- [ ] Author production Idle / Walk / Sit / Lie / Sleep / Wake assets.
- [ ] Add production hit masks/atlas metadata derived from the final assets.

### Desktop behavior / window interaction

- [x] Split transparent `pet` overlay from independent `companion` window.
- [x] Current monitor / DPI / work-area context.
- [x] Window-moved / scale-factor observation and debounced hit-region refresh.
- [x] Semantic pet hit zones.
- [x] Native selective transparent click-through.
- [x] Facing-aware hit-region remapping.
- [x] Native work-area movement controller.
- [x] Synchronize `walk` / `run` locomotion with native window movement.
- [x] Seed motion direction from Pet State `facing` rather than forcing startup Right.
- [x] Weighted deterministic ambient personality/curiosity behavior selection.
- [x] Safe horizontal multi-monitor Explore across genuinely adjacent displays.
- [x] Reject disconnected/vertical monitor teleporting.
- [x] Hover, touch, pet, play interaction loop.
- [x] Drag / pick-up / held / drop behavior.
- [x] Remove double-click-to-open overlap so petting and Companion opening are separate gestures.
- [x] System tray controls.
- [x] Opt-in Windows launch-at-login + Settings control.

## Phase 2 — Persistent life

- [x] Bundled SQLite through `rusqlite`.
- [x] WAL / busy timeout / schema versioning / migrations.
- [x] Persist facing, energy, curiosity, bond and sleep pressure.
- [x] Restore long-lived values into fresh transient runtime defaults.
- [x] DB-owning persistence worker.
- [x] Changed-only 30-second autosave.
- [x] Graceful final save with bounded acknowledgement.
- [x] Session-only fallback when persistence cannot initialize.
- [x] Bounded meaningful-event journal: 30 days / 2,000 rows.
- [x] Relationship-event history.
- [x] Typed episodic / semantic / preference / relationship memory.
- [x] FTS5 + BM25 / importance / recency retrieval.
- [x] Hourly interaction rhythm.
- [x] Memory Browser/editor V1.
- [x] Manual memory create/search/list/update/delete.
- [x] Memory source-event provenance and Activity view.
- [ ] Consolidate duplicated memory-domain types before Memory Evaluator growth.
- [ ] Add Memory Evaluator: store / merge / discard automatic candidates.
- [ ] Add stronger relationship/personality evolution from history.
- [ ] Add user-facing export/reset/backup policy for local data.

## Phase 3 — Windows awareness and Focus Companion

- [x] Low-cost user idle / return sensor.
- [x] Local time sensor independent of WebView JavaScript clocks.
- [x] Foreground executable identity without window titles by default.
- [x] Privacy-gated visible DWM frame bounds.
- [x] Fail-closed per-app privacy exclusions before Domain Event / Screen Context exposure.
- [x] Local `privacy-rules.json` deny list with no recent-app inventory.
- [x] Flushed temporary-write + Windows replace-existing/write-through privacy-rule replacement.
- [x] Structured Screen Context Broker for active app, bounds, idle and local hour.
- [x] Opt-in bounded Windows UI Automation metadata.
- [x] Keep accessibility context free of Name, Value, HelpText, raw text and tree dumps.
- [x] Drop UI Automation resources when the capability is disabled.
- [x] Retry UI Automation initialization after transient failure.
- [x] Expose privacy/accessibility state in Settings.
- [x] Focus Guard mode and low-noise context bubbles.
- [ ] Add observation freshness/timestamps before AI depends on structured context.

## Phase 4 — Consolidation before AI

- [x] Rewrite README and Architecture to match current code instead of older milestone descriptions.
- [x] Document one runtime-manifest source of truth.
- [x] Mark the original Foundation Review as historical rather than current architecture.
- [ ] Split large `CompanionView.svelte` into Home / Memory / Activity / Settings section components.
- [ ] Split `src-tauri/src/lib.rs` composition root into commands/setup/shutdown modules before more services are added.
- [ ] Introduce common background-worker lifecycle/supervision: cancel, shutdown, join/restart and health reporting.
- [ ] Complete or simplify the `ready / degraded / recovering / error` runtime-health state machine.
- [ ] Replace frontend polling growth with a bounded shared subscription/event strategy before adding many more live surfaces.
- [ ] Add dependency lockfiles.
- [ ] Add full Svelte-specific diagnostic/typecheck gate.
- [ ] Add a release/manual Windows `tauri build` CI path.

## Phase 5 — Local AI brain

Do not start this phase until the consolidation/reproducibility and target-machine baseline above are in acceptable shape.

- [ ] Isolated LLM worker executable.
- [ ] llama.cpp integration.
- [ ] MiniCPM5-1B GGUF benchmark on R3 2200G + 16 GB.
- [ ] Load/unload and memory-pressure policy.
- [ ] Dynamic context composer.
- [ ] Privacy-approved fresh Screen Context + relevant memory composition.
- [ ] Structured AI intent contract; Pet Brain remains authoritative for ordinary life.
- [ ] Connect Memory Evaluator/retrieval without making Pet Brain depend on the LLM.

## Phase 6 — Optional visual understanding

- [ ] Define normalized `ScreenObservation` contract.
- [ ] On-demand window/region capture with visible privacy indicator.
- [ ] Reuse per-app deny policy for capture.
- [ ] No screenshot history by default.
- [ ] Separate unloadable vision worker.
- [ ] Benchmark only if structured Windows/accessibility signals are insufficient.
- [ ] Do not implement continuous capture by default.

## Phase 7 — Companion depth

- [x] Home / Memory / Activity / Settings surfaces.
- [x] Startup control.
- [x] Per-app privacy exclusions.
- [x] Accessibility-context capability control/status.
- [ ] Conversation panel.
- [ ] Personality and bond evolution.
- [ ] Long-term adaptive behavior.
- [ ] Optional voice layer.
- [ ] Optional phone companion bridge.

## Before public release

- [ ] Choose code license.
- [ ] Define separate Lenvu character/artwork licensing.
- [ ] Add `package-lock.json` and `Cargo.lock` from a real resolved dependency graph.
- [ ] Add `SECURITY.md`.
- [ ] Restore a restrictive Content Security Policy.
- [ ] Replace low-resolution concept/reference derivatives.
- [ ] Replace temporary Windows placeholder icon with canonical Lenvu application icon.
- [ ] Produce production character assets.
- [ ] Run clean Windows executable/bundle build.
- [ ] Measure target-hardware RAM/CPU/GPU baseline.
- [ ] Verify no model weights, local databases, secrets, logs or private user data are tracked.
- [ ] Verify local-data export/reset/backup behavior.

## Definition of success

Unload MiniCPM5-1B and any future vision worker completely. Lenvu still feels alive and useful: movement, sleep/wake, petting, attention, focus behavior, persistence, inspectable memory and ordinary desktop interaction continue without AI.
