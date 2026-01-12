<script lang="ts">
  import pato from "../../assets/patoblack.png";
  
  export let pendingUpdate: any;
  export let onShowUpdateDialog: () => void;
</script>

<div class="flex items-center gap-3">
  <button type="button" on:click={onShowUpdateDialog} class="relative cursor-pointer">
    <img src={pato} alt="Logo" class="size-16 rounded-full object-cover" />
    {#if pendingUpdate}
      <span class="absolute -top-1 -right-1 size-3 bg-blue-500 rounded-full animate-pulse"></span>
    {/if}
  </button>
  <div class="flex flex-col">
    <h1 class="text-lg font-bold leading-normal text-white">Transform Images</h1>
    <p class="text-sm font-normal leading-normal text-slate-400">Image Optimizer</p>
  </div>
</div>
