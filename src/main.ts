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
let clearBtn: HTMLButtonElement;
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

async function initialize() {
  // Get DOM elements
  dropZone = document.getElementById("drop-zone")!;
  dropZoneView = document.getElementById("drop-zone-view")!;
  browsebtn = document.getElementById("browse-btn") as HTMLButtonElement;
  browseFolderBtn = document.getElementById("browse-folder-btn") as HTMLButtonElement;
  imagesList = document.getElementById("images-list")!;
  imagesListView = document.getElementById("images-list-view")!;
  imageCount = document.getElementById("image-count")!;
  clearBtn = document.getElementById("clear-btn") as HTMLButtonElement;
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
  clearBtn.addEventListener("click", handleClearImages);
  processBtn.addEventListener("click", handleProcessImages);
  cancelBtn.addEventListener("click", handleCancelProcessing);

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
          extensions: ["png", "jpg", "jpeg", "webp", "gif"],
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

function handleClearImages() {
  appState.clearImages();
  updateUI();
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
      transformationOptions: undefined, // Transformations en Fase 6
    };

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

function renderImagesList() {
  // Clear existing items
  imagesList.innerHTML = "";

  // Add image items
  appState.images.forEach((img) => {
    const item = document.createElement("div");
    item.className = "image-item";
    item.innerHTML = `
        <div class="image-info">
          <div class="image-name">${getFileName(img.path)}</div>
          <div class="image-meta">
            <span>${img.format.toUpperCase()}</span>
            <span>${img.width}x${img.height}</span>
            <span>${formatBytes(img.sizeBytes)}</span>
          </div>
        </div>
      `;
    imagesList.appendChild(item);
  });
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

  // Calculate stats
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
      <div class="stat-card">
        <div class="stat-value">${successful}</div>
        <div class="stat-label">Successful</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">${failed}</div>
        <div class="stat-label">Failed</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">${formatBytes(totalSaved)}</div>
        <div class="stat-label">Space Saved</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">${avgCompression.toFixed(1)}%</div>
        <div class="stat-label">Avg Compression</div>
      </div>
    `;

  // Display results list
  resultsList.innerHTML = "";
  results.forEach((result) => {
    const item = document.createElement("div");
    item.className = `result-item ${result.success ? "success" : "error"}`;
    item.innerHTML = `
        <div 
  class="image-name">${getFileName(result.originalPath)}</div>
        ${
          result.success
            ? `
          <div class="image-meta">
            <span>${formatBytes(result.originalSize)} → 
  ${formatBytes(result.outputSize)}</span>
            <span>${result.compressionRatio.toFixed(1)}% 
  smaller</span>
          </div>
        `
            : `<div style="color: var(--danger); margin-top: 
  0.5rem;">${result.errorMessage}</div>`
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
