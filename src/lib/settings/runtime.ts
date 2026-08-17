import { invoke } from '@tauri-apps/api/core';

export interface StartupStatus {
  supported: boolean;
  enabled: boolean;
}

export const fallbackStartupStatus: StartupStatus = {
  supported: false,
  enabled: false,
};

export async function getStartupStatus(): Promise<StartupStatus> {
  return invoke<StartupStatus>('startup_get');
}

export async function setStartupEnabled(enabled: boolean): Promise<StartupStatus> {
  return invoke<StartupStatus>('startup_set', { enabled });
}
