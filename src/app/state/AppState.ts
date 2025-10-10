import type { ImageInfo, OptimizationOptions } from "../../models/types";

export class AppState {
  images: ImageInfo[] = [];
  isProcessing: boolean = false;
  progress: {
    current: number;
    total: number;
    percentage: number;
    currentFile: string;
  } = { current: 0, total: 0, percentage: 0, currentFile: "" };

  outputDirectory: string = "";

  options: OptimizationOptions = {
    quality: 85,
    outputDirectory: "",
    preserveMetadata: false,
    overwriteExisting: false,
  };

  addImages(newImages: ImageInfo[]) {
    this.images.push(...newImages);
  }

  clearImages() {
    this.images = [];
  }

  setProcessing(value: boolean) {
    this.isProcessing = value;
  }

  updateProgress(
    current: number,
    total: number,
    currentFile: string,
    percentage: number
  ) {
    this.progress = { current, total, currentFile, percentage };
  }

  resetProgress() {
    this.progress = { current: 0, total: 0, percentage: 0, currentFile: "" };
  }

  setOutputDirectory(dir: string) {
    this.outputDirectory = dir;
    this.options.outputDirectory = dir;
  }

  setQuality(quality: number) {
    this.options.quality = quality;
  }

  setPreserveMetadata(value: boolean) {
    this.options.preserveMetadata = value;
  }

  setOverwriteExisting(value: boolean) {
    this.options.overwriteExisting = value;
  }

  setOutputFormat(format: string | undefined) {
    this.options.outputFormat = format;
  }
}
