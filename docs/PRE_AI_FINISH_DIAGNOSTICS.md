# Pre-AI Finish Diagnostics

- Source commit: 6d447c774dcb70177d2803e3c5d2af73e830e739
- patch: failure
- npm ci: skipped
- tracked: skipped
- secrets: skipped
- target PowerShell parse: skipped
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
Corrected finish-patch preconditions and selected robust semantic hardening v2.
Pre-AI finish patch applied.
Current-state README semantics finalized.
Traceback (most recent call last):
  File "D:\a\NorthPalace-my-pet\NorthPalace-my-pet\scripts\finish-pre-ai-followup.py", line 119, in <module>
    replace(
  File "D:\a\NorthPalace-my-pet\NorthPalace-my-pet\scripts\finish-pre-ai-followup.py", line 20, in replace
    raise RuntimeError(f"{path}: expected {count} match(es), found {actual}: {old!r}")
RuntimeError: src-tauri/src/memory_evaluator.rs: expected 1 match(es), found 0: 'INSERT INTO memories (kind, content, importance, source_event_id, created_at_ms, updated_at_ms)\\n\\\\n             VALUES (?1, ?2, ?3, ?4, ?5, ?5)'
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
