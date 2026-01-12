<script lang="ts">
  export let showTransformations: boolean;
  export let resizeWidth: number | null;
  export let resizeHeight: number | null;
  export let preserveAspectRatio: boolean;
  export let resizeFilter: string;
  export let rotation: number;
  export let flipHorizontal: boolean;
  export let flipVertical: boolean;
  export let onResetTransformations: () => void;
</script>

<div class="flex flex-col gap-4">
  <button on:click={() => showTransformations = !showTransformations} class="flex px-4 items-center justify-between text-sm font-semibold text-red-400 bg-amber-400/40 uppercase tracking-wider transition-colors rounded-full">
    <span>Transformations</span>
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

      <button on:click={onResetTransformations} class="w-full px-4 py-2 rounded-lg bg-slate-700 text-slate-200 text-sm font-medium hover:bg-slate-600 transition-colors">
        Reset Transformations
      </button>
    </div>
  {/if}
</div>
