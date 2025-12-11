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
async fn generate_ai_text(prompt: &str) -> Result<String, String> {
    // Get API key from environment variable
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY environment variable not set".to_string())?;
    
    // Create HTTP client
    let client = Client::new();
    
    // Prepare the request payload
    let payload = serde_json::json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "max_tokens": 1000
    });
    
    // Send request to OpenAI API
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    // Check if the response status is successful
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("API request failed with status {}: {}", status, error_text));
    }
    
    // Parse response
    let response_json: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response JSON: {}", e))?;
    
    // Debug: Print the response for troubleshooting
    println!("OpenAI API Response: {:?}", response_json);
    
    // Extract the generated text with better error handling
    let generated_text = response_json
        .get("choices")
        .and_then(|choices| choices.as_array())
        .and_then(|choices| choices.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))
        .and_then(|content| content.as_str())
        .ok_or_else(|| {
            let error_detail = if let Some(error) = response_json.get("error") {
                format!("API Error: {:?}", error)
            } else {
                format!("Unexpected response structure: {:?}", response_json)
            };
            format!("Failed to extract generated text from response. {}", error_detail)
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
            generate_ai_text
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}