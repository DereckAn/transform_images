use image::{DynamicImage, ImageFormat as ImageCrateFormat};
use std::fs;
use std::io::Cursor;
use std::path::Path;

use crate::domain::{
    Dimensions, DomainError, DomainResult, Image, ImageFormat, ImageProcessor, ProcessingSettings,
    Transformation,
};
use crate::infrastructure::error::{InfraError, InfraResult};
use crate::infrastructure::image_processor::optimizers::{
    JpegOptimizer, PngOptimizer, WebpOptimizer,
};
use crate::infrastructure::image_processor::transformers::{Resizer, Rotator};
use crate::infrastructure::image_processor::RawProcessor;

/// Main image processor implementation
pub struct ImageProcessorImpl {
    png_optimizer: PngOptimizer,
    jpeg_optimizer: JpegOptimizer,
    webp_optimizer: WebpOptimizer,
    resizer: Resizer,
    rotator: Rotator,
    raw_processor: RawProcessor,
}

impl ImageProcessorImpl {
    pub fn new() -> Self {
        Self {
            png_optimizer: PngOptimizer::new(),
            jpeg_optimizer: JpegOptimizer::new(),
            webp_optimizer: WebpOptimizer::new(),
            resizer: Resizer::new(),
            rotator: Rotator::new(),
            raw_processor: RawProcessor::new(),
        }
    }

    /// Load DynamicImage from file
    fn load_dynamic_image(&self, path: &Path) -> InfraResult<DynamicImage> {
        // Check if it's a RAW file
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_string();
            if RawProcessor::is_raw_format(&ext_str) {
                // Use RAW processor
                return self.raw_processor.process_raw(path);
            }
        }

        // Use standard image decoder for other formats
        image::open(path).map_err(|e| {
            InfraError::ImageReadError(format!(
                "Failed to open image file '{}': {}",
                path.display(),
                e
            ))
        })
    }

    /// Convert domain ImageFormat to image crate format
    fn convert_format(format: ImageFormat) -> ImageCrateFormat {
        match format {
            ImageFormat::Png => ImageCrateFormat::Png,
            ImageFormat::Jpeg => ImageCrateFormat::Jpeg,
            ImageFormat::Webp => ImageCrateFormat::WebP,
            ImageFormat::Gif => ImageCrateFormat::Gif,
            ImageFormat::Raw => ImageCrateFormat::Jpeg, // RAW se convierte a JPEG por defecto
        }
    }

    /// Encode image to bytes
    fn encode_image(
        &self,
        img: &DynamicImage,
        format: ImageFormat,
        settings: &ProcessingSettings,
    ) -> InfraResult<Vec<u8>> {
        let output = match format {
            ImageFormat::Png => {
                let mut bytes = Vec::new();
                let mut cursor = Cursor::new(&mut bytes);
                img.write_to(&mut cursor, ImageCrateFormat::Png)
                    .map_err(|e| {
                        InfraError::EncodeError(format!(
                            "Failed to encode PNG ({}x{}): {}",
                            img.width(),
                            img.height(),
                            e
                        ))
                    })?;
                // oxipng optimization with built-in metadata stripping
                self.png_optimizer.optimize(&bytes, settings.quality())?
            }
            ImageFormat::Jpeg | ImageFormat::Raw => {
                // mozjpeg creates fresh JPEG from RGB data (no EXIF copied)
                self.jpeg_optimizer
                    .optimize_from_dynamic_image(img, settings.quality())?
            }
            ImageFormat::Webp => {
                // WebP encoder creates fresh file from pixel data (no EXIF)
                self.webp_optimizer.optimize(img, settings.quality())?
            }
            ImageFormat::Gif => {
                let mut bytes = Vec::new();
                let mut cursor = Cursor::new(&mut bytes);
                img.write_to(&mut cursor, Self::convert_format(format))
                    .map_err(|e| {
                        InfraError::EncodeError(format!(
                            "Failed to encode {:?} ({}x{}): {}",
                            format,
                            img.width(),
                            img.height(),
                            e
                        ))
                    })?;
                bytes
            }
        };

        // NOTE: Metadata stripping is now handled by the optimizers themselves.
        // - PNG: oxipng strips metadata via StripChunks::Safe during optimization
        // - JPEG: mozjpeg creates fresh JPEG from RGB pixels (no EXIF copied from DynamicImage)
        // - WebP: encoder creates fresh file from pixel data (no EXIF in DynamicImage)
        // - RAW: LibRaw outputs RGB pixels only, then encoded as JPEG (no metadata)
        // The metadata_cleaner is no longer needed as it was re-encoding and destroying optimizations.

        Ok(output)
    }

    /// Apply transformations to image
    fn apply_transformations(
        &self,
        img: &DynamicImage,
        transformation: &Transformation,
        original_dimensions: &Dimensions,
    ) -> InfraResult<DynamicImage> {
        let mut result = img.clone();

        // Aplicar resize si existe
        if let Some(resize) = transformation.resize() {
            result = self.resizer.resize(&result, resize, original_dimensions)?;
        }

        // Aplicar rotaciones y flips
        result = self.rotator.apply_transformations(
            &result,
            transformation.rotation(),
            transformation.flip_horizontal,
            transformation.flip_vertical,
        )?;

        Ok(result)
    }
}

impl ImageProcessor for ImageProcessorImpl {
    fn load_image(&self, path: &Path) -> DomainResult<Image> {
        // Verificar que el archivo existe
        if !path.exists() {
            return Err(DomainError::FileNotFound(
                path.to_string_lossy().to_string(),
            ));
        }

        // Detectar formato primero
        let format =
            ImageFormat::from_extension(path.extension().and_then(|s| s.to_str()).unwrap_or(""))?;

        // Obtener dimensiones según el tipo de archivo
        let dimensions = if format.is_raw() {
            // Para archivos RAW: decodificar para obtener dimensiones
            // No hay forma de obtener dimensiones sin decodificar en RAW
            // let dynamic_img = self
            //     .raw_processor
            //     .process_raw(path)
            //     .map_err(|e| DomainError::UnsupportedTransformation(e.to_string()))?;

            // let (width, height) = (dynamic_img.width(), dynamic_img.height());
            // Dimensions::new(width, height)?

            let (width, height) = RawProcessor::get_raw_metadata(&path)
                .map_err(|e| DomainError::UnsupportedTransformation(e.to_string()))?;
            Dimensions::new(width, height)?
        } else {
            // Para formatos estándar: OPTIMIZACIÓN - leer SOLO metadata sin decodificar
            // Esto es MUCHO más rápido que decodificar toda la imagen
            let reader = image::ImageReader::open(path)
                .map_err(|e| DomainError::UnsupportedTransformation(e.to_string()))?;

            // Obtener dimensiones SIN decodificar
            let dimensions_result = reader
                .into_dimensions()
                .map_err(|e| DomainError::UnsupportedTransformation(e.to_string()))?;
            let (width, height) = dimensions_result;
            Dimensions::new(width, height)?
        };

        // Obtener metadata del archivo (tamaño)
        let metadata_fs =
            fs::metadata(path).map_err(|e| DomainError::InvalidFilePath(e.to_string()))?;
        let size_bytes = metadata_fs.len();

        // Crear Image (solo metadata, no la imagen decodificada para formatos estándar)
        let image = Image::new(
            path.to_path_buf(),
            format,
            dimensions,
            size_bytes,
            None, // Metadata EXIF se agregará en Fase 7
        )?;

        Ok(image)
    }

    fn optimize(&self, image: &Image, settings: &ProcessingSettings) -> DomainResult<Vec<u8>> {
        // Cargar imagen
        let dynamic_img = self
            .load_dynamic_image(image.path())
            .map_err(|e| DomainError::UnsupportedTransformation(e.to_string()))?;

        // Determinar formato de salida
        let output_format = settings.determine_output_format(image.format());

        // Encodear y optimizar
        self.encode_image(&dynamic_img, output_format, settings)
            .map_err(|e| DomainError::UnsupportedTransformation(e.to_string()))
    }

    fn transform(&self, image: &Image, transformation: &Transformation) -> DomainResult<Vec<u8>> {
        // Cargar imagen
        let dynamic_img = self
            .load_dynamic_image(image.path())
            .map_err(|e| DomainError::UnsupportedTransformation(e.to_string()))?;

        // Aplicar transformaciones
        let transformed = self
            .apply_transformations(&dynamic_img, transformation, image.dimensions())
            .map_err(|e| DomainError::UnsupportedTransformation(e.to_string()))?;

        // Encodear (sin optimización especial)
        let mut bytes = Vec::new();
        let mut cursor = Cursor::new(&mut bytes);
        transformed
            .write_to(&mut cursor, Self::convert_format(image.format()))
            .map_err(|e| DomainError::UnsupportedTransformation(e.to_string()))?;

        Ok(bytes)
    }

    fn process(
        &self,
        image: &Image,
        transformation: Option<&Transformation>,
        settings: &ProcessingSettings,
    ) -> DomainResult<Vec<u8>> {
        // Cargar imagen
        let mut dynamic_img = self
            .load_dynamic_image(image.path())
            .map_err(|e| DomainError::UnsupportedTransformation(e.to_string()))?;

        // Aplicar transformaciones si existen
        if let Some(trans) = transformation {
            dynamic_img = self
                .apply_transformations(&dynamic_img, trans, image.dimensions())
                .map_err(|e| DomainError::UnsupportedTransformation(e.to_string()))?;
        }

        // Determinar formato de salida
        let output_format = settings.determine_output_format(image.format());

        // Optimizar y encodear
        self.encode_image(&dynamic_img, output_format, settings)
            .map_err(|e| DomainError::UnsupportedTransformation(e.to_string()))
    }

    fn save_image(
        &self,
        data: &[u8],
        output_path: &Path,
        _format: ImageFormat,
    ) -> DomainResult<()> {
        // Crear directorio si no existe
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).map_err(|e| DomainError::InvalidFilePath(e.to_string()))?;
        }

        // Escribir archivo
        fs::write(output_path, data).map_err(|e| DomainError::InvalidFilePath(e.to_string()))?;

        Ok(())
    }
}

impl Default for ImageProcessorImpl {
    fn default() -> Self {
        Self::new()
    }
}
