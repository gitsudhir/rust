# ESP32 WiFi Project with LCD Display Documentation

## Overview
This project demonstrates how to connect an ESP32 to a WiFi network using Rust and the esp-idf-svc crate, while displaying connection status and IP address information on a 1602 LCD display via I2C interface.

## Features
- WiFi connection to a configured network
- HTTP web server with HTML interface and JSON API endpoints
- Real-time status display on 1602 LCD module
- Robust error handling and connection retry mechanisms
- CORS support for cross-origin API requests

## Hardware Requirements
- ESP32 development board
- 1602 LCD display with I2C backpack (PCF8574)
- USB cable for flashing and monitoring
- Jumper wires for LCD connections

## Software Requirements
- Rust (via [rustup](https://rustup.rs/))
- ESP-IDF development environment
- ESP32 development tools

## Hardware Setup

### LCD Connection
Connect the 1602 LCD display to the ESP32 using the following pin configuration:

| LCD Pin | ESP32 Pin | Function |
|---------|-----------|----------|
| VCC     | 3.3V/5V   | Power (check your LCD module specifications) |
| GND     | GND       | Ground |
| SDA     | GPIO 21   | I2C Data |
| SCL     | GPIO 22   | I2C Clock |

## Installation

### 1. Install ESP-IDF
Follow the official Espressif guide for installing ESP-IDF:
[ESP-IDF Get Started Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/)

### 2. Install Rust for ESP32
```bash
# Install rustup if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install the ESP Rust toolchain
curl -L https://github.com/esp-rs/espup/releases/latest/download/espup-x86_64-unknown-linux-gnu -o espup
chmod +x espup
./espup install
```

### 3. Configure WiFi Credentials
Before building the project, configure your WiFi credentials:

1. Open `src/main.rs`
2. Find the WiFi configuration section (lines 45-48)
3. Update the SSID and password with your network credentials:

```rust
let wifi_configuration = WifiConfiguration::Client(ClientConfiguration {
    ssid: "YourWiFiSSID".try_into().unwrap(), // Replace with your WiFi SSID
    password: "YourWiFiPassword".try_into().unwrap(), // Replace with your WiFi password
    auth_method: esp_idf_svc::wifi::AuthMethod::WPA2Personal, // Authentication method
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

## LCD Display Functionality

### Status Messages
The LCD display shows the following status messages during operation:

1. **"WiFi Connecting..."** - Initial status when the application starts
2. **"WiFi Started"** - When WiFi driver is successfully initialized
3. **"WiFi Connected"** - When successfully connected to the WiFi network
4. **"WiFi Failed"** - If WiFi connection fails
5. **IP Address** - The assigned IP address when connection is successful
6. **"No IP Address"** - If IP assignment fails
7. **"IP Error"** - If there's an error retrieving IP information

### Display Layout
```
Line 1: Status message or "IP Address:"
Line 2: IP address or status message
```

## Web Interface

Once your ESP32 is running and connected to WiFi, you can access the web interface:

1. Connect your device (phone, laptop, etc.) to the same WiFi network
2. Open a web browser
3. Navigate to `http://[ESP32_IP_ADDRESS]` (e.g., http://192.168.1.23)

### Available Endpoints

- **Main Page**: `/` - Shows the web interface with device information
- **API Status**: `/api/status` - Returns JSON with device status
- **CORS Support**: All endpoints include proper CORS headers for cross-origin requests

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

See `Cargo.toml` for the complete list of dependencies.

## Application Logic

The main application (`src/main.rs`) performs the following:

1. Initializes ESP-IDF system patches
2. Sets up the logging system
3. Initializes I2C interface for LCD communication
4. Initializes LCD display
5. Initializes peripherals, NVS, event loop, and timer service
6. Creates a WiFi driver
7. Configures WiFi with the provided SSID and password
8. Starts the WiFi driver
9. Connects to the WiFi network with retry mechanism
10. Waits for connection and displays IP information on LCD
11. Starts the HTTP web server
12. Maintains the connection and server in a loop

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

### LCD Display Issues
1. **No display output**: Check wiring connections, especially VCC, GND, SDA, and SCL
2. **Garbled text**: Verify I2C address (default is 0x27) and wiring
3. **Partial display**: Check power supply stability and voltage requirements

### WiFi Connection Issues
1. Check that your SSID and password are correct
2. Ensure your WiFi network is accessible (2.4GHz band recommended)
3. Check the serial output for error messages
4. Verify your ESP32 board is properly connected
5. Try restarting your ESP32 and router

### Web Server Access Issues
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

## LCD Driver Implementation Details

The LCD driver is implemented using the HD44780 controller protocol with a PCF8574 I2C backpack. Key features include:

- 4-bit mode communication
- Support for 16x2 character displays
- Backlight control
- Proper timing delays for reliable operation
- Error handling for I2C communication

The driver implements all necessary commands for initializing and controlling the LCD display, including clearing the display, setting cursor position, and writing text.