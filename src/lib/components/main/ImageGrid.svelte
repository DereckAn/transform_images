<script lang="ts">
  import type { ImageInfo } from "$lib/models/types";
  import ImageCard from "./ImageCard.svelte";

  export let images: ImageInfo[];
  export let onRemoveImage: (index: number) => void;
</script>

<div class="h-full flex flex-col gap-4">
  <div class="flex items-center justify-between pb-2 border-b border-slate-800">
    <h2 class="text-md font-bold text-white">Selected Images</h2>
    <span
      class="px-3 rounded-full bg-slate-800 text-sm font-semibold text-slate-300"
    >
      {images.length}
    </span>
  </div>

  <div
    class="flex-1 overflow-y-auto scrollbar-thin grid grid-cols-3 xl:grid-cols-4 gap-4 pb-4"
  >
    {#each images as img, index (img.path)}
      <ImageCard image={img} {index} onRemove={onRemoveImage} />
    {/each}
  </div>
</div>
