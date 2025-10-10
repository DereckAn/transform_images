import type { ImageInfo, OptimizationOptions } from "../../models/types";

export class AppState {
  images: ImageInfo[] = [];
  isProcessing: boolean = false;
  progress: { current: number; total: number } = { current: 0, total: 0 };
  outputDirectory: string = "";
  options: OptimizationOptions = {
    quality: 85,
    outputDirectory: "",
    preserveMetadata: false,
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

  updateProgress(current: number, total: number) {
    this.progress = { current, total };
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
}
