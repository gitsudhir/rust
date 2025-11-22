//! Web server implementation with SSE support
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_svc::http::Method;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

// Counter type alias
pub type Counter = Arc<Mutex<i32>>;

// Client tracking for SSE
pub type Clients = Arc<Mutex<HashMap<usize, std::sync::mpsc::Sender<String>>>>;
static CLIENT_ID: AtomicUsize = AtomicUsize::new(0);

/// Start the HTTP server with all endpoints
pub fn start_web_server(counter: Counter, clients: Clients) -> anyhow::Result<EspHttpServer<'static>> {
    // Configure the HTTP server with appropriate settings
    let config = Configuration {
        stack_size: 10240,
        max_resp_headers: 10,
        ..Default::default()
    };
    
    let mut server = EspHttpServer::new(&config)?;
    
    // Main page endpoint
    server.fn_handler("/", Method::Get, move |request| {
        let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>ESP32 Counter</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        body { 
            font-family: Arial, sans-serif; 
            text-align: center; 
            margin: 0;
            padding: 20px;
            background-color: #f0f0f0;
        }
        .container {
            max-width: 600px;
            margin: 20px auto;
            background-color: white;
            padding: 30px;
            border-radius: 10px;
            box-shadow: 0 0 10px rgba(0,0,0,0.1);
        }
        h1 { 
            color: #333; 
            margin-top: 0;
            font-size: 28px;
        }
        #counter { 
            font-size: 72px; 
            color: #2196F3; 
            margin: 30px 0;
            font-weight: bold;
            font-family: 'Courier New', monospace;
        }
        #reset-form {
            margin: 20px 0;
            display: flex;
            justify-content: center;
            gap: 10px;
            flex-wrap: wrap;
        }
        #reset-value {
            padding: 12px;
            font-size: 18px;
            width: 120px;
            border: 2px solid #ddd;
            border-radius: 5px;
            text-align: center;
        }
        #reset-btn {
            padding: 12px 20px;
            font-size: 18px;
            background-color: #4CAF50;
            color: white;
            border: none;
            border-radius: 5px;
            cursor: pointer;
            min-width: 120px;
        }
        #reset-btn:hover {
            background-color: #45a049;
        }
        .status {
            padding: 12px;
            margin: 15px 0;
            border-radius: 5px;
            font-weight: bold;
        }
        .connected {
            background-color: #dff0d8;
            color: #3c763d;
        }
        .disconnected {
            background-color: #f2dede;
            color: #a94442;
        }
        .instructions {
            background-color: #e3f2fd;
            padding: 15px;
            border-radius: 5px;
            margin: 20px 0;
            text-align: left;
        }
        .instructions h3 {
            margin-top: 0;
            color: #1976d2;
        }
        .instructions ul {
            padding-left: 20px;
        }
        .instructions li {
            margin: 8px 0;
        }
        /* Mobile responsive design */
        @media (max-width: 768px) {
            body {
                padding: 10px;
            }
            .container {
                margin: 10px;
                padding: 20px;
            }
            h1 {
                font-size: 24px;
            }
            #counter {
                font-size: 48px;
            }
            #reset-value {
                width: 100px;
                font-size: 16px;
                padding: 10px;
            }
            #reset-btn {
                padding: 10px 16px;
                font-size: 16px;
                min-width: 100px;
            }
        }
        @media (max-width: 480px) {
            #counter {
                font-size: 36px;
            }
            #reset-form {
                flex-direction: column;
                align-items: center;
            }
            #reset-value, #reset-btn {
                width: 100%;
                max-width: 200px;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>ESP32 Counter</h1>
        <div id="counter">0</div>
        
        <form id="reset-form">
            <input type="number" id="reset-value" placeholder="New value" min="0">
            <button type="submit" id="reset-btn">Reset Counter</button>
        </form>
        
        <div id="status" class="disconnected">Connecting to server...</div>
        
        <div class="instructions">
            <h3>How to use:</h3>
            <ul>
                <li>The counter above updates automatically every 0.5 seconds</li>
                <li>Enter a number and click "Reset Counter" to set a new value</li>
                <li>Works on both desktop and mobile devices</li>
                <li>Connection status is shown below the counter</li>
            </ul>
        </div>
    </div>

    <script>
        // Connect to SSE endpoint
        const counterElement = document.getElementById('counter');
        const statusElement = document.getElementById('status');
        
        function connectSSE() {
            const eventSource = new EventSource('/events');
            
            eventSource.onopen = function() {
                statusElement.textContent = 'Connected to server';
                statusElement.className = 'status connected';
            };
            
            eventSource.onmessage = function(event) {
                counterElement.textContent = event.data;
            };
            
            eventSource.onerror = function() {
                statusElement.textContent = 'Connection lost, reconnecting...';
                statusElement.className = 'status disconnected';
                setTimeout(connectSSE, 5000);
            };
            
            return eventSource;
        }
        
        let eventSource = connectSSE();
        
        // Handle reset form submission
        document.getElementById('reset-form').addEventListener('submit', function(e) {
            e.preventDefault();
            const newValue = document.getElementById('reset-value').value;
            
            if (newValue !== '') {
                fetch('/reset', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({counter: parseInt(newValue)})
                })
                .then(response => {
                    if (response.ok) {
                        document.getElementById('reset-value').value = '';
                    } else {
                        alert('Failed to reset counter');
                    }
                })
                .catch(error => {
                    alert('Error: ' + error);
                });
            }
        });
    </script>
</body>
</html>"#;
        
        let mut response = request.into_response(200, Some("OK"), &[("Content-Type", "text/html")])
            .map_err(|e| anyhow::anyhow!("Failed to create response: {:?}", e))?;
        response.write(html.as_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to write response: {:?}", e))?;
        Ok::<(), anyhow::Error>(())
    })?;
    
    // SSE endpoint - clone references for closure
    let sse_counter = counter.clone();
    let sse_clients = clients.clone();
    server.fn_handler("/events", Method::Get, move |request| -> anyhow::Result<()> {
        // Set CORS headers
        let headers = &[
            ("Content-Type", "text/event-stream"),
            ("Cache-Control", "no-cache"),
            ("Connection", "keep-alive"),
            ("Access-Control-Allow-Origin", "*"),
        ];
        
        let mut response = request.into_response(200, Some("OK"), headers)
            .map_err(|e| anyhow::anyhow!("Failed to create SSE response: {:?}", e))?;
        
        // Generate unique client ID
        let client_id = CLIENT_ID.fetch_add(1, Ordering::Relaxed);
        
        // Create channel for this client
        let (sender, receiver) = std::sync::mpsc::channel::<String>();
        
        // Add client to clients map
        {
            let mut clients_map = sse_clients.lock().unwrap();
            clients_map.insert(client_id, sender);
        }
        
        // Remove client when connection closes (cleanup)
        let cleanup_clients = sse_clients.clone();
        let cleanup_id = client_id;
        
        // Send initial value
        let current_value = *sse_counter.lock().unwrap();
        let initial_message = format!("data: {}\n\n", current_value);
        if let Err(e) = response.write(initial_message.as_bytes()) {
            log::error!("Failed to send initial SSE message: {:?}", e);
            // Remove client on error
            let mut clients_map = cleanup_clients.lock().unwrap();
            clients_map.remove(&cleanup_id);
            return Ok(());
        }
        
        // Flush the response to ensure initial message is sent
        if let Err(e) = response.flush() {
            log::error!("Failed to flush initial SSE response: {:?}", e);
            // Remove client on error
            let mut clients_map = cleanup_clients.lock().unwrap();
            clients_map.remove(&cleanup_id);
            return Ok(());
        }
        
        // Keep connection alive and send updates
        loop {
            // Check if we have a message to send
            match receiver.recv_timeout(std::time::Duration::from_millis(100)) {
                Ok(message) => {
                    // Try to send the message
                    match response.write(message.as_bytes()) {
                        Ok(_) => {
                            // Try to flush the response
                            if let Err(e) = response.flush() {
                                log::error!("Failed to flush SSE response: {:?}", e);
                                // Remove client on error
                                let mut clients_map = cleanup_clients.lock().unwrap();
                                clients_map.remove(&cleanup_id);
                                break;
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to write SSE message: {:?}", e);
                            // Remove client on error
                            let mut clients_map = cleanup_clients.lock().unwrap();
                            clients_map.remove(&cleanup_id);
                            break;
                        }
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    // Timeout, continue loop
                    continue;
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    log::info!("SSE client {} disconnected", cleanup_id);
                    // Remove client on disconnect
                    let mut clients_map = cleanup_clients.lock().unwrap();
                    clients_map.remove(&cleanup_id);
                    break;
                }
            }
        }
        
        // Clean up client if not already removed
        let mut clients_map = cleanup_clients.lock().unwrap();
        clients_map.remove(&cleanup_id);
        Ok(())
    })?;
    
    // Reset endpoint
    let reset_counter = counter.clone();
    let reset_clients = clients.clone();
    server.fn_handler("/reset", Method::Post, move |mut request| -> anyhow::Result<()> {
        log::info!("Reset endpoint called");
        
        // Read the request body
        let mut buffer = [0u8; 1024];
        let len = request.read(&mut buffer)
            .map_err(|e| {
                log::error!("Failed to read request: {:?}", e);
                e
            })?;
        let body = std::str::from_utf8(&buffer[..len])
            .map_err(|e| {
                log::error!("Failed to parse request body: {:?}", e);
                e
            })?;
        
        log::info!("Reset request body: {}", body);
        
        // Parse JSON
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
            log::info!("Parsed JSON: {:?}", json);
            if let Some(counter_value) = json.get("counter").and_then(|v| v.as_i64()) {
                log::info!("Resetting counter to: {}", counter_value);
                
                // Update counter
                *reset_counter.lock().unwrap() = counter_value as i32;
                
                // Notify all SSE clients
                let message = format!("data: {}\n\n", counter_value);
                let clients_map = reset_clients.lock().unwrap();
                let mut failed_clients = Vec::new();
                
                // Send to all clients and track failures
                for (client_id, sender) in clients_map.iter() {
                    if let Err(e) = sender.send(message.clone()) {
                        log::error!("Failed to send to client {}: {:?}", client_id, e);
                        failed_clients.push(*client_id);
                    }
                }
                
                // Remove failed clients
                if !failed_clients.is_empty() {
                    let mut clients_map = reset_clients.lock().unwrap();
                    for client_id in failed_clients {
                        clients_map.remove(&client_id);
                    }
                }
                
                // Send response
                let mut response = request.into_response(200, Some("OK"), &[("Access-Control-Allow-Origin", "*")])
                    .map_err(|e| {
                        log::error!("Failed to create response: {:?}", e);
                        e
                    })?;
                response.write(b"Counter reset successfully")
                    .map_err(|e| {
                        log::error!("Failed to write response: {:?}", e);
                        e
                    })?;
                log::info!("Reset successful");
                return Ok(());
            } else {
                log::warn!("Missing counter field in JSON");
            }
        } else {
            log::warn!("Failed to parse JSON");
        }
        
        // Send error response
        let mut response = request.into_response(400, Some("Bad Request"), &[("Access-Control-Allow-Origin", "*")])
            .map_err(|e| {
                log::error!("Failed to create error response: {:?}", e);
                e
            })?;
        response.write(b"Invalid JSON or missing counter field")
            .map_err(|e| {
                log::error!("Failed to write error response: {:?}", e);
                e
            })?;
        log::info!("Reset failed - bad request");
        Ok(())
    })?;
    
    // CORS preflight handler
    server.fn_handler("/reset", Method::Options, move |request| -> anyhow::Result<()> {
        log::info!("CORS preflight request received");
        
        let headers = &[
            ("Access-Control-Allow-Origin", "*"),
            ("Access-Control-Allow-Methods", "POST, OPTIONS"),
            ("Access-Control-Allow-Headers", "Content-Type"),
            ("Content-Length", "0")
        ];
        
        let _response = request.into_response(200, Some("OK"), headers)
            .map_err(|e| {
                log::error!("Failed to create CORS response: {:?}", e);
                e
            })?;
        log::info!("CORS preflight response sent");
        Ok(())
    })?;
    
    Ok(server)
}

/// Notify all SSE clients of a counter update
pub fn notify_clients(clients: &Clients, counter: i32) {
    let message = format!("data: {}\n\n", counter);
    let clients_map = clients.lock().unwrap();
    let mut failed_clients = Vec::new();
    
    // Send to all clients and track failures
    for (client_id, sender) in clients_map.iter() {
        if let Err(e) = sender.send(message.clone()) {
            log::error!("Failed to send to client {}: {:?}", client_id, e);
            failed_clients.push(*client_id);
        }
    }
    
    // Remove failed clients
    if !failed_clients.is_empty() {
        let mut clients_map = clients.lock().unwrap();
        for client_id in failed_clients {
            clients_map.remove(&client_id);
        }
    }
}