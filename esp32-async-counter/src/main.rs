//! Async ESP32 Counter with TM1637 Display and Web Server

use embassy_time::{Duration, Timer};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::wifi::{AuthMethod, ClientConfiguration, Configuration, EspWifi};
use esp_idf_svc::http::server::{Configuration as HttpConfiguration, EspHttpServer};
use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Import our modules
mod display;

use display::TM1637Display;

// WiFi credentials - UPDATE THESE TO YOUR NETWORK
const WIFI_SSID: &str = "YOUR_WIFI_SSID";
const WIFI_PASS: &str = "YOUR_WIFI_PASSWORD";

// Shared state for the application
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppState {
    counter: i32,
}

type SharedState = Arc<Mutex<NoopRawMutex, AppState>>;

#[embassy_executor::task]
async fn counter_task(state: SharedState, mut display: TM1637Display<'static, 'static>) {
    loop {
        // Increment counter
        let new_value = {
            let mut app_state = state.lock().await;
            app_state.counter += 1;
            app_state.counter
        };
        
        // Update display
        display.display_number(new_value as u16);
        
        // Log the counter value
        log::info!("Counter: {}", new_value);
        
        // Wait for 1 second
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn web_task(state: SharedState, ip_address: std::net::Ipv4Addr) {
    // Start the web server
    let mut server = EspHttpServer::new(&HttpConfiguration::default()).unwrap();
    
    // Main page endpoint
    server.fn_handler("/", esp_idf_svc::http::Method::Get, move |request| {
        let html = r#"<!DOCTYPE html>
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
</html>"#;
        
        let mut response = request.into_response(200, Some("OK"), &[("Content-Type", "text/html")]).unwrap();
        response.write(html.as_bytes()).unwrap();
        Ok::<(), anyhow::Error>(())
    }).unwrap();
    
    // Counter endpoint
    let counter_state = state.clone();
    server.fn_handler("/counter", esp_idf_svc::http::Method::Get, move |request| {
        let app_state = counter_state.lock();
        let counter_value = app_state.counter;
        let json_response = format!("{{\"counter\": {}}}", app_state.counter);
        
        let mut response = request.into_response(200, Some("OK"), &[("Content-Type", "application/json")]).unwrap();
        response.write(json_response.as_bytes()).unwrap();
        Ok::<(), anyhow::Error>(())
    }).unwrap();
    
    // Reset endpoint
    let reset_state = state.clone();
    server.fn_handler("/reset", esp_idf_svc::http::Method::Post, move |mut request| {
        // Read the request body
        let mut buffer = [0u8; 1024];
        let len = request.read(&mut buffer).unwrap();
        let body = std::str::from_utf8(&buffer[..len]).unwrap();
        
        // Parse JSON
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
            if let Some(counter_value) = json.get("counter").and_then(|v| v.as_i64()) {
                // Update counter
                {
                    let mut app_state = reset_state.lock();
                    app_state.counter = counter_value as i32;
                    app_state.counter = counter_value as i32;
                }
                
                // Send response
                let json_response = format!("{{\"counter\": {}}}", counter_value);
                let mut response = request.into_response(200, Some("OK"), &[("Content-Type", "application/json")]).unwrap();
                response.write(json_response.as_bytes()).unwrap();
                return Ok::<(), anyhow::Error>(());
            }
        }
        
        // Send error response
        let mut response = request.into_response(400, Some("Bad Request"), &[("Content-Type", "application/json")]).unwrap();
        response.write(b"{\"error\": \"Invalid JSON or missing counter field\"}").unwrap();
        Ok::<(), anyhow::Error>(())
    }).unwrap();
    
    log::info!("Web server started at http://{}", ip_address);
    println!("Web server started at http://{}", ip_address);
    
    // Keep the server running
    loop {
        Timer::after(Duration::from_secs(3600)).await;
    }
}

fn main() -> anyhow::Result<()> {
    // Initialize ESP-IDF
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // Initialize peripherals
    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;

    // Configure GPIO pins for TM1637
    // CLK connected to GPIO22, DIO connected to GPIO21
    let clk = PinDriver::output(peripherals.pins.gpio22)?;
    let dio = PinDriver::output(peripherals.pins.gpio21)?;

    // Create TM1637 display instance
    let display = TM1637Display::new(clk, dio);

    // Initialize shared state
    let state = Arc::new(Mutex::new(AppState {
        counter: 0,
    }));
    
    // Display initial value
    let mut init_display = display;
    init_display.display_number(0);

    // Connect to Wi-Fi
    let wifi = connect_wifi(
        WIFI_SSID,
        WIFI_PASS,
        peripherals.modem,
        sysloop,
    )?;

    // Get and display IP address
    let ip_info = wifi.sta_netif().get_ip_info()?;
    log::info!("Connected to Wi-Fi with IP address: {}", ip_info.ip);
    println!("Connected to Wi-Fi with IP address: {}", ip_info.ip);

    // Create an executor
    // For now, we'll just loop indefinitely
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn connect_wifi(
    ssid: &str,
    pass: &str,
    modem: esp_idf_svc::hal::modem::Modem,
    sysloop: EspSystemEventLoop,
) -> anyhow::Result<EspWifi<'static>> {
    let mut auth_method = AuthMethod::WPA2Personal;
    if ssid.is_empty() {
        anyhow::bail!("Missing WiFi name")
    }
    if pass.is_empty() {
        auth_method = AuthMethod::None;
        log::info!("Wifi password is empty");
    }

    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;

    esp_wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;

    esp_wifi.start()?;

    esp_wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.try_into().unwrap(),
        password: pass.try_into().unwrap(),
        auth_method,
        ..Default::default()
    }))?;

    log::info!("Wifi started, connecting...");

    esp_wifi.connect()?;

    log::info!("Wifi connected");

    // Wait for IP address
    while !esp_wifi.is_up()? {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(esp_wifi)
}