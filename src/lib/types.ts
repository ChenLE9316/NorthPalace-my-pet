export type PetActivity =
  | 'idle'
  | 'observe'
  | 'walk'
  | 'sit'
  | 'rest'
  | 'sleep'
  | 'focus_guard'
  | 'thinking';

export type Mood = 'calm' | 'curious' | 'happy' | 'sleepy' | 'focused';

export interface PetState {
  activity: PetActivity;
  mood: Mood;
  energy: number;
  curiosity: number;
  bond: number;
  focus: number;
  idleSeconds: number;
  aiAvailable: boolean;
}

export type PetInteraction =
  | 'hover'
  | 'touch'
  | 'pet'
  | 'play'
  | 'focus_start'
  | 'focus_stop'
  | 'user_returned';
