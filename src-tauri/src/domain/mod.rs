pub mod error;
pub mod models;
pub mod services;
pub mod value_objects;

// Re-export commonly used types
pub use error::{DomainError, DomainResult};
pub use models::{Image, ProcessingSettings, Transformation};
pub use services::ImageProcessor;
pub use value_objects::{Dimensions, ImageFormat};
// Quality se usará en fases posteriores
// pub use value_objects::Quality;
