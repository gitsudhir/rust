// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use std::path::Path;
use std::env;
use reqwest::Client;
use serde_json::Value;

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

#[tauri::command]
fn list_directory_contents(path: &str) -> Result<Vec<(String, bool)>, String> {
    let mut entries = Vec::new();
    let dir_path = Path::new(path);
    
    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let file_name = entry.file_name().to_string_lossy().to_string();
            let is_dir = entry.file_type().map_err(|e| e.to_string())?.is_dir();
            entries.push((file_name, is_dir));
        }
        // Sort entries: directories first, then files, both alphabetically
        entries.sort_by(|a, b| {
            if a.1 && !b.1 {
                std::cmp::Ordering::Less
            } else if !a.1 && b.1 {
                std::cmp::Ordering::Greater
            } else {
                a.0.cmp(&b.0)
            }
        });
        Ok(entries)
    } else {
        Err("Path is not a directory".to_string())
    }
}

#[tauri::command]
async fn generate_ai_text(prompt: &str) -> Result<String, String> {
    // Create HTTP client
    let client = Client::new();
    
    // Prepare the request payload for Ollama API
    let payload = serde_json::json!({
        "model": "llama3",
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "stream": false
    });
    
    // Send request to local Ollama API
    let response = client
        .post("http://localhost:11434/api/chat")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to send request to Ollama: {}", e))?;
    
    // Check if the response status is successful
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Ollama API request failed with status {}: {}", status, error_text));
    }
    
    // Parse response
    let response_json: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Ollama response JSON: {}", e))?;
    
    // Debug: Print the response for troubleshooting
    println!("Ollama API Response: {:?}", response_json);
    
    // Extract the generated text with better error handling
    let generated_text = response_json
        .get("message")
        .and_then(|message| message.get("content"))
        .and_then(|content| content.as_str())
        .ok_or_else(|| {
            let error_detail = if let Some(error) = response_json.get("error") {
                format!("Ollama API Error: {:?}", error)
            } else {
                format!("Unexpected response structure from Ollama: {:?}", response_json)
            };
            format!("Failed to extract generated text from Ollama response. {}", error_detail)
        })?
        .to_string();
    
    Ok(generated_text)
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
            delete_file,
            list_directory_contents,
            generate_ai_text
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}