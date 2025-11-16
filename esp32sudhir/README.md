# ESP32 Rust Project

This is a template project for developing ESP32 applications using Rust and the [esp-idf](https://github.com/espressif/esp-idf) framework.

## Project Overview

This project demonstrates a basic ESP32 application written in Rust that:
- Initializes the ESP-IDF system
- Sets up logging facilities
- Runs a simple counter loop that prints to the console every second

## Prerequisites

Before you begin, ensure you have the following installed:
- Rust (via [rustup](https://rustup.rs/))
- ESP-IDF development environment

### Installing ESP-IDF

Follow the official Espressif guide for installing ESP-IDF:
- [ESP-IDF Get Started Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/)

### Installing Rust for ESP32

```bash
# Install rustup if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install the required targets
rustup install nightly
rustup component add rust-src --toolchain nightly
```

## Project Structure

```
├── src/
│   └── main.rs          # Main application entry point
├── Cargo.toml           # Rust package configuration
├── Cargo.lock           # Dependency lock file
├── build.rs             # Build script for ESP-IDF integration
├── rust-toolchain.toml  # Specifies the Rust toolchain version
├── sdkconfig.defaults   # Default configuration for ESP-IDF
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
```

For release builds:

```bash
cargo build --release
```

## Flashing to ESP32

After building, flash the application to your ESP32 device:

```bash
# Using espflash
espflash flash target/xtensa-esp32-none-elf/debug/esp32sudhir /dev/ttyUSB0

# Or using cargo-espflash
cargo espflash flash --target xtensa-esp32-none-elf --chip esp32
```

## Monitoring Serial Output

To view the serial output from your ESP32:

```bash
# Using espflash
espflash monitor /dev/ttyUSB0

# Or using cargo-espflash
cargo espflash monitor
```

## Dependencies

This project uses the following key dependencies:

- [`esp-idf-svc`](https://crates.io/crates/esp-idf-svc): High-level services for ESP-IDF
- [`log`](https://crates.io/crates/log): Logging facade for Rust

See `Cargo.toml` for the complete list of dependencies.

## Application Logic

The main application (`src/main.rs`) performs the following:

1. Initializes ESP-IDF system patches
2. Sets up the logging system
3. Prints "Hello, world!" to the console
4. Enters an infinite loop that:
   - Increments a counter
   - Prints the counter value every second
   - Sleeps for 1 second between iterations

## Configuration

The project can be configured using:

- `sdkconfig.defaults`: Default ESP-IDF configuration options
- `Cargo.toml`: Rust package and dependency configuration
- `rust-toolchain.toml`: Rust toolchain specification

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Resources

- [ESP-IDF Documentation](https://docs.espressif.com/projects/esp-idf/)
- [Rust on ESP-IDF Book](https://esp-rs.github.io/book/)
- [esp-idf-svc Crate Documentation](https://docs.rs/esp-idf-svc)