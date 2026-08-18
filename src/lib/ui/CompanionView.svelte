<script lang="ts">
  import { onMount } from 'svelte';
  import type { PetInteraction, PetRuntimeSnapshot } from '../types';
  import { fallbackSnapshot, getPetSnapshot, interact, observePetSnapshots } from '../pet/runtime';
  import {
    fallbackDisplayContext,
    getDisplayContext,
    hideCompanionWindow,
    observePetDisplayContext,
    type DisplayContext,
  } from '../window/runtime';
  import ActivitySection from './companion/ActivitySection.svelte';
  import HomeSection from './companion/HomeSection.svelte';
  import MemorySection from './companion/MemorySection.svelte';
  import SettingsSection from './companion/SettingsSection.svelte';

  type CompanionSection = 'home' | 'memory' | 'activity' | 'settings';

  let snapshot = fallbackSnapshot;
  let displayContext: DisplayContext = fallbackDisplayContext;
  let activityRefreshTimer: number | undefined;
  let activityRefreshEpoch = 0;
  let activeSection: CompanionSection = 'home';

  const percent = (value: number) => `${Math.round(value * 100)}%`;

  function applySnapshot(nextSnapshot: PetRuntimeSnapshot) {
    snapshot = nextSnapshot;
  }

  async function refresh() {
    applySnapshot(await getPetSnapshot());
  }

  async function refreshDisplay() {
    displayContext = await getDisplayContext();
  }

  function openSection(section: CompanionSection) {
    activeSection = section;
  }

  async function send(kind: PetInteraction) {
    await interact(kind);
    await refresh();

    if (
      activeSection === 'activity'
      && (kind === 'pet' || kind === 'play' || kind === 'focus_start' || kind === 'focus_stop')
    ) {
      window.clearTimeout(activityRefreshTimer);
      activityRefreshTimer = window.setTimeout(() => {
        activityRefreshEpoch += 1;
      }, 250);
    }
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
    let disposed = false;
    let stopSnapshotObservation: (() => void) | null = null;
    let stopDisplayContextObservation: (() => void) | null = null;

    void observePetSnapshots(applySnapshot)
      .then((stop) => {
        if (disposed) stop();
        else stopSnapshotObservation = stop;
      })
      .catch((error) => console.error('Failed to observe Pet Runtime snapshots', error));

    void observePetDisplayContext((context) => {
      displayContext = context;
    })
      .then((stop) => {
        if (disposed) stop();
        else stopDisplayContextObservation = stop;
      })
      .catch((error) => console.error('Failed to observe pet display context', error));

    void refresh();
    void refreshDisplay();

    return () => {
      disposed = true;
      window.clearTimeout(activityRefreshTimer);
      stopSnapshotObservation?.();
      stopDisplayContextObservation?.();
    };
  });
</script>

<main class="companion-stage">
  <section class="companion-panel companion-panel--standalone">
    <header class="companion-header">
      <div>
        <strong>Lenvu</strong>
        <span>Neralune Digital Companion · runtime {snapshot.health}</span>
      </div>
      <button onclick={() => void hideCompanionWindow()} aria-label="關閉 Companion">×</button>
    </header>

    <section class="status-grid status-grid--summary" aria-label="Lenvu 即時狀態摘要">
      <article><span>Energy</span><b>{percent(snapshot.state.energy)}</b></article>
      <article><span>Curiosity</span><b>{percent(snapshot.state.curiosity)}</b></article>
      <article><span>Bond</span><b>{percent(snapshot.state.bond)}</b></article>
      <article><span>Sleep</span><b>{percent(snapshot.state.sleepPressure)}</b></article>
    </section>

    <nav class="companion-tabs" role="tablist" aria-label="Companion 功能">
      <button
        role="tab"
        aria-selected={activeSection === 'home'}
        class:active={activeSection === 'home'}
        onclick={() => openSection('home')}
      >Home</button>
      <button
        role="tab"
        aria-selected={activeSection === 'memory'}
        class:active={activeSection === 'memory'}
        onclick={() => openSection('memory')}
      >Memory</button>
      <button
        role="tab"
        aria-selected={activeSection === 'activity'}
        class:active={activeSection === 'activity'}
        onclick={() => openSection('activity')}
      >Activity</button>
      <button
        role="tab"
        aria-selected={activeSection === 'settings'}
        class:active={activeSection === 'settings'}
        onclick={() => openSection('settings')}
      >Settings</button>
    </nav>

    {#if activeSection === 'home'}
      <HomeSection
        {snapshot}
        {displayContext}
        onPet={pet}
        onPlay={play}
        onToggleFocus={toggleFocus}
      />
    {:else if activeSection === 'memory'}
      <MemorySection />
    {:else if activeSection === 'activity'}
      <ActivitySection refreshEpoch={activityRefreshEpoch} />
    {:else}
      <SettingsSection />
    {/if}
  </section>
</main>