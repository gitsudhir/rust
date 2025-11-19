use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::sys::link_patches;
use esp_idf_svc::wifi::{Configuration as WifiConfiguration, ClientConfiguration, EspWifi};
use esp_idf_svc::nvs::*;
use esp_idf_svc::eventloop::*;
use esp_idf_svc::timer::*;
use esp_idf_svc::http::server::*;
use esp_idf_svc::ipv4::IpInfo;
use esp_idf_svc::hal::gpio::{PinDriver, Output};
use std::time::Duration;
use esp_idf_svc::io::Write;

use log::*;
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

// MAX7219 registers
const REG_DECODE_MODE: u8 = 0x09;
const REG_INTENSITY: u8 = 0x0A;
const REG_SCAN_LIMIT: u8 = 0x0B;
const REG_SHUTDOWN: u8 = 0x0C;
const REG_DISPLAY_TEST: u8 = 0x0F;

// Global static for the MAX7219 driver
static MAX7219_DRIVER: OnceCell<Mutex<Option<Max7219MatrixDriver>>> = OnceCell::new();

struct Max7219MatrixDriver {
    // Using PhantomData to handle the lifetime and type parameters
    din_pin: PinDriver<'static, esp_idf_svc::hal::gpio::Gpio23, Output>,
    clk_pin: PinDriver<'static, esp_idf_svc::hal::gpio::Gpio18, Output>,
    cs_pin: PinDriver<'static, esp_idf_svc::hal::gpio::Gpio5, Output>,
}

impl Max7219MatrixDriver {
    fn new(
        din_pin: PinDriver<'static, esp_idf_svc::hal::gpio::Gpio23, Output>,
        clk_pin: PinDriver<'static, esp_idf_svc::hal::gpio::Gpio18, Output>,
        cs_pin: PinDriver<'static, esp_idf_svc::hal::gpio::Gpio5, Output>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut driver = Self {
            din_pin,
            clk_pin,
            cs_pin,
        };
        
        // Initialize pins
        driver.cs_pin.set_high()?;
        driver.clk_pin.set_low()?;
        driver.din_pin.set_low()?;
        
        Ok(driver)
    }
    
    fn send_bit(&mut self, bit: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.din_pin.set_level(bit.into())?;
        self.clk_pin.set_high()?;
        // Small delay for timing
        esp_idf_svc::hal::delay::Ets::delay_us(1);
        self.clk_pin.set_low()?;
        esp_idf_svc::hal::delay::Ets::delay_us(1);
        Ok(())
    }
    
    fn send_byte(&mut self, data: u8) -> Result<(), Box<dyn std::error::Error>> {
        for i in 0..8 {
            let bit = (data >> (7 - i)) & 1 == 1;
            self.send_bit(bit)?;
        }
        Ok(())
    }
    
    fn send_command(&mut self, reg: u8, data: u8) -> Result<(), Box<dyn std::error::Error>> {
        self.cs_pin.set_low()?;
        self.send_byte(reg)?;
        self.send_byte(data)?;
        self.cs_pin.set_high()?;
        Ok(())
    }
    
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Turn off display test mode
        self.send_command(REG_DISPLAY_TEST, 0)?;
        
        // Set decode mode to no decode (matrix mode)
        self.send_command(REG_DECODE_MODE, 0)?;
        
        // Set scan limit to 8 rows
        self.send_command(REG_SCAN_LIMIT, 7)?;
        
        // Set intensity to medium brightness
        self.send_command(REG_INTENSITY, 0x07)?;
        
        // Clear all rows
        for row in 1..=8 {
            self.send_command(row, 0)?;
        }
        
        // Turn on display
        self.send_command(REG_SHUTDOWN, 1)?;
        
        Ok(())
    }
    
    fn display_row(&mut self, row: u8, data: u8) -> Result<(), Box<dyn std::error::Error>> {
        if row >= 1 && row <= 8 {
            self.send_command(row, data)?;
        }
        Ok(())
    }
    
    fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for row in 1..=8 {
            self.send_command(row, 0)?;
        }
        Ok(())
    }
    
    fn display_pattern(&mut self, pattern: &[[u8; 8]; 8]) -> Result<(), Box<dyn std::error::Error>> {
        for (row_index, row_data) in pattern.iter().enumerate() {
            let mut byte_data = 0u8;
            for (col_index, &bit) in row_data.iter().enumerate() {
                if bit != 0 {
                    // Reverse the bit order to correct mirroring issue
                    byte_data |= 1 << (7 - col_index);
                }
            }
            self.display_row((row_index + 1) as u8, byte_data)?;
        }
        Ok(())
    }
    
    fn get_char_pattern(ch: char) -> u8 {
        match ch {
            'H' => 0b10111010,
            'E' => 0b10011110,
            'L' => 0b10010010,
            'O' => 0b00111100,
            'W' => 0b10111010, // Same as H
            'R' => 0b00111110,
            'D' => 0b10111100,
            '!' => 0b00001000,
            '0' => 0b00111100,
            '1' => 0b00011000,
            '2' => 0b01101110,
            '3' => 0b11001110,
            '4' => 0b10011010,
            '5' => 0b11010110,
            '6' => 0b11110110,
            '7' => 0b00001110,
            '8' => 0b11111110,
            '9' => 0b11011110,
            _ => 0b00000000, // Space/default
        }
    }
    
    fn display_text(&mut self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        // For simplicity, display first 4 characters
        let chars: Vec<char> = text.chars().take(4).collect();
        
        for (i, &ch) in chars.iter().enumerate() {
            let pattern = Self::get_char_pattern(ch);
            // Reverse bit order for each pattern to correct mirroring
            let reversed_pattern = ((pattern & 0x01) << 7) |
                                 ((pattern & 0x02) << 5) |
                                 ((pattern & 0x04) << 3) |
                                 ((pattern & 0x08) << 1) |
                                 ((pattern & 0x10) >> 1) |
                                 ((pattern & 0x20) >> 3) |
                                 ((pattern & 0x40) >> 5) |
                                 ((pattern & 0x80) >> 7);
            self.display_row((i + 1) as u8, reversed_pattern)?;
        }
        
        // Clear remaining rows
        for i in chars.len()..8 {
            self.display_row((i + 1) as u8, 0)?;
        }
        
        Ok(())
    }
    
    fn display_ip_address(&mut self, ip_address: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Extract last octet
        if let Some(last_octet) = ip_address.split('.').last() {
            if let Ok(number) = last_octet.parse::<u32>() {
                self.display_number(number)
            } else {
                self.display_text("ERR")
            }
        } else {
            self.display_text("ERR")
        }
    }
    
    fn display_number(&mut self, number: u32) -> Result<(), Box<dyn std::error::Error>> {
        let num_str = number.to_string();
        self.display_text(&num_str)
    }
}

#[derive(Serialize)]
struct MatrixHealthResponse {
    status: String,
}

#[derive(Serialize)]
struct MatrixStatusResponse {
    status: String,
    r#type: String,
}

#[derive(Deserialize)]
struct TextRequest {
    text: String,
}

#[derive(Deserialize)]
struct PatternRequest {
    pattern: [[u8; 8]; 8],
}

#[derive(Serialize)]
struct ApiResponse {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

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
    let mut optional_max7219 = {
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
                                let _ = driver.display_text("INIT");
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
    if let Some(driver) = optional_max7219.take() {
        let _ = MAX7219_DRIVER.set(Mutex::new(Some(driver)));
    } else {
        let _ = MAX7219_DRIVER.set(Mutex::new(None));
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
    if let Some(ref max7219_mutex) = MAX7219_DRIVER.get() {
        if let Ok(mut guard) = max7219_mutex.lock() {
            if let Some(ref mut max7219) = *guard {
                // If we have an IP, display it, otherwise display "NOIP"
                if ip_address != "0.0.0.0" {
                    let _ = max7219.display_ip_address(&ip_address);
                } else {
                    let _ = max7219.display_text("NOIP");
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
                <li><span class="api-endpoint">POST /api/matrix/text</span> - Display text</li>
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
                <li><span class="api-endpoint">POST /api/matrix/text</span> - Display text</li>
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
                let _ = response.write_all(&html);
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
        let status = format!(r#"{{"status": "running", "ip": "{}"}}"#, ip_address);
        
        match req.into_response(200, Some("OK"), &[
            ("Content-Type", "application/json"),
            ("Access-Control-Allow-Origin", "*"),
            ("Access-Control-Allow-Methods", "GET, OPTIONS"),
            ("Access-Control-Allow-Headers", "Content-Type")
        ]) {
            Ok(mut response) => {
                let _ = response.write_all(status.as_bytes());
                Ok(())
            }
            Err(e) => Err(e)
        }
    })?;
    
    // Matrix API endpoints
    
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
        let response = if MAX7219_DRIVER.get().is_some() {
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
        let response = if let Some(max7219_mutex) = MAX7219_DRIVER.get() {
            if let Ok(guard) = max7219_mutex.lock() {
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
    
    // Display text endpoint
    server.fn_handler("/api/matrix/text", Method::Options, |req| {
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
    
    server.fn_handler("/api/matrix/text", Method::Post, |mut req| {
        let mut buffer = [0u8; 1024];
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
        
        let text_request: TextRequest = match serde_json::from_str(body) {
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
        
        // Try to display the text on the matrix
        let result = if let Some(max7219_mutex) = MAX7219_DRIVER.get() {
            if let Ok(mut guard) = max7219_mutex.lock() {
                if let Some(ref mut max7219) = *guard {
                    match max7219.display_text(&text_request.text) {
                        Ok(_) => Some(ApiResponse {
                            status: "success".to_string(),
                            message: None,
                        }),
                        Err(e) => Some(ApiResponse {
                            status: "error".to_string(),
                            message: Some(format!("Failed to display text: {}", e)),
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
        let result = if let Some(max7219_mutex) = MAX7219_DRIVER.get() {
            if let Ok(mut guard) = max7219_mutex.lock() {
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
        let result = if let Some(max7219_mutex) = MAX7219_DRIVER.get() {
            if let Ok(mut guard) = max7219_mutex.lock() {
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
    
    // Keep the server alive by returning it
    Ok(server)
}