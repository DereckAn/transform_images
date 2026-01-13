<script lang="ts">
  import type { ImageInfo } from "$lib/models/types";
  import { convertFileSrc } from "@tauri-apps/api/core";

  export let image: ImageInfo;
  export let index: number;
  export let onRemove: (index: number) => void;

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
  class="relative group rounded-xl overflow-hidden h-fit bg-slate-900 border border-slate-800 hover:border-slate-700 transition-all"
>
  <div
    class="aspect-square bg-slate-800 flex items-center justify-center relative overflow-hidden"
  >
    {#if image.format.toLowerCase() === "raw"}
      <div class="text-4xl">📷</div>
    {:else}
      <img
        src={convertFileSrc(image.path)}
        alt={getFileName(image.path)}
        class="w-full h-full object-cover"
        loading="lazy"
      />
    {/if}
    <button
      on:click={() => onRemove(index)}
      class="absolute top-2 right-2 w-6 h-6 rounded-full bg-red-600/90 hover:bg-red-700 text-white flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all hover:scale-110 z-10"
    >
      ✕
    </button>
  </div>
  <div class="p-3">
    <div class="flex items-start justify-between gap-2 mb-2">
      <p class="text-xs font-semibold text-white truncate flex-1">
        {getFileName(image.path)}
      </p>
      {#if image.format.toLowerCase() === "raw"}
        <span class="badge-raw">RAW</span>
      {:else}
        <span class="badge-standard text-xs">{image.format.toUpperCase()}</span>
      {/if}
    </div>
    <div class="flex items-center gap-2 text-[10px] text-slate-400">
      <span>{image.width}×{image.height}</span>
      <span>•</span>
      <span class="text-nowrap">{formatBytes(image.sizeBytes)}</span>
    </div>
  </div>
</div>
