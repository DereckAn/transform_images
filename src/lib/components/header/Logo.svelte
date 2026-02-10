<script lang="ts">
  import { getVersion } from "@tauri-apps/api/app";
  import { onMount } from "svelte";
  import pato from "../../assets/patoblack.png";
  
  export let pendingUpdate: any;
  export let onShowUpdateDialog: () => void;

  let version = "";

  onMount(async () => {
    version = await getVersion();
  });
</script>

<div class="flex items-center gap-3">
  <button
    type="button"
    on:click={onShowUpdateDialog}
    class="relative cursor-pointer group"
    title={pendingUpdate ? `Update available: v${pendingUpdate.version}` : ""}
  >
    <img src={pato} alt="Logo" class="size-16 rounded-full object-cover" />
    {#if pendingUpdate}
      <!-- Larger pulsing indicator with glow effect -->
      <span class="absolute -top-1 -right-1 size-4 bg-blue-500 rounded-full animate-pulse shadow-lg shadow-blue-500/50 ring-2 ring-white"></span>
      <!-- Subtle badge on hover -->
      <span class="absolute -bottom-1 left-1/2 -translate-x-1/2 opacity-0 group-hover:opacity-100 transition-opacity bg-blue-600 text-white text-xs px-2 py-0.5 rounded-full whitespace-nowrap">
        Update!
      </span>
    {/if}
  </button>
  <div class="flex flex-col">
    <h1 class="text-lg font-bold leading-normal text-white">Transform Images</h1>
    <p class="text-sm font-normal leading-normal text-slate-400">Image Optimizer</p>
    {#if version}
      <p class="text-xs font-normal leading-normal text-slate-500">
        v{version}
        {#if pendingUpdate}
          <span class="text-blue-400 ml-1">→ v{pendingUpdate.version} available</span>
        {/if}
      </p>
    {/if}
  </div>
</div>
