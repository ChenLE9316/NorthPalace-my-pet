import { invoke } from '@tauri-apps/api/core';

export type ActiveAppContextState = 'unknown' | 'available' | 'privacy_blocked';

export interface WindowBounds {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface ScreenContextSnapshot {
  activeAppId: string | null;
  activeAppState: ActiveAppContextState;
  activeWindowBounds: WindowBounds | null;
  userIdleMs: number;
  localHour: number;
  sequence: number;
}

export const fallbackScreenContext: ScreenContextSnapshot = {
  activeAppId: null,
  activeAppState: 'unknown',
  activeWindowBounds: null,
  userIdleMs: 0,
  localHour: 12,
  sequence: 0,
};

export async function getScreenContext(): Promise<ScreenContextSnapshot> {
  return invoke<ScreenContextSnapshot>('screen_context_get');
}
