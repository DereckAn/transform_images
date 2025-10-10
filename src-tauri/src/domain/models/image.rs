use crate::domain::error::{DomainError, DomainResult};
use crate::domain::value_objects::{Dimensions, ImageFormat};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Represents an image file with its metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    /// Absolute path to the image file
    path: PathBuf,
    /// Image format
    format: ImageFormat,
    /// Image dimensions
    dimensions: Dimensions,
    /// File size in bytes
    size_bytes: u64,
    /// Optional EXIF metadata
    metadata: Option<ImageMetadata>,
}

impl Image {
    /// Create a new Image
    pub fn new(
        path: PathBuf,
        format: ImageFormat,
        dimensions: Dimensions,
        size_bytes: u64,
        metadata: Option<ImageMetadata>,
    ) -> DomainResult<Self> {
        if !path.is_absolute() {
            return Err(DomainError::InvalidFilePath(
                path.to_string_lossy().to_string(),
            ));
        }

        Ok(Image {
            path,
            format,
            dimensions,
            size_bytes,
            metadata,
        })
    }

    /// Create Image from file path (without loading the actual image data)
    pub fn from_path(path: impl AsRef<Path>) -> DomainResult<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(DomainError::FileNotFound(
                path.to_string_lossy().to_string(),
            ));
        }

        let path_buf = path.to_path_buf();
        let format = Self::detect_format(&path_buf)?;

        // Note: dimensions y size_bytes se cargarán cuando se lea   el archivo
        // Por ahora creamos con valores temporales
        Ok(Image {
            path: path_buf,
            format,
            dimensions: Dimensions::new(1, 1)?, // Temporal
            size_bytes: 0,                      // Temporal
            metadata: None,
        })
    }

    /// Detect image format from file extension
    fn detect_format(path: &Path) -> DomainResult<ImageFormat> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| DomainError::InvalidImageFormat("No file extension".to_string()))
            .and_then(ImageFormat::from_extension)
    }

    /// Get the image path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get the image format
    pub fn format(&self) -> ImageFormat {
        self.format
    }

    /// Get the image dimensions
    pub fn dimensions(&self) -> &Dimensions {
        &self.dimensions
    }

    /// Get the file size in bytes
    pub fn size_bytes(&self) -> u64 {
        self.size_bytes
    }

    /// Get the file size in megabytes
    pub fn size_mb(&self) -> f64 {
        self.size_bytes as f64 / (1024.0 * 1024.0)
    }

    /// Get metadata if available
    pub fn metadata(&self) -> Option<&ImageMetadata> {
        self.metadata.as_ref()
    }

    /// Check if image has metadata
    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    /// Get file name without extension
    pub fn file_stem(&self) -> Option<&str> {
        self.path.file_stem().and_then(|s| s.to_str())
    }

    /// Get file name with extension
    pub fn file_name(&self) -> Option<&str> {
        self.path.file_name().and_then(|s| s.to_str())
    }

    /// Get parent directory
    pub fn parent_dir(&self) -> Option<&Path> {
        self.path.parent()
    }

    /// Update dimensions (usado cuando se carga la imagen real)
    pub fn set_dimensions(&mut self, dimensions: Dimensions) {
        self.dimensions = dimensions;
    }

    /// Update size (usado cuando se carga la imagen real)
    pub fn set_size_bytes(&mut self, size: u64) {
        self.size_bytes = size;
    }

    /// Update metadata
    pub fn set_metadata(&mut self, metadata: Option<ImageMetadata>) {
        self.metadata = metadata;
    }
}

/// EXIF metadata from image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    /// Camera make
    pub camera_make: Option<String>,
    /// Camera model
    pub camera_model: Option<String>,
    /// Date/time when image was taken
    pub date_time: Option<String>,
    /// ISO speed
    pub iso_speed: Option<u32>,
    /// Exposure time
    pub exposure_time: Option<String>,
    /// F-number (aperture)
    pub f_number: Option<f64>,
    /// Focal length
    pub focal_length: Option<f64>,
    /// GPS coordinates (latitude, longitude)
    pub gps_coordinates: Option<(f64, f64)>,
    /// Orientation
    pub orientation: Option<u32>,
}

impl ImageMetadata {
    /// Create empty metadata
    pub fn empty() -> Self {
        ImageMetadata {
            camera_make: None,
            camera_model: None,
            date_time: None,
            iso_speed: None,
            exposure_time: None,
            f_number: None,
            focal_length: None,
            gps_coordinates: None,
            orientation: None,
        }
    }

    /// Check if metadata is empty
    pub fn is_empty(&self) -> bool {
        self.camera_make.is_none()
            && self.camera_model.is_none()
            && self.date_time.is_none()
            && self.iso_speed.is_none()
            && self.exposure_time.is_none()
            && self.f_number.is_none()
            && self.focal_length.is_none()
            && self.gps_coordinates.is_none()
            && self.orientation.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_image() {
        let path = PathBuf::from("/tmp/test.png")
            .canonicalize()
            .unwrap_or(PathBuf::from("/tmp/test.png"));
        let format = ImageFormat::Png;
        let dimensions = Dimensions::new(1920, 1080).unwrap();
        let size = 1024 * 1024; // 1MB

        let image = Image::new(path, format, dimensions, size, None);
        assert!(image.is_ok());
    }

    #[test]
    fn test_size_mb() {
        let path = PathBuf::from("/tmp/test.png")
            .canonicalize()
            .unwrap_or(PathBuf::from("/tmp/test.png"));
        let dimensions = Dimensions::new(100, 100).unwrap();
        let image = Image::new(
            path,
            ImageFormat::Png,
            dimensions,
            1024 * 1024, // 1MB
            None,
        )
        .unwrap();

        assert!((image.size_mb() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_metadata_empty() {
        let meta = ImageMetadata::empty();
        assert!(meta.is_empty());
    }
}
