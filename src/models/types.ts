// Type definitions for frontend

export interface ImageInfo {
  path: string;
  format: string;
  width: number;
  height: number;
  sizeBytes: number;
}

export interface OptimizationOptions {
  quality: number;
  outputFormat?: string;
  outputDirectory: string;
  preserveMetadata: boolean;
}

export interface TransformationOptions {
  resize?: {
    width: number;
    height: number;
    preserveAspectRatio: boolean;
  };
  rotate?: number;
  flipHorizontal: boolean;
  flipVertical: boolean;
}

export interface ProcessedImage {
  originalPath: string;
  outputPath: string;
  originalSize: number;
  outputSize: number;
  compressionRatio: number;
}

export interface ProgressPayload {
  current: number;
  total: number;
  currentFile: string;
}
