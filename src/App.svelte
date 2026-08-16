<script lang="ts">
  import { onMount } from 'svelte';
  import type { PetInteraction } from './lib/types';
  import { fallbackSnapshot, getPetSnapshot, interact } from './lib/pet/runtime';
  import { PetRenderer } from './lib/pet/renderer';
  import {
    fallbackDisplayContext,
    getDisplayContext,
    type DisplayContext,
  } from './lib/window/runtime';

  let snapshot = fallbackSnapshot;
  let displayContext: DisplayContext = fallbackDisplayContext;
  let animation = 'idle';
  let showPanel = false;
  let snapshotTimer: number | undefined;
  let displayTimer: number | undefined;
  let petCanvas: HTMLDivElement;
  let renderer: PetRenderer | null = null;

  const percent = (value: number) => `${Math.round(value * 100)}%`;

  async function refresh() {
    snapshot = await getPetSnapshot();
    renderer?.update(snapshot);
    animation = renderer?.currentAnimation() ?? animation;
  }

  async function refreshDisplay() {
    displayContext = await getDisplayContext();
  }

  async function send(kind: PetInteraction) {
    await interact(kind);
    await refresh();
  }

  async function pet() {
    await send('pet');
  }

  async function play() {
    await send('play');
  }

  async function toggleFocus() {
    await send(snapshot.state.mode === 'focus_guard' ? 'focus_stop' : 'focus_start');
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
    void refreshDisplay();
    snapshotTimer = window.setInterval(() => void refresh(), 500);
    displayTimer = window.setInterval(() => void refreshDisplay(), 2_000);

    return () => {
      disposed = true;
      window.clearInterval(snapshotTimer);
      window.clearInterval(displayTimer);
      renderer?.destroy();
      renderer = null;
    };
  });
</script>

<main class="desktop-stage">
  <button
    class="pet"
    aria-label="摸摸 Lenvu"
    onclick={pet}
    ondblclick={() => (showPanel = !showPanel)}
    onmouseenter={() => void send('hover')}
    onmouseleave={() => void send('hover_end')}
    onpointerdown={() => void send('touch')}
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

  {#if showPanel}
    <aside class="companion-panel">
      <header>
        <div>
          <strong>Lenvu</strong>
          <span>Neralune Digital Companion · runtime {snapshot.health}</span>
        </div>
        <button onclick={() => (showPanel = false)}>×</button>
      </header>

      <section class="status-grid">
        <article><span>Energy</span><b>{percent(snapshot.state.energy)}</b></article>
        <article><span>Curiosity</span><b>{percent(snapshot.state.curiosity)}</b></article>
        <article><span>Bond</span><b>{percent(snapshot.state.bond)}</b></article>
        <article><span>Sleep pressure</span><b>{percent(snapshot.state.sleepPressure)}</b></article>
      </section>

      <section class="actions">
        <button onclick={pet}>摸摸</button>
        <button onclick={play}>玩耍</button>
        <button onclick={toggleFocus}>
          {snapshot.state.mode === 'focus_guard' ? '離開專注' : 'Focus Guard'}
        </button>
      </section>

      <section class="status-grid">
        <article><span>Posture</span><b>{snapshot.state.posture}</b></article>
        <article><span>Attention</span><b>{snapshot.state.attention}</b></article>
        <article><span>Emotion</span><b>{snapshot.state.emotion}</b></article>
        <article><span>Cognition</span><b>{snapshot.state.cognition}</b></article>
      </section>

      <section class="status-grid">
        <article><span>Animation</span><b>{animation}</b></article>
        <article><span>DPI scale</span><b>{displayContext.scaleFactor.toFixed(2)}×</b></article>
        <article><span>Monitor</span><b>{displayContext.monitorName ?? 'unknown'}</b></article>
        <article><span>Displays</span><b>{displayContext.monitorCount}</b></article>
      </section>

      <section class="concept-card">
        <img src="/lenvu-system-overview.webp" alt="NorthPalace-my-pet UI/UX and architecture concept" />
        <div>
          <strong>Pet first, AI second.</strong>
          <p>Rust Pet Runtime 維持生命狀態；PixiJS 專門負責高頻角色渲染，Svelte 只處理面板與互動。</p>
        </div>
      </section>
    </aside>
  {/if}

  <button class="panel-handle" onclick={() => (showPanel = !showPanel)} aria-label="開啟 Lenvu 面板">☾</button>
</main>
