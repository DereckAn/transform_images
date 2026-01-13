<script lang="ts">
  import type { ProcessedImage } from "$lib/models/types";
  import ResultCard from "./ResultCard.svelte";

  export let results: ProcessedImage[];
  export let successful: number;
  export let failed: number;
  export let totalSaved: number;
  export let avgCompression: number;

  function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
  }
</script>

<div class="p-2 rounded-xl bg-slate-900/50 border border-slate-800">
  <h3
    class="text-lg font-bold text-white mb-4 flex items-center justify-center gap-2"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="1.5em"
      height="1.5em"
      viewBox="0 0 24 24"
    >
      <path
        fill="currentColor"
        d="m10.6 16.6l7.05-7.05l-1.4-1.4l-5.65 5.65l-2.85-2.85l-1.4 1.4zM12 22q-2.075 0-3.9-.788t-3.175-2.137T2.788 15.9T2 12t.788-3.9t2.137-3.175T8.1 2.788T12 2t3.9.788t3.175 2.137T21.213 8.1T22 12t-.788 3.9t-2.137 3.175t-3.175 2.138T12 22"
      />
    </svg>
    Results
  </h3>

  <!-- Stats Grid -->
  <div class="grid grid-cols-2 md:grid-cols-4 gap-2 mb-4 text-sm">
    <div class="bg-slate-800 p-2 rounded-lg text-center">
      <p class="text-md font-bold text-white">{successful}</p>
      <p class="text-[10px] text-slate-400 uppercase tracking-wider mt-1">
        Successful
      </p>
    </div>
    <div class="bg-slate-800 p-2 rounded-lg text-center">
      <p class="text-md font-bold text-red-400">{failed}</p>
      <p class="text-[10px] text-slate-400 uppercase tracking-wider mt-1">
        Failed
      </p>
    </div>
    <div class="bg-slate-800 p-2 rounded-lg text-center">
      <p class="text-md font-bold text-green-400">
        {formatBytes(totalSaved)}
      </p>
      <p class="text-[10px] text-slate-400 uppercase tracking-wider mt-1">
        Space Saved
      </p>
    </div>
    <div class="bg-slate-800 p-2 rounded-lg text-center">
      <p class="text-md font-bold text-blue-400">
        {avgCompression.toFixed(1)}%
      </p>
      <p class="text-[10px] text-slate-400 uppercase tracking-wider mt-1">
        Avg Compression
      </p>
    </div>
  </div>

  <!-- Results List -->
  <div class="max-h-60 overflow-y-auto scrollbar-thin space-y-2">
    {#each results as result}
      <ResultCard {result} />
    {/each}
  </div>
</div>
