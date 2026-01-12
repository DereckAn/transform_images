<script lang="ts">
  import { ImageService } from '$lib/app/services/ImageService';
  import { AppState } from '$lib/app/state/AppState';
  import type { ProcessedImage } from '$lib/models/types';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { open } from '@tauri-apps/plugin-dialog';
  import { relaunch } from '@tauri-apps/plugin-process';
  import { check } from '@tauri-apps/plugin-updater';
  import { onMount } from 'svelte';

  // Services
  const imageService = new ImageService();
  const appState = new AppState();

  // Reactive state
  let images = appState.images;
  let isProcessing = appState.isProcessing;
  let quality = 85;
  let outputDirectory = '';
  let outputFormat: string | undefined = undefined;
  let preserveMetadata = false;
  let overwriteExisting = false;
  let threadCount = 0;
  let showTransformations = false;
  let progress = { current: 0, total: 0, percentage: 0, currentFile: '' };
  let results: ProcessedImage[] = [];
  let showResults = false;
  let pendingUpdate: Awaited<ReturnType<typeof check>> | null = null;
  let showUpdateDialog = false;

  // Transformations
  let resizeWidth: number | null = null;
  let resizeHeight: number | null = null;
  let preserveAspectRatio = true;
  let resizeFilter = 'Lanczos3';
  let rotation = 0;
  let flipHorizontal = false;
  let flipVertical = false;

  // Reactive computed
  $: canProcess = images.length > 0 && outputDirectory && !isProcessing;
  $: hasImages = images.length > 0;

  onMount(async () => {
    // Setup file drop listener
    await listen('tauri://file-drop', (event) => {
      const paths = event.payload as string[];
      handleFileDrop(paths);
    });

    // Progress listener
    imageService.onProgress((current, total, file, percentage) => {
      progress = { current, total, currentFile: file, percentage };
    });

    // Get thread count
    threadCount = await imageService.getOptimalThreads();

    // Check for updates
    checkForUpdates();
  });

  async function checkForUpdates() {
    try {
      const update = await check();
      if (update) {
        pendingUpdate = update;
      }
    } catch (e) {
      console.error('Failed to check for updates:', e);
    }
  }

  async function handleUpdate() {
    if (pendingUpdate) {
      await pendingUpdate.downloadAndInstall();
      await relaunch();
    }
  }

  async function handleFileDrop(paths: string[]) {
    await loadImagePaths(paths);
  }

  async function handleBrowseFiles() {
    const selected = await open({
      multiple: true,
      filters: [{
        name: 'Images',
        extensions: ['png', 'jpg', 'jpeg', 'webp', 'gif', 'arw', 'cr2', 'cr3', 'nef', 'dng', 'raf', 'orf', 'rw2', 'pef', 'srw']
      }]
    });

    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected];
      await loadImagePaths(paths);
    }
  }

  async function handleBrowseFolder() {
    const selected = await open({ directory: true, multiple: false });
    if (selected && typeof selected === 'string') {
      const loadedImages = await imageService.loadImagesFromFolder(selected);
      appState.addImages(loadedImages);
      images = appState.images;
    }
  }

  async function loadImagePaths(paths: string[]) {
    const loadedImages = await imageService.loadImagesInfo(paths);
    appState.addImages(loadedImages);
    images = appState.images;
  }

  async function handleSelectOutputDir() {
    const selected = await open({ directory: true, multiple: false });
    if (selected && typeof selected === 'string') {
      outputDirectory = selected;
      appState.setOutputDirectory(selected);
    }
  }

  function handleRemoveImage(index: number) {
    appState.removeImage(index);
    images = appState.images;
  }

  function handleClearAll() {
    appState.clearImages();
    images = appState.images;
    showResults = false;
  }

  async function handleProcess() {
    if (!canProcess) return;

    showResults = false;
    isProcessing = true;
    appState.setProcessing(true);

    // Update app state with current values
    appState.setQuality(quality);
    appState.setOutputFormat(outputFormat);
    appState.setPreserveMetadata(preserveMetadata);
    appState.setOverwriteExisting(overwriteExisting);
    appState.setResize(resizeWidth, resizeHeight, preserveAspectRatio, resizeFilter);
    appState.setRotation(rotation);
    appState.setFlipHorizontal(flipHorizontal);
    appState.setFlipVertical(flipVertical);

    try {
      const request = {
        imagePaths: images.map(img => img.path),
        optimizationOptions: appState.options,
        transformationOptions: appState.hasTransformations() ? appState.transformations : undefined
      };

      results = await imageService.processImages(request);
      showResults = true;
    } catch (error) {
      alert(`Processing failed: ${error}`);
    } finally {
      isProcessing = false;
      appState.setProcessing(false);
    }
  }

  async function handleCancel() {
    await imageService.cancelProcessing();
    isProcessing = false;
    appState.setProcessing(false);
  }

  function handleResetTransformations() {
    resizeWidth = null;
    resizeHeight = null;
    preserveAspectRatio = true;
    resizeFilter = 'Lanczos3';
    rotation = 0;
    flipHorizontal = false;
    flipVertical = false;
    appState.resetTransformations();
  }

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

  $: successful = results.filter(r => r.success).length;
  $: failed = results.length - successful;
  $: totalSaved = results.filter(r => r.success).reduce((sum, r) => sum + (r.originalSize - r.outputSize), 0);
  $: avgCompression = successful > 0 ? results.filter(r => r.success).reduce((sum, r) => sum + r.compressionRatio, 0) / successful : 0;
</script>

<div class="relative flex h-screen w-full">
  <!-- SIDEBAR -->
  <aside class="flex w-100 flex-col border-r border-slate-800 bg-background-dark/50">
    <div class="flex h-full flex-col justify-between p-4">
      <div class="flex flex-col gap-6">
        <!-- Header -->
        <div class="flex items-center gap-3 px-2">
          <button type="button" on:click={() => pendingUpdate && (showUpdateDialog = true)} class="relative cursor-pointer">
            <img src="/src/assets/patoblack.png" alt="Logo" class="size-16 rounded-full object-cover" />
            {#if pendingUpdate}
              <span class="absolute -top-1 -right-1 size-3 bg-blue-500 rounded-full animate-pulse"></span>
            {/if}
          </button>
          <div class="flex flex-col">
            <h1 class="text-lg font-bold leading-normal text-white">Transform Images</h1>
            <p class="text-sm font-normal leading-normal text-slate-400">Image Optimizer</p>
          </div>
        </div>

        <!-- Settings -->
        <div class="flex flex-col gap-4 overflow-y-auto scrollbar-thin pr-2" style="max-height: calc(100vh - 250px)">
          <div class="flex flex-col gap-4">
            <h3 class="text-sm font-semibold text-red-400 uppercase tracking-wider">Optimization</h3>

            <!-- Quality -->
            <div class="flex flex-col gap-2">
              <div class="flex items-center justify-between">
                <label class="text-sm font-medium text-slate-200">Quality</label>
                <span class="text-sm text-slate-400">{quality}%</span>
              </div>
              <input type="range" bind:value={quality} min="1" max="100" class="w-full h-1.5 bg-slate-700 rounded-full appearance-none cursor-pointer accent-primary" />
            </div>

            <!-- Format -->
            <div class="flex flex-col gap-2">
              <span class="text-sm font-medium text-slate-200">Format</span>
              <select bind:value={outputFormat} class="w-full appearance-none rounded-lg border border-slate-700 bg-slate-800 px-3 py-2.5 text-sm text-slate-200 focus:border-primary focus:outline-none focus:ring-2 focus:ring-primary/20">
                <option value={undefined}>Keep Original</option>
                <option value="jpeg">JPEG</option>
                <option value="png">PNG</option>
                <option value="webp">WebP</option>
              </select>
            </div>

            <!-- Output Directory -->
            <label class="flex flex-col gap-2">
              <span class="text-sm font-medium text-slate-200">Output Directory</span>
              <div class="flex gap-2">
                <input type="text" value={outputDirectory} readonly placeholder="Same as source" class="flex-1 rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm text-slate-200 placeholder:text-slate-500" />
                <button on:click={handleSelectOutputDir} class="px-3 py-2 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition-colors text-sm font-medium">Browse</button>
              </div>
            </label>

            <!-- Checkboxes -->
            <label class="flex items-center gap-3 cursor-pointer">
              <input type="checkbox" bind:checked={preserveMetadata} class="w-4 h-4 rounded border-slate-600 bg-slate-800 text-primary" />
              <span class="text-sm text-slate-200">Preserve EXIF metadata</span>
            </label>

            <label class="flex items-center gap-3 cursor-pointer">
              <input type="checkbox" bind:checked={overwriteExisting} class="w-4 h-4 rounded border-slate-600 bg-slate-800 text-primary" />
              <span class="text-sm text-slate-200">Overwrite existing files</span>
            </label>
          </div>

          <div class="border-t border-slate-800"></div>

          <!-- Transformations -->
          <div class="flex flex-col gap-4">
            <button on:click={() => showTransformations = !showTransformations} class="flex items-center justify-between text-sm font-semibold text-red-400 uppercase tracking-wider hover:text-slate-300 transition-colors">
              <span>🔄 Transformations</span>
              <span class="transform transition-transform" style="transform: rotate({showTransformations ? 0 : -90}deg)">▼</span>
            </button>

            {#if showTransformations}
              <div class="flex flex-col gap-4">
                <div class="flex gap-2">
                  <label class="flex-1 flex flex-col gap-2">
                    <span class="text-sm font-medium text-slate-200">Width</span>
                    <input type="number" bind:value={resizeWidth} placeholder="Auto" class="w-full rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm text-slate-200" />
                  </label>
                  <label class="flex-1 flex flex-col gap-2">
                    <span class="text-sm font-medium text-slate-200">Height</span>
                    <input type="number" bind:value={resizeHeight} placeholder="Auto" class="w-full rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm text-slate-200" />
                  </label>
                </div>

                <label class="flex items-center gap-3 cursor-pointer">
                  <input type="checkbox" bind:checked={preserveAspectRatio} class="w-4 h-4 rounded border-slate-600 bg-slate-800 text-primary" />
                  <span class="text-sm text-slate-200">Maintain aspect ratio</span>
                </label>

                <label class="flex flex-col gap-2">
                  <span class="text-sm font-medium text-slate-200">Resize Filter</span>
                  <select bind:value={resizeFilter} class="w-full appearance-none rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm text-slate-200">
                    <option value="Lanczos3">Lanczos3 (Best)</option>
                    <option value="CatmullRom">CatmullRom</option>
                    <option value="Gaussian">Gaussian</option>
                    <option value="Triangle">Triangle</option>
                    <option value="Nearest">Nearest (Fastest)</option>
                  </select>
                </label>

                <label class="flex flex-col gap-2">
                  <span class="text-sm font-medium text-slate-200">Rotate</span>
                  <select bind:value={rotation} class="w-full appearance-none rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm text-slate-200">
                    <option value={0}>None</option>
                    <option value={90}>90°</option>
                    <option value={180}>180°</option>
                    <option value={270}>270°</option>
                  </select>
                </label>

                <label class="flex items-center gap-3 cursor-pointer">
                  <input type="checkbox" bind:checked={flipHorizontal} class="w-4 h-4 rounded border-slate-600 bg-slate-800 text-primary" />
                  <span class="text-sm text-slate-200">Flip Horizontal</span>
                </label>

                <label class="flex items-center gap-3 cursor-pointer">
                  <input type="checkbox" bind:checked={flipVertical} class="w-4 h-4 rounded border-slate-600 bg-slate-800 text-primary" />
                  <span class="text-sm text-slate-200">Flip Vertical</span>
                </label>

                <button on:click={handleResetTransformations} class="w-full px-4 py-2 rounded-lg bg-slate-700 text-slate-200 text-sm font-medium hover:bg-slate-600 transition-colors">
                  Reset Transformations
                </button>
              </div>
            {/if}
          </div>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="flex flex-col gap-2 pt-4 border-t border-slate-800">
        {#if !isProcessing}
          <button on:click={handleProcess} disabled={!canProcess} class="w-full px-4 py-3 rounded-lg bg-primary text-white text-sm font-bold tracking-wide hover:bg-black/90 transition-colors disabled:opacity-30 disabled:cursor-not-allowed">
            ▶️ Start Processing
          </button>
        {:else}
          <button on:click={handleCancel} class="w-full px-4 py-3 rounded-lg bg-red-600 text-white text-sm font-bold tracking-wide hover:bg-red-700 transition-colors">
            Cancel
          </button>
        {/if}

        <button on:click={handleClearAll} class="w-full px-4 py-2 rounded-lg bg-transparent text-slate-400 text-sm font-medium hover:bg-slate-800 transition-colors">
          Clear All
        </button>

        <p class="text-xs text-center text-slate-500 pt-2">
          Threads: <span class="font-semibold">{threadCount}</span>
        </p>
      </div>
    </div>
  </aside>

  <!-- MAIN CONTENT -->
  <main class="flex-1 p-8 bg-background-dark overflow-hidden">
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
                <button on:click={handleBrowseFiles} class="px-6 py-3 rounded-lg bg-slate-800 text-slate-200 text-sm font-bold hover:bg-slate-700 transition-colors">
                  📄 Browse Files
                </button>
                <button on:click={handleBrowseFolder} class="px-6 py-3 rounded-lg bg-slate-800 text-slate-200 text-sm font-bold hover:bg-slate-700 transition-colors">
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
                  <button on:click={() => handleRemoveImage(index)} class="absolute top-2 right-2 w-6 h-6 rounded-full bg-red-600/90 hover:bg-red-700 text-white flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all hover:scale-110 z-10">
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
</div>

<!-- Update Dialog -->
{#if showUpdateDialog}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div class="bg-slate-800 rounded-xl p-6 max-w-2xl w-full mx-4 border-4 border-slate-700">
      <h2 class="text-lg font-bold text-white mb-2">Update Available</h2>
      <p class="text-sm text-slate-400 mb-4">Version {pendingUpdate?.version} is available.</p>
      <div class="flex gap-3 justify-end">
        <button on:click={() => showUpdateDialog = false} class="px-4 py-2 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition-colors text-sm font-medium">
          Later
        </button>
        <button on:click={handleUpdate} class="px-4 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-500 transition-colors text-sm font-medium">
          Update Now
        </button>
      </div>
    </div>
  </div>
{/if}
