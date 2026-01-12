<script lang="ts">
  export let progress: {
    current: number;
    total: number;
    percentage: number;
    currentFile: string;
  };
</script>

<div class="p-4 rounded-xl bg-slate-900/50 border border-slate-800">
  <div class="flex items-center justify-between mb-2">
    <p class="text-sm font-medium text-slate-300">
      Processing: {progress.current} / {progress.total}
    </p>
    <p class="text-sm font-semibold text-slate-300">
      {progress.percentage.toFixed(0)}%
    </p>
  </div>
  <div class="w-full h-2 bg-slate-800 rounded-full overflow-hidden">
    <div
      class="h-full bg-primary transition-all duration-300"
      style="width: {progress.percentage}%"
    ></div>
  </div>
  <p class="text-xs text-slate-500 mt-2 truncate">
    Current: {progress.currentFile}
  </p>
</div>
