import type { PetRuntimeSnapshot } from '../types';
import {
  animationProfile as profileFromManifest,
  isLenvuAnimationId,
  type LenvuAnimationId,
  type LenvuAnimationProfile,
} from './manifest';

export type { LenvuAnimationId } from './manifest';
export type AnimationProfile = LenvuAnimationProfile;

export function resolveAnimation(snapshot: PetRuntimeSnapshot): LenvuAnimationId {
  const behaviorAnimation = snapshot.behavior?.animation;
  if (behaviorAnimation && isLenvuAnimationId(behaviorAnimation)) {
    return behaviorAnimation;
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
  return profileFromManifest(id);
}
