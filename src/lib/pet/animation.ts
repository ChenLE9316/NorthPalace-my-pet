import type { PetRuntimeSnapshot } from '../types';

export type LenvuAnimationId =
  | 'idle'
  | 'observe'
  | 'sit'
  | 'rest'
  | 'sleep'
  | 'wake'
  | 'pet_receive'
  | 'play'
  | 'focus_guard'
  | 'walk'
  | 'run'
  | 'jump'
  | 'thinking';

export interface AnimationProfile {
  id: LenvuAnimationId;
  fps: number;
  loop: boolean;
  lowPowerFps: number;
  bodyBob: number;
  sway: number;
}

const profiles: Record<LenvuAnimationId, AnimationProfile> = {
  idle: { id: 'idle', fps: 8, loop: true, lowPowerFps: 4, bodyBob: 1.5, sway: 0.01 },
  observe: { id: 'observe', fps: 8, loop: true, lowPowerFps: 4, bodyBob: 1.2, sway: 0.015 },
  sit: { id: 'sit', fps: 8, loop: true, lowPowerFps: 4, bodyBob: 1.0, sway: 0.008 },
  rest: { id: 'rest', fps: 6, loop: true, lowPowerFps: 3, bodyBob: 0.7, sway: 0.004 },
  sleep: { id: 'sleep', fps: 5, loop: true, lowPowerFps: 2, bodyBob: 0.55, sway: 0 },
  wake: { id: 'wake', fps: 12, loop: false, lowPowerFps: 8, bodyBob: 2.6, sway: 0.02 },
  pet_receive: { id: 'pet_receive', fps: 12, loop: true, lowPowerFps: 8, bodyBob: 2.4, sway: 0.018 },
  play: { id: 'play', fps: 16, loop: true, lowPowerFps: 10, bodyBob: 7.5, sway: 0.08 },
  focus_guard: { id: 'focus_guard', fps: 8, loop: true, lowPowerFps: 4, bodyBob: 0.65, sway: 0.004 },
  walk: { id: 'walk', fps: 12, loop: true, lowPowerFps: 8, bodyBob: 3.0, sway: 0.02 },
  run: { id: 'run', fps: 18, loop: true, lowPowerFps: 12, bodyBob: 4.5, sway: 0.03 },
  jump: { id: 'jump', fps: 16, loop: false, lowPowerFps: 10, bodyBob: 9.0, sway: 0.025 },
  thinking: { id: 'thinking', fps: 8, loop: true, lowPowerFps: 4, bodyBob: 0.8, sway: 0.006 },
};

export function resolveAnimation(snapshot: PetRuntimeSnapshot): LenvuAnimationId {
  const behaviorAnimation = snapshot.behavior?.animation;
  if (behaviorAnimation && behaviorAnimation in profiles) {
    return behaviorAnimation as LenvuAnimationId;
  }

  if (snapshot.state.mode === 'focus_guard') return 'focus_guard';
  if (snapshot.state.cognition === 'thinking') return 'thinking';

  switch (snapshot.state.locomotion) {
    case 'run': return 'run';
    case 'walk': return 'walk';
    case 'jump': return 'jump';
    default: break;
  }

  switch (snapshot.state.posture) {
    case 'sleep': return 'sleep';
    case 'lie': return 'rest';
    case 'sit': return 'sit';
    default: break;
  }

  if (snapshot.state.attention !== 'idle') return 'observe';
  return 'idle';
}

export function animationProfile(id: LenvuAnimationId): AnimationProfile {
  return profiles[id];
}
