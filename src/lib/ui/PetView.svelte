<script lang="ts">
  import { onMount } from 'svelte';
  import type { PetInteraction } from '../types';
  import { lenvuManifest, type LenvuHitZoneId } from '../pet/manifest';
  import { fallbackSnapshot, getPetSnapshot, interact } from '../pet/runtime';
  import { PetRenderer } from '../pet/renderer';
  import {
    configurePetHitRegions,
    getDisplayContext,
    observePetDisplayChanges,
    startPetWindowDrag,
    toggleCompanionWindow,
    type CursorHitRegion,
  } from '../window/runtime';

  const DRAG_THRESHOLD_PX = 8;
  const DISPLAY_REFRESH_DEBOUNCE_MS = 80;

  interface PendingPointerGesture {
    pointerId: number;
    startX: number;
    startY: number;
    zone: LenvuHitZoneId;
  }

  let snapshot = fallbackSnapshot;
  let animation = 'idle';
  let snapshotTimer: number | undefined;
  let displayRefreshTimer: number | undefined;
  let petCanvas: HTMLDivElement;
  let panelHandle: HTMLButtonElement;
  let renderer: PetRenderer | null = null;
  let pendingPointer: PendingPointerGesture | null = null;

  async function refresh() {
    const previousFacing = snapshot.state.facing;
    snapshot = await getPetSnapshot();
    renderer?.update(snapshot);
    animation = renderer?.currentAnimation() ?? animation;

    if (snapshot.state.facing !== previousFacing) {
      window.requestAnimationFrame(configureNativeHitTest);
    }
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

  function bubbleText() {
    if (snapshot.health !== 'ready') return 'Pet Runtime 正在恢復。';
    if (snapshot.state.posture === 'held') return '欸？被抱起來了。';
    if (snapshot.state.mode === 'focus_guard') return 'Focus Guard 已啟動。';
    switch (snapshot.behavior?.kind) {
      case 'receive_pet': return '嗯……再摸一下。';
      case 'play': return '一起玩。';
      case 'wake': return '我醒了。';
      case 'sleep': return '晚點叫我……';
      default: return '嗯，我在。';
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
    await getDisplayContext();
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

    void refreshDisplayContext();
    void refresh();
    snapshotTimer = window.setInterval(() => void refresh(), 500);

    return () => {
      disposed = true;
      pendingPointer = null;
      window.clearInterval(snapshotTimer);
      window.clearTimeout(displayRefreshTimer);
      window.cancelAnimationFrame(hitTestFrame);
      stopDisplayObservation?.();
      renderer?.destroy();
      renderer = null;
    };
  });
</script>

<main class="desktop-stage">
  <button
    class="pet"
    aria-label="與 Lenvu 互動"
    ondblclick={() => void toggleCompanionWindow()}
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
    class:visible={snapshot.state.posture === 'held' || snapshot.state.emotion === 'happy' || snapshot.state.mode === 'focus_guard' || snapshot.health !== 'ready'}
  >
    {bubbleText()}
  </div>

  <button
    class="panel-handle"
    bind:this={panelHandle}
    onclick={() => void toggleCompanionWindow()}
    aria-label="開啟 Lenvu Companion"
  >☾</button>
</main>
