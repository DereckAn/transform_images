<script lang="ts">
  export let preserveMetadata: boolean;
  export let overwriteExisting: boolean;
</script>

<div class="flex flex-col gap-4">
  <label class="flex items-center gap-3 cursor-pointer">
    <input type="checkbox" bind:checked={preserveMetadata} class="w-4 h-4 rounded border-slate-600 bg-slate-800 text-primary" />
    <span class="text-sm text-slate-200">Preserve EXIF metadata</span>
  </label>

  <label class="flex items-center gap-3 cursor-pointer">
    <input type="checkbox" bind:checked={overwriteExisting} class="w-4 h-4 rounded border-slate-600 bg-slate-800 text-primary" />
    <span class="text-sm text-slate-200">Overwrite existing files</span>
  </label>
</div>
