use crate::domain::error::{DomainError, DomainResult};
use crate::domain::value_objects::Dimensions;
use serde::{Deserialize, Serialize};

/// Represents a set of transformations to apply to an image
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Transformation {
    /// Resize transformation
    pub resize: Option<ResizeTransformation>,
    /// Rotation in degrees (0, 90, 180, 270)
    pub rotation: Option<Rotation>,
    /// Flip horizontal
    pub flip_horizontal: bool,
    /// Flip vertical
    pub flip_vertical: bool,
}

impl Transformation {
    /// Create a new empty transformation
    pub fn new() -> Self {
        Self::default()
    }

    /// Create transformation with resize
    pub fn with_resize(resize: ResizeTransformation) -> Self {
        Self {
            resize: Some(resize),
            ..Default::default()
        }
    }

    /// Create transformation with rotation
    pub fn with_rotation(rotation: Rotation) -> Self {
        Self {
            rotation: Some(rotation),
            ..Default::default()
        }
    }

    /// Add resize transformation
    pub fn set_resize(&mut self, resize: ResizeTransformation) -> &mut Self {
        self.resize = Some(resize);
        self
    }

    /// Add rotation transformation
    pub fn set_rotation(&mut self, rotation: Rotation) -> &mut Self {
        self.rotation = Some(rotation);
        self
    }

    /// Set flip horizontal
    pub fn set_flip_horizontal(&mut self, flip: bool) -> &mut Self {
        self.flip_horizontal = flip;
        self
    }

    /// Set flip vertical
    pub fn set_flip_vertical(&mut self, flip: bool) -> &mut Self {
        self.flip_vertical = flip;
        self
    }

    /// Check if transformation has any operations
    pub fn has_operations(&self) -> bool {
        self.resize.is_some()
            || self.rotation.is_some()
            || self.flip_horizontal
            || self.flip_vertical
    }

    /// Get resize if present
    pub fn resize(&self) -> Option<&ResizeTransformation> {
        self.resize.as_ref()
    }

    /// Get rotation if present
    pub fn rotation(&self) -> Option<Rotation> {
        self.rotation
    }
}

/// Resize transformation options
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResizeTransformation {
    /// Target dimensions
    target_dimensions: Dimensions,
    /// Preserve aspect ratio
    preserve_aspect_ratio: bool,
    /// Resize algorithm/filter
    filter: ResizeFilter,
}

impl ResizeTransformation {
    /// Create a new resize transformation
    pub fn new(
        target_dimensions: Dimensions,
        preserve_aspect_ratio: bool,
        filter: ResizeFilter,
    ) -> Self {
        Self {
            target_dimensions,
            preserve_aspect_ratio,
            filter,
        }
    }

    /// Create resize with default filter (Lanczos3)
    pub fn with_dimensions(target_dimensions: Dimensions, preserve_aspect_ratio: bool) -> Self {
        Self::new(
            target_dimensions,
            preserve_aspect_ratio,
            ResizeFilter::Lanczos3,
        )
    }

    /// Get target dimensions
    pub fn target_dimensions(&self) -> &Dimensions {
        &self.target_dimensions
    }

    /// Check if aspect ratio should be preserved
    pub fn preserve_aspect_ratio(&self) -> bool {
        self.preserve_aspect_ratio
    }

    /// Get resize filter
    pub fn filter(&self) -> ResizeFilter {
        self.filter
    }

    /// Calculate final dimensions based on original dimensions
    pub fn calculate_final_dimensions(&self, original: &Dimensions) -> DomainResult<Dimensions> {
        if self.preserve_aspect_ratio {
            original.fit_within(
                self.target_dimensions.width(),
                self.target_dimensions.height(),
            )
        } else {
            Ok(self.target_dimensions)
        }
    }
}

/// Image resize filters/algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResizeFilter {
    /// Nearest neighbor (fastest, lowest quality)
    Nearest,
    /// Linear interpolation
    Triangle,
    /// Cubic interpolation
    CatmullRom,
    /// Gaussian filter
    Gaussian,
    /// Lanczos with window 3 (best quality, slower)
    Lanczos3,
}

impl Default for ResizeFilter {
    fn default() -> Self {
        ResizeFilter::Lanczos3
    }
}

/// Rotation angles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rotation {
    /// No rotation
    None = 0,
    /// Rotate 90 degrees clockwise
    Clockwise90 = 90,
    /// Rotate 180 degrees
    Rotate180 = 180,
    /// Rotate 270 degrees clockwise (90 counter-clockwise)
    Clockwise270 = 270,
}

impl Rotation {
    /// Create rotation from degrees
    pub fn from_degrees(degrees: i32) -> DomainResult<Self> {
        match degrees {
            0 => Ok(Rotation::None),
            90 => Ok(Rotation::Clockwise90),
            180 => Ok(Rotation::Rotate180),
            270 => Ok(Rotation::Clockwise270),
            _ => Err(DomainError::InvalidRotation(degrees)),
        }
    }

    /// Get rotation angle in degrees
    pub fn degrees(&self) -> i32 {
        *self as i32
    }

    /// Check if rotation changes dimensions (90 or 270)
    pub fn swaps_dimensions(&self) -> bool {
        matches!(self, Rotation::Clockwise90 | Rotation::Clockwise270)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_transformation() {
        let t = Transformation::new();
        assert!(!t.has_operations());
    }

    #[test]
    fn test_transformation_with_operations() {
        let mut t = Transformation::new();
        t.set_flip_horizontal(true);
        assert!(t.has_operations());
    }

    #[test]
    fn test_resize_transformation() {
        let dims = Dimensions::new(1920, 1080).unwrap();
        let resize = ResizeTransformation::with_dimensions(dims, true);

        assert_eq!(resize.target_dimensions(), &dims);
        assert!(resize.preserve_aspect_ratio());
    }

    #[test]
    fn test_rotation_from_degrees() {
        assert_eq!(Rotation::from_degrees(90).unwrap(), Rotation::Clockwise90);
        assert_eq!(Rotation::from_degrees(180).unwrap(), Rotation::Rotate180);
        assert!(Rotation::from_degrees(45).is_err());
    }

    #[test]
    fn test_rotation_swaps_dimensions() {
        assert!(Rotation::Clockwise90.swaps_dimensions());
        assert!(Rotation::Clockwise270.swaps_dimensions());
        assert!(!Rotation::None.swaps_dimensions());
        assert!(!Rotation::Rotate180.swaps_dimensions());
    }

    #[test]
    fn test_calculate_final_dimensions() {
        let original = Dimensions::new(2000, 1000).unwrap();
        let target = Dimensions::new(1000, 1000).unwrap();
        let resize = ResizeTransformation::with_dimensions(target, true);

        let final_dims = resize.calculate_final_dimensions(&original).unwrap();
        assert_eq!(final_dims.width(), 1000);
        assert_eq!(final_dims.height(), 500); // Mantiene aspect ratio
    }
}
