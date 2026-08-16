<script lang="ts">
  import { onMount } from 'svelte';
  import type { PetInteraction } from './lib/types';
  import { fallbackSnapshot, getPetSnapshot, interact } from './lib/pet/runtime';

  let snapshot = fallbackSnapshot;
  let showPanel = false;
  let timer: number | undefined;

  const percent = (value: number) => `${Math.round(value * 100)}%`;

  async function refresh() {
    snapshot = await getPetSnapshot();
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
    void refresh();
    timer = window.setInterval(() => void refresh(), 500);
    return () => window.clearInterval(timer);
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
    <div class:focus={snapshot.state.mode === 'focus_guard'} class="holo-ring"></div>
    <div class="pet-core">
      <div class="ears">◆　◆</div>
      <div class="eyes">◉　◉</div>
      <div class="name">Lenvu</div>
      <div class="activity">{stateLabel()}</div>
      {#if snapshot.behavior}
        <div class="activity">{snapshot.behavior.animation}</div>
      {/if}
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

      <section class="concept-card">
        <img src="/lenvu-system-overview.webp" alt="NorthPalace-my-pet UI/UX and architecture concept" />
        <div>
          <strong>Pet first, AI second.</strong>
          <p>現在由 Rust Pet Runtime 維持 Lenvu 的生命時鐘；UI 只讀 snapshot，不再驅動 Pet Brain。</p>
        </div>
      </section>
    </aside>
  {/if}

  <button class="panel-handle" onclick={() => (showPanel = !showPanel)} aria-label="開啟 Lenvu 面板">☾</button>
</main>
