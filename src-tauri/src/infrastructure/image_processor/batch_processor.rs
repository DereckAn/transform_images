use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use crate::domain::{
    DomainError, DomainResult, Image, ImageProcessor, ProcessingSettings, Transformation,
};
use crate::infrastructure::image_processor::ImageProcessorImpl;

/// Result of processing a single image
#[derive(Debug, Clone)]
pub struct ProcessingResult {
    pub original_path: PathBuf,
    pub output_path: PathBuf,
    pub original_size: u64,
    pub output_size: u64,
    pub success: bool,
    pub error_message: Option<String>,
}

impl ProcessingResult {
    /// Calculate compression ratio (percentage saved)
    pub fn compression_ratio(&self) -> f64 {
        if self.original_size == 0 {
            return 0.0;
        }
        let saved = self.original_size.saturating_sub(self.output_size) as f64;
        (saved / self.original_size as f64) * 100.0
    }

    /// Calculate size reduction in bytes
    pub fn bytes_saved(&self) -> u64 {
        self.original_size.saturating_sub(self.output_size)
    }
}

/// Progress callback function type
pub type ProgressCallback = Arc<dyn Fn(usize, usize, &str) + Send + Sync>;

/// Batch processor for processing multiple images in parallel
pub struct BatchProcessor {
    max_threads: Option<usize>,
}

impl BatchProcessor {
    /// Create a new batch processor
    pub fn new() -> Self {
        Self {
            max_threads: None,
        }
    }

    /// Create with custom thread pool size
    pub fn with_threads(max_threads: usize) -> Self {
        Self {
            max_threads: Some(max_threads),
        }
    }

    /// Process multiple images in parallel
    pub fn process_batch(
        &self,
        images: Vec<Image>,
        transformation: Option<Transformation>,
        settings: ProcessingSettings,
        cancel_signal: Arc<AtomicBool>,
        progress_callback: Option<ProgressCallback>,
    ) -> Vec<ProcessingResult> {
        let total = images.len();
        let counter = Arc::new(AtomicUsize::new(0));

        // Configurar pool de threads si se especificó
        let pool = if let Some(threads) = self.max_threads {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .ok()
        } else {
            None
        };

        // Función para procesar cada imagen
        let process_one = |img: &Image| -> ProcessingResult {
            // Verificar señal de cancelación
            if cancel_signal.load(Ordering::SeqCst) {
                return ProcessingResult {
                    original_path: img.path().to_path_buf(),
                    output_path: PathBuf::new(),
                    original_size: img.size_bytes(),
                    output_size: 0,
                    success: false,
                    error_message: Some("Operation cancelled".to_string()),
                };
            }

            let result = self.process_single_image(img, transformation.as_ref(), &settings);

            // Actualizar progreso
            let count = counter.fetch_add(1, Ordering::SeqCst) + 1;
            if let Some(ref callback) = progress_callback {
                let file_name = img.file_name().unwrap_or("unknown");
                callback(count, total, file_name);
            }

            result
        };

        // Procesar en paralelo
        if let Some(pool) = pool {
            pool.install(|| images.par_iter().map(process_one).collect())
        } else {
            images.par_iter().map(process_one).collect()
        }
    }

    /// Process a single image
    fn process_single_image(
        &self,
        image: &Image,
        transformation: Option<&Transformation>,
        settings: &ProcessingSettings,
    ) -> ProcessingResult {
        let original_path = image.path().to_path_buf();
        let original_size = image.size_bytes();

        // Crear procesador para este thread (stateless, barato de crear)
        let processor = ImageProcessorImpl::new();

        // Determinar ruta de salida
        let output_path = match self.determine_output_path(image, settings) {
            Ok(path) => path,
            Err(e) => {
                return ProcessingResult {
                    original_path,
                    output_path: PathBuf::new(),
                    original_size,
                    output_size: 0,
                    success: false,
                    error_message: Some(e.to_string()),
                };
            }
        };

        // Procesar imagen
        match processor.process(image, transformation, settings) {
            Ok(data) => {
                let output_size = data.len() as u64;

                // Guardar archivo
                match processor.save_image(
                    &data,
                    &output_path,
                    settings.determine_output_format(image.format()),
                ) {
                    Ok(_) => ProcessingResult {
                        original_path,
                        output_path,
                        original_size,
                        output_size,
                        success: true,
                        error_message: None,
                    },
                    Err(e) => ProcessingResult {
                        original_path,
                        output_path: PathBuf::new(),
                        original_size,
                        output_size: 0,
                        success: false,
                        error_message: Some(format!("Failed to save: {}", e)),
                    },
                }
            }
            Err(e) => ProcessingResult {
                original_path,
                output_path: PathBuf::new(),
                original_size,
                output_size: 0,
                success: false,
                error_message: Some(format!("Processing failed: {}", e)),
            },
        }
    }

    /// Determine output file path
    fn determine_output_path(
        &self,
        image: &Image,
        settings: &ProcessingSettings,
    ) -> DomainResult<PathBuf> {
        let output_format = settings.determine_output_format(image.format());
        let file_stem = image
            .file_stem()
            .ok_or_else(|| DomainError::InvalidFilePath("No file name".to_string()))?;

        let output_filename = format!("{}.{}", file_stem, output_format.extension());
        let output_path = settings.output_directory().join(output_filename);

        // Verificar si el archivo existe y no queremos sobrescribir
        if output_path.exists() && !settings.overwrite_existing() {
            return Err(DomainError::InvalidFilePath(format!(
                "File already exists: {}",
                output_path.display()
            )));
        }

        Ok(output_path)
    }

    /// Get optimal number of threads for processing
    pub fn optimal_thread_count() -> usize {
        // Usar número de CPUs disponibles
        rayon::current_num_threads()
    }
}

impl Default for BatchProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_batch_processor() {
        let processor = BatchProcessor::new();
        assert!(processor.max_threads.is_none());
    }

    #[test]
    fn test_create_with_threads() {
        let processor = BatchProcessor::with_threads(4);
        assert_eq!(processor.max_threads, Some(4));
    }

    #[test]
    fn test_optimal_thread_count() {
        let count = BatchProcessor::optimal_thread_count();
        assert!(count > 0);
    }

    #[test]
    fn test_compression_ratio() {
        let result = ProcessingResult {
            original_path: PathBuf::from("test.png"),
            output_path: PathBuf::from("out.png"),
            original_size: 1000,
            output_size: 500,
            success: true,
            error_message: None,
        };

        assert_eq!(result.compression_ratio(), 50.0);
        assert_eq!(result.bytes_saved(), 500);
    }
}
