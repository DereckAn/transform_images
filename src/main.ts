import { convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import { ImageService } from "./app/services/ImageService";
import { AppState } from "./app/state/AppState";
import type { ProcessedImage } from "./models/types";

// Initialize services
const imageService = new ImageService();
const appState = new AppState();

// DOM Elements
let dropZone: HTMLElement;
let dropZoneView: HTMLElement;
let browsebtn: HTMLButtonElement;
let browseFolderBtn: HTMLButtonElement;
let imagesList: HTMLElement;
let imagesListView: HTMLElement;
let imageCount: HTMLElement;
let clearAllBtn: HTMLButtonElement;
let processBtn: HTMLButtonElement;
let cancelBtn: HTMLButtonElement;
let qualityInput: HTMLInputElement;
let qualityValue: HTMLElement;
let outputFormatSelect: HTMLSelectElement;
let outputDirInput: HTMLInputElement;
let selectDirBtn: HTMLButtonElement;
let preserveMetadataCheck: HTMLInputElement;
let overwriteExistingCheck: HTMLInputElement;
let progressSection: HTMLElement;
let progressBar: HTMLElement;
let progressText: HTMLElement;
let progressPercent: HTMLElement;
let currentFileText: HTMLElement;
let resultsSection: HTMLElement;
let resultsStats: HTMLElement;
let resultsList: HTMLElement;
let threadCount: HTMLElement;

// Transformation elements
let toggleTransformationsBtn: HTMLButtonElement;
let transformationsContent: HTMLElement;
let resizeWidthInput: HTMLInputElement;
let resizeHeightInput: HTMLInputElement;
let preserveAspectRatioCheck: HTMLInputElement;
let resizeFilterSelect: HTMLSelectElement;
let rotationSelect: HTMLSelectElement;
let flipHorizontalCheck: HTMLInputElement;
let flipVerticalCheck: HTMLInputElement;
let resetTransformationsBtn: HTMLButtonElement;
let transformIcon: HTMLElement;

async function initialize() {
  // Get DOM elements
  dropZone = document.getElementById("drop-zone")!;
  dropZoneView = document.getElementById("drop-zone-view")!;
  browsebtn = document.getElementById("browse-btn") as HTMLButtonElement;
  browseFolderBtn = document.getElementById(
    "browse-folder-btn"
  ) as HTMLButtonElement;
  imagesList = document.getElementById("images-list")!;
  imagesListView = document.getElementById("images-list-view")!;
  imageCount = document.getElementById("image-count")!;
  clearAllBtn = document.getElementById("clear-all-btn") as HTMLButtonElement;
  processBtn = document.getElementById("process-btn") as HTMLButtonElement;
  cancelBtn = document.getElementById("cancel-btn") as HTMLButtonElement;
  qualityInput = document.getElementById("quality") as HTMLInputElement;
  qualityValue = document.getElementById("quality-value")!;
  outputFormatSelect = document.getElementById(
    "output-format"
  ) as HTMLSelectElement;
  outputDirInput = document.getElementById("output-dir") as HTMLInputElement;
  selectDirBtn = document.getElementById("select-dir-btn") as HTMLButtonElement;
  preserveMetadataCheck = document.getElementById(
    "preserve-metadata"
  ) as HTMLInputElement;
  overwriteExistingCheck = document.getElementById(
    "overwrite-existing"
  ) as HTMLInputElement;
  progressSection = document.getElementById("progress-section")!;
  progressBar = document.getElementById("progress-bar")!;
  progressText = document.getElementById("progress-text")!;
  progressPercent = document.getElementById("progress-percent")!;
  currentFileText = document.getElementById("current-file")!;
  resultsSection = document.getElementById("results-section")!;
  resultsStats = document.getElementById("results-stats")!;
  resultsList = document.getElementById("results-list")!;
  threadCount = document.getElementById("thread-count")!;

  // Get transformation elements
  toggleTransformationsBtn = document.getElementById(
    "toggle-transformations"
  ) as HTMLButtonElement;
  transformationsContent = document.getElementById("transformations-content")!;
  resizeWidthInput = document.getElementById(
    "resize-width"
  ) as HTMLInputElement;
  resizeHeightInput = document.getElementById(
    "resize-height"
  ) as HTMLInputElement;
  preserveAspectRatioCheck = document.getElementById(
    "preserve-aspect-ratio"
  ) as HTMLInputElement;
  resizeFilterSelect = document.getElementById(
    "resize-filter"
  ) as HTMLSelectElement;
  rotationSelect = document.getElementById("rotation") as HTMLSelectElement;
  flipHorizontalCheck = document.getElementById(
    "flip-horizontal"
  ) as HTMLInputElement;
  flipVerticalCheck = document.getElementById(
    "flip-vertical"
  ) as HTMLInputElement;
  resetTransformationsBtn = document.getElementById(
    "reset-transformations"
  ) as HTMLButtonElement;
  transformIcon = document.getElementById("transform-icon") as HTMLElement;

  // Setup event listeners
  setupEventListeners();

  // Listen to Tauri file drop events
  setupTauriFileDrop();

  // Get optimal thread count
  const threads = await imageService.getOptimalThreads();
  threadCount.textContent = threads.toString();

  console.log("Transform Images App initialized!");
  console.log("Architecture: Clean + Hexagonal + Multithreading");
}

function setupEventListeners() {
  // Drop zone events - SOLO click
  dropZone.addEventListener("click", handleBrowseClick);

  // Browse button
  browsebtn.addEventListener("click", (e) => {
    e.stopPropagation();
    handleBrowseClick();
  });

  // Browse folder button
  browseFolderBtn.addEventListener("click", (e) => {
    e.stopPropagation();
    handleBrowseFolderClick();
  });

  // Settings
  qualityInput.addEventListener("input", handleQualityChange);
  outputFormatSelect.addEventListener("change", handleOutputFormatChange);
  selectDirBtn.addEventListener("click", handleSelectOutputDir);
  preserveMetadataCheck.addEventListener(
    "change",
    handlePreserveMetadataChange
  );
  overwriteExistingCheck.addEventListener(
    "change",
    handleOverwriteExistingChange
  );

  // Actions
  if (clearAllBtn) {
    clearAllBtn.addEventListener("click", handleClearImages);
  } else {
    console.error("clearAllBtn not found!");
  }

  if (processBtn) {
    processBtn.addEventListener("click", handleProcessImages);
  } else {
    console.error("processBtn not found!");
  }

  if (cancelBtn) {
    cancelBtn.addEventListener("click", handleCancelProcessing);
  } else {
    console.error("cancelBtn not found!");
  }
  // Transformations
  toggleTransformationsBtn.addEventListener(
    "click",
    handleToggleTransformations
  );
  resizeWidthInput.addEventListener("input", handleResizeChange);
  resizeHeightInput.addEventListener("input", handleResizeChange);
  preserveAspectRatioCheck.addEventListener("change", handleResizeChange);
  resizeFilterSelect.addEventListener("change", handleResizeChange);
  rotationSelect.addEventListener("change", handleRotationChange);
  flipHorizontalCheck.addEventListener("change", handleFlipChange);
  flipVerticalCheck.addEventListener("change", handleFlipChange);
  resetTransformationsBtn.addEventListener("click", handleResetTransformations);

  // Progress events
  imageService.onProgress((current, total, file, percentage) => {
    updateProgress(current, total, file, percentage);
  });
}

// Setup Tauri file drop listener
async function setupTauriFileDrop() {
  console.log("Setting up Tauri file drop listeners...");

  try {
    // Tauri emite eventos cuando se arrastran archivos
    await listen("tauri://file-drop", (event) => {
      console.log("🎯 FILE DROP EVENT RECEIVED!");
      console.log("Event:", event);
      const paths = event.payload as string[];
      console.log("Paths:", paths);
      handleFileDrop(paths);
    });
    console.log("✅ file-drop listener registered");

    // También escuchar hover events para visual feedback
    await listen("tauri://file-drop-hover", () => {
      console.log("🔵 HOVER EVENT");
      dropZone.classList.add("drag-over");
    });
    console.log("✅ file-drop-hover listener registered");

    await listen("tauri://file-drop-cancelled", () => {
      console.log("❌ DROP CANCELLED");
      dropZone.classList.remove("drag-over");
    });
    console.log("✅ file-drop-cancelled listener registered");

    console.log("All Tauri listeners setup complete!");
  } catch (error) {
    console.error("❌ Error setting up Tauri listeners:", error);
  }
}

// Event Handlers
async function handleFileDrop(paths: string[]) {
  console.log("📥 handleFileDrop called with:", paths);
  dropZone.classList.remove("drag-over");
  await loadImagePaths(paths);
}

async function handleBrowseClick() {
  try {
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "Images",
          extensions: [
            "png",
            "jpg",
            "jpeg",
            "webp",
            "gif",
            // RAW formats
            "arw",
            "cr2",
            "cr3",
            "nef",
            "nrw",
            "dng",
            "raf",
            "orf",
            "rw2",
            "pef",
            "srw",
            "x3f",
            "raw",
            "rwl",
            "mrw",
            "erf",
            "3fr",
            "ari",
            "srf",
            "sr2",
            "bay",
            "crw",
            "iiq",
            "k25",
            "kdc",
            "mef",
            "mos",
            "r3d",
          ],
        },
      ],
    });

    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected];
      await loadImagePaths(paths);
    }
  } catch (error) {
    console.error("Error selecting files:", error);
  }
}

async function handleBrowseFolderClick() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (selected && typeof selected === "string") {
      await loadImagesFromFolder(selected);
    }
  } catch (error) {
    console.error("Error selecting folder:", error);
  }
}

async function loadImagePaths(paths: string[]) {
  console.log("🔄 loadImagePaths called with:", paths);
  try {
    const images = await imageService.loadImagesInfo(paths);
    console.log("✅ Images loaded:", images);
    appState.addImages(images);
    updateUI();
  } catch (error) {
    console.error("Error loading images:", error);
    alert(`Error loading images: ${error}`);
  }
}

async function loadImagesFromFolder(folderPath: string) {
  console.log("🔄 loadImagesFromFolder called with:", folderPath);
  try {
    const images = await imageService.loadImagesFromFolder(folderPath);
    console.log("✅ Images loaded from folder:", images);
    appState.addImages(images);
    updateUI();
  } catch (error) {
    console.error("Error loading images from folder:", error);
    alert(`Error loading images from folder: ${error}`);
  }
}

function handleQualityChange() {
  const value = parseInt(qualityInput.value);
  qualityValue.textContent = value.toString();
  appState.setQuality(value);
}

function handleOutputFormatChange() {
  const format = outputFormatSelect.value || undefined;
  appState.setOutputFormat(format);
}

async function handleSelectOutputDir() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (selected && typeof selected === "string") {
      outputDirInput.value = selected;
      appState.setOutputDirectory(selected);
      updateProcessButton();
    }
  } catch (error) {
    console.error("Error selecting directory:", error);
  }
}

function handlePreserveMetadataChange() {
  appState.setPreserveMetadata(preserveMetadataCheck.checked);
}

function handleOverwriteExistingChange() {
  appState.setOverwriteExisting(overwriteExistingCheck.checked);
}

// Transformation handlers
function handleToggleTransformations() {
  const isHidden = transformationsContent.style.display === "none";
  transformationsContent.style.display = isHidden ? "flex" : "none";
  if (transformIcon) {
    transformIcon.style.transform = isHidden
      ? "rotate(0deg)"
      : "rotate(-90deg)";
  }
}

function handleResizeChange() {
  const width = resizeWidthInput.value
    ? parseInt(resizeWidthInput.value)
    : null;
  const height = resizeHeightInput.value
    ? parseInt(resizeHeightInput.value)
    : null;
  const preserveAspect = preserveAspectRatioCheck.checked;
  const filter = resizeFilterSelect.value;

  appState.setResize(width, height, preserveAspect, filter);
}

function handleRotationChange() {
  const degrees = parseInt(rotationSelect.value);
  appState.setRotation(degrees);
}

function handleFlipChange() {
  appState.setFlipHorizontal(flipHorizontalCheck.checked);
  appState.setFlipVertical(flipVerticalCheck.checked);
}

function handleResetTransformations() {
  // Reset AppState
  appState.resetTransformations();

  // Reset UI
  resizeWidthInput.value = "";
  resizeHeightInput.value = "";
  preserveAspectRatioCheck.checked = true;
  resizeFilterSelect.value = "Lanczos3";
  rotationSelect.value = "0";
  flipHorizontalCheck.checked = false;
  flipVerticalCheck.checked = false;
}

function handleClearImages() {
  appState.clearImages();
  updateUI();

  // Show empty state, hide images section
  const dropZoneView = document.getElementById("drop-zone-view");
  const imagesListView = document.getElementById("images-list-view");

  if (dropZoneView && imagesListView) {
    dropZoneView.style.display = "flex";
    imagesListView.style.display = "none";
  }
}

function handleRemoveImage(index: number) {
  appState.removeImage(index);
  updateUI();

  // If no images left, show empty state
  if (appState.images.length === 0) {
    const dropZoneView = document.getElementById("drop-zone-view");
    const imagesListView = document.getElementById("images-list-view");

    if (dropZoneView && imagesListView) {
      dropZoneView.style.display = "flex";
      imagesListView.style.display = "none";
    }
  }
}

async function handleProcessImages() {
  if (appState.images.length === 0 || !appState.outputDirectory) {
    return;
  }

  // Hide results, show progress
  resultsSection.style.display = "none";
  progressSection.style.display = "block";
  appState.setProcessing(true);
  updateProcessButton();

  try {
    const request = {
      imagePaths: appState.images.map((img) => img.path),
      optimizationOptions: appState.options,
      transformationOptions: appState.hasTransformations()
        ? appState.transformations
        : undefined,
    };

    console.log(
      "Processing with transformations:",
      request.transformationOptions
    );

    const results = await imageService.processImages(request);
    displayResults(results);
  } catch (error) {
    console.error("Processing error:", error);
    alert(`Processing failed: ${error}`);
  } finally {
    appState.setProcessing(false);
    progressSection.style.display = "none";
    updateProcessButton();
  }
}

async function handleCancelProcessing() {
  try {
    await imageService.cancelProcessing();
    appState.setProcessing(false);
    progressSection.style.display = "none";
    updateProcessButton();
  } catch (error) {
    console.error("Cancel error:", error);
  }
}

function renderImagesList() {
  imagesList.innerHTML = "";
  appState.images.forEach((img, index) => {
    const isRaw = img.format.toLowerCase() === "raw";

    // Convert file path to URL that browser can load
    const imageUrl = convertFileSrc(img.path);

    const item = document.createElement("div");
    item.className =
      "relative group rounded-xl overflow-hidden bg-slate-900 border border-slate-800 hover:border-slate-700 transition-all";

    item.innerHTML = `
        <div class="aspect-square bg-slate-800 flex items-center justify-center relative overflow-hidden">
          ${
            isRaw
              ? // RAW files: Show icon (can't preview RAW directly in  browser)
                `<div class="text-4xl">RAW IMAGE</div>`
              : // Standard formats: Show actual image preview
                `<img
                  src="${imageUrl}"
                  alt="${getFileName(img.path)}"
                  class="w-full h-full object-cover"
                  loading="lazy"
                  onerror="this.style.display='none';
                    this.nextElementSibling.style.display='flex';"
                />
                <div class="hidden w-full h-full items-center justify-center text-4xl">
                    RAW IMAGE
                </div>`
          }

          <!-- Remove button (shows on hover) -->
          <button class="remove-image-btn absolute top-2 right-2 w-6 h-6
            rounded-full bg-red-600/90 hover:bg-red-700 text-white flex
            items-center justify-center opacity-0 group-hover:opacity-100
            transition-all hover:scale-110 z-10"
            data-index="${index}"
            title="Remove image"
          >
            ✕
          </button>
        </div>
        <div class="p-3">
          <div class="flex items-start justify-between gap-2 mb-2">
            <p class="text-sm font-semibold text-white truncate flex-1" title="${getFileName(
              img.path
            )}">
              ${getFileName(img.path)}
            </p>
            ${
              isRaw
                ? '<span class="badge-raw">RAW</span>'
                : '<span class="badge-standard">' +
                  img.format.toUpperCase() +
                  "</span>"
            }
          </div>
          <div class="flex items-center gap-2 text-xs text-slate-400">
            <span>${img.width}×${img.height}</span>
            <span>•</span>
            <span>${formatBytes(img.sizeBytes)}</span>
          </div>

          <!-- Progress bar for this image (hidden initially) -->
          <div class="image-progress mt-2 h-1 bg-slate-800 rounded-full overflow-hidden" data-index="${index}" style="display: none;">
            <div class="h-full bg-primary transition-all duration-300" style="width: 0%"></div>
          </div>
        </div>
      `;

    imagesList.appendChild(item);
  });

  // Add click handlers for remove buttons
  const removeButtons = imagesList.querySelectorAll(".remove-image-btn");
  removeButtons.forEach((button) => {
    button.addEventListener("click", (e) => {
      e.stopPropagation();
      const index = parseInt(
        (e.target as HTMLElement).getAttribute("data-index") || "0"
      );
      handleRemoveImage(index);
    });
  });
}

// UI Updates
function updateUI() {
  // Update image count
  imageCount.textContent = appState.images.length.toString();

  // ✅ Alternar entre drop zone y lista en EL MISMO LUGAR
  if (appState.images.length === 0) {
    dropZoneView.style.display = "block";
    imagesListView.style.display = "none";
  } else {
    dropZoneView.style.display = "none";
    imagesListView.style.display = "block";
    renderImagesList();
  }

  updateProcessButton();
}

function updateProcessButton() {
  const canProcess =
    appState.images.length > 0 &&
    appState.outputDirectory &&
    !appState.isProcessing;

  processBtn.disabled = !canProcess;
  processBtn.style.display = appState.isProcessing ? "none" : "block";
  cancelBtn.style.display = appState.isProcessing ? "block" : "none";
}

function updateProgress(
  current: number,
  total: number,
  file: string,
  percentage: number
) {
  appState.updateProgress(current, total, file, percentage);

  progressText.textContent = `Processing: ${current} / ${total}`;
  progressPercent.textContent = `${percentage.toFixed(0)}%`;
  progressBar.style.width = `${percentage}%`;
  currentFileText.textContent = `Current: ${file}`;
}

function displayResults(results: ProcessedImage[]) {
  resultsSection.style.display = "block";

  const successful = results.filter((r) => r.success).length;
  const failed = results.length - successful;
  const totalSaved = results
    .filter((r) => r.success)
    .reduce((sum, r) => sum + (r.originalSize - r.outputSize), 0);
  const avgCompression =
    results
      .filter((r) => r.success)
      .reduce((sum, r) => sum + r.compressionRatio, 0) / (successful || 1);

  // Display stats
  resultsStats.innerHTML = `
      <div class="bg-slate-800 p-4 rounded-lg text-center">
        <p class="text-2xl font-bold text-white">${successful}</p>
        <p class="text-xs text-slate-400 uppercase tracking-wider 
  mt-1">Successful</p>
      </div>
      <div class="bg-slate-800 p-4 rounded-lg text-center">
        <p class="text-2xl font-bold text-red-400">${failed}</p>
        <p class="text-xs text-slate-400 uppercase tracking-wider 
  mt-1">Failed</p>
      </div>
      <div class="bg-slate-800 p-4 rounded-lg text-center">
        <p class="text-2xl font-bold 
  text-green-400">${formatBytes(totalSaved)}</p>
        <p class="text-xs text-slate-400 uppercase tracking-wider 
  mt-1">Space Saved</p>
      </div>
      <div class="bg-slate-800 p-4 rounded-lg text-center">
        <p class="text-2xl font-bold 
  text-blue-400">${avgCompression.toFixed(1)}%</p>
        <p class="text-xs text-slate-400 uppercase tracking-wider 
  mt-1">Avg Compression</p>
      </div>
    `;

  // Display results list
  resultsList.innerHTML = "";
  results.forEach((result) => {
    const item = document.createElement("div");
    item.className = `p-3 rounded-lg border ${
      result.success
        ? "bg-green-500/10 border-green-500/30"
        : "bg-red-500/10 border-red-500/30"
    }`;

    item.innerHTML = `
        <div class="flex items-center justify-between">
          <p class="text-sm font-medium text-white truncate flex-1">${getFileName(
            result.originalPath
          )}</p>
          ${
            result.success
              ? `<span class="text-xs font-semibold text-green-400">${result.compressionRatio.toFixed(
                  1
                )}% smaller</span>`
              : `<span class="text-xs font-semibold text-red-400">Failed</span>`
          }
        </div>
        ${
          result.success
            ? `<p class="text-xs text-slate-400 mt-1">${formatBytes(
                result.originalSize
              )} → ${formatBytes(result.outputSize)}</p>`
            : `<p class="text-xs text-red-400 mt-1">${result.errorMessage}</p>`
        }
      `;

    resultsList.appendChild(item);
  });
}

// Utility Functions
function getFileName(path: string): string {
  return path.split(/[\\/]/).pop() || path;
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
}

// Initialize app when DOM is ready
window.addEventListener("DOMContentLoaded", initialize);
