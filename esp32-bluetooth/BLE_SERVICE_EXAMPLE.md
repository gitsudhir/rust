# ESP32 BLE Service Implementation Example

This document describes a complete working example of creating a BLE service with characteristics using the esp32-nimble crate version 0.11.

## Overview

The example implements a BLE GATT server with:
- One custom service
- Three characteristics with different properties
- Connection handling
- Notification support
- Write callback handling

## Code Explanation

### 1. Imports and Setup

```rust
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::sys::link_patches;
use esp_idf_svc::timer::EspTimerService;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;
use log::*;

use esp32_nimble::{BLEDevice, NimbleProperties, BLEReturnCode};
```

### 2. BLE Device Initialization

```rust
// Initialize BLE device
let ble_device = BLEDevice::take();
ble_device.set_device_name("ESP32-BLE-Server")?;

// Create BLE server
let server = ble_device.get_server();
```

### 3. Connection Event Handlers

```rust
server.on_connect(|server, desc| {
    info!("Client connected: {:?}", desc);
    
    // Update connection parameters
    server.update_conn_params(desc.conn_handle, 24, 48, 0, 60).unwrap();
});

server.on_disconnect(|desc, reason| {
    info!("Client disconnected: {:?}, reason: {:?}", desc, reason);
});
```

### 4. Service Creation

```rust
// Create a BLE service
let service = server.create_service(uuid128!("9b574847-f706-436c-bed7-fc01eb0965c1"))?;
```

### 5. Characteristic Creation

#### Static Read-Only Characteristic

```rust
let static_characteristic = service.lock().create_characteristic(
    uuid128!("681285a6-247f-48c6-80ad-68c3dce18585"),
    NimbleProperties::READ,
)?;
static_characteristic.lock().set_value(b"Hello ESP32 BLE World!")?;
```

#### Notifying Characteristic

```rust
let notifying_characteristic = service.lock().create_characteristic(
    uuid128!("681285a6-247f-48c6-80ad-68c3dce18586"),
    NimbleProperties::READ | NimbleProperties::NOTIFY,
)?;
notifying_characteristic.lock().set_value(b"0")?;
```

#### Writable Characteristic

```rust
let writable_characteristic = service.lock().create_characteristic(
    uuid128!("681285a6-247f-48c6-80ad-68c3dce18587"),
    NimbleProperties::READ | NimbleProperties::WRITE,
)?;
writable_characteristic.lock().set_value(b"Writable Characteristic")?;

// Set up write callback
writable_characteristic.lock().on_write(|attr, _| {
    let value = attr.value();
    info!("Received write request: {:?}", std::str::from_utf8(value).unwrap_or("<non-UTF8>"));
    Ok(())
});
```

### 6. Advertising

```rust
let ble_advertising = ble_device.get_advertising();
ble_advertising.lock().start()?;
```

### 7. Main Loop with Notifications

```rust
// Counter for notifications
let mut counter = 0;

// Main loop
loop {
    // Update the notifying characteristic every 5 seconds
    counter += 1;
    let value = counter.to_string();
    notifying_characteristic.lock().set_value(value.as_bytes())?;
    
    // Send notification if there are subscribers
    if notifying_characteristic.lock().subscribed_count() > 0 {
        if let Err(err) = notifying_characteristic.lock().notify() {
            error!("Error sending notification: {:?}", err);
        } else {
            info!("Sent notification: {}", value);
        }
    }
    
    std::thread::sleep(Duration::from_secs(5));
}
```

## Key Concepts

### Error Handling

The example uses proper error handling with `Result` types and the `?` operator to propagate errors.

### Thread Safety

Characteristics are wrapped in `Arc<Mutex<>>` for thread-safe access, accessed using `.lock()`.

### BLE Properties

Different characteristics use different property combinations:
- `NimbleProperties::READ`: Read-only characteristic
- `NimbleProperties::READ | NimbleProperties::NOTIFY`: Readable with notification support
- `NimbleProperties::READ | NimbleProperties::WRITE`: Readable and writable

### UUID Generation

UUIDs are generated using the `uuid128!` macro for 128-bit UUIDs.

## Testing

To test this implementation:

1. Flash the code to your ESP32 device
2. Use a BLE scanner app (like nRF Connect) on your smartphone
3. Scan for devices and look for "ESP32-BLE-Server"
4. Connect to the device
5. Discover services and characteristics
6. Interact with the characteristics:
   - Read the static characteristic
   - Subscribe to notifications on the notifying characteristic
   - Write to the writable characteristic and observe log output

## Customization

To customize this example for your own use:

1. Change the service UUID to your own unique identifier
2. Modify characteristic UUIDs as needed
3. Adjust properties based on your requirements
4. Implement additional event handlers as needed
5. Add more characteristics or services as required

## Troubleshooting

Common issues and solutions:

1. **Device not appearing in scans**:
   - Check that Bluetooth is enabled in sdkconfig
   - Verify antenna connections
   - Ensure sufficient power supply

2. **Unable to connect**:
   - Check connection parameters
   - Verify service and characteristic UUIDs
   - Ensure proper error handling in callbacks

3. **Notifications not working**:
   - Verify that the client has subscribed to notifications
   - Check that `notify()` is called correctly
   - Ensure proper error handling for notification failures