use crate::domain::value_objects::Quality;
use crate::infrastructure::error::{InfraError, InfraResult};
use oxipng::Options;

/// PNG lossless optimizer using oxipng
pub struct PngOptimizer;

impl PngOptimizer {
    pub fn new() -> Self {
        Self
    }

    /// Optimize PNG image data
    pub fn optimize(&self, input_data: &[u8], quality: Quality) -> InfraResult<Vec<u8>> {
        let options = self.create_options(quality);

        // oxipng optimiza desde memoria
        match oxipng::optimize_from_memory(input_data, &options) {
            Ok(optimized) => Ok(optimized),
            Err(e) => Err(InfraError::PngOptimizationFailed(e.to_string())),
        }
    }

    /// Create oxipng options based on quality
    ///
    /// Maps the quality slider to oxipng optimization levels (0-6).
    /// Higher levels = more processing time but better compression.
    /// PNG is lossless, so "quality" here means compression effort, not visual quality.
    fn create_options(&self, quality: Quality) -> Options {
        // Map quality slider (1-100) to oxipng optimization level (0-6)
        // More aggressive mapping for better compression by default
        let optimization_level = match quality.value() {
            1..=15 => 1,   // Fast (minimal optimization)
            16..=35 => 2,  // Faster (basic optimization)
            36..=55 => 3,  // Normal (good balance) - DEFAULT
            56..=70 => 4,  // Good (better compression)
            71..=85 => 5,  // Better (significant compression)
            86..=100 => 6, // Best (maximum compression, slower)
            _ => 3,        // Default fallback
        };

        // Create options from preset level
        // oxipng presets configure filter strategies, compression, etc.
        let mut opts = Options::from_preset(optimization_level);

        // Strip all non-critical metadata chunks to reduce file size
        // Removes: EXIF, XMP, color profiles, text chunks, timestamps
        // Keeps: IHDR, IDAT, IEND, tRNS (transparency), PLTE (palette), etc.
        opts.strip = oxipng::StripChunks::Safe;

        opts
    }
}

impl Default for PngOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_optimizer() {
        let optimizer = PngOptimizer::new();
        let options = optimizer.create_options(Quality::default());
        // Verificar que las opciones se crean correctamente
        assert_eq!(options.strip, oxipng::StripChunks::Safe);
    }

    #[test]
    fn test_quality_mapping() {
        let optimizer = PngOptimizer::new();

        // Baja calidad = optimización rápida
        let _opts_low = optimizer.create_options(Quality::new(20).unwrap());

        // Alta calidad = optimización máxima
        let _opts_high = optimizer.create_options(Quality::new(100).unwrap());

        // Si compila, el test pasa
    }
}
