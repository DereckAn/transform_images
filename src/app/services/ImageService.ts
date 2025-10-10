import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import type {
  ImageInfo,
  OptimizationOptions,
  ProcessedImage,
  ProgressPayload,
} from "../../models/types";

export class ImageService {
  /**
   * Test backend connection
   */
  async greet(name: string): Promise<string> {
    return invoke("greet", { name });
  }

  /**
   * Optimize images
   * TODO: Implement in Phase 4
   */
  async optimizeImages(
    images: ImageInfo[],
    options: OptimizationOptions
  ): Promise<ProcessedImage[]> {
    return invoke("optimize_images", { images, options });
  }

  /**
   * Cancel ongoing operation
   * TODO: Implement in Phase 4
   */
  async cancelOperation(): Promise<void> {
    return invoke("cancel_operation");
  }

  /**
   * Listen to progress events
   */
  onProgress(
    callback: (current: number, total: number, file: string) => void
  ): Promise<UnlistenFn> {
    return listen<ProgressPayload>("progress", (event) => {
      callback(
        event.payload.current,
        event.payload.total,
        event.payload.currentFile
      );
    });
  }
}
