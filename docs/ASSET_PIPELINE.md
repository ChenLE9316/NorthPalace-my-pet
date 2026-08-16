# Lenvu Runtime Asset Pipeline

## Goal

The concept sheets define Lenvu's visual identity, but they are not animation-ready runtime assets. Production animation must normalize anatomy, scale, anchor, silhouette and frame timing before any sheet is consumed by PixiJS.

## Repository split

```text
assets/
|- reference/
|  |- anatomy/
|  |- expressions/
|  |- movement/
|  |- behavior/
|  |- abilities/
|  `- ui-concepts/
`- runtime/
   `- lenvu/
      |- sprites/
      |- atlases/
      |- masks/
      |- effects/
      `- manifest.json
```

The currently served runtime contract lives at `public/assets/runtime/lenvu/manifest.json`. It contains animation identifiers and canonical render metadata while production sprite/atlas paths are intentionally empty.

## Canonical character rules

Before authoring frames, lock:

- head/body ratio;
- ear size and attachment points;
- horn position and gold ring details;
- heterochromia orientation;
- Lumen-Code markings;
- paw size and leg length;
- tail base and holographic tip silhouette;
- neutral ground contact line;
- pivot/anchor location;
- left/right facing convention.

Every animation sheet must be derived from this canonical body, not independently redrawn from a different concept sheet.

## Animation contract

Initial IDs:

- `idle`
- `observe`
- `sit`
- `rest`
- `sleep`
- `wake`
- `pet_receive`
- `play`
- `focus_guard`
- `walk`
- `run`
- `jump`
- `thinking`

Pet Brain emits domain state and Behavior Intents. `src/lib/pet/animation.ts` resolves those facts into one renderer-facing animation ID. The renderer must not infer personality or mutate Pet Brain state.

## PixiJS loading strategy

Production assets should use PixiJS Assets/manifest bundles and sprite sheets rather than dozens of independent image requests. The expected runtime flow is:

```text
Lenvu runtime manifest
        |
        v
quality-tier bundle
        |
        v
sprite sheet / atlas
        |
        v
animation frames
        |
        v
PixiJS AnimatedSprite / renderer graph
```

Keep a vector placeholder path until the first canonical atlas is complete so engineering does not block on art production.

## Low-end target policy

Target: Ryzen 3 2200G + Vega 8 + 16 GB DRAM.

- default renderer resolution: 1x;
- avoid antialiasing unless measurements justify it;
- prefer atlases to many standalone textures;
- keep idle animation frame rates low;
- allow a low-power animation tier;
- avoid permanent particles/glow layers;
- unload optional/high-cost asset bundles when not needed;
- record VRAM/system-memory impact of each quality tier.

## Hit masks

Runtime hit masks are separate from visible textures. Each major pose/animation family eventually needs a coarse interaction mask for:

- head/petting region;
- body/drag region;
- tail interaction region;
- transparent pass-through region.

Do not use full per-pixel alpha testing as the first implementation. A small number of normalized geometric zones is cheaper and easier to keep stable across frames.

## Acceptance criteria for the first production atlas

1. No body-size popping between idle, sit, walk and sleep.
2. Anchor remains stable on the desktop ground line.
3. Heterochromia and horn details do not swap accidentally when facing changes.
4. Hit regions stay aligned with visible anatomy.
5. Idle rendering remains cheap enough for all-day use on Vega 8.
