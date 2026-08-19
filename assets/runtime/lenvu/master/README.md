# Lenvu production master workspace

This directory is for **normalized production-master artwork and review candidates**, not concept/reference sheets and not runtime animation frames.

## Current staging artifact

`review/lenvu-source-normalization-contact-sheet-v1.svg` is a **source-derived review aid**. It directly crops the authoritative anatomy reference using the measured source evidence regions; it does not redraw Lenvu, does not create a new identity source and is not the canonical master artwork.

Its purpose is to keep front/profile/back/3⁄4/head/detail evidence visible on one controlled review surface before any normalized production artwork is promoted.

## Promotion stages

```text
reference evidence
    ↓
source-derived normalization review
    ↓
review candidate artwork
    ↓ human + contract QA
approved normalized master views
    ↓
animation source
    ↓
sprite/atlas exports
```

A generated or redrawn image is always a **review candidate first**. It must not be wired into `src/lib/pet/lenvu.manifest.json` as an animation texture.

The review gate is tracked by `assets/runtime/lenvu/source-notes/master-candidate.json`. Identity asymmetry is non-negotiable: Lenvu right eye is cyan, left eye is violet, and the gold crescent/ring belongs to Lenvu's left horn. Blind horizontal mirroring is forbidden.

Approved production views must retain the canonical root/ground/pivot contract defined in `canonical-master.json` and the QA landmarks in `master-landmarks.json`.
