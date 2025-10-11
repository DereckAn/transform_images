use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{Image, ProcessingSettings, Transformation};
use crate::infrastructure::image_processor::{BatchProcessor, ProcessingResult, ProgressCallback};

/// Status of a processing task
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Idle,
    Running,
    Completed,
    Cancelled,
    Error(String),
}

/// Task manager for handling async image processing
pub struct TaskManager {
    batch_processor: Arc<BatchProcessor>,
    cancel_signal: Arc<AtomicBool>,
    status: Arc<RwLock<TaskStatus>>,
    results: Arc<Mutex<Vec<ProcessingResult>>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            batch_processor: Arc::new(BatchProcessor::new()),
            cancel_signal: Arc::new(AtomicBool::new(false)),
            status: Arc::new(RwLock::new(TaskStatus::Idle)),
            results: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Start processing images asynchronously
    pub async fn process_images(
        &self,
        images: Vec<Image>,
        transformation: Option<Transformation>,
        settings: ProcessingSettings,
        progress_callback: Option<ProgressCallback>,
    ) -> Result<Vec<ProcessingResult>, String> {
        // Verificar si ya hay una tarea corriendo
        {
            let current_status = self.status.read().await;
            if *current_status == TaskStatus::Running {
                return Err("A task is already running".to_string());
            }
        }

        // Reset cancel signal y status
        self.cancel_signal.store(false, Ordering::SeqCst);
        *self.status.write().await = TaskStatus::Running;
        self.results.lock().clear();

        // Clonar referencias para la tarea async
        let batch_processor = Arc::clone(&self.batch_processor);
        let cancel_signal = Arc::clone(&self.cancel_signal);

        // Procesar en un thread separado
        let handle = tokio::task::spawn_blocking(move || {
            batch_processor.process_batch(
                images,
                transformation,
                settings,
                cancel_signal,
                progress_callback,
            )
        });

        // Esperar resultado
        match handle.await {
            Ok(processing_results) => {
                // Verificar si fue cancelado
                if self.cancel_signal.load(Ordering::SeqCst) {
                    *self.status.write().await = TaskStatus::Cancelled;
                } else {
                    *self.status.write().await = TaskStatus::Completed;
                }

                // Guardar resultados
                *self.results.lock() = processing_results.clone();

                Ok(processing_results)
            }
            Err(e) => {
                let error_msg = format!("Task execution failed: {}", e);
                *self.status.write().await = TaskStatus::Error(error_msg.clone());
                Err(error_msg)
            }
        }
    }

    /// Cancel the current processing task
    pub async fn cancel(&self) {
        self.cancel_signal.store(true, Ordering::SeqCst);
        *self.status.write().await = TaskStatus::Cancelled;
    }

    /// Get current task status
    pub async fn get_status(&self) -> TaskStatus {
        self.status.read().await.clone()
    }

    /// Get results of last completed task
    pub fn get_results(&self) -> Vec<ProcessingResult> {
        self.results.lock().clone()
    }

    /// Check if a task is currently running
    pub async fn is_running(&self) -> bool {
        *self.status.read().await == TaskStatus::Running
    }

    /// Reset task manager to idle state
    pub async fn reset(&self) {
        self.cancel_signal.store(false, Ordering::SeqCst);
        *self.status.write().await = TaskStatus::Idle;
        self.results.lock().clear();
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_task_manager() {
        let manager = TaskManager::new();
        assert_eq!(manager.get_status().await, TaskStatus::Idle);
    }

    #[tokio::test]
    async fn test_cancel_signal() {
        let manager = TaskManager::new();
        assert!(!manager.is_running().await);

        manager.cancel().await;
        assert_eq!(manager.get_status().await, TaskStatus::Cancelled);
    }

    #[tokio::test]
    async fn test_reset() {
        let manager = TaskManager::new();
        manager.cancel().await;
        manager.reset().await;
        assert_eq!(manager.get_status().await, TaskStatus::Idle);
    }
}
