import { invoke } from '@tauri-apps/api/core';

export interface StartupStatus {
  supported: boolean;
  enabled: boolean;
}

export interface PrivacyRulesSnapshot {
  excludedApps: string[];
  failClosed: boolean;
}

export const fallbackStartupStatus: StartupStatus = {
  supported: false,
  enabled: false,
};

export const fallbackPrivacyRules: PrivacyRulesSnapshot = {
  excludedApps: [],
  failClosed: true,
};

export async function getStartupStatus(): Promise<StartupStatus> {
  return invoke<StartupStatus>('startup_get');
}

export async function setStartupEnabled(enabled: boolean): Promise<StartupStatus> {
  return invoke<StartupStatus>('startup_set', { enabled });
}

export async function getPrivacyRules(): Promise<PrivacyRulesSnapshot> {
  return invoke<PrivacyRulesSnapshot>('privacy_get');
}

export async function addPrivacyExcludedApp(appId: string): Promise<PrivacyRulesSnapshot> {
  return invoke<PrivacyRulesSnapshot>('privacy_add_excluded_app', { appId });
}

export async function removePrivacyExcludedApp(appId: string): Promise<PrivacyRulesSnapshot> {
  return invoke<PrivacyRulesSnapshot>('privacy_remove_excluded_app', { appId });
}
