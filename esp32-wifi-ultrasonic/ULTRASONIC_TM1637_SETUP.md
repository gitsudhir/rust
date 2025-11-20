# ESP32 Ultrasonic Sensor with TM1637 Display Setup Guide

## Overview

This document provides complete instructions for setting up the ESP32 ultrasonic sensor project with TM1637 display integration. It includes hardware wiring diagrams, software configuration, and troubleshooting tips.

## Hardware Components Required

1. ESP32 development board (ESP32-DevKitC or similar)
2. HC-SR04 ultrasonic sensor
3. TM1637 4-digit 7-segment display
4. Breadboard
5. Jumper wires
6. USB cable for programming and power
7. Logic level converter OR voltage divider resistors (1kΩ and 2kΩ)

## Important Voltage Considerations

### ESP32 vs HC-SR04 Voltage Compatibility

The HC-SR04 ultrasonic sensor requires 5V to operate properly, but ESP32 GPIO pins are only 3.3V tolerant. Direct connection can damage your ESP32. You must use one of these solutions:

1. **Voltage Divider** (Recommended for beginners)
2. **Logic Level Converter** (Professional solution)
3. **3.3V Compatible Sensor** (US-100, JSN-SR04T, etc.)

## Hardware Wiring Connections

### Option 1: Using Voltage Divider (Recommended)

#### HC-SR04 Ultrasonic Sensor Connections

| HC-SR04 Pin | Connection                     | Description              |
|-------------|--------------------------------|--------------------------|
| VCC         | External 5V power supply       | Power (5V)               |
| GND         | ESP32 GND                      | Ground                   |
| Trig        | ESP32 GPIO12                   | Trigger signal (3.3V OK) |
| Echo        | Voltage divider to ESP32 GPIO13| Echo signal (5V → 3.3V)  |

#### TM1637 Display Connections

| TM1637 Pin | ESP32 Pin | Description        |
|------------|-----------|--------------------|
| VCC        | 3.3V      | Power (3.3V)       |
| GND        | GND       | Ground             |
| CLK        | GPIO14    | Clock signal       |
| DIO        | GPIO15    | Data signal        |

### Voltage Divider for Echo Pin

To safely connect the 5V Echo signal to the 3.3V ESP32 GPIO13:
- Connect 1kΩ resistor between HC-SR04 Echo pin and ESP32 GPIO13
- Connect 2kΩ resistor between ESP32 GPIO13 and GND
- This creates a voltage divider that steps 5V down to approximately 3.33V

### Wiring Diagram with Voltage Divider

```
ESP32 DevKitC
┌─────────────────────────────────────────────────────────────┐
│  ┌───┐                                                    │
│  │5V │────────────────────────────────────────────┐        │
│  └───┘                                         │        │
│  ┌───┐                                        │        │
│  │GND│──┬────────────┬─────────────────────────┼─────┐  │
│  └───┘  │            │                         │     │  │
│  ┌───┐  │            │                         │     │  │
│  │12 │──┘            │                         │     │  │
│  └───┘               │                         │     │  │
│  ┌───┐               │    Voltage Divider      │     │  │
│  │13 │───────────────┼───[1kΩ]──[2kΩ]──────────┘     │  │
│  └───┘               │        │                      │  │
│  ┌───┐               │        └── To GPIO13          │  │
│  │14 │───────────────┼─────────────────────────────────────┤  │
│  └───┘               │                                    │  │
│  ┌───┐               │                                    │  │
│  │15 │───────────────┼─────────────────────────────────────┤  │
│  └───┘               │                                    │  │
└─────────────────────────────────────────────────────────────┘

External 5V Power Supply
┌─────────────────────────────┐
│  +5V  GND                  │
│  ┌─────┬─────┐             │
│  │  ●  │  ●  │             │
│  └─────┴─────┘             │
└─────────────────────────────┘
   │    │
   │    └──────────────────── GND (to ESP32)
   └───────────────────────── HC-SR04 VCC

HC-SR04 Ultrasonic Sensor
┌─────────────────────────────┐
│  VCC  GND  Trig  Echo      │
│  ┌─────┬─────┬─────┬─────┐  │
│  │  ●  │  ●  │  ●  │  ●  │  │
│  └─────┴─────┴─────┴─────┘  │
└─────────────────────────────┘
   │    │    │     │
   │    │    │     └──[1kΩ]──[2kΩ]── To GPIO13
   │    │    │              │
   │    │    │              └─────── GND
   │    │    └────────────────────── GPIO12 (Trig)
   │    └─────────────────────────── GND (to ESP32)
   └──────────────────────────────── External 5V

TM1637 Display
┌─────────────────────────────┐
│  GND  VCC  CLK  DIO        │
│  ┌─────┬─────┬─────┬─────┐  │
│  │  ●  │  ●  │  ●  │  ●  │  │
│  └─────┴─────┴─────┴─────┘  │
└─────────────────────────────┘
   │    │    │     │
   │    │    │     └────────── GPIO15 (DIO)
   │    │    └──────────────── GPIO14 (CLK)
   │    └───────────────────── 3.3V (from ESP32)
   └────────────────────────── GND (to ESP32)
```

### Option 2: Using Logic Level Converter

If you have a logic level converter:
1. Connect HC-SR04 VCC to 5V side of converter
2. Connect ESP32 GPIO pins to 3.3V side of converter
3. Connect converter's 5V to external 5V supply
4. Connect converter's 3.3V to ESP32 3.3V

### Option 3: Using 3.3V Compatible Ultrasonic Sensor

Consider using a 3.3V compatible sensor like:
- US-100 (can operate at 3.3V)
- JSN-SR04T (3.3V compatible version)
- DYP-ME007Y (3.3V compatible)

With these sensors, you can connect directly:
- VCC → ESP32 3.3V
- GND → ESP32 GND
- Trig → ESP32 GPIO12
- Echo → ESP32 GPIO13

## Software Configuration

### 1. WiFi Setup

Before building the project, configure your WiFi credentials in `src/main.rs`:

```rust
let wifi_configuration = WifiConfiguration::Client(ClientConfiguration {
    ssid: "YourWiFiSSID".try_into().unwrap(),     // Replace with your WiFi SSID
    password: "YourWiFiPassword".try_into().unwrap(), // Replace with your WiFi password
    auth_method: esp_idf_svc::wifi::AuthMethod::WPA2Personal,
    channel: None,
    ..Default::default()
});
```

### 2. API Configuration

The project supports configuring the API endpoint and other settings via environment variables at compile time:

- `ULTRASONIC_API_ENDPOINT`: API endpoint for ultrasonic data (default: http://sudhirkumar.in/api/ultrasonic)
- `ULTRASONIC_API_TIMEOUT`: Request timeout in seconds (default: 10)
- `ULTRASONIC_API_MAX_RETRIES`: Maximum retry attempts (default: 3)

Example usage:
```bash
ULTRASONIC_API_ENDPOINT="http://your-api.com/ultrasonic" cargo build --release
```

### 3. GPIO Pin Configuration

The GPIO pins are configured in `src/main.rs`:

```rust
// HC-SR04 ultrasonic sensor pins
PinDriver::output(peripherals.pins.gpio12), // Trigger pin
PinDriver::input(peripherals.pins.gpio13),  // Echo pin

// TM1637 display pins
PinDriver::output(peripherals.pins.gpio14), // CLK pin
PinDriver::output(peripherals.pins.gpio15), // DIO pin
```

You can modify these pins if needed, but make sure to update both the initialization code and the wiring accordingly.

## Building and Flashing

### Prerequisites

1. Install Rust for ESP32 development:
   ```bash
   curl -L https://github.com/esp-rs/espup/releases/latest/download/espup-x86_64-unknown-linux-gnu -o espup
   chmod +x espup
   ./espup install
   ```

2. Install ESP-IDF development environment following the official Espressif guide.

### Building the Project

```bash
# Export ESP-IDF environment (adjust path as needed)
export IDF_PATH=/path/to/esp-idf
. $IDF_PATH/export.sh

# Build the project
cargo build --release
```

### Flashing to ESP32

```bash
# Using espflash
espflash flash target/xtensa-esp32-espidf/release/esp32-wifi-ultrasonic /dev/ttyUSB0
```

Replace `/dev/ttyUSB0` with your actual serial port.

## How It Works

### 1. Ultrasonic Distance Measurement

The HC-SR04 sensor works by sending an ultrasonic pulse and measuring the time it takes for the echo to return:

1. A 10μs pulse is sent to the Trigger pin
2. The sensor sends out 8 ultrasonic pulses at 40kHz
3. The Echo pin goes HIGH for the duration of the pulse travel time
4. Distance is calculated using: `Distance = (Time / 2) × Speed of Sound`

### 2. TM1637 Display

The TM1637 is a 4-digit 7-segment display driver that communicates via a custom protocol:

1. Start condition: CLK HIGH, DIO goes LOW
2. Data transfer: 8 bits, LSB first, with CLK pulses
3. ACK: DIO pulled LOW by the display during CLK HIGH
4. Stop condition: CLK LOW, DIO goes HIGH

### 3. API Data Transmission

The system periodically:

1. Measures distance using the ultrasonic sensor
2. Displays the measurement on the TM1637 display
3. Sends the data to the configured API endpoint with:
   - Distance measurement in centimeters
   - Device IP address
   - Timestamp

## Troubleshooting

### Common Issues and Solutions

1. **Display Not Working**:
   - Check TM1637 VCC connection (should be 3.3V)
   - Verify CLK and DIO connections
   - Ensure pull-up resistors are present (internal pull-ups are used in code)

2. **Ultrasonic Sensor Not Responding**:
   - Verify 5V power connection to HC-SR04
   - Check Trigger and Echo pin connections
   - Ensure voltage divider is correctly wired
   - Verify no interference from other devices

3. **WiFi Connection Issues**:
   - Verify SSID and password in the configuration
   - Check router settings (some block device-to-device communication)
   - Ensure the ESP32 is within WiFi range

4. **API Data Not Sending**:
   - Check API endpoint configuration
   - Verify network connectivity
   - Check server logs for error messages

### Debugging Tips

1. Monitor serial output for error messages:
   ```bash
   espflash monitor /dev/ttyUSB0
   ```

2. Check if the display initializes correctly:
   - On startup, the display should show "8888"
   - After initialization, it should show distance measurements

3. Verify ultrasonic sensor functionality:
   - Place an object at a known distance
   - Check if the display shows approximately the correct value

## Safety Considerations

1. **Voltage Levels**: Never connect 5V directly to ESP32 GPIO pins
2. **Power Supply**: Ensure stable power supply to avoid brownouts
3. **Physical Installation**: Secure all connections to prevent accidental shorts
4. **Environmental**: Protect from moisture and extreme temperatures

## Maintenance

1. **Regular Cleaning**: Keep sensors clean from dust and debris
2. **Connection Checks**: Periodically verify all connections
3. **Firmware Updates**: Update firmware as needed for bug fixes and improvements
4. **Calibration**: Periodically verify accuracy against known distances

This setup provides a complete solution for measuring distance with an ultrasonic sensor, displaying the results on a 7-segment display, and sending the data to a remote API for further processing.