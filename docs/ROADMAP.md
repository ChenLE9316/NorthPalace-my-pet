# NorthPalace-my-pet Roadmap

A checked item means the capability exists in the repository; it does not automatically mean the latest `main` commit has been observed passing Windows CI or performance-validated on the Ryzen 3 2200G target.

## Phase 0 — Foundation

- [x] Tauri + Svelte + Rust scaffold.
- [x] Original Lenvu overview preserved as product north star.
- [x] Pet-first / AI-second acceptance rule.
- [x] Rust-owned monotonic Pet Runtime.
- [x] Domain Event channel and immutable snapshots.
- [x] Parallel Pet State V2 and Behavior Intent.
- [x] Truthful runtime health: `ready / degraded / error`.
- [x] Runtime panic preserves last snapshot and marks `error`.
- [x] Windows CI on `main` with tracked-data guard, frontend build, Rust fmt, Clippy and tests.
- [x] Root-local runtime ignore is anchored so `assets/runtime/` remains available for production assets.
- [x] CI rejects tracked local databases, model weights, `.env`, privacy rules, logs and private key containers.
- [x] Add manual Windows NSIS bundle workflow and upload build artifacts for inspection.
- [ ] Observe the first successful manual Windows bundle run.
- [ ] Dependency lockfiles / reproducible installs.
- [ ] Clean executable/bundle run on target Windows machine.
- [ ] RAM / idle CPU / GPU baseline on Ryzen 3 2200G + Vega 8.

## Phase 1 — Living desktop pet

### Character / renderer

- [x] Character Bible and original-image visual authority.
- [x] Source provenance + measured canonical landmark/remap pipeline.
- [x] Freeze visual governance; next work should produce assets.
- [x] One authoritative runtime manifest.
- [x] Procedural PixiJS placeholder + semantic animation/FPS policy.
- [x] No blind whole-root identity mirror.
- [ ] High-quality source-faithful anatomy/README derivatives.
- [ ] Canonical production Lenvu master.
- [ ] Production Idle / Walk / Sit / Lie / Sleep / Wake assets.
- [ ] Production sprite atlas / hit masks.

### Desktop behavior

- [x] Separate `pet` and `companion` windows.
- [x] Monitor / DPI / work-area context.
- [x] Semantic hit zones + selective native click-through.
- [x] Native work-area movement seeded from domain facing.
- [x] Weighted ambient personality selection.
- [x] Safe horizontal multi-monitor Explore only.
- [x] Hover / touch / pet / play / drag / held / drop.
- [x] Petting and Companion opening separated.
- [x] System tray + opt-in Windows startup.

## Phase 2 — Persistent life

- [x] Bundled SQLite + WAL + migrations.
- [x] Long-lived state persistence and transient-state reset.
- [x] DB worker + changed-only autosave + bounded final flush.
- [x] Session-only fallback.
- [x] Bounded activity/relationship history.
- [x] Typed memories + FTS5/BM25/importance/recency retrieval.
- [x] Hourly interaction rhythm.
- [x] Memory Browser/editor + CRUD + provenance.
- [x] Consolidate duplicated memory-domain types.
- [ ] Memory Evaluator: store / merge / discard.
- [ ] Stronger relationship/personality evolution.
- [ ] Local-data export/reset/backup policy.

## Phase 3 — Windows awareness / privacy

- [x] Idle / return and local-hour sensors.
- [x] Foreground executable identity without titles by default.
- [x] Privacy-gated DWM bounds and fail-closed app exclusions.
- [x] Durable `privacy-rules.json` replacement.
- [x] Structured Screen Context Broker.
- [x] Opt-in bounded UI Automation metadata with unload/retry behavior.
- [x] Sensor freshness timestamps for active app, accessibility, idle and local hour.
- [x] Semantic `sequence` separated from freshness heartbeats.
- [x] Focus Guard + low-noise context bubbles.

## Phase 4 — Consolidation before AI

- [x] README / Architecture synchronized with current code.
- [x] Foundation Review marked historical.
- [x] Runtime manifest source consolidated.
- [x] Runtime-health contract simplified to implemented semantics.
- [ ] Split `CompanionView.svelte` into section components.
- [ ] Split `src-tauri/src/lib.rs` composition root.
- [ ] Common worker lifecycle/supervision.
- [ ] Replace growth of independent frontend polling loops with bounded shared subscription/event strategy.
- [ ] Dependency lockfiles.
- [ ] Svelte-specific diagnostic gate after TypeScript 7 toolchain compatibility is fixed and locked.

## Phase 5 — Local AI brain

Do not start until consolidation/reproducibility and target-machine baseline are acceptable.

- [ ] Isolated LLM worker executable.
- [ ] llama.cpp integration.
- [ ] MiniCPM5-1B benchmark on R3 2200G + 16 GB.
- [ ] Load/unload + memory-pressure policy.
- [ ] Dynamic context composer with freshness-gated structured context.
- [ ] Structured AI intent contract; Pet Brain remains authoritative.
- [ ] Connect Memory Evaluator without ordinary-life LLM dependency.

## Phase 6 — Optional visual understanding

- [ ] Normalized `ScreenObservation` contract.
- [ ] On-demand region/window capture + visible indicator.
- [ ] Reuse per-app privacy deny policy.
- [ ] No screenshot history by default.
- [ ] Separate unloadable vision worker.
- [ ] No continuous capture by default.

## Before public release

- [ ] Code license and separate Lenvu artwork license.
- [ ] `package-lock.json` + `Cargo.lock` from real dependency resolution.
- [x] Add `SECURITY.md` with pre-release vulnerability/privacy reporting guidance.
- [x] Configure a restrictive production CSP for local bundle resources and Tauri IPC; keep Vite development CSP explicitly disabled.
- [ ] Verify the production CSP in the first successful Windows bundle/run.
- [ ] Production derivatives/icon/character assets.
- [ ] Successful clean Windows bundle plus target performance baseline.
- [ ] Final manual tracked-data/secrets/private-data audit before visibility/release change.
- [ ] Local-data export/reset/backup verification.

## Definition of success

Unload MiniCPM5-1B and any future vision worker completely. Lenvu still feels alive and useful: movement, sleep/wake, petting, attention, focus behavior, persistence, inspectable memory and ordinary desktop interaction continue without AI.
