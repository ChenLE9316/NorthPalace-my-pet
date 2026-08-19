<script lang="ts">
  import { onMount } from 'svelte';
  import {
    createMemory,
    deleteMemory,
    getActivity,
    listMemories,
    searchMemories,
    updateMemory,
    type ActivityHistoryRecord,
    type MemoryKind,
    type MemoryRecord,
  } from '../../memory/runtime';

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

  function memorySourceLabel(memory: MemoryRecord) {
    if (memory.sourceEventId === null) return '來源 · 手動建立';
    const source = memorySources.get(memory.sourceEventId);
    if (!source) return `來源事件 #${memory.sourceEventId}`;
    return `來源 · ${activityLabel(source)} · ${memoryTime(source.createdAtMs)}`;
  }

  onMount(() => {
    void refreshMemories();
  });
</script>

<div id="panel-memory" class="companion-section" role="tabpanel" aria-labelledby="tab-memory">
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
</div>
