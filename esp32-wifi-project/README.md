# ESP32 WiFi Web Server Project

This project demonstrates how to connect an ESP32 to a WiFi network using Rust and the esp-idf-svc crate, and run a web server to serve HTML content and API endpoints.

## Project Overview

This project shows how to:
- Initialize the ESP32 system
- Configure and connect to a WiFi network
- Obtain IP address information
- Run an embedded HTTP web server
- Serve HTML web pages and JSON APIs
- Maintain a WiFi connection

## Features

- **WiFi Connection**: Automatically connects to your configured WiFi network
- **Web Server**: Built-in HTTP server running on port 80
- **Web Interface**: Responsive HTML interface with device information
- **API Endpoints**: JSON API for programmatic access
- **Error Handling**: Robust connection handling with timeouts
- **Logging**: Detailed logging for debugging and monitoring

## Prerequisites

Before you begin, ensure you have the following installed:
- Rust (via [rustup](https://rustup.rs/))
- ESP-IDF development environment
- ESP32 development board
- USB cable for flashing and monitoring

### Installing ESP-IDF

Follow the official Espressif guide for installing ESP-IDF:
- [ESP-IDF Get Started Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/)

### Installing Rust for ESP32

```bash
# Install rustup if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install the ESP Rust toolchain
curl -L https://github.com/esp-rs/espup/releases/latest/download/espup-x86_64-unknown-linux-gnu -o espup
chmod +x espup
./espup install
```

## Configuration

Before building the project, you need to configure your WiFi credentials:

1. Open `src/main.rs`
2. Find the WiFi configuration section
3. Update the SSID and password with your network credentials

```rust
let wifi_configuration = WifiConfiguration::Client(ClientConfiguration {
    ssid: "Airtel_sudh_3277".try_into().unwrap(), // Replace with your WiFi SSID
    password: "Air@14803".try_into().unwrap(), // Replace with your WiFi password
    ..Default::default()
});
```

## Project Structure

```
├── src/
│   └── main.rs          # Main application entry point
├── Cargo.toml           # Rust package configuration
├── build.rs             # Build script for ESP-IDF integration
├── sdkconfig.defaults    # ESP-IDF configuration defaults
├── .cargo/
│   └── config.toml      # Cargo configuration for ESP32 target
└── target/              # Build output directory
```

## Building the Project

To build the project for ESP32:

```bash
# Export ESP-IDF environment (adjust path as needed)
export IDF_PATH=/path/to/esp-idf
. $IDF_PATH/export.sh

# Build the project
cargo build

# For release builds (recommended for ESP32):
cargo build --release
```

## Flashing to ESP32

After building, flash the application to your ESP32 device:

```bash
# Using espflash
espflash flash target/xtensa-esp32-espidf/release/esp32-wifi-project /dev/ttyUSB0

# Or using cargo-espflash
cargo espflash flash --release --target xtensa-esp32-espidf --chip esp32
```

## Monitoring Serial Output

To view the serial output from your ESP32:

```bash
# Using espflash
espflash monitor /dev/ttyUSB0

# Or using cargo-espflash
cargo espflash monitor

# Or using screen
screen /dev/ttyUSB0 115200
```

## Web Interface

Once your ESP32 is running and connected to WiFi, you can access the web interface:

1. Connect your device (phone, laptop, etc.) to the same WiFi network
2. Open a web browser
3. Navigate to `http://[ESP32_IP_ADDRESS]` (e.g., http://192.168.1.23)

### Available Endpoints

- **Main Page**: `/` - Shows the web interface with device information
- **API Status**: `/api/status` - Returns JSON with device status

Example API response:
```json
{
  "status": "running",
  "ip": "192.168.1.23"
}
```

## Dependencies

This project uses the following key dependencies:

- [`esp-idf-svc`](https://crates.io/crates/esp-idf-svc): High-level services for ESP-IDF
- [`esp-idf-hal`](https://crates.io/crates/esp-idf-hal): Hardware abstraction layer for ESP-IDF
- [`embedded-svc`](https://crates.io/crates/embedded-svc): Common embedded service traits
- [`log`](https://crates.io/crates/log): Logging facade for Rust
- [`embuild`](https://crates.io/crates/embuild): Build utilities for embedded systems

See `Cargo.toml` for the complete list of dependencies.

## Application Logic

The main application (`src/main.rs`) performs the following:

1. Initializes ESP-IDF system patches
2. Sets up the logging system
3. Initializes peripherals, NVS, event loop, and timer service
4. Creates a WiFi driver
5. Configures WiFi with the provided SSID and password
6. Starts the WiFi driver
7. Connects to the WiFi network
8. Waits for connection and displays IP information
9. Starts the HTTP web server
10. Maintains the connection and server in a loop

## Network Access

### From the Same Network (Recommended)
✅ **Works**: Connect your device to the same WiFi network as your ESP32
✅ **URL**: `http://[ESP32_IP_ADDRESS]` (e.g., http://192.168.1.23)

### From External Networks
❌ **Does NOT work**: Mobile data, different WiFi networks, etc.

To access from external networks, you would need:
- Port forwarding on your router
- A tunneling service (ngrok, Cloudflare Tunnel, etc.)
- Public IP address or domain name

## Troubleshooting

If you're having issues connecting to WiFi:
1. Check that your SSID and password are correct
2. Ensure your WiFi network is accessible (2.4GHz band)
3. Check the serial output for error messages
4. Verify your ESP32 board is properly connected
5. Try restarting your ESP32 and router

If you can't access the web server:
1. Ensure your device is on the same WiFi network
2. Check that you're using the correct IP address
3. Verify your router doesn't block device-to-device communication
4. Check your device's firewall settings

Common error messages and solutions:
- "WiFi connection timeout": Check credentials and network availability
- "Failed to get IP info": Router DHCP issues, try restarting router
- "Failed to open serial port": Permission issues, add user to dialout group

## Expanding the Project

Ideas for extending this project:
1. Add more API endpoints for sensor data
2. Implement WebSocket for real-time updates
3. Add GPIO control endpoints
4. Include sensor libraries for temperature, humidity, etc.
5. Add authentication for secure access
6. Implement HTTPS support
7. Add a configuration web interface

## Resources

- [ESP-IDF Documentation](https://docs.espressif.com/projects/esp-idf/)
- [Rust on ESP-IDF Book](https://esp-rs.github.io/book/)
- [esp-idf-svc Crate Documentation](https://docs.rs/esp-idf-svc)
- [ESP32 Datasheet](https://www.espressif.com/sites/default/files/documentation/esp32_datasheet_en.pdf)