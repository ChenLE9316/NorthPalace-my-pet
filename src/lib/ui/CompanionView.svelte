<script lang="ts">
  import { onMount } from 'svelte';
  import type { PetInteraction } from '../types';
  import { resolveAnimation } from '../pet/animation';
  import { fallbackSnapshot, getPetSnapshot, interact } from '../pet/runtime';
  import {
    createMemory,
    deleteMemory,
    listMemories,
    searchMemories,
    updateMemory,
    type MemoryKind,
    type MemoryRecord,
  } from '../memory/runtime';
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

  let memories: MemoryRecord[] = [];
  let memoryQuery = '';
  let memoryFilter: MemoryKind | 'all' = 'all';
  let memoryStatus = '';
  let memoryBusy = false;
  let draftContent = '';
  let draftKind: MemoryKind = 'episodic';
  let draftImportance = 0.65;

  const percent = (value: number) => `${Math.round(value * 100)}%`;
  const kindLabels: Record<MemoryKind, string> = {
    episodic: '事件',
    semantic: '事實',
    preference: '偏好',
    relationship: '關係',
  };

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

  async function refreshMemories() {
    memoryBusy = true;
    memoryStatus = '';
    try {
      const query = memoryQuery.trim();
      const records = query
        ? await searchMemories(query, 50)
        : await listMemories(memoryFilter === 'all' ? null : memoryFilter, 50);
      memories = memoryFilter === 'all' || !query
        ? records
        : records.filter((memory) => memory.kind === memoryFilter);
    } catch (error) {
      memoryStatus = error instanceof Error ? error.message : String(error);
      memories = [];
    } finally {
      memoryBusy = false;
    }
  }

  async function remember() {
    if (!draftContent.trim() || memoryBusy) return;
    memoryBusy = true;
    memoryStatus = '';
    try {
      await createMemory({
        kind: draftKind,
        content: draftContent.trim(),
        importance: draftImportance,
      });
      draftContent = '';
      memoryStatus = '已經記住。';
      await refreshMemories();
    } catch (error) {
      memoryStatus = error instanceof Error ? error.message : String(error);
    } finally {
      memoryBusy = false;
    }
  }

  async function saveMemory(memory: MemoryRecord) {
    if (!memory.content.trim() || memoryBusy) return;
    memoryBusy = true;
    memoryStatus = '';
    try {
      await updateMemory(memory.id, {
        kind: memory.kind,
        content: memory.content.trim(),
        importance: memory.importance,
      });
      memoryStatus = '記憶已更新。';
      await refreshMemories();
    } catch (error) {
      memoryStatus = error instanceof Error ? error.message : String(error);
    } finally {
      memoryBusy = false;
    }
  }

  async function removeMemory(memory: MemoryRecord) {
    if (memoryBusy || !window.confirm('確定要讓 Lenvu 忘記這一筆嗎？')) return;
    memoryBusy = true;
    memoryStatus = '';
    try {
      await deleteMemory(memory.id);
      memoryStatus = '已刪除這筆記憶。';
      await refreshMemories();
    } catch (error) {
      memoryStatus = error instanceof Error ? error.message : String(error);
    } finally {
      memoryBusy = false;
    }
  }

  function memoryTime(value: number) {
    return new Date(value).toLocaleString();
  }

  onMount(() => {
    void refresh();
    void refreshDisplay();
    void refreshMemories();
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

    <section class="memory-panel">
      <div class="section-heading">
        <div>
          <strong>Memory</strong>
          <span>本地 SQLite · 手動可檢視／修改／刪除</span>
        </div>
        <button onclick={() => void refreshMemories()} disabled={memoryBusy}>重新整理</button>
      </div>

      <div class="memory-create">
        <textarea bind:value={draftContent} rows="3" placeholder="請 Lenvu 記住這件事……"></textarea>
        <div class="memory-toolbar">
          <select bind:value={draftKind} aria-label="記憶類型">
            <option value="episodic">事件</option>
            <option value="semantic">事實</option>
            <option value="preference">偏好</option>
            <option value="relationship">關係</option>
          </select>
          <label>
            重要度 {Math.round(draftImportance * 100)}%
            <input type="range" min="0" max="1" step="0.05" bind:value={draftImportance} />
          </label>
          <button onclick={() => void remember()} disabled={memoryBusy || !draftContent.trim()}>記住</button>
        </div>
      </div>

      <div class="memory-search">
        <input
          bind:value={memoryQuery}
          onkeydown={(event) => event.key === 'Enter' && void refreshMemories()}
          placeholder="搜尋 Lenvu 的記憶"
          aria-label="搜尋記憶"
        />
        <select bind:value={memoryFilter} onchange={() => void refreshMemories()} aria-label="篩選記憶類型">
          <option value="all">全部</option>
          <option value="episodic">事件</option>
          <option value="semantic">事實</option>
          <option value="preference">偏好</option>
          <option value="relationship">關係</option>
        </select>
        <button onclick={() => void refreshMemories()} disabled={memoryBusy}>搜尋</button>
      </div>

      {#if memoryStatus}
        <p class="memory-status">{memoryStatus}</p>
      {/if}

      <div class="memory-list" aria-busy={memoryBusy}>
        {#if !memoryBusy && memories.length === 0}
          <div class="memory-empty">目前沒有符合條件的長期記憶。</div>
        {/if}

        {#each memories as memory (memory.id)}
          <article class="memory-card">
            <div class="memory-card-meta">
              <select bind:value={memory.kind} aria-label="記憶類型">
                <option value="episodic">事件</option>
                <option value="semantic">事實</option>
                <option value="preference">偏好</option>
                <option value="relationship">關係</option>
              </select>
              <span>{kindLabels[memory.kind]} · {memoryTime(memory.updatedAtMs)}</span>
            </div>
            <textarea bind:value={memory.content} rows="3" aria-label="記憶內容"></textarea>
            <div class="memory-card-actions">
              <label>
                重要度 {Math.round(memory.importance * 100)}%
                <input type="range" min="0" max="1" step="0.05" bind:value={memory.importance} />
              </label>
              <button onclick={() => void saveMemory(memory)} disabled={memoryBusy}>儲存</button>
              <button class="danger" onclick={() => void removeMemory(memory)} disabled={memoryBusy}>忘記</button>
            </div>
          </article>
        {/each}
      </div>
    </section>

    <section class="concept-card">
      <img src="/lenvu-system-overview.webp" alt="NorthPalace-my-pet UI/UX and architecture concept" />
      <div>
        <strong>Pet first, AI second.</strong>
        <p>Pet Overlay 與 Companion Window 已分離。Memory Browser 直接管理本地長期記憶，不需要先載入 MiniCPM5-1B。</p>
      </div>
    </section>
  </section>
</main>
