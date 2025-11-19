use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::sys::link_patches;
use esp_idf_svc::timer::EspTimerService;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use std::time::Duration;
use log::*;

use esp32_nimble::{BLEDevice, NimbleProperties, uuid128, BLEAdvertisementData};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();

    info!("Starting ESP32 Bluetooth example...");

    // Initialize peripherals
    let _peripherals = Peripherals::take().unwrap();
    let _sys_loop = EspSystemEventLoop::take()?;
    let _timer = EspTimerService::new()?;

    // Initialize BLE device
    let ble_device = BLEDevice::take();
    let _ = BLEDevice::set_device_name("ESP32-BLE-Server");
    
    // Create BLE server
    let server = ble_device.get_server();
    server.on_connect(|_server, desc| {
        info!("Client connected: {:?}", desc);
    });
    
    server.on_disconnect(|desc, reason| {
        info!("Client disconnected: {:?}, reason: {:?}", desc, reason);
    });

    // Create a BLE service
    let service = server.create_service(uuid128!("9b574847-f706-436c-bed7-fc01eb0965c1"));
    
    // Create a readable characteristic with device info
    let info_characteristic = service.lock().create_characteristic(
        uuid128!("681285a6-247f-48c6-80ad-68c3dce18585"),
        NimbleProperties::READ,
    );
    info_characteristic.lock().set_value(b"ESP32 BLE Server v1.0");
    
    // Create a notifying characteristic for periodic updates
    let notify_characteristic = service.lock().create_characteristic(
        uuid128!("681285a6-247f-48c6-80ad-68c3dce18586"),
        NimbleProperties::READ | NimbleProperties::NOTIFY,
    );
    notify_characteristic.lock().set_value(b"Ready for notifications");
    
    // Create a writable characteristic for receiving data from clients
    let write_characteristic = service.lock().create_characteristic(
        uuid128!("681285a6-247f-48c6-80ad-68c3dce18587"),
        NimbleProperties::WRITE,
    );
    write_characteristic.lock().set_value(b"Send data here");
    
    // Create a readable/writable characteristic for bidirectional communication
    let rw_characteristic = service.lock().create_characteristic(
        uuid128!("681285a6-247f-48c6-80ad-68c3dce18588"),
        NimbleProperties::READ | NimbleProperties::WRITE | NimbleProperties::NOTIFY,
    );
    rw_characteristic.lock().set_value(b"Send/receive data here");
    
    // Clone characteristics for use in callbacks
    let rw_char_clone = rw_characteristic.clone();
    
    // Set up write callback for the write characteristic
    write_characteristic.lock().on_write(move |args| {
        let value = args.recv_data();
        let value_str = std::str::from_utf8(value).unwrap_or("<non-UTF8>");
        info!("Received on write characteristic: {}", value_str);
        
        // Update the RW characteristic with the received data
        rw_char_clone.lock().set_value(value);
    });
    
    // Set up write callback for the RW characteristic (simple echo)
    rw_characteristic.lock().on_write(|args| {
        let value = args.recv_data();
        let value_str = std::str::from_utf8(value).unwrap_or("<non-UTF8>");
        info!("Received on RW characteristic: {}", value_str);
        
        // Echo the data back
        let echo_msg = format!("Echo: {}", value_str);
        // We can't easily clone here, so we'll just log the echo
        info!("Echo response: {}", echo_msg);
    });
    
    // Start advertising with proper name
    let ble_advertising = ble_device.get_advertising();
    
    // Set advertisement data
    let mut advertisement_data = BLEAdvertisementData::new();
    advertisement_data.name("ESP32-BLE-Server");
    advertisement_data.add_service_uuid(uuid128!("9b574847-f706-436c-bed7-fc01eb0965c1"));
    let _ = ble_advertising.lock().set_data(&mut advertisement_data);
    
    let _ = ble_advertising.lock().start();
    
    info!("BLE server started. Advertising as 'ESP32-BLE-Server'");
    info!("Characteristics:");
    info!("  1. Info (read): 681285a6-247f-48c6-80ad-68c3dce18585");
    info!("  2. Notify: 681285a6-247f-48c6-80ad-68c3dce18586");
    info!("  3. Write: 681285a6-247f-48c6-80ad-68c3dce18587");
    info!("  4. Read/Write/Notify: 681285a6-247f-48c6-80ad-68c3dce18588");

    // Counter for notifications
    let mut counter = 0;
    
    // Main loop
    loop {
        // Update the notifying characteristic every 5 seconds
        counter += 1;
        let status_msg = format!("Uptime: {}s", counter * 5);
        notify_characteristic.lock().set_value(status_msg.as_bytes());
        
        // Send notification if there are subscribers
        if notify_characteristic.lock().subscribed_count() > 0 {
            let _ = notify_characteristic.lock().notify();
            info!("Sent status notification: {}", status_msg);
        }
        
        std::thread::sleep(Duration::from_secs(5));
    }
}