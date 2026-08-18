import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { PetInteraction, PetRuntimeSnapshot } from '../types';

const PET_RUNTIME_SNAPSHOT_EVENT = 'pet-runtime-snapshot';

export const fallbackSnapshot: PetRuntimeSnapshot = {
  health: 'degraded',
  sequence: 0,
  state: {
    locomotion: 'stationary',
    facing: 'right',
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

export async function observePetSnapshots(
  onSnapshot: (snapshot: PetRuntimeSnapshot) => void,
): Promise<UnlistenFn> {
  return listen<PetRuntimeSnapshot>(PET_RUNTIME_SNAPSHOT_EVENT, (event) => {
    onSnapshot(event.payload);
  });
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
