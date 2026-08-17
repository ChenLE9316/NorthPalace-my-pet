import { invoke } from '@tauri-apps/api/core';

export type ActiveAppContextState = 'unknown' | 'available' | 'privacy_blocked';
export type AccessibilityContextState = 'disabled' | 'available' | 'unavailable' | 'privacy_blocked';

export interface WindowBounds {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface AccessibilityContext {
  controlTypeId: number | null;
  isEnabled: boolean | null;
  isKeyboardFocusable: boolean | null;
  hasKeyboardFocus: boolean | null;
  isOffscreen: boolean | null;
  isPassword: boolean | null;
  bounds: WindowBounds | null;
}

export interface ScreenContextSnapshot {
  activeAppId: string | null;
  activeAppState: ActiveAppContextState;
  activeWindowBounds: WindowBounds | null;
  accessibilityState: AccessibilityContextState;
  accessibility: AccessibilityContext | null;
  userIdleMs: number;
  localHour: number;
  sequence: number;
}

export const fallbackScreenContext: ScreenContextSnapshot = {
  activeAppId: null,
  activeAppState: 'unknown',
  activeWindowBounds: null,
  accessibilityState: 'disabled',
  accessibility: null,
  userIdleMs: 0,
  localHour: 12,
  sequence: 0,
};

export async function getScreenContext(): Promise<ScreenContextSnapshot> {
  return invoke<ScreenContextSnapshot>('screen_context_get');
}
