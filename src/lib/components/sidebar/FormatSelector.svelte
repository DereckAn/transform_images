<script lang="ts">
  export let outputFormat: string | undefined;
</script>

<div class="flex flex-col gap-2">
  <span class="text-sm font-medium text-slate-200">Format</span>
  <select bind:value={outputFormat} class="w-full appearance-none rounded-lg border border-slate-700 bg-slate-800 px-3 py-2.5 text-sm text-slate-200 focus:border-primary focus:outline-none focus:ring-2 focus:ring-primary/20">
    <option value={undefined}>Keep Original</option>
    <option value="jpeg">JPEG</option>
    <option value="png">PNG</option>
    <option value="webp">WebP</option>
  </select>
</div>
