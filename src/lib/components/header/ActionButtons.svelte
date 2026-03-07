<script lang="ts">
  export let isProcessing: boolean;
  export let canProcess: boolean | string;
  export let threadCount: number;
  export let progress: { current: number; total: number; percentage: number; currentFile: string };
  export let onProcess: () => void;
  export let onCancel: () => void;
  export let onClearAll: () => void;
</script>

<div class="grid grid-cols-7 gap-3">
  {#if !isProcessing}
    <button
      on:click={onProcess}
      title="start"
      disabled={!canProcess}
      class={`col-span-6 px-6 py-2 rounded-lg text-white text-sm font-bold tracking-wide transition-colors disabled:opacity-30 disabled:cursor-not-allowed ${canProcess ? "bg-green-400/50  hover:bg-green-500/90" : "bg-green-900/50"}`}
    >
      ▶️ Start Processing
    </button>
  {:else}
    <button
      on:click={onCancel}
      title="cancel"
      class="col-span-6 px-6 py-2 rounded-lg bg-red-600 text-white text-sm font-bold tracking-wide hover:bg-red-700 transition-colors"
    >
      ⏹️ Cancel
    </button>
  {/if}

  <button
    on:click={onClearAll}
    title="Clear all images"
    class=" p-2 rounded-lg bg-red-800 text-slate-400 font-medium hover:bg-red-600 transition-colors"
  >
    <svg xmlns="http://www.w3.org/2000/svg" width="1.5em" height="1.5em" viewBox="0 0 24 24">
      <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7h16m-10 4v6m4-6v6M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2l1-12M9 7V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v3"/>
    </svg>
  </button>

  <div class="col-span-7 flex flex-col gap-1">
    {#if isProcessing && progress.total > 0}
      <div class="w-full h-1.5 rounded-full bg-slate-700 overflow-hidden">
        <div
          class="h-full rounded-full bg-green-400 transition-all duration-300"
          style="width: {progress.percentage}%"
        ></div>
      </div>
      <div class="flex justify-between text-xs text-slate-500">
        <span class="truncate max-w-[70%]" title={progress.currentFile}>{progress.currentFile}</span>
        <span class="font-semibold text-slate-400 shrink-0">{progress.current}/{progress.total}</span>
      </div>
    {:else}
      <div class="text-xs text-slate-500 text-center">
        Threads: <span class="font-semibold text-slate-400">{threadCount}</span>
      </div>
    {/if}
  </div>
</div>
