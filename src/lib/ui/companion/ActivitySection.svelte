<script lang="ts">
  import { onMount } from 'svelte';
  import { listActivity, type ActivityHistoryRecord } from '../../memory/runtime';

  export let refreshEpoch = 0;

  let activities: ActivityHistoryRecord[] = [];
  let historyBusy = false;
  let historyLoaded = false;
  let historyStatus = '';
  let handledEpoch = refreshEpoch;

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

  $: if (historyLoaded && refreshEpoch !== handledEpoch) {
    handledEpoch = refreshEpoch;
    void refreshActivity();
  }

  onMount(() => {
    handledEpoch = refreshEpoch;
    void refreshActivity();
  });
</script>

<div id="panel-activity" class="companion-section" role="tabpanel" aria-labelledby="tab-activity">
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
</div>
