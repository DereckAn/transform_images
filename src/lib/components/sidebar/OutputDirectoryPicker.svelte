<script lang="ts">
  export let outputDirectory: string;
  export let onSelectOutputDir: () => void;

  $: isEmpty = !outputDirectory || outputDirectory.trim() === "";
</script>

<label class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <span class="text-sm font-medium text-slate-200">Output Directory</span>
    {#if isEmpty}
      <span class="text-xs text-red-400 font-medium">Required</span>
    {/if}
  </div>
  <div class="flex gap-2">
    <input
      type="text"
      value={outputDirectory}
      readonly
      placeholder="Select output directory"
      class="flex-1 rounded-lg border {isEmpty
        ? 'border-red-500/50 bg-red-950/20'
        : 'border-slate-700 bg-slate-800'} px-3 py-1 text-sm text-slate-200 placeholder:text-slate-500 transition-colors"
    />
    <button
      on:click={onSelectOutputDir}
      class="px-3 py-1 rounded-lg {isEmpty
        ? 'bg-red-600 hover:bg-red-700'
        : 'bg-slate-700 hover:bg-slate-600'} text-slate-200 transition-colors text-sm font-medium"
    >
      Browse
    </button>
  </div>
</label>
