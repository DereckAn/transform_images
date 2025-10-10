use thiserror::Error;

/// Domain-level errors - business logic validation errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum DomainError {
    #[error("Invalid quality value: {0}. Quality must be between 1 and 100")]
    InvalidQuality(u8),

    #[error("Invalid dimensions: width={0}, height={1}. Both must be greater than 0")]
    InvalidDimensions(u32, u32),

    #[error("Invalid image format: {0}")]
    InvalidImageFormat(String),

    #[error("Invalid rotation angle: {0}. Must be 0, 90, 180, or   270")]
    InvalidRotation(i32),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid file path: {0}")]
    InvalidFilePath(String),

    #[error("Unsupported transformation: {0}")]
    UnsupportedTransformation(String),
}

pub type DomainResult<T> = Result<T, DomainError>;
