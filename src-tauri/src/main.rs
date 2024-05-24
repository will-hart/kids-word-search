// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use words::{build_grid, WordGrid, WordListType};

mod words;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_grid(size: usize) -> WordGrid {
    build_grid(WordListType::EarlyPrimary, size.clamp(5, 15))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_grid])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
