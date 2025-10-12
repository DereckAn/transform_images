use img_parts::jpeg::Jpeg;
use img_parts::png::Png;
use img_parts::webp::WebP;
use img_parts::{Bytes, ImageEXIF};

use crate::domain::ImageFormat;
use crate::infrastructure::error::{InfraError, InfraResult};

/// Metadata cleaner - Elimina EXIF/metadata de imágenes
pub struct MetadataCleaner;

impl MetadataCleaner {
    pub fn new() -> Self {
        Self
    }

    /// Elimina metadatos de una imagen basándose en su formato
    pub fn strip_metadata(&self, data: &[u8], format: ImageFormat) -> InfraResult<Vec<u8>> {
        match format {
            ImageFormat::Jpeg => self.strip_jpeg_metadata(data),
            ImageFormat::Png => self.strip_png_metadata(data),
            ImageFormat::Webp => self.strip_webp_metadata(data),
            ImageFormat::Gif => Ok(data.to_vec()), // GIF raramente tiene EXIF
            ImageFormat::Raw => Ok(data.to_vec()), // RAW ya fue procesado, no tiene EXIF
        }
    }

    /// Elimina metadatos de JPEG
    fn strip_jpeg_metadata(&self, data: &[u8]) -> InfraResult<Vec<u8>> {
        // Convertir &[u8] a Bytes de forma eficiente
        // Bytes::from() usa Vec::from() internamente, evitando copias intermedias
        let mut jpeg = Jpeg::from_bytes(Bytes::from(data.to_vec())).map_err(|e| {
            InfraError::DecodeError(format!(
                "Failed to parse JPEG file ({} bytes): {}",
                data.len(),
                e
            ))
        })?;

        // Eliminar EXIF
        jpeg.set_exif(None);

        // Encodear de vuelta a bytes
        let output_bytes = jpeg.encoder().bytes();
        Ok(output_bytes.to_vec())
    }

    /// Elimina metadatos de PNG
    fn strip_png_metadata(&self, data: &[u8]) -> InfraResult<Vec<u8>> {
        // Convertir &[u8] a Bytes de forma eficiente
        let mut png = Png::from_bytes(Bytes::from(data.to_vec())).map_err(|e| {
            InfraError::DecodeError(format!(
                "Failed to parse PNG file ({} bytes): {}",
                data.len(),
                e
            ))
        })?;

        // Eliminar EXIF
        png.set_exif(None);

        // Encodear de vuelta a bytes
        let output_bytes = png.encoder().bytes();
        Ok(output_bytes.to_vec())
    }

    /// Elimina metadatos de WebP
    fn strip_webp_metadata(&self, data: &[u8]) -> InfraResult<Vec<u8>> {
        // Convertir &[u8] a Bytes de forma eficiente
        let mut webp = WebP::from_bytes(Bytes::from(data.to_vec())).map_err(|e| {
            InfraError::DecodeError(format!(
                "Failed to parse WebP file ({} bytes): {}",
                data.len(),
                e
            ))
        })?;

        // Eliminar EXIF
        webp.set_exif(None);

        // Encodear de vuelta a bytes
        let output_bytes = webp.encoder().bytes();
        Ok(output_bytes.to_vec())
    }
}

impl Default for MetadataCleaner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cleaner() {
        let _cleaner = MetadataCleaner::new();
    }
}
