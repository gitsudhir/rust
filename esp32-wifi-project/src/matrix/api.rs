use esp_idf_svc::http::server::*;
use esp_idf_svc::io::Write;
use serde_json;
use super::{MatrixHealthResponse, MatrixStatusResponse, PatternRequest, ApiResponse, get_matrix_driver};

pub fn register_matrix_api_handlers(server: &mut EspHttpServer, _ip_address: String) -> Result<(), Box<dyn std::error::Error>> {
    // Health check endpoint
    server.fn_handler("/api/matrix/health", Method::Options, |req| {
        match req.into_response(200, Some("OK"), &[
            ("Access-Control-Allow-Origin", "*"),
            ("Access-Control-Allow-Methods", "GET, OPTIONS"),
            ("Access-Control-Allow-Headers", "Content-Type"),
            ("Content-Length", "0")
        ]) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    })?;
    
    server.fn_handler("/api/matrix/health", Method::Get, |req| {
        let response = if get_matrix_driver().is_some() {
            MatrixHealthResponse { status: "available".to_string() }
        } else {
            MatrixHealthResponse { status: "unavailable".to_string() }
        };
        
        let json_response = serde_json::to_string(&response).unwrap_or_else(|_| r#"{"status": "unavailable"}"#.to_string());
        
        match req.into_response(200, Some("OK"), &[
            ("Content-Type", "application/json"),
            ("Access-Control-Allow-Origin", "*"),
            ("Access-Control-Allow-Methods", "GET, OPTIONS"),
            ("Access-Control-Allow-Headers", "Content-Type")
        ]) {
            Ok(mut response) => {
                let _ = response.write_all(json_response.as_bytes());
                Ok(())
            }
            Err(e) => Err(e)
        }
    })?;
    
    // Matrix status endpoint
    server.fn_handler("/api/matrix/status", Method::Options, |req| {
        match req.into_response(200, Some("OK"), &[
            ("Access-Control-Allow-Origin", "*"),
            ("Access-Control-Allow-Methods", "GET, OPTIONS"),
            ("Access-Control-Allow-Headers", "Content-Type"),
            ("Content-Length", "0")
        ]) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    })?;
    
    server.fn_handler("/api/matrix/status", Method::Get, |req| {
        let response = if let Some(max7219_mutex) = get_matrix_driver() {
            if let Some(mutex) = max7219_mutex.get() {
                if let Ok(guard) = mutex.lock() {
                    if guard.is_some() {
                        MatrixStatusResponse { 
                            status: "connected".to_string(), 
                            r#type: "MAX7219_8x8".to_string() 
                        }
                    } else {
                        MatrixStatusResponse { 
                            status: "disconnected".to_string(), 
                            r#type: "MAX7219_8x8".to_string() 
                        }
                    }
                } else {
                    MatrixStatusResponse { 
                        status: "unavailable".to_string(), 
                        r#type: "MAX7219_8x8".to_string() 
                    }
                }
            } else {
                MatrixStatusResponse { 
                    status: "unavailable".to_string(), 
                    r#type: "MAX7219_8x8".to_string() 
                }
            }
        } else {
            MatrixStatusResponse { 
                status: "unavailable".to_string(), 
                r#type: "MAX7219_8x8".to_string() 
            }
        };
        
        let json_response = serde_json::to_string(&response).unwrap_or_else(|_| r#"{"status": "unavailable", "type": "MAX7219_8x8"}"#.to_string());
        
        match req.into_response(200, Some("OK"), &[
            ("Content-Type", "application/json"),
            ("Access-Control-Allow-Origin", "*"),
            ("Access-Control-Allow-Methods", "GET, OPTIONS"),
            ("Access-Control-Allow-Headers", "Content-Type")
        ]) {
            Ok(mut response) => {
                let _ = response.write_all(json_response.as_bytes());
                Ok(())
            }
            Err(e) => Err(e)
        }
    })?;
    
    // Display pattern endpoint
    server.fn_handler("/api/matrix/pattern", Method::Options, |req| {
        match req.into_response(200, Some("OK"), &[
            ("Access-Control-Allow-Origin", "*"),
            ("Access-Control-Allow-Methods", "POST, OPTIONS"),
            ("Access-Control-Allow-Headers", "Content-Type"),
            ("Content-Length", "0")
        ]) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    })?;
    
    server.fn_handler("/api/matrix/pattern", Method::Post, |mut req| {
        let mut buffer = [0u8; 2048];
        let len = match req.read(&mut buffer) {
            Ok(len) => len,
            Err(e) => {
                let error_response = ApiResponse {
                    status: "error".to_string(),
                    message: Some(format!("Failed to read request body: {}", e)),
                };
                let json_response = serde_json::to_string(&error_response).unwrap_or_default();
                
                match req.into_response(400, Some("Bad Request"), &[
                    ("Content-Type", "application/json"),
                    ("Access-Control-Allow-Origin", "*"),
                    ("Access-Control-Allow-Methods", "POST, OPTIONS"),
                    ("Access-Control-Allow-Headers", "Content-Type")
                ]) {
                    Ok(mut response) => {
                        let _ = response.write_all(json_response.as_bytes());
                        return Ok(());
                    }
                    Err(e) => return Err(e)
                }
            }
        };
        
        let body = match std::str::from_utf8(&buffer[..len]) {
            Ok(body) => body,
            Err(e) => {
                let error_response = ApiResponse {
                    status: "error".to_string(),
                    message: Some(format!("Invalid UTF-8 in request body: {}", e)),
                };
                let json_response = serde_json::to_string(&error_response).unwrap_or_default();
                
                match req.into_response(400, Some("Bad Request"), &[
                    ("Content-Type", "application/json"),
                    ("Access-Control-Allow-Origin", "*"),
                    ("Access-Control-Allow-Methods", "POST, OPTIONS"),
                    ("Access-Control-Allow-Headers", "Content-Type")
                ]) {
                    Ok(mut response) => {
                        let _ = response.write_all(json_response.as_bytes());
                        return Ok(());
                    }
                    Err(e) => return Err(e)
                }
            }
        };
        
        let pattern_request: PatternRequest = match serde_json::from_str(body) {
            Ok(request) => request,
            Err(e) => {
                let error_response = ApiResponse {
                    status: "error".to_string(),
                    message: Some(format!("Invalid JSON: {}", e)),
                };
                let json_response = serde_json::to_string(&error_response).unwrap_or_default();
                
                match req.into_response(400, Some("Bad Request"), &[
                    ("Content-Type", "application/json"),
                    ("Access-Control-Allow-Origin", "*"),
                    ("Access-Control-Allow-Methods", "POST, OPTIONS"),
                    ("Access-Control-Allow-Headers", "Content-Type")
                ]) {
                    Ok(mut response) => {
                        let _ = response.write_all(json_response.as_bytes());
                        return Ok(());
                    }
                    Err(e) => return Err(e)
                }
            }
        };
        
        // Try to display the pattern on the matrix
        let result = if let Some(max7219_mutex) = get_matrix_driver() {
            if let Some(mutex) = max7219_mutex.get() {
                if let Ok(mut guard) = mutex.lock() {
                    if let Some(ref mut max7219) = *guard {
                        match max7219.display_pattern(&pattern_request.pattern) {
                            Ok(_) => Some(ApiResponse {
                                status: "success".to_string(),
                                message: None,
                            }),
                            Err(e) => Some(ApiResponse {
                                status: "error".to_string(),
                                message: Some(format!("Failed to display pattern: {}", e)),
                            })
                        }
                    } else {
                        Some(ApiResponse {
                            status: "error".to_string(),
                            message: Some("MAX7219 matrix not connected".to_string()),
                        })
                    }
                } else {
                    Some(ApiResponse {
                        status: "error".to_string(),
                        message: Some("Failed to acquire matrix driver lock".to_string()),
                    })
                }
            } else {
                Some(ApiResponse {
                    status: "error".to_string(),
                    message: Some("MAX7219 driver not initialized".to_string()),
                })
            }
        } else {
            Some(ApiResponse {
                status: "error".to_string(),
                message: Some("MAX7219 driver not initialized".to_string()),
            })
        };
        
        if let Some(response) = result {
            let json_response = serde_json::to_string(&response).unwrap_or_default();
            let status_code = if response.status == "success" { 200 } else { 500 };
            
            match req.into_response(status_code, Some(if status_code == 200 { "OK" } else { "Internal Server Error" }), &[
                ("Content-Type", "application/json"),
                ("Access-Control-Allow-Origin", "*"),
                ("Access-Control-Allow-Methods", "POST, OPTIONS"),
                ("Access-Control-Allow-Headers", "Content-Type")
            ]) {
                Ok(mut response) => {
                    let _ = response.write_all(json_response.as_bytes());
                    Ok(())
                }
                Err(e) => Err(e)
            }
        } else {
            let error_response = ApiResponse {
                status: "error".to_string(),
                message: Some("Unknown error".to_string()),
            };
            let json_response = serde_json::to_string(&error_response).unwrap_or_default();
            
            match req.into_response(500, Some("Internal Server Error"), &[
                ("Content-Type", "application/json"),
                ("Access-Control-Allow-Origin", "*"),
                ("Access-Control-Allow-Methods", "POST, OPTIONS"),
                ("Access-Control-Allow-Headers", "Content-Type")
            ]) {
                Ok(mut response) => {
                    let _ = response.write_all(json_response.as_bytes());
                    Ok(())
                }
                Err(e) => Err(e)
            }
        }
    })?;
    
    // Clear matrix endpoint
    server.fn_handler("/api/matrix/clear", Method::Options, |req| {
        match req.into_response(200, Some("OK"), &[
            ("Access-Control-Allow-Origin", "*"),
            ("Access-Control-Allow-Methods", "POST, OPTIONS"),
            ("Access-Control-Allow-Headers", "Content-Type"),
            ("Content-Length", "0")
        ]) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    })?;
    
    server.fn_handler("/api/matrix/clear", Method::Post, |req| {
        let result = if let Some(max7219_mutex) = get_matrix_driver() {
            if let Some(mutex) = max7219_mutex.get() {
                if let Ok(mut guard) = mutex.lock() {
                    if let Some(ref mut max7219) = *guard {
                        match max7219.clear() {
                            Ok(_) => Some(ApiResponse {
                                status: "success".to_string(),
                                message: None,
                            }),
                            Err(e) => Some(ApiResponse {
                                status: "error".to_string(),
                                message: Some(format!("Failed to clear matrix: {}", e)),
                            })
                        }
                    } else {
                        Some(ApiResponse {
                            status: "error".to_string(),
                            message: Some("MAX7219 matrix not connected".to_string()),
                        })
                    }
                } else {
                    Some(ApiResponse {
                        status: "error".to_string(),
                        message: Some("Failed to acquire matrix driver lock".to_string()),
                    })
                }
            } else {
                Some(ApiResponse {
                    status: "error".to_string(),
                    message: Some("MAX7219 driver not initialized".to_string()),
                })
            }
        } else {
            Some(ApiResponse {
                status: "error".to_string(),
                message: Some("MAX7219 driver not initialized".to_string()),
            })
        };
        
        if let Some(response) = result {
            let json_response = serde_json::to_string(&response).unwrap_or_default();
            let status_code = if response.status == "success" { 200 } else { 500 };
            
            match req.into_response(status_code, Some(if status_code == 200 { "OK" } else { "Internal Server Error" }), &[
                ("Content-Type", "application/json"),
                ("Access-Control-Allow-Origin", "*"),
                ("Access-Control-Allow-Methods", "POST, OPTIONS"),
                ("Access-Control-Allow-Headers", "Content-Type")
            ]) {
                Ok(mut response) => {
                    let _ = response.write_all(json_response.as_bytes());
                    Ok(())
                }
                Err(e) => Err(e)
            }
        } else {
            let error_response = ApiResponse {
                status: "error".to_string(),
                message: Some("Unknown error".to_string()),
            };
            let json_response = serde_json::to_string(&error_response).unwrap_or_default();
            
            match req.into_response(500, Some("Internal Server Error"), &[
                ("Content-Type", "application/json"),
                ("Access-Control-Allow-Origin", "*"),
                ("Access-Control-Allow-Methods", "POST, OPTIONS"),
                ("Access-Control-Allow-Headers", "Content-Type")
            ]) {
                Ok(mut response) => {
                    let _ = response.write_all(json_response.as_bytes());
                    Ok(())
                }
                Err(e) => Err(e)
            }
        }
    })?;
    
    Ok(())
}