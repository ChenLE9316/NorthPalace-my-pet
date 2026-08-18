<script lang="ts">
  import { onMount } from 'svelte';
  import {
    fallbackStartupStatus,
    getStartupStatus,
    setStartupEnabled,
    type StartupStatus,
  } from '../../settings/runtime';
  import PrivacySettings from '../PrivacySettings.svelte';

  let startupStatus: StartupStatus = fallbackStartupStatus;
  let startupBusy = false;
  let startupLoaded = false;
  let startupMessage = '';

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

  onMount(() => {
    void refreshStartup();
  });
</script>

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

    <PrivacySettings />
  </section>
</section>
