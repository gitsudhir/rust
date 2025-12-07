use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn create_video(
    app: tauri::AppHandle,
    image_path: String,
    audio_url: String, // Can be a URL or a local file path
    save_path: String, // The path where the user wants to save the video
) -> Result<String, String> {
    #[cfg(not(target_os = "android"))]
    {
        // Initialize FFmpeg
        ffmpeg_next::init().map_err(|e| format!("Failed to initialize FFmpeg: {}", e))?;
        
        // Download audio if it's a URL
        let audio_path = if audio_url.starts_with("http") {
            // Download the audio file
            let audio_res = reqwest::get(&audio_url).await.map_err(|e| e.to_string())?;
            let audio_content = audio_res.bytes().await.map_err(|e| e.to_string())?;
            
            // Save to a temporary file
            let temp_dir = app.path().temp_dir().expect("Failed to get temp dir");
            let audio_ext = std::path::Path::new(&audio_url)
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("mp3");
            let audio_file_path = temp_dir.join(format!("downloaded_audio.{}", audio_ext));
            std::fs::write(&audio_file_path, audio_content).map_err(|e| e.to_string())?;
            audio_file_path.to_string_lossy().to_string()
        } else {
            audio_url
        };
        
        // Create video from image and audio
        let temp_output_path = create_video_from_image_and_audio(&image_path, &audio_path)
            .map_err(|e| format!("Failed to create video: {}", e))?;
        
        // Copy the temporary file to the user-specified location
        std::fs::copy(&temp_output_path, &save_path).map_err(|e| format!("Failed to save video: {}", e))?;
        
        // Clean up the temporary file
        let _ = std::fs::remove_file(&temp_output_path);
        
        Ok(save_path)
    }
    
    #[cfg(target_os = "android")]
    {
        // Return an error for Android as FFmpeg is not available
        Err("Video creation is not supported on Android".to_string())
    }
}

#[cfg(not(target_os = "android"))]
fn create_video_from_image_and_audio(image_path: &str, audio_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    use ffmpeg_next as ffmpeg;
    
    // Initialize FFmpeg
    ffmpeg::init().map_err(|e| format!("Failed to initialize FFmpeg: {}", e))?;
    
    // Create a temporary output path in the system temp directory
    let temp_dir = std::env::temp_dir();
    let output_filename = format!("output_{}.mp4", chrono::Utc::now().timestamp());
    let output_path = temp_dir.join(output_filename);
    let output_path_str = output_path.to_string_lossy().to_string();
    
    // Define possible FFmpeg paths
    let ffmpeg_paths = [
        "/opt/homebrew/bin/ffmpeg", // Homebrew on Apple Silicon
        "/usr/local/bin/ffmpeg",    // Homebrew on Intel Mac
        "/usr/bin/ffmpeg",          // System FFmpeg
        "ffmpeg",                   // Default PATH
    ];
    
    // Find the first available FFmpeg binary
    let ffmpeg_binary = ffmpeg_paths.iter().find(|&path| {
        std::path::Path::new(path).exists() || (path == &"ffmpeg" && std::process::Command::new("which").arg("ffmpeg").output().map_or(false, |o| o.status.success()))
    }).ok_or("FFmpeg binary not found in any expected location")?;
    
    // Use FFmpeg command-line approach for simplicity and reliability
    let output = std::process::Command::new(ffmpeg_binary)
        .args([
            "-y", // Overwrite output file
            "-loop", "1", // Loop the image
            "-i", image_path, // Input image
            "-i", audio_path, // Input audio
            "-c:v", "libx264", // Video codec
            "-tune", "stillimage", // Optimize for still images
            "-c:a", "aac", // Audio codec
            "-b:a", "192k", // Audio bitrate
            "-pix_fmt", "yuv420p", // Pixel format
            "-shortest", // Finish when the shortest stream ends
            "-vf", "scale=1920:1080:force_original_aspect_ratio=decrease,pad=1920:1080:(ow-iw)/2:(oh-ih)/2", // Scale and pad to 1920x1080
            &output_path_str, // Output file
        ])
        .output()
        .map_err(|e| format!("Failed to execute FFmpeg command '{}': {}", ffmpeg_binary, e))?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("FFmpeg command failed: {}", error_msg).into());
    }
    
    Ok(output_path_str)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, create_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}