use crate::domain::value_objects::{ImageFormat, Quality};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Processing settings for image optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingSettings {
    /// Quality for lossy compression
    quality: Quality,
    /// Output format (None = keep original)
    output_format: Option<ImageFormat>,
    /// Output directory
    output_directory: PathBuf,
    /// Preserve EXIF metadata
    preserve_metadata: bool,
    /// Overwrite existing files
    overwrite_existing: bool,
    /// Number of parallel workers (None = auto)
    max_workers: Option<usize>,
}

impl ProcessingSettings {
    /// Create new processing settings
    pub fn new(quality: Quality, output_directory: PathBuf) -> Self {
        Self {
            quality,
            output_format: None,
            output_directory,
            preserve_metadata: false,
            overwrite_existing: false,
            max_workers: None,
        }
    }

    /// Create with default quality
    pub fn with_directory(output_directory: PathBuf) -> Self {
        Self::new(Quality::default(), output_directory)
    }

    /// Set quality
    pub fn set_quality(&mut self, quality: Quality) -> &mut Self {
        self.quality = quality;
        self
    }

    /// Set output format
    pub fn set_output_format(&mut self, format: Option<ImageFormat>) -> &mut Self {
        self.output_format = format;
        self
    }

    /// Set preserve metadata
    pub fn set_preserve_metadata(&mut self, preserve: bool) -> &mut Self {
        self.preserve_metadata = preserve;
        self
    }

    /// Set overwrite existing
    pub fn set_overwrite_existing(&mut self, overwrite: bool) -> &mut Self {
        self.overwrite_existing = overwrite;
        self
    }

    /// Set max workers
    pub fn set_max_workers(&mut self, workers: Option<usize>) -> &mut Self {
        self.max_workers = workers;
        self
    }

    /// Get quality
    pub fn quality(&self) -> Quality {
        self.quality
    }

    /// Get output format
    pub fn output_format(&self) -> Option<ImageFormat> {
        self.output_format
    }

    /// Get output directory
    pub fn output_directory(&self) -> &PathBuf {
        &self.output_directory
    }

    /// Get preserve metadata
    pub fn preserve_metadata(&self) -> bool {
        self.preserve_metadata
    }

    /// Get overwrite existing
    pub fn overwrite_existing(&self) -> bool {
        self.overwrite_existing
    }

    /// Get max workers
    pub fn max_workers(&self) -> Option<usize> {
        self.max_workers
    }

    /// Determine the output format for a given input format
    pub fn determine_output_format(&self, input_format: ImageFormat) -> ImageFormat {
        self.output_format.unwrap_or(input_format)
    }
}

impl Default for ProcessingSettings {
    fn default() -> Self {
        Self {
            quality: Quality::default(),
            output_format: None,
            output_directory: PathBuf::from("."),
            preserve_metadata: false,
            overwrite_existing: false,
            max_workers: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = ProcessingSettings::default();
        assert_eq!(settings.quality().value(), 85);
        assert!(!settings.preserve_metadata());
    }

    #[test]
    fn test_builder_pattern() {
        let mut settings = ProcessingSettings::with_directory(PathBuf::from("/tmp"));
        settings
            .set_quality(Quality::maximum())
            .set_preserve_metadata(true);

        assert_eq!(settings.quality().value(), 100);
        assert!(settings.preserve_metadata());
    }

    #[test]
    fn test_determine_output_format() {
        let mut settings = ProcessingSettings::default();

        // Sin formato de salida definido, mantiene el original
        assert_eq!(
            settings.determine_output_format(ImageFormat::Png),
            ImageFormat::Png
        );

        // Con formato de salida definido, usa el nuevo
        settings.set_output_format(Some(ImageFormat::Jpeg));
        assert_eq!(
            settings.determine_output_format(ImageFormat::Png),
            ImageFormat::Jpeg
        );
    }
}
