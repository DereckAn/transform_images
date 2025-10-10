use crate::application::task_manager::TaskManager;
use parking_lot::Mutex;
use std::sync::Arc;

/// Application state shared across commands
pub struct AppState {
    /// Task manager for async processing
    pub task_manager: Arc<TaskManager>,
    /// Optional: Store processing statistics
    pub stats: Arc<Mutex<ProcessingStats>>,
}

#[derive(Debug, Default, Clone)]
pub struct ProcessingStats {
    pub total_processed: usize,
    pub total_saved_bytes: u64,
    pub total_images_processed: usize,
}

impl ProcessingStats {
    pub fn add_processed(&mut self, bytes_saved: u64) {
        self.total_processed += 1;
        self.total_saved_bytes += bytes_saved;
        self.total_images_processed += 1;
    }

    pub fn reset(&mut self) {
        self.total_processed = 0;
        self.total_saved_bytes = 0;
        self.total_images_processed = 0;
    }

    pub fn average_savings(&self) -> f64 {
        if self.total_processed == 0 {
            return 0.0;
        }
        self.total_saved_bytes as f64 / self.total_processed as f64
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            task_manager: Arc::new(TaskManager::new()),
            stats: Arc::new(Mutex::new(ProcessingStats::default())),
        }
    }

    pub fn update_stats(&self, bytes_saved: u64) {
        let mut stats = self.stats.lock();
        stats.add_processed(bytes_saved);
    }

    pub fn get_stats(&self) -> ProcessingStats {
        self.stats.lock().clone()
    }

    pub fn reset_stats(&self) {
        self.stats.lock().reset();
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
