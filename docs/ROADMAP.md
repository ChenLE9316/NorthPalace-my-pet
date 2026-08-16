# NorthPalace-my-pet Roadmap

## Phase 0 — Foundation (now)

- [x] Initialize private repository.
- [x] Commit first architecture/UI concept image.
- [x] Create Tauri + Svelte + Rust scaffold.
- [x] Create minimal offline Pet Brain domain model.
- [x] Add initial architecture, UI/UX and model-runtime specifications.
- [ ] Run first clean build on the target Windows 11 machine.
- [ ] Record baseline RAM / idle CPU / GPU usage.

## Phase 1 — Living desktop pet

- [ ] Replace placeholder visual with production Lenvu render assets.
- [ ] PixiJS renderer and animation graph.
- [ ] Transparent hit-test regions / click-through behavior.
- [ ] Multi-monitor and DPI behavior.
- [ ] Idle/observe/sit/rest/sleep transitions.
- [ ] Walk/run/reposition behavior.
- [ ] Hover, touch, pet, drag and play interactions.
- [ ] Tray control and startup policy.

## Phase 2 — Persistent life

- [ ] SQLite schema and migrations.
- [ ] Save pet state and relationship.
- [ ] Episodic/semantic/preference/relationship memory types.
- [ ] Time-of-day rhythm.
- [ ] Activity/event journal with bounded retention.

## Phase 3 — Focus companion

- [ ] User idle / return sensor.
- [ ] Active-window adapter.
- [ ] Focus Guard sessions.
- [ ] Context bubbles and low-noise reminders.
- [ ] Per-app privacy exclusions.

## Phase 4 — Local AI brain

- [ ] Isolated LLM worker executable.
- [ ] llama.cpp integration.
- [ ] MiniCPM5-1B GGUF benchmark on R3 2200G + 16 GB.
- [ ] Dynamic context composer.
- [ ] Structured AI intent contract.
- [ ] AI load/unload and memory-pressure policy.

## Phase 5 — Companion depth

- [ ] Conversation panel.
- [ ] Memory browser/editor.
- [ ] Personality and bond evolution.
- [ ] Long-term behavior adaptation.
- [ ] Optional voice layer.
- [ ] Optional phone companion bridge.

## Definition of success

Lenvu should still feel alive and useful when the network is offline and the LLM worker is completely unloaded.
