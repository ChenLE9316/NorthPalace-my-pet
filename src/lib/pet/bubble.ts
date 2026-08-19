import type { PetRuntimeSnapshot, RuntimeHealth } from '../types';

export type BubbleTone = 'soft' | 'focus' | 'warning';

export interface BubbleCue {
  key: string;
  text: string;
  tone: BubbleTone;
  durationMs: number;
  priority: number;
}

const healthText: Record<Exclude<RuntimeHealth, 'ready'>, string> = {
  degraded: '我還在，只是有些功能暫時休息。',
  error: '我遇到一點問題，但 Pet Runtime 不會假裝沒事。',
};

export function resolveBubbleCue(
  previous: PetRuntimeSnapshot,
  current: PetRuntimeSnapshot,
): BubbleCue | null {
  if (current.health !== 'ready' && current.health !== previous.health) {
    return {
      key: `health:${current.health}`,
      text: healthText[current.health],
      tone: 'warning',
      durationMs: 4_000,
      priority: 100,
    };
  }

  if (previous.health !== 'ready' && current.health === 'ready') {
    return {
      key: 'health:ready',
      text: '好了，我回來了。',
      tone: 'soft',
      durationMs: 2_200,
      priority: 90,
    };
  }

  if (current.state.posture === 'held' && previous.state.posture !== 'held') {
    return {
      key: 'interaction:held',
      text: '欸？被抱起來了。',
      tone: 'soft',
      durationMs: 1_800,
      priority: 80,
    };
  }

  if (current.state.mode === 'focus_guard' && previous.state.mode !== 'focus_guard') {
    return {
      key: 'mode:focus_guard',
      text: '我幫你守著。專心吧。',
      tone: 'focus',
      durationMs: 2_800,
      priority: 75,
    };
  }

  if (previous.state.mode === 'focus_guard' && current.state.mode === 'ambient') {
    return {
      key: 'mode:focus_end',
      text: '辛苦了。',
      tone: 'soft',
      durationMs: 1_800,
      priority: 60,
    };
  }

  const previousBehavior = previous.behavior?.kind;
  const currentBehavior = current.behavior?.kind;
  if (!currentBehavior || currentBehavior === previousBehavior) return null;

  switch (currentBehavior) {
    case 'receive_pet':
      return {
        key: 'behavior:pet',
        text: '嗯……',
        tone: 'soft',
        durationMs: 1_500,
        priority: 65,
      };
    case 'play':
      return {
        key: 'behavior:play',
        text: '一起玩。',
        tone: 'soft',
        durationMs: 1_700,
        priority: 65,
      };
    case 'wake':
      return {
        key: 'behavior:wake',
        text: '我醒了。',
        tone: 'soft',
        durationMs: 1_600,
        priority: 55,
      };
    case 'sleep':
      return {
        key: 'behavior:sleep',
        text: '我先睡一下……',
        tone: 'soft',
        durationMs: 1_600,
        priority: 40,
      };
    default:
      // Ambient observe/sit/explore and ordinary state changes intentionally stay silent.
      return null;
  }
}
