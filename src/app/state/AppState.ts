import type {
  ImageInfo,
  OptimizationOptions,
  TransformationOptions,
} from "../../models/types";

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

  transformations: TransformationOptions = {
    flipHorizontal: false,
    flipVertical: false,
  };

  addImages(newImages: ImageInfo[]) {
    this.images.push(...newImages);
  }

  clearImages(): void {
    this.images = [];
  }

  removeImage(index: number): void {
    if (index >= 0 && index < this.images.length) {
      this.images.splice(index, 1);
    }
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

  // Transformation methods
  setResize(
    width: number | null,
    height: number | null,
    preserveAspectRatio: boolean,
    filter: string
  ) {
    if (width && height) {
      this.transformations.resize = {
        width,
        height,
        preserveAspectRatio,
        filter,
      };
    } else {
      this.transformations.resize = undefined;
    }
  }

  setRotation(degrees: number) {
    if (degrees === 0) {
      this.transformations.rotate = undefined;
    } else {
      this.transformations.rotate = degrees;
    }
  }

  setFlipHorizontal(value: boolean) {
    this.transformations.flipHorizontal = value;
  }

  setFlipVertical(value: boolean) {
    this.transformations.flipVertical = value;
  }

  resetTransformations() {
    this.transformations = {
      flipHorizontal: false,
      flipVertical: false,
    };
  }

  hasTransformations(): boolean {
    return !!(
      this.transformations.resize ||
      this.transformations.rotate ||
      this.transformations.flipHorizontal ||
      this.transformations.flipVertical
    );
  }
}
