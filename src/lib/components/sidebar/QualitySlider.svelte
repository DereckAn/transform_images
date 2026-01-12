<script lang="ts">
  export let quality: number;
</script>

<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <span class="text-sm font-medium text-slate-200">Quality</span>
    <span class="text-sm text-slate-400">{quality}%</span>
  </div>
  <input type="range" bind:value={quality} min="1" max="100" class="w-full h-1.5 bg-slate-700 rounded-full appearance-none cursor-pointer accent-primary" />
</div>
