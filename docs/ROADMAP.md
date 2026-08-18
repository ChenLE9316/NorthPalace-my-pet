# NorthPalace-my-pet Roadmap

A checked item means the capability exists in the repository; it does not automatically mean the latest `main` commit has been observed passing Windows CI or performance-validated on the Ryzen 3 2200G target.

## Phase 0 — Foundation

- [x] Tauri + Svelte + Rust scaffold.
- [x] Original Lenvu system-overview board preserved as product north star.
- [x] Pet-first / AI-second acceptance rule.
- [x] Rust-owned monotonic Pet Runtime.
- [x] Domain Event channel and immutable snapshots.
- [x] Parallel Pet State V2.
- [x] Behavior Intent priority / TTL / interruption model.
- [x] Runtime health simplified to truthful `ready / degraded / error` semantics.
- [x] Runtime panic preserves the last published snapshot and marks it `error`.
- [x] Svelte consumes snapshots instead of driving simulation time.
- [x] Windows CI workflow on `main`.
- [x] Rust formatting and Clippy CI gates.
- [ ] Dependency lockfiles and locked/reproducible installs.
- [ ] First clean executable/bundle build on target Windows machine.
- [ ] Baseline RAM / idle CPU / GPU on Ryzen 3 2200G + 16 GB + Vega 8.

## Phase 1 — Living desktop pet

### Character / visual ground truth

- [x] Character Bible and original-image visual authority.
- [x] Original-source checksum/dimension/provenance metadata.
- [x] Source-measured front/profile/back landmarks.
- [x] Aspect-preserving source → canonical remap.
- [x] Canonical bounds/pivots corrected for long canine anatomy.
- [x] Validator recomputes canonical landmarks from source measurements.
- [x] Source-faithful candidate promotion gate.
- [x] Freeze current visual governance; next work should produce assets.
- [ ] High-quality source-faithful anatomy derivative.
- [ ] High-resolution README concept-board derivative.
- [ ] Canonical Lenvu production master artwork.
- [ ] Additional selected original reference sheets with provenance.

### Renderer / animation

- [x] Reference/runtime asset separation.
- [x] One authoritative runtime manifest: `src/lib/pet/lenvu.manifest.json`.
- [x] Conflicting second public runtime manifest removed.
- [x] Semantic animation resolver and per-animation FPS budgets.
- [x] PixiJS procedural placeholder renderer.
- [x] No blind whole-root mirror for asymmetric identity.
- [ ] Production sprite/atlas animation graph.
- [ ] Production Idle / Walk / Sit / Lie / Sleep / Wake assets.
- [ ] Production hit masks/atlas metadata.

### Desktop behavior / window interaction

- [x] Separate `pet` and `companion` windows.
- [x] Monitor / DPI / work-area context.
- [x] Window-moved / scale-factor observation.
- [x] Semantic hit zones + native selective click-through.
- [x] Facing-aware hit regions.
- [x] Native work-area movement.
- [x] Motion direction seeded from Pet State `facing`.
- [x] Weighted deterministic ambient personality selection.
- [x] Safe horizontal multi-monitor Explore only.
- [x] Hover / touch / pet / play.
- [x] Drag / pick-up / held / drop.
- [x] Petting and Companion opening separated; no double-click overlap.
- [x] System tray.
- [x] Opt-in Windows startup control.

## Phase 2 — Persistent life

- [x] Bundled SQLite + WAL + migrations.
- [x] Persist long-lived pet state.
- [x] DB-owning worker + changed-only autosave + bounded final flush.
- [x] Session-only fallback.
- [x] Bounded activity journal and relationship events.
- [x] Typed memories + FTS5/BM25/importance/recency retrieval.
- [x] Hourly interaction rhythm.
- [x] Memory Browser/editor + manual CRUD.
- [x] Memory provenance + Activity UI.
- [ ] Consolidate duplicated memory-domain types before Memory Evaluator growth.
- [ ] Memory Evaluator: store / merge / discard automatic candidates.
- [ ] Stronger relationship/personality evolution.
- [ ] Local-data export/reset/backup policy.

## Phase 3 — Windows awareness / privacy

- [x] Idle / return and local-hour sensors.
- [x] Foreground executable identity without titles by default.
- [x] Privacy-gated visible DWM bounds.
- [x] Fail-closed per-app exclusions.
- [x] `privacy-rules.json` separated from memory/history.
- [x] Flushed temp-write + Windows replace-existing/write-through rule replacement.
- [x] Structured Screen Context Broker.
- [x] Opt-in bounded UI Automation metadata.
- [x] No Name / Value / HelpText / raw text / tree dumps.
- [x] Accessibility reader unload when disabled.
- [x] Accessibility initialization retry/backoff.
- [x] Settings status/control.
- [x] Focus Guard + low-noise context bubbles.
- [ ] Add observation freshness/timestamps before AI depends on structured context.

## Phase 4 — Consolidation before AI

- [x] README / Architecture synchronized with current code.
- [x] Foundation Review marked historical.
- [x] Runtime-manifest source of truth consolidated.
- [x] Runtime-health contract simplified to implemented semantics.
- [ ] Split `CompanionView.svelte` into section components.
- [ ] Split `src-tauri/src/lib.rs` composition root into setup/commands/shutdown modules.
- [ ] Common worker lifecycle/supervision: cancel, shutdown, join/restart, health.
- [ ] Replace growth of independent frontend polling loops with a bounded shared subscription/event strategy.
- [ ] Dependency lockfiles.
- [ ] Full Svelte-specific diagnostic/typecheck gate.
- [ ] Release/manual Windows `tauri build` CI path.

## Phase 5 — Local AI brain

Do not start until consolidation/reproducibility and the target-machine baseline are acceptable.

- [ ] Isolated LLM worker executable.
- [ ] llama.cpp integration.
- [ ] MiniCPM5-1B benchmark on R3 2200G + 16 GB.
- [ ] Load/unload and memory-pressure policy.
- [ ] Dynamic context composer.
- [ ] Fresh privacy-approved Screen Context + relevant-memory composition.
- [ ] Structured AI intent contract; Pet Brain remains authoritative.
- [ ] Connect Memory Evaluator without making ordinary life depend on the LLM.

## Phase 6 — Optional visual understanding

- [ ] Normalized `ScreenObservation` contract.
- [ ] On-demand window/region capture + visible indicator.
- [ ] Reuse per-app privacy policy for capture.
- [ ] No screenshot history by default.
- [ ] Separate unloadable vision worker.
- [ ] Benchmark only if structured context is insufficient.
- [ ] No continuous capture by default.

## Phase 7 — Companion depth

- [x] Home / Memory / Activity / Settings surfaces.
- [x] Startup, privacy and accessibility controls.
- [ ] Conversation panel.
- [ ] Personality/bond evolution.
- [ ] Long-term adaptive behavior.
- [ ] Optional voice.
- [ ] Optional phone bridge.

## Before public release

- [ ] Code license.
- [ ] Separate Lenvu character/artwork license.
- [ ] `package-lock.json` and `Cargo.lock` from a real dependency resolution.
- [ ] `SECURITY.md`.
- [ ] Restrictive CSP.
- [ ] Production reference derivatives/icon/character assets.
- [ ] Clean Windows executable/bundle build.
- [ ] Target-hardware RAM/CPU/GPU baseline.
- [ ] Tracked-data/secrets/private-data audit.
- [ ] Local-data export/reset/backup verification.

## Definition of success

Unload MiniCPM5-1B and any future vision worker completely. Lenvu still feels alive and useful: movement, sleep/wake, petting, attention, focus behavior, persistence, inspectable memory and ordinary desktop interaction continue without AI.
