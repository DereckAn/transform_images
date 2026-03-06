use image::{DynamicImage, RgbImage};
use std::ffi::CString;
use std::io::Cursor;
use std::path::Path;

use crate::domain::RawQualityMode;
use crate::infrastructure::error::{InfraError, InfraResult};

/// Helper: Convert LibRaw error code to human-readable message
fn libraw_error_message(code: i32) -> &'static str {
    match code {
        -1 => "Unspecified error",
        -2 => "Unsupported file format",
        -3 => "Request for nonexistent image number",
        -4 => "Out of order call of LibRaw function",
        -5 => "No thumbnail in file",
        -6 => "Unsupported thumbnail format",
        -7 => "Cannot parse input file",
        -100007 => "Cancelled by user callback",
        -100008 => "Bad crop box",
        _ => "Unknown error",
    }
}

/// RAW image processor using LibRaw directly via FFI
/// Supports ALL cameras including Sony a7C, Canon R5, Nikon Z9,   etc.
pub struct RawProcessor;

impl RawProcessor {
    pub fn new() -> Self {
        Self
    }

    /// Convert RAW file to DynamicImage using LibRaw FFI
    pub fn process_raw(&self, path: &Path, quality_mode: RawQualityMode) -> InfraResult<DynamicImage> {
        if !path.exists() {
            return Err(InfraError::ImageReadError(format!(
                "RAW file not found: {}",
                path.display()
            )));
        }

        let path_str = path
            .to_str()
            .ok_or_else(|| InfraError::ImageReadError("Invalid file path".to_string()))?;
        let c_path = CString::new(path_str)
            .map_err(|e| InfraError::ImageReadError(format!("Invalid path: {}", e)))?;

        if quality_mode == RawQualityMode::Thumbnail {
            return self.extract_thumbnail(path, &c_path);
        }

        unsafe {
            let data = libraw_sys::libraw_init(0);
            if data.is_null() {
                return Err(InfraError::DecodeError(
                    "Failed to initialize LibRaw".to_string(),
                ));
            }

            let _guard = LibRawGuard(data);

            // Always-on performance params
            libraw_sys::libraw_set_no_auto_bright(data, 1);
            libraw_sys::libraw_set_highlight(data, 0);
            libraw_sys::libraw_set_fbdd_noiserd(data, 0);
            libraw_sys::libraw_set_output_color(data, 1);
            libraw_sys::libraw_set_output_bps(data, 8);
            (*data).params.use_camera_wb = 1;

            match quality_mode {
                RawQualityMode::Thumbnail => unreachable!(),
                RawQualityMode::Fast => {
                    (*data).params.half_size = 1;
                    libraw_sys::libraw_set_demosaic(data, 0);
                }
                RawQualityMode::Balanced => {
                    (*data).params.half_size = 0;
                    libraw_sys::libraw_set_demosaic(data, 2);
                }
                RawQualityMode::Quality => {
                    (*data).params.half_size = 0;
                    libraw_sys::libraw_set_demosaic(data, 3);
                }
            }

            let ret = libraw_sys::libraw_open_file(data, c_path.as_ptr());
            if ret != 0 {
                return Err(InfraError::ImageReadError(format!(
                    "Failed to open RAW file '{}': {} (error {})",
                    path.display(), libraw_error_message(ret), ret
                )));
            }

            let ret = libraw_sys::libraw_unpack(data);
            if ret != 0 {
                return Err(InfraError::DecodeError(format!(
                    "Failed to unpack RAW data from '{}': {} (error {})",
                    path.display(), libraw_error_message(ret), ret
                )));
            }

            let ret = libraw_sys::libraw_dcraw_process(data);
            if ret != 0 {
                return Err(InfraError::DecodeError(format!(
                    "Failed to process RAW data from '{}': {} (error {})",
                    path.display(), libraw_error_message(ret), ret
                )));
            }

            let mut err_code: i32 = 0;
            let processed = libraw_sys::libraw_dcraw_make_mem_image(data, &mut err_code);
            if processed.is_null() {
                return Err(InfraError::DecodeError(format!(
                    "Failed to create image from RAW file '{}': {} (error {})",
                    path.display(), libraw_error_message(err_code), err_code
                )));
            }

            let _processed_guard = ProcessedImageGuard(processed);
            self.convert_libraw_to_dynamic_image(processed)
        }
    }

    /// Extract embedded JPEG thumbnail from RAW file — no demosaicing, ~100x faster.
    /// Falls back to Balanced demosaicing if no usable thumbnail is found.
    fn extract_thumbnail(&self, path: &Path, c_path: &CString) -> InfraResult<DynamicImage> {
        unsafe {
            let data = libraw_sys::libraw_init(0);
            if data.is_null() {
                return Err(InfraError::DecodeError(
                    "Failed to initialize LibRaw".to_string(),
                ));
            }

            let _guard = LibRawGuard(data);

            let ret = libraw_sys::libraw_open_file(data, c_path.as_ptr());
            if ret != 0 {
                return Err(InfraError::ImageReadError(format!(
                    "Failed to open RAW file '{}': {} (error {})",
                    path.display(), libraw_error_message(ret), ret
                )));
            }

            // Unpack only the thumbnail — skips all sensor data decoding
            let ret = libraw_sys::libraw_unpack_thumb(data);
            if ret != 0 {
                // No thumbnail in this file — fall back to Balanced demosaicing
                drop(_guard);
                return self.process_raw(path, RawQualityMode::Balanced);
            }

            let mut err_code: i32 = 0;
            let thumb = libraw_sys::libraw_dcraw_make_mem_thumb(data, &mut err_code);
            if thumb.is_null() {
                drop(_guard);
                return self.process_raw(path, RawQualityMode::Balanced);
            }

            let _thumb_guard = ProcessedImageGuard(thumb);
            let img = &*thumb;

            match img.image_type {
                libraw_sys::LibRaw_image_formats::LIBRAW_IMAGE_JPEG => {
                    // Thumbnail is a raw JPEG blob — decode it directly with the image crate
                    let data_size = img.data_size as usize;
                    let jpeg_bytes = std::slice::from_raw_parts(img.data.as_ptr(), data_size);
                    image::load_from_memory_with_format(jpeg_bytes, image::ImageFormat::Jpeg)
                        .map_err(|e| InfraError::DecodeError(format!(
                            "Failed to decode embedded JPEG thumbnail from '{}': {}",
                            path.display(), e
                        )))
                }
                libraw_sys::LibRaw_image_formats::LIBRAW_IMAGE_BITMAP => {
                    // Thumbnail is already a decoded RGB bitmap
                    self.convert_libraw_to_dynamic_image(thumb)
                }
            }
        }
    }

    /// Fast metadata extraction from RAW files WITHOUT decoding pixels
    /// This is used during image selection to show file info quickly
    pub fn get_raw_metadata(path: &Path) -> InfraResult<(u32, u32)> {
        use std::os::raw::c_char;

        // Convert path to C string for FFI
        let path_str = path
            .to_str()
            .ok_or_else(|| InfraError::ImageReadError("Invalid path".to_string()))?;
        let c_path = CString::new(path_str)
            .map_err(|e| InfraError::ImageReadError(format!("Path conversion failed: {}", e)))?;

        unsafe {
            // Initialize LibRaw handle
            let raw = libraw_sys::libraw_init(0);
            if raw.is_null() {
                return Err(InfraError::DecodeError(
                    "Failed to initialize LibRaw".to_string(),
                ));
            }

            // Open file but DON'T unpack pixel data
            let ret = libraw_sys::libraw_open_file(raw, c_path.as_ptr() as *const c_char);
            if ret != 0 {
                let error_msg = libraw_error_message(ret);
                libraw_sys::libraw_close(raw);
                return Err(InfraError::ImageReadError(format!(
                    "Failed to open RAW file: {}",
                    error_msg
                )));
            }

            // Read metadata from imgdata struct
            let imgdata = &*raw;
            let width = imgdata.sizes.width as u32;
            let height = imgdata.sizes.height as u32;

            // Clean up
            libraw_sys::libraw_close(raw);

            Ok((width, height))
        }
    }

    /// Convertir libraw_processed_image_t a DynamicImage
    unsafe fn convert_libraw_to_dynamic_image(
        &self,
        processed: *mut libraw_sys::libraw_processed_image_t,
    ) -> InfraResult<DynamicImage> {
        let img = &*processed;

        let width = img.width as u32;
        let height = img.height as u32;
        let colors = img.colors as usize;

        // Verificar que es RGB (3 canales)
        if colors != 3 {
            return Err(InfraError::DecodeError(format!(
                "Unsupported color format: {} channels (expected 3)",
                colors
            )));
        }

        // Convertir datos de LibRaw a Vec
        // Nota: Debemos copiar porque LibRaw posee la memoria original y será liberada
        let data_size = (width * height * 3) as usize;
        let data_slice = std::slice::from_raw_parts(img.data.as_ptr(), data_size);

        // Vec::from() es más eficiente que to_vec() para slices grandes
        let pixel_data = Vec::from(data_slice);

        // Crear RgbImage desde los datos
        let rgb_image = RgbImage::from_raw(width, height, pixel_data).ok_or_else(|| {
            InfraError::DecodeError("Failed to create RGB image from RAW data".to_string())
        })?;

        // Convertir a DynamicImage
        Ok(DynamicImage::ImageRgb8(rgb_image))
    }

    /// Check if file extension is a known RAW format
    pub fn is_raw_format(extension: &str) -> bool {
        matches!(
            extension.to_lowercase().as_str(),
            "arw"  // Sony
              | "cr2" | "cr3"  // Canon
              | "nef" | "nrw"  // Nikon
              | "dng"  // Adobe Digital Negative
              | "raf"  // Fujifilm
              | "orf"  // Olympus
              | "rw2"  // Panasonic
              | "pef"  // Pentax
              | "srw"  // Samsung
              | "x3f"  // Sigma
              | "raw"  // Generic
              | "rwl"  // Leica
              | "mrw"  // Minolta
              | "erf"  // Epson
              | "3fr"  // Hasselblad
              | "ari"  // ARRI
              | "srf"  // Sony
              | "sr2"  // Sony
              | "bay"  // Casio
              | "crw"  // Canon (old)
              | "iiq"  // Phase One
              | "k25" | "kdc"  // Kodak
              | "mef"  // Mamiya
              | "mos"  // Leaf
              | "r3d" // RED
        )
    }
}

// RAII guard para libraw_data_t - limpia automáticamente cuando   se destruye
struct LibRawGuard(*mut libraw_sys::libraw_data_t);

impl Drop for LibRawGuard {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                libraw_sys::libraw_close(self.0);
            }
        }
    }
}

// RAII guard para libraw_processed_image_t - limpia   automáticamente
struct ProcessedImageGuard(*mut libraw_sys::libraw_processed_image_t);

impl Drop for ProcessedImageGuard {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                libraw_sys::libraw_dcraw_clear_mem(self.0);
            }
        }
    }
}

impl Default for RawProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_raw_format() {
        assert!(RawProcessor::is_raw_format("arw"));
        assert!(RawProcessor::is_raw_format("ARW"));
        assert!(RawProcessor::is_raw_format("cr2"));
        assert!(RawProcessor::is_raw_format("nef"));
        assert!(RawProcessor::is_raw_format("dng"));
        assert!(!RawProcessor::is_raw_format("jpg"));
        assert!(!RawProcessor::is_raw_format("png"));
    }

    #[test]
    fn test_create_processor() {
        let _processor = RawProcessor::new();
    }
}
