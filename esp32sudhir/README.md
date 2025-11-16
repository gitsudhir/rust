# ESP32 Rust Project with TM1637 7-Segment Display

This is a template project for developing ESP32 applications using Rust and the [esp-idf](https://github.com/espressif/esp-idf) framework. It includes support for a 4-digit 7-segment TM1637 display to visualize a counter.

## Project Overview

This project demonstrates a basic ESP32 application written in Rust that:
- Initializes the ESP-IDF system
- Sets up logging facilities
- Connects to a TM1637 4-digit 7-segment display
- Runs a simple counter that increments every second and displays the value on both the console and the 7-segment display

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

## Hardware Setup

To use the TM1637 4-digit 7-segment display, connect it to your ESP32 as follows:
- VCC → 3.3V
- GND → GND
- CLK → GPIO4 (configurable in code)
- DIO → GPIO5 (configurable in code)

Note: You can change the pin assignments in `src/main.rs` according to your wiring.

## Project Structure

```
├── src/
│   └── main.rs          # Main application entry point
├── docs/
│   └── TM1637_DISPLAY.md # Detailed documentation for TM1637 implementation
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
3. Configures GPIO pins for the TM1637 display
4. Initializes the TM1637 display driver
5. Sets maximum brightness for the display
6. Prints "Hello, world!" to the console
7. Enters an infinite loop that:
   - Increments a counter
   - Displays the counter value on both the console and 7-segment display
   - Sleeps for 1 second between iterations

## TM1637 Implementation

The TM1637 driver is implemented directly in the code using bit-banging techniques:
- Custom `OutputPin` trait for abstracting GPIO pin operations
- Functions for TM1637 protocol communication (`tm1637_start`, `tm1637_stop`, `tm1637_write_byte`)
- `display_number` function to show a 4-digit number on the display
- 7-segment digit encoding for numbers 0-9

For detailed information about the TM1637 implementation, see [TM1637 Display Documentation](docs/TM1637_DISPLAY.md).

## Configuration

The project can be configured using:

- `sdkconfig.defaults`: Default ESP-IDF configuration options
- `Cargo.toml`: Rust package and dependency configuration
- `rust-toolchain.toml`: Rust toolchain specification
- Pin assignments in `src/main.rs`: GPIO pins for TM1637 CLK and DIO signals

## Customization

To use different GPIO pins for the TM1637 display:
1. Modify the pin assignments in `src/main.rs`:
   ```rust
   let mut clk = PinDriver::output(peripherals.pins.gpio4).unwrap();  // Change to your CLK pin
   let mut dio = PinDriver::output(peripherals.pins.gpio5).unwrap();  // Change to your DIO pin
   ```

To adjust display brightness:
1. Modify the brightness setting in the `display_number` function:
   ```rust
   // Change the brightness value (0-7) in this line:
   tm1637_write_byte(clk, dio, 0x88 | 0x07); // 0x88 = display on, 0x07 = max brightness
   ```

## Troubleshooting

If the display is not showing values:
1. Check all wiring connections
2. Verify the GPIO pin assignments in the code match your wiring
3. Ensure the display is receiving proper power (3.3V)
4. Confirm that the TM1637 protocol implementation is working correctly

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Resources

- [ESP-IDF Documentation](https://docs.espressif.com/projects/esp-idf/)
- [Rust on ESP-IDF Book](https://esp-rs.github.io/book/)
- [esp-idf-svc Crate Documentation](https://docs.rs/esp-idf-svc)
- [TM1637 Datasheet](https://www.mcielectronics.cl/ftp_content/otros/TM1637.pdf)