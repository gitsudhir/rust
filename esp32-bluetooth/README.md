# ESP32 Bluetooth Project

This project demonstrates how to use Bluetooth functionality on an ESP32 using Rust and the esp-idf-svc crate.

## Project Overview

This project shows how to:
- Initialize the ESP32 system with Bluetooth support
- Configure Bluetooth components
- Implement Bluetooth services
- Handle Bluetooth connections

## Features (Planned)

- **Bluetooth Classic**: Support for Bluetooth Classic protocols
- **BLE (Bluetooth Low Energy)**: Support for BLE services and characteristics
- **GATT Server**: Implementation of GATT services
- **Bluetooth Serial**: Serial communication over Bluetooth
- **Device Discovery**: Scan for nearby Bluetooth devices
- **Connection Management**: Handle multiple Bluetooth connections

## Prerequisites

Before you begin, ensure you have the following installed:
- Rust (via [rustup](https://rustup.rs/))
- ESP-IDF development environment
- ESP32 development board with Bluetooth capability
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

## Project Structure

```
├── src/
│   └── main.rs          # Main application entry point
├── Cargo.toml           # Rust package configuration
├── build.rs             # Build script for ESP-IDF integration
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
espflash flash target/xtensa-esp32-espidf/release/esp32-bluetooth-project /dev/ttyUSB0

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

## Bluetooth Configuration

The project is currently in development. You'll need to implement specific Bluetooth functionality based on your requirements:

1. **BLE GATT Server**: For creating custom BLE services
2. **Bluetooth Serial**: For serial communication over Bluetooth
3. **Device Discovery**: For scanning and connecting to other Bluetooth devices

## Dependencies

This project uses the following key dependencies:

- [`esp-idf-svc`](https://crates.io/crates/esp-idf-svc): High-level services for ESP-IDF
- [`esp-idf-hal`](https://crates.io/crates/esp-idf-hal): Hardware abstraction layer for ESP-IDF
- [`embedded-svc`](https://crates.io/crates/embedded-svc): Common embedded service traits
- [`log`](https://crates.io/crates/log): Logging facade for Rust
- [`embuild`](https://crates.io/crates/embuild): Build utilities for embedded systems

See `Cargo.toml` for the complete list of dependencies.

## Application Logic

The main application (`src/main.rs`) currently performs the following:

1. Initializes ESP-IDF system patches
2. Sets up the logging system
3. Initializes peripherals
4. Sets up placeholder for Bluetooth initialization
5. Runs a main loop with periodic status messages

## Bluetooth Implementation Guide

To implement specific Bluetooth functionality, you'll need to:

1. **Enable Bluetooth in menuconfig**:
   ```bash
   cargo pio run menuconfig
   ```
   Navigate to `Component config` → `Bluetooth` and enable the required options.

2. **Add Bluetooth initialization code** in `src/main.rs`:
   ```rust
   // Example BLE initialization (to be implemented)
   let ble_device = BleDevice::take();
   // Configure and start BLE services
   ```

3. **Implement specific Bluetooth services** based on your needs:
   - GATT server for custom services
   - BLE client for connecting to other devices
   - Bluetooth Classic for serial communication

## BLE Service Example

This project now includes a complete BLE service example with the following features:

1. **Custom BLE Service**: Created with a unique UUID
2. **Three Characteristics**:
   - Static characteristic (read-only)
   - Notifying characteristic (sends periodic updates)
   - Writable characteristic (accepts write requests)
3. **Connection Management**: Handles client connections and disconnections
4. **Advertising**: Broadcasts the service for discovery
5. **Notifications**: Sends periodic updates to subscribed clients

### Service and Characteristic UUIDs

- Service UUID: `9b574847-f706-436c-bed7-fc01eb0965c1`
- Static Characteristic UUID: `681285a6-247f-48c6-80ad-68c3dce18585`
- Notifying Characteristic UUID: `681285a6-247f-48c6-80ad-68c3dce18586`
- Writable Characteristic UUID: `681285a6-247f-48c6-80ad-68c3dce18587`

### Testing the BLE Service

To test the BLE service:

1. Flash the application to your ESP32 device
2. Use a BLE scanner app on your phone (like nRF Connect)
3. Look for a device named "ESP32-BLE-Server"
4. Connect to the device
5. Explore the custom service and interact with the characteristics

## Troubleshooting

Common issues and solutions:

1. **Bluetooth not initializing**:
   - Check that Bluetooth is enabled in menuconfig
   - Verify your ESP32 module has Bluetooth capability
   - Ensure proper power supply to the ESP32

2. **Connection issues**:
   - Check antenna connections
   - Verify Bluetooth permissions on connected devices
   - Ensure proper pairing procedures

3. **Build errors**:
   - Make sure ESP-IDF is properly installed and sourced
   - Check that the correct Rust toolchain is being used
   - Verify all dependencies are up to date

## Resources

- [ESP-IDF Bluetooth Documentation](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/bluetooth/index.html)
- [Rust on ESP-IDF Book](https://esp-rs.github.io/book/)
- [esp-idf-svc Crate Documentation](https://docs.rs/esp-idf-svc)
- [ESP32 Bluetooth Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-guides/bluetooth.html)