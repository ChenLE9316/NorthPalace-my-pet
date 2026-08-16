export type Locomotion = 'stationary' | 'walk' | 'run' | 'jump';
export type Facing = 'left' | 'right';
export type Posture = 'stand' | 'sit' | 'lie' | 'sleep';
export type Attention = 'idle' | 'user' | 'cursor' | 'window' | 'object';
export type Emotion = 'calm' | 'curious' | 'happy' | 'shy' | 'concerned' | 'sleepy' | 'focused';
export type PetMode = 'ambient' | 'focus_guard' | 'do_not_disturb' | 'play';
export type CognitionState = 'idle' | 'listening' | 'thinking' | 'speaking' | 'remembering';

export interface PetState {
  locomotion: Locomotion;
  facing: Facing;
  posture: Posture;
  attention: Attention;
  emotion: Emotion;
  mode: PetMode;
  cognition: CognitionState;
  energy: number;
  curiosity: number;
  bond: number;
  sleepPressure: number;
  userIdleMs: number;
  aiAvailable: boolean;
}

export type BehaviorKind =
  | 'ambient_idle'
  | 'explore'
  | 'observe_user'
  | 'receive_pet'
  | 'play'
  | 'settle_to_rest'
  | 'sleep'
  | 'wake'
  | 'focus_guard';

export interface BehaviorIntent {
  kind: BehaviorKind;
  priority: number;
  remainingMs: number;
  interruptible: boolean;
  animation: string;
}

export type RuntimeHealth = 'ready' | 'degraded' | 'recovering' | 'error';

export interface PetRuntimeSnapshot {
  health: RuntimeHealth;
  sequence: number;
  state: PetState;
  behavior: BehaviorIntent | null;
}

export type PetInteraction =
  | 'hover'
  | 'hover_end'
  | 'touch'
  | 'pet'
  | 'play'
  | 'focus_start'
  | 'focus_stop'
  | 'user_returned';
