import rawManifest from './lenvu.manifest.json';

export type LenvuAnimationId = keyof typeof rawManifest.animations;
export type LenvuHitZoneId = 'head' | 'body' | 'tail';

export interface LenvuAnimationProfile {
  fps: number;
  loop: boolean;
  lowPowerFps: number;
  bodyBob: number;
  sway: number;
  asset: string | null;
}

export interface LenvuHitZone {
  id: LenvuHitZoneId;
  shape: 'ellipse';
  cx: number;
  cy: number;
  rx: number;
  ry: number;
}

export interface LenvuManifest {
  schemaVersion: number;
  character: {
    id: 'lenvu';
    displayName: string;
    species: 'Neralune';
    referenceCanvas: { width: number; height: number };
    anchor: { x: number; y: number };
  };
  render: {
    nominalWidth: number;
    nominalHeight: number;
    idleFrameBudgetFps: number;
    lowPowerFrameBudgetFps: number;
  };
  hitZones: LenvuHitZone[];
  animations: Record<LenvuAnimationId, LenvuAnimationProfile>;
}

const requiredAnimations = [
  'idle',
  'observe',
  'sit',
  'rest',
  'sleep',
  'wake',
  'pet_receive',
  'play',
  'focus_guard',
  'walk',
  'run',
  'jump',
  'thinking',
] as const satisfies readonly LenvuAnimationId[];

function validateManifest(manifest: typeof rawManifest): asserts manifest is typeof rawManifest & LenvuManifest {
  if (manifest.schemaVersion !== 1) {
    throw new Error(`Unsupported Lenvu manifest schema: ${manifest.schemaVersion}`);
  }

  for (const id of requiredAnimations) {
    if (!(id in manifest.animations)) {
      throw new Error(`Missing required Lenvu animation: ${id}`);
    }
  }

  for (const zone of manifest.hitZones) {
    if (zone.shape !== 'ellipse') {
      throw new Error(`Unsupported Lenvu hit-zone shape: ${zone.shape}`);
    }
    if (zone.cx < 0 || zone.cx > 1 || zone.cy < 0 || zone.cy > 1 || zone.rx <= 0 || zone.ry <= 0) {
      throw new Error(`Invalid normalized Lenvu hit zone: ${zone.id}`);
    }
  }
}

validateManifest(rawManifest);

export const lenvuManifest: LenvuManifest = rawManifest as LenvuManifest;

export function isLenvuAnimationId(value: string): value is LenvuAnimationId {
  return value in lenvuManifest.animations;
}

export function animationProfile(id: LenvuAnimationId): LenvuAnimationProfile {
  return lenvuManifest.animations[id];
}
