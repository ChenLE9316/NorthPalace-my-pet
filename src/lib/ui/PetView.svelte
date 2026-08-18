<script lang="ts">
  import { onMount } from 'svelte';
  import type { PetInteraction, PetRuntimeSnapshot } from '../types';
  import { resolveBubbleCue, type BubbleCue } from '../pet/bubble';
  import { lenvuManifest, type LenvuHitZoneId } from '../pet/manifest';
  import { fallbackSnapshot, getPetSnapshot, interact, observePetSnapshots } from '../pet/runtime';
  import { PetRenderer } from '../pet/renderer';
  import {
    configurePetHitRegions,
    getDisplayContext,
    observePetDisplayChanges,
    publishPetDisplayContext,
    startPetWindowDrag,
    toggleCompanionWindow,
    type CursorHitRegion,
  } from '../window/runtime';

  const DRAG_THRESHOLD_PX = 8;
  const DISPLAY_REFRESH_DEBOUNCE_MS = 80;
  const BUBBLE_REPEAT_COOLDOWN_MS = 12_000;

  interface PendingPointerGesture {
    pointerId: number;
    startX: number;
    startY: number;
    zone: LenvuHitZoneId;
  }

  let snapshot = fallbackSnapshot;
  let animation = 'idle';
  let displayRefreshTimer: number | undefined;
  let bubbleTimer: number | undefined;
  let petCanvas: HTMLDivElement;
  let panelHandle: HTMLButtonElement;
  let renderer: PetRenderer | null = null;
  let pendingPointer: PendingPointerGesture | null = null;
  let bubbleCue: BubbleCue | null = null;
  let hasRuntimeSnapshot = false;
  const lastBubbleShownAt = new Map<string, number>();

  function applySnapshot(nextSnapshot: PetRuntimeSnapshot) {
    const previousFacing = snapshot.state.facing;
    const previousForCue = hasRuntimeSnapshot
      ? snapshot
      : { ...nextSnapshot, health: 'ready' as const };

    hasRuntimeSnapshot = true;
    const cue = resolveBubbleCue(previousForCue, nextSnapshot);
    snapshot = nextSnapshot;
    renderer?.update(snapshot);
    animation = renderer?.currentAnimation() ?? animation;

    if (cue) showBubble(cue);

    if (snapshot.state.facing !== previousFacing) {
      window.requestAnimationFrame(configureNativeHitTest);
    }
  }

  async function refresh() {
    applySnapshot(await getPetSnapshot());
  }

  function showBubble(cue: BubbleCue) {
    const now = Date.now();
    const lastShownAt = lastBubbleShownAt.get(cue.key) ?? 0;
    const cooldown = cue.priority >= 90 ? 2_500 : BUBBLE_REPEAT_COOLDOWN_MS;
    if (now - lastShownAt < cooldown) return;
    if (bubbleCue && bubbleCue.priority > cue.priority) return;

    lastBubbleShownAt.set(cue.key, now);
    bubbleCue = cue;
    window.clearTimeout(bubbleTimer);
    bubbleTimer = window.setTimeout(() => {
      if (bubbleCue?.key === cue.key) bubbleCue = null;
    }, cue.durationMs);
  }

  async function send(kind: PetInteraction) {
    await interact(kind);
    await refresh();
  }

  function handlePetPointerDown(event: PointerEvent) {
    if (event.button !== 0 || !event.isPrimary) return;
    const zone = renderer?.hitTest(event.clientX, event.clientY);
    if (!zone) return;

    pendingPointer = {
      pointerId: event.pointerId,
      startX: event.clientX,
      startY: event.clientY,
      zone,
    };

    const target = event.currentTarget as HTMLButtonElement;
    target.setPointerCapture(event.pointerId);
  }

  async function handlePetPointerMove(event: PointerEvent) {
    const pending = pendingPointer;
    if (!pending || pending.pointerId !== event.pointerId) return;

    const distance = Math.hypot(
      event.clientX - pending.startX,
      event.clientY - pending.startY,
    );
    if (distance < DRAG_THRESHOLD_PX) return;

    pendingPointer = null;
    const target = event.currentTarget as HTMLButtonElement;
    if (target.hasPointerCapture(event.pointerId)) {
      target.releasePointerCapture(event.pointerId);
    }
    event.preventDefault();

    await send('pick_up');
    try {
      await startPetWindowDrag();
    } catch (error) {
      console.error('Failed to start native Lenvu drag', error);
    } finally {
      await send('drop');
      scheduleDisplayRefresh();
    }
  }

  async function handlePetPointerUp(event: PointerEvent) {
    const pending = pendingPointer;
    if (!pending || pending.pointerId !== event.pointerId) return;
    pendingPointer = null;

    const target = event.currentTarget as HTMLButtonElement;
    if (target.hasPointerCapture(event.pointerId)) {
      target.releasePointerCapture(event.pointerId);
    }

    if (pending.zone === 'head') {
      await send('pet');
    } else {
      await send('touch');
    }
  }

  function handlePetPointerCancel(event: PointerEvent) {
    if (pendingPointer?.pointerId === event.pointerId) {
      pendingPointer = null;
    }
    const target = event.currentTarget as HTMLButtonElement;
    if (target.hasPointerCapture(event.pointerId)) {
      target.releasePointerCapture(event.pointerId);
    }
  }

  function stateLabel() {
    const { posture, emotion, attention, facing } = snapshot.state;
    return `${posture} · ${emotion} · ${attention} · ${facing}`;
  }

  function configureNativeHitTest() {
    if (!petCanvas || !panelHandle || window.innerWidth <= 0 || window.innerHeight <= 0) return;

    const petBounds = petCanvas.getBoundingClientRect();
    const facing = snapshot.state.facing;
    const regions: CursorHitRegion[] = lenvuManifest.hitZones.map((zone) => {
      const cx = facing === 'left' ? 1 - zone.cx : zone.cx;
      return {
        shape: 'ellipse',
        cx: (petBounds.left + cx * petBounds.width) / window.innerWidth,
        cy: (petBounds.top + zone.cy * petBounds.height) / window.innerHeight,
        rx: (zone.rx * petBounds.width) / window.innerWidth,
        ry: (zone.ry * petBounds.height) / window.innerHeight,
      };
    });

    const handleBounds = panelHandle.getBoundingClientRect();
    const margin = 5;
    const left = Math.max(0, handleBounds.left - margin);
    const top = Math.max(0, handleBounds.top - margin);
    const right = Math.min(window.innerWidth, handleBounds.right + margin);
    const bottom = Math.min(window.innerHeight, handleBounds.bottom + margin);

    regions.push({
      shape: 'rect',
      x: left / window.innerWidth,
      y: top / window.innerHeight,
      width: Math.max(1, right - left) / window.innerWidth,
      height: Math.max(1, bottom - top) / window.innerHeight,
    });

    void configurePetHitRegions(regions);
  }

  async function refreshDisplayContext() {
    const context = await getDisplayContext();
    try {
      await publishPetDisplayContext(context);
    } catch (error) {
      console.debug('Failed to publish pet display context', error);
    }
    window.requestAnimationFrame(configureNativeHitTest);
  }

  function scheduleDisplayRefresh() {
    window.clearTimeout(displayRefreshTimer);
    displayRefreshTimer = window.setTimeout(
      () => void refreshDisplayContext(),
      DISPLAY_REFRESH_DEBOUNCE_MS,
    );
  }

  onMount(() => {
    let disposed = false;
    let hitTestFrame = 0;
    let stopDisplayObservation: (() => void) | null = null;
    let stopSnapshotObservation: (() => void) | null = null;

    void (async () => {
      const instance = new PetRenderer();
      await instance.mount(petCanvas);
      if (disposed) {
        instance.destroy();
        return;
      }
      renderer = instance;
      renderer.update(snapshot);
      animation = renderer.currentAnimation();
      hitTestFrame = window.requestAnimationFrame(configureNativeHitTest);
    })();

    void observePetDisplayChanges(scheduleDisplayRefresh)
      .then((stop) => {
        if (disposed) stop();
        else stopDisplayObservation = stop;
      })
      .catch((error) => console.error('Failed to observe pet display changes', error));

    void observePetSnapshots(applySnapshot)
      .then((stop) => {
        if (disposed) stop();
        else stopSnapshotObservation = stop;
      })
      .catch((error) => console.error('Failed to observe Pet Runtime snapshots', error));

    void refreshDisplayContext();
    void refresh();

    return () => {
      disposed = true;
      pendingPointer = null;
      window.clearTimeout(displayRefreshTimer);
      window.clearTimeout(bubbleTimer);
      window.cancelAnimationFrame(hitTestFrame);
      stopDisplayObservation?.();
      stopSnapshotObservation?.();
      renderer?.destroy();
      renderer = null;
    };
  });
</script>

<main class="desktop-stage">
  <button
    class="pet"
    aria-label="與 Lenvu 互動"
    onmouseenter={() => void send('hover')}
    onmouseleave={() => void send('hover_end')}
    onpointerdown={handlePetPointerDown}
    onpointermove={(event) => void handlePetPointerMove(event)}
    onpointerup={(event) => void handlePetPointerUp(event)}
    onpointercancel={handlePetPointerCancel}
  >
    <div class="pet-canvas" bind:this={petCanvas}></div>
    <div class="pet-label">
      <div class="name">Lenvu</div>
      <div class="activity">{stateLabel()}</div>
      <div class="activity">anim · {animation}</div>
    </div>
  </button>

  <div
    class="bubble"
    class:visible={bubbleCue !== null}
    class:focus-tone={bubbleCue?.tone === 'focus'}
    class:warning-tone={bubbleCue?.tone === 'warning'}
    aria-live="polite"
  >
    {bubbleCue?.text ?? ''}
  </div>

  <button
    class="panel-handle"
    bind:this={panelHandle}
    onclick={() => void toggleCompanionWindow()}
    aria-label="開啟 Lenvu Companion"
  >☾</button>
</main>