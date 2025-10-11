 use tauri::Manager;

// Re-exportar módulos principales como públicos
  pub mod application;
  pub mod domain;
  pub mod infrastructure;

  // Re-exportar tipos comúnmente usados para facilitar imports
  pub use domain::{
      error::{DomainError, DomainResult},
      models::{
          Image,
          ProcessingSettings,
          ResizeFilter,
          ResizeTransformation,
          Rotation,
          Transformation
      },
      services::ImageProcessor,
      value_objects::{Dimensions, ImageFormat, Quality},
  };

  pub use infrastructure::{
      error::{InfraError, InfraResult},
      image_processor::ImageProcessorImpl,
  };


  #[cfg_attr(mobile, tauri::mobile_entry_point)]
  pub fn run() {
      tauri::Builder::default()
          .plugin(tauri_plugin_opener::init())
          .plugin(tauri_plugin_dialog::init())
          .setup(|app| {
              let app_state = application::state::AppState::new();
              app.manage(app_state);
              Ok(())
          })
          .invoke_handler(tauri::generate_handler![
              application::commands::greet,
              application::commands::load_image_info,
              application::commands::load_images_info,
              application::commands::load_images_from_folder,
              application::commands::process_images,
              application::commands::cancel_processing,
              application::commands::get_processing_status,
              application::commands::is_processing,
              application::commands::get_stats,
              application::commands::reset_stats,
              application::commands::get_optimal_threads,
          ])
          .run(tauri::generate_context!())
          .expect("error while running tauri application");
  }