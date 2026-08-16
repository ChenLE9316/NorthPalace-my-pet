import { invoke } from '@tauri-apps/api/core';
import type { PetInteraction, PetRuntimeSnapshot } from '../types';

export const fallbackSnapshot: PetRuntimeSnapshot = {
  health: 'degraded',
  sequence: 0,
  state: {
    locomotion: 'stationary',
    posture: 'stand',
    attention: 'idle',
    emotion: 'calm',
    mode: 'ambient',
    cognition: 'idle',
    energy: 0.82,
    curiosity: 0.62,
    bond: 0.2,
    sleepPressure: 0.1,
    userIdleMs: 0,
    aiAvailable: false,
  },
  behavior: null,
};

export async function getPetSnapshot(): Promise<PetRuntimeSnapshot> {
  try {
    return await invoke<PetRuntimeSnapshot>('get_pet_snapshot');
  } catch (error) {
    console.error('Failed to read Pet Runtime snapshot', error);
    return { ...fallbackSnapshot, health: 'error' };
  }
}

export async function interact(kind: PetInteraction): Promise<boolean> {
  try {
    await invoke('pet_interact', { kind });
    return true;
  } catch (error) {
    console.error(`Failed to dispatch pet interaction: ${kind}`, error);
    return false;
  }
}
