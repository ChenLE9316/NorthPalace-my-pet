<script lang="ts">
  import { onMount } from 'svelte';
  import type { PetInteraction } from '../types';
  import { fallbackSnapshot, getPetSnapshot, interact } from '../pet/runtime';
  import { PetRenderer } from '../pet/renderer';
  import { toggleCompanionWindow } from '../window/runtime';

  let snapshot = fallbackSnapshot;
  let animation = 'idle';
  let snapshotTimer: number | undefined;
  let petCanvas: HTMLDivElement;
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

  onMount(() => {
    let disposed = false;

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
    })();

    void refresh();
    snapshotTimer = window.setInterval(() => void refresh(), 500);

    return () => {
      disposed = true;
      window.clearInterval(snapshotTimer);
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
    onclick={() => void toggleCompanionWindow()}
    aria-label="開啟 Lenvu Companion"
  >☾</button>
</main>
