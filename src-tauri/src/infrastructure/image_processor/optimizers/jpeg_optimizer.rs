use crate::domain::value_objects::Quality;
use crate::infrastructure::error::{InfraError, InfraResult};
use mozjpeg::{ColorSpace, Compress, ScanMode};

/// JPEG optimizer using mozjpeg
pub struct JpegOptimizer;

impl JpegOptimizer {
    pub fn new() -> Self {
        Self
    }

    /// Optimize JPEG image from raw RGB data
    ///
    /// Creates a fresh JPEG file from pixel data only - no metadata is copied.
    /// This ensures the output is clean and optimized without EXIF/XMP/IPTC.
    pub fn optimize(
        &self,
        width: usize,
        height: usize,
        rgb_data: &[u8],
        quality: Quality,
    ) -> InfraResult<Vec<u8>> {
        // Create mozjpeg compressor from raw RGB pixels (no metadata)
        let mut comp = Compress::new(ColorSpace::JCS_RGB);

        comp.set_size(width, height);

        // Map quality slider to actual JPEG quality for better compression
        // This provides more aggressive compression while maintaining visual quality
        let jpeg_quality = self.map_quality_to_jpeg(quality);
        comp.set_quality(jpeg_quality);

        // Enable progressive encoding for better compression and progressive loading
        comp.set_progressive_mode();

        // Optimize scans for better compression
        comp.set_scan_optimization_mode(ScanMode::AllComponentsTogether);
        comp.set_optimize_scans(true);

        // start_compress ahora toma el writer como argumento
        let mut compressor = comp
            .start_compress(Vec::new())
            .map_err(|e| InfraError::JpegOptimizationFailed(e.to_string()))?;

        // Escribir scanlines
        assert_eq!(rgb_data.len(), width * height * 3, "Invalid RGB data size");

        // Escribir datos por scanlines
        compressor
            .write_scanlines(rgb_data)
            .map_err(|e| InfraError::JpegOptimizationFailed(e.to_string()))?;

        // Finalizar y obtener datos
        let compressed_data = compressor
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

    /// Map quality slider (1-100) to actual JPEG quality for optimal compression
    ///
    /// This mapping provides more aggressive compression than direct 1:1 mapping
    /// while maintaining excellent visual quality. JPEG quality 60-85 is the
    /// sweet spot for most photos - visually lossless with significant file size reduction.
    fn map_quality_to_jpeg(&self, quality: Quality) -> f32 {
        match quality.value() {
            1..=10 => 40.0,   // Very aggressive compression (small files, visible artifacts)
            11..=30 => 50.0,  // Aggressive compression (good for web thumbnails)
            31..=50 => 60.0,  // Moderate compression (good balance)
            51..=70 => 70.0,  // Good quality (recommended for most photos)
            71..=85 => 80.0,  // High quality (excellent for professional photos)
            86..=95 => 90.0,  // Very high quality (minimal compression)
            96..=100 => 95.0, // Maximum quality (near-lossless)
            _ => 85.0,        // Default fallback
        }
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
