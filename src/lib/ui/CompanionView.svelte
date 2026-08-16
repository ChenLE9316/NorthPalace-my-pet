<script lang="ts">
  import { onMount } from 'svelte';
  import type { PetInteraction } from '../types';
  import { resolveAnimation } from '../pet/animation';
  import { fallbackSnapshot, getPetSnapshot, interact } from '../pet/runtime';
  import {
    fallbackDisplayContext,
    getDisplayContext,
    hideCompanionWindow,
    type DisplayContext,
  } from '../window/runtime';

  let snapshot = fallbackSnapshot;
  let displayContext: DisplayContext = fallbackDisplayContext;
  let snapshotTimer: number | undefined;
  let displayTimer: number | undefined;

  const percent = (value: number) => `${Math.round(value * 100)}%`;

  async function refresh() {
    snapshot = await getPetSnapshot();
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

  onMount(() => {
    void refresh();
    void refreshDisplay();
    snapshotTimer = window.setInterval(() => void refresh(), 500);
    displayTimer = window.setInterval(() => void refreshDisplay(), 2_000);

    return () => {
      window.clearInterval(snapshotTimer);
      window.clearInterval(displayTimer);
    };
  });
</script>

<main class="companion-stage">
  <section class="companion-panel companion-panel--standalone">
    <header>
      <div>
        <strong>Lenvu</strong>
        <span>Neralune Digital Companion · runtime {snapshot.health}</span>
      </div>
      <button onclick={() => void hideCompanionWindow()} aria-label="關閉 Companion">×</button>
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
      <article><span>Animation</span><b>{resolveAnimation(snapshot)}</b></article>
      <article><span>Sequence</span><b>{snapshot.sequence}</b></article>
      <article><span>Panel DPI</span><b>{displayContext.scaleFactor.toFixed(2)}×</b></article>
      <article><span>Displays</span><b>{displayContext.monitorCount}</b></article>
    </section>

    <section class="concept-card">
      <img src="/lenvu-system-overview.webp" alt="NorthPalace-my-pet UI/UX and architecture concept" />
      <div>
        <strong>Pet first, AI second.</strong>
        <p>Pet Overlay 與 Companion Window 已分離。關閉面板不會停止 Rust Pet Runtime，也不會停止 Lenvu 的桌面生命狀態。</p>
      </div>
    </section>
  </section>
</main>
