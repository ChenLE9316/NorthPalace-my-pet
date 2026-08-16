<script lang="ts">
  import { onMount } from 'svelte';
  import type { PetState } from './lib/types';
  import { fallbackState, getPetState, interact, tickPet } from './lib/pet/runtime';

  let state: PetState = fallbackState;
  let showPanel = false;
  let timer: number | undefined;

  const percent = (value: number) => `${Math.round(value * 100)}%`;

  async function refresh() {
    state = await getPetState();
  }

  async function pet() {
    state = await interact('pet');
  }

  async function play() {
    state = await interact('play');
  }

  async function toggleFocus() {
    state = await interact(state.activity === 'focus_guard' ? 'focus_stop' : 'focus_start');
  }

  onMount(() => {
    refresh();
    timer = window.setInterval(async () => {
      state = await tickPet(1);
    }, 1000);

    return () => window.clearInterval(timer);
  });
</script>

<main class="desktop-stage">
  <button class="pet" aria-label="摸摸 Lenvu" onclick={pet} ondblclick={() => (showPanel = !showPanel)}>
    <div class:focus={state.activity === 'focus_guard'} class="holo-ring"></div>
    <div class="pet-core">
      <div class="ears">◆　◆</div>
      <div class="eyes">◉　◉</div>
      <div class="name">Lenvu</div>
      <div class="activity">{state.activity}</div>
    </div>
  </button>

  <div class="bubble" class:visible={state.mood === 'happy' || state.activity === 'focus_guard'}>
    {state.activity === 'focus_guard' ? 'Focus Guard 已啟動。' : '嗯，我在。'}
  </div>

  {#if showPanel}
    <aside class="companion-panel">
      <header>
        <div>
          <strong>Lenvu</strong>
          <span>Neralune Digital Companion</span>
        </div>
        <button onclick={() => (showPanel = false)}>×</button>
      </header>

      <section class="status-grid">
        <article><span>Energy</span><b>{percent(state.energy)}</b></article>
        <article><span>Curiosity</span><b>{percent(state.curiosity)}</b></article>
        <article><span>Bond</span><b>{percent(state.bond)}</b></article>
        <article><span>Focus</span><b>{percent(state.focus)}</b></article>
      </section>

      <section class="actions">
        <button onclick={pet}>摸摸</button>
        <button onclick={play}>玩耍</button>
        <button onclick={toggleFocus}>{state.activity === 'focus_guard' ? '離開專注' : 'Focus Guard'}</button>
      </section>

      <section class="concept-card">
        <img src="/lenvu-system-overview.webp" alt="NorthPalace-my-pet UI/UX and architecture concept" />
        <div>
          <strong>Pet first, AI second.</strong>
          <p>即使 AI Brain 關閉，Lenvu 仍是一隻完整的桌面寵物。</p>
        </div>
      </section>
    </aside>
  {/if}

  <button class="panel-handle" onclick={() => (showPanel = !showPanel)} aria-label="開啟 Lenvu 面板">☾</button>
</main>
