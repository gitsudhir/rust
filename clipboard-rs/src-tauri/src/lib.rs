use tauri::{Emitter};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_store::StoreExt;
use serde_json::Value;
use std::time::Duration;
use std::thread;
use uuid::Uuid;
use image::{ImageFormat, RgbaImage, GenericImageView};
use base64::{Engine as _, engine::general_purpose};

#[tauri::command]
fn read_clipboard_text(app: tauri::AppHandle) -> Result<String, String> {
    app.clipboard().read_text().map_err(|e| e.to_string())
}

#[tauri::command]
fn write_clipboard_text(app: tauri::AppHandle, text: &str) -> Result<(), String> {
    app.clipboard().write_text(text).map_err(|e| e.to_string())
}

#[tauri::command]
fn read_clipboard_image(app: tauri::AppHandle) -> Result<String, String> {
    match app.clipboard().read_image() {
        Ok(image) => {
            // Return image info with dimensions
            Ok(format!("Image data available: {}x{}", image.width(), image.height()))
        },
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
async fn save_clipboard_history(app: tauri::AppHandle, text: &str) -> Result<(), String> {
    // Don't save empty values
    if text.trim().is_empty() {
        return Ok(());
    }
    
    let store = app.store("clipboard-history.bin").map_err(|e| e.to_string())?;
    let mut history: Vec<Value> = store.get("history").unwrap_or(Value::Array(vec![])).as_array().cloned().unwrap_or(vec![]);
    
    // Check if the text already exists in history (for uniqueness)
    let text_exists = history.iter().any(|item| {
        if let Value::String(existing_text) = item {
            // Extract the text part (before the timestamp)
            let text_part = existing_text.split('|').next().unwrap_or(existing_text);
            text_part == text
        } else {
            false
        }
    });
    
    // Only add if it doesn't already exist
    if !text_exists {
        // Add timestamp to the text entry
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs();
        
        let entry = format!("{}|{}", text, timestamp);
        
        // Add the new entry to the beginning of the history
        history.insert(0, Value::String(entry));
        
        // Limit history to 50 items
        if history.len() > 50 {
            history.truncate(50);
        }
        
        store.set("history", Value::Array(history));
        store.save().map_err(|e| e.to_string())?;
        
        // Emit event to notify frontend of clipboard update
        let _ = app.emit("clipboard-update", ());
    }
    Ok(())
}

#[tauri::command]
fn load_clipboard_history(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let store = app.store("clipboard-history.bin").map_err(|e| e.to_string())?;
    let history: Vec<Value> = store.get("history").unwrap_or(Value::Array(vec![])).as_array().cloned().unwrap_or(vec![]);
    let string_history: Vec<String> = history.into_iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();
    Ok(string_history)
}

#[tauri::command]
fn clear_clipboard_history(app: tauri::AppHandle) -> Result<(), String> {
    let store = app.store("clipboard-history.bin").map_err(|e| e.to_string())?;
    store.delete("history");
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

// Function to save image data to a temporary file and return the file path
#[tauri::command]
fn save_clipboard_image_data(app: tauri::AppHandle) -> Result<String, String> {
    match app.clipboard().read_image() {
        Ok(image) => {
            // Create a unique filename for the image
            let filename = format!("{}.png", Uuid::new_v4());
            let temp_dir = std::env::temp_dir();
            let file_path = temp_dir.join(filename);
            
            // Convert clipboard image to RGBA image
            let width = image.width();
            let height = image.height();
            let rgba_data = image.rgba();
            
            // Create RGBA image from clipboard data
            let img = RgbaImage::from_raw(width, height, rgba_data.to_vec())
                .ok_or("Failed to create image from clipboard data")?;
            
            // Save the image to file
            img.save_with_format(&file_path, ImageFormat::Png)
                .map_err(|e| format!("Failed to save image: {}", e))?;
            
            // Return the file path
            Ok(file_path.to_string_lossy().to_string())
        },
        Err(e) => Err(e.to_string())
    }
}

// Function to copy an image file to a temporary location
fn copy_image_file_to_temp(file_path: String) -> Result<String, String> {
    let source_path = std::path::Path::new(&file_path);
    if !source_path.exists() {
        return Err("Source file does not exist".to_string());
    }
    
    // Create a unique filename for the copied image
    let filename = format!("{}.{}", Uuid::new_v4(), source_path.extension().unwrap_or_default().to_string_lossy());
    let temp_dir = std::env::temp_dir();
    let dest_path = temp_dir.join(filename);
    
    // Copy the file
    std::fs::copy(source_path, &dest_path)
        .map_err(|e| format!("Failed to copy image file: {}", e))?;
    
    // Return the destination file path
    Ok(dest_path.to_string_lossy().to_string())
}

// Function to load image from file and copy it to clipboard
#[tauri::command(rename_all = "snake_case")]
fn copy_image_from_file_to_clipboard(app: tauri::AppHandle, file_path: &str) -> Result<(), String> {
    // Load image from file
    let img = image::open(file_path)
        .map_err(|e| format!("Failed to load image from file: {}", e))?;
    
    // Convert to RGBA
    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();
    
    // Get the raw image data
    let raw_data = rgba_img.into_raw();
    
    // Create Image struct for clipboard
    let image = tauri::image::Image::new(&raw_data, width, height);
    
    // Copy to clipboard
    app.clipboard()
        .write_image(&image)
        .map_err(|e| format!("Failed to write image to clipboard: {}", e))?;
    
    Ok(())
}

// Function to generate base64 thumbnail for image preview
#[tauri::command]
fn get_image_thumbnail(_app: tauri::AppHandle, file_path: &str) -> Result<String, String> {
    // Load image from file
    let img = image::open(file_path)
        .map_err(|e| format!("Failed to load image from file: {}", e))?;
    
    // Resize to thumbnail size (100x100 max)
    let thumbnail = img.thumbnail(100, 100);
    
    // Encode as base64 PNG
    let mut buffer: Vec<u8> = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut buffer);
    thumbnail.write_to(&mut cursor, ImageFormat::Png)
        .map_err(|e| format!("Failed to encode image: {}", e))?;
    
    let base64_data = general_purpose::STANDARD.encode(&buffer);
    Ok(format!("data:image/png;base64,{}", base64_data))
}

// Function to start clipboard monitoring in a background thread
fn start_clipboard_monitoring(app_handle: tauri::AppHandle) {
    thread::spawn(move || {
        let mut last_clipboard_content = String::new();
        let mut last_image_hash = String::new();
        
        loop {
            // Sleep for a short duration to avoid excessive CPU usage
            thread::sleep(Duration::from_millis(500));
            
            // Try to read clipboard content
            let text_result = app_handle.clipboard().read_text();
            match text_result {
                Ok(current_content) => {
                    // Check if clipboard content has changed and is not empty
                    if current_content != last_clipboard_content && !current_content.trim().is_empty() {
                        // Update last clipboard content
                        last_clipboard_content = current_content.clone();
                        
                        // Check if the text content is a file path to an image file
                        let is_image_file = {
                            let path = std::path::Path::new(&current_content);
                            if path.exists() && path.is_file() {
                                // Check if it's an image file by extension
                                if let Some(extension) = path.extension() {
                                    let ext = extension.to_string_lossy().to_lowercase();
                                    matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "tiff" | "tif")
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        };
                        
                        if is_image_file {
                            // Handle file path to image file
                            // Create a simple hash of the file path to detect changes
                            let image_info = format!("ImageFile:{}", current_content);
                            
                            if image_info != last_image_hash {
                                last_image_hash = image_info.clone();
                                
                                // Try to load the image to get its dimensions
                                match image::open(&current_content) {
                                    Ok(img) => {
                                        let (width, height) = img.dimensions();
                                        
                                        // Copy the image file to our temp directory
                                        if let Ok(file_path) = copy_image_file_to_temp(current_content.clone()) {
                                            if let Ok(store) = app_handle.store("clipboard-history.bin") {
                                                let mut history: Vec<Value> = store.get("history").unwrap_or(Value::Array(vec![])).as_array().cloned().unwrap_or(vec![]);
                                                
                                                // Add timestamp to the image entry
                                                let timestamp = match std::time::SystemTime::now()
                                                    .duration_since(std::time::UNIX_EPOCH) {
                                                        Ok(duration) => duration.as_secs(),
                                                        Err(_) => 0, // fallback to 0 if timestamp fails
                                                    };
                                                
                                                let image_entry = format!("[Image] {}x{}|{}|{}", width, height, file_path, timestamp);
                                                history.insert(0, Value::String(image_entry));
                                                
                                                // Limit history to 50 items
                                                if history.len() > 50 {
                                                    history.truncate(50);
                                                }
                                                
                                                store.set("history", Value::Array(history));
                                                let _ = store.save();
                                                
                                                // Emit event to notify frontend of clipboard update
                                                let _ = app_handle.emit("clipboard-update", ());
                                            }
                                        }
                                    },
                                    Err(_) => {
                                        // If we can't load the image, still copy it but use placeholder dimensions
                                        if let Ok(file_path) = copy_image_file_to_temp(current_content.clone()) {
                                            if let Ok(store) = app_handle.store("clipboard-history.bin") {
                                                let mut history: Vec<Value> = store.get("history").unwrap_or(Value::Array(vec![])).as_array().cloned().unwrap_or(vec![]);
                                                
                                                // Add timestamp to the image entry
                                                let timestamp = match std::time::SystemTime::now()
                                                    .duration_since(std::time::UNIX_EPOCH) {
                                                        Ok(duration) => duration.as_secs(),
                                                        Err(_) => 0, // fallback to 0 if timestamp fails
                                                    };
                                                
                                                let image_entry = format!("[Image] Unknown|{}|{}", file_path, timestamp);
                                                history.insert(0, Value::String(image_entry));
                                                
                                                // Limit history to 50 items
                                                if history.len() > 50 {
                                                    history.truncate(50);
                                                }
                                                
                                                store.set("history", Value::Array(history));
                                                let _ = store.save();
                                                
                                                // Emit event to notify frontend of clipboard update
                                                let _ = app_handle.emit("clipboard-update", ());
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            // Handle regular text content
                            // Save to history (with uniqueness check)
                            if let Ok(store) = app_handle.store("clipboard-history.bin") {
                                let mut history: Vec<Value> = store.get("history").unwrap_or(Value::Array(vec![])).as_array().cloned().unwrap_or(vec![]);
                                
                                // Check if the text already exists in history (for uniqueness)
                                let text_exists = history.iter().any(|item| {
                                    if let Value::String(existing_text) = item {
                                        existing_text == &current_content
                                    } else {
                                        false
                                    }
                                });
                                
                                // Only add if it doesn't already exist
                                if !text_exists {
                                    // Add timestamp to the text entry
                                    let timestamp = match std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH) {
                                            Ok(duration) => duration.as_secs(),
                                            Err(_) => 0, // fallback to 0 if timestamp fails
                                        };
                                    
                                    let entry = format!("{}|{}", current_content, timestamp);
                                    
                                    // Add the new entry to the beginning of the history
                                    history.insert(0, Value::String(entry));
                                    
                                    // Limit history to 50 items
                                    if history.len() > 50 {
                                        history.truncate(50);
                                    }
                                    
                                    store.set("history", Value::Array(history));
                                    let _ = store.save();
                                    
                                    // Emit event to notify frontend of clipboard update
                                    let _ = app_handle.emit("clipboard-update", ());
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    // Ignore text reading errors
                }
            }
            
            // Always try to read image data
            match app_handle.clipboard().read_image() {
                Ok(image) => {
                    // Create a simple hash of the image data to detect changes
                    // Using dimensions for simplicity
                    let image_info = format!("Image:{}x{}", image.width(), image.height());
                    
                    if image_info != last_image_hash {
                        last_image_hash = image_info.clone();
                        
                        // Save the image data to a file
                        if let Ok(file_path) = save_clipboard_image_data(app_handle.clone()) {
                            if let Ok(store) = app_handle.store("clipboard-history.bin") {
                                let mut history: Vec<Value> = store.get("history").unwrap_or(Value::Array(vec![])).as_array().cloned().unwrap_or(vec![]);
                                
                                // Add timestamp to the image entry
                                let timestamp = match std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH) {
                                        Ok(duration) => duration.as_secs(),
                                        Err(_) => 0, // fallback to 0 if timestamp fails
                                    };
                                
                                let image_entry = format!("[Image] {}x{}|{}|{}", image.width(), image.height(), file_path, timestamp);
                                history.insert(0, Value::String(image_entry));
                                
                                // Limit history to 50 items
                                if history.len() > 50 {
                                    history.truncate(50);
                                }
                                
                                store.set("history", Value::Array(history));
                                let _ = store.save();
                                
                                // Emit event to notify frontend of clipboard update
                                let _ = app_handle.emit("clipboard-update", ());
                            }
                        }
                    }
                }
                Err(_) => {
                    // Ignore errors silently to avoid spamming logs
                }
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            // Start clipboard monitoring when the app starts
            let app_handle = app.handle().clone();
            start_clipboard_monitoring(app_handle);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            read_clipboard_text, 
            write_clipboard_text,
            read_clipboard_image,
            save_clipboard_history,
            load_clipboard_history,
            clear_clipboard_history,
            save_clipboard_image_data,
            copy_image_from_file_to_clipboard,
            get_image_thumbnail
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}