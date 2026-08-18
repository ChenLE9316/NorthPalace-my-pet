<script lang="ts">
  import { resolveAnimation } from '../../pet/animation';
  import type { PetRuntimeSnapshot } from '../../types';
  import type { DisplayContext } from '../../window/runtime';

  export let snapshot: PetRuntimeSnapshot;
  export let displayContext: DisplayContext;
  export let onPet: () => void;
  export let onPlay: () => void;
  export let onToggleFocus: () => void;
</script>

<section class="companion-section home-section" role="tabpanel" aria-label="Home">
  <div class="section-heading section-heading--compact">
    <div>
      <strong>Companion</strong>
      <span>即時互動留在這裡；Memory / Activity / Settings 按需載入。</span>
    </div>
  </div>

  <section class="actions">
    <button onclick={onPet}>摸摸</button>
    <button onclick={onPlay}>玩耍</button>
    <button onclick={onToggleFocus}>
      {snapshot.state.mode === 'focus_guard' ? '離開專注' : 'Focus Guard'}
    </button>
  </section>

  <section class="status-grid">
    <article><span>Posture</span><b>{snapshot.state.posture}</b></article>
    <article><span>Attention</span><b>{snapshot.state.attention}</b></article>
    <article><span>Emotion</span><b>{snapshot.state.emotion}</b></article>
    <article><span>Cognition</span><b>{snapshot.state.cognition}</b></article>
  </section>

  <section class="status-grid status-grid--technical">
    <article><span>Animation</span><b>{resolveAnimation(snapshot)}</b></article>
    <article><span>Sequence</span><b>{snapshot.sequence}</b></article>
    <article><span>Panel DPI</span><b>{displayContext.scaleFactor.toFixed(2)}×</b></article>
    <article><span>Displays</span><b>{displayContext.monitorCount}</b></article>
  </section>

  <section class="concept-card">
    <img src="/lenvu-system-overview.webp" alt="NorthPalace-my-pet UI/UX and architecture concept" />
    <div>
      <strong>Pet first, AI second.</strong>
      <p>Pet Runtime、互動、記憶與 Activity 都能在 MiniCPM5-1B 未載入時獨立運作。</p>
    </div>
  </section>
</section>
