<script lang="ts">
  import { ImageService } from '$lib/app/services/ImageService';
  import { AppState } from '$lib/app/state/AppState';
  import MainContent from '$lib/components/MainContent.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import Header from '$lib/components/header/Header.svelte';
  import type { ProcessedImage } from '$lib/models/types';
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
  $: successful = results.filter(r => r.success).length;
  $: failed = results.length - successful;
  $: totalSaved = results.filter(r => r.success).reduce((sum, r) => sum + (r.originalSize - r.outputSize), 0);
  $: avgCompression = successful > 0 ? results.filter(r => r.success).reduce((sum, r) => sum + r.compressionRatio, 0) / successful : 0;

  onMount(async () => {
    await listen('tauri://file-drop', (event) => {
      const paths = event.payload as string[];
      handleFileDrop(paths);
    });

    imageService.onProgress((current, total, file, percentage) => {
      progress = { current, total, currentFile: file, percentage };
    });

    threadCount = await imageService.getOptimalThreads();
    checkForUpdates();
  });

  async function checkForUpdates() {
    try {
      const update = await check();
      if (update) pendingUpdate = update;
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
      filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'webp', 'gif', 'arw', 'cr2', 'cr3', 'nef', 'dng', 'raf', 'orf', 'rw2', 'pef', 'srw'] }]
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
</script>

<div class="relative flex h-screen w-full flex-col">
  <Header
    {pendingUpdate}
    {isProcessing}
    {canProcess}
    {threadCount}
    onShowUpdateDialog={() => pendingUpdate && (showUpdateDialog = true)}
    onProcess={handleProcess}
    onCancel={handleCancel}
    onClearAll={handleClearAll}
  />

  <div class="flex flex-1 overflow-hidden">
    <Sidebar
      bind:quality
      bind:outputFormat
      bind:outputDirectory
      bind:preserveMetadata
      bind:overwriteExisting
      bind:showTransformations
      bind:resizeWidth
      bind:resizeHeight
      bind:preserveAspectRatio
      bind:resizeFilter
      bind:rotation
      bind:flipHorizontal
      bind:flipVertical
      onSelectOutputDir={handleSelectOutputDir}
      onResetTransformations={handleResetTransformations}
    />

    <MainContent
      {hasImages}
      {images}
      {isProcessing}
      {progress}
      {showResults}
      {results}
      {successful}
      {failed}
      {totalSaved}
      {avgCompression}
      onBrowseFiles={handleBrowseFiles}
      onBrowseFolder={handleBrowseFolder}
      onRemoveImage={handleRemoveImage}
    />
  </div>
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
