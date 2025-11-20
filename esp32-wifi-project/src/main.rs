use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::sys::link_patches;
use esp_idf_svc::wifi::{Configuration as WifiConfiguration, ClientConfiguration, EspWifi};
use esp_idf_svc::nvs::*;
use esp_idf_svc::eventloop::*;
use esp_idf_svc::timer::*;
use esp_idf_svc::http::server::*;
use esp_idf_svc::ipv4::IpInfo;
use esp_idf_svc::hal::gpio::PinDriver;
use std::time::Duration;

use log::*;

// Import the matrix module
mod matrix;
use matrix::{Max7219MatrixDriver, init_matrix_driver, get_matrix_driver};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();

    info!("Starting WiFi example...");

    // Initialize peripherals
    let peripherals = Peripherals::take().unwrap();

    // Initialize NVS (Non-Volatile Storage)
    let nvs = EspDefaultNvsPartition::take().unwrap();

    // Create event loop
    let sys_loop = EspSystemEventLoop::take().unwrap();

    // Create timer service
    let _timer = EspTimerService::new().unwrap();

    // Initialize MAX7219 8x8 LED matrix driver
    let optional_max7219 = {
        info!("Initializing MAX7219 8x8 LED matrix...");
        
        match (
            PinDriver::output(peripherals.pins.gpio23),
            PinDriver::output(peripherals.pins.gpio18),
            PinDriver::output(peripherals.pins.gpio5),
        ) {
            (Ok(din_pin), Ok(clk_pin), Ok(cs_pin)) => {
                match Max7219MatrixDriver::new(din_pin, clk_pin, cs_pin) {
                    Ok(mut driver) => {
                        match driver.init() {
                            Ok(_) => {
                                info!("MAX7219 8x8 LED matrix initialized successfully");
                                // Display initial message
                                let _ = driver.display_number(1111); // INIT as numbers
                                Some(driver)
                            }
                            Err(e) => {
                                warn!("Failed to initialize MAX7219 8x8 LED matrix: {:?}", e);
                                None
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to create MAX7219 8x8 LED matrix driver: {:?}", e);
                        None
                    }
                }
            }
            _ => {
                warn!("Failed to configure GPIO pins for MAX7219 8x8 LED matrix");
                None
            }
        }
    };

    // Store the MAX7219 driver in a global static for API access
    if let Some(driver) = optional_max7219 {
        let _ = init_matrix_driver(driver);
    }

    // Create WiFi driver
    let mut wifi = EspWifi::new(
        peripherals.modem,
        sys_loop.clone(),
        Some(nvs.clone()),
    ).unwrap();

    // Configure WiFi with more robust settings
    let wifi_configuration = WifiConfiguration::Client(ClientConfiguration {
        ssid: "Airtel_sudh_3277".try_into().unwrap(), // Your WiFi SSID
        password: "Air@14803".try_into().unwrap(), // Your WiFi password
        auth_method: esp_idf_svc::wifi::AuthMethod::WPA2Personal, // Explicitly set authentication method
        channel: None, // Let the ESP32 scan for the best channel
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration).unwrap();

    info!("Starting WiFi...");

    // Scan for available networks for debugging
    info!("Scanning for available WiFi networks...");
    match wifi.scan() {
        Ok(access_points) => {
            info!("Found {} WiFi networks:", access_points.len());
            for ap in access_points.iter().take(5) { // Show first 5 networks
                info!("  - {} (channel: {}, signal_strength: {})", 
                      ap.ssid.as_str(), 
                      ap.channel, 
                      ap.signal_strength);
            }
        }
        Err(e) => {
            warn!("Failed to scan for networks: {:?}", e);
        }
    }

    // Start WiFi with error handling
    match wifi.start() {
        Ok(_) => info!("WiFi started successfully"),
        Err(e) => {
            error!("Failed to start WiFi: {:?}", e);
            return Err(Box::new(e));
        }
    }

    info!("Connecting to WiFi...");

    // Try to connect with a timeout and retry mechanism
    let connect_start = std::time::Instant::now();
    let connect_timeout = Duration::from_secs(60); // Increase timeout to 60 seconds
    let mut retry_count = 0;
    const MAX_RETRIES: u32 = 5; // Increase retries
    
    loop {
        // Attempt connection with error handling
        match wifi.connect() {
            Ok(_) => {
                info!("Connection command sent successfully");
                break;
            }
            Err(e) => {
                error!("Failed to send connection command: {:?}", e);
                retry_count += 1;
                
                if retry_count >= MAX_RETRIES {
                    error!("Max retries reached, giving up");
                    return Err(Box::new(e));
                }
                
                info!("Retrying connection... (attempt {}/{})", retry_count + 1, MAX_RETRIES);
                std::thread::sleep(Duration::from_millis(3000)); // Increase delay between retries
            }
        }
        
        // Check if we've timed out
        if connect_start.elapsed() > connect_timeout {
            error!("WiFi connection timeout after {} seconds", connect_timeout.as_secs());
            break;
        }
    }

    // Wait for connection with timeout and better status checking
    while !wifi.is_connected().unwrap_or(false) {
        info!("Waiting for WiFi connection...");
        
        std::thread::sleep(Duration::from_millis(1000));
        
        if connect_start.elapsed() > connect_timeout {
            error!("WiFi connection timeout after {} seconds", connect_timeout.as_secs());
            break;
        }
    }

    if wifi.is_connected().unwrap_or(false) {
        info!("Connected to WiFi!");
    } else {
        error!("Failed to connect to WiFi within timeout period");
        // Try to get more information about why the connection failed
        match wifi.get_configuration() {
            Ok(config) => {
                error!("Current WiFi configuration: {:?}", config);
            }
            Err(e) => {
                error!("Failed to get WiFi configuration for debugging: {:?}", e);
            }
        }
        // Continue anyway to start the HTTP server for debugging
    }

    // Wait for IP assignment (with timeout)
    let mut ip_info: Option<IpInfo> = None;
    let ip_start = std::time::Instant::now();
    let ip_timeout = Duration::from_secs(60); // Increase timeout to 60 seconds
    
    // Only try to get IP if we're connected
    if wifi.is_connected().unwrap_or(false) {
        while ip_info.is_none() || ip_info.as_ref().map_or(true, |info| info.ip.is_unspecified()) {
            match wifi.sta_netif().get_ip_info() {
                Ok(info) => {
                    if !info.ip.is_unspecified() {
                        ip_info = Some(info);
                        info!("Got valid IP address: {:?}", info.ip);
                    } else {
                        info!("Got IP info but IP is still unspecified");
                    }
                }
                Err(e) => {
                    warn!("Failed to get IP info: {:?}", e);
                }
            }
            
            if ip_info.is_none() || ip_info.as_ref().map_or(true, |info| info.ip.is_unspecified()) {
                info!("Waiting for IP assignment...");
                std::thread::sleep(Duration::from_millis(1000));
            }
            
            if ip_start.elapsed() > ip_timeout {
                error!("IP assignment timeout after {} seconds", ip_timeout.as_secs());
                break;
            }
        }
    } else {
        error!("Not attempting to get IP because WiFi is not connected");
    }

    let ip_address = if let Some(ip_info) = ip_info {
        info!("IP Address: {:?}", ip_info.ip);
        info!("Subnet Mask: {:?}", ip_info.subnet);
        info!("Gateway: {:?}", ip_info.subnet.gateway);
        ip_info.ip.to_string()
    } else {
        error!("Failed to get IP address");
        // Try one more time to get IP info before giving up
        match wifi.sta_netif().get_ip_info() {
            Ok(info) => {
                if !info.ip.is_unspecified() {
                    info!("Got IP on final attempt: {:?}", info.ip);
                    info!("IP Address: {:?}", info.ip);
                    info!("Subnet Mask: {:?}", info.subnet);
                    info!("Gateway: {:?}", info.subnet.gateway);
                    info.ip.to_string()
                } else {
                    "0.0.0.0".to_string()
                }
            }
            Err(e) => {
                error!("Final attempt to get IP failed: {:?}", e);
                "0.0.0.0".to_string()
            }
        }
    };
    
    // Display IP address on MAX7219 if available
    if let Some(max7219_mutex) = get_matrix_driver() {
        if let Some(mutex) = max7219_mutex.get() {
            if let Ok(mut guard) = mutex.lock() {
                if let Some(ref mut max7219) = *guard {
                    // If we have an IP, display it, otherwise display "NOIP"
                    if ip_address != "0.0.0.0" {
                        let _: Result<(), _> = max7219.display_ip_address(&ip_address);
                    } else {
                        let _: Result<(), _> = max7219.display_number(9999); // NOIP as numbers
                    }
                }
            }
        }
    }
    
    // Start HTTP server
    let _server = create_http_server(ip_address.clone())?;
    
    info!("HTTP server started on http://{}:80", ip_address);
    info!("Open this address in your mobile browser to see the Hello message");
    info!("Make sure your device is on the same WiFi network (Airtel_sudh_3277)");

    // Keep the program running
    loop {
        // Check WiFi connection status
        match wifi.is_connected() {
            Ok(connected) => {
                if connected {
                    info!("WiFi is still connected");
                } else {
                    info!("WiFi is disconnected");
                }
            }
            Err(e) => {
                error!("Error checking WiFi connection: {:?}", e);
            }
        }
        std::thread::sleep(Duration::from_secs(10));
    }
}

fn create_http_server(ip_address: String) -> Result<EspHttpServer<'static>, Box<dyn std::error::Error>> {
    let server_config = esp_idf_svc::http::server::Configuration::default();
    let mut server = EspHttpServer::new(&server_config)?;
    
    // Clone IP address for first handler
    let ip_address_main = ip_address.clone();
    let ip_address_status = ip_address.clone();
    
    // Main page handler
    server.fn_handler("/", Method::Get, move |req| {
        let html = if ip_address_main != "0.0.0.0" {
            format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>ESP32 Web Server</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
        body {{
            font-family: Arial, sans-serif;
            text-align: center;
            margin: 20px;
            background-color: #f0f0f0;
        }}
        .container {{
            background-color: white;
            padding: 20px;
            border-radius: 10px;
            box-shadow: 0 4px 8px rgba(0,0,0,0.1);
            max-width: 600px;
            margin: 0 auto;
        }}
        h1 {{
            color: #4CAF50;
        }}
        .info-box {{
            background-color: #e8f5e9;
            padding: 15px;
            border-radius: 5px;
            margin: 10px 0;
        }}
        .ip-address {{
            font-weight: bold;
            color: #1976d2;
        }}
        .instructions {{
            background-color: #fff3e0;
            padding: 15px;
            border-radius: 5px;
            margin: 20px 0;
            text-align: left;
        }}
        .status {{
            font-size: 1.2em;
            font-weight: bold;
            color: #4CAF50;
        }}
        .error {{
            color: #f44336;
            background-color: #ffebee;
            padding: 15px;
            border-radius: 5px;
            margin: 10px 0;
        }}
        .api-section {{
            background-color: #e3f2fd;
            padding: 15px;
            border-radius: 5px;
            margin: 20px 0;
            text-align: left;
        }}
        .api-endpoint {{
            font-family: monospace;
            background-color: #f5f5f5;
            padding: 5px;
            border-radius: 3px;
            display: inline-block;
            margin: 5px 0;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>ESP32 Web Server</h1>
        <div class="info-box">
            <p>Device IP Address: <span class="ip-address">{}</span></p>
            <p>Status: <span class="status">Running</span></p>
        </div>
        <div class="instructions">
            <h3>Access Instructions:</h3>
            <ul>
                <li>✅ Connect your mobile device to the same WiFi network (Airtel_sudh_3277)</li>
                <li>✅ Open your browser and go to: <strong>http://{}</strong></li>
                <li>❌ This will NOT work from mobile data or different networks</li>
            </ul>
        </div>
        <div class="api-section">
            <h3>Matrix API Endpoints:</h3>
            <p>Control the connected MAX7219 8x8 LED matrix:</p>
            <ul>
                <li><span class="api-endpoint">GET /api/matrix/health</span> - Check if matrix is available</li>
                <li><span class="api-endpoint">GET /api/matrix/status</span> - Get detailed status</li>
                <li><span class="api-endpoint">POST /api/matrix/pattern</span> - Display custom pattern</li>
                <li><span class="api-endpoint">POST /api/matrix/clear</span> - Clear the matrix</li>
            </ul>
        </div>
        <div class="error">
            <h3>Connection Issues?</h3>
            <p>If you can't connect:</p>
            <ol>
                <li>Verify your WiFi credentials are correct</li>
                <li>Check that you're on the same network</li>
                <li>Try restarting your ESP32</li>
                <li>Check router settings (some block device-to-device communication)</li>
            </ol>
        </div>
        <p>Your IoT device is successfully running a web server!</p>
    </div>
</body>
</html>"#, ip_address_main, ip_address_main)
        } else {
            format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>ESP32 Web Server - No Network</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
        body {{
            font-family: Arial, sans-serif;
            text-align: center;
            margin: 20px;
            background-color: #f0f0f0;
        }}
        .container {{
            background-color: white;
            padding: 20px;
            border-radius: 10px;
            box-shadow: 0 4px 8px rgba(0,0,0,0.1);
            max-width: 600px;
            margin: 0 auto;
        }}
        h1 {{
            color: #f44336;
        }}
        .info-box {{
            background-color: #ffebee;
            padding: 15px;
            border-radius: 5px;
            margin: 10px 0;
        }}
        .ip-address {{
            font-weight: bold;
            color: #f44336;
        }}
        .instructions {{
            background-color: #fff3e0;
            padding: 15px;
            border-radius: 5px;
            margin: 20px 0;
            text-align: left;
        }}
        .status {{
            font-size: 1.2em;
            font-weight: bold;
            color: #f44336;
        }}
        .error {{
            color: #f44336;
            background-color: #ffebee;
            padding: 15px;
            border-radius: 5px;
            margin: 10px 0;
        }}
        .api-section {{
            background-color: #e3f2fd;
            padding: 15px;
            border-radius: 5px;
            margin: 20px 0;
            text-align: left;
        }}
        .api-endpoint {{
            font-family: monospace;
            background-color: #f5f5f5;
            padding: 5px;
            border-radius: 3px;
            display: inline-block;
            margin: 5px 0;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>ESP32 Web Server - No Network Connection</h1>
        <div class="info-box">
            <p>Device Status: <span class="status">No Network Connection</span></p>
            <p>IP Address: <span class="ip-address">Not Assigned</span></p>
        </div>
        <div class="error">
            <h3>Network Connection Failed</h3>
            <p>The device failed to connect to the WiFi network "Airtel_sudh_3277".</p>
            <p>Check the serial logs for more detailed error information.</p>
        </div>
        <div class="instructions">
            <h3>Troubleshooting Steps:</h3>
            <ul>
                <li>Verify WiFi credentials (SSID: Airtel_sudh_3277)</li>
                <li>Check if the WiFi network is accessible</li>
                <li>Ensure the WiFi password is correct</li>
                <li>Check router settings and signal strength</li>
                <li>Try restarting the ESP32 device</li>
            </ul>
        </div>
        <div class="api-section">
            <h3>Matrix API Endpoints (Still Available):</h3>
            <p>Control the connected MAX7219 8x8 LED matrix:</p>
            <ul>
                <li><span class="api-endpoint">GET /api/matrix/health</span> - Check if matrix is available</li>
                <li><span class="api-endpoint">GET /api/matrix/status</span> - Get detailed status</li>
                <li><span class="api-endpoint">POST /api/matrix/pattern</span> - Display custom pattern</li>
                <li><span class="api-endpoint">POST /api/matrix/clear</span> - Clear the matrix</li>
            </ul>
        </div>
        <p>The HTTP server is running, but network connectivity is unavailable.</p>
    </div>
</body>
</html>"#)
        }.into_bytes();
        
        match req.into_response(200, Some("OK"), &[
            ("Content-Type", "text/html"),
            ("Access-Control-Allow-Origin", "*"),
            ("Access-Control-Allow-Methods", "GET, OPTIONS, POST"),
            ("Access-Control-Allow-Headers", "Content-Type")
        ]) {
            Ok(mut response) => {
                let _ = response.write(&html);
                Ok(())
            }
            Err(e) => Err(e)
        }
    })?;
    
    // API endpoint to get status
    // Handle preflight OPTIONS request for CORS
    server.fn_handler("/api/status", Method::Options, |req| {
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
    
    server.fn_handler("/api/status", Method::Get, move |req| {
        let status = format!(r#"{{"status": "running", "ip": "{}"}}"#, ip_address_status);
        
        match req.into_response(200, Some("OK"), &[
            ("Content-Type", "application/json"),
            ("Access-Control-Allow-Origin", "*"),
            ("Access-Control-Allow-Methods", "GET, OPTIONS"),
            ("Access-Control-Allow-Headers", "Content-Type")
        ]) {
            Ok(mut response) => {
                let _ = response.write(status.as_bytes());
                Ok(())
            }
            Err(e) => Err(e)
        }
    })?;
    
    
    // Register matrix API handlers
    matrix::api::register_matrix_api_handlers(&mut server, ip_address.clone())?;
    
    // Keep the server alive by returning it
    Ok(server)
}