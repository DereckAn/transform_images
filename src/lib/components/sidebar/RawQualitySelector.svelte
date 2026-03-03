<script lang="ts">
  export let rawQualityMode: string | undefined = "balanced";
</script>

<div class="flex flex-col gap-2">
  <span class="text-xs font-medium text-slate-200">RAW Quality</span>
  <select bind:value={rawQualityMode} class="w-full appearance-none rounded-lg border border-slate-700 bg-slate-800 px-3 py-1 text-sm text-slate-200 focus:outline-none focus:ring-0">
    <option value="fast">Fast (Half-res, ~15x faster)</option>
    <option value="balanced">Balanced (Full-res, ~3x faster)</option>
    <option value="quality">Quality (Full-res, best output)</option>
  </select>
</div>
