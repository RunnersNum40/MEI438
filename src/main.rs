use anyhow::Result;
use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::EspWifi};
use heapless::String;
use std::time::Instant;
use std::{str::FromStr, thread::sleep, time::Duration};

mod secrets;
use secrets::{WIFI_PASSWORD, WIFI_SSID};

fn setup() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
}

fn get_wifi_driver() -> Result<EspWifi<'static>> {
    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let wifi_driver = EspWifi::new(peripherals.modem, sys_loop, Some(nvs))?;

    Ok(wifi_driver)
}

fn connect_to_wifi(wifi_driver: &mut EspWifi, timeout_secs: u64) -> Result<()> {
    let start = Instant::now();
    let timeout = Duration::from_secs(timeout_secs);
    const CHECK_INTERVAL: Duration = Duration::from_millis(100);

    wifi_driver.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: String::from_str(WIFI_SSID).unwrap(),
        password: String::from_str(WIFI_PASSWORD).unwrap(),
        ..Default::default()
    }))?;

    wifi_driver.start()?;
    wifi_driver.connect()?;
    while !wifi_driver.is_connected()? {
        if start.elapsed() > timeout {
            anyhow::bail!("WiFi connection timed out");
        }
        sleep(CHECK_INTERVAL);
    }

    Ok(())
}

fn main() {
    setup();

    let mut wifi_driver = get_wifi_driver().unwrap();
    connect_to_wifi(&mut wifi_driver, 10).unwrap();

    loop {
        log::info!(
            "IP info: {:?}",
            wifi_driver.sta_netif().get_ip_info().unwrap()
        );
        sleep(Duration::new(10, 0));
    }
}
