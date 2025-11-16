# TM1637 4-Digit 7-Segment Display Documentation

## Overview

This document provides detailed information about the TM1637 4-digit 7-segment display implementation in the ESP32 Rust project. The TM1637 is a specialized LED driver chip designed for driving 7-segment digital tubes, commonly used in applications such as digital clocks, timers, and counters.

## Hardware Specifications

### TM1637 Features
- Operating voltage: 5V (3.3V compatible)
- Communication protocol: Custom 2-wire serial interface (CLK and DIO)
- Maximum drive capability: 24 segments × 8 grids
- Built-in digital adjustment circuit
- Built-in auto-brightness adjustment circuit
- Built-in flicker elimination circuit
- Support for key scanning (16 keys × 8 grids or 8 keys × 16 grids)

### Pin Configuration
The TM1637 module typically has 4 pins:
1. **VCC**: Power supply (3.3V-5V)
2. **GND**: Ground
3. **CLK**: Clock signal line
4. **DIO**: Data input/output line

## Connection to ESP32

### Wiring Diagram
```
ESP32        TM1637 Module
----         -------------
GPIO4   -->  CLK
GPIO5   -->  DIO
3.3V    -->  VCC
GND     -->  GND
```

### Pin Assignment
The current implementation uses:
- **CLK**: GPIO4
- **DIO**: GPIO5

These pins can be changed by modifying the following lines in `src/main.rs`:
```rust
let mut clk = PinDriver::output(peripherals.pins.gpio4).unwrap();
let mut dio = PinDriver::output(peripherals.pins.gpio5).unwrap();
```

## Communication Protocol

### Basic Commands
The TM1637 uses a custom 2-wire serial protocol with the following commands:

1. **Data Command Setting (0x40)**
   - Format: `0b0100ABCD`
   - A: Read/Write mode (0=write, 1=read)
   - B: Address auto-increment mode (0=off, 1=on)
   - CD: Test mode (00=normal, 01=display, 10=normal, 11=normal)

2. **Address Command (0xC0 + address)**
   - Sets the starting address for data display
   - Address range: 0x00 to 0x0F

3. **Display Control Command (0x80 + 0x08 + brightness)**
   - Format: `0b1000ABCD`
   - A: Display on/off (0=off, 1=on)
   - BCD: Brightness level (000-111, 000=dimmest, 111=brightest)

### Communication Sequence
1. **Start Condition**: DIO goes low while CLK is high
2. **Data Transmission**: 8 bits are transmitted MSB first
3. **ACK**: After each byte, the controller pulls DIO low for acknowledgment
4. **Stop Condition**: CLK goes high, then DIO goes high

## Implementation Details

### Custom Traits
The implementation defines a custom `OutputPin` trait to abstract GPIO pin operations:

```rust
trait OutputPin {
    fn set_high(&mut self) -> Result<(), EspError>;
    fn set_low(&mut self) -> Result<(), EspError>;
}
```

This trait is implemented for `PinDriver` to provide a consistent interface for controlling the CLK and DIO pins.

### Key Functions

#### `tm1637_start()`
Initiates communication with the TM1637 by generating the start condition.

#### `tm1637_stop()`
Terminates communication with the TM1637 by generating the stop condition.

#### `tm1637_write_byte()`
Transmits a single byte to the TM1637, handling bit-banging and ACK detection.

#### `display_number()`
Displays a 4-digit number on the 7-segment display by:
1. Setting up data command
2. Sending address command
3. Transmitting digit data
4. Setting display control with brightness

#### `digit_to_segments()`
Converts a decimal digit (0-9) to its 7-segment representation.

### 7-Segment Encoding
The implementation uses the following bit mapping for 7-segment digits:
```
  a
f   b
  g
e   c
  d   dp
```

Bit positions: `dp g f e d c b a`

Examples:
- 0: 0x3F (0b00111111)
- 1: 0x06 (0b00000110)
- 2: 0x5B (0b01011011)
- 3: 0x4F (0b01001111)
- 4: 0x66 (0b01100110)
- 5: 0x6D (0b01101101)
- 6: 0x7D (0b01111101)
- 7: 0x07 (0b00000111)
- 8: 0x7F (0b01111111)
- 9: 0x6F (0b01101111)

## Usage

### Basic Operation
The display is updated every second in the main loop:
```rust
loop {
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Tick... {}", count);
    display_number(&mut clk, &mut dio, &mut delay, count as u16);
    count += 1;
}
```

### Customization
To display a different value, call `display_number()` with your desired number:
```rust
display_number(&mut clk, &mut dio, &mut delay, your_value);
```

## Troubleshooting

### Common Issues
1. **Display not showing anything**
   - Check power connections (VCC and GND)
   - Verify CLK and DIO wiring
   - Ensure correct GPIO pin assignments in code

2. **Incorrect digits displayed**
   - Check 7-segment encoding in `digit_to_segments()`
   - Verify wiring between TM1637 and 7-segment display

3. **Flickering display**
   - Increase delay between updates
   - Check for electrical noise on power lines

### Debugging Tips
1. Use a logic analyzer to verify CLK and DIO signals
2. Add debug prints to verify communication sequence
3. Test with a known working TM1637 library on another platform

## Future Enhancements

### Possible Improvements
1. **Brightness Control**: Add functions to dynamically adjust display brightness
2. **Decimal Point Support**: Extend the implementation to control decimal points
3. **Character Display**: Add support for displaying letters and special characters
4. **Error Handling**: Improve error handling for communication failures
5. **Key Scanning**: Implement key scanning functionality if needed

### Performance Optimizations
1. **DMA Support**: Use DMA for faster data transmission
2. **Interrupt-Driven Updates**: Implement interrupt-driven display updates
3. **Caching**: Cache segment data to reduce computation overhead

## References

1. TM1637 Datasheet
2. ESP-IDF Documentation
3. Rust on ESP-IDF Book
4. Embedded HAL Documentation

This implementation provides a solid foundation for using TM1637 displays with ESP32 microcontrollers in Rust projects.