# Canonical Lenvu Production Master Contract V1

This document freezes the **production coordinate, identity and mirroring contract** used to normalize Lenvu before any runtime sprite atlas is accepted.

**The production master is not allowed to redefine the character.** Visual identity authority comes from `docs/LENVU_VISUAL_GROUND_TRUTH.md` and the original image evidence it names. The master is a normalized engineering/art-production derivative of that evidence.

## 1. Status

- Contract version: `1`
- Character: `Lenvu`
- Species: `Neralune`
- Production master state: **source measurement complete; normalized master artwork still pending**
- Visual ground truth: `docs/LENVU_VISUAL_GROUND_TRUTH.md`
- Primary anatomy evidence: `assets/reference/anatomy/lenvu-anatomy-reference.webp`
- README visual context: `docs/assets/lenvu-system-overview.webp`
- Runtime consumer: `src/lib/pet/lenvu.manifest.json`
- Landmark state: **measured from the original high-resolution anatomy source and normalized in `assets/runtime/lenvu/source-notes/master-landmarks.json`**

This milestone does **not** mark the production sprite/atlas as complete.

## 2. Source authority and anti-drift rule

The production master must preserve the source-defined Lenvu silhouette and materials: tall/lean long-legged canine anatomy, elongated muzzle, tall ears, two dark segmented horns, slate/blue-gray + white fur, cyan structural Lumen-Code, heterochromia, the left-horn gold accent, and a physical fluffy tail that transitions into a cyan holographic terminal.

Generated/redrawn candidates are review material only. A polished candidate that becomes chibi, cat-like, plush, predominantly pure white/gold, short-legged or otherwise visually divergent is rejected rather than promoted into a new character definition.

If a normalization coordinate would require visible anatomy to be distorted away from the Visual Ground Truth, revise the coordinate. Do not distort the character to satisfy inferred numbers.

## 3. Coordinate system

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

The coordinates above are production starting contracts, not visual-source facts. They remain adjustable if measured normalization proves that a small correction is required to preserve the source anatomy.

### 3.1 Landmark measurement status

`assets/runtime/lenvu/source-notes/source-measurement.json` and `assets/runtime/lenvu/source-notes/master-landmarks.json` now record the completed measurement pass against the original high-resolution anatomy source. The stored landmark values are source-measured normalization evidence, not measurements inferred from a generated candidate.

The approval order remains:

```text
original high-resolution image evidence
        ↓
measured source-faithful landmarks
        ↓
normalized production master
        ↓
review/promotion
        ↓
runtime animation assets
```

A generated candidate cannot be used to retroactively change source landmarks or visual identity. If a candidate visibly conflicts with the measured source or Visual Ground Truth, the candidate is rejected; the source contract is not rewritten around it.

## 4. Identity asymmetry — never blindly mirror

The frontal anatomy evidence freezes these semantic sides:

- **Lenvu anatomical right eye:** cyan;
- **Lenvu anatomical left eye:** violet / purple;
- **gold ring/crescent-like horn accent:** Lenvu anatomical left horn;
- forehead glyph: centered, not side-swapped;
- Lumen-Code markings: preserve canonical anatomical placement rather than treating them as disposable decoration.

Viewer-left/viewer-right must never be substituted for anatomical left/right in metadata.

A naive horizontal flip would swap the eye identity and gold horn detail. Production rendering must therefore use one of these strategies:

1. authored left/right animation variants; or
2. mirror geometry/base-fur layers only and restore semantic identity layers after the transform.

## 5. Stable pivots

All poses are authored against the same normalized pivot vocabulary:

| Pivot | Normalized position | Purpose |
| --- | --- | --- |
| `root` | `(0.50, 0.86)` | native-window grounding / global placement |
| `body` | `(0.50, 0.58)` | torso sway and body bob |
| `head` | `(0.50, 0.29)` | look / tilt / pet reaction |
| `tailBase` | `(0.70, 0.61)` | tail motion origin |
| `frontPawLeft` | `(0.43, 0.84)` | contact alignment |
| `frontPawRight` | `(0.57, 0.84)` | contact alignment |

These are production starting pivots, not permission to distort anatomy to hit coordinates exactly. During master normalization, artwork is aligned to this system once; animation frames then inherit it.

## 6. Silhouette invariants

Across idle, observe, sit, rest, sleep, held, wake, pet, play, focus, walk, run, jump and thinking:

- head-to-body ratio remains non-chibi;
- elongated canine muzzle remains readable and does not become a cat/kitten face;
- tall ear length and ear-base spacing remain stable;
- horn count, dark segmented construction, spacing and curvature remain stable;
- neck/chest ruff remains part of the silhouette;
- slim torso, shoulder height, long forelegs and hind legs remain within one canonical anatomy;
- canine paw scale remains stable;
- tail-base thickness and fluffy physical tail volume remain stable;
- the holographic/cyan tail terminal may deform as an effect, but the physical tail base cannot disappear;
- slate/blue-gray + white fur distribution remains recognizable;
- white forehead glyph, heterochromia, left-horn gold accent and main cyan Lumen-Code remain recognizable at nominal desktop size.

## 7. Lumen-Code / material invariants

Production simplification may reduce detail, but must preserve the source family of digital structures:

- cyan circuitry/light inside ears;
- cyan route along back/spine/flank;
- vertical cyan paths on forelegs;
- lower-leg/paw diamond or circuit clusters;
- centered rear/back route where visible;
- cyan holographic geometry at the tail terminal.

Gold does not replace the Lumen-Code system. Cyan is a restrained digital accent, not a full-body emissive fill.

## 8. Ground/contact convention

`root.y = 0.86` is the starting ground contract used by the current native pet-window movement system.

- standing/walking/running frames place the lowest supporting paw on the ground line;
- sitting frames place supporting anatomy on the same ground convention;
- rest/sleep may spread horizontally but may not silently change root height;
- jump is allowed above the ground line, with takeoff/landing returning to the exact root contact;
- held frames intentionally detach visually from the ground but retain the same root coordinate for interpolation and hit-zone math.

## 9. Runtime cell and padding policy

Initial production export target:

```text
cell size             512 × 512 px
transparent padding   retained and consistent
root                  normalized (0.50, 0.86)
texture filtering     renderer-controlled
atlas groups          ambient / locomotion / interaction / special
```

Do not crop every frame tightly around visible fur. Variable per-frame crop boxes create visual jitter, broken hit zones and unstable pivots.

## 10. Hit-zone relationship

The existing semantic `head`, `body` and `tail` zones in `lenvu.manifest.json` remain the coarse interaction contract. Production alpha/frame masks may refine them but must not change Pet Brain semantics.

```text
Pet Brain interaction
        ↓
semantic zone id
        ↓
production mask / alpha hit test
        ↓
rendered Lenvu frame
```

## 11. Required master views before atlas production

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

No animation frame is accepted as production-ready until it can be traced back to those normalized views and to the Visual Ground Truth.

## 12. Acceptance checklist

A future `canonical-master` artwork milestone is complete only when:

- [ ] species/silhouette matches the primary anatomy evidence;
- [ ] head/body ratio is non-chibi and muzzle remains canine;
- [ ] slate-gray/white fur distribution is preserved;
- [x] original high-resolution source landmarks have been measured and recorded;
- [ ] all required views share one body scale and ground line;
- [ ] anatomical right-eye cyan / left-eye violet are verified;
- [ ] anatomical left-horn gold accent is verified;
- [ ] ears/horns/muzzle/chest/legs/paws/tail are dimensionally consistent;
- [ ] Lumen-Code remains source-faithful rather than generically decorative;
- [ ] physical tail base and cyan digital terminal are both preserved;
- [ ] root/head/body/tail pivots are measured against the final artwork;
- [ ] coarse semantic hit zones still cover the intended anatomy;
- [ ] a 180×220 desktop preview remains readable;
- [ ] left/right renderer strategy preserves identity asymmetry;
- [ ] source provenance points back to the reference manifest and Visual Ground Truth.

Only after this checklist passes should runtime animation `asset` fields stop being `null`.
