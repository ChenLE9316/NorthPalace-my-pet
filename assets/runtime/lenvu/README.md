# Lenvu Runtime Assets

Only optimized production assets that the application may load at runtime belong here.

Planned structure:

- `sprites/` — source frame exports when needed;
- `atlases/` — packed PixiJS sprite atlases;
- `masks/` — frame-aligned interaction/native hit-test masks;
- `effects/` — separable Lumen-Code/focus effects;
- `source-notes/` — provenance and conversion notes for runtime derivatives.

The semantic contract lives in `src/lib/pet/lenvu.manifest.json`. Do not couple Pet Brain code to filenames in this directory.

Current status: production sprite assets are not yet normalized; the renderer intentionally uses a procedural placeholder.
