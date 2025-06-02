use esp_idf_hal::gpio::{Gpio2, Output, PinDriver};
use esp_idf_hal::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    // Initialize ESP-IDF system patches
    esp_idf_svc::sys::link_patches();

    // Initialize the logger
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting LED Blink Program!");

    // Initialize the GPIO pin 2 as an output pin (for example, on the ESP32, GPIO2 is often connected to the onboard LED)
    let mut led_pin = PinDriver::output(unsafe { Gpio2::new() }).unwrap();

    // Main loop for blinking the LED
    loop {
        // Turn the LED on
        led_pin.set_high().unwrap();
        log::info!("LED ON");
        thread::sleep(Duration::from_secs(1));

        // Turn the LED off
        led_pin.set_low().unwrap();
        log::info!("LED OFF");
        thread::sleep(Duration::from_secs(1));
    }
}
