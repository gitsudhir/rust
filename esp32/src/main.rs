use esp_idf_hal::gpio::{Gpio2, PinDriver};
use esp_idf_hal::prelude::*;
use esp_idf_svc::log::EspLogger;
use std::{thread, time::Duration};
use esp_idf_sys::ble::*;
use esp_idf_sys::esp::*;

fn main() {
    // Initialize ESP-IDF system patches
    esp_idf_svc::sys::link_patches();

    // Initialize the logger
    EspLogger::initialize_default();
    log::info!("Starting LED Blink Program with Bluetooth!");

    // Initialize GPIO for LED blink
    let mut led_pin = PinDriver::output(unsafe { Gpio2::new() }).unwrap();

    // Initialize BLE
    unsafe {
        esp_bt_controller_init();
        esp_bt_controller_enable(ESP_BT_MODE_BTDM);
        esp_bluedroid_init();
        esp_bluedroid_enable();
    }

    // Set up BLE advertisement data
    let adv_data = BleAdvData {
        set_scan_rsp: false,
        include_name: true,
        include_txpower: true,
        min_interval: 0x100,
        max_interval: 0x200,
        appearance: 0,
        manufacturer_data: None,
    };

    unsafe {
        // Start BLE advertisement
        esp_ble_gap_start_advertising(&adv_data);
    }

    log::info!("BLE Advertisement Started!");

    // Main loop for blinking the LED
    loop {
        led_pin.set_high().unwrap();
        log::info!("LED ON");
        thread::sleep(Duration::from_secs(1));

        led_pin.set_low().unwrap();
        log::info!("LED OFF");
        thread::sleep(Duration::from_secs(1));
    }
}
