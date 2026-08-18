# Windows Validation Baseline

Machine-generated on a clean GitHub-hosted windows-latest runner. The one-time workflow that produced this file removes itself in the same commit.

- Source commit: 06fb280276edc6e647e1b477bd2314e127adf801
- Recorded at (UTC): 2026-08-18T23:05:22.1974191Z
- Node.js: v24.19.0
- npm: 11.17.0
- rustc: rustc 1.97.1 (8bab26f4f 2026-07-14)
- cargo: cargo 1.97.1 (c980f4866 2026-06-30)

| Check | Outcome |
|---|---|
| tracked local/private data guard | success |
| npm ci from committed package-lock.json | success |
| Svelte / PixiJS production build | success |
| stable Rust + rustfmt + Clippy setup | success |
| cargo metadata --locked | success |
| cargo fmt --check | failure |
| cargo clippy --locked --all-targets -- -D warnings | failure |
| cargo test --locked | success |

This is source-validation evidence only. NSIS execution/bundle verification and Ryzen 3 2200G target-machine performance remain separate acceptance gates.
