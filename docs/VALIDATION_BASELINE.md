# Windows Validation Baseline

Machine-generated on a clean GitHub-hosted windows-latest runner after the Rust quality-gate repair.

- Validated working-tree parent: 81ee320ba8e0ecb916a8eb77992b0738af604c86
- Recorded at (UTC): 2026-08-19T05:05:28.2212882Z
- Node.js: v24.19.0
- npm: 11.17.0
- rustc: rustc 1.97.1 (8bab26f4f 2026-07-14)
- cargo: cargo 1.97.1 (c980f4866 2026-06-30)

| Check | Outcome |
|---|---|
| tracked local/private data guard | success |
| npm ci from committed package-lock.json | success |
| Svelte / PixiJS production build | success |
| cargo fmt --check | success |
| cargo clippy --locked --all-targets -- -D warnings | success |
| cargo test --locked | success |

NSIS bundle execution and Ryzen 3 2200G target-machine performance remain separate acceptance gates.
