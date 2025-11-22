# ESP32 Multi-Client Counter with TM1637 Display

This is a Rust project for ESP32 that implements a counter application with a 4-digit 7-segment display and real-time web interface that supports multiple concurrent connections.

## Features

- ✅ TM1637 4-digit 7-segment display showing a counter
- ✅ WiFi connectivity
- ✅ Real-time web UI with Server-Sent Events (SSE)
- ✅ Multi-client support - connect from multiple devices simultaneously
- ✅ REST API for resetting the counter
- ✅ Thread-safe counter implementation
- ✅ Mobile-responsive web interface

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
├── web.rs       # Web server with SSE broadcast pattern
└── wifi.rs      # WiFi connection module
```

## Multi-Client SSE Implementation

This project implements a **broadcast-style SSE system** that allows unlimited clients to connect simultaneously:

### Key Features:
- **Broadcast Pattern**: Each client gets its own communication channel
- **Multi-Client Support**: Unlimited devices can connect at the same time
- **Automatic Cleanup**: Disconnected clients are automatically removed
- **Real-time Updates**: All clients receive updates simultaneously
- **Client Count Display**: Shows how many devices are connected

### Technical Implementation:
1. Each SSE client connection gets a unique ID and dedicated channel
2. Clients are stored in a thread-safe `Arc<Mutex<HashMap<usize, Sender<String>>>>`
3. Messages are broadcast to all connected clients
4. Failed/disconnected clients are automatically removed
5. Real-time client count updates are sent to all clients

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
- `GET /events` - SSE endpoint that emits counter values to all connected clients
- `POST /reset` - Reset the counter to a specific value (JSON payload)

Example reset request:
```bash
curl -X POST http://[ESP32_IP_ADDRESS]/reset \
  -H "Content-Type: application/json" \
  -d '{"counter": 42}'
```

## Testing Multi-Client Support

1. Open the web interface on your laptop
2. Open the web interface on your mobile device
3. Open the web interface on a second mobile device or browser tab
4. All devices should show the same counter value updating in real-time
5. The "Connected clients" display should show the total number of connections
6. Close one device's tab - the others should still work and show updated client count

## Implementation Details

The project uses:
- `esp-idf-svc` for ESP-IDF services
- Custom TM1637 driver implementation using bit-banging
- Server-Sent Events with broadcast pattern for real-time updates
- Thread-safe counter using `Arc<Mutex<i32>>`
- Multi-producer, multi-consumer channels for client communication

## Architecture

1. **Main Thread**: Updates the 7-segment display every 50ms
2. **Counter Thread**: Increments the counter every 500ms and notifies all web clients
3. **Web Server**: Handles HTTP requests and manages SSE client connections
4. **SSE Broadcast System**: Sends updates to all connected clients simultaneously

This implementation follows the recommended broadcast pattern for multi-client SSE support, ensuring that any number of devices can connect and receive real-time updates simultaneously.