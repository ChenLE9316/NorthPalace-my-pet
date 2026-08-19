# Pre-AI Finish Diagnostics

- Source commit: 0f5fea615b9136e91cedcf51b288872d58257ace
- patch: failure
- npm ci: skipped
- tracked: skipped
- secrets: skipped
- Svelte TS6: skipped
- Svelte TS7: skipped
- frontend: skipped
- fmt: skipped
- fmt check: skipped
- Cargo lock: skipped
- Clippy: skipped
- tests: skipped

## Patch
``text
Traceback (most recent call last):
  File "D:\a\NorthPalace-my-pet\NorthPalace-my-pet\scripts\finish-pre-ai.py", line 367, in <module>
    literal(
  File "D:\a\NorthPalace-my-pet\NorthPalace-my-pet\scripts\finish-pre-ai.py", line 23, in literal
    raise RuntimeError(f"{path}: expected {expected} match(es), found {count}: {old!r}")
RuntimeError: src-tauri/src/persistence.rs: expected 2 match(es), found 1: '    #[allow(dead_code)]\n    pub fn queue_memory(&self, memory: MemoryDraft) -> Result<(), String> {\n        self.tx\n            .send(PersistenceCommand::StoreMemory(memory))\n            .map_err(|_| "persistence worker channel is unavailable".to_owned())\n    }\n\n'
``

## Svelte TS6
``text
not run
``

## Svelte TS7
``text
not run
``

## Frontend
``text
not run
``

## Clippy
``text
not run
``

## Tests
``text
not run
``
