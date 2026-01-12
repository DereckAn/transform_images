<script lang="ts">
  export let onBrowseFiles: () => void;
  export let onBrowseFolder: () => void;
</script>

<div class="flex items-center justify-center h-full">
  <div class="w-full max-w-2xl">
    <div
      class="flex flex-col items-center gap-6 rounded-xl border-2 border-dashed border-slate-700 px-6 py-20 hover:border-slate-600 hover:bg-slate-900/20 transition-all cursor-pointer"
    >
      <div
        class="flex items-center justify-center size-16 rounded-full bg-primary/20"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="4em"
          height="4em"
          viewBox="0 0 16 16"
          ><g fill="white"
            ><path d="M4.502 9a1.5 1.5 0 1 0 0-3a1.5 1.5 0 0 0 0 3" /><path
              d="M14.002 13a2 2 0 0 1-2 2h-10a2 2 0 0 1-2-2V5A2 2 0 0 1 2 3a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v8a2 2 0 0 1-1.998 2M14 2H4a1 1 0 0 0-1 1h9.002a2 2 0 0 1 2 2v7A1 1 0 0 0 15 11V3a1 1 0 0 0-1-1M2.002 4a1 1 0 0 0-1 1v8l2.646-2.354a.5.5 0 0 1 .63-.062l2.66 1.773l3.71-3.71a.5.5 0 0 1 .577-.094l1.777 1.947V5a1 1 0 0 0-1-1z"
            /></g
          ></svg
        >
      </div>
      <div class="flex flex-col items-center gap-2">
        <p class="text-xl font-bold text-white text-center">Upload Images</p>
        <p class="text-sm text-slate-400 text-center">
          Drop your images here or click to browse files from your computer.
          Supports PNG, JPEG, WebP, and 30+ RAW formats.
        </p>
      </div>
      <div class="flex gap-3">
        <button
          on:click={onBrowseFiles}
          class="px-6 py-3 rounded-lg bg-slate-800 text-slate-200 text-sm font-bold hover:bg-slate-700 transition-colors"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="1.5em"
            height="1.5em"
            viewBox="0 0 24 24"
            class="inline-block"
            ><g
              fill="none"
              stroke="currentColor"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              ><path d="M15 3v4a1 1 0 0 0 1 1h4" /><path
                d="M18 17h-7a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4l5 5v7a2 2 0 0 1-2 2"
              /><path
                d="M16 17v2a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V9a2 2 0 0 1 2-2h2"
              /></g
            ></svg
          > Files
        </button>
        <button
          on:click={onBrowseFolder}
          class="px-6 py-3 rounded-lg bg-slate-800 text-slate-200 text-sm font-bold hover:bg-slate-700 transition-colors"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="1.5em"
            height="1.5em"
            viewBox="0 0 24 24"
            class="inline-block"
            ><path
              fill="currentColor"
              d="M3 21q-.825 0-1.412-.587T1 19V7q0-.425.288-.712T2 6t.713.288T3 7v12h16q.425 0 .713.288T20 20t-.288.713T19 21zm4-4q-.825 0-1.412-.587T5 15V4q0-.825.588-1.412T7 2h4.175q.4 0 .763.15t.637.425L14 4h7q.825 0 1.413.588T23 6v9q0 .825-.587 1.413T21 17z"
            /></svg
          > Folder
        </button>
      </div>
    </div>
  </div>
</div>
