<script lang="ts">
  export let isProcessing: boolean;
  export let canProcess: boolean;
  export let threadCount: number;
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

  <div class="text-xs col-span-7 text-slate-500 text-center">
    Threads: <span class="font-semibold text-slate-400">{threadCount}</span>
  </div>
</div>
