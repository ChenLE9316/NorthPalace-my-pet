# Desktop Window, DPI and Hit-Testing Model

## Scope

NorthPalace-my-pet is a Windows-first desktop pet. The `pet` overlay is not a normal app window: Lenvu occupies desktop space, stays visually lightweight and preserves access to applications underneath transparent regions.

## Current window topology

The desktop shell currently uses two real Tauri windows:

1. `pet` — transparent, always-on-top, skip-taskbar PixiJS creature overlay;
2. `companion` — independent Svelte management/status window that can be shown or hidden without stopping Pet Runtime.

Future logical roles may add a detached bubble/settings/debug window only when there is a concrete UX or performance reason. The Pet Runtime must never depend on any one WebView remaining visible.

## Display context

The Rust Windows platform layer exposes physical display/window facts through `get_display_context`:

- monitor name;
- monitor count;
- monitor physical bounds;
- monitor work area excluding taskbar;
- monitor/window scale factor;
- pet-window physical position;
- pet-window physical size.

Physical coordinates stay at the platform boundary. Renderer/UI code converts them only where required.

## DPI and monitor observation

Never assume 96 DPI or scale factor 1.0. The pet may be explicitly dragged between displays with different scaling.

The presentation layer subscribes to native/Tauri window-moved and scale-factor-changed events and debounces display-context refresh. After a move or scale transition it re-evaluates display context and normalized native hit regions.

```text
window moved / scale changed
          |
          v
80 ms debounced refresh
          |
          +--> current display context
          `--> semantic hit-region refresh
```

The autonomous motion controller independently re-reads current monitor, work area and scale while it is moving, so movement does not rely on stale WebView state.

## Work area

Autonomous movement stays inside the current monitor work area rather than covering the taskbar. Explicit user dragging is intentionally allowed to cross normal autonomous boundaries; after the user drops Lenvu, autonomous motion remains stationary until Pet Brain selects movement again.

Multi-monitor **autonomous** traversal remains a separate policy decision. Explicit user drag across monitors is already supported by the native window drag path.

## Selective click-through

Transparent pixels must not make the entire desktop rectangle interactive.

Current pipeline:

```text
Lenvu manifest hit zones
        |
        +-- head
        +-- body
        `-- tail
        |
        v
facing mirror + CSS/window transform
        |
        v
normalized native hit regions
        |
        v
Windows cursor passthrough controller
```

Outside those regions (plus the companion handle), pointer input passes through to the desktop/application beneath. A native cursor sensor can restore interaction when the cursor re-enters a semantic pet region, avoiding the trap of permanently ignoring the whole WebView.

Coarse semantic geometry is intentional for the foundation. Production sprite atlases may later provide animation-specific masks without changing Pet Brain.

## Pick-up / drag interaction

Dragging is modeled as a cross-layer interaction rather than a special Window API hidden inside Pet Brain.

```text
pointer down on Lenvu
      |
      +-- release before threshold -> touch / pet
      |
      `-- move >= 8 px
              |
              +--> DomainEvent::PetPickedUp
              |       `--> posture = Held
              |           locomotion = Stationary
              |
              +--> native Tauri window drag
              |
              `--> DomainEvent::PetDropped
                      `--> stable Stand or FocusGuard Sit
```

This keeps ordinary affection interactions easy while still allowing Lenvu to be picked up and moved naturally.

`Held` is a domain posture. Pet Brain knows that Lenvu is being held but never knows about monitor coordinates, Tauri APIs or Windows messages.

While `Held`:

- autonomous movement is paused;
- ambient behavior selection cannot overwrite the posture;
- Focus Guard mode may remain logically active and resumes its sitting presentation after drop;
- AI is not involved.

## Movement controller boundary

Pet Brain decides *why* Lenvu wants to move. The Windows Desktop Movement Controller decides *where/how* that is feasible.

Current native movement behavior:

- `walk` / `run` translate the `pet` window horizontally;
- speed is scaled for current DPI;
- Y stays at the current work-area floor;
- left/right boundaries reverse facing;
- `stationary` / `jump` do not translate the native window;
- a user pick-up forces `stationary`, so autonomous motion cannot fight manual dragging.

## Remaining desktop-space work

- production sprite/atlas bounds and animation-specific hit masks;
- weighted personality/curiosity movement selection;
- explicit multi-monitor autonomous movement policy;
- optional snapping/resting relationships to application-window bounds;
- target-hardware validation for WebView/Pixi/native-motion idle cost.
