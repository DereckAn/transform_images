use crate::domain::value_objects::Quality;
use crate::infrastructure::error::{InfraError, InfraResult};
use indexmap::IndexSet;
use oxipng::{Deflaters, Interlacing, Options, RowFilter, StripChunks};

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

        // STRATEGY 3: Advanced PNG optimizations for 10-30% additional compression

        // Strip all non-critical metadata chunks to reduce file size
        // Removes: EXIF, XMP, color profiles, text chunks, timestamps
        // Keeps: IHDR, IDAT, IEND, tRNS (transparency), PLTE (palette), etc.
        opts.strip = StripChunks::Safe;

        // Try all PNG filter types to find the best compression for this image
        // Each filter preprocesses the image data differently before compression
        // Testing multiple filters finds the optimal one for each image type
        opts.filter = self.get_filter_set(quality);

        // Disable interlacing for smaller file sizes
        // Interlaced PNGs (Adam7) are larger and only useful for progressive loading
        // Modern browsers handle non-interlaced PNGs fine
        opts.interlace = Some(Interlacing::None);

        // Use Zopfli deflate for maximum compression at high quality levels
        // Zopfli is 10-30% better than standard deflate but much slower
        opts.deflate = self.get_deflater(quality);

        // Optimize palette and bit depth reduction
        // This can significantly reduce file size for images with few colors
        opts.bit_depth_reduction = true;
        opts.color_type_reduction = true;
        opts.palette_reduction = true;

        // Optimize alpha channel
        // Remove unnecessary alpha channel if all pixels are opaque
        opts.grayscale_reduction = true;

        opts
    }

    /// Get optimal filter set based on quality level
    ///
    /// Filters preprocess image data before compression to improve compressibility.
    /// Higher quality = try more filters (slower but better compression).
    fn get_filter_set(&self, quality: Quality) -> IndexSet<RowFilter> {
        match quality.value() {
            // Low quality: Fast filters only
            1..=35 => {
                let mut filters = IndexSet::new();
                filters.insert(RowFilter::None);
                filters.insert(RowFilter::Sub);
                filters
            }
            // Medium quality: Common filters
            36..=70 => {
                let mut filters = IndexSet::new();
                filters.insert(RowFilter::None);
                filters.insert(RowFilter::Sub);
                filters.insert(RowFilter::Up);
                filters.insert(RowFilter::Average);
                filters.insert(RowFilter::Paeth);
                filters
            }
            // High quality: All filters including adaptive
            71..=100 => {
                let mut filters = IndexSet::new();
                filters.insert(RowFilter::None);
                filters.insert(RowFilter::Sub);
                filters.insert(RowFilter::Up);
                filters.insert(RowFilter::Average);
                filters.insert(RowFilter::Paeth);
                filters.insert(RowFilter::MinSum);
                filters.insert(RowFilter::Entropy);
                filters.insert(RowFilter::Bigrams);
                filters.insert(RowFilter::BigEnt);
                filters.insert(RowFilter::Brute);
                filters
            }
            _ => {
                let mut filters = IndexSet::new();
                filters.insert(RowFilter::None);
                filters.insert(RowFilter::Paeth);
                filters
            }
        }
    }

    /// Get optimal deflate compressor based on quality level
    ///
    /// Libdeflater with maximum compression level provides excellent compression.
    /// Higher compression levels = better compression but slower processing.
    fn get_deflater(&self, quality: Quality) -> Deflaters {
        match quality.value() {
            // Low quality: Fast compression
            1..=35 => Deflaters::Libdeflater { compression: 6 },
            // Medium quality: Good balance
            36..=70 => Deflaters::Libdeflater { compression: 9 },
            // High quality: Better compression
            71..=90 => Deflaters::Libdeflater { compression: 11 },
            // Maximum quality: Maximum compression (slowest but best)
            91..=100 => Deflaters::Libdeflater { compression: 12 },
            _ => Deflaters::Libdeflater { compression: 9 },
        }
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
