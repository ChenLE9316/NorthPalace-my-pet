# Desktop Window, DPI and Hit-Testing Model

## Scope

NorthPalace-my-pet is a Windows-first desktop pet. The pet overlay is not a normal resizable app window: it must behave like a creature occupying desktop space while preserving access to the applications underneath it.

## Display context

The Rust Windows platform layer exposes physical display/window facts through `get_display_context`:

- monitor name;
- monitor count;
- monitor physical bounds;
- monitor work area excluding taskbar;
- monitor/window scale factor;
- pet-window physical position;
- pet-window physical client size.

Physical coordinates are kept at the platform boundary. Renderer/UI code may convert them to logical coordinates only when necessary.

## DPI rule

Never assume 96 DPI or scale factor 1.0. The pet may move between displays with different scaling. A future movement controller must refresh geometry when the window changes monitor or the scale factor changes.

```text
physical monitor/window facts
          |
          v
Desktop Space adapter
          |
          +--> logical movement constraints
          +--> renderer scale policy
          `--> hit-region transforms
```

## Work area

Ambient movement should normally stay inside the current monitor work area rather than covering the taskbar. Explicit user dragging may temporarily move Lenvu elsewhere, but the autonomous movement planner should clamp target positions to a safe work-area inset.

## Click-through

Tauri supports whole-window cursor ignore mode, but NorthPalace-my-pet needs selective interaction:

- visible/interactive Lenvu region: receive pointer input;
- transparent area: pass pointer input through to the desktop/app beneath;
- companion/settings panels: ordinary interactive windows.

The project must not solve this by permanently enabling whole-window cursor ignore, because once the whole window is ignored the web layer cannot reliably discover that the cursor entered Lenvu again.

The intended Windows implementation is a native hit-test boundary that can answer transparent vs interactive regions using renderer-provided normalized hit zones. Whole-window `set_ignore_cursor_events` remains useful only for explicit modes such as temporary complete pass-through.

## Hit-region pipeline

```text
Animation ID + pose
        |
        v
Normalized hit zones
        |
        v
scale + anchor + DPI transform
        |
        v
window-local physical regions
        |
        v
native Windows hit test
```

First implementation should use coarse geometric regions, not full alpha masks.

## Window roles

Target logical roles remain:

1. `pet` - transparent always-on-top creature overlay;
2. `bubble` - lightweight temporary context/speech layer;
3. `companion` - status/chat/memory panel;
4. `settings` - privacy/model/performance configuration;
5. `debug` - development-only runtime inspector.

The current V0.2 UI still renders companion content inside the pet WebView as transitional scaffolding. This is not the final window topology.

## Movement controller boundary

Pet Brain decides *why* Lenvu wants to move and emits an intent. A Desktop Movement Controller decides *where/how* that intent is feasible on the current monitor layout.

Pet Brain must not contain monitor coordinates, DPI math or Tauri window APIs.

## Next implementation slice

- define `DesktopPose` and normalized hit zones;
- define autonomous movement target contract;
- clamp movement to the active work area;
- add monitor/DPI change observation;
- implement native selective hit testing;
- then add walk/run window repositioning synchronized with animation.
