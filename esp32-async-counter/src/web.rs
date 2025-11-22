//! Async web server implementation using picoserve
use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use picoserve::routing::get;
use picoserve::response::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Application state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub counter: i32,
}

/// Counter response
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CounterResponse {
    counter: i32,
}

/// Reset request
#[derive(Debug, Clone, Deserialize)]
struct ResetRequest {
    counter: i32,
}

/// HTML page for the counter
fn counter_html() -> &'static str {
    r#"<!DOCTYPE html>
<html>
<head>
    <title>ESP32 Async Counter</title>
    <style>
        body { 
            font-family: Arial, sans-serif; 
            text-align: center; 
            margin-top: 50px;
            background-color: #f0f0f0;
        }
        .container {
            max-width: 600px;
            margin: 0 auto;
            background-color: white;
            padding: 30px;
            border-radius: 10px;
            box-shadow: 0 0 10px rgba(0,0,0,0.1);
        }
        h1 { color: #333; }
        #counter { 
            font-size: 72px; 
            color: #2196F3; 
            margin: 30px 0;
            font-weight: bold;
        }
        #reset-form {
            margin: 20px 0;
        }
        #reset-value {
            padding: 10px;
            font-size: 18px;
            width: 100px;
            margin-right: 10px;
        }
        #reset-btn {
            padding: 10px 20px;
            font-size: 18px;
            background-color: #4CAF50;
            color: white;
            border: none;
            border-radius: 5px;
            cursor: pointer;
        }
        #reset-btn:hover {
            background-color: #45a049;
        }
        .status {
            padding: 10px;
            margin: 10px 0;
            border-radius: 5px;
        }
        .connected {
            background-color: #dff0d8;
            color: #3c763d;
        }
        .disconnected {
            background-color: #f2dede;
            color: #a94442;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>ESP32 Async Counter</h1>
        <div id="counter">0</div>
        
        <form id="reset-form">
            <input type="number" id="reset-value" placeholder="New value" min="0">
            <button type="submit" id="reset-btn">Reset Counter</button>
        </form>
        
        <div id="status" class="disconnected">Connecting to server...</div>
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
                const data = JSON.parse(event.data);
                counterElement.textContent = data.counter;
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
</html>"#
}

/// SSE endpoint for real-time updates
async fn sse_handler(
    state: Arc<Mutex<NoopRawMutex, AppState>>,
) -> Result<impl picoserve::response::Response, picoserve::response::Response> {
    use picoserve::response::IntoResponse;
    
    // For simplicity, we'll just return the current counter value
    let app_state = state.lock().await;
    let response = CounterResponse {
        counter: app_state.counter,
    };
    
    Ok(Json(response))
}

/// Reset counter endpoint
async fn reset_handler(
    state: Arc<Mutex<NoopRawMutex, AppState>>,
    Json(request): Json<ResetRequest>,
) -> Result<impl picoserve::response::Response, picoserve::response::Response> {
    use picoserve::response::IntoResponse;
    
    {
        let mut app_state = state.lock().await;
        app_state.counter = request.counter;
    }
    
    let response = CounterResponse {
        counter: request.counter,
    };
    
    Ok(Json(response))
}

/// Get current counter value
async fn counter_handler(
    state: Arc<Mutex<NoopRawMutex, AppState>>,
) -> Result<impl picoserve::response::Response, picoserve::response::Response> {
    use picoserve::response::IntoResponse;
    
    let app_state = state.lock().await;
    let response = CounterResponse {
        counter: app_state.counter,
    };
    
    Ok(Json(response))
}

/// Start the web server
pub async fn start_web_server(state: Arc<Mutex<NoopRawMutex, AppState>>) {
    let app = picoserve::Router::new()
        .route("/", get(|| async { counter_html() }))
        .route("/counter", get(move || counter_handler(state.clone())))
        .route("/reset", picoserve::routing::post(move |body| reset_handler(state.clone(), body)))
        .route("/events", get(move || sse_handler(state.clone())));

    let config = picoserve::Config::new(picoserve::Timeouts {
        start_read_request: Some(embassy_time::Duration::from_secs(5)),
        read_request: Some(embassy_time::Duration::from_secs(5)),
        write_response: Some(embassy_time::Duration::from_secs(5)),
    });

    let stack = esp_idf_svc::eth::EspEth::new(esp_idf_svc::handle::RawHandle::empty());
    let stack = stack.unwrap();
    
    let mut rx_buffer = [0u8; 1024];
    let mut tx_buffer = [0u8; 1024];
    let mut socket = esp_idf_svc::hal::socket::Socket::new(&stack, &mut rx_buffer, &mut tx_buffer);
    
    let port = 80;
    let mut tcp_server = picoserve::Server::new(&mut socket, port);
    
    loop {
        tcp_server
            .serve(&app, &config, |index| {
                log::info!("Servicing connection {index}");
            })
            .await
            .unwrap();
    }
}