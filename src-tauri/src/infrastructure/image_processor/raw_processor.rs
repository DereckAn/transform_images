use image::{DynamicImage, RgbImage};
use std::ffi::CString;
use std::path::Path;

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
    pub fn process_raw(&self, path: &Path) -> InfraResult<DynamicImage> {
        // Verificar que el archivo existe
        if !path.exists() {
            return Err(InfraError::ImageReadError(format!(
                "RAW file not found: {}",
                path.display()
            )));
        }

        // Convertir path a CString (para FFI C)
        let path_str = path.to_str().ok_or_else(|| {
            InfraError::ImageReadError(
                "Invalid 
  file path"
                    .to_string(),
            )
        })?;
        let c_path = CString::new(path_str)
            .map_err(|e| InfraError::ImageReadError(format!("Invalid path: {}", e)))?;

        unsafe {
            // Paso 1: Crear procesador LibRaw
            let data = libraw_sys::libraw_init(0);
            if data.is_null() {
                return Err(InfraError::DecodeError(
                    "Failed to 
  initialize LibRaw"
                        .to_string(),
                ));
            }

            // Guard garantiza limpieza automática si hay error
            let _guard = LibRawGuard(data);

            // Paso 2: Abrir archivo RAW
            let ret = libraw_sys::libraw_open_file(data, c_path.as_ptr());
            if ret != 0 {
                return Err(InfraError::ImageReadError(format!(
                    "Failed to open RAW file '{}': {} (error {})",
                    path.display(),
                    libraw_error_message(ret),
                    ret
                )));
            }

            // Paso 3: Desempaquetar datos RAW del sensor
            let ret = libraw_sys::libraw_unpack(data);
            if ret != 0 {
                return Err(InfraError::DecodeError(format!(
                    "Failed to unpack RAW data from '{}': {} (error {})",
                    path.display(),
                    libraw_error_message(ret),
                    ret
                )));
            }

            // Paso 4: Procesar RAW → RGB (demosaicing, balance blanco, corrección color)
            let ret = libraw_sys::libraw_dcraw_process(data);
            if ret != 0 {
                return Err(InfraError::DecodeError(format!(
                    "Failed to process RAW data from '{}': {} (error {})",
                    path.display(),
                    libraw_error_message(ret),
                    ret
                )));
            }

            // Paso 5: Obtener imagen procesada en memoria
            let mut err_code: i32 = 0;
            let processed = libraw_sys::libraw_dcraw_make_mem_image(data, &mut err_code);
            if processed.is_null() {
                return Err(InfraError::DecodeError(format!(
                    "Failed to create image from RAW file '{}': {} (error {})",
                    path.display(),
                    libraw_error_message(err_code),
                    err_code
                )));
            }

            // Guard garantiza limpieza de imagen procesada
            let _processed_guard = ProcessedImageGuard(processed);

            // Paso 6: Convertir de LibRaw a DynamicImage
            self.convert_libraw_to_dynamic_image(processed)
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
                "Unsupported color format: {} channels (expected 
  3)",
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
