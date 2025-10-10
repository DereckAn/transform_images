use crate::domain::value_objects::Quality;
use crate::infrastructure::error::{InfraError, InfraResult};
use mozjpeg::{ColorSpace, Compress, ScanMode};
use std::io::Cursor;

/// JPEG optimizer using mozjpeg
pub struct JpegOptimizer;

impl JpegOptimizer {
    pub fn new() -> Self {
        Self
    }

    /// Optimize JPEG image
    pub fn optimize(
        &self,
        width: usize,
        height: usize,
        rgb_data: &[u8],
        quality: Quality,
    ) -> InfraResult<Vec<u8>> {
        // Crear compresor
        let mut comp = Compress::new(ColorSpace::JCS_RGB);

        comp.set_size(width, height);
        comp.set_quality(quality.value() as f32);

        // Optimización progresiva para mejor compresión

        comp.set_scan_optimization_mode(ScanMode::AllComponentsTogether);
        comp.set_optimize_scans(true);

        // Habilitar modo progresivo
        comp.set_progressive_mode();

        // CORRECCIÓN: La API nueva requiere pasar el Vec directamente
        let mut compressed_data = Vec::new();

        // start_compress ahora toma el writer como argumento
        let mut compressor = comp
            .start_compress(Vec::new())
            .map_err(|e| InfraError::JpegOptimizationFailed(e.to_string()))?;

        // Escribir scanlines
        assert_eq!(rgb_data.len(), width * height * 3, "Invalid RGB data size");

        // Escribir datos por scanlines
        compressor.write_scanlines(rgb_data);

        // Finalizar y obtener datos
        compressed_data = compressor
            .finish()
            .map_err(|e| InfraError::JpegOptimizationFailed(e.to_string()))?;

        Ok(compressed_data)
    }

    /// Optimize from raw image data (using image crate)
    pub fn optimize_from_dynamic_image(
        &self,
        img: &image::DynamicImage,
        quality: Quality,
    ) -> InfraResult<Vec<u8>> {
        let rgb_img = img.to_rgb8();
        let (width, height) = (rgb_img.width() as usize, rgb_img.height() as usize);

        self.optimize(width, height, rgb_img.as_raw(), quality)
    }
}

impl Default for JpegOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_optimizer() {
        let _optimizer = JpegOptimizer::new();
        // Si compila, el test pasa
    }

    // Tests con imágenes reales se harán en integration tests
}
