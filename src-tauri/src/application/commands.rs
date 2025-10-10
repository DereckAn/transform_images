use std::sync::Arc;
use tauri::{Emitter, State, Window};

use crate::application::dto::{
    BatchProcessRequest, ImageDto, ProcessedImageDto, ProcessingStatsDto, ProgressPayload,
};
use crate::application::state::AppState;
use crate::domain::{Image, ImageProcessor};
use crate::infrastructure::image_processor::{ImageProcessorImpl, ProgressCallback};

/// Test command - greet
#[tauri::command]
pub fn greet(name: &str, _state: State<AppState>) -> String {
    format!("Hello, {}! Transform Images is ready.", name)
}

/// Load image metadata from file path
#[tauri::command]
pub async fn load_image_info(path: String) -> Result<ImageDto, String> {
    let processor = ImageProcessorImpl::new();
    let image = processor
        .load_image(std::path::Path::new(&path))
        .map_err(|e| e.to_string())?;

    Ok(ImageDto::from(&image))
}

/// Load multiple images metadata
#[tauri::command]
pub async fn load_images_info(paths: Vec<String>) -> Result<Vec<ImageDto>, String> {
    let processor = ImageProcessorImpl::new();
    let mut images = Vec::new();

    for path in paths {
        match processor.load_image(std::path::Path::new(&path)) {
            Ok(image) => images.push(ImageDto::from(&image)),
            Err(e) => {
                eprintln!("Failed to load {}: {}", path, e);
                // Continuar con las demás imágenes
            }
        }
    }

    if images.is_empty() {
        return Err("No valid images found".to_string());
    }

    Ok(images)
}

/// Process a batch of images
#[tauri::command]
pub async fn process_images(
    request: BatchProcessRequest,
    state: State<'_, AppState>,
    window: Window,
) -> Result<Vec<ProcessedImageDto>, String> {
    // Verificar que no haya una tarea corriendo
    if state.task_manager.is_running().await {
        return Err("A processing task is already 
  running"
            .to_string());
    }

    // Cargar imágenes
    let processor = ImageProcessorImpl::new();
    let mut images = Vec::new();

    for path in request.image_paths {
        match processor.load_image(std::path::Path::new(&path)) {
            Ok(image) => images.push(image),
            Err(e) => {
                eprintln!("Failed to load {}: {}", path, e);
            }
        }
    }

    if images.is_empty() {
        return Err("No valid images to process".to_string());
    }

    // Convertir DTOs a domain models
    let settings = request.optimization_options.to_domain()?;

    let transformation = if let Some(trans_dto) = request.transformation_options {
        trans_dto.to_domain()?
    } else {
        None
    };

    // Crear callback de progreso
    let progress_callback: ProgressCallback = Arc::new(move |current, total, file_name| {
        let payload = ProgressPayload::new(current, total, file_name.to_string());

        // Emitir evento de progreso
        if let Err(e) = window.emit("processing-progress", &payload) {
            eprintln!("Failed to emit progress: {}", e);
        }
    });

    // Procesar imágenes
    let results = state
        .task_manager
        .process_images(images, transformation, settings, Some(progress_callback))
        .await?;

    // Actualizar estadísticas
    for result in &results {
        if result.success {
            state.update_stats(result.bytes_saved());
        }
    }

    // Convertir resultados a DTOs
    Ok(results.into_iter().map(ProcessedImageDto::from).collect())
}

/// Cancel current processing operation
#[tauri::command]
pub async fn cancel_processing(state: State<'_, AppState>) -> Result<(), String> {
    state.task_manager.cancel().await;
    Ok(())
}

/// Get current processing status
#[tauri::command]
pub async fn get_processing_status(state: State<'_, AppState>) -> Result<String, String> {
    let status = state.task_manager.get_status().await;
    Ok(format!("{:?}", status))
}

/// Check if processing is running
#[tauri::command]
pub async fn is_processing(state: State<'_, AppState>) -> Result<bool, String> {
    Ok(state.task_manager.is_running().await)
}

/// Get processing statistics
#[tauri::command]
pub async fn get_stats(state: State<'_, AppState>) -> Result<ProcessingStatsDto, String> {
    let stats = state.get_stats();
    Ok(ProcessingStatsDto {
        total_processed: stats.total_processed,
        total_saved_bytes: stats.total_saved_bytes,
        average_savings: stats.average_savings(),
    })
}

/// Reset processing statistics
#[tauri::command]
pub async fn reset_stats(state: State<'_, AppState>) -> Result<(), String> {
    state.reset_stats();
    Ok(())
}

/// Get optimal thread count for processing
#[tauri::command]
pub fn get_optimal_threads() -> usize {
    use crate::infrastructure::image_processor::BatchProcessor;
    BatchProcessor::optimal_thread_count()
}
