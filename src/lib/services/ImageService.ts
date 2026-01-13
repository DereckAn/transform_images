import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn, } from "@tauri-apps/api/event";
import type {
  ImageInfo,
  OptimizationOptions,
  ProcessedImage,
  ProgressPayload,
  TransformationOptions,
} from "../models/types";

export interface BatchProcessRequest {
  imagePaths: string[];
  optimizationOptions: OptimizationOptions;
  transformationOptions?: TransformationOptions;
}

export interface ProcessingStats {
  totalProcessed: number;
  totalSavedBytes: number;
  averageSavings: number;
}

export class ImageService {
  /**
   * Test backend connection
   */
  async greet(name: string): Promise<string> {
    return invoke("greet", { name });
  }

  /**
   * Load single image metadata
   */
  async loadImageInfo(path: string): Promise<ImageInfo> {
    return invoke("load_image_info", { path });
  }

  /**
   * Load multiple images metadata
   */
  async loadImagesInfo(paths: string[]): Promise<ImageInfo[]> {
    return invoke("load_images_info", { paths });
  }

  /**
   * Load images from a folder
   */
  async loadImagesFromFolder(folderPath: string): Promise<ImageInfo[]> {
    return invoke("load_images_from_folder", { folderPath });
  }

  /**
   * Process images with optimization and optional transformations
   */
  async processImages(request: BatchProcessRequest): Promise<ProcessedImage[]> {
    return invoke("process_images", { request });
  }

  /**
   * Cancel ongoing processing operation
   */
  async cancelProcessing(): Promise<void> {
    return invoke("cancel_processing");
  }

  /**
   * Get current processing status
   */
  async getProcessingStatus(): Promise<string> {
    return invoke("get_processing_status");
  }

  /**
   * Check if processing is running
   */
  async isProcessing(): Promise<boolean> {
    return invoke("is_processing");
  }

  /**
   * Get processing statistics
   */
  async getStats(): Promise<ProcessingStats> {
    return invoke("get_stats");
  }

  /**
   * Reset processing statistics
   */
  async resetStats(): Promise<void> {
    return invoke("reset_stats");
  }

  /**
   * Get optimal thread count
   */
  async getOptimalThreads(): Promise<number> {
    return invoke("get_optimal_threads");
  }

  /**
   * Listen to progress events
   */
  onProgress(
    callback: (
      current: number,
      total: number,
      file: string,
      percentage: number
    ) => void
  ): Promise<UnlistenFn> {
    return listen<ProgressPayload>("processing-progress", (event) => {
      callback(
        event.payload.current,
        event.payload.total,
        event.payload.currentFile,
        event.payload.percentage
      );
    });
  }
}
