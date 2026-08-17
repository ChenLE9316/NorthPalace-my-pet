<script lang="ts">
  import { onMount } from 'svelte';
  import {
    fallbackScreenContext,
    getScreenContext,
    type ScreenContextSnapshot,
  } from '../context/runtime';
  import {
    addPrivacyExcludedApp,
    fallbackPrivacyRules,
    getPrivacyRules,
    removePrivacyExcludedApp,
    setAccessibilityContextEnabled,
    type PrivacyRulesSnapshot,
  } from '../settings/runtime';

  let rules: PrivacyRulesSnapshot = fallbackPrivacyRules;
  let context: ScreenContextSnapshot = fallbackScreenContext;
  let contextTimer: number | undefined;
  let draft = '';
  let busy = false;
  let loaded = false;
  let message = '';

  async function refresh() {
    busy = true;
    message = '';
    try {
      rules = await getPrivacyRules();
    } catch (error) {
      rules = fallbackPrivacyRules;
      message = error instanceof Error ? error.message : String(error);
    } finally {
      loaded = true;
      busy = false;
    }
  }

  async function refreshContext() {
    try {
      context = await getScreenContext();
    } catch {
      context = fallbackScreenContext;
    }
  }

  async function addExclusion() {
    const appId = draft.trim();
    if (!appId || busy) return;

    busy = true;
    message = '';
    try {
      rules = await addPrivacyExcludedApp(appId);
      draft = '';
      message = '已加入排除；該程式的 active-app identity 與 structured context 會在 Windows sensor 邊界被阻擋。';
    } catch (error) {
      message = error instanceof Error ? error.message : String(error);
      await refreshAfterFailure();
    } finally {
      loaded = true;
      busy = false;
    }
  }

  async function removeExclusion(appId: string) {
    if (busy) return;

    busy = true;
    message = '';
    try {
      rules = await removePrivacyExcludedApp(appId);
      message = `已移除 ${appId} 的排除規則。`;
    } catch (error) {
      message = error instanceof Error ? error.message : String(error);
      await refreshAfterFailure();
    } finally {
      loaded = true;
      busy = false;
    }
  }

  async function toggleAccessibilityContext(event: Event) {
    const requested = (event.currentTarget as HTMLInputElement).checked;
    if (busy || rules.failClosed) return;

    busy = true;
    message = '';
    try {
      rules = await setAccessibilityContextEnabled(requested);
      message = rules.accessibilityContextEnabled
        ? '已授權 bounded accessibility context。現階段 collector 尚未啟用，不會因此開始讀取 accessibility metadata。'
        : '已關閉 accessibility context 授權。';
    } catch (error) {
      message = error instanceof Error ? error.message : String(error);
      await refreshAfterFailure();
    } finally {
      loaded = true;
      busy = false;
    }
  }

  async function refreshAfterFailure() {
    try {
      rules = await getPrivacyRules();
    } catch {
      rules = fallbackPrivacyRules;
    }
  }

  function contextLabel() {
    switch (context.activeAppState) {
      case 'available':
        return context.activeAppId ? `Available · ${context.activeAppId}` : 'Available';
      case 'privacy_blocked':
        return 'Privacy blocked';
      default:
        return 'Unknown';
    }
  }

  function boundsLabel() {
    const bounds = context.activeWindowBounds;
    if (!bounds) return 'not exposed';
    return `${bounds.width}×${bounds.height} @ ${bounds.x}, ${bounds.y}`;
  }

  onMount(() => {
    void refresh();
    void refreshContext();
    contextTimer = window.setInterval(() => void refreshContext(), 2_000);

    return () => {
      window.clearInterval(contextTimer);
    };
  });
</script>

<section class="privacy-settings" aria-label="Privacy and structured context">
  <div class="privacy-heading">
    <div>
      <strong>Privacy</strong>
      <span>排除發生在 Windows sensor → Domain Event / Screen Context 之前，不只是 UI 隱藏。</span>
    </div>
    <button onclick={() => { void refresh(); void refreshContext(); }} disabled={busy}>重新讀取</button>
  </div>

  {#if !loaded && busy}
    <div class="privacy-empty">正在讀取本地 privacy rules……</div>
  {:else}
    <div class:privacy-alert={rules.failClosed} class="privacy-state">
      <strong>{rules.failClosed ? 'Fail-closed' : 'Privacy gate ready'}</strong>
      <span>
        {rules.failClosed
          ? 'Privacy rules 尚未安全載入；目前所有 active-app identity 與敏感 structured context 都會被阻擋。'
          : `目前有 ${rules.excludedApps.length} 個 app exclusion。`}
      </span>
    </div>

    <article class="privacy-capability">
      <div>
        <strong>Accessibility context</strong>
        <span>只授權未來的 bounded structured metadata；不包含 screenshot、screen pixels、window title 或 raw text dump。</span>
      </div>
      <label class="capability-toggle">
        <input
          type="checkbox"
          checked={rules.accessibilityContextEnabled}
          disabled={busy || rules.failClosed}
          onchange={(event) => void toggleAccessibilityContext(event)}
        />
        <span>{rules.accessibilityContextEnabled ? '已授權' : '未授權'}</span>
      </label>
      <p>目前 collector 尚未實作；開啟此能力只保存明確 opt-in，不會立即開始收集 accessibility metadata。</p>
    </article>

    <div class="context-state" aria-label="Structured Screen Context status">
      <div>
        <strong>Structured Screen Context</strong>
        <span>{contextLabel()}</span>
      </div>
      <dl>
        <div><dt>Window bounds</dt><dd>{boundsLabel()}</dd></div>
        <div><dt>User idle</dt><dd>{Math.round(context.userIdleMs / 1000)} s</dd></div>
        <div><dt>Local hour</dt><dd>{context.localHour}:00</dd></div>
      </dl>
      <p>這裡只顯示目前 Broker 已允許的結構化訊號。被排除的 app 不會顯示 identity 或 bounds。</p>
    </div>

    <div class="privacy-add-wrap">
      <strong>App Privacy Exclusions</strong>
      <span>輸入 process app-id，讓該程式在感知邊界直接變成 privacy-blocked。</span>
    </div>

    <form class="privacy-add" onsubmit={(event) => { event.preventDefault(); void addExclusion(); }}>
      <input
        bind:value={draft}
        maxlength="128"
        placeholder="process app-id，例如 discord"
        aria-label="要排除的 process app-id"
        disabled={busy || rules.failClosed}
      />
      <button type="submit" disabled={busy || rules.failClosed || !draft.trim()}>加入</button>
    </form>

    <p class="privacy-help">
      輸入執行檔名稱即可，不需要完整路徑；大小寫與尾端 .exe 會自動正規化。這裡不會建立「最近使用程式」清單。
    </p>

    <div class="privacy-list" aria-busy={busy}>
      {#if rules.excludedApps.length === 0}
        <div class="privacy-empty">目前沒有使用者設定的 app exclusion。</div>
      {/if}

      {#each rules.excludedApps as appId (appId)}
        <div class="privacy-row">
          <code>{appId}</code>
          <button
            class="remove"
            onclick={() => void removeExclusion(appId)}
            disabled={busy || rules.failClosed}
            aria-label={`移除 ${appId} 排除規則`}
          >移除</button>
        </div>
      {/each}
    </div>
  {/if}

  {#if message}
    <p class="privacy-message" aria-live="polite">{message}</p>
  {/if}
</section>

<style>
  .privacy-settings {
    display: grid;
    gap: 10px;
    padding-top: 12px;
    border-top: 1px solid rgba(106, 217, 255, .12);
  }

  .privacy-heading,
  .privacy-capability,
  .context-state,
  .privacy-add-wrap {
    display: grid;
    gap: 5px;
  }

  .privacy-heading {
    grid-template-columns: 1fr auto;
    align-items: start;
    gap: 10px;
  }

  .privacy-heading > div {
    display: grid;
    gap: 3px;
  }

  .privacy-heading strong,
  .privacy-state strong,
  .privacy-capability strong,
  .context-state strong,
  .privacy-add-wrap strong {
    font-size: 12px;
    color: #e4f8ff;
  }

  .privacy-heading span,
  .privacy-state span,
  .privacy-capability span,
  .privacy-capability p,
  .context-state span,
  .context-state p,
  .privacy-add-wrap span,
  .privacy-help {
    font-size: 10px;
    color: #7fa2b3;
    line-height: 1.5;
  }

  button {
    border: 1px solid rgba(102, 220, 255, .25);
    border-radius: 9px;
    background: rgba(31, 104, 138, .25);
    color: #dff8ff;
    padding: 7px 9px;
    cursor: pointer;
    font: inherit;
  }

  button:disabled {
    opacity: .45;
    cursor: default;
  }

  .privacy-heading button {
    flex: 0 0 auto;
    font-size: 10px;
  }

  .privacy-state,
  .privacy-capability,
  .context-state {
    padding: 10px;
    border: 1px solid rgba(106, 217, 255, .14);
    border-radius: 11px;
    background: rgba(77, 187, 229, .045);
  }

  .privacy-state {
    display: grid;
    gap: 4px;
  }

  .privacy-state.privacy-alert {
    border-color: rgba(255, 168, 178, .30);
    background: rgba(120, 35, 55, .12);
  }

  .privacy-state.privacy-alert strong {
    color: #ffc9d2;
  }

  .capability-toggle {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    padding-top: 8px;
    border-top: 1px solid rgba(106, 217, 255, .10);
    color: #a8dce9;
    font-size: 11px;
  }

  .capability-toggle input {
    width: 17px;
    height: 17px;
    margin: 0;
  }

  .privacy-capability p,
  .context-state p,
  .privacy-help {
    margin: 0;
  }

  .context-state > div:first-child {
    display: flex;
    justify-content: space-between;
    gap: 10px;
  }

  .context-state dl {
    display: grid;
    gap: 4px;
    margin: 2px 0;
  }

  .context-state dl div {
    display: flex;
    justify-content: space-between;
    gap: 10px;
    font-size: 10px;
  }

  .context-state dt { color: #6f93a6; }
  .context-state dd { margin: 0; color: #a7eaff; text-align: right; overflow-wrap: anywhere; }

  .privacy-add-wrap { margin-top: 2px; }

  .privacy-add {
    display: flex;
    gap: 7px;
  }

  .privacy-add input {
    min-width: 0;
    flex: 1;
    border: 1px solid rgba(102, 220, 255, .20);
    border-radius: 9px;
    background: rgba(3, 12, 23, .78);
    color: #e7f8ff;
    padding: 7px 9px;
    outline: none;
    font: inherit;
  }

  .privacy-add input:focus {
    border-color: rgba(102, 220, 255, .52);
  }

  .privacy-help {
    margin-top: -2px;
  }

  .privacy-list {
    display: grid;
    gap: 7px;
  }

  .privacy-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    padding: 8px 9px;
    border: 1px solid rgba(106, 217, 255, .11);
    border-radius: 10px;
    background: rgba(77, 187, 229, .035);
  }

  .privacy-row code {
    color: #a7eaff;
    font-size: 11px;
    overflow-wrap: anywhere;
  }

  .privacy-row .remove {
    padding: 5px 7px;
    border-color: rgba(255, 129, 150, .24);
    background: rgba(120, 35, 55, .18);
    color: #ffc1cb;
    font-size: 10px;
  }

  .privacy-empty {
    padding: 10px;
    border: 1px dashed rgba(106, 217, 255, .14);
    border-radius: 10px;
    color: #7699aa;
    font-size: 10px;
    text-align: center;
  }

  .privacy-message {
    margin: 0;
    padding: 7px 9px;
    border-radius: 9px;
    background: rgba(87, 203, 242, .07);
    color: #9fe8fb;
    font-size: 10px;
    line-height: 1.4;
  }
</style>
