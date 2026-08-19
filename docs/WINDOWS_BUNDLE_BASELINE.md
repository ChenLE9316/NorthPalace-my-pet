# Windows Bundle Baseline

Machine-generated on a clean GitHub-hosted windows-latest runner.

- Validated source commit: 609d137a5155ef5ee40394060bc6b2a55626ed92
- Recorded at (UTC): 2026-08-19T05:32:16.8262059Z
- Node.js: v24.19.0
- npm: 11.17.0
- rustc: rustc 1.97.1 (8bab26f4f 2026-07-14)
- cargo: cargo 1.97.1 (c980f4866 2026-06-30)

| Check | Outcome |
|---|---|
| npm ci | success |
| Svelte TS6 + TS7 zero-warning gates | success |
| tracked-data guard | success |
| cargo metadata --locked | success |
| Tauri release + NSIS build | success |
| release executable 8-second smoke launch | success |
| bundle artifact discovery | success |

## Artifacts

- App: northpalace-my-pet.exe
- App bytes: 11999232
- App SHA-256: 92AC41516C0FB4FAF22274C04DBE922E8F9C2123643963234977B3B97A5AB847
- NSIS: NorthPalace-my-pet_0.1.0_x64-setup.exe
- NSIS bytes: 3515254
- NSIS SHA-256: DB7EC56B6CE8EC813DAB289E3E8D2C6A0979E56644000A9457886C5197D1D009

This validates clean Windows buildability and a bounded launch smoke test on GitHub-hosted Windows. It does not replace Ryzen 3 2200G target-machine behavior/performance validation.
