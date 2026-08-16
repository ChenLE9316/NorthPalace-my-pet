# Lenvu Character Bible

This document freezes the character traits that runtime art must preserve while production animation assets are normalized.

## Identity

- **Name:** Lenvu
- **Species:** Neralune
- **Role:** quiet local-first digital companion / desktop pet
- **Product rule:** Lenvu is a living desktop creature first; AI is one optional cognition layer.

## Canonical visual markers

The following features define Lenvu's identity and should stay consistent across sprites, expressions, UI art and future 3D/2D variants:

- gray-white dog-dragon body and soft fur silhouette;
- large ears with expressive rotation/fold language;
- horn pair with a restrained golden ring/crescent identity detail;
- heterochromia: cyan and violet/purple eyes;
- cyan luminous circuit/Lumen-Code markings;
- fluffy tail with a holographic/cyan digital-energy terminal effect;
- friendly rounded proportions rather than an aggressive dragon silhouette.

Exact pixel colors, horn angles, ear dimensions and body ratios are **not yet frozen numerically**. They must be derived from the normalized master reference, not independently redrawn per animation.

## Silhouette rules

1. The ears, horn pair and tail must remain readable at small desktop-pet sizes.
2. Eye heterochromia must survive the default desktop scale.
3. The cyan markings are secondary accents; they must not turn the entire body into a high-frequency glowing effect.
4. The golden horn detail is an identity accent, not a dominant UI color.
5. Tail volume and ear size must not fluctuate between animation sheets.

## Expression language

Lenvu should communicate by motion before text.

Primary channels:

- ears: attention, uncertainty, alertness;
- eyes/blink: calm, curiosity, sleepiness, happiness;
- head tilt: curiosity / user attention;
- tail: comfort, playfulness, excitement;
- posture: energy and social intent;
- cyan Lumen-Code intensity: system/focus/AI state, used sparingly.

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

- one canonical body scale;
- one ground/contact convention;
- stable feet/body anchors;
- stable head/ear/horn proportions;
- consistent left/right eye identity;
- consistent marking placement;
- defined frame bounds and transparent padding;
- hit-mask alignment with the rendered silhouette.

If a concept sheet conflicts with the normalized master, the normalized master wins for runtime assets while the source sheet remains preserved as reference art.

## UI relationship

Lenvu is not a mascot pasted inside a dashboard. The default experience is an ambient transparent desktop creature. Panels, bubbles and Focus Guard effects must visually orbit/support the character rather than become the visual center.

## Reference material

The current architecture/UI concept board remains at:

`docs/assets/lenvu-system-overview.webp`

Additional user-provided anatomy, expression, movement and interaction sheets should be stored under `assets/reference/` as source/reference material after provenance and resolution are recorded.
