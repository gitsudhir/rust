# ESP32 Rust Counter with TM1637 Display and Web UI

This is a Rust project for ESP32 that implements a counter application with a 4-digit 7-segment display and a real-time web interface.

## Features

- TM1637 4-digit 7-segment display showing a counter
- WiFi connectivity
- Real-time web UI with Server-Sent Events (SSE)
- REST API for resetting the counter
- Thread-safe counter implementation

## Hardware Requirements

- ESP32 DevKit
- TM1637 4-digit 7-segment display

## Wiring Diagram

```
ESP32        TM1637
----         ------
GPIO22   --> CLK
GPIO21   --> DIO
3.3V     --> VCC
GND      --> GND
```

## Project Structure

```
src/
├── main.rs      # Main application entry point
├── display.rs   # TM1637 display driver implementation
└── web.rs       # Web server with SSE support
```

## Setup

1. Install Rust and ESP-IDF as per the [Standard Installation](https://esp-rs.github.io/book/installation/index.html)

2. Update WiFi credentials in `src/main.rs`:
   ```rust
   const WIFI_SSID: &str = "YOUR_WIFI_SSID";
   const WIFI_PASS: &str = "YOUR_WIFI_PASSWORD";
   ```

## Build and Flash

### Using cargo-espflash

1. Install cargo-espflash:
   ```bash
   cargo install espflash
   ```

2. Build and flash:
   ```bash
   cargo espflash flash --target xtensa-esp32-none-elf --chip esp32 --monitor
   ```

### Using idf.py (if you have ESP-IDF installed)

1. Export ESP-IDF environment:
   ```bash
   export IDF_PATH=/path/to/esp-idf
   . $IDF_PATH/export.sh
   ```

2. Build:
   ```bash
   cargo build
   ```

3. Flash:
   ```bash
   espflash flash target/xtensa-esp32-none-elf/debug/esp32-counter /dev/ttyUSB0
   ```

## Web Interface

After flashing and connecting to WiFi, the ESP32 will print its IP address to the serial console. 
Open a web browser and navigate to `http://[ESP32_IP_ADDRESS]` to access the web interface.

### HTTP Endpoints

- `GET /` - Returns the main HTML page
- `GET /events` - SSE endpoint that emits counter values
- `POST /reset` - Reset the counter to a specific value

Example reset request:
```bash
curl -X POST http://[ESP32_IP_ADDRESS]/reset \
  -H "Content-Type: application/json" \
  -d '{"counter": 42}'
```

## Implementation Details

The project uses:
- `esp-idf-svc` for ESP-IDF services
- Custom TM1637 driver implementation using bit-banging
- Server-Sent Events for real-time updates to web clients
- Thread-safe counter using `Arc<Mutex<i32>>`

## About Async Implementation

While Embassy and async/await provide excellent capabilities for embedded systems, we've implemented a synchronous version that works reliably. The async version would require additional dependencies and careful handling of the critical-section crate conflicts.

If you want to implement an async version in the future, you would need to:
1. Add Embassy dependencies to Cargo.toml
2. Use `#[embassy_executor::main]` as the entry point
3. Implement async tasks for the counter and web server
4. Use Embassy's mutex for shared state management