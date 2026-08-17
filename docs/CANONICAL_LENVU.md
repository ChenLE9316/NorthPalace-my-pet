# Canonical Lenvu Production Master Contract V1

This document freezes the **production coordinate, identity and mirroring contract** used to normalize Lenvu before any runtime sprite atlas is accepted.

The reference library is evidence; the production master is the authority for runtime art.

## 1. Status

- Contract version: `1`
- Character: `Lenvu`
- Species: `Neralune`
- Production master state: **contract frozen; normalized master artwork still pending**
- Primary anatomy evidence: `assets/reference/anatomy/lenvu-anatomy-reference.webp`
- Runtime consumer: `src/lib/pet/lenvu.manifest.json`

This milestone does **not** mark the production sprite/atlas as complete.

## 2. Coordinate system

The canonical authoring canvas is square so every animation family shares one coordinate system.

```text
master canvas       1024 × 1024
origin              top-left
normalized space    x/y = 0.0 … 1.0
root anchor          (0.50, 0.86)
ground line          y = 0.86
safe visual bounds   x = 0.08 … 0.92
                     y = 0.05 … 0.90
```

Runtime export may use 512×512 cells, but anchors and pivots are normalized and must not drift when resolution changes.

The current desktop nominal render target remains approximately `180 × 220` CSS logical pixels. Higher-density texture cells are allowed and preferred; nominal display size and source texture resolution are separate concerns.

## 3. Identity asymmetry — never blindly mirror

The frontal anatomy reference freezes these semantic sides:

- **Lenvu right eye:** cyan;
- **Lenvu left eye:** violet / purple;
- **gold crescent / ring:** Lenvu left horn;
- forehead glyph: centered, not side-swapped;
- Lumen-Code markings: preserve their canonical anatomical side/placement rather than treating them as disposable decoration.

A naive horizontal flip would swap the eye identity and gold horn detail. Production rendering must therefore use one of these strategies:

1. authored left/right animation variants; or
2. mirror only geometry/fur base layers and redraw semantic identity layers after the transform.

The second approach is preferred if it meaningfully reduces atlas size without increasing frame-time cost.

## 4. Stable pivots

All poses are authored against the same normalized pivot vocabulary:

| Pivot | Normalized position | Purpose |
| --- | --- | --- |
| `root` | `(0.50, 0.86)` | native-window grounding / global placement |
| `body` | `(0.50, 0.58)` | torso sway and body bob |
| `head` | `(0.50, 0.29)` | look / tilt / pet reaction |
| `tailBase` | `(0.70, 0.61)` | tail motion origin |
| `frontPawLeft` | `(0.43, 0.84)` | contact alignment |
| `frontPawRight` | `(0.57, 0.84)` | contact alignment |

These are production starting pivots, not permission to distort anatomy to hit coordinates exactly. During master normalization, the drawing is aligned to this system once; animation frames then inherit it.

## 5. Silhouette invariants

Across idle, observe, sit, rest, sleep, held, wake, pet, play, focus, walk, run, jump and thinking:

- ear length and ear-base spacing do not change between sheets;
- horn count, horn spacing and horn curvature remain stable;
- muzzle length remains friendly/rounded rather than becoming fox-like or aggressive;
- chest volume, shoulder height and foreleg thickness stay within one canonical anatomy;
- tail-base thickness and fluffy tail volume remain stable;
- the holographic/cyan tail terminal can deform as an effect, but the physical tail base cannot;
- paws keep the same relative scale and contact convention;
- white forehead glyph, heterochromia, gold ring and main cyan body marks remain recognizable at nominal desktop size.

## 6. Ground/contact convention

`root.y = 0.86` is the ground contract used by the current native pet-window movement system.

- standing/walking/running frames place the lowest supporting paw on the ground line;
- sitting frames place the supporting body/paws on the same ground line;
- rest/sleep may spread horizontally but may not silently change root height;
- jump is allowed above the ground line, with takeoff/landing frames returning to the exact root contact;
- held frames intentionally detach visually from the ground, but retain the same root coordinate for interpolation and hit-zone math.

## 7. Runtime cell and padding policy

Initial production export target:

```text
cell size             512 × 512 px
transparent padding   retained and consistent
root                   normalized (0.50, 0.86)
texture filtering      renderer-controlled
atlas groups           ambient / locomotion / interaction / special
```

Do not crop every frame tightly around the visible fur. Variable per-frame crop boxes create visual jitter, broken hit zones and unstable pivots.

## 8. Hit-zone relationship

The existing semantic `head`, `body` and `tail` zones in `lenvu.manifest.json` remain the coarse interaction contract. Production alpha/frame masks may refine them, but must not change Pet Brain semantics.

```text
Pet Brain interaction
        ↓
semantic zone id
        ↓
production mask / alpha hit test
        ↓
rendered Lenvu frame
```

## 9. Required master views before atlas production

The normalized production master must include, on one controlled scale/grid:

- front neutral;
- left profile;
- right profile;
- back neutral;
- front-left 3/4;
- front-right 3/4;
- head close-up showing eye/horn asymmetry;
- paws / primary Lumen-Code marking placement;
- tail base and holographic terminal reference.

No animation frame is accepted as production-ready until it can be traced back to those normalized views.

## 10. Acceptance checklist

A future `canonical-master` artwork milestone is complete only when:

- [ ] all required views share one body scale and ground line;
- [ ] right-eye cyan / left-eye violet are verified;
- [ ] left-horn gold crescent is verified;
- [ ] ears/horns/muzzle/chest/legs/tail are dimensionally consistent;
- [ ] root/head/body/tail pivots are measured against the final artwork;
- [ ] coarse semantic hit zones still cover the intended anatomy;
- [ ] a 180×220 desktop preview remains readable;
- [ ] left/right renderer strategy preserves identity asymmetry;
- [ ] source provenance points back to the reference manifest.

Only after this checklist passes should runtime animation `asset` fields stop being `null`.
