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
