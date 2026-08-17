# Canonical Lenvu Landmark Guide V1

This guide turns the Canonical Lenvu contract into a repeatable authoring/QA coordinate system.

It is deliberately a **target guide**, not a claim that concept art was measured to engineering precision. Concept sheets are reference evidence; the normalized production master is expected to converge on these landmarks once and then remain stable across animation families.

## Coordinate convention

- authoring canvas: `1024 × 1024`;
- origin: top-left;
- normalized coordinates: `0.0 … 1.0`;
- ground line: `y = 0.86`;
- all major landmarks live in `assets/runtime/lenvu/source-notes/master-landmarks.json`;
- semantic side names are **Lenvu's anatomical left/right**, not viewer-left/viewer-right.

The guide is aligned with `docs/CANONICAL_LENVU.md` and `src/lib/pet/lenvu.manifest.json`.

## Identity-critical landmarks

The first invariant is identity, not symmetry:

```text
viewer-left in a frontal drawing       viewer-right
            │                              │
       Lenvu RIGHT eye                Lenvu LEFT eye
           cyan                           violet

                         Lenvu LEFT horn
                         carries gold crescent
```

A production-direction transform may move geometry, but it must preserve those semantic sides. A simple whole-sprite horizontal flip is therefore not a valid final direction strategy.

## Front neutral targets

| Landmark | x | y | tolerance |
| --- | ---: | ---: | ---: |
| root | 0.50 | 0.86 | ground ±0.005 |
| right ear tip | 0.31 | 0.07 | silhouette ±0.025 |
| left ear tip | 0.69 | 0.07 | silhouette ±0.025 |
| right horn tip | 0.44 | 0.13 | major ±0.020 |
| left horn tip | 0.56 | 0.13 | major ±0.020 |
| right/cyan eye | 0.42 | 0.29 | identity ±0.010 |
| left/violet eye | 0.58 | 0.29 | identity ±0.010 |
| nose | 0.50 | 0.36 | identity ±0.010 |
| chin | 0.50 | 0.41 | major ±0.020 |
| right shoulder | 0.38 | 0.46 | major ±0.020 |
| left shoulder | 0.62 | 0.46 | major ±0.020 |
| right front-paw contact | 0.40 | 0.86 | ground ±0.005 |
| left front-paw contact | 0.60 | 0.86 | ground ±0.005 |

These coordinates are a normalization target. Fur tips can cross them; skeletal/contact identity should not drift frame-to-frame.

## Left profile neutral targets

The existing anatomy sheet's profile evidence faces screen-left. For the target guide:

| Landmark | x | y |
| --- | ---: | ---: |
| nose | 0.18 | 0.34 |
| visible right/cyan eye | 0.29 | 0.29 |
| head pivot | 0.32 | 0.29 |
| shoulder | 0.39 | 0.44 |
| front-paw contact | 0.38 | 0.86 |
| hip | 0.66 | 0.55 |
| rear-paw contact | 0.70 | 0.86 |
| tail base | 0.70 | 0.50 |
| tail effect peak | 0.84 | 0.27 |

The physical tail base is anatomy. The cyan/holographic terminal is an effect and may deform more aggressively without changing the tail-base landmark.

## Back neutral targets

The back view primarily locks spine/tail symmetry:

- spine top `(0.50, 0.17)`;
- spine mid `(0.50, 0.49)`;
- tail base `(0.50, 0.58)`;
- right rear-paw contact `(0.40, 0.86)`;
- left rear-paw contact `(0.60, 0.86)`.

The glowing dorsal Lumen-Code line may animate in intensity, but its anatomical centerline may not wander between frames.

## QA tolerances

The default V1 acceptance tolerances are:

```text
ground contact      ±0.005 normalized  ≈ 5 px @ 1024
identity feature    ±0.010             ≈ 10 px
major joint/pivot   ±0.020             ≈ 20 px
outer silhouette    ±0.025             ≈ 26 px
```

Animation-specific deformation may intentionally exceed a silhouette tolerance—for example ears folding or a jump pose—but the deviation must be authored and should not result from inconsistent source scaling.

## Reference evidence regions

The machine-readable guide also records approximate source-sheet crop regions for front/profile/back/3⁄4/head/detail evidence on the original `1536 × 1024` anatomy board. These regions are navigation aids for artists/tools, not runtime crop rectangles.

## Production usage

The intended pipeline is:

```text
reference evidence
      ↓
canonical master landmarks
      ↓
normalized master views
      ↓
animation pose keyframes
      ↓
frame QA against stable root/pivots
      ↓
directional identity validation
      ↓
atlas export
```

For the first runtime asset family, `idle → sit → sleep → walk` should be normalized before adding highly deforming play/jump states. This gives the renderer a stable low-cost baseline and makes anchor/hit-mask errors easier to diagnose.

## Completion boundary

This landmark guide completes the **measurement/QA contract** only.

It does not mean:
- normalized master artwork is finished;
- production sprites exist;
- atlas assets are ready;
- the PixiJS placeholder can be removed.

Those milestones remain separate in the roadmap.
