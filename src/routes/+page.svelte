<script lang="ts">
  import { ImageService } from "$lib/app/services/ImageService";
  import { AppState } from "$lib/app/state/AppState";
  import MainContent from "$lib/components/MainContent.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import Header from "$lib/components/header/Header.svelte";
  import type { ProcessedImage } from "$lib/models/types";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { open } from "@tauri-apps/plugin-dialog";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { check } from "@tauri-apps/plugin-updater";
  import { onMount } from "svelte";

  // Services
  const imageService = new ImageService();
  const appState = new AppState();

  // Reactive state
  let images = appState.images;
  let isProcessing = appState.isProcessing;
  let quality = 85;
  let outputDirectory = "";
  let outputFormat: string | undefined = undefined;
  let preserveMetadata = false;
  let overwriteExisting = false;
  let threadCount = 0;
  let showTransformations = false;
  let progress = { current: 0, total: 0, percentage: 0, currentFile: "" };
  let results: ProcessedImage[] = [];
  let showResults = false;
  let pendingUpdate: Awaited<ReturnType<typeof check>> | null = null;
  let showUpdateDialog = false;

  // Transformations
  let resizeWidth: number | null = null;
  let resizeHeight: number | null = null;
  let preserveAspectRatio = true;
  let resizeFilter = "Lanczos3";
  let rotation = 0;
  let flipHorizontal = false;
  let flipVertical = false;

  // Reactive computed
  $: canProcess = images.length > 0 && outputDirectory && !isProcessing;
  $: hasImages = images.length > 0;
  $: successful = results.filter((r) => r.success).length;
  $: failed = results.length - successful;
  $: totalSaved = results
    .filter((r) => r.success)
    .reduce((sum, r) => sum + (r.originalSize - r.outputSize), 0);
  $: avgCompression =
    successful > 0
      ? results
          .filter((r) => r.success)
          .reduce((sum, r) => sum + r.compressionRatio, 0) / successful
      : 0;

  onMount(async () => {
    console.log("🚀 App mounted, setting up drag & drop listener...");

    try {
      const webview = getCurrentWebviewWindow();

      // Listener moderno de Tauri v2 para drag & drop
      const unlisten = await webview.onDragDropEvent((event: any) => {
        console.log("🎯 Drag drop event received:", event);

        if (event.payload.type === "over") {
          console.log("� User hovering over window", event.payload.position);
        } else if (event.payload.type === "drop") {
          console.log("📦 User dropped files:", event.payload.paths);
          handleFileDrop(event.payload.paths);
        } else if (event.payload.type === "cancel") {
          console.log("❌ File drop cancelled");
        }
      });
      console.log("✅ Drag & drop listener registered successfully");

      imageService.onProgress((current, total, file, percentage) => {
        progress = { current, total, currentFile: file, percentage };
      });

      threadCount = await imageService.getOptimalThreads();
      console.log("🧵 Optimal threads:", threadCount);

      checkForUpdates();

      // Cleanup
      return () => {
        console.log("🧹 Cleaning up drag & drop listener...");
        unlisten();
      };
    } catch (error) {
      console.error("❌ Error setting up listeners:", error);
    }
  });

  async function checkForUpdates() {
    try {
      const update = await check();
      if (update) {
        pendingUpdate = update;
        console.log(`🎉 Update available: v${update.version}`);

        // Optional: Automatically show dialog after 3 seconds
        // Uncomment the line below if you want the dialog to auto-open
        // setTimeout(() => showUpdateDialog = true, 3000);
      }
    } catch (e) {
      console.error("Failed to check for updates:", e);
    }
  }

  async function handleUpdate() {
    if (pendingUpdate) {
      await pendingUpdate.downloadAndInstall();
      await relaunch();
    }
  }

  async function handleFileDrop(paths: string[]) {
    console.log("🔄 handleFileDrop called with:", paths);
    try {
      await loadImagePaths(paths);
      console.log("✅ Images loaded successfully");
    } catch (error) {
      console.error("❌ Error loading images:", error);
    }
  }

  async function handleBrowseFiles() {
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "Images",
          extensions: [
            "png",
            "jpg",
            "jpeg",
            "webp",
            "gif",
            "arw",
            "cr2",
            "cr3",
            "nef",
            "dng",
            "raf",
            "orf",
            "rw2",
            "pef",
            "srw",
          ],
        },
      ],
    });

    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected];
      await loadImagePaths(paths);
    }
  }

  async function handleBrowseFolder() {
    const selected = await open({ directory: true, multiple: false });
    if (selected && typeof selected === "string") {
      const loadedImages = await imageService.loadImagesFromFolder(selected);
      appState.addImages(loadedImages);
      images = appState.images;
    }
  }

  async function loadImagePaths(paths: string[]) {
    console.log("📸 Loading image paths:", paths);
    try {
      const loadedImages = await imageService.loadImagesInfo(paths);
      console.log("📊 Loaded images:", loadedImages);
      appState.addImages(loadedImages);
      images = appState.images;
      console.log("✅ Images added to state. Total:", images.length);
    } catch (error) {
      console.error("❌ Error in loadImagePaths:", error);
      throw error;
    }
  }

  async function handleSelectOutputDir() {
    const selected = await open({ directory: true, multiple: false });
    if (selected && typeof selected === "string") {
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
    appState.setResize(
      resizeWidth,
      resizeHeight,
      preserveAspectRatio,
      resizeFilter
    );
    appState.setRotation(rotation);
    appState.setFlipHorizontal(flipHorizontal);
    appState.setFlipVertical(flipVertical);

    try {
      const request = {
        imagePaths: images.map((img) => img.path),
        optimizationOptions: appState.options,
        transformationOptions: appState.hasTransformations()
          ? appState.transformations
          : undefined,
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
    resizeFilter = "Lanczos3";
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
{#if showUpdateDialog && pendingUpdate}
  <div class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50">
    <div
      class="bg-slate-800 rounded-xl p-6 max-w-md w-full mx-4 border-2 border-blue-500/50 shadow-2xl"
    >
      <!-- Header with icon -->
      <div class="flex items-center gap-3 mb-4">
        <div class="size-12 rounded-full bg-blue-600 flex items-center justify-center">
          <span class="text-2xl">🎉</span>
        </div>
        <div class="flex-1">
          <h2 class="text-lg font-bold text-white">Update Available!</h2>
          <p class="text-xs text-slate-400">
            Current: v{version} → New: v{pendingUpdate.version}
          </p>
        </div>
      </div>

      <!-- Description -->
      <div class="mb-6 space-y-2">
        <p class="text-sm text-slate-300">
          A new version of Quak Images is ready to install.
        </p>
        {#if pendingUpdate.body || pendingUpdate.notes}
          <div class="max-h-32 overflow-y-auto bg-slate-900/50 rounded-lg p-3 text-xs text-slate-400">
            {pendingUpdate.body || pendingUpdate.notes || "See release notes on GitHub"}
          </div>
        {/if}
        <p class="text-xs text-slate-500 mt-2">
          ✓ Download and installation are automatic<br />
          ✓ The app will restart after updating
        </p>
      </div>

      <!-- Actions -->
      <div class="flex gap-3">
        <button
          on:click={() => (showUpdateDialog = false)}
          class="flex-1 px-4 py-2.5 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition-colors text-sm font-medium"
        >
          Remind Me Later
        </button>
        <button
          on:click={handleUpdate}
          class="flex-1 px-4 py-2.5 rounded-lg bg-gradient-to-r from-blue-600 to-blue-500 text-white hover:from-blue-500 hover:to-blue-400 transition-all text-sm font-medium shadow-lg shadow-blue-600/30"
        >
          Update Now
        </button>
      </div>
    </div>
  </div>
{/if}
