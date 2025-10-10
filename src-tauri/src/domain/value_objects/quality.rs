use crate::domain::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Image quality value object (1-100)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Quality(u8);

impl Quality {
    /// Minimum quality value
    pub const MIN: u8 = 1;
    /// Maximum quality value
    pub const MAX: u8 = 100;

    /// Create a new Quality value
    pub fn new(value: u8) -> DomainResult<Self> {
        if value < Self::MIN || value > Self::MAX {
            return Err(DomainError::InvalidQuality(value));
        }
        Ok(Quality(value))
    }

    /// Create Quality with default value (85)
    pub fn default_quality() -> Self {
        Quality(85)
    }

    /// Create Quality for maximum quality (100)
    pub fn maximum() -> Self {
        Quality(100)
    }

    /// Create Quality for web optimization (80)
    pub fn web_optimized() -> Self {
        Quality(80)
    }

    /// Get the inner value
    pub fn value(&self) -> u8 {
        self.0
    }

    /// Get as f32 (0.0 - 1.0)
    pub fn as_normalized(&self) -> f32 {
        self.0 as f32 / 100.0
    }
}

impl Default for Quality {
    fn default() -> Self {
        Self::default_quality()
    }
}

impl fmt::Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

impl TryFrom<u8> for Quality {
    type Error = DomainError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_quality() {
        assert!(Quality::new(50).is_ok());
        assert!(Quality::new(1).is_ok());
        assert!(Quality::new(100).is_ok());
    }

    #[test]
    fn test_invalid_quality() {
        assert!(Quality::new(0).is_err());
        assert!(Quality::new(101).is_err());
        assert!(Quality::new(255).is_err());
    }

    #[test]
    fn test_default_quality() {
        assert_eq!(Quality::default().value(), 85);
    }

    #[test]
    fn test_as_normalized() {
        assert_eq!(Quality::new(100).unwrap().as_normalized(), 1.0);
        assert_eq!(Quality::new(50).unwrap().as_normalized(), 0.5);
    }

    #[test]
    fn test_presets() {
        assert_eq!(Quality::maximum().value(), 100);
        assert_eq!(Quality::web_optimized().value(), 80);
    }
}
