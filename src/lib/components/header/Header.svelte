<script lang="ts">
  import ActionButtons from "./ActionButtons.svelte";
  import Logo from "./Logo.svelte";
  
  export let pendingUpdate: any;
  export let onShowUpdateDialog: () => void;
  export let isProcessing: boolean;
  export let canProcess: boolean | string;
  export let threadCount: number;
  export let progress: { current: number; total: number; percentage: number; currentFile: string };
  export let onProcess: () => void;
  export let onCancel: () => void;
  export let onClearAll: () => void;
</script>

<header class="flex items-center justify-between px-6 py-2 bg-background">
  <Logo {pendingUpdate} {onShowUpdateDialog} />

  <ActionButtons
    {isProcessing}
    {canProcess}
    {threadCount}
    {progress}
    {onProcess}
    {onCancel}
    {onClearAll}
  />
</header>
