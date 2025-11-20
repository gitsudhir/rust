//! Configuration module for the ESP32 ultrasonic sensor project.
//! 
//! This module provides configuration options for the API endpoint
//! and other settings that can be customized via environment variables
//! at compile time.

/// Get the API endpoint for sending ultrasonic distance data.
/// 
/// This can be configured at compile time using the ULTRASONIC_API_ENDPOINT
/// environment variable. If not set, it defaults to "http://sudhirkumar.in/api/ultrasonic".
/// 
/// # Example
/// 
/// ```bash
/// ULTRASONIC_API_ENDPOINT="http://your-api.com/ultrasonic" cargo build --release
/// ```
pub fn get_api_endpoint() -> &'static str {
    option_env!("ULTRASONIC_API_ENDPOINT").unwrap_or("http://sudhirkumar.in/api/ultrasonic")
}

/// Get the API timeout in seconds.
/// 
/// This can be configured at compile time using the ULTRASONIC_API_TIMEOUT
/// environment variable. If not set, it defaults to 10 seconds.
/// 
/// # Example
/// 
/// ```bash
/// ULTRASONIC_API_TIMEOUT=15 cargo build --release
/// ```
pub fn get_api_timeout() -> u64 {
    option_env!("ULTRASONIC_API_TIMEOUT")
        .and_then(|s| s.parse().ok())
        .unwrap_or(10)
}

/// Get the maximum number of retries for API requests.
/// 
/// This can be configured at compile time using the ULTRASONIC_API_MAX_RETRIES
/// environment variable. If not set, it defaults to 3 retries.
/// 
/// # Example
/// 
/// ```bash
/// ULTRASONIC_API_MAX_RETRIES=5 cargo build --release
/// ```
pub fn get_api_max_retries() -> u32 {
    option_env!("ULTRASONIC_API_MAX_RETRIES")
        .and_then(|s| s.parse().ok())
        .unwrap_or(3)
}