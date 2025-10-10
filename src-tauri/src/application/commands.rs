use tauri::State;
use crate::application::state::AppState;

#[tauri::command]
pub fn greet(name: &str, _state: State<AppState>) -> String {
    format!("Hello, {}! Transform Images is ready.", name)
}


  // TODO: Agregar más comandos en fases posteriores
  // - optimize_images
  // - transform_images
  // - cancel_operation
  // - select_output_directory