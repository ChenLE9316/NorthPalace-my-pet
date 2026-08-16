import { invoke } from '@tauri-apps/api/core';
import type { PetInteraction, PetState } from '../types';

export const fallbackState: PetState = {
  activity: 'idle',
  mood: 'calm',
  energy: 0.82,
  curiosity: 0.62,
  bond: 0.2,
  focus: 0.5,
  idleSeconds: 0,
  aiAvailable: false,
};

export async function getPetState(): Promise<PetState> {
  try {
    return await invoke<PetState>('get_pet_state');
  } catch {
    return fallbackState;
  }
}

export async function tickPet(seconds = 1): Promise<PetState> {
  try {
    return await invoke<PetState>('tick_pet', { seconds });
  } catch {
    return fallbackState;
  }
}

export async function interact(kind: PetInteraction): Promise<PetState> {
  try {
    return await invoke<PetState>('pet_interact', { kind });
  } catch {
    return { ...fallbackState, mood: kind === 'pet' ? 'happy' : 'curious' };
  }
}
