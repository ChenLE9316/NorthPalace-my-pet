<script lang="ts">
  import { onMount } from 'svelte';
  import type { PetInteraction } from '../types';
  import { resolveAnimation } from '../pet/animation';
  import { fallbackSnapshot, getPetSnapshot, interact } from '../pet/runtime';
  import {
    createMemory,
    deleteMemory,
    getActivity,
    listActivity,
    listMemories,
    searchMemories,
    updateMemory,
    type ActivityHistoryRecord,
    type MemoryKind,
    type MemoryRecord,
  } from '../memory/runtime';
  import {
    fallbackStartupStatus,
    getStartupStatus,
    setStartupEnabled,
    type StartupStatus,
  } from '../settings/runtime';
  import {
    fallbackDisplayContext,
    getDisplayContext,
    hideCompanionWindow,
    type DisplayContext,
  } from '../window/runtime';

  type CompanionSection = 'home' | 'memory' | 'activity' | 'settings';

  let snapshot = fallbackSnapshot;
  let displayContext: DisplayContext = fallbackDisplayContext;
  let snapshotTimer: number | undefined;
  let displayTimer: number | undefined;
  let activityRefreshTimer: number | undefined;
  let activeSection: CompanionSection = 'home';

  let memories: MemoryRecord[] = [];
  let memoryQuery = '';
  let memoryFilter: MemoryKind | 'all' = 'all';
  let memoryStatus = '';
  let memoryBusy = false;
  let memoryLoaded = false;
  let draftContent = '';
  let draftKind: MemoryKind = 'episodic';
  let draftImportance = 0.65;
  let memorySources = new Map<number, ActivityHistoryRecord>();

  let activities: ActivityHistoryRecord[] = [];
  let historyBusy = false;
  let historyLoaded = false;
  let historyStatus = '';

  let startupStatus: StartupStatus = fallbackStartupStatus;
  let startupBusy = false;
  let startupLoaded = false;
  let startupMessage = '';

  const percent = (value: number) => `${Math.round(value * 100)}%`;
  const kindLabels: Record<MemoryKind, string> = {
    episodic: '事件',
    semantic: '事實',
    preference: '偏好',
    relationship: '關係',
  };
  const activityLabels: Record<string, string> = {
    user_returned: '你回到電腦前',
    pet_petted: '你摸了 Lenvu',
    pet_play: '你和 Lenvu 玩耍',
    focus_started: 'Focus Guard 開始',
    focus_ended: 'Focus Guard 結束',
  };
  const relationshipLabels: Record<string, string> = {
    reunion: '重逢',
    affection: '親近',
    play: '玩耍',
  };

  async function refresh() {
    snapshot = await getPetSnapshot();
  }

  async function refreshDisplay() {
    displayContext = await getDisplayContext();
  }

  async function openSection(section: CompanionSection) {
    activeSection = section;
    if (section === 'memory' && !memoryLoaded) {
      await refreshMemories();
    } else if (section === 'activity' && !historyLoaded) {
      await refreshActivity();
    } else if (section === 'settings' && !startupLoaded) {
      await refreshStartup();
    }
  }

  async function send(kind: PetInteraction) {
    await interact(kind);
    await refresh();
    if (kind === 'pet' || kind === 'play' || kind === 'focus_start' || kind === 'focus_stop') {
      historyLoaded = false;
      if (activeSection === 'activity') {
        window.clearTimeout(activityRefreshTimer);
        activityRefreshTimer = window.setTimeout(() => void refreshActivity(), 250);
      }
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
      await refreshMemorySources(memories);
    } catch (error) {
      memoryStatus = error instanceof Error ? error.message : String(error);
      memories = [];
      memorySources = new Map();
    } finally {
      memoryLoaded = true;
      memoryBusy = false;
    }
  }

  async function refreshMemorySources(records: MemoryRecord[]) {
    const sourceIds = [...new Set(
      records
        .map((memory) => memory.sourceEventId)
        .filter((id): id is number => id !== null),
    )];
    if (sourceIds.length === 0) {
      memorySources = new Map();
      return;
    }

    const resolved = await Promise.all(
      sourceIds.map(async (id) => [id, await getActivity(id)] as const),
    );
    memorySources = new Map(
      resolved
        .filter((entry): entry is readonly [number, ActivityHistoryRecord] => entry[1] !== null),
    );
  }

  async function refreshActivity() {
    historyBusy = true;
    historyStatus = '';
    try {
      activities = await listActivity(40);
    } catch (error) {
      historyStatus = error instanceof Error ? error.message : String(error);
      activities = [];
    } finally {
      historyLoaded = true;
      historyBusy = false;
    }
  }

  async function refreshStartup() {
    startupBusy = true;
    startupMessage = '';
    try {
      startupStatus = await getStartupStatus();
    } catch (error) {
      startupStatus = fallbackStartupStatus;
      startupMessage = error instanceof Error ? error.message : String(error);
    } finally {
      startupLoaded = true;
      startupBusy = false;
    }
  }

  async function toggleStartup(event: Event) {
    const requested = (event.currentTarget as HTMLInputElement).checked;
    if (startupBusy) return;

    startupBusy = true;
    startupMessage = '';
    try {
      startupStatus = await setStartupEnabled(requested);
      startupMessage = startupStatus.enabled
        ? '已啟用：登入 Windows 後自動啟動 Lenvu。'
        : '已關閉：Lenvu 不會跟隨 Windows 登入自動啟動。';
    } catch (error) {
      startupMessage = error instanceof Error ? error.message : String(error);
      try {
        startupStatus = await getStartupStatus();
      } catch {
        startupStatus = fallbackStartupStatus;
      }
    } finally {
      startupLoaded = true;
      startupBusy = false;
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

  function activityLabel(activity: ActivityHistoryRecord) {
    return activityLabels[activity.eventType] ?? activity.eventType.replaceAll('_', ' ');
  }

  function relationshipLabel(activity: ActivityHistoryRecord) {
    if (!activity.relationshipKind) return '';
    return relationshipLabels[activity.relationshipKind] ?? activity.relationshipKind;
  }

  function bondDeltaLabel(activity: ActivityHistoryRecord) {
    const delta = activity.bondDelta ?? 0;
    if (Math.abs(delta) < 0.0001) return '';
    const value = Math.round(delta * 1000) / 10;
    return `${value > 0 ? '+' : ''}${value}% bond`;
  }

  function memorySourceLabel(memory: MemoryRecord) {
    if (memory.sourceEventId === null) return '來源 · 手動建立';
    const source = memorySources.get(memory.sourceEventId);
    if (!source) return `來源事件 #${memory.sourceEventId}`;
    return `來源 · ${activityLabel(source)} · ${memoryTime(source.createdAtMs)}`;
  }

  onMount(() => {
    void refresh();
    void refreshDisplay();
    snapshotTimer = window.setInterval(() => void refresh(), 500);
    displayTimer = window.setInterval(() => void refreshDisplay(), 2_000);

    return () => {
      window.clearInterval(snapshotTimer);
      window.clearInterval(displayTimer);
      window.clearTimeout(activityRefreshTimer);
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
        onclick={() => void openSection('home')}
      >Home</button>
      <button
        role="tab"
        aria-selected={activeSection === 'memory'}
        class:active={activeSection === 'memory'}
        onclick={() => void openSection('memory')}
      >Memory</button>
      <button
        role="tab"
        aria-selected={activeSection === 'activity'}
        class:active={activeSection === 'activity'}
        onclick={() => void openSection('activity')}
      >Activity</button>
      <button
        role="tab"
        aria-selected={activeSection === 'settings'}
        class:active={activeSection === 'settings'}
        onclick={() => void openSection('settings')}
      >Settings</button>
    </nav>

    {#if activeSection === 'home'}
      <section class="companion-section home-section" role="tabpanel" aria-label="Home">
        <div class="section-heading section-heading--compact">
          <div>
            <strong>Companion</strong>
            <span>即時互動留在這裡；Memory / Activity / Settings 按需載入。</span>
          </div>
        </div>

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
    {:else if activeSection === 'memory'}
      <section class="companion-section" role="tabpanel" aria-label="Memory">
        <section class="memory-panel">
          <div class="section-heading">
            <div>
              <strong>Memory</strong>
              <span>本地 SQLite · 可搜尋、修改、查看來源與刪除</span>
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
            {#if memoryBusy && !memoryLoaded}
              <div class="memory-empty">正在讀取本地記憶……</div>
            {:else if memories.length === 0}
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
                <p class="memory-provenance">{memorySourceLabel(memory)}</p>
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
      </section>
    {:else if activeSection === 'activity'}
      <section class="companion-section" role="tabpanel" aria-label="Activity">
        <section class="activity-panel">
          <div class="section-heading">
            <div>
              <strong>Activity History</strong>
              <span>只顯示低頻、有意義的互動；不是桌面監控紀錄</span>
            </div>
            <button onclick={() => void refreshActivity()} disabled={historyBusy}>重新整理</button>
          </div>

          {#if historyStatus}
            <p class="memory-status">{historyStatus}</p>
          {/if}

          <div class="activity-list" aria-busy={historyBusy}>
            {#if historyBusy && !historyLoaded}
              <div class="memory-empty">正在讀取 Activity History……</div>
            {:else if activities.length === 0}
              <div class="memory-empty">目前還沒有可顯示的活動紀錄。</div>
            {/if}

            {#each activities as activity (activity.id)}
              <article class="activity-row">
                <div>
                  <strong>{activityLabel(activity)}</strong>
                  <span>{activity.category}{relationshipLabel(activity) ? ` · ${relationshipLabel(activity)}` : ''}</span>
                </div>
                <div class="activity-meta">
                  {#if bondDeltaLabel(activity)}
                    <b>{bondDeltaLabel(activity)}</b>
                  {/if}
                  <time>{memoryTime(activity.createdAtMs)}</time>
                </div>
              </article>
            {/each}
          </div>
        </section>
      </section>
    {:else}
      <section class="companion-section" role="tabpanel" aria-label="Settings">
        <section class="settings-panel">
          <div class="section-heading">
            <div>
              <strong>Settings</strong>
              <span>只有打開此分頁才讀取較深層的系統設定。</span>
            </div>
            <button onclick={() => void refreshStartup()} disabled={startupBusy}>重新讀取</button>
          </div>

          <article class="setting-card">
            <div class="setting-copy">
              <strong>Windows 開機啟動</strong>
              <span>登入 Windows 後自動啟動 Lenvu。預設關閉，只有你明確開啟才會註冊。</span>
            </div>

            {#if startupBusy && !startupLoaded}
              <span class="setting-state">正在讀取 Windows 狀態……</span>
            {:else}
              <label class="setting-toggle">
                <input
                  type="checkbox"
                  checked={startupStatus.enabled}
                  disabled={startupBusy || !startupStatus.supported}
                  onchange={(event) => void toggleStartup(event)}
                />
                <span>
                  {startupStatus.supported
                    ? startupStatus.enabled ? '已啟用' : '已關閉'
                    : '目前平台不支援'}
                </span>
              </label>
            {/if}
          </article>

          {#if startupMessage}
            <p class="memory-status">{startupMessage}</p>
          {/if}

          <div class="settings-note">
            這個開關只控制是否跟隨 Windows 登入啟動。Lenvu 啟動後仍先維持桌寵 Overlay 與 Tray；Companion 視窗不會因開機自啟而主動跳出。
          </div>
        </section>
      </section>
    {/if}
  </section>
</main>
