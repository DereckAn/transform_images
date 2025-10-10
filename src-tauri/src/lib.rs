// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Declare modules
mod application;
mod domain;
mod infrastructure;

use application::commands;
use application::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::load_image_info,
            commands::load_images_info,
            commands::process_images,
            commands::cancel_processing,
            commands::get_processing_status,
            commands::is_processing,
            commands::get_stats,
            commands::reset_stats,
            commands::get_optimal_threads,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
