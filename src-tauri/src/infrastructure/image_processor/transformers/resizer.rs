use crate::domain::models::{ResizeFilter, ResizeTransformation};
use crate::infrastructure::error::InfraResult;
use image::{imageops::FilterType, DynamicImage};

/// Image resizer
pub struct Resizer;

impl Resizer {
    pub fn new() -> Self {
        Self
    }

    /// Resize an image based on transformation
    pub fn resize(
        &self,
        img: &DynamicImage,
        transformation: &ResizeTransformation,
        original_dimensions: &crate::domain::value_objects::Dimensions,
    ) -> InfraResult<DynamicImage> {
        let final_dims = transformation.calculate_final_dimensions(original_dimensions)?;
        let filter = Self::convert_filter(transformation.filter());

        let resized = if transformation.preserve_aspect_ratio() {
            img.resize(final_dims.width(), final_dims.height(), filter)
        } else {
            img.resize_exact(final_dims.width(), final_dims.height(), filter)
        };

        Ok(resized)
    }

    /// Convert domain ResizeFilter to image crate FilterType
    fn convert_filter(filter: ResizeFilter) -> FilterType {
        match filter {
            ResizeFilter::Nearest => FilterType::Nearest,
            ResizeFilter::Triangle => FilterType::Triangle,
            ResizeFilter::CatmullRom => FilterType::CatmullRom,
            ResizeFilter::Gaussian => FilterType::Gaussian,
            ResizeFilter::Lanczos3 => FilterType::Lanczos3,
        }
    }
}

impl Default for Resizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::Dimensions;

    #[test]
    fn test_convert_filter() {
        assert_eq!(
            Resizer::convert_filter(ResizeFilter::Lanczos3),
            FilterType::Lanczos3
        );
    }

    // Tests con imágenes reales en integration tests
}
