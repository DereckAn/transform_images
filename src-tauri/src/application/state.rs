use parking_lot::Mutex;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Application state shared across commands
pub struct AppState {
    /// Signal for cancelling ongoing operations
    pub cancel_signal: Arc<AtomicBool>,
    /// Optional: Store processing statistics
    pub stats: Arc<Mutex<ProcessingStats>>,
}

#[derive(Debug, Default)]
pub struct ProcessingStats {
    pub total_processed: usize,
    pub total_saved_bytes: u64
}

impl AppState {
    pub fn new() -> Self {
        Self {
            cancel_signal: Arc::new(AtomicBool::new(false)),
            stats: Arc::new(Mutex::new(ProcessingStats::default())),
        }
    }

    pub fn reset_cancel_signal(&self) {
        self.cancel_signal.store(false, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancel_signal.load(Ordering::SeqCst)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}