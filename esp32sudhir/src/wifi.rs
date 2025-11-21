use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripheral;
use esp_idf_svc::wifi::{AuthMethod, ClientConfiguration, Configuration, EspWifi};
use log::info;
use std::net::Ipv4Addr;

pub fn wifi(
    ssid: &str,
    pass: &str,
    modem: impl peripheral::Peripheral<P = esp_idf_svc::hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
) -> anyhow::Result<Box<EspWifi<'static>>> {
    let mut auth_method = AuthMethod::WPA2Personal;
    if ssid.is_empty() {
        anyhow::bail!("Missing WiFi name")
    }
    if pass.is_empty() {
        auth_method = AuthMethod::None;
        info!("Wifi password is empty");
    }

    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;

    let mut wifi = Box::new(esp_wifi);

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.try_into().unwrap(),
        password: pass.try_into().unwrap(),
        auth_method,
        ..Default::default()
    }))?;

    wifi.start()?;

    info!("Wifi started");

    wifi.connect()?;

    info!("Wifi connected");

    wifi.wait_netif_up()?;

    let ip_info = wifi.sta_netif().get_ip_info()?;

    info!("Wifi DHCP info: {:?}", ip_info);

    Ok(wifi)
}
