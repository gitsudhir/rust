# Async ESP32 Counter with TM1637 Display and Web Server

This is an asynchronous Rust project for ESP32 that implements a counter application with a 4-digit 7-segment display and a real-time web interface using Embassy and picoserve.

## Features

- TM1637 4-digit 7-segment display showing a counter
- WiFi connectivity with async/await
- Real-time web UI with JSON API
- Asynchronous task management with Embassy
- Thread-safe state management with Embassy mutexes

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
├── main.rs      # Main application entry point with async tasks
├── display.rs   # TM1637 display driver implementation
└── web.rs       # Async web server with picoserve
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

## Web Interface

After flashing and connecting to WiFi, the ESP32 will print its IP address to the serial console. 
Open a web browser and navigate to `http://[ESP32_IP_ADDRESS]` to access the web interface.

### HTTP Endpoints

- `GET /` - Returns the main HTML page
- `GET /counter` - Returns current counter value as JSON
- `GET /events` - SSE endpoint that emits counter values
- `POST /reset` - Reset the counter to a specific value (JSON payload)

Example reset request:
```bash
curl -X POST http://[ESP32_IP_ADDRESS]/reset \
  -H "Content-Type: application/json" \
  -d '{"counter": 42}'
```

## Implementation Details

The project uses:
- `esp-idf-svc` for ESP-IDF services with Embassy integration
- `Embassy` as the async runtime
- `picoserve` as the async HTTP server
- Custom TM1637 driver implementation using bit-banging
- Embassy mutexes for shared state management
- Async tasks for concurrent operations

## Async Architecture

The application uses Embassy's async executor to run multiple concurrent tasks:
1. **Main task** - Initializes the system and spawns other tasks
2. **Counter task** - Increments the counter and updates the display every second
3. **Web task** - Handles HTTP requests and serves the web interface

This architecture allows the ESP32 to handle multiple operations concurrently without blocking.