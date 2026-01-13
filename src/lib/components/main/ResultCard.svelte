<script lang="ts">
  import type { ProcessedImage } from "$lib/models/types";

  export let result: ProcessedImage;

  function getFileName(path: string): string {
    return path.split(/[\\/]/).pop() || path;
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
  }
</script>

<div
  class="p-3 rounded-lg border {result.success
    ? 'bg-green-500/10 border-green-500/30'
    : 'bg-red-500/10 border-red-500/30'}"
>
  <div class="flex items-center justify-between">
    <p class="text-sm font-medium text-white truncate flex-1">
      {getFileName(result.originalPath)}
    </p>
    {#if result.success}
      <span class="text-xs font-semibold text-green-400">
        {result.compressionRatio.toFixed(1)}% smaller
      </span>
    {:else}
      <span class="text-xs font-semibold text-red-400">Failed</span>
    {/if}
  </div>
  {#if result.success}
    <p class="text-xs text-slate-400 mt-1">
      {formatBytes(result.originalSize)} → {formatBytes(result.outputSize)}
    </p>
  {:else}
    <p class="text-xs text-red-400 mt-1">{result.errorMessage}</p>
  {/if}
</div>
