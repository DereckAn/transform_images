use crate::domain::error::DomainError;
use thiserror::Error;

/// Infrastructure layer errors
#[derive(Error, Debug)]
pub enum InfraError {
    #[error("Failed to read image: {0}")]
    ImageReadError(String),

    #[error("Failed to write image: {0}")]
    ImageWriteError(String),

    #[error("Failed to decode image: {0}")]
    DecodeError(String),

    #[error("Failed to encode image: {0}")]
    EncodeError(String),

    #[error("PNG optimization failed: {0}")]
    PngOptimizationFailed(String),

    #[error("JPEG optimization failed: {0}")]
    JpegOptimizationFailed(String),

    #[error("Unsupported format for optimization: {0}")]
    UnsupportedFormat(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
}

pub type InfraResult<T> = Result<T, InfraError>;

// Conversión de InfraError a DomainError para mantener la separación de capas
impl From<InfraError> for DomainError {
    fn from(err: InfraError) -> Self {
        DomainError::UnsupportedTransformation(err.to_string())
    }
}
