// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Declare modules
mod application;
mod domain;
mod infrastructure;

use application::commands;
use application::state::AppState;

#[cfg_attr(mobile,tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_dialog::init())
    .manage(AppState::new())
    .invoke_handler(tauri::generate_handler![
        commands::greet,
        // Mas comandos se agregaran aqui
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
