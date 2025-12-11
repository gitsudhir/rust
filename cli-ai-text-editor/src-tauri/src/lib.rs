// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_file_content(file_path: &str) -> Result<String, String> {
    println!("Reading file: {}", file_path);
    match fs::read_to_string(file_path) {
        Ok(content) => {
            println!("File read successfully");
            Ok(content)
        },
        Err(e) => {
            println!("Error reading file: {}", e);
            Err(format!("Failed to read file: {}", e))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, read_file_content])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}