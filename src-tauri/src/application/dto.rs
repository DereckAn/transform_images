use serde::{Deserialize, Serialize};

/// Data transfer objects for frontend-backend communication

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDto {
    pub path: String,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationOptionsDto {
    pub quality: u8,
    pub output_format: Option<String>,
    pub output_directory: String,
    pub preserve_metadata: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationOptionsDto {
    pub resize: Option<ResizeOptionsDto>,
    pub rotate: Option<i32>,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizeOptionsDto {
    pub width: u32,
    pub height: u32,
    pub preserve_aspect_ratio: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecessedImageDto {
    pub original_path: String,
    pub output_path: String,
    pub original_size: u64,
    pub output_size: u64,
    pub compression_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressPayload {
    pub current: usize,
    pub total: usize,
    pub current_file: String,
}