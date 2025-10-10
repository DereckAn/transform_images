use crate::domain::error::DomainResult;
use crate::domain::models::{Image, ProcessingSettings, Transformation};
use std::path::Path;

/// Trait for image processing operations
/// This is the main abstraction for the infrastructure layer
pub trait ImageProcessor: Send + Sync {
    /// Load an image from disk
    fn load_image(&self, path: &Path) -> DomainResult<Image>;

    /// Optimize an image with given settings
    fn optimize(&self, image: &Image, settings: &ProcessingSettings) -> DomainResult<Vec<u8>>;

    /// Apply transformations to an image
    fn transform(&self, image: &Image, transformation: &Transformation) -> DomainResult<Vec<u8>>;

    /// Optimize and transform in one operation
    fn process(
        &self,
        image: &Image,
        transformation: Option<&Transformation>,
        settings: &ProcessingSettings,
    ) -> DomainResult<Vec<u8>>;

    /// Save processed image to disk
    fn save_image(
        &self,
        data: &[u8],
        output_path: &Path,
        format: crate::domain::value_objects::ImageFormat,
    ) -> DomainResult<()>;
}
