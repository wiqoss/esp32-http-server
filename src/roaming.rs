use std::thread;
use std::time::Duration;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::modem::Modem;
use esp_idf_svc::nvs::{EspNvsPartition, NvsDefault};
use esp_idf_svc::wifi::{AccessPointConfiguration, AuthMethod, Configuration, EspWifi};
use crate::NETWORK_INITIALIZED;

pub fn start_roaming(modem: Modem,
                     sys_loop: EspSystemEventLoop,
                     nvs: EspNvsPartition<NvsDefault>,
) {
    let mut wifi = EspWifi::new(
        modem,
        sys_loop,
        Some(nvs),
    ).unwrap();

    // Config
    let wifi_config = Configuration::AccessPoint(AccessPointConfiguration {
        ssid: String::from("ESP32").parse().unwrap(),
        ssid_hidden: false,
        channel: 6,
        secondary_channel: None,
        protocols: Default::default(),
        auth_method: AuthMethod::WPA2Personal,
        password: String::from("13201930").parse().unwrap(),
        max_connections: 1,
    });
    wifi.set_configuration(&wifi_config).unwrap();

    log::info!("Wi-Fi configured");
    log::info!("Starting Wi-Fi...");

    // Start
    wifi.start().unwrap();

    log::info!("Wi-Fi started");

    // Print the status
    let is_up = if wifi.is_up().unwrap() { "up" } else { "down" };
    let is_started = if wifi.is_started().unwrap() { "" } else { "not " };
    log::info!("Wi-Fi is {} and {}started", is_up, is_started);

    NETWORK_INITIALIZED.store(true, std::sync::atomic::Ordering::Relaxed);

    loop {
        thread::sleep(Duration::from_millis(10_000))
    }
}