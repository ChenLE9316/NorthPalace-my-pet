# Lenvu Visual Ground Truth V1

This document defines the **image-evidence authority** for Lenvu. It exists specifically to prevent character drift caused by summaries, prompts, generated review candidates, or production-normalization numbers.

## 1. Authority order

When visual sources disagree, use this order:

1. **Original high-resolution Lenvu character sheets supplied for the project** — anatomy/model sheets are the primary visual evidence.
2. **The original high-resolution system overview used by the README** — authoritative for the intended overall look in the product/UI context.
3. Other original Lenvu expression, movement, interaction and ability sheets that remain consistent with the primary anatomy evidence.
4. `docs/CHARACTER_BIBLE.md` — a textual interpretation of the image evidence, never a replacement for it.
5. Canonical production coordinates / anchors / landmarks — normalization tools only.
6. Generated or redrawn review candidates — **never an authority source** until they are proven to match items 1–3.

Conversation summaries, assistant memory, prompt wording and generated images are not canonical visual evidence.

## 2. Primary repository evidence

- README visual context: `docs/assets/lenvu-system-overview.webp`
- Primary anatomy reference: `assets/reference/anatomy/lenvu-anatomy-reference.webp`
- Reference provenance: `assets/reference/manifest.json`

The committed anatomy WebP is currently a transport derivative of the original 1536×1024 source. The source dimensions and SHA-256 recorded in the reference manifest remain the provenance authority.

## 3. Observed silhouette — do not reinterpret

The primary sheets visibly define Lenvu as a **tall, lean, long-legged canine/wolf-like fantasy creature with dragon/digital features**.

Observed invariants:

- elongated canine muzzle with a dark nose;
- adult/young-adult canine proportions rather than kitten/chibi proportions;
- tall triangular ears that are large relative to the head but do not make the skull round or cat-like;
- two dark segmented/ridged horns rising and sweeping backward from the crown;
- pronounced neck/chest ruff;
- slim torso, long forelegs and hind legs, canine paws;
- thick physical tail base and a large fluffy tail silhouette;
- overall silhouette remains athletic and elegant, not plush, toy-like or super-deformed.

A production candidate fails identity QA if it becomes a cat, fox-like chibi mascot, round-faced plush creature, very short-legged pet, or disproportionately large-headed character.

## 4. Observed fur and material language

The image evidence shows a cool **slate / blue-gray and white** fur palette:

- slate/blue-gray dominates the crown, back, outer torso and upper limbs;
- white/very light fur dominates muzzle, cheek areas, throat/chest, underside and significant lower-limb areas;
- shading remains cool and restrained;
- cyan light is an accent/effect, not the base fur color;
- gold is a small identity accent, not a body-marking palette.

Do not normalize Lenvu into a mostly pure-white animal with gold decorative markings.

## 5. Head identity

Directly observable markers:

- two dark horns with visible segmented/ridged construction;
- tall ears with cyan luminous/digital structures visible inside the ear surfaces;
- a centered white forehead glyph/mark;
- heterochromia:
  - **Lenvu anatomical right eye = cyan**;
  - **Lenvu anatomical left eye = violet/purple**;
- a restrained gold ring/crescent-like ornament associated with **Lenvu's anatomical left horn**;
- dark nose and a long canine muzzle.

Viewer-left/viewer-right must never be substituted for anatomical left/right in asset metadata.

## 6. Lumen-Code / digital anatomy

Cyan digital markings are structurally integrated into the character rather than painted decorative swirls.

Observed families include:

- cyan circuitry/light inside the ears;
- a thin luminous route along the back/spine/flank silhouette;
- vertical cyan light paths on the forelegs;
- diamond/circuit clusters near the lower legs/paws;
- a centered cyan route visible on the back/rear view;
- digital cyan geometry associated with the tail terminal.

Production art may simplify these at small size, but it must not replace them with unrelated gold motifs or arbitrary ornamentation.

## 7. Tail identity

The tail is a hybrid physical/digital structure:

- the base and most of the tail are visibly physical, fluffy fur with gray/white volume;
- the terminal region transitions into bright cyan holographic/digital energy;
- the energy terminal can show translucent segmented/grid/circuit structure;
- the physical tail volume must remain present and readable before the digital terminal.

Do not turn the entire tail into a detached energy ribbon, nor remove the physical tail base.

## 8. Style boundary

The original visual language is stylized 2D fantasy concept art with readable canine anatomy and restrained sci-fi/digital accents.

Allowed production simplification:

- fewer fur strands at desktop scale;
- simplified glow geometry;
- reduced marking density in low-power mode;
- animation-friendly shape cleanup.

Not allowed:

- chibi/kitten skull proportions;
- plush-toy anatomy;
- cat muzzle/whisker-face reinterpretation;
- pure-white + gold redesign;
- replacing the dark horn pair with small decorative nubs;
- replacing cyan circuitry with generic magical ornaments;
- blindly mirroring semantic eye/horn identity.

## 9. Production normalization is subordinate to the images

The 1024×1024 authoring canvas, 512×512 runtime cells, pivots, anchors and landmark coordinates are engineering contracts. They must **fit the source identity**.

If an anchor or landmark would require changing Lenvu's visible anatomy to match a number, the number must be revised. The source identity is not distorted to satisfy an inferred coordinate.

## 10. Candidate acceptance rule

A generated/redrawn canonical master candidate is accepted only if visual comparison confirms all of the following:

- species/silhouette matches the primary anatomy sheet;
- head-to-body ratio remains non-chibi;
- muzzle length and canine facial structure match;
- slate-gray/white fur distribution remains recognizable;
- horn pair, ears and neck/chest silhouette match;
- right-eye cyan / left-eye violet are correct;
- left-horn gold accent is correct;
- forehead glyph is preserved;
- Lumen-Code placement remains structurally related to the original;
- physical tail base plus cyan digital terminal are preserved;
- the candidate works at desktop scale without redesigning the character.

If any of these fail, the candidate is rejected; it does not become a new reference source.
