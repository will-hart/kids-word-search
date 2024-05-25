// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use words::{build_grid, WordGrid, WordListType};

mod words;

#[tauri::command]
fn get_grid(list: WordListType, size: usize) -> WordGrid {
    build_grid(list, size.clamp(5, 20))
}

#[tauri::command]
fn get_word_list_options() -> Vec<WordListType> {
    WordListType::get_options()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_grid, get_word_list_options])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
