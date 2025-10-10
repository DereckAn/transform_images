use crate::domain::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Image dimensions value object
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dimensions {
    width: u32,
    height: u32,
}

impl Dimensions {
    /// Create new dimensions
    pub fn new(width: u32, height: u32) -> DomainResult<Self> {
        if width == 0 || height == 0 {
            return Err(DomainError::InvalidDimensions(width, height));
        }
        Ok(Dimensions { width, height })
    }

    /// Get width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Calculate aspect ratio
    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }

    /// Calculate total pixels
    pub fn total_pixels(&self) -> u64 {
        self.width as u64 * self.height as u64
    }

    /// Check if dimensions are landscape
    pub fn is_landscape(&self) -> bool {
        self.width > self.height
    }

    /// Check if dimensions are portrait
    pub fn is_portrait(&self) -> bool {
        self.height > self.width
    }

    /// Check if dimensions are square
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// Scale dimensions by a factor, preserving aspect ratio
    pub fn scale(&self, factor: f64) -> DomainResult<Self> {
        let new_width = (self.width as f64 * factor).round() as u32;
        let new_height = (self.height as f64 * factor).round() as u32;
        Self::new(new_width, new_height)
    }

    /// Fit dimensions within a bounding box, preserving aspect ratio
    pub fn fit_within(&self, max_width: u32, max_height: u32) -> DomainResult<Self> {
        if self.width <= max_width && self.height <= max_height {
            return Ok(*self);
        }

        let width_ratio = max_width as f64 / self.width as f64;
        let height_ratio = max_height as f64 / self.height as f64;
        let scale_factor = width_ratio.min(height_ratio);

        self.scale(scale_factor)
    }
}

impl fmt::Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_dimensions() {
        let dims = Dimensions::new(1920, 1080).unwrap();
        assert_eq!(dims.width(), 1920);
        assert_eq!(dims.height(), 1080);
    }

    #[test]
    fn test_invalid_dimensions() {
        assert!(Dimensions::new(0, 100).is_err());
        assert!(Dimensions::new(100, 0).is_err());
        assert!(Dimensions::new(0, 0).is_err());
    }

    #[test]
    fn test_aspect_ratio() {
        let dims = Dimensions::new(1920, 1080).unwrap();
        assert!((dims.aspect_ratio() - 16.0 / 9.0).abs() < 0.01);
    }

    #[test]
    fn test_orientation() {
        let landscape = Dimensions::new(1920, 1080).unwrap();
        assert!(landscape.is_landscape());
        assert!(!landscape.is_portrait());

        let portrait = Dimensions::new(1080, 1920).unwrap();
        assert!(portrait.is_portrait());
        assert!(!portrait.is_landscape());

        let square = Dimensions::new(1000, 1000).unwrap();
        assert!(square.is_square());
    }

    #[test]
    fn test_scale() {
        let dims = Dimensions::new(100, 100).unwrap();
        let scaled = dims.scale(2.0).unwrap();
        assert_eq!(scaled.width(), 200);
        assert_eq!(scaled.height(), 200);
    }

    #[test]
    fn test_fit_within() {
        let dims = Dimensions::new(2000, 1000).unwrap();
        let fitted = dims.fit_within(1000, 1000).unwrap();
        assert_eq!(fitted.width(), 1000);
        assert_eq!(fitted.height(), 500);
    }
}
