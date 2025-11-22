# ESP32 Buzzer Happy Birthday Player

This project uses an ESP32 microcontroller with Rust to play the "Happy Birthday" melody on a buzzer.

## Features

- Plays the complete "Happy Birthday" melody using PWM tones
- Uses ESP-IDF Rust bindings for hardware control
- Clean, reusable melody player implementation
- Easy to modify for other songs

## Hardware Requirements

- ESP32 development board
- Buzzer (active or passive)
- Jumper wires
- Breadboard (optional)

## Wiring Diagram

| ESP32 Pin | Buzzer Pin |
|-----------|------------|
| GPIO 15   | Signal     |
| 3.3V      | +          |
| GND       | -          |

## Setup and Installation

1. Install the Rust ESP toolchain:
   ```bash
   curl -LO https://github.com/esp-rs/espup/releases/latest/download/espup-installer.sh
   chmod +x espup-installer.sh
   ./espup-installer.sh
   ```

2. Install cargo-generate (if not already installed):
   ```bash
   cargo install cargo-generate
   ```

3. Create a new project using the ESP-IDF template:
   ```bash
   cargo generate --git https://github.com/esp-rs/esp-idf-template cargo
   ```
   When prompted:
   - Choose your chip: **esp32**
   - Name project: **buzzer_happy_birthday**
   - Continue with defaults

4. Replace the contents of `src/main.rs` with the code from this project

5. Add `anyhow = "1.0"` to the `[dependencies]` section in `Cargo.toml`

## Building and Running

To build and flash the project to your ESP32:

```bash
cargo run
```

This will compile the code, flash it to your ESP32, and start a serial monitor.

## Code Structure

- `main.rs`: Contains the main application code with the melody player
- `Cargo.toml`: Project dependencies and configuration
- `build.rs`: Build script for ESP-IDF integration

## Customization

### Adding New Songs

To add new melodies, create a new constant array similar to `MELODY`:

```rust
const NEW_SONG: &[(u32, u64)] = &[
    (NOTE_C4, 400), (NOTE_D4, 400), (NOTE_E4, 800),
    // Add more notes...
];
```

Then modify the main loop to play your new song:

```rust
for (freq, duration) in NEW_SONG {
    play_tone(&mut timer, &mut ledc, *freq, *duration)?;
}
```

### Changing the Buzzer Pin

To use a different GPIO pin for the buzzer, modify this line in `main()`:

```rust
let pin = peripherals.pins.gpio15; // Change gpio15 to your desired pin
```

## Notes

- The code supports both active and passive buzzers
- For passive buzzers, PWM generates the tones
- For active buzzers, the frequency changes will still work but may be less noticeable
- The melody plays in a continuous loop
- Adjust note durations by changing the millisecond values in the melody arrays

## Troubleshooting

If you encounter issues:

1. Ensure all dependencies are correctly installed
2. Check your wiring connections
3. Verify you've selected the correct ESP32 chip in the template
4. Make sure your ESP32 is properly connected to your computer

## License

This project is licensed under the MIT License - see the LICENSE file for details.