use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::infrastructure::image_processor::RawProcessor;

/// File system utilities for reading and discovering images
pub struct FileHandler;

impl FileHandler {
    /// Discover image files in a directory (non-recursive)
    pub fn discover_images(dir: &Path) -> Vec<PathBuf> {
        WalkDir::new(dir)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| Self::is_image_file(e.path()))
            .map(|e| e.path().to_path_buf())
            .collect()
    }

    /// Check if a file is an image based on extension (includes RAW formats)
    pub fn is_image_file(path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();

            // Check standard formats
            if matches!(ext_str.as_str(), "png" | "jpg" | "jpeg" | "webp" | "gif") {
                return true;
            }

            // Check RAW formats
            RawProcessor::is_raw_format(&ext_str)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_image_file() {
        // Standard formats
        assert!(FileHandler::is_image_file(Path::new("test.png")));
        assert!(FileHandler::is_image_file(Path::new("test.jpg")));
        assert!(FileHandler::is_image_file(Path::new("test.jpeg")));
        assert!(FileHandler::is_image_file(Path::new("test.webp")));
        assert!(FileHandler::is_image_file(Path::new("test.gif")));

        // RAW formats
        assert!(FileHandler::is_image_file(Path::new("test.arw"))); // Sony
        assert!(FileHandler::is_image_file(Path::new("test.cr2"))); // Canon
        assert!(FileHandler::is_image_file(Path::new("test.nef"))); // Nikon
        assert!(FileHandler::is_image_file(Path::new("test.dng"))); // Adobe

        // Non-image files
        assert!(!FileHandler::is_image_file(Path::new("test.txt")));
        assert!(!FileHandler::is_image_file(Path::new("test.pdf")));
    }
}
