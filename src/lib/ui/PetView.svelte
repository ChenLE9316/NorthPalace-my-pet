<script lang="ts">
  import { onMount } from 'svelte';
  import type { PetInteraction } from '../types';
  import { lenvuManifest } from '../pet/manifest';
  import { fallbackSnapshot, getPetSnapshot, interact } from '../pet/runtime';
  import { PetRenderer } from '../pet/renderer';
  import {
    configurePetHitRegions,
    toggleCompanionWindow,
    type CursorHitRegion,
  } from '../window/runtime';

  let snapshot = fallbackSnapshot;
  let animation = 'idle';
  let snapshotTimer: number | undefined;
  let petCanvas: HTMLDivElement;
  let panelHandle: HTMLButtonElement;
  let renderer: PetRenderer | null = null;

  async function refresh() {
    snapshot = await getPetSnapshot();
    renderer?.update(snapshot);
    animation = renderer?.currentAnimation() ?? animation;
  }

  async function send(kind: PetInteraction) {
    await interact(kind);
    await refresh();
  }

  async function handlePetPointerDown(event: PointerEvent) {
    const zone = renderer?.hitTest(event.clientX, event.clientY);
    if (!zone) return;

    if (zone === 'head') {
      await send('pet');
      return;
    }

    await send('touch');
  }

  function bubbleText() {
    if (snapshot.health !== 'ready') return 'Pet Runtime 正在恢復。';
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
    const { posture, emotion, attention } = snapshot.state;
    return `${posture} · ${emotion} · ${attention}`;
  }

  function configureNativeHitTest() {
    if (!petCanvas || !panelHandle || window.innerWidth <= 0 || window.innerHeight <= 0) return;

    const petBounds = petCanvas.getBoundingClientRect();
    const regions: CursorHitRegion[] = lenvuManifest.hitZones.map((zone) => ({
      shape: 'ellipse',
      cx: (petBounds.left + zone.cx * petBounds.width) / window.innerWidth,
      cy: (petBounds.top + zone.cy * petBounds.height) / window.innerHeight,
      rx: (zone.rx * petBounds.width) / window.innerWidth,
      ry: (zone.ry * petBounds.height) / window.innerHeight,
    }));

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

  onMount(() => {
    let disposed = false;
    let hitTestFrame = 0;

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

    void refresh();
    snapshotTimer = window.setInterval(() => void refresh(), 500);

    return () => {
      disposed = true;
      window.clearInterval(snapshotTimer);
      window.cancelAnimationFrame(hitTestFrame);
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
    onpointerdown={(event) => void handlePetPointerDown(event)}
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
    class:visible={snapshot.state.emotion === 'happy' || snapshot.state.mode === 'focus_guard' || snapshot.health !== 'ready'}
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
