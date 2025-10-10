use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::domain::models::{ResizeFilter, ResizeTransformation, Rotation};
use crate::domain::{Dimensions, Image, ImageFormat, ProcessingSettings, Quality, Transformation};
use crate::infrastructure::image_processor::ProcessingResult;

/// Data Transfer Objects for frontend-backend communication

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDto {
    pub path: String,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub size_bytes: u64,
}

impl From<&Image> for ImageDto {
    fn from(image: &Image) -> Self {
        ImageDto {
            path: image.path().to_string_lossy().to_string(),
            format: image.format().to_string(),
            width: image.dimensions().width(),
            height: image.dimensions().height(),
            size_bytes: image.size_bytes(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationOptionsDto {
    pub quality: u8,
    pub output_format: Option<String>,
    pub output_directory: String,
    pub preserve_metadata: bool,
    pub overwrite_existing: bool,
}

impl OptimizationOptionsDto {
    /// Convert DTO to domain ProcessingSettings
    pub fn to_domain(&self) -> Result<ProcessingSettings, String> {
        let quality = Quality::new(self.quality).map_err(|e| e.to_string())?;

        let output_format = if let Some(ref fmt) = self.output_format {
            Some(ImageFormat::from_extension(fmt).map_err(|e| e.to_string())?)
        } else {
            None
        };

        let mut settings = ProcessingSettings::new(quality, PathBuf::from(&self.output_directory));

        settings
            .set_output_format(output_format)
            .set_preserve_metadata(self.preserve_metadata)
            .set_overwrite_existing(self.overwrite_existing);

        Ok(settings)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationOptionsDto {
    pub resize: Option<ResizeOptionsDto>,
    pub rotate: Option<i32>,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

impl TransformationOptionsDto {
    /// Convert DTO to domain Transformation
    pub fn to_domain(&self) -> Result<Option<Transformation>, String> {
        if self.resize.is_none()
            && self.rotate.is_none()
            && !self.flip_horizontal
            && !self.flip_vertical
        {
            return Ok(None);
        }

        let mut transformation = Transformation::new();

        if let Some(ref resize_dto) = self.resize {
            let resize = resize_dto.to_domain()?;
            transformation.set_resize(resize);
        }

        if let Some(degrees) = self.rotate {
            let rotation = Rotation::from_degrees(degrees).map_err(|e| e.to_string())?;
            transformation.set_rotation(rotation);
        }

        transformation.set_flip_horizontal(self.flip_horizontal);
        transformation.set_flip_vertical(self.flip_vertical);

        Ok(Some(transformation))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizeOptionsDto {
    pub width: u32,
    pub height: u32,
    pub preserve_aspect_ratio: bool,
    pub filter: Option<String>,
}

impl ResizeOptionsDto {
    /// Convert DTO to domain ResizeTransformation
    pub fn to_domain(&self) -> Result<ResizeTransformation, String> {
        let dimensions = Dimensions::new(self.width, self.height).map_err(|e| e.to_string())?;

        let filter = if let Some(ref f) = self.filter {
            Self::parse_filter(f)?
        } else {
            ResizeFilter::Lanczos3
        };

        Ok(ResizeTransformation::new(
            dimensions,
            self.preserve_aspect_ratio,
            filter,
        ))
    }

    fn parse_filter(filter: &str) -> Result<ResizeFilter, String> {
        match filter.to_lowercase().as_str() {
            "nearest" => Ok(ResizeFilter::Nearest),
            "triangle" | "linear" => Ok(ResizeFilter::Triangle),
            "catmullrom" | "cubic" => Ok(ResizeFilter::CatmullRom),
            "gaussian" => Ok(ResizeFilter::Gaussian),
            "lanczos3" | "lanczos" => Ok(ResizeFilter::Lanczos3),
            _ => Err(format!("Unknown filter: {}", filter)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedImageDto {
    pub original_path: String,
    pub output_path: String,
    pub original_size: u64,
    pub output_size: u64,
    pub compression_ratio: f64,
    pub success: bool,
    pub error_message: Option<String>,
}

impl From<ProcessingResult> for ProcessedImageDto {
    fn from(result: ProcessingResult) -> Self {
        ProcessedImageDto {
            original_path: result.original_path.to_string_lossy().to_string(),
            output_path: result.output_path.to_string_lossy().to_string(),
            original_size: result.original_size,
            output_size: result.output_size,
            compression_ratio: result.compression_ratio(),
            success: result.success,
            error_message: result.error_message,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressPayload {
    pub current: usize,
    pub total: usize,
    pub current_file: String,
    pub percentage: f64,
}

impl ProgressPayload {
    pub fn new(current: usize, total: usize, current_file: String) -> Self {
        let percentage = if total > 0 {
            (current as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        ProgressPayload {
            current,
            total,
            current_file,
            percentage,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchProcessRequest {
    pub image_paths: Vec<String>,
    pub optimization_options: OptimizationOptionsDto,
    pub transformation_options: Option<TransformationOptionsDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingStatsDto {
    pub total_processed: usize,
    pub total_saved_bytes: u64,
    pub average_savings: f64,
}
