import { lenvuManifest, type LenvuHitZoneId } from './manifest';

export interface RectLike {
  left: number;
  top: number;
  width: number;
  height: number;
}

export interface NormalizedPoint {
  x: number;
  y: number;
}

export function normalizePoint(clientX: number, clientY: number, bounds: RectLike): NormalizedPoint | null {
  if (bounds.width <= 0 || bounds.height <= 0) return null;

  const x = (clientX - bounds.left) / bounds.width;
  const y = (clientY - bounds.top) / bounds.height;
  if (x < 0 || x > 1 || y < 0 || y > 1) return null;
  return { x, y };
}

export function hitTestLenvu(clientX: number, clientY: number, bounds: RectLike): LenvuHitZoneId | null {
  const point = normalizePoint(clientX, clientY, bounds);
  if (!point) return null;

  for (const zone of lenvuManifest.hitZones) {
    const dx = (point.x - zone.cx) / zone.rx;
    const dy = (point.y - zone.cy) / zone.ry;
    if (dx * dx + dy * dy <= 1) return zone.id;
  }

  return null;
}
