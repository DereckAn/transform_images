use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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

    /// Check if a file is an image based on extension
    pub fn is_image_file(path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "webp" | "gif")
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
        assert!(FileHandler::is_image_file(Path::new("test.png")));
        assert!(FileHandler::is_image_file(Path::new("test.jpg")));
        assert!(FileHandler::is_image_file(Path::new("test.jpeg")));
        assert!(!FileHandler::is_image_file(Path::new("test.txt")));
    }
}
