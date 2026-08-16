# Lenvu Asset Pipeline

Reference art and runtime art are intentionally separated.

## Repository layout

```text
assets/
├─ reference/
│  ├─ anatomy/
│  ├─ expressions/
│  ├─ movement/
│  ├─ behavior/
│  ├─ abilities/
│  └─ ui-concepts/
└─ runtime/
   └─ lenvu/
      ├─ sprites/
      ├─ atlases/
      ├─ masks/
      ├─ effects/
      └─ source-notes/
```

Runtime behavior metadata lives in `src/lib/pet/lenvu.manifest.json` so the renderer has one versioned contract for animation IDs, frame budgets, anchors and normalized hit zones.

## Pipeline

```text
Source concept/reference art
        ↓
Canonical anatomy normalization
        ↓
Master pose / anchor guide
        ↓
Animation cleanup
        ↓
Consistent transparent frame bounds
        ↓
Sprite atlas + hit masks
        ↓
Manifest asset paths
        ↓
PixiJS renderer
```

## Rules

1. Never use concept-board screenshots as production sprite frames.
2. Never resize each animation independently; normalize against one canonical master.
3. Preserve original reference files when possible. Runtime derivatives may be optimized separately.
4. Runtime texture dimensions should be selected from measured Vega 8 memory/performance results, not assumed high-resolution defaults.
5. Transparent padding, pivot and ground contact must remain stable across frames.
6. Hit masks must follow the visible body while keeping interaction forgiving at small sizes.
7. The renderer consumes semantic animation IDs such as `sleep`, `walk` or `pet_receive`; Pet Brain never knows atlas/frame filenames.
8. Lumen-Code glow/effects should be separable from base character sprites so low-power mode can reduce or disable them.

## Manifest contract

Current schema version: `1`.

The manifest records:

- character identity and reference canvas;
- canonical anchor;
- nominal desktop render size;
- normal and low-power frame budgets;
- normalized interaction hit zones;
- semantic animation profiles;
- future atlas asset locations.

An animation entry with `asset: null` means the current procedural/vector placeholder remains active. When production sprites are ready, set the entry to a runtime atlas reference without changing domain behavior.

## Interaction masks

V0.2 uses forgiving normalized ellipse zones for `head`, `body` and `tail`. They are **interaction semantics**, not final per-pixel Windows hit masks.

The migration path is:

```text
normalized semantic hit zones
        ↓
frame-aligned mask metadata
        ↓
Windows cursor re-entry / native hit-test layer
        ↓
transparent desktop click-through outside Lenvu
```

The application must not enable whole-window cursor passthrough until a reliable native re-entry path exists, otherwise Lenvu could become impossible to click again from inside the WebView.

## Low-power renderer policy

The PixiJS ticker must be capped using animation-specific FPS values. Rest/sleep can use lower frame budgets than active play/run states. This matters because NorthPalace-my-pet targets an always-on Ryzen 3 2200G + Vega 8 + 16 GB system.

## Acceptance checklist for a production animation

- anatomy matches the canonical master;
- left/right eye identity is correct;
- horn-ring and markings remain stable;
- frame bounds do not jitter;
- anchor/ground contact is stable;
- loop has no visible pop when `loop=true`;
- hit mask aligns with the pose;
- normal and low-power FPS both look acceptable;
- memory cost is recorded;
- source/provenance is documented.
