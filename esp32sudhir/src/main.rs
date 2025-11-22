//! Main application entry point
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Import our modules
mod display;
mod web;
mod wifi;

use display::TM1637Display;
use web::{Counter, Clients, start_web_server, notify_clients};

// WiFi credentials - UPDATE THESE TO YOUR NETWORK
const WIFI_SSID: &str = "Airtel_sudh_3277";
const WIFI_PASS: &str = "Air@14803";

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly.
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Initialize peripherals
    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;

    // Configure GPIO pins for TM1637
    // CLK connected to GPIO22, DIO connected to GPIO21
    let clk = PinDriver::output(peripherals.pins.gpio22)?;
    let dio = PinDriver::output(peripherals.pins.gpio21)?;

    // Create TM1637 display instance
    let mut display = TM1637Display::new(clk, dio);

    // Initialize counter and clients
    let counter: Counter = Arc::new(Mutex::new(0i32));
    let clients: Clients = Arc::new(Mutex::new(std::collections::HashMap::new()));
    
    // Display initial value
    display.display_number(0);

    // Connect to Wi-Fi
    let wifi = wifi::wifi(
        WIFI_SSID,
        WIFI_PASS,
        peripherals.modem,
        sysloop,
    )?;

    // Get and display IP address
    let ip_info = wifi.sta_netif().get_ip_info()?;
    println!("Connected to Wi-Fi with IP address: {}", ip_info.ip);
    log::info!("Connected to Wi-Fi with IP address: {}", ip_info.ip);

    // Start HTTP server
    let _server = start_web_server(counter.clone(), clients.clone())?;
    
    // Keep server running
    println!("Server started. Access it at http://{}", ip_info.ip);
    
    // Counter increment thread
    let counter_for_thread = counter.clone();
    let clients_for_thread = clients.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(500)); // Faster update - 500ms instead of 1000ms
            
            // Increment counter
            let new_value = {
                let mut cnt = counter_for_thread.lock().unwrap();
                *cnt += 1;
                *cnt
            };
            
            // Notify web clients
            notify_clients(&clients_for_thread, new_value);
            
            // Log the counter value
            log::info!("Counter: {}", new_value);
        }
    });

    // Main loop - update display based on counter value and keep server alive
    let counter_for_display = counter.clone();
    // Keep the server alive by using it in the main loop
    loop {
        thread::sleep(Duration::from_millis(50)); // Much faster display updates - 50ms instead of 100ms
        
        // Get current counter value
        let current_value = *counter_for_display.lock().unwrap();
        
        // Update display
        display.display_number(current_value as u16);
        
        // The server is kept alive by being in scope
    }
}