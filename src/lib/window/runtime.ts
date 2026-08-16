import { invoke } from '@tauri-apps/api/core';

export interface PhysicalPoint {
  x: number;
  y: number;
}

export interface PhysicalExtent {
  width: number;
  height: number;
}

export interface PhysicalArea {
  position: PhysicalPoint;
  size: PhysicalExtent;
}

export interface DisplayContext {
  scaleFactor: number;
  monitorName: string | null;
  monitorCount: number;
  monitorBounds: PhysicalArea | null;
  workArea: PhysicalArea | null;
  windowPosition: PhysicalPoint;
  windowSize: PhysicalExtent;
}

export type CursorHitRegion =
  | { shape: 'ellipse'; cx: number; cy: number; rx: number; ry: number }
  | { shape: 'rect'; x: number; y: number; width: number; height: number };

export const fallbackDisplayContext: DisplayContext = {
  scaleFactor: 1,
  monitorName: null,
  monitorCount: 1,
  monitorBounds: null,
  workArea: null,
  windowPosition: { x: 0, y: 0 },
  windowSize: { width: 0, height: 0 },
};

export async function getDisplayContext(): Promise<DisplayContext> {
  try {
    return await invoke<DisplayContext>('get_display_context');
  } catch (error) {
    console.error('Failed to read display context', error);
    return fallbackDisplayContext;
  }
}

export async function toggleCompanionWindow(): Promise<boolean> {
  try {
    return await invoke<boolean>('toggle_companion_window');
  } catch (error) {
    console.error('Failed to toggle companion window', error);
    return false;
  }
}

export async function hideCompanionWindow(): Promise<void> {
  try {
    await invoke('hide_companion_window');
  } catch (error) {
    console.error('Failed to hide companion window', error);
  }
}

export async function configurePetHitRegions(regions: CursorHitRegion[]): Promise<void> {
  try {
    await invoke('configure_pet_hit_regions', { regions });
  } catch (error) {
    // The command is Windows-only. Keeping this non-fatal preserves safe non-passthrough
    // behavior on unsupported/dev environments.
    console.debug('Native pet hit testing is unavailable', error);
  }
}
