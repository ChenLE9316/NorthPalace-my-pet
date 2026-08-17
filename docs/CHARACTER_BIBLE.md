# Lenvu Character Bible

This document summarizes the character traits runtime art must preserve. **Image evidence is authoritative; this text is subordinate to `docs/LENVU_VISUAL_GROUND_TRUTH.md`.**

## Identity

- **Name:** Lenvu
- **Species:** Neralune
- **Role:** quiet local-first digital companion / desktop pet
- **Product rule:** Lenvu is a living desktop creature first; AI is one optional cognition layer.

## Visual authority

Use this order whenever sources disagree:

1. original high-resolution Lenvu character sheets;
2. original high-resolution README/system-overview artwork;
3. other original reference sheets that remain consistent with the anatomy source;
4. this Character Bible;
5. production anchors/landmarks;
6. generated review candidates.

Assistant memory, conversation summaries and prompt wording are **not** visual authority. A generated candidate cannot redefine Lenvu simply because it looks polished.

See `docs/LENVU_VISUAL_GROUND_TRUTH.md` for the direct observed-image specification.

## Canonical visual markers

The source sheets consistently show:

- a tall, lean, long-legged canine/wolf-like body with dragon/digital traits;
- cool slate/blue-gray upper fur with white/light muzzle, chest, underside and limb regions;
- large tall triangular ears with cyan digital/circuit structures inside;
- a pair of dark segmented/ridged horns swept upward/back;
- a restrained gold ring/crescent-like identity accent on Lenvu's anatomical left horn;
- heterochromia: anatomical right eye cyan, anatomical left eye violet/purple;
- a centered white forehead glyph;
- cyan Lumen-Code circuitry/light paths integrated into ears, back/spine/flank and legs;
- canine paws with lower-leg diamond/circuit details;
- a thick physical fluffy tail whose terminal transitions into cyan holographic/digital energy;
- an athletic/elegant silhouette, not a chibi, kitten, plush or super-deformed mascot silhouette.

Exact pixel colors and normalized production coordinates may evolve during measured normalization, but they must not change the visible identity above.

## Silhouette rules

1. Preserve the elongated canine muzzle; do not round it into a cat/kitten face.
2. Preserve long legs and a relatively small head-to-body ratio; do not shorten the body into chibi proportions.
3. Preserve the tall ear pair, dark horn pair, neck/chest ruff and physical tail volume.
4. Eye heterochromia must survive default desktop scale.
5. Cyan markings remain secondary structural accents rather than becoming full-body glow noise.
6. Gold remains a small horn identity accent, not a dominant palette or body-marking system.
7. Tail-base thickness, ear size, horn spacing and muzzle length must not fluctuate between animation families.

## Expression language

Lenvu communicates by motion before text.

Primary channels:

- ears: attention, uncertainty, alertness;
- eyes/blink: calm, curiosity, sleepiness, happiness;
- head angle/tilt: curiosity and user attention;
- tail: comfort, playfulness, excitement;
- posture: energy and social intent;
- cyan Lumen-Code intensity: system/focus/AI state, used sparingly.

Expression changes may deform soft fur and ear/tail pose, but must not change species anatomy or identity markings.

## Canonical pose families

The production set should normalize at least:

- stand / idle;
- observe / head tilt;
- sit;
- lie / rest;
- sleep;
- wake / stretch;
- walk;
- run;
- jump;
- receive petting;
- play;
- Focus Guard;
- thinking / listening.

These pose families map to `src/lib/pet/lenvu.manifest.json` animation IDs. Pet Brain never addresses raw frame names.

## Animation consistency contract

Every production animation must share:

- one source-faithful body scale and species silhouette;
- one ground/contact convention;
- stable feet/body anchors;
- stable head/ear/horn proportions;
- consistent anatomical left/right eye identity;
- consistent Lumen-Code placement;
- physical-tail-base continuity with the digital terminal;
- defined frame bounds and transparent padding;
- hit-mask alignment with the rendered silhouette.

**If a normalized master conflicts with the Visual Ground Truth, the master fails QA and must be corrected.** The master does not override the original character identity.

## UI relationship

Lenvu is not a mascot pasted inside a dashboard. The default experience is an ambient transparent desktop creature. Panels, bubbles and Focus Guard effects must visually orbit/support the character rather than become the visual center.

## Reference material

- visual authority specification: `docs/LENVU_VISUAL_GROUND_TRUTH.md`
- README/system overview: `docs/assets/lenvu-system-overview.webp`
- primary anatomy reference: `assets/reference/anatomy/lenvu-anatomy-reference.webp`
- reference provenance: `assets/reference/manifest.json`

Additional original anatomy, expression, movement and interaction sheets belong under `assets/reference/` after provenance and resolution are recorded. They are reference evidence, not runtime frames.
