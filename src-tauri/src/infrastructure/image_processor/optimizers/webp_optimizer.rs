use crate::domain::value_objects::Quality;
use crate::infrastructure::error::InfraResult;
use image::DynamicImage;
use webp::Encoder;

/// WebP optimizer backed by libwebp via the `webp` crate.
pub struct WebpOptimizer;

impl WebpOptimizer {
    pub fn new() -> Self {
        Self
    }

    /// Encode the incoming image as WebP using lossy or lossless mode according to the requested quality.
    pub fn optimize(&self, image: &DynamicImage, quality: Quality) -> InfraResult<Vec<u8>> {
        // Convert to RGBA because the encoder expects packed RGB(A) buffers.
        let rgba = image.to_rgba8();
        let encoder = Encoder::from_rgba(rgba.as_raw(), rgba.width(), rgba.height());

        // Use near-lossless for very high quality targets, otherwise standard lossy encoding.
        let encoded = if quality.value() >= 98 {
            encoder.encode_lossless()
        } else {
            encoder.encode(self.map_quality(quality))
        };

        Ok(encoded.to_vec())
    }

    fn map_quality(&self, quality: Quality) -> f32 {
        match quality.value() {
            0..=10 => 40.0,
            11..=30 => 50.0,
            31..=50 => 60.0,
            51..=70 => 70.0,
            71..=85 => 80.0,
            86..=95 => 90.0,
            _ => 95.0,
        }
    }
}

impl Default for WebpOptimizer {
    fn default() -> Self {
        Self::new()
    }
}
