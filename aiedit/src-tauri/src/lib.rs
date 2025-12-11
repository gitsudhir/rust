// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use std::path::Path;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
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

#[tauri::command]
fn write_file(path: &str, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn create_directory(path: &str) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

#[tauri::command]
fn delete_file(path: &str) -> Result<(), String> {
    if Path::new(path).is_dir() {
        fs::remove_dir_all(path).map_err(|e| e.to_string())
    } else {
        fs::remove_file(path).map_err(|e| e.to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            read_file,
            read_file_content,
            write_file,
            create_directory,
            file_exists,
            delete_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}