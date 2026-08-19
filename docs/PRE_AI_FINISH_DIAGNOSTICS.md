# Pre-AI Finish Diagnostics

- Source commit: 13f88b61c28f045e37fde199070f38e923841ade
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
Corrected queue_memory patch precondition for this clean-runner worktree.
Traceback (most recent call last):
  File "D:\a\NorthPalace-my-pet\NorthPalace-my-pet\scripts\finish-pre-ai.py", line 952, in <module>
    literal(
  File "D:\a\NorthPalace-my-pet\NorthPalace-my-pet\scripts\finish-pre-ai.py", line 23, in literal
    raise RuntimeError(f"{path}: expected {expected} match(es), found {count}: {old!r}")
RuntimeError: README.md: expected 1 match(es), found 0: '`docs/SVELTE_DIAGNOSTIC_BASELINE.md` records the clean dual-Svelte gate and `docs/VALIDATION_BASELINE.md` records the clean frontend/Rust gate. `docs/WINDOWS_BUNDLE_BASELINE.md` records a clean GitHub-hosted Windows release/NSIS build, artifact discovery and bounded release-executable smoke launch. The manual Windows Bundle workflow consumes the same committed dependency graphs rather than generating independent candidates.\n'
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
