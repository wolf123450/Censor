#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod commands;
mod models;
mod ollama;
mod image_processor;
mod config;
mod state;
mod build_config;

use std::sync::Mutex;

#[derive(Default)]
struct AppState {
    current_project: Mutex<Option<String>>,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::process_image,
            commands::detect_content,
            commands::apply_censoring,
            commands::get_censoring_regions,
            commands::adjust_region,
            commands::save_project,
            commands::load_project,
            commands::get_config,
            commands::set_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
