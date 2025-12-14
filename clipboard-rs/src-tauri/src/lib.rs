use tauri::{Emitter};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_store::StoreExt;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::thread;
use uuid::Uuid;
use image::{ImageFormat, RgbaImage, GenericImageView};
use base64::{Engine as _, engine::general_purpose};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ClipboardItem {
    content: String,
    timestamp: u64,
    is_favorite: bool,
    tags: Vec<String>,
}

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
    let mut history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
    
    // Check if the text already exists in history (for uniqueness)
    let text_exists = history.iter().any(|item| {
        // Extract the text part (before the timestamp)
        let text_part = item.content.split('|').next().unwrap_or(&item.content);
        text_part == text
    });
    
    // Only add if it doesn't already exist
    if !text_exists {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs();
        
        let new_item = ClipboardItem {
            content: format!("{}|{}", text, timestamp),
            timestamp,
            is_favorite: false,
            tags: vec![],
        };
        
        // Add the new entry to the beginning of the history
        history.insert(0, new_item);
        
        // Limit history to 50 items
        if history.len() > 50 {
            history.truncate(50);
        }
        
        store.set("history", serde_json::to_value(&history).map_err(|e| e.to_string()).unwrap());
        store.save().map_err(|e| e.to_string())?;
        
        // Emit event to notify frontend of clipboard update
        let _ = app.emit("clipboard-update", ());
    }
    Ok(())
}

#[tauri::command]
fn load_clipboard_history(app: tauri::AppHandle) -> Result<Vec<ClipboardItem>, String> {
    let store = app.store("clipboard-history.bin").map_err(|e| e.to_string())?;
    let history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
    Ok(history)
}

#[tauri::command]
fn clear_clipboard_history(app: tauri::AppHandle) -> Result<(), String> {
    let store = app.store("clipboard-history.bin").map_err(|e| e.to_string())?;
    store.delete("history");
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

// Function to search clipboard history
#[tauri::command]
fn search_clipboard_history(app: tauri::AppHandle, query: &str) -> Result<Vec<String>, String> {
    let store = app.store("clipboard-history.bin").map_err(|e| e.to_string())?;
    let history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
    let string_history: Vec<String> = history.into_iter()
        .filter(|item| {
            // Extract the text part (before the timestamp)
            let text_part = item.content.split('|').next().unwrap_or(&item.content);
            text_part.to_lowercase().contains(&query.to_lowercase())
        })
        .map(|item| item.content)
        .collect();
    Ok(string_history)
}

// Function to toggle favorite status of an item
#[tauri::command]
fn toggle_favorite(app: tauri::AppHandle, item_content: &str) -> Result<bool, String> {
    let store = app.store("clipboard-history.bin").map_err(|e| e.to_string())?;
    let mut history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
    
    let mut is_now_favorite = false;
    
    // Find the item and toggle its favorite status
    for item in history.iter_mut() {
        if item.content == item_content {
            item.is_favorite = !item.is_favorite;
            is_now_favorite = item.is_favorite;
            break;
        }
    }
    
    store.set("history", serde_json::to_value(history).map_err(|e| e.to_string())?);
    store.save().map_err(|e| e.to_string())?;
    
    Ok(is_now_favorite)
}

// Function to load favorite items
#[tauri::command]
fn load_favorites(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let store = app.store("clipboard-history.bin").map_err(|e| e.to_string())?;
    let history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
    let favorites: Vec<String> = history.into_iter()
        .filter(|item| item.is_favorite)
        .map(|item| item.content)
        .collect();
    Ok(favorites)
}

// Function to clean up old clipboard items
#[tauri::command]
fn cleanup_old_items(app: tauri::AppHandle, max_age_seconds: u64) -> Result<usize, String> {
    let store = app.store("clipboard-history.bin").map_err(|e| e.to_string())?;
    let history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
    
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();
    
    let original_count = history.len();
    
    // Filter out items older than max_age_seconds, but keep favorites
    let filtered_history: Vec<ClipboardItem> = history.into_iter()
        .filter(|item| {
            // Keep favorites regardless of age
            item.is_favorite || (current_time - item.timestamp) <= max_age_seconds
        })
        .collect();
    
    let removed_count = original_count - filtered_history.len();
    
    store.set("history", serde_json::to_value(filtered_history).map_err(|e| e.to_string()).unwrap());
    store.save().map_err(|e| e.to_string())?;
    
    // Emit event to notify frontend of clipboard update
    let _ = app.emit("clipboard-update", ());
    
    Ok(removed_count)
}

// Function to export clipboard history to JSON
#[tauri::command]
fn export_history(app: tauri::AppHandle) -> Result<String, String> {
    let store = app.store("clipboard-history.bin").map_err(|e| e.to_string())?;
    let history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
    
    serde_json::to_string_pretty(&history).map_err(|e| e.to_string())
}

// Function to import clipboard history from JSON
#[tauri::command]
fn import_history(app: tauri::AppHandle, json_data: &str) -> Result<usize, String> {
    let imported_history: Vec<ClipboardItem> = serde_json::from_str(json_data).map_err(|e| e.to_string())?;
    
    let store = app.store("clipboard-history.bin").map_err(|e| e.to_string())?;
    let mut existing_history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
    
    let original_count = existing_history.len();
    
    // Add imported items to existing history
    existing_history.extend(imported_history);
    
    // Remove duplicates based on content
    existing_history.sort_by_key(|item| item.content.clone());
    existing_history.dedup_by_key(|item| item.content.clone());
    
    // Limit to 100 items
    if existing_history.len() > 100 {
        existing_history.truncate(100);
    }
    
    let added_count = existing_history.len() - original_count;
    
    store.set("history", serde_json::to_value(&existing_history).map_err(|e| e.to_string()).unwrap());
    store.save().map_err(|e| e.to_string())?;
    
    // Emit event to notify frontend of clipboard update
    let _ = app.emit("clipboard-update", ());
    
    Ok(added_count)
}

// Function to add a tag to an item
#[tauri::command]
fn add_tag_to_item(app: tauri::AppHandle, item_content: &str, tag: &str) -> Result<(), String> {
    let store = app.store("clipboard-history.bin").map_err(|e| e.to_string())?;
    let mut history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
    
    // Find the item and add the tag
    for item in history.iter_mut() {
        if item.content == item_content {
            // Check if tag already exists
            if !item.tags.contains(&tag.to_string()) {
                item.tags.push(tag.to_string());
            }
            break;
        }
    }
    
    store.set("history", serde_json::to_value(&history).map_err(|e| e.to_string()).unwrap());
    store.save().map_err(|e| e.to_string())?;
    
    // Emit event to notify frontend of clipboard update
    let _ = app.emit("clipboard-update", ());
    
    Ok(())
}

// Function to remove a tag from an item
#[tauri::command]
fn remove_tag_from_item(app: tauri::AppHandle, item_content: &str, tag: &str) -> Result<(), String> {
    let store = app.store("clipboard-history.bin").map_err(|e| e.to_string())?;
    let mut history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
    
    // Find the item and remove the tag
    for item in history.iter_mut() {
        if item.content == item_content {
            item.tags.retain(|t| t != tag);
            break;
        }
    }
    
    store.set("history", serde_json::to_value(&history).map_err(|e| e.to_string()).unwrap());
    store.save().map_err(|e| e.to_string())?;
    
    // Emit event to notify frontend of clipboard update
    let _ = app.emit("clipboard-update", ());
    
    Ok(())
}

// Function to get all unique tags
#[tauri::command]
fn get_all_tags(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let store = app.store("clipboard-history.bin").map_err(|e| e.to_string())?;
    let history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
    
    let mut tags: Vec<String> = Vec::new();
    for item in history {
        tags.extend(item.tags);
    }
    
    // Remove duplicates
    tags.sort();
    tags.dedup();
    
    Ok(tags)
}

// Function to get clipboard statistics
#[tauri::command]
fn get_clipboard_statistics(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let store = app.store("clipboard-history.bin").map_err(|e| e.to_string())?;
    let history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
    
    let total_items = history.len();
    let favorite_items = history.iter().filter(|item| item.is_favorite).count();
    let text_items = history.iter().filter(|item| !item.content.starts_with("[Image]")).count();
    let image_items = history.iter().filter(|item| item.content.starts_with("[Image]")).count();
    
    // Count tags
    let mut tag_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for item in &history {
        for tag in &item.tags {
            *tag_counts.entry(tag.clone()).or_insert(0) += 1;
        }
    }
    
    // Get most used tags (top 5)
    let mut sorted_tags: Vec<(String, usize)> = tag_counts.into_iter().collect();
    sorted_tags.sort_by(|a, b| b.1.cmp(&a.1));
    let top_tags = sorted_tags.into_iter().take(5).collect::<Vec<_>>();
    
    // Calculate date range
    let earliest_timestamp = history.iter().map(|item| item.timestamp).min().unwrap_or(0);
    let latest_timestamp = history.iter().map(|item| item.timestamp).max().unwrap_or(0);
    
    let stats = serde_json::json!({
        "totalItems": total_items,
        "favoriteItems": favorite_items,
        "textItems": text_items,
        "imageItems": image_items,
        "topTags": top_tags,
        "earliestTimestamp": earliest_timestamp,
        "latestTimestamp": latest_timestamp,
    });
    
    Ok(stats)
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
    
    // Calculate dimensions for thumbnail while preserving aspect ratio
    let (width, height) = img.dimensions();
    let max_size = 200u32;
    
    let scale_factor = if width > height {
        max_size as f32 / width as f32
    } else {
        max_size as f32 / height as f32
    };
    
    let new_width = (width as f32 * scale_factor) as u32;
    let new_height = (height as f32 * scale_factor) as u32;
    
    // Resize to thumbnail size while preserving aspect ratio
    let thumbnail = img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);
    
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
                                                let mut history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
                                                
                                                // Add timestamp to the image entry
                                                let timestamp = match std::time::SystemTime::now()
                                                    .duration_since(std::time::UNIX_EPOCH) {
                                                        Ok(duration) => duration.as_secs(),
                                                        Err(_) => 0, // fallback to 0 if timestamp fails
                                                    };
                                                
                                                let image_entry = ClipboardItem {
                                                    content: format!("[Image] {}x{}|{}|{}", width, height, file_path, timestamp),
                                                    timestamp,
                                                    is_favorite: false,
                                                    tags: vec![],
                                                };
                                                history.insert(0, image_entry);
                                                
                                                // Limit history to 50 items
                                                if history.len() > 50 {
                                                    history.truncate(50);
                                                }
                                                
                                                store.set("history", serde_json::to_value(history).map_err(|e| e.to_string()).unwrap());
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
                                                let mut history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
                                                
                                                // Add timestamp to the image entry
                                                let timestamp = match std::time::SystemTime::now()
                                                    .duration_since(std::time::UNIX_EPOCH) {
                                                        Ok(duration) => duration.as_secs(),
                                                        Err(_) => 0, // fallback to 0 if timestamp fails
                                                    };
                                                
                                                let image_entry = ClipboardItem {
                                                    content: format!("[Image] Unknown|{}|{}", file_path, timestamp),
                                                    timestamp,
                                                    is_favorite: false,
                                                    tags: vec![],
                                                };
                                                history.insert(0, image_entry);
                                                
                                                // Limit history to 50 items
                                                if history.len() > 50 {
                                                    history.truncate(50);
                                                }
                                                
                                                store.set("history", serde_json::to_value(history).map_err(|e| e.to_string()).unwrap());
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
                                let mut history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
                                
                                // Check if the text already exists in history (for uniqueness)
                                let text_exists = history.iter().any(|item| {
                                    // Extract the text part (before the timestamp)
                                    let text_part = item.content.split('|').next().unwrap_or(&item.content);
                                    text_part == &current_content
                                });
                                
                                // Only add if it doesn't already exist
                                if !text_exists {
                                    let timestamp = match std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH) {
                                            Ok(duration) => duration.as_secs(),
                                            Err(_) => 0, // fallback to 0 if timestamp fails
                                        };
                                    
                                    let new_item = ClipboardItem {
                                        content: format!("{}|{}", current_content, timestamp),
                                        timestamp,
                                        is_favorite: false,
                                        tags: vec![],
                                    };
                                    
                                    // Add the new entry to the beginning of the history
                                    history.insert(0, new_item);
                                    
                                    // Limit history to 50 items
                                    if history.len() > 50 {
                                        history.truncate(50);
                                    }
                                    
                                    store.set("history", serde_json::to_value(history).map_err(|e| e.to_string()).unwrap());
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
                                let mut history: Vec<ClipboardItem> = store.get("history").and_then(|v| serde_json::from_value(v).ok()).unwrap_or_else(Vec::new);
                                
                                // Add timestamp to the image entry
                                let timestamp = match std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH) {
                                        Ok(duration) => duration.as_secs(),
                                        Err(_) => 0, // fallback to 0 if timestamp fails
                                    };
                                
                                let image_entry = ClipboardItem {
                                    content: format!("[Image] {}x{}|{}|{}", image.width(), image.height(), file_path, timestamp),
                                    timestamp,
                                    is_favorite: false,
                                    tags: vec![],
                                };
                                history.insert(0, image_entry);
                                
                                // Limit history to 50 items
                                if history.len() > 50 {
                                    history.truncate(50);
                                }
                                
                                store.set("history", serde_json::to_value(history).map_err(|e| e.to_string()).unwrap());
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
            search_clipboard_history,
            toggle_favorite,
            load_favorites,
            cleanup_old_items,
            export_history,
            import_history,
            add_tag_to_item,
            remove_tag_from_item,
            get_all_tags,
            get_clipboard_statistics,
            save_clipboard_image_data,
            copy_image_from_file_to_clipboard,
            get_image_thumbnail
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}