use crate::domain::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Supported image formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Webp,
    Gif,
    Raw, // RAW formats (ARW, CR2, NEF, DNG, etc.) - read-only, convert to output format
         // Formatos futuros (Fase post-MVP)
         // Tiff,
         // Heic,
         // Ico,
}

impl ImageFormat {
    /// Get file extension for this format
    pub fn extension(&self) -> &str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpg",
            ImageFormat::Webp => "webp",
            ImageFormat::Gif => "gif",
            ImageFormat::Raw => "jpg", // RAW se convierte a JPG por defecto
        }
    }

    /// Get MIME type for this format
    pub fn mime_type(&self) -> &str {
        match self {
            ImageFormat::Png => "image/png",
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::Webp => "image/webp",
            ImageFormat::Gif => "image/gif",
            ImageFormat::Raw => "image/x-raw", // MIME genérico para RAW
        }
    }

    /// Check if format supports transparency
    pub fn supports_transparency(&self) -> bool {
        matches!(
            self,
            ImageFormat::Png | ImageFormat::Webp | ImageFormat::Gif
        )
    }

    /// Check if format supports lossy compression
    pub fn supports_lossy(&self) -> bool {
        matches!(self, ImageFormat::Jpeg | ImageFormat::Webp)
    }

    /// Check if format is a RAW format
    pub fn is_raw(&self) -> bool {
        matches!(self, ImageFormat::Raw)
    }

    /// Parse from file extension
    pub fn from_extension(ext: &str) -> DomainResult<Self> {
        match ext.to_lowercase().as_str() {
            "png" => Ok(ImageFormat::Png),
            "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
            "webp" => Ok(ImageFormat::Webp),
            "gif" => Ok(ImageFormat::Gif),
            // RAW formats
            "arw" | "cr2" | "cr3" | "nef" | "nrw" | "dng" | "raf" | "orf" | "rw2" | "pef"
            | "srw" | "x3f" | "raw" | "rwl" | "mrw" | "erf" | "3fr" | "ari" | "srf" | "sr2"
            | "bay" | "crw" | "iiq" | "k25" | "kdc" | "mef" | "mos" | "r3d" => Ok(ImageFormat::Raw),
            _ => Err(DomainError::InvalidImageFormat(ext.to_string())),
        }
    }
}

impl FromStr for ImageFormat {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_extension(s)
    }
}

impl fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpg",
            ImageFormat::Webp => "webp",
            ImageFormat::Gif => "gif",
            ImageFormat::Raw => "raw", // identifier, not output extension
        };
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_extension() {
        assert_eq!(
            ImageFormat::from_extension("png").unwrap(),
            ImageFormat::Png
        );
        assert_eq!(
            ImageFormat::from_extension("jpg").unwrap(),
            ImageFormat::Jpeg
        );
        assert_eq!(
            ImageFormat::from_extension("jpeg").unwrap(),
            ImageFormat::Jpeg
        );
        assert_eq!(
            ImageFormat::from_extension("PNG").unwrap(),
            ImageFormat::Png
        );
    }

    #[test]
    fn test_invalid_extension() {
        assert!(ImageFormat::from_extension("txt").is_err());
        assert!(ImageFormat::from_extension("pdf").is_err());
    }

    #[test]
    fn test_transparency_support() {
        assert!(ImageFormat::Png.supports_transparency());
        assert!(!ImageFormat::Jpeg.supports_transparency());
    }

    #[test]
    fn test_lossy_support() {
        assert!(ImageFormat::Jpeg.supports_lossy());
        assert!(!ImageFormat::Png.supports_lossy());
    }

    #[test]
    fn test_raw_format_identifier_is_raw_not_jpg() {
        // ImageDto uses format.to_string() to send the format name to the frontend.
        // RAW files must serialize as "raw" so the frontend can distinguish them
        // from real JPEGs and show the RAW quality selector.
        // Regression: previously Raw.to_string() returned "jpg" (the output extension)
        // which made ARW files look identical to JPEG on the frontend.
        assert_eq!(
            ImageFormat::Raw.to_string(),
            "raw",
            "Raw format must serialize as 'raw', not as its output extension 'jpg'"
        );

        // Sanity check: other formats still serialize as their extensions
        assert_eq!(ImageFormat::Jpeg.to_string(), "jpg");
        assert_eq!(ImageFormat::Png.to_string(), "png");
        assert_eq!(ImageFormat::Webp.to_string(), "webp");
    }
}
