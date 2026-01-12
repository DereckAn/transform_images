<script lang="ts">
  import type { ImageInfo, ProcessedImage } from '$lib/models/types';
  import { convertFileSrc } from '@tauri-apps/api/core';

  export let hasImages: boolean;
  export let images: ImageInfo[];
  export let isProcessing: boolean;
  export let progress: { current: number; total: number; percentage: number; currentFile: string };
  export let showResults: boolean;
  export let results: ProcessedImage[];
  export let successful: number;
  export let failed: number;
  export let totalSaved: number;
  export let avgCompression: number;
  
  export let onBrowseFiles: () => void;
  export let onBrowseFolder: () => void;
  export let onRemoveImage: (index: number) => void;

  function getFileName(path: string): string {
    return path.split(/[\\/]/).pop() || path;
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
  }
</script>

<main class="flex-1 p-8 bg-background overflow-hidden">
  <div class="h-full w-full flex flex-col">
    {#if !hasImages}
      <!-- Drop Zone -->
      <div class="flex items-center justify-center h-full">
        <div class="w-full max-w-2xl">
          <div class="flex flex-col items-center gap-6 rounded-xl border-2 border-dashed border-slate-700 px-6 py-20 hover:border-slate-600 hover:bg-slate-900/20 transition-all cursor-pointer">
            <div class="flex items-center justify-center size-16 rounded-full bg-primary/20">
              <span class="text-4xl">📤</span>
            </div>
            <div class="flex flex-col items-center gap-2">
              <p class="text-xl font-bold text-white text-center">Upload Images</p>
              <p class="text-sm text-slate-400 text-center">
                Drop your images here or click to browse files from your computer. Supports PNG, JPEG, WebP, and 30+ RAW formats.
              </p>
            </div>
            <div class="flex gap-3">
              <button on:click={onBrowseFiles} class="px-6 py-3 rounded-lg bg-slate-800 text-slate-200 text-sm font-bold hover:bg-slate-700 transition-colors">
                📄 Browse Files
              </button>
              <button on:click={onBrowseFolder} class="px-6 py-3 rounded-lg bg-slate-800 text-slate-200 text-sm font-bold hover:bg-slate-700 transition-colors">
                📁 Browse Folder
              </button>
            </div>
          </div>
        </div>
      </div>
    {:else}
      <!-- Images List -->
      <div class="h-full flex flex-col gap-4">
        <div class="flex items-center justify-between pb-4 border-b border-slate-800">
          <div class="flex items-center gap-3">
            <h2 class="text-xl font-bold text-white">Selected Images</h2>
            <span class="px-3 py-1 rounded-full bg-slate-800 text-sm font-semibold text-slate-300">{images.length}</span>
          </div>
        </div>

        <div class="flex-1 overflow-y-auto scrollbar-thin grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 pb-4">
          {#each images as img, index (img.path)}
            <div class="relative group rounded-xl overflow-hidden bg-slate-900 border border-slate-800 hover:border-slate-700 transition-all">
              <div class="aspect-square bg-slate-800 flex items-center justify-center relative overflow-hidden">
                {#if img.format.toLowerCase() === 'raw'}
                  <div class="text-4xl">📷</div>
                {:else}
                  <img src={convertFileSrc(img.path)} alt={getFileName(img.path)} class="w-full h-full object-cover" loading="lazy" />
                {/if}
                <button on:click={() => onRemoveImage(index)} class="absolute top-2 right-2 w-6 h-6 rounded-full bg-red-600/90 hover:bg-red-700 text-white flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all hover:scale-110 z-10">
                  ✕
                </button>
              </div>
              <div class="p-3">
                <div class="flex items-start justify-between gap-2 mb-2">
                  <p class="text-sm font-semibold text-white truncate flex-1">{getFileName(img.path)}</p>
                  {#if img.format.toLowerCase() === 'raw'}
                    <span class="badge-raw">RAW</span>
                  {:else}
                    <span class="badge-standard">{img.format.toUpperCase()}</span>
                  {/if}
                </div>
                <div class="flex items-center gap-2 text-xs text-slate-400">
                  <span>{img.width}×{img.height}</span>
                  <span>•</span>
                  <span>{formatBytes(img.sizeBytes)}</span>
                </div>
              </div>
            </div>
          {/each}
        </div>

        <!-- Progress -->
        {#if isProcessing}
          <div class="p-4 rounded-xl bg-slate-900/50 border border-slate-800">
            <div class="flex items-center justify-between mb-2">
              <p class="text-sm font-medium text-slate-300">Processing: {progress.current} / {progress.total}</p>
              <p class="text-sm font-semibold text-slate-300">{progress.percentage.toFixed(0)}%</p>
            </div>
            <div class="w-full h-2 bg-slate-800 rounded-full overflow-hidden">
              <div class="h-full bg-primary transition-all duration-300" style="width: {progress.percentage}%"></div>
            </div>
            <p class="text-xs text-slate-500 mt-2 truncate">Current: {progress.currentFile}</p>
          </div>
        {/if}

        <!-- Results -->
        {#if showResults}
          <div class="p-6 rounded-xl bg-slate-900/50 border border-slate-800">
            <h3 class="text-lg font-bold text-white mb-4">✅ Results</h3>
            <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
              <div class="bg-slate-800 p-4 rounded-lg text-center">
                <p class="text-2xl font-bold text-white">{successful}</p>
                <p class="text-xs text-slate-400 uppercase tracking-wider mt-1">Successful</p>
              </div>
              <div class="bg-slate-800 p-4 rounded-lg text-center">
                <p class="text-2xl font-bold text-red-400">{failed}</p>
                <p class="text-xs text-slate-400 uppercase tracking-wider mt-1">Failed</p>
              </div>
              <div class="bg-slate-800 p-4 rounded-lg text-center">
                <p class="text-2xl font-bold text-green-400">{formatBytes(totalSaved)}</p>
                <p class="text-xs text-slate-400 uppercase tracking-wider mt-1">Space Saved</p>
              </div>
              <div class="bg-slate-800 p-4 rounded-lg text-center">
                <p class="text-2xl font-bold text-blue-400">{avgCompression.toFixed(1)}%</p>
                <p class="text-xs text-slate-400 uppercase tracking-wider mt-1">Avg Compression</p>
              </div>
            </div>
            <div class="max-h-60 overflow-y-auto scrollbar-thin space-y-2">
              {#each results as result}
                <div class="p-3 rounded-lg border {result.success ? 'bg-green-500/10 border-green-500/30' : 'bg-red-500/10 border-red-500/30'}">
                  <div class="flex items-center justify-between">
                    <p class="text-sm font-medium text-white truncate flex-1">{getFileName(result.originalPath)}</p>
                    {#if result.success}
                      <span class="text-xs font-semibold text-green-400">{result.compressionRatio.toFixed(1)}% smaller</span>
                    {:else}
                      <span class="text-xs font-semibold text-red-400">Failed</span>
                    {/if}
                  </div>
                  {#if result.success}
                    <p class="text-xs text-slate-400 mt-1">{formatBytes(result.originalSize)} → {formatBytes(result.outputSize)}</p>
                  {:else}
                    <p class="text-xs text-red-400 mt-1">{result.errorMessage}</p>
                  {/if}
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</main>
