<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import CompanionView from './lib/ui/CompanionView.svelte';
  import PetView from './lib/ui/PetView.svelte';

  const windowLabel = getCurrentWindow().label;
</script>

{#if windowLabel === 'companion'}
  <CompanionView />
{:else}
  <PetView />
{/if}
