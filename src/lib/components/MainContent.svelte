<script lang="ts">
  import type { ImageInfo, ProcessedImage } from "$lib/models/types";
  import DropZone from "./main/DropZone.svelte";
  import ImageGrid from "./main/ImageGrid.svelte";
  import ProgressBar from "./main/ProgressBar.svelte";
  import ResultsPanel from "./main/ResultsPanel.svelte";

  export let hasImages: boolean;
  export let images: ImageInfo[];
  export let isProcessing: boolean;
  export let progress: {
    current: number;
    total: number;
    percentage: number;
    currentFile: string;
  };
  export let showResults: boolean;
  export let results: ProcessedImage[];
  export let successful: number;
  export let failed: number;
  export let totalSaved: number;
  export let avgCompression: number;

  export let onBrowseFiles: () => void;
  export let onBrowseFolder: () => void;
  export let onRemoveImage: (index: number) => void;
</script>

<main class="flex-1 px-4 bg-background overflow-hidden">
  <div class="h-full w-full flex flex-col">
    {#if !hasImages}
      <DropZone {onBrowseFiles} {onBrowseFolder} />
    {:else}
      <div class="h-full flex flex-col gap-4">
        <ImageGrid {images} {onRemoveImage} />

        {#if isProcessing}
          <ProgressBar {progress} />
        {/if}

        {#if showResults}
          <ResultsPanel
            {results}
            {successful}
            {failed}
            {totalSaved}
            {avgCompression}
          />
        {/if}
      </div>
    {/if}
  </div>
</main>
