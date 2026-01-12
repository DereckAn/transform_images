<script lang="ts">
  export let outputDirectory: string;
  export let onSelectOutputDir: () => void;
</script>

<label class="flex flex-col gap-2">
  <span class="text-sm font-medium text-slate-200">Output Directory</span>
  <div class="flex gap-2">
    <input type="text" value={outputDirectory} readonly placeholder="Same as source" class="flex-1 rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm text-slate-200 placeholder:text-slate-500" />
    <button on:click={onSelectOutputDir} class="px-3 py-2 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition-colors text-sm font-medium">Browse</button>
  </div>
</label>
