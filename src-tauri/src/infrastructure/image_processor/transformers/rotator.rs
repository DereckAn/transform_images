use crate::domain::models::Rotation;
use crate::infrastructure::error::InfraResult;
use image::DynamicImage;

/// Image rotator and flipper
pub struct Rotator;

impl Rotator {
    pub fn new() -> Self {
        Self
    }

    /// Rotate an image
    pub fn rotate(&self, img: &DynamicImage, rotation: Rotation) -> InfraResult<DynamicImage> {
        let rotated = match rotation {
            Rotation::None => img.clone(),
            Rotation::Clockwise90 => img.rotate90(),
            Rotation::Rotate180 => img.rotate180(),
            Rotation::Clockwise270 => img.rotate270(),
        };

        Ok(rotated)
    }

    /// Flip image horizontally
    pub fn flip_horizontal(&self, img: &DynamicImage) -> InfraResult<DynamicImage> {
        Ok(img.fliph())
    }

    /// Flip image vertically
    pub fn flip_vertical(&self, img: &DynamicImage) -> InfraResult<DynamicImage> {
        Ok(img.flipv())
    }

    /// Apply all rotation and flip transformations
    pub fn apply_transformations(
        &self,
        img: &DynamicImage,
        rotation: Option<Rotation>,
        flip_h: bool,
        flip_v: bool,
    ) -> InfraResult<DynamicImage> {
        let mut result = img.clone();

        // Aplicar rotación primero
        if let Some(rot) = rotation {
            result = self.rotate(&result, rot)?;
        }

        // Luego flips
        if flip_h {
            result = self.flip_horizontal(&result)?;
        }

        if flip_v {
            result = self.flip_vertical(&result)?;
        }

        Ok(result)
    }
}

impl Default for Rotator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_rotator() {
        let _rotator = Rotator::new();
    }

    // Tests con imágenes reales en integration tests
}
