use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::sys::link_patches;
use esp_idf_svc::wifi::{Configuration as WifiConfiguration, ClientConfiguration, EspWifi};
use esp_idf_svc::nvs::*;
use esp_idf_svc::eventloop::*;
use esp_idf_svc::timer::*;
use esp_idf_svc::http::server::*;
use esp_idf_svc::ipv4::IpInfo;
use std::time::Duration;
use esp_idf_svc::io::Write;

use log::*;

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
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration).unwrap();

    info!("Starting WiFi...");

    wifi.start().unwrap();

    info!("Connecting to WiFi...");

    // Try to connect with a timeout
    let connect_start = std::time::Instant::now();
    let connect_timeout = Duration::from_secs(30); // 30 second timeout
    
    wifi.connect().unwrap();

    // Wait for connection with timeout
    while !wifi.is_connected().unwrap() {
        info!("Waiting for WiFi connection...");
        std::thread::sleep(Duration::from_millis(1000));
        
        if connect_start.elapsed() > connect_timeout {
            error!("WiFi connection timeout after 30 seconds");
            break;
        }
    }

    if wifi.is_connected().unwrap() {
        info!("Connected to WiFi!");
    } else {
        error!("Failed to connect to WiFi within timeout period");
        // Continue anyway to start the HTTP server for debugging
    }

    // Wait for IP assignment (with timeout)
    let mut ip_info: Option<IpInfo> = None;
    let ip_start = std::time::Instant::now();
    let ip_timeout = Duration::from_secs(30);
    
    while ip_info.is_none() || ip_info.as_ref().unwrap().ip.is_unspecified() {
        match wifi.sta_netif().get_ip_info() {
            Ok(info) => {
                if !info.ip.is_unspecified() {
                    ip_info = Some(info);
                }
            }
            Err(e) => {
                warn!("Failed to get IP info: {:?}", e);
            }
        }
        
        if ip_info.is_none() || ip_info.as_ref().unwrap().ip.is_unspecified() {
            info!("Waiting for IP assignment...");
            std::thread::sleep(Duration::from_millis(1000));
        }
        
        if ip_start.elapsed() > ip_timeout {
            error!("IP assignment timeout after 30 seconds");
            break;
        }
    }

    let ip_address = if let Some(ip_info) = ip_info {
        info!("IP Address: {:?}", ip_info.ip);
        info!("Subnet Mask: {:?}", ip_info.subnet);
        info!("Gateway: {:?}", ip_info.subnet.gateway);
        ip_info.ip.to_string()
    } else {
        error!("Failed to get IP address");
        "0.0.0.0".to_string()
    };
    
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
    
    // Main page handler
    server.fn_handler("/", Method::Get, move |req| {
        let html = format!(r#"
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
</html>"#, ip_address_main, ip_address_main).into_bytes();
        
        match req.into_response(200, Some("OK"), &[("Content-Type", "text/html")]) {
            Ok(mut response) => {
                let _ = response.write_all(&html);
                Ok(())
            }
            Err(e) => Err(e)
        }
    })?;
    
    // API endpoint to get status
    server.fn_handler("/api/status", Method::Get, move |req| {
        let status = format!(r#"{{"status": "running", "ip": "{}"}}"#, ip_address);
        
        match req.into_response(200, Some("OK"), &[("Content-Type", "application/json")]) {
            Ok(mut response) => {
                let _ = response.write_all(status.as_bytes());
                Ok(())
            }
            Err(e) => Err(e)
        }
    })?;
    
    // Keep the server alive by returning it
    Ok(server)
}