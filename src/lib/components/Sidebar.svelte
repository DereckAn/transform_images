<script lang="ts">
  import FormatSelector from "./sidebar/FormatSelector.svelte";
  import MetadataOptions from "./sidebar/MetadataOptions.svelte";
  import OutputDirectoryPicker from "./sidebar/OutputDirectoryPicker.svelte";
  import QualitySlider from "./sidebar/QualitySlider.svelte";
  import TransformationsPanel from "./sidebar/TransformationsPanel.svelte";

  export let quality: number;
  export let outputFormat: string | undefined;
  export let outputDirectory: string;
  export let preserveMetadata: boolean;
  export let overwriteExisting: boolean;
  export let showTransformations: boolean;
  export let resizeWidth: number | null;
  export let resizeHeight: number | null;
  export let preserveAspectRatio: boolean;
  export let resizeFilter: string;
  export let rotation: number;
  export let flipHorizontal: boolean;
  export let flipVertical: boolean;

  export let onSelectOutputDir: () => void;
  export let onResetTransformations: () => void;
</script>

<aside class="flex w-80 flex-col border-r border-slate-800 bg-background">
  <div class="flex flex-col gap-4 p-6 overflow-y-auto scrollbar-thin h-full">
    <h3 class="text-sm font-semibold text-slate-400 uppercase tracking-wider">
      Optimization Settings
    </h3>

    <QualitySlider bind:quality />
    <FormatSelector bind:outputFormat />
    <OutputDirectoryPicker {outputDirectory} {onSelectOutputDir} />
    <MetadataOptions bind:preserveMetadata bind:overwriteExisting />

    <TransformationsPanel
      bind:showTransformations
      bind:resizeWidth
      bind:resizeHeight
      bind:preserveAspectRatio
      bind:resizeFilter
      bind:rotation
      bind:flipHorizontal
      bind:flipVertical
      {onResetTransformations}
    />
  </div>
</aside>
