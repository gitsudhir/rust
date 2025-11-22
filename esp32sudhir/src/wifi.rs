use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripheral;
use esp_idf_svc::wifi::{AuthMethod, ClientConfiguration, Configuration, EspWifi};
use log::info;

pub fn wifi(
    ssid: &str,
    pass: &str,
    modem: impl peripheral::Peripheral<P = esp_idf_svc::hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
) -> anyhow::Result<EspWifi<'static>> {
    let mut auth_method = AuthMethod::WPA2Personal;
    if ssid.is_empty() {
        anyhow::bail!("Missing WiFi name")
    }
    if pass.is_empty() {
        auth_method = AuthMethod::None;
        info!("Wifi password is empty");
    }

    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;

    esp_wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.try_into().unwrap(),
        password: pass.try_into().unwrap(),
        auth_method,
        ..Default::default()
    }))?;

    esp_wifi.start()?;

    info!("Wifi started");

    esp_wifi.connect()?;

    info!("Wifi connected");

    // Wait for IP address instead of using wait_netif_up
    while !esp_wifi.is_up()? {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let ip_info = esp_wifi.sta_netif().get_ip_info()?;

    info!("Wifi DHCP info: {:?}", ip_info);

    Ok(esp_wifi)
}
