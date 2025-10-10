use crate::domain::value_objects::Quality;
use crate::infrastructure::error::{InfraError, InfraResult};
use oxipng::Options;

/// PNG lossless optimizer using oxipng
pub struct PngOptimizer;

impl PngOptimizer {
    pub fn new() -> Self {
        Self
    }

    /// Optimize PNG image data
    pub fn optimize(&self, input_data: &[u8], quality: Quality) -> InfraResult<Vec<u8>> {
        let options = self.create_options(quality);

        // oxipng optimiza desde memoria
        match oxipng::optimize_from_memory(input_data, &options) {
            Ok(optimized) => Ok(optimized),
            Err(e) => Err(InfraError::PngOptimizationFailed(e.to_string())),
        }
    }

    /// Create oxipng options based on quality
    fn create_options(&self, quality: Quality) -> Options {
        // Mapear quality (1-100) a nivel de optimización oxipng (0-6)
        // Quality más alta = más tiempo de procesamiento pero mejor compresión
        let optimization_level = match quality.value() {
            1..=20 => 1,   // Muy rápido
            21..=40 => 2,  // Rápido
            41..=60 => 3,  // Normal
            61..=80 => 4,  // Bueno
            81..=95 => 5,  // Muy bueno
            96..=100 => 6, // Máximo (más lento)
            _ => 3,
        };

        // CORRECCIÓN: Usar el método correcto para crear Options
        // En oxipng 9.x, usamos from_preset con el nivel
        let mut opts = Options::from_preset(optimization_level);

        // Configurar opciones de optimización
        opts.strip = oxipng::StripChunks::Safe; // Mantiene chunks importantes

        opts
    }
}

impl Default for PngOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_optimizer() {
        let optimizer = PngOptimizer::new();
        let options = optimizer.create_options(Quality::default());
        // Verificar que las opciones se crean correctamente
        assert_eq!(options.strip, oxipng::StripChunks::Safe);
    }

    #[test]
    fn test_quality_mapping() {
        let optimizer = PngOptimizer::new();

        // Baja calidad = optimización rápida
        let _opts_low = optimizer.create_options(Quality::new(20).unwrap());

        // Alta calidad = optimización máxima
        let _opts_high = optimizer.create_options(Quality::new(100).unwrap());

        // Si compila, el test pasa
    }
}
